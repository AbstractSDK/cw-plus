use cw_orch::interface;

use cw3_fixed_multisig::contract;
pub use cw3_fixed_multisig::msg::{ExecuteMsg, InstantiateMsg, QueryMsg};
#[cfg(not(target_arch = "wasm32"))]
pub use interfaces::{ExecuteMsgInterfaceFns, QueryMsgInterfaceFns, AsyncQueryMsgInterfaceFns};

#[interface(InstantiateMsg, ExecuteMsg, QueryMsg, Empty)]
pub struct Cw3FixedMultisig;

#[cfg(not(target_arch = "wasm32"))]
use cw_orch::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
impl<Chain: CwEnv> Uploadable for Cw3FixedMultisig<Chain> {
    // Return the path to the wasm file
    fn wasm(_chain: &ChainInfoOwned) -> WasmPath {
        artifacts_dir_from_workspace!()
            .find_wasm_path("cw3_fixed_multisig")
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
mod interfaces {
    use super::*;

    use cosmwasm_schema::schemars::JsonSchema;
    #[derive(cw_orch::ExecuteFns)]
enum ExecuteMsgInterface {
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
    Execute {
        proposal_id: u64,
    },
    Close {
        proposal_id: u64,
    },
}

impl From<ExecuteMsgInterface> for ExecuteMsg {
    fn from(value: ExecuteMsgInterface) -> Self {
        match value {
            ExecuteMsgInterface::Propose { title, description, msgs, latest } => ExecuteMsg::Propose { title, description, msgs, latest } ,
            ExecuteMsgInterface::Vote { proposal_id, vote } => ExecuteMsg::Vote { proposal_id, vote } ,
            ExecuteMsgInterface::Execute { proposal_id } => ExecuteMsg::Execute { proposal_id } ,
            ExecuteMsgInterface::Close { proposal_id } => ExecuteMsg::Close { proposal_id } ,
        }
    }
}

#[cosmwasm_schema::cw_serde]
#[derive(cosmwasm_schema::QueryResponses, cw_orch::QueryFns, from_interface_derive::FromInterface)]
enum QueryMsgInterface {
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
}

}