use crate::msg::ValueResp;
 
pub mod query {
    use crate::msg::ValueResp;
    
    // query::value() function will be called when QueryMsg::Value {} is received
    // returns an arbitrary object which would be serialized before sending as a response
    pub fn value() -> ValueResp {
        ValueResp { value: 0 }
    }
}