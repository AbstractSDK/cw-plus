use cosmwasm_std::Uint128;
use cw20::{Cw20ExecuteMsgFns, MinterResponse};
use cw20_base::{
    interface::Cw20BaseInterface,
    msg::{ExecuteMsg, InstantiateMsg, QueryMsgFns},
};
use cw_orch::prelude::*;

#[test]
fn create_token() -> Result<(), CwOrchError> {
    let mock = MockBech32::new("chain");
    let sender = mock.sender();

    let init_msg = InstantiateMsg {
        name: "TestToken".to_string(),
        symbol: "TTKN".to_string(),
        decimals: 6,
        initial_balances: vec![],
        mint: Some(MinterResponse {
            minter: sender.to_string(),
            cap: None,
        }),
        marketing: None,
    };

    let my_coin = Cw20BaseInterface::new("my_coin", mock);

    my_coin.upload()?;

    my_coin.instantiate(&init_msg, None, None)?;

    my_coin.mint(Uint128::new(100_000), sender.to_string())?;

    let balance = my_coin.balance(sender.to_string())?;

    assert_eq!(balance.balance.u128(), 100_000);

    Ok(())
}
