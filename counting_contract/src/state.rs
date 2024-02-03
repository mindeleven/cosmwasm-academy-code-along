use cosmwasm_std::{Addr, Coin};
use cw_storage_plus::Item;

/// the contract state is defined by creating accessors to the state objects
/// instead of defining state variables we are creating atoms like Item
/// Item is a type accessing a single object which may exist in the blockchain storage
/// Item is used to access the values on the blockchain
/// the string passed to Item on instantiation is part of a key to address the data
/// Item takes care of serialization and deserialization
pub const COUNTER: Item<u64> = Item::new("counter");
/// Coin is a type representing a single native token amount 
/// it contains a denominator (its unique identifier) and the number of tokens sent
/// contract should only accept messages with a minimal amount of coins
/// setting constant for this minimal amount
pub const MINIMAL_DONATION: Item<Coin> = Item::new("minimal_donation");
/// keeping information about who created the contract
pub const OWNER: Item<Addr> = Item::new("owner");