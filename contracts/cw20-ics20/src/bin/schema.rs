use cosmwasm_schema::write_api;

use abstract_cw20_ics20::msg::{ExecuteMsg, InitMsg, QueryMsg};

fn main() {
    write_api! {
        instantiate: InitMsg,
        execute: ExecuteMsg,
        query: QueryMsg,
    }
}
