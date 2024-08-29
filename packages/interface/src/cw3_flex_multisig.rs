use cw_orch::interface;

use cw3_flex_multisig::contract;
pub use cw3_flex_multisig::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

#[interface(InstantiateMsg, ExecuteMsg, QueryMsg, Empty)]
pub struct Cw3FlexMultisig;

#[cfg(not(target_arch = "wasm32"))]
use cw_orch::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
impl<Chain: CwEnv> Uploadable for Cw3FlexMultisig<Chain> {
    // Return the path to the wasm file
    fn wasm(_chain: &ChainInfoOwned) -> WasmPath {
        artifacts_dir_from_workspace!()
            .find_wasm_path("cw3_flex_multisig")
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
