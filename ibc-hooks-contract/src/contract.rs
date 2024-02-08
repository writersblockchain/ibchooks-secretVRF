use cosmwasm_std::{
    entry_point, CosmosMsg, DepsMut, Env, IbcMsg, IbcTimeout, MessageInfo, Response,
    StdResult, StdError, Uint64, Coin, Uint128, IbcReceiveResponse
};

use crate::{
    msg::{IBCLifecycleComplete, InstantiateMsg, Msg}
};

use secret_toolkit::{
    crypto::{sha_256}
};

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
pub fn execute(deps: DepsMut, env: Env, info: MessageInfo, msg: Msg) -> StdResult<Response> {
    match msg {
        Msg::RequestRandom {
            job_id,
            num_words,
        } => try_execute_random(deps, env, info, job_id, num_words)
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

pub fn try_execute_random(_deps: DepsMut, env: Env, _info: MessageInfo, job_id: String, num_words: Uint64) -> Result<Response, StdError> {
     
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
    let result = base64::encode(random_numbers);

    //construct the callback IBC memo that calls the recieving contract on the way back.
    let callback_memo = format!(
        "{{\"wasm\": {{\"contract\": \"{}\", \"msg\": {{\"execute_receive\": {{\"job_id\": {}, \"random_numbers\": {}}}}}}}}}",
        job_id,
        result
    );


    Ok(
        Response::default(),
    )
}
