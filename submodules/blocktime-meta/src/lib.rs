// mod abi;
mod pb;

use pb::types;
use substreams::{prelude::*, store, log};
use substreams::store::{StoreSetProto};
use chrono::{NaiveDateTime};
use crate::pb::block;

#[derive(Debug, PartialEq)]
pub struct BlockTimestamp(chrono::NaiveDateTime);

impl BlockTimestamp {
    pub fn from_block(blk: &types::Block) -> Self {
        let header = blk.header.as_ref().unwrap();
        let timestamp = header.timestamp.as_ref().unwrap();

        BlockTimestamp(NaiveDateTime::from_timestamp(
            timestamp.seconds,
            timestamp.nanos as u32,
        ))
    }
}

impl ToString for BlockTimestamp {
    fn to_string(&self) -> String {
        self.0.to_string()
    }
}

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
