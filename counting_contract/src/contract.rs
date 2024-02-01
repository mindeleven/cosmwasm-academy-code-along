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

/// creating a message handler for the execute entry point
pub mod exec {
    use cosmwasm_std::{DepsMut, MessageInfo, Response, StdResult};
 
    use crate::state::COUNTER;

    // adding the MessageInfo to the update function
    // -> MessageInfo contains additional metadata about the sent message 
    // (message sender and the funds sent)
    pub fn poke(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
        // similar to instantiate, but instead of just storing value in the COUNTER
        // the update function is used to update the underlying value

        // the update function takes the (1) borrow to the storage object 
        // and then (2) the closure, which would be executed on the underlying object
        
        // the value returned by the closure should be a Result 
        // with the type stored as a COUNTER in an Ok variant
        // the Err variant can be anything implementing From<cosmwasm_std::StdError>

        // Rust has to know what type it should use because the error type is never used here
        // the type hint for the type returned from closure has to be provided
        /* 
        COUNTER.update(
            deps.storage, 
            |counter| -> StdResult<_> { Ok(counter + 1) }
        )?;
        */
        
        // splitting updating the counter to keep the new counter value for further usage

        let counter = COUNTER.load(deps.storage)? + 1;
        COUNTER.save(deps.storage, &counter)?;
        
        // every execution emits events (logs reporting what was perfromed by an action)
        // an event contains a type and the set of key-value pairs named attributes

        // events are emitted from execution using the Response::add_event function
        // passing the constructed Event type

        // every execution emits at least one default event
        // to add attributes to the wasm event we can use a Response::add_attribute function
        // adding three attributes to Response object: action, sender & counter
        let resp = Response::new()
            .add_attribute("action", "poke")
            .add_attribute("sender", info.sender.as_str())
            .add_attribute("counter", counter.to_string());
 
        // Ok(Response::new())
        Ok(resp)

    }
    
    // assignment lesson 6: adding another execution message
    // which resets an internal counter (setting it to given value)
    pub fn reset(deps: DepsMut, info: MessageInfo, counter: u64) -> StdResult<Response> {
        COUNTER.save(deps.storage, &counter)?;

        let resp = Response::new()
            .add_attribute("action", "reset")
            .add_attribute("sender", info.sender.as_str())
            .add_attribute("counter", counter.to_string());

        Ok(resp)
    }

}