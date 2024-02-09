use cosmwasm_std::{entry_point, to_json_binary, DepsMut, Env, IbcMsg, MessageInfo, Response, StdResult, Uint128, Uint64};

use crate::ibc::Ics20Packet;
use crate::msg::{ExecuteMsg, InstantiateMsg};

const SECRET_VRF_CONTRACT_ADDRESS: &str = "secret1qvkg5twz0gydr0fs7eqlwehzj4ehfvadeaznn6";
const SECRET_VRF_VERIFICATION_KEY: &str = "BNtlc9vlRCATKKM+OyqN6bEy/t4PW8CaxAgjC0hUMJzzUOPFONVyogCjRHbuVlv5jkD3xwB2YaaFEt9QNVtox2Y=";
const SECRET_TRANSFER_CHANNEL_ID: &str = "channel-3";


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
        env.contract.address.as_str(),
        timeout_sec_from_now.to_string()
    );
    let timeout = env.block.time.plus_seconds(timeout_sec_from_now);

    // build ics20 packet
    let packet = Ics20Packet::new(
        Uint128::new(1),
        "uscrt".to_string(),
        &env.contract.address.to_string(),
        SECRET_VRF_CONTRACT_ADDRESS,
        ibc_hook_memo
    );
    packet.validate()?;

    // prepare ibc message
    let msg = IbcMsg::SendPacket {
        channel_id: SECRET_TRANSFER_CHANNEL_ID.to_string(),
        data: to_json_binary(&packet)?,
        timeout: timeout.into(),
    };

    Ok(
        Response::new().add_message(msg)
    )
}