use cw_orch::interface;

use abstract_cw4_stake::contract;
pub use abstract_cw4_stake::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

#[interface(InstantiateMsg, ExecuteMsg, QueryMsg, Empty)]
pub struct Cw4Stake;

#[cfg(not(target_arch = "wasm32"))]
use cw_orch::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
impl<Chain: CwEnv> Uploadable for Cw4Stake<Chain> {
    // Return the path to the wasm file
    fn wasm(_chain: &ChainInfoOwned) -> WasmPath {
        artifacts_dir_from_workspace!()
            .find_wasm_path("cw1_whitelist")
            .unwrap()
    }
    // Return a CosmWasm contract wrapper
    fn wrapper() -> Box<dyn MockContract<Empty>> {
        Box::new(ContractWrapper::new_with_empty(
            contract::execute,
            contract::instantiate,
            contract::query,
        ))
    }
}
