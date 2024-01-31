pub mod query {
    use crate::msg::ValueResp;
    use cosmwasm_std::{Deps, StdResult};
    use crate::state::COUNTER;
    
    // query::value() function will be called when QueryMsg::Value {} is received
    // returns an arbitrary object which would be serialized before sending as a response
    // pub fn value() -> ValueResp {
    //     ValueResp { value: 0 }
    // }
    // loading a value from the state
    // adding Deps argument to have access to contract storage
    // returning an error in case loading from the state fails
    // the returned value has to be wrapped in Ok(...)
    pub fn value(deps: Deps) -> StdResult<ValueResp> {
        // utilizing the load function to load from the state
        // taking the state accessor as an argument
        let value = COUNTER.load(deps.storage)?;
        Ok(ValueResp { value })
    }
    
    // assignment lesson 3: add query to the contract 
    // taking a single number as its argument and returning the send argument incremented
    pub fn incremented(value: u64) -> ValueResp {
        ValueResp { value: value + 1 }
    }
    
}