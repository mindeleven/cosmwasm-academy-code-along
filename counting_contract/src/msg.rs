use serde::{Deserialize, Serialize};

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
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub struct ValueResp {
    pub value: u64,
}
