use cw_orch::interface;

use cw3_flex_multisig::contract;
pub use cw3_flex_multisig::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
#[cfg(not(target_arch = "wasm32"))]
pub use interfaces::{AsyncQueryMsgInterfaceFns, ExecuteMsgInterfaceFns, QueryMsgInterfaceFns};

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

#[cfg(not(target_arch = "wasm32"))]
/// Copy messages of the contract to implement cw-orch helpers on Execute([`cw_orch::ExecuteFns`]) and Query([`cw_orch::QueryFns`]) interfaces
mod interfaces {
    use super::*;

    #[derive(cw_orch::ExecuteFns, from_interface_derive::FromInterface)]
    enum ExecuteMsgInterface {
        #[cw_orch(payable)]
        Propose {
            title: String,
            description: String,
            msgs: Vec<cosmwasm_std::CosmosMsg<Empty>>,
            // note: we ignore API-spec'd earliest if passed, always opens immediately
            latest: Option<cw_utils::Expiration>,
        },
        Vote {
            proposal_id: u64,
            vote: cw3::Vote,
        },
        // This method is renamed to not conflict with [`CwOrchExecute::execute`]
        #[cw_orch(fn_name("execute_proposal"))]
        Execute {
            proposal_id: u64,
        },
        Close {
            proposal_id: u64,
        },
        /// Handles update hook messages from the group contract
        MemberChangedHook(cw4::MemberChangedHookMsg),
    }

    #[cosmwasm_schema::cw_serde]
    #[derive(
        cosmwasm_schema::QueryResponses, cw_orch::QueryFns, from_interface_derive::FromInterface,
    )]
    pub enum QueryMsgInterface {
        #[returns(cw_utils::ThresholdResponse)]
        Threshold {},
        #[returns(cw3::ProposalResponse)]
        Proposal { proposal_id: u64 },
        #[returns(cw3::ProposalListResponse)]
        ListProposals {
            start_after: Option<u64>,
            limit: Option<u32>,
        },
        #[returns(cw3::ProposalListResponse)]
        ReverseProposals {
            start_before: Option<u64>,
            limit: Option<u32>,
        },
        #[returns(cw3::VoteResponse)]
        // This method is renamed to not conflict with [`ExecuteMsgInterface::Vote`]
        #[cw_orch(fn_name("get_vote"))]
        Vote { proposal_id: u64, voter: String },
        #[returns(cw3::VoteListResponse)]
        ListVotes {
            proposal_id: u64,
            start_after: Option<String>,
            limit: Option<u32>,
        },
        #[returns(cw3::VoterResponse)]
        Voter { address: String },
        #[returns(cw3::VoterListResponse)]
        ListVoters {
            start_after: Option<String>,
            limit: Option<u32>,
        },
        /// Gets the current configuration.
        #[returns(cw3_flex_multisig::state::Config)]
        Config {},
    }
}
