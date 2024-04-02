use cw_orch::interface;
use crate::msg::{InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg};

#[interface(InstantiateMsg, ExecuteMsg, QueryMsg, MigrateMsg)]
pub struct Cw20BaseInterface;