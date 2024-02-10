use cosmwasm_std::{entry_point, to_json_binary, Binary, Coin, CosmosMsg::Stargate, DepsMut, Env, IbcTimeoutBlock, MessageInfo, Response, StdResult, Uint128, Uint64};

use crate::msg::{ExecuteMsg, InstantiateMsg};
use anybuf::Anybuf;

const SECRET_VRF_CONTRACT_ADDRESS: &str = "secret1n8l2qrkxhqt9sk9raux5ju22elk60080walrue";
const SECRET_VRF_VERIFICATION_KEY: &str = "BMaKDLqG2Ren2ujxcg6NUWnQHt8yetZI98tYjL+sZRq1URSKgaxRToOSHP+GnOedKTLvL/e1AF+3hn90zDo4F4k=";
const SECRET_TRANSFER_CHANNEL_ID: &str = "channel-8";
const CHAIN_TRANSFER_CHANNEL_ID: &str = "channel-48";

// Instantiate entry point
#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> StdResult<Response> {
    // Return a response with an attribute "init" containing the serialized last operation
    Ok(Response::default()
        .add_attribute("init", to_json_binary(&"Initialized".to_string())?.to_string()))
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> StdResult<Response> {
    match msg {
        ExecuteMsg::RequestRandom { job_id, num_words } => request_random(deps, env, job_id, num_words),
        ExecuteMsg::ReceiveRandom { job_id, result, signature} => receive_random(deps, env, job_id, result, signature)
    }
}
fn receive_random(deps: DepsMut, env: Env, job_id: String, result: String, signature: String) -> StdResult<Response> {
    
   // deps.api.secp256k1_verify(message_hash, signature, public_key)
    Ok(Response::new())
}
fn request_random(deps: DepsMut, env: Env, job_id: String, num_words: Uint64) -> StdResult<Response>{

    let timeout_sec_from_now: u64 = 900;

    let ibc_hook_memo = format!(
        "{{\"wasm\": {{\"contract\": \"{}\", \"msg\": {{\"request_random\": {{\"job_id\": \"{}\", \"num_words\": \"{}\", \"callback_channel_id\": \"{}\", \"callback_to_address\": \"{}\", \"timeout_sec_from_now\": \"{}\"}}}}}}}}",
        SECRET_VRF_CONTRACT_ADDRESS,
        job_id,
        num_words,
        SECRET_TRANSFER_CHANNEL_ID,
        env.contract.address,
        timeout_sec_from_now.to_string()
    );

    let ibc_msg_transfer = Anybuf::new()
        .append_string(1, "transfer") // source port
        .append_string(2, CHAIN_TRANSFER_CHANNEL_ID.to_string()) // source channel
        .append_message(
            3,
            &Anybuf::new()
                .append_string(1, "ujuno")
                .append_string(2, "1".to_string()),
        ) // Token
        .append_string(4, env.contract.address) // sender
        .append_string(5, SECRET_VRF_CONTRACT_ADDRESS.to_string()) // receiver
        .append_message(6, &Anybuf::new().append_uint64(1, 0).append_uint64(2, 0)) // TimeoutHeight
        .append_uint64(7, env.block.time.plus_seconds(timeout_sec_from_now).nanos()) // TimeoutTimestamp
        .append_string(8, ibc_hook_memo); // Memo

    // Construct a CosmosMsg::Stargate message with the serialized data
    let msg = Stargate {
        type_url: "/ibc.applications.transfer.v1.MsgTransfer".to_string(),
        value:  ibc_msg_transfer.into_vec().into()
    };

    // Return the response with your custom IBC message included
    Ok(Response::new().add_message(msg))
}