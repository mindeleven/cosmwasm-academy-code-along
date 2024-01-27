use serde::{Deserialize, Serialize};

/// creating a query message
/// when the contract is queried, it should be able to create a variety of queries
/// we typically create query messages as enum types
/// every single variant is a separate query the contract understands
#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum QueryMsg {
    Value {},
}

