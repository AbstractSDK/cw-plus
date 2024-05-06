use cosmwasm_std::{DepsMut, Env, Ibc3ChannelOpenResponse, IbcChannelOpenMsg};
use cw_orch::interface;

pub use abstract_cw20_ics20::msg::{ExecuteMsg, InitMsg, MigrateMsg, QueryMsg};

use abstract_cw20_ics20::{
    contract,
    ibc::{
        ibc_channel_close, ibc_channel_connect, ibc_channel_open, ibc_packet_ack,
        ibc_packet_receive, ibc_packet_timeout,
    },
};

#[interface(InitMsg, ExecuteMsg, QueryMsg, MigrateMsg)]
pub struct Cw20Ics20;

#[cfg(not(target_arch = "wasm32"))]
use cw_orch::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
impl<Chain: CwEnv> Uploadable for Cw20Ics20<Chain> {
    // Return the path to the wasm file
    fn wasm(_chain: &ChainInfoOwned) -> WasmPath {
        artifacts_dir_from_workspace!()
            .find_wasm_path("cw20_ics20")
            .unwrap()
    }
    // Return a CosmWasm contract wrapper
    fn wrapper() -> Box<dyn MockContract<Empty>> {
        Box::new(
            ContractWrapper::new_with_empty(
                contract::execute,
                contract::instantiate,
                contract::query,
            )
            .with_migrate(contract::migrate)
            .with_ibc(
                ibc_channel_open_fix,
                ibc_channel_connect,
                ibc_channel_close,
                ibc_packet_receive,
                ibc_packet_ack,
                ibc_packet_timeout,
            ),
        )
    }
}

/// Temporary fix until the cw20_ics20 implementation follows the IBC3 standard
pub fn ibc_channel_open_fix(
    deps: DepsMut,
    env: Env,
    msg: IbcChannelOpenMsg,
) -> Result<Option<Ibc3ChannelOpenResponse>, abstract_cw20_ics20::ContractError> {
    ibc_channel_open(deps, env, msg)?;
    Ok(None)
}
