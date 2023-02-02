use substreams::errors::Error;
use substreams_antelope::Block;

#[path = "pb/antelope.eosio.ibc.v1.rs"]
#[allow(dead_code)]
pub mod eosio_ibc;
pub use self::eosio_ibc::*;

#[substreams::handlers::map]
fn map_lightproof(block: Block) -> Result<Lightproof, Error> {
    Ok(Lightproof {
        number: block.number,
        id: block.id,
        timestamp: block.header.unwrap().timestamp,
        active_nodes: block.blockroot_merkle.unwrap().active_nodes,
    })
}
