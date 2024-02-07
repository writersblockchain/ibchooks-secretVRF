use cosmwasm_std::{
    entry_point, to_binary, CosmosMsg, DepsMut, Env, IbcMsg, IbcTimeout, MessageInfo, Response,
    StdResult,
};

use crate::{
    msg::{IBCLifecycleComplete, InstantiateMsg, Msg},
    ContractError,
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
pub fn execute(_deps: DepsMut, env: Env, info: MessageInfo, msg: Msg) -> StdResult<Response> {
    match msg {
        // Msg::RequestRandom {
        //     random_address,
        //     random_code_hash,
        // } => Ok(Response::default().add_messages(vec![CosmosMsg::Wasm(
        //     cosmwasm_std::WasmMsg::Execute {
        //         contract_addr: random_address.clone(),
        //         code_hash: random_code_hash.clone(),
        //         msg: to_binary(&ExecuteRandom {}).unwrap(),
        //         funds: info.funds.clone(),
        //     },
        // )])),
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

pub fn try_execute_random(deps: DepsMut, env: Env) -> Result<Response, ContractError> {
    let random_value = env.block.random.clone().unwrap();
    let new_random = RandomBinary {
        random_binary: random_value,
    };
    RANDOM_BINARY.save(deps.storage, &new_random)?;

    Ok(Response::default())
}
