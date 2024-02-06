use pb::bitcoin::txo::Txo;
use substreams::{
    errors::Error,
    store::{StoreAddInt64, StoreGetInt64},
};
use substreams::{log, prelude::*};
use substreams_bitcoin::pb::btc::v1::Block;

mod pb;

#[substreams::handlers::store]
pub fn store_txo(block: Block, s: StoreAddInt64) {
    log::info!("txs: {}", block.transactions().count());
    for (i, tx) in block.transactions().enumerate() {
        log::info!("txos: {}", tx.vout.len());
        s.add(i as u64, "txo", tx.vout.len() as i64);
    }
}

#[substreams::handlers::map]
pub fn map_txo(txo_store: StoreGetInt64) -> Result<Txo, Error> {
    let txo_count = txo_store.get_last("txo").unwrap();
    log::info!("txo_count: {}", txo_count);
    Ok(Txo { txo_count })
}
