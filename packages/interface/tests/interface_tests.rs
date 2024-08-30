mod cw1_subkeys {
    use abstract_cw_plus_interface::cw1_subkeys::{
        Cw1SubKeys, ExecuteMsgInterfaceFns, InstantiateMsg, QueryMsgInterfaceFns,
    };
    use cw1_whitelist::msg::AdminListResponse;
    use cw_orch::{mock::Mock, prelude::*};

    #[test]
    fn check_interface() {
        let chain = Mock::new("sender");
        let contract = Cw1SubKeys::new("cw1", chain.clone());
        contract.upload().unwrap();
        contract
            .instantiate(
                &InstantiateMsg {
                    admins: vec![chain.sender_addr().to_string()],
                    mutable: true,
                },
                None,
                &[],
            )
            .unwrap();
        contract.execute_requests(vec![]).unwrap();

        let admins = contract.admin_list().unwrap();
        assert_eq!(
            admins,
            AdminListResponse {
                admins: vec![chain.sender_addr().to_string()],
                mutable: true
            }
        );
        contract.freeze().unwrap();
        let admins = contract.admin_list().unwrap();
        assert_eq!(
            admins,
            AdminListResponse {
                admins: vec![chain.sender_addr().to_string()],
                mutable: false
            }
        )
    }
}

mod cw1_whitelist {
    use abstract_cw_plus_interface::cw1_whitelist::{
        Cw1Whitelist, ExecuteMsgInterfaceFns, InstantiateMsg, QueryMsgInterfaceFns,
    };
    use cw1_whitelist::msg::AdminListResponse;
    use cw_orch::{mock::Mock, prelude::*};

    #[test]
    fn check_interface() {
        let chain = Mock::new("sender");
        let contract = Cw1Whitelist::new("cw1", chain.clone());
        contract.upload().unwrap();
        contract
            .instantiate(
                &InstantiateMsg {
                    admins: vec![chain.sender_addr().to_string()],
                    mutable: true,
                },
                None,
                &[],
            )
            .unwrap();
        contract.execute_requests(vec![]).unwrap();

        let admins = contract.admin_list().unwrap();
        assert_eq!(
            admins,
            AdminListResponse {
                admins: vec![chain.sender_addr().to_string()],
                mutable: true
            }
        );
        contract.freeze().unwrap();
        let admins = contract.admin_list().unwrap();
        assert_eq!(
            admins,
            AdminListResponse {
                admins: vec![chain.sender_addr().to_string()],
                mutable: false
            }
        )
    }
}

mod cw3_fixed_multisig {
    use abstract_cw_plus_interface::cw3_fixed_multisig::{
        Cw3FixedMultisig, ExecuteMsgInterfaceFns, InstantiateMsg, QueryMsgInterfaceFns,
    };
    use cw3_fixed_multisig::msg::Voter;
    use cw_orch::{mock::Mock, prelude::*};

    #[test]
    fn check_interface() {
        let chain = Mock::new("sender");
        let voter = chain.addr_make("voter");
        let contract = Cw3FixedMultisig::new("cw3", chain.clone());
        contract.upload().unwrap();
        contract
            .instantiate(
                &InstantiateMsg {
                    voters: vec![
                        Voter {
                            addr: voter.to_string(),
                            weight: 1,
                        },
                        Voter {
                            addr: chain.sender_addr().to_string(),
                            weight: 1,
                        },
                    ],
                    threshold: cw_utils::Threshold::AbsoluteCount { weight: 2 },
                    max_voting_period: cw_utils::Duration::Time(42424242),
                },
                None,
                &[],
            )
            .unwrap();
        contract
            .call_as(&voter)
            .propose("foobar", vec![], "title", None)
            .unwrap();
        let proposals = contract.list_proposals(None, None).unwrap();
        let proposal_id = proposals.proposals[0].id;
        contract.vote(proposal_id, cw3::Vote::Yes).unwrap();
        contract.execute_proposal(proposal_id).unwrap();

        let proposal = contract.proposal(proposal_id).unwrap();
        assert_eq!(proposal.status, cw3::Status::Executed);
    }
}

mod cw4_group_cw3_flex_multisig {
    use abstract_cw_plus_interface::{
        cw3_flex_multisig::{
            Cw3FlexMultisig, ExecuteMsgInterfaceFns as _, InstantiateMsg as Cw3InstantiateMsg,
            QueryMsgInterfaceFns as _,
        },
        cw4_group::{
            Cw4Group, ExecuteMsgInterfaceFns, InstantiateMsg as Cw4InstantiateMsg,
            QueryMsgInterfaceFns as _,
        },
    };
    use cw_orch::{mock::Mock, prelude::*};

    #[test]
    fn check_interface() {
        let chain = Mock::new("sender");
        let voter1 = chain.addr_make("voter1");
        let voter2 = chain.addr_make("voter2");
        let cw4 = Cw4Group::new("cw4", chain.clone());
        cw4.upload().unwrap();
        cw4.instantiate(
            &Cw4InstantiateMsg {
                admin: Some(chain.sender_addr().to_string()),
                members: vec![
                    cw4::Member {
                        addr: voter1.to_string(),
                        weight: 1,
                    },
                    cw4::Member {
                        addr: voter2.to_string(),
                        weight: 1,
                    },
                ],
            },
            None,
            &[],
        )
        .unwrap();
        chain.wait_blocks(10).unwrap();

        let hook_addr = chain.addr_make("hook");
        cw4.add_hook(hook_addr.to_string()).unwrap();
        let hooks = cw4.hooks().unwrap();
        assert_eq!(hooks.hooks, vec![hook_addr.to_string()]);
        cw4.remove_hook(hook_addr.to_string()).unwrap();
        let hooks = cw4.hooks().unwrap();
        assert!(hooks.hooks.is_empty());

        let cw3 = Cw3FlexMultisig::new("cw3", chain.clone());
        cw3.upload().unwrap();
        cw3.instantiate(
            &Cw3InstantiateMsg {
                group_addr: cw4.address().unwrap().to_string(),
                threshold: cw_utils::Threshold::AbsoluteCount { weight: 2 },
                max_voting_period: cw_utils::Duration::Time(1111111),
                executor: Some(cw3_flex_multisig::state::Executor::Only(
                    chain.sender_addr(),
                )),
                proposal_deposit: None,
            },
            None,
            &[],
        )
        .unwrap();
        cw3.call_as(&voter1)
            .propose("foobar", vec![], "title", None)
            .unwrap();
        let proposals = cw3.list_proposals(None, None).unwrap();
        let proposal_id = proposals.proposals[0].id;
        cw3.call_as(&voter2)
            .vote(proposal_id, cw3::Vote::Yes)
            .unwrap();
        cw3.execute_proposal(proposal_id).unwrap();

        let proposal = cw3.proposal(proposal_id).unwrap();
        assert_eq!(proposal.status, cw3::Status::Executed);
    }
}

// TODO: cw4_stake, cw20_base, cw20_ics20
