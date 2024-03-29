use cosmwasm_std::{
    entry_point, to_binary, CosmosMsg, DepsMut, Env, IbcMsg, IbcTimeout, MessageInfo, Response,
    StdResult, StdError, Uint64, Coin, Binary, Deps
};

use crate::{
    msg::{IBCLifecycleComplete, InstantiateMsg, Msg, PublicKeyResponse, QueryMsg},
    state::{KeyPair, State, CONFIG}
};

use secret_toolkit::{
    crypto::secp256k1::{PrivateKey, PublicKey},
    crypto::{sha_256, ContractPrng},
};

#[entry_point]
pub fn instantiate(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    // Save both key pairs
    let creator_raw = deps.api.addr_canonicalize(info.sender.as_str())?;
    let state = State {
        admin: creator_raw,
        keyed: false,
        signing_keys: KeyPair::default(),
    };

    CONFIG.save(deps.storage, &state)?;
    let _result = create_signing_keys(deps, env);
    Ok(Response::default())
}

#[entry_point]
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: Msg) -> StdResult<Response> {
    match msg {
        Msg::RequestRandom {
            job_id,
            num_words,
            callback_channel_id, 
            callback_to_address, 
            timeout_sec_from_now
        } => try_execute_random(deps, env, info, job_id, num_words, callback_channel_id, callback_to_address, timeout_sec_from_now),
        Msg::IBCLifecycleComplete(IBCLifecycleComplete::IBCAck {
            channel,
            sequence,
            ack,
            success,
        }) => Ok(Response::default().add_attributes(vec![
            ("ibc_lifecycle_complete.ibc_ack.channel", channel),
            (
                "ibc_lifecycle_complete.ibc_ack.sequence",
                sequence.to_string(),
            ),
            ("ibc_lifecycle_complete.ibc_ack.ack", ack),
            (
                "ibc_lifecycle_complete.ibc_ack.success",
                success.to_string(),
            ),
        ])),
        Msg::IBCLifecycleComplete(IBCLifecycleComplete::IBCTimeout { channel, sequence }) => {
            Ok(Response::default().add_attributes(vec![
                ("ibc_lifecycle_complete.ibc_timeout.channel", channel),
                (
                    "ibc_lifecycle_complete.ibc_timeout.sequence",
                    sequence.to_string(),
                ),
            ]))
        }
    }
}

pub fn try_execute_random(deps: DepsMut, env: Env, info: MessageInfo, job_id: String, num_words: Uint64, callback_channel_id: String, callback_to_address: String, timeout_sec_from_now: Uint64) -> Result<Response, StdError> {
     
    //get base random from secret VRF
    let base_random = match env.block.random {
        Some(random_value) => random_value,
        None => return Err(StdError::generic_err("No random value available")),
    };

    let mut random_numbers = Vec::new();

    //create as many 32byte chunks of random numbers as needed by taking base_random + a counter and then sha256 hash that.
    for i in 0..num_words.into() {
        let mut data = base_random.0.clone();
        data.extend_from_slice(&i.to_be_bytes());
        let hashed_number = sha_256(&data); 
        random_numbers.extend_from_slice(hashed_number.as_slice()); 
    }

    //encode the result as base64 for transfer
    let random_numbers_base64 = base64::encode(random_numbers);

    let config = CONFIG.load(deps.storage)?;

    // load this gateway's signing key
    let mut signing_key_bytes = [0u8; 32];
    signing_key_bytes.copy_from_slice(config.signing_keys.sk.as_slice());

    // used in production to create signature. 
    //This will automatically do a sha256 of the bytes and then sign them. 
    //KEEP THIS IN MIND WHEN VERIFYING THE SIGNATURE!!
    let signature = deps.api.secp256k1_sign([job_id.clone(), random_numbers_base64.clone()].concat().as_bytes(), &signing_key_bytes)
        .map_err(|err| StdError::generic_err(err.to_string()))?;

    let callback_memo = format!(
        "{{\"wasm\": {{\"contract\": \"{}\", \"msg\": {{\"receive_random\": {{\"job_id\": \"{}\", \"randomness\": \"{}\", \"signature\": \"{}\"}}}}}}}}",
        callback_to_address,
        job_id,
        random_numbers_base64,
        base64::encode(signature)
    );
    
    Ok(
        Response::default().add_messages(vec![CosmosMsg::Ibc(IbcMsg::Transfer {
            channel_id: callback_channel_id,
            to_address: callback_to_address,
            amount: Coin { denom: info.funds[0].denom.clone(), amount: info.funds[0].amount.clone() },
            timeout: IbcTimeout::with_timestamp(
                env.block.time.plus_seconds(timeout_sec_from_now.u64()),
            ),
            memo: callback_memo,
        })]),
    )
}

pub fn generate_keypair(
    env: &Env,
) -> Result<(PrivateKey, PublicKey), StdError> {

    // generate and return key pair
    let mut rng = ContractPrng::from_env(env);
    let sk = PrivateKey::parse(&rng.rand_bytes())?;
    let pk = sk.pubkey();

    Ok((sk, pk))
}

fn create_signing_keys(deps: DepsMut, env: Env) -> StdResult<Response> {
    // load config
    let state = CONFIG.load(deps.storage)?;

    // check if the keys have already been created
    if state.keyed {
        return Err(StdError::generic_err(
            "keys have already been created".to_string(),
        ));
    }

    // Generate secp256k1 key pair for signing messages
    let (secret, public) = generate_keypair(&env)?;
    let signing_keys = KeyPair {
        sk: Binary(secret.serialize().to_vec()), // private key is 32 bytes,
        pk: Binary(public.serialize().to_vec()), // public key is 65 bytes
    };

    CONFIG.update(deps.storage, |mut state| {
        state.keyed = true;
        state.signing_keys = signing_keys.clone();
        Ok(state)
    })?;

    let signing_pubkey = signing_keys.pk.to_base64();

    Ok(Response::new()
        .add_attribute_plaintext("signing_pubkey", signing_pubkey))
}

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    let response = match msg {
        QueryMsg::GetPublicKey {} => query_public_key(deps),
    };
    Ok(response.unwrap())
}

// the encryption key will be a base64 string
fn query_public_key(deps: Deps) -> StdResult<Binary> {
    let state: State = CONFIG.load(deps.storage)?;
    to_binary(&PublicKeyResponse {
        signing_keys: state.signing_keys.pk,
    })
}