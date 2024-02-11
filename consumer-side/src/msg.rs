use cosmwasm_schema::cw_serde;

#[cw_serde]
pub struct InstantiateMsg {}

#[cw_serde]
pub enum ExecuteMsg {
    RequestRandom { job_id: String},
    ReceiveRandom { job_id: String, randomness: String, signature: String },
}