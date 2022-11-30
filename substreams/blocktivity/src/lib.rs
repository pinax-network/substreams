// mod abi;
mod pb;

use substreams::{prelude::*, log};
use crate::pb::{blocktivity};

/// Extracts the stats from each block
#[substreams::handlers::map]
fn map_blocks(blk: substreams_antelope_core::pb::antelope::Block) -> Result<blocktivity::BlockStats, substreams::errors::Error> {
    Ok(blocktivity::BlockStats {
        block_num: blk.number,
        timestamp: Some(blk.header.as_ref().unwrap().timestamp.clone().unwrap()),
        trx_count: blk.transaction_traces_count(),
        act_count: blk.executed_total_action_count(),
        cpu_usage_us: blk.executed_transaction_traces().map(|trx| trx.receipt.as_ref().unwrap().cpu_usage_micro_seconds).sum(),
        net_usage_words: blk.executed_transaction_traces().map(|trx| trx.receipt.as_ref().unwrap().net_usage_words).sum(),
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

#[substreams::handlers::store]
fn store_cpu_usage_us(blocktivity: blocktivity::BlockStats, s: StoreAddInt64) {
    log::debug!("block {}: adding cpu_usage_us {}", blocktivity.block_num, blocktivity.cpu_usage_us);
    s.add(1, get_key(blocktivity.block_num.clone()).to_string(), blocktivity.cpu_usage_us.clone() as i64)
}

#[substreams::handlers::store]
fn store_net_usage_words(blocktivity: blocktivity::BlockStats, s: StoreAddInt64) {
    log::debug!("block {}: adding net_usage_words {}", blocktivity.block_num, blocktivity.net_usage_words);
    s.add(1, get_key(blocktivity.block_num.clone()).to_string(), blocktivity.net_usage_words.clone() as i64)
}

/// Emits hourly accumulated stats
#[substreams::handlers::map]
fn map_hourly_stats(
    blk: substreams_antelope_core::pb::antelope::Block,
    store_trx_count: StoreGetInt64,
    store_act_count: StoreGetInt64,
    store_cpu_usage_us: StoreGetInt64,
    store_net_usage_words: StoreGetInt64,
) -> Result<blocktivity::Stats, substreams::errors::Error> {
    let mut res: Vec<blocktivity::HourlyStats> = Vec::new();

    // this is the last block of the block range, emit the accumulated stats
    if (blk.number + 1) % 7200 == 0 {
        log::debug!("block stats of block_num {} for key {} with trx_count {} and act_count {}", blk.number, get_key(blk.number), store_trx_count.get_at(1, get_key(blk.number).to_string()).unwrap(), store_act_count.get_at(1, get_key(blk.number).to_string()).unwrap());
        res.push(blocktivity::HourlyStats {
            block_num: get_key(blk.number),
            timestamp: Some(blk.header.as_ref().unwrap().timestamp.clone().unwrap()), // todo this needs to be replaced with the start block time
            trx_count: store_trx_count.get_at(1, get_key(blk.number).to_string()).unwrap(),
            act_count: store_act_count.get_at(1, get_key(blk.number).to_string()).unwrap(),
            cpu_usage_us: store_cpu_usage_us.get_at(1, get_key(blk.number).to_string()).unwrap(),
            net_usage_words: store_net_usage_words.get_at(1, get_key(blk.number).to_string()).unwrap(),
        })
    }

    Ok(blocktivity::Stats{stats: res})
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

