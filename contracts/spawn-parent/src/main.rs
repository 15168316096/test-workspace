#![cfg_attr(not(feature = "native-simulator"), no_std)]
#![cfg_attr(not(test), no_main)]

#[cfg(any(feature = "native-simulator", test))]
extern crate alloc;

#[cfg(not(any(feature = "native-simulator", test)))]
use ckb_std::default_alloc;
#[cfg(not(any(feature = "native-simulator", test)))]
ckb_std::entry!(program_entry);
#[cfg(not(any(feature = "native-simulator", test)))]
default_alloc!();

pub fn program_entry() -> i8 {
    ckb_std::debug!("This is a sample contract!");

    0
}
