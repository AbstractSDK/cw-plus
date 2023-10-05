use cw_orch::{
    interface,
    prelude::*,
};

use cw20_base::{
    msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
    contract
};

#[interface(InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg)]
pub struct Cw20Base;

impl<Chain: CwEnv> Uploadable for Cw20Base<Chain> {
    // Return the path to the wasm file
    fn wasm(&self) -> WasmPath {
        let crate_path = env!("CARGO_MANIFEST_DIR");
        let wasm_path = format!("{}/../../artifacts/{}", crate_path, "cw20_base.wasm");

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