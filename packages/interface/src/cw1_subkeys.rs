use cw_orch::{interface, prelude::*};

use abstract_cw1_subkeys::contract;
pub use abstract_cw1_subkeys::msg::{ExecuteMsg, QueryMsg};
pub use abstract_cw1_whitelist::msg::InstantiateMsg;

#[interface(InstantiateMsg, ExecuteMsg, QueryMsg, Empty)]
pub struct Cw1SubKeys;

impl<Chain: CwEnv> Uploadable for Cw1SubKeys<Chain> {
    // Return the path to the wasm file
    fn wasm(_chain: &ChainInfoOwned) -> WasmPath {
        artifacts_dir_from_workspace!()
            .find_wasm_path("cw1_subkeys")
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
            .with_migrate(contract::migrate),
        )
    }
}
