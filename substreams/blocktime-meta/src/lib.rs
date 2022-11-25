// mod abi;
mod pb;

use pb::types;
use substreams::{prelude::*, store, log};
use substreams::store::{StoreSetProto};
use crate::pb::block;

/// Extracts blockmeta from each block
#[substreams::handlers::map]
fn map_blockmeta(blk: types::Block) -> Result<block::BlockMeta, substreams::errors::Error> {
    Ok(block::BlockMeta {
        block_num: blk.number,
        block_id: blk.id,
        trx_count: blk.unfiltered_transaction_count,
        timestamp: Some(blk.header.unwrap().timestamp.unwrap())
    })
}

#[substreams::handlers::store]
fn store_blockmeta(block: block::BlockMeta, s: store::StoreSetProto<block::BlockMeta>) {
    log::debug!("storing new block meta at timestamp {} for block {}", block.clone().timestamp.unwrap().to_string(), block.clone().block_num);
    s.set(1, block.clone().timestamp.unwrap().to_string(), &block);
}
