use cosmwasm_schema::cw_serde;
use cosmwasm_std::Uint64;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    RequestRandom { job_id: String, num_words: Uint64 },
    ReceiveRandom { job_id: String, result: String, signature: String },
}