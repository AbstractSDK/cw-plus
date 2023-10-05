use cw_orch::{
    interface,
    prelude::*,
};

use cw1_subkeys::{
    msg::{ExecuteMsg, QueryMsg},
    contract
};
use cw1_whitelist::msg::InstantiateMsg;

#[interface(InstantiateMsg, ExecuteMsg, QueryMsg, Empty)]
pub struct Cw1SubKeys;

impl<Chain: CwEnv> Uploadable for Cw1SubKeys<Chain> {
    // Return the path to the wasm file
    fn wasm(&self) -> WasmPath {
        let crate_path = env!("CARGO_MANIFEST_DIR");
        let wasm_path = format!("{}/../../artifacts/{}", crate_path, "cw1_subkeys.wasm");

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
            .with_migrate(contract::migrate),
        )
    }
}