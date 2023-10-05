use cw_orch::{
    interface,
    prelude::*,
};

use cw1_whitelist::contract;
use cw1_whitelist::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

#[interface(InstantiateMsg, ExecuteMsg, QueryMsg, Empty)]
pub struct Cw1Whitelist;

impl<Chain: CwEnv> Uploadable for Cw1Whitelist<Chain> {
    // Return the path to the wasm file
    fn wasm(&self) -> WasmPath {
        let crate_path = env!("CARGO_MANIFEST_DIR");
        let wasm_path = format!("{}/../../artifacts/{}", crate_path, "cw1_whitelist.wasm");

        WasmPath::new(wasm_path).unwrap()
    }
    // Return a CosmWasm contract wrapper
    fn wrapper(&self) -> Box<dyn MockContract<Empty>> {
        Box::new(
            ContractWrapper::new_with_empty(
                contract::execute,
                contract::instantiate,
                contract::query,
            )
        )
    }
}