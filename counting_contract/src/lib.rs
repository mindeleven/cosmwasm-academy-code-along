/// coding along with CosmWasm Academy tutorial part 2, Prepare a project
/// https://academy.cosmwasm.com/learn/smart-contracts/prepare-a-project
/// 
/// code examples and comments are taken from the tutorial
/// check with 
/// cosmwasm-check ./target/wasm32-unknown-unknown/release/counting_contract.wasm

use cosmwasm_std::{
    entry_point, Binary, Deps, DepsMut, Env, MessageInfo, Response, StdResult, to_json_binary
};

use msg::InstantiateMsg;

/// contract module for logic implementation
/// it's private because it contains internal contract logic
mod contract;
pub mod msg;
mod state;
mod error;

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
	deps: DepsMut,
	_env:  Env,
	info: MessageInfo,
	msg: InstantiateMsg,
) -> StdResult<Response> {
    // initializing the state
    // storing some default value on contract instantiation
    // storing is done by calling the save method on the accessor (the Item)
    // the function takes two arguments:
    // -> 1st one is the object implementing Storage trait
    // -> 2nd argument is data is to be stored
    // save function returns StdError type so the ? operator is needed
    //COUNTER.save(deps.storage, &0)?;

	//Ok(Response::new())
    // contract::instantiate(deps, msg.counter)

    contract::instantiate(deps, info, msg.counter, msg.minimal_donation)
}

/// the execute entry point is for handling messages which are able to modify contract state
/// they are used to perform some actual actions
#[entry_point]
pub fn execute(
    deps: DepsMut, 
    env: Env, 
    info: MessageInfo, 
    msg: msg::ExecMsg,
) -> StdResult<Response> {
    use contract::exec;
    use msg::ExecMsg::*;
 
    match msg {
        // Poke {} => exec::poke(deps, info),
        Donate {} => exec::donate(deps, info),
        Reset { counter } => exec::reset(deps, info, counter),
        Withdraw {} => exec::withdraw(deps, env, info),
    }
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
    deps: Deps, 
    _env: Env, 
    msg: msg::QueryMsg
) -> StdResult<Binary> {
    use msg::QueryMsg::*;
    use contract::query;
 
    match msg {
        // Value {} => to_json_binary(&query::value()),
        Value {} => to_json_binary(&query::value(deps)?),
        Incremented { value } => to_json_binary(&query::incremented(value)),
    }
}

/// testing the contract with a multitest in a contract wrapper 
/// the contract wrapper would forward all messages to the proper entry point
/// -> writing a function that creates such a wrapper
/// the execute entry point is needed it for multitest
#[cfg(test)]
mod test {
    // use cosmwasm_std::{Addr, Empty};
    use cosmwasm_std::{coin, coins, Addr, Empty};
    use cw_multi_test::{App, Contract, ContractWrapper, Executor};

    use crate::msg::{ExecMsg, InstantiateMsg, QueryMsg, ValueResp};
    use crate::{execute, instantiate, query};
    
    // some cosmos blockchains need to have a contract returned here 
    // but for this tutorial the returned Contract will always be empty
    fn counting_contract() -> Box<dyn Contract<Empty>> {
        let contract = ContractWrapper::new(execute, instantiate, query);
        // returning an object which can be used with multitest
        Box::new(contract)
    }

    #[test]
    fn query_value() {
        // an App object is the blockchain simulator
        // creating a default App instance
        let mut app = App::default();
        
        // registering the contract object in the blockchain
        // returns the contract ID
        // for the test there is no code stored anywhere 
        // but it performs an equivalent of storing code on the blockchain
        let contract_id = app.store_code(counting_contract());

        // contract instantiation
        // -> the next step is contract instantiation - creating the contract on the blockchain
        // instantiate the contract with the contract ID
        // uploaded code id is the ID returned from the store_code call

        // the sender is the address which sends the message
        // to create a CosmWasm address, we are using the Addr::unchecked function
        // Addr::unchecked("sender") creates an arbitrary address

        // the instantiation message we send to the contract is an Empty message here
        // it is first serialized to JSON and then deserialized back to send it to the contract

        // definition of native funds we want to send with the message
        // most messages can have some tokens sent with them, we pass an empty slice for now

        // next: label of the contract
        // -> the human-readable name of the created contract

        // admin address of the contract
        // -> admins are the only addresses that can later perform migrations of smart contracts

        let contract_addr = app
            .instantiate_contract(
                contract_id,
                Addr::unchecked("sender"),
                // &Empty {},
                &InstantiateMsg {
                    counter: 10,
                    minimal_donation: coin(10, "atom"),
                },
                &[],
                "Counting contract",
                None,
            )
            .unwrap();
        
        // if instantiate_contract() is successful we get back the contract address
        // and can use it to query the contract

        // querying the contract
        // to query the contract on the blockchain we first need to call the wrap() method
        // it converts the app object to a temporary QuerierWrapper object
        // which allows us to query the blockchain
        // to query the contract, we use the query_wasm_smart function
        let resp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Value {})
            .unwrap();
    
        // assert_eq!(resp, ValueResp { value: 0 });
        assert_eq!(resp, ValueResp { value: 10 });
    }
/* 
    // poke got replaced with donate so no test for poking
    #[test]
    fn poke() {
        let mut app = App::default();
    
        let contract_id = app.store_code(counting_contract());
    
        let contract_addr = app
            .instantiate_contract(
                contract_id,
                Addr::unchecked("sender"),
                &InstantiateMsg { counter: 0 },
                &[],
                "Counting contract",
                None,
            )
            .unwrap();
    
        app.execute_contract(
            Addr::unchecked("sender"),
            contract_addr.clone(),
            &ExecMsg::Poke {},
            &[],
        )
        .unwrap();
    
        let resp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Value {})
            .unwrap();
    
        assert_eq!(resp, ValueResp { value: 1 });
    }
*/
    #[test]
    fn donate() {
        let mut app = App::default();

        let contract_id = app.store_code(counting_contract());

        let contract_addr = app
            .instantiate_contract(
                contract_id,
                Addr::unchecked("sender"),
                &InstantiateMsg {
                    counter: 0,
                    minimal_donation: coin(10, "atom"),
                },
                &[],
                "Counting contract",
                None,
            )
            .unwrap();

        app.execute_contract(
            Addr::unchecked("sender"),
            contract_addr.clone(),
            &ExecMsg::Donate {},
            &[],
        )
        .unwrap();

        let resp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Value {})
            .unwrap();

        assert_eq!(resp, ValueResp { value: 0 });
    }

    #[test]
    fn donate_with_funds() {
        // we need to set some initial "sender" tokens balance while creating an app
        // Multi-test is a blockchain simulator that refuses to send tokens out of nowhere

        let sender = Addr::unchecked("sender");

        let mut app = App::new(|router, _api, storage| {
            router
                .bank
                .init_balance(storage, &sender, coins(10, "atom"))
                .unwrap();
        });

        let contract_id = app.store_code(counting_contract());

        let contract_addr = app
            .instantiate_contract(
                contract_id,
                Addr::unchecked("sender"),
                &InstantiateMsg {
                    counter: 0,
                    minimal_donation: coin(10, "atom"),
                },
                &[],
                "Counting contract",
                None,
            )
            .unwrap();

        app.execute_contract(
            Addr::unchecked("sender"),
            contract_addr.clone(),
            &ExecMsg::Donate {},
            &coins(10, "atom"),
        )
        .unwrap();

        let resp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Value {})
            .unwrap();

        assert_eq!(resp, ValueResp { value: 1 });
    }
    
    #[test]
    fn withdraw() {
        //  one way of sending funds is to send a bank message to the blockchain
        let owner = Addr::unchecked("owner");
        let sender = Addr::unchecked("sender");

        let mut app = App::new(|router, _api, storage| {
            router
                .bank
                .init_balance(storage, &sender, coins(10, "atom"))
                .unwrap();
        });

        let contract_id = app.store_code(counting_contract());

        let contract_addr = app
            .instantiate_contract(
                contract_id,
                owner.clone(),
                &InstantiateMsg {
                    counter: 0,
                    minimal_donation: coin(10, "atom"),
                },
                &[],
                "Counting contract",
                None,
            )
            .unwrap();

        app.execute_contract(
            sender.clone(),
            contract_addr.clone(),
            &ExecMsg::Donate {},
            &coins(10, "atom"),
        )
        .unwrap();

        app.execute_contract(
            owner.clone(),
            contract_addr.clone(),
            &ExecMsg::Withdraw {},
            &[],
        )
        .unwrap();

        assert_eq!(
            app.wrap().query_all_balances(owner).unwrap(),
            coins(10, "atom")
        );
        assert_eq!(app.wrap().query_all_balances(sender).unwrap(), vec![]);
        assert_eq!(
            app.wrap().query_all_balances(contract_addr).unwrap(),
            vec![]
        );
    }

    #[test]
    fn reset() {
        let mut app = App::default();

        let contract_id = app.store_code(counting_contract());

        let contract_addr = app
            .instantiate_contract(
                contract_id,
                Addr::unchecked("sender"),
                &InstantiateMsg { 
                    counter: 0,
                    minimal_donation: coin(10, "atom"),
                },
                &[],
                "Counting contract",
                None,
            )
            .unwrap();

        app.execute_contract(
            Addr::unchecked("sender"),
            contract_addr.clone(),
            &ExecMsg::Reset { counter: 10 },
            &[],
        )
        .unwrap();

        let resp: ValueResp = app
            .wrap()
            .query_wasm_smart(contract_addr, &QueryMsg::Value {})
            .unwrap();

        assert_eq!(resp, ValueResp { value: 10 });
    }

}