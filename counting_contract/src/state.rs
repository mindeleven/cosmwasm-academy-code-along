use cw_storage_plus::Item;

/// the contract state is defined by creating accessors to the state objects
/// instead of defining state variables we are creating atoms like Item
/// these atoms are used to access the values on the blockchain
pub const COUNTER: Item<u64> = Item::new("counter");
