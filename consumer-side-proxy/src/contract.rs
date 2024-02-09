use cosmwasm_std::{DepsMut, entry_point, Env, 
 IbcMsg, MessageInfo, Response, StdResult, to_json_binary, CosmosMsg, Uint128, Coin, IbcTimeout};

use cosmwasm_std::crypto::secp256k1;

use crate::msg::{ExecuteMsg, InstantiateMsg};

const SECRET_VRF_CONTRACT_ADDRESS: String = "secret1qvkg5twz0gydr0fs7eqlwehzj4ehfvadeaznn6".to_string();
const SECRET_VRF_VERIFICATION_KEY: String  = "BNtlc9vlRCATKKM+OyqN6bEy/t4PW8CaxAgjC0hUMJzzUOPFONVyogCjRHbuVlv5jkD3xwB2YaaFEt9QNVtox2Y=".to_string();
const SECRET_TRANSFER_CHANNEL_ID: String  = "channel-3".to_string();

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
        ExecuteMsg::RequestRandom { job_id, num_words } => request_random(deps, env, job_id, num_words)
        ExecuteMsg::ReceiveRandom { job_id, result, signature} => receive_random(deps, env, job_id, result, signature)
    }
}
fn receive_random(deps: DepsMut, env: Env, job_id: String, result: String, signature: String) -> StdResult<Response> {

    Ok(Response::new())
}
fn request_random(deps: DepsMut, env: Env, job_id: String, num_words: String) -> StdResult<Response>{

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

    Ok(
        Response::default().add_messages(vec![CosmosMsg::Ibc(IbcMsg::Transfer {
            channel_id: SECRET_TRANSFER_CHANNEL_ID,
            to_address: SECRET_VRF_CONTRACT_ADDRESS,
            amount: Coin { denom: "uscrt".to_string(), amount: Uint128::new(1) },
            timeout: IbcTimeout::with_timestamp(
                env.block.time.plus_seconds(timeout_sec_from_now.u64()),
            ),
            memo: ibc_hook_memo,
        })]),
    )
}