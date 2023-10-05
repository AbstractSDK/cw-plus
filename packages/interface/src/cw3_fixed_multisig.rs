use cw_orch::{
    interface,
    prelude::*,
};

use cw3_fixed_multisig::contract;
use cw3_fixed_multisig::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};

#[interface(InstantiateMsg, ExecuteMsg, QueryMsg, Empty)]
pub struct Cw3FixedMultisig;

impl<Chain: CwEnv> Uploadable for Cw3FixedMultisig<Chain> {
    // Return the path to the wasm file
    fn wasm(&self) -> WasmPath {
        let crate_path = env!("CARGO_MANIFEST_DIR");
        let wasm_path = format!("{}/../../artifacts/{}", crate_path, "cw3_fixed_multisig.wasm");

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