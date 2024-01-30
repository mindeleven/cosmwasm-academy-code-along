/// coding along with CosmWasm Academy tutorial part 2, Prepare a project
/// https://academy.cosmwasm.com/learn/smart-contracts/prepare-a-project
/// 
/// code examples and comments are taken from the tutorial
/// check with 
/// cosmwasm-check ./target/wasm32-unknown-unknown/release/counting_contract.wasm

use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Empty, Env, MessageInfo, Response, StdResult, to_json_binary
};

/// contract module for logic implementation
/// it's private because it contains internal contract logic
mod contract;
pub mod msg;

/// the entry point is the first function called by CosmWasm virtual machine 
/// when action is performed on a smart contract
/// unlike native binaries, smart contracts have multiple entry points
/// 
/// the instantiate entry point is called when the smart contract is created for the first time
/// it's like a constructor for a contract
/// 
/// the instantiate arguments:
/// (1) deps: DepsMut -> a utility type for communicating with the outer world
///     allows querying and updating the contract state, 
///     querying another contract state, 
///     gives access to an Api object for dealing with CW addresses
/// (2) env: Env -> an object representing the blockchains state when executing the message
///     the chain height and id, 
///     current timestamp, 
///     and the called contract address
/// (3) info: MessageInfo -> contains metainformation about the message which triggered an execution
///     an address that sends the message 
///     and chain native tokens sent with the message
/// (4) msg: Empty -> the message triggering execution itself
///     for now, it is the Empty type that represents {} JSON, 
///     but the type of this argument can be anything that is deserializable
/// 
/// StdResult<Response> is used as a return tyüe for this simple example
/// it's an alias for Result<Response, StdError>
/// 
/// the entry point is decorated by the #[entry_point] attribute
/// it's wrapping the whole entry point to the form Wasm runtime understands
/// 
/// the instantiate entry point is called when the smart contract is created for the first time
/// it's like a constructor for a contract
#[entry_point]
pub fn instantiate(
	_deps: DepsMut,
	_env:  Env,
	_info: MessageInfo,
	_msg:  Empty,
) -> StdResult<Response> {
	Ok(Response::new())
}

/// the execute entry point is for handling messages which are able to modify contract state
/// they are used to perform some actual actions
#[entry_point]
pub fn execute(
    _deps: DepsMut, 
    _env: Env, 
    _info: MessageInfo, 
    _msg: Empty
) -> StdResult<Response> {
    Ok(Response::new())
}

/// the query entry point is for handling messages requesting some information from a contract
/// they can never affect any contract state, and are used just like database queries
/// -> deps argument is of type Deps instead of DepsMut
///    because queries can never modify any blockchain state
/// -> there is no MessageInfo argument
///    queries can never depend on who sends the message 
///    or on any query circumstances besides the blockchain state itself
///    queries also never have funds sent with them
/// -> the returned type is not a Response, but a Binary type instead
///     it is because the query returns arbitrary data to the querier 
///     instead of processing a full actor flow which is handled with Response type
/// -> querys should be pure function calls that give back the smart contract state
#[entry_point]
pub fn query(
    _deps: Deps, 
    _env: Env, 
    msg: msg::QueryMsg
) -> StdResult<Binary> {
    use msg::QueryMsg::*;
    use contract::query;
 
    match msg {
        Value {} => to_json_binary(&query::value()),
        Incremented { value } => to_json_binary(&query::incremented(value)),
    }
}

/// testing the contract with a multitest in a contract wrapper 
/// the contract wrapper would forward all messages to the proper entry point
/// -> writing a function that creates such a wrapper
#[cfg(test)]
mod test {
    use cosmwasm_std::Empty;
    use cw_multi_test::Contract;
 
    fn counting_contract() -> Box<dyn Contract<Empty>> {
        todo!()
    }
}