use cosmwasm_std::{Coin, DepsMut, Response, StdResult, MessageInfo};

use crate::state::{COUNTER, MINIMAL_DONATION, OWNER};

// there's no creator added to the instantiation message
// we are relying on who sends the instantiation message
pub fn instantiate(
    deps: DepsMut,
    info: MessageInfo,
    counter: u64,
    minimal_donation: Coin
) -> StdResult<Response> {
    COUNTER.save(deps.storage, &counter)?;
    // initializing a minimal donation
    MINIMAL_DONATION.save(deps.storage, &minimal_donation)?;
    // initializing the owner on instantiation
    OWNER.save(deps.storage, &info.sender)?;

    Ok(Response::new())
}

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
    /* 
    pub fn incremented(value: u64) -> ValueResp {
        ValueResp { value: value + 1 }
    }
    */
}

/// creating a message handler for the execute entry point
pub mod exec {
    // use cosmwasm_std::{DepsMut, MessageInfo, Response, StdResult};
    use cosmwasm_std::{
        Coin, BankMsg, DepsMut, Env, MessageInfo, Response, StdResult, Uint128
    };
 
    use crate::{error::ContractError, state::{COUNTER, MINIMAL_DONATION, OWNER}};

    // adding the MessageInfo to the update function
    // -> MessageInfo contains additional metadata about the sent message 
    // (message sender and the funds sent)
    // replacing poke with donate functionality
    // pub fn poke(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
    pub fn donate(deps: DepsMut, info: MessageInfo) -> StdResult<Response> {
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

        // let counter = COUNTER.load(deps.storage)? + 1;
        // COUNTER.save(deps.storage, &counter)?;
        let mut counter = COUNTER.load(deps.storage)?;
        let minimal_donation = MINIMAL_DONATION.load(deps.storage)?;

        // we're having a minimal donation you want to count
        // now we want to iterate through all the funds sent to the contract 
        // and find out if there is any which is of expected denom, and minimal amount
        // funds sent with the message can be addressed using the funds field of the info argument
        if info.funds.iter().any(|coin| {
            // to filter interesting donations
            // you first need to load a minimal donation from the state
            coin.denom == minimal_donation.denom && coin.amount >= minimal_donation.amount
        }) {
            // not loading a counter if it should not be incremented to save gas
            counter += 1;
            COUNTER.save(deps.storage, &counter)?;
        }
        
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
    pub fn reset(deps: DepsMut, info: MessageInfo, counter: u64) -> Result<Response, ContractError>  {
        
        COUNTER.save(deps.storage, &counter)?;

        let owner = OWNER.load(deps.storage)?;
        if info.sender != owner {
            // return Err(StdError::generic_err("Unauthorized"));
            return Err(ContractError::Unauthorized {
                owner: owner.to_string(),
            });
        }

        let resp = Response::new()
            .add_attribute("action", "reset")
            .add_attribute("sender", info.sender.as_str())
            .add_attribute("counter", counter.to_string());

        Ok(resp)
    }
    
    // handler for the execution message variant
    pub fn withdraw(deps: DepsMut, env: Env, info: MessageInfo) -> Result<Response, ContractError> {
        let owner = OWNER.load(deps.storage)?;
        // we need to check if the message sender is the one who created a contract
        if info.sender != owner {
            // if not, we immediately fail execution with some generic error
            // return Err(StdError::generic_err("Unauthorized"));
            // using our custom error type for our contract instead
            return Err(ContractError::Unauthorized {
                owner: owner.to_string(),
            });
        }
        
        // then we need to figure out how much funds to send to the contract owner
        // because we want to send all the funds we query the blockchain for its state 
        // via a a querier object on the deps argument
        // to get the contract's address we use the env entry point argument
        // it contains all relevant meta information like the currently executed contract address
        let balance = deps.querier.query_all_balances(&env.contract.address)?;
        // preparing the message for the blockchain: the message we are looking for is a BankMsg
        // particularly the Send variant of a BankMsg
        // it takes a funds receiver and amount
        let bank_msg = BankMsg::Send {
            to_address: info.sender.to_string(),
            amount: balance,
        };
        
        // we can add the Send variant of a BankMsg to the Response using the add_message method
        let resp = Response::new()
            .add_message(bank_msg)
            .add_attribute("action", "withdraw")
            .add_attribute("sender", info.sender.as_str());
     
        Ok(resp)
    }
    
    pub fn withdraw_to(
        deps: DepsMut,
        env: Env,
        info: MessageInfo,
        receiver: String,
        funds: Vec<Coin>,
    ) -> Result<Response, ContractError> {
        let owner = OWNER.load(deps.storage)?;
        if info.sender != owner {
            // return Err(StdError::generic_err("Unauthorized"));
            return Err(ContractError::Unauthorized {
                owner: owner.to_string(),
            });
        }

        let mut balance = deps.querier.query_all_balances(&env.contract.address)?;

        if !funds.is_empty() {
            for coin in &mut balance {
                let limit = funds
                    .iter()
                    .find(|c| c.denom == coin.denom)
                    .map(|c| c.amount)
                    .unwrap_or(Uint128::zero());

                coin.amount = std::cmp::min(coin.amount, limit);
            }
        }

        let bank_msg = BankMsg::Send {
            to_address: receiver,
            amount: balance,
        };

        let resp = Response::new()
            .add_message(bank_msg)
            .add_attribute("action", "withdraw")
            .add_attribute("sender", info.sender.as_str());

        Ok(resp)
    }

}