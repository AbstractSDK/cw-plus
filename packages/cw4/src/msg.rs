use cosmwasm_schema::cw_serde;
#[cfg(feature = "interface")]
use cw_orch::ExecuteFns;

#[cw_serde]
#[cfg_attr(feature = "interface", derive(ExecuteFns))]
pub enum Cw4ExecuteMsg {
    /// Change the admin
    UpdateAdmin { admin: Option<String> },
    /// Add a new hook to be informed of all membership changes. Must be called by Admin
    AddHook { addr: String },
    /// Remove a hook. Must be called by Admin
    RemoveHook { addr: String },
}
