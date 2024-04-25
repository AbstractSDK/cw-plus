use cw_orch::{interface, prelude::*};

use abstract_cw1_whitelist::contract;
pub use abstract_cw1_whitelist::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

#[interface(InstantiateMsg, ExecuteMsg, QueryMsg, Empty)]
pub struct Cw1Whitelist;

impl<Chain: CwEnv> Uploadable for Cw1Whitelist<Chain> {
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
