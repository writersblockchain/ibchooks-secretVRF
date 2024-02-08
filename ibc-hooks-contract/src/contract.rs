use cosmwasm_std::{
    entry_point, CosmosMsg, DepsMut, Env, IbcMsg, IbcTimeout, MessageInfo, Response,
    StdResult, StdError, Uint64, Coin, Uint128
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
            request_id,
            num_words,
            callback_channel_id,
            callback_to_address,
            timeout_sec_from_now,
        } => try_execute_random(deps, env, info, request_id, num_words, callback_channel_id, callback_to_address, timeout_sec_from_now),
        Msg::IBCTransfer {
            channel_id,
            to_address,
            amount,
            timeout_sec_from_now,
        } => Ok(
            Response::default().add_messages(vec![CosmosMsg::Ibc(IbcMsg::Transfer {
                channel_id,
                to_address: to_address,
                amount: amount,
                timeout: IbcTimeout::with_timestamp(
                    env.block.time.plus_seconds(timeout_sec_from_now.u64()),
                ),
                memo: format!(
                    "{{\"ibc_callback\":\"{}\"}}",
                    env.contract.address.to_string()
                ),
            })]),
        ),
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

pub fn try_execute_random(_deps: DepsMut, env: Env, _info: MessageInfo, request_id: Uint64, num_words: Uint64, callback_channel_id: String, callback_to_address: String, timeout_sec_from_now: Uint64) -> Result<Response, StdError> {
     
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
        "{{\"wasm\": {{\"contract\": \"{}\", \"msg\": {{\"execute_receive\": {{\"request_id\": {}, \"random_numbers\": {}}}}}}}}}",
        callback_to_address,
        request_id,
        result
    );
    
    Ok(
        Response::default().add_messages(vec![CosmosMsg::Ibc(IbcMsg::Transfer {
            channel_id: callback_channel_id,
            to_address: callback_to_address,
            amount: Coin { denom: "uscrt".to_string(), amount: Uint128::new(1) },
            timeout: IbcTimeout::with_timestamp(
                env.block.time.plus_seconds(timeout_sec_from_now.u64()),
            ),
            memo: callback_memo,
        })]),
    )
}
