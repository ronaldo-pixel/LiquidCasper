#![no_std]
#![no_main]

extern crate alloc;

use alloc::{vec, string::ToString};

use casper_contract::contract_api::{runtime, storage, system};
use casper_types::{
    contracts::{
        EntryPoint, EntryPointAccess, EntryPointType, EntryPoints, NamedKeys,
    },
    account::AccountHash,
    runtime_args, RuntimeArgs,
    CLType, Parameter, PublicKey, U512,
};

const ARG_AMOUNT: &str = "amount";
const ARG_VALIDATOR: &str = "validator";

#[no_mangle]
pub extern "C" fn delegate_to_validator() {
    let amount: U512 = runtime::get_named_arg(ARG_AMOUNT);
    let validator: PublicKey = runtime::get_named_arg(ARG_VALIDATOR);

    // Delegator MUST be the transaction signer
    let delegator: AccountHash = runtime::get_caller();

    let auction_hash = system::get_auction();

    runtime::call_contract::<()>(
        auction_hash,
        "delegate",
        runtime_args! {
            "delegator" => delegator,
            "validator" => validator,
            "amount" => amount,
        },
    );
}

#[no_mangle]
pub extern "C" fn call() {
    let mut entry_points = EntryPoints::new();

    entry_points.add_entry_point(EntryPoint::new(
        "delegate_to_validator",
        vec![
            Parameter::new(ARG_AMOUNT, CLType::U512),
            Parameter::new(ARG_VALIDATOR, CLType::PublicKey),
        ],
        CLType::Unit,
        EntryPointAccess::Public,
        EntryPointType::Contract,
    ));

    let (contract_hash, _) = storage::new_contract(
        entry_points,
        Some(NamedKeys::new()),
        Some("liquid_staking_contract".to_string()),
        None,
    );

    runtime::put_key("liquid_staking_contract", contract_hash.into());
}

