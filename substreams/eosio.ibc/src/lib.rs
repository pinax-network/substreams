use substreams::errors::Error;
use substreams_antelope_core::pb::antelope::{Block};

// local protobuf definitions
mod pb;
use crate::pb::eosio_ibc::{Lightproof};

#[substreams::handlers::map]
fn map_lightproof(block: Block) -> Result<Lightproof, Error> {
    Ok(Lightproof {
        number: block.number,
        id: block.id,
        timestamp: block.header.unwrap().timestamp,
        active_nodes: block.blockroot_merkle.unwrap().active_nodes,
    })
}
