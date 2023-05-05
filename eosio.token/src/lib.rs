#[path = "pb/antelope.eosio.token.v1.rs"]
#[allow(dead_code)]
pub mod eosio_token;
pub use self::eosio_token::*;

mod abi;
mod maps;
mod sinks;
mod utils;
