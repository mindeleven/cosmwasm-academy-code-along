use cosmwasm_std::Coin;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq)]
#[serde(rename_all = "snake_case")]
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
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    // curly braces here are related to how serde is serializing JSON values
    Value {},
    Incremented { value: u64 },
}

// creating a new message for the execute entry point
// define an enum with a single variant per execution message we want to handle
// the message handler for this is in src/contract.rs
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum ExecMsg {
    // removing poke and adding donate Msg
    // Poke {},
    Donate {},
    Reset {
        #[serde(default)]
        counter: u64,
    },
    // execution message variant to send funds
    Withdraw {},
}

/// second message I created is a response to the Value query
/// very similar to the Query message but a struct this time
/// the rename_all attribute is unnecessary but there for consistency
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct ValueResp {
    pub value: u64,
}

