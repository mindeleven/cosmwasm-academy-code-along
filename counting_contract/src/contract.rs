pub mod query {
    use crate::msg::ValueResp;
    
    // query::value() function will be called when QueryMsg::Value {} is received
    // returns an arbitrary object which would be serialized before sending as a response
    pub fn value() -> ValueResp {
        ValueResp { value: 0 }
    }
    
    // assignment lesson 3: add query to the contract 
    // taking a single number as its argument and returning the send argument incremented
    pub fn incremented(value: u64) -> ValueResp {
        ValueResp { value: value + 1 }
    }
}