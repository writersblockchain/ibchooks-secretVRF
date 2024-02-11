use cosmwasm_std::{entry_point, CosmosMsg::Stargate, DepsMut, Env, MessageInfo, Response, StdResult, StdError};

use sha2::{Sha256, Digest};
use crate::msg::{ExecuteMsg, InstantiateMsg};
use anybuf::Anybuf;


//For this demo, these values are provided as hardcoded consts. 
//You can also store these values into the storage if needed
const SECRET_VRF_CONTRACT_ADDRESS: &str = "secret1up0mymn4993hgn7zpzu4je34w0n5s7l0mem7rk";
const SECRET_VRF_VERIFICATION_KEY: &str = "BClOY6gcGjBCqeaFskrg0VIzptmyftgfY329GcZOvr3/eH/C4pJ4nH6ch6W/gjog8UErnEpIbMUOmElayUOxDas=";

//Juno
const SECRET_TRANSFER_CHANNEL_ID: &str = "channel-8";
const CHAIN_TRANSFER_CHANNEL_ID: &str = "channel-48";

//Archway
//const SECRET_TRANSFER_CHANNEL_ID: &str = "channel-84";
//const CHAIN_TRANSFER_CHANNEL_ID: &str = "channel-21";

// Instantiate entry point
#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::RequestRandom { job_id } => request_random(deps, env, info, job_id),
        ExecuteMsg::ReceiveRandom { job_id, randomness, signature} => receive_random(deps, env, job_id, randomness, signature)
    }
}
fn receive_random(deps: DepsMut, _env: Env, job_id: String, randomness: String, signature: String) -> StdResult<Response> {
    
    //check if the randomness is correct was wasn't manipulated during transit
    //using deps.api is fine for this example here, but this might change with future versions of cosmwasm, careful.
    // Create a new Sha256 hasher instance to hash the input data for verfication
    let mut hasher = Sha256::new();
    hasher.update([job_id.clone(), randomness.clone()].concat().as_bytes());
    let hash_result = hasher.finalize();
    let signature_correct = deps.api.secp256k1_verify(&hash_result, &base64::decode(signature).unwrap(), &base64::decode(SECRET_VRF_VERIFICATION_KEY).unwrap())
    .map_err(|err| StdError::generic_err(err.to_string()))?;
    if !signature_correct {
        return Err(StdError::generic_err("Could not verify Secret VRF signature"));
    }

    //do whatever computation you need to do 

    Ok(Response::default().add_attribute("random", "successfull"))
}
fn request_random(_deps: DepsMut, env: Env, info: MessageInfo, job_id: String) -> StdResult<Response>{

    //do your preparation for requesting a random number here 

    //create the IBC Hook memo that will be execute by Secret Network 
    let ibc_callback_hook_memo = format!(
        "{{\"wasm\": {{\"contract\": \"{}\", \"msg\": {{\"request_random\": {{\"job_id\": \"{}\", \"num_words\": \"1\", \"callback_channel_id\": \"{}\", \"callback_to_address\": \"{}\", \"timeout_sec_from_now\": \"{}\"}}}}}}}}",
        SECRET_VRF_CONTRACT_ADDRESS, // Secret VRF Contract address on Secret Network
        job_id, 
        SECRET_TRANSFER_CHANNEL_ID, // IBC Channel on the Secret Network side to send it back 
        env.contract.address,
        "900" //IBC callback timeout, here 900s = 15 min
    );

    // Construct a CosmosMsg::Stargate message with the serialized IBC Transfer Data
    let msg = Stargate {
        type_url: "/ibc.applications.transfer.v1.MsgTransfer".to_string(),
        value:  Anybuf::new() 
        .append_string(1, "transfer") // source port
        .append_string(2, CHAIN_TRANSFER_CHANNEL_ID.to_string()) // source channel (IBC Channel on your network side)
        .append_message(3,&Anybuf::new().append_string(1, info.funds[0].denom.clone()).append_string(2, info.funds[0].amount.to_string()),) // Token
        .append_string(4, env.contract.address) // sender
        .append_string(5, SECRET_VRF_CONTRACT_ADDRESS.to_string()) // receiver
        .append_message(6, &Anybuf::new().append_uint64(1, 0).append_uint64(2, 0)) // TimeoutHeight
        .append_uint64(7, env.block.time.plus_seconds(900).nanos()) // TimeoutTimestamp, here 900s = 15 min
        .append_string(8, ibc_callback_hook_memo).into_vec().into()
    };

    // Return the response with the Secret VRF IBC message added to it
    Ok(Response::new().add_message(msg))
}