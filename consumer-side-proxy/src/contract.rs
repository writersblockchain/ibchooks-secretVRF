use cosmwasm_std::{Binary, Deps, DepsMut, entry_point, Env, from_json, Ibc3ChannelOpenResponse, IbcBasicResponse, IbcChannelCloseMsg, IbcChannelConnectMsg, IbcChannelOpenMsg, IbcChannelOpenResponse, IbcMsg, IbcPacketAckMsg, IbcPacketReceiveMsg, IbcPacketTimeoutMsg, IbcReceiveResponse, IbcTimeout, MessageInfo, Response, StdResult, to_json_binary, WasmMsg, secp256k1};

use crate::msg::{CallbackInfo, ExecuteMsg, InstantiateMsg, PacketMsg, QueryMsg, RandomCallback};
use crate::state::{Channel, load_callback, save_callback, StoredRandomAnswer};

// use crate::utils::verify_callback;

// Define a constant for the IBC app version
pub const IBC_APP_VERSION: &str = "ibc-v1";
// Define a constant for the packet lifetime in seconds
const PACKET_LIFETIME: u64 = 60 * 60;

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

        ExecuteMsg::RequestRandom { job_id, callback } => {

            let job_id = 1; 
            let 
            // memo: JSON.stringify({
            //     wasm: {
            //       contract: hooksContractAddress,
            //       msg: {
            //         request_random: {
            //           job_id: "1",
            //           num_words: "20",
            //         },
            //       },
            //     },
            //   }),
            Ok(Response::new().add_message(IbcMsg::SendPacket {
                channel_id,
                data: to_json_binary(&packet)?,
                timeout: IbcTimeout::with_timestamp(env.block.time.plus_seconds(PACKET_LIFETIME)),
                memo: 
            }))
        }
        ExecuteMsg::RecieveRandom { job_id, callback } => {
            // Get the last opened channel ID
            let channel_id = Channel::get_last_opened(deps.storage)?;
            // Create a new packet message to request a random value
            let packet = PacketMsg::RequestRandom {
                job_id: job_id.clone(),
                length: None,
            };

            save_callback(deps.storage, callback)?;
            // memo: JSON.stringify({
            //     wasm: {
            //       contract: hooksContractAddress,
            //       msg: {
            //         request_random: {
            //           job_id: "1",
            //           num_words: "20",
            //         },
            //       },
            //     },
            //   }),
            Ok(Response::new().add_message(IbcMsg::SendPacket {
                channel_id,
                data: to_json_binary(&packet)?,
                timeout: IbcTimeout::with_timestamp(env.block.time.plus_seconds(PACKET_LIFETIME)),
                memo: 
            }))
        }
    }
}