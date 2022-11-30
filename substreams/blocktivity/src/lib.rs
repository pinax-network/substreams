// mod abi;
mod pb;

use substreams::{prelude::*, log};
use crate::pb::{blocktivity};

/// Extracts the blocktivity from each block
#[substreams::handlers::map]
fn map_blocktivity(blk: substreams_antelope_core::pb::antelope::Block) -> Result<blocktivity::Blocktivity, substreams::errors::Error> {
    Ok(blocktivity::Blocktivity {
        block_num: blk.number,
        timestamp: Some(blk.header.as_ref().unwrap().timestamp.clone().unwrap()),
        trx_count: blk.transaction_traces_count(),
        act_count: blk.executed_total_action_count(),
        cpu_usage_us: blk.executed_transaction_traces().map(|trx| trx.receipt.as_ref().unwrap().cpu_usage_micro_seconds).sum(),
        net_usage_words: blk.executed_transaction_traces().map(|trx| trx.receipt.as_ref().unwrap().net_usage_words).sum(),
    })
}

#[substreams::handlers::store]
fn store_trx_count(blocktivity: blocktivity::Blocktivity, s: StoreAddInt64) {
    log::debug!("block {}: adding transaction count {}", blocktivity.block_num, blocktivity.trx_count);
    s.add(blocktivity.block_num as u64, get_key(blocktivity.block_num.clone()), blocktivity.trx_count.clone() as i64)
}

#[substreams::handlers::store]
fn store_act_count(blocktivity: blocktivity::Blocktivity, s: StoreAddInt64) {
    log::debug!("block {}: adding action count {}", blocktivity.block_num, blocktivity.act_count);
    s.add(blocktivity.block_num as u64, get_key(blocktivity.block_num.clone()), blocktivity.act_count.clone() as i64)
}

#[substreams::handlers::store]
fn store_cpu_usage_us(blocktivity: blocktivity::Blocktivity, s: StoreAddInt64) {
    log::debug!("block {}: adding cpu_usage_us {}", blocktivity.block_num, blocktivity.cpu_usage_us);
    s.add(blocktivity.block_num as u64, get_key(blocktivity.block_num.clone()), blocktivity.cpu_usage_us.clone() as i64)
}

#[substreams::handlers::store]
fn store_net_usage_words(blocktivity: blocktivity::Blocktivity, s: StoreAddInt64) {
    log::debug!("block {}: adding net_usage_words {}", blocktivity.block_num, blocktivity.net_usage_words);
    s.add(blocktivity.block_num as u64, get_key(blocktivity.block_num.clone()), blocktivity.net_usage_words.clone() as i64)
}

fn get_key(block_num: u32) -> String {
    return if block_num % 7200 == 0 {
        block_num.clone().to_string()
    } else {
        (block_num.clone() - block_num.rem_euclid(7200)).to_string()
    };
}

#[test]
fn block_num_key_0() {
    assert_eq!("0", get_key(0));
}

#[test]
fn block_num_key_2() {
    assert_eq!("0", get_key(2));
}

#[test]
fn block_num_key_7200() {
    assert_eq!("7200", get_key(7200));
}

#[test]
fn block_num_key_8000() {
    assert_eq!("7200", get_key(8000));
}

#[test]
fn block_num_key_14400() {
    assert_eq!("14400", get_key(14400));
}

