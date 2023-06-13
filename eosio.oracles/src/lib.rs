#[path = "pb/antelope.eosio.oracles.v1.rs"]
#[allow(dead_code)]
pub mod eosio_oracles;
pub use self::eosio_oracles::*;

mod abi;
mod maps;
mod sinks;
mod utils;
