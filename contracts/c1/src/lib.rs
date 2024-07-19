#![cfg_attr(not(feature = "simulator"), no_std)]
#![allow(special_module_name)]
#[cfg(feature = "simulator")]
mod main;
#[cfg(feature = "simulator")]
pub use main::program_entry;

#[cfg(feature = "simulator")]
ckb_std::entry_simulator!(main::program_entry);
