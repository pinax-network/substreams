// mod abi;
mod pb;

use substreams::{prelude::*, log};
use crate::pb::{blocktivity};
use substreams_database_change::pb::database::{table_change::Operation, DatabaseChanges};

/// Extracts the stats from each block
#[substreams::handlers::map]
fn map_blocks(blk: substreams_antelope_core::pb::antelope::Block) -> Result<blocktivity::BlockStats, substreams::errors::Error> {
    Ok(blocktivity::BlockStats {
        block_num: blk.number,
        timestamp: Some(blk.header.as_ref().unwrap().timestamp.clone().unwrap()),
        trx_count: blk.transaction_traces_count(),
        act_count: blk.executed_total_action_count(),
    })
}

#[substreams::handlers::store]
fn store_trx_count(blocktivity: blocktivity::BlockStats, s: StoreAddInt64) {
    log::debug!("block {}: adding transaction count {}", blocktivity.block_num, blocktivity.trx_count);
    s.add(1, get_key(blocktivity.block_num.clone()).to_string(), blocktivity.trx_count.clone() as i64)
}

#[substreams::handlers::store]
fn store_act_count(blocktivity: blocktivity::BlockStats, s: StoreAddInt64) {
    log::debug!("block {}: adding action count {}", blocktivity.block_num, blocktivity.act_count);
    s.add(1, get_key(blocktivity.block_num.clone()).to_string(), blocktivity.act_count.clone() as i64)
}

/// Emits hourly accumulated stats
#[substreams::handlers::map]
fn map_hourly_stats(
    blk: substreams_antelope_core::pb::antelope::Block,
    store_trx_count: StoreGetInt64,
    store_act_count: StoreGetInt64,
) -> Result<blocktivity::Stats, substreams::errors::Error> {
    let mut res: Vec<blocktivity::HourlyStats> = Vec::new();

    // this is the last block of the block range, emit the accumulated stats
    if (blk.number + 1) % 7200 == 0 {
        log::debug!("block stats of block_num {} for key {} with trx_count {} and act_count {}", blk.number, get_key(blk.number), store_trx_count.get_at(1, get_key(blk.number).to_string()).unwrap(), store_act_count.get_at(1, get_key(blk.number).to_string()).unwrap());
        res.push(blocktivity::HourlyStats {
            block_num: get_key(blk.number),
            chain: "EOS".parse().unwrap(),
            timestamp: Some(blk.header.as_ref().unwrap().timestamp.clone().unwrap()), // todo this needs to be replaced with the start block time
            trx_count: store_trx_count.get_at(1, get_key(blk.number).to_string()).unwrap(),
            act_count: store_act_count.get_at(1, get_key(blk.number).to_string()).unwrap(),
        })
    }

    Ok(blocktivity::Stats { stats: res })
}


#[substreams::handlers::map]
pub fn db_out(
    stats: blocktivity::Stats
) -> Result<DatabaseChanges, substreams::errors::Error> {
    let mut database_changes: DatabaseChanges = Default::default();

    for stat in stats.stats {
        database_changes.push_change("hourly_stats", &*(stat.block_num).to_string(), 0, Operation::Create)
            .change("chain", (stat.chain.clone(), stat.chain.clone()))
            .change("trx_count", (0, stat.trx_count))
            .change("act_count", (0, stat.act_count));
    }

    Ok(database_changes)
}

fn get_key(block_num: u32) -> u32 {
    return if block_num % 7200 == 0 {
        block_num.clone()
    } else {
        block_num.clone() - block_num.rem_euclid(7200)
    };
}

#[test]
fn block_num_key_0() {
    assert_eq!(0, get_key(0));
}

#[test]
fn block_num_key_2() {
    assert_eq!(0, get_key(2));
}

#[test]
fn block_num_key_7200() {
    assert_eq!(7200, get_key(7200));
}

#[test]
fn block_num_key_8000() {
    assert_eq!(7200, get_key(8000));
}

#[test]
fn block_num_key_14400() {
    assert_eq!(14400, get_key(14400));
}

