/// coding along with CosmWasm Academy tutorial part 2, Prepare a project
/// https://academy.cosmwasm.com/learn/smart-contracts/prepare-a-project
/// 
/// code examples and comments are taken from the tutorial
/// 

use cosmwasm_std::{
    DepsMut, Env, MessageInfo, Empty, StdResult, Response, entry_point
};

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
/// the entry point is decorated by the #[entry_point] attribute
/// it's wrapping the whole entry point to the form Wasm runtime understands
#[entry_point]
pub fn instantiate(
	_deps: DepsMut,
	_env:  Env,
	_info: MessageInfo,
	_msg:  Empty,
) -> StdResult<Response> {
	Ok(Response::new())
}