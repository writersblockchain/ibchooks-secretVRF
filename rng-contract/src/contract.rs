use cosmwasm_std::{
    entry_point, to_binary, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdError, StdResult,
};

use crate::error::ContractError;
use crate::msg::{ExecuteMsg, InstantiateMsg, QueryMsg, RandomResponse};
use crate::state::{RandomBinary, RANDOM_BINARY};
#[entry_point]
pub fn instantiate(
    _deps: DepsMut,
    _env: Env,
    _info: MessageInfo,
    _msg: InstantiateMsg,
) -> Result<Response, StdError> {
    Ok(Response::default())
}

#[entry_point]
pub fn execute(
    deps: DepsMut,
    env: Env,
    _info: MessageInfo,
    msg: ExecuteMsg,
) -> Result<Response, ContractError> {
    match msg {
        ExecuteMsg::ExecuteRandom {} => try_execute_random(deps, env),
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

#[entry_point]
pub fn query(deps: Deps, _env: Env, msg: QueryMsg) -> StdResult<Binary> {
    match msg {
        QueryMsg::GetRandom {} => to_binary(&query_random(deps)?),
    }
}

fn query_random(deps: Deps) -> StdResult<RandomResponse> {
    let random = RANDOM_BINARY.load(deps.storage)?;
    Ok(RandomResponse { random })
}
