use cosmwasm_std::Coin;
use cosmwasm_schema::cw_serde;
// use schemars::JsonSchema;
// use serde::{Deserialize, Serialize};
use cosmwasm_schema::QueryResponses;

// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// #[serde(rename_all = "snake_case")]
// replacing precious derives with #[cw_serde]
#[cw_serde]
pub struct InstantiateMsg {
    #[serde(default)]
    pub counter: u64,
    // initializing a minimal donation
    pub minimal_donation: Coin,
}

/// creating a query message
/// when the contract is queried, it should be able to create a variety of queries
/// we typically create query messages as enum types
/// every single variant is a separate query the contract understands
/// the message has to derive Deserialize and implement Serialize
/// so it can be used to send this message from a different contract or tests
// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
// #[serde(rename_all = "snake_case")]
// replacing precious derives with #[cw_serde]
#[cw_serde]
#[derive(QueryResponses)]
pub enum QueryMsg {
    // #[returns(...)] attribute is now required on every query variant
    // it describes what response type is returned for the particular query
    #[returns(ValueResp)]
    // curly braces here are related to how serde is serializing JSON values
    Value {},
    // Incremented { value: u64 },
}

// creating a new message for the execute entry point
// define an enum with a single variant per execution message we want to handle
// the message handler for this is in src/contract.rs
// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, JsonSchema)]
// #[serde(rename_all = "snake_case")]
// replacing precious derives with #[cw_serde]
#[cw_serde]
pub enum ExecMsg {
    // removing poke and adding donate Msg
    // Poke {},
    Donate {},
    Reset {
        counter: u64,
    },
    // execution message variant to send funds
    Withdraw {},
    WithdrawTo {
        receiver: String,
        funds: Vec<Coin>,
    },
}

/// second message I created is a response to the Value query
/// very similar to the Query message but a struct this time
/// the rename_all attribute is unnecessary but there for consistency
// #[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, JsonSchema)]
// #[serde(rename_all = "snake_case")]
// replacing precious derives with #[cw_serde]
#[cw_serde]
pub struct ValueResp {
    pub value: u64,
}

