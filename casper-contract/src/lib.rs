#![no_std]
#![no_main]

extern crate alloc;

use alloc::string::String;
use casper_contract::contract_api::{runtime, storage};

#[no_mangle]
pub extern "C" fn call() {
    // Create a new on-chain value
    let greeting = String::from("Hello Casper");

    // Store it in global state and get a URef
    let greeting_ref = storage::new_uref(greeting);

    // Save it under a named key so it’s easy to find
    runtime::put_key("greeting", greeting_ref.into());
}
