use cosmwasm_std::StdError;
use thiserror::Error;

// deriving the thiserror::Error trait generates all the boilerplate
// so the error is implementing std::error::Error trait
#[derive(Error, Debug, PartialEq)]
pub enum ContractError {
    // it is also crucial to implement the other Std variant
    // it wraps any error of the StdError type
    // which could be returned by CosmWasm standard library or utilities
    #[error("{0}")]
    Std(#[from] StdError),
    // for our contract, the most important error is and Unauthorized variant
    // it is the only thing we return from the contract manually
    // needs error attribute with some format string how to print the error
    #[error("Unauthorized - only {owner} can call it")]
    Unauthorized { owner: String },
}