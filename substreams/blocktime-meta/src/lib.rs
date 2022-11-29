// mod abi;
mod pb;

use substreams::{prelude::*, store, log};
use substreams::store::{StoreSetProto};
use crate::pb::block;

/// Extracts blockmeta from each block
#[substreams::handlers::map]
fn map_blockmeta(blk: substreams_antelope_core::pb::antelope::Block) -> Result<block::BlockMeta, substreams::errors::Error> {
    Ok(block::BlockMeta {
        block_num: blk.number,
        block_id: blk.id.clone(),
        trx_count: blk.transaction_traces_count(),
        timestamp: Some(blk.header.unwrap().timestamp.unwrap())
    })
}

#[substreams::handlers::store]
fn store_blockmeta(block: block::BlockMeta, s: store::StoreSetProto<block::BlockMeta>) {
    log::debug!("storing new block meta at timestamp {} for block {}", block.clone().timestamp.unwrap().to_string(), block.clone().block_num);
    s.set(1, block.clone().timestamp.unwrap().to_string(), &block);
}
