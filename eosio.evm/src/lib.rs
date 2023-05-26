#[path = "pb/antelope.eosio.evm.v1.rs"]
#[allow(dead_code)]
pub mod eosio_evm;
pub use self::eosio_evm::*;

mod abi;
mod maps;
mod sinks;
mod utils;
