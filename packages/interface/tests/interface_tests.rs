mod cw1_whitelist {
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
