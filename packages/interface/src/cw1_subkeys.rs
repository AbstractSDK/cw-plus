use cw_orch::interface;

use cw1_subkeys::contract;
pub use cw1_subkeys::msg::{ExecuteMsg, QueryMsg};
pub use cw1_whitelist::msg::InstantiateMsg;
#[cfg(not(target_arch = "wasm32"))]
pub use interfaces::{ExecuteMsgInterfaceFns, QueryMsgInterfaceFns, AsyncQueryMsgInterfaceFns};

#[interface(InstantiateMsg, ExecuteMsg, QueryMsg, Empty)]
pub struct Cw1SubKeys;

#[cfg(not(target_arch = "wasm32"))]
use cw_orch::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
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

#[cfg(not(target_arch = "wasm32"))]
mod interfaces {
    use super::*;

    use cosmwasm_schema::schemars::JsonSchema;

    #[derive(cw_orch::ExecuteFns)]
    enum ExecuteMsgInterface<T = Empty>
    where
        T: Clone + std::fmt::Debug + PartialEq + JsonSchema,
    {
        /// Execute requests the contract to re-dispatch all these messages with the
        /// contract's address as sender. Every implementation has it's own logic to
        /// determine in
        // This method is renamed to not conflict with `execute` method
        ExecuteRequests {
            msgs: Vec<cosmwasm_std::CosmosMsg<T>>,
        },
        /// Freeze will make a mutable contract immutable, must be called by an admin
        Freeze {},
        /// UpdateAdmins will change the admin set of the contract, must be called by an existing admin,
        /// and only works if the contract is mutable
        UpdateAdmins { admins: Vec<String> },

        /// Add an allowance to a given subkey (subkey must not be admin)
        IncreaseAllowance {
            spender: String,
            amount: Coin,
            expires: Option<cw_utils::Expiration>,
        },
        /// Decreases an allowance for a given subkey (subkey must not be admin)
        DecreaseAllowance {
            spender: String,
            amount: Coin,
            expires: Option<cw_utils::Expiration>,
        },

        // Setups up permissions for a given subkey.
        SetPermissions {
            spender: String,
            permissions: cw1_subkeys::state::Permissions,
        },
    }

    impl<T> From<ExecuteMsgInterface<T>> for ExecuteMsg<T>
    where
        T: Clone + std::fmt::Debug + PartialEq + JsonSchema,
    {
        fn from(value: ExecuteMsgInterface<T>) -> Self {
            match value {
                ExecuteMsgInterface::ExecuteRequests { msgs } => ExecuteMsg::Execute { msgs },
                ExecuteMsgInterface::Freeze {} => ExecuteMsg::Freeze {},
                ExecuteMsgInterface::UpdateAdmins { admins } => ExecuteMsg::UpdateAdmins { admins },
                ExecuteMsgInterface::IncreaseAllowance {
                    spender,
                    amount,
                    expires,
                } => ExecuteMsg::IncreaseAllowance {
                    spender,
                    amount,
                    expires,
                },
                ExecuteMsgInterface::DecreaseAllowance {
                    spender,
                    amount,
                    expires,
                } => ExecuteMsg::DecreaseAllowance {
                    spender,
                    amount,
                    expires,
                },
                ExecuteMsgInterface::SetPermissions {
                    spender,
                    permissions,
                } => ExecuteMsg::SetPermissions {
                    spender,
                    permissions,
                },
            }
        }
    }

    #[cosmwasm_schema::cw_serde]
    #[derive(cosmwasm_schema::QueryResponses, cw_orch::QueryFns)]
    enum QueryMsgInterface<T = Empty>
    where
        T: Clone + std::fmt::Debug + PartialEq + JsonSchema,
    {
        /// Shows all admins and whether or not it is mutable
        #[returns(cw1_whitelist::msg::AdminListResponse)]
        AdminList {},
        /// Get the current allowance for the given subkey (how much it can spend)
        #[returns(cw1_subkeys::state::Allowance)]
        Allowance { spender: String },
        /// Get the current permissions for the given subkey (how much it can spend)
        #[returns(cw1_subkeys::msg::PermissionsInfo)]
        Permissions { spender: String },
        /// Checks permissions of the caller on this proxy.
        /// If CanExecute returns true then a call to `Execute` with the same message,
        /// before any further state changes, should also succeed.
        #[returns(cw1::CanExecuteResponse)]
        CanExecute {
            sender: String,
            msg: cosmwasm_std::CosmosMsg<T>,
        },
        /// Gets all Allowances for this contract
        #[returns(cw1_subkeys::msg::AllAllowancesResponse)]
        AllAllowances {
            start_after: Option<String>,
            limit: Option<u32>,
        },
        /// Gets all Permissions for this contract
        #[returns(cw1_subkeys::msg::AllPermissionsResponse)]
        AllPermissions {
            start_after: Option<String>,
            limit: Option<u32>,
        },
    }

    impl <T> From<QueryMsgInterface<T>> for QueryMsg <T>
    where
    T: Clone + std::fmt::Debug + PartialEq + JsonSchema{
        fn from(value: QueryMsgInterface<T>) -> Self {
            match value {
                QueryMsgInterface::AdminList {  } => QueryMsg::AdminList {  },
                QueryMsgInterface::Allowance { spender } => QueryMsg::Allowance { spender },
                QueryMsgInterface::Permissions { spender } => QueryMsg::Permissions { spender },
                QueryMsgInterface::CanExecute { sender, msg } => QueryMsg::CanExecute { sender, msg },
                QueryMsgInterface::AllAllowances { start_after, limit } => QueryMsg::AllAllowances { start_after, limit },
                QueryMsgInterface::AllPermissions { start_after, limit } => QueryMsg::AllPermissions { start_after, limit },
            }
        }
    }
}
