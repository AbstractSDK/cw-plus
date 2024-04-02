use crate::{
    contract,
    msg::{ExecuteMsg, InstantiateMsg, MigrateMsg, QueryMsg},
};
use cw_orch::{interface, prelude::*};

#[interface(InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg)]
pub struct Cw20BaseInterface;

#[cfg(not(target_arch = "wasm32"))]
impl<Chain: CwEnv> Uploadable for Cw20BaseInterface<Chain> {
    fn wrapper(&self) -> Box<dyn MockContract<Empty, Empty>> {
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
