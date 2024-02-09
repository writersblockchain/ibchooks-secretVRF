use cosmwasm_schema::{cw_serde};
use cosmwasm_std::{Binary, ContractInfo};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    RequestRandom {},
}