use cosmwasm_schema::write_api;

use abstract_cw1_subkeys::msg::{ExecuteMsg, QueryMsg};

use abstract_cw1_whitelist::msg::InstantiateMsg;

fn main() {
    write_api! {
        instantiate: InstantiateMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}
