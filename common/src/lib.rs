
use substreams::{prelude::*, log};
use substreams::errors::Error;
use substreams_antelope::{Block, BlockHeader, ActionTraces, BlockRootMerkle, TransactionTraces, DbOps };

#[substreams::handlers::map]
fn map_block_header(block: Block) -> Result<BlockHeader, Error> {
    Ok(block.header.unwrap())
}

#[substreams::handlers::map]
fn map_blockroot_merkle(block: Block) -> Result<BlockRootMerkle, Error> {
    Ok(block.blockroot_merkle.unwrap() )
}

#[substreams::handlers::map]
fn map_transaction_traces(block: Block) -> Result<TransactionTraces, Error> {
    let mut transaction_traces = vec![];  

    for trx in block.clone().all_transaction_traces() {
        transaction_traces.push(trx.clone());
    }
    Ok(TransactionTraces { transaction_traces })
}

#[substreams::handlers::map]
fn map_action_traces(block: Block) -> Result<ActionTraces, Error> {
    let mut action_traces = vec![];  

    for trx in block.clone().all_transaction_traces() {
        for trace in trx.action_traces.clone() {
            action_traces.push(trace);
        }
    }
    Ok(ActionTraces { action_traces })
}

#[substreams::handlers::map]
fn map_db_ops(block: Block) -> Result<DbOps, Error> {
    let mut db_ops = vec![];

    for trx in block.clone().all_transaction_traces() {
        for db_op in trx.db_ops.clone() {
            db_ops.push(db_op);
        }
    }
    Ok(DbOps { db_ops })
}

#[substreams::handlers::store]
fn store_timestamp(block: Block, s: StoreSetInt64) {
    let timestamp = block.header.unwrap().timestamp.unwrap().seconds.to_string();
    let block_number = &(block.number as i64);
    log::debug!("storing timestamp {} at block {}", timestamp, block_number);
    s.set(1, timestamp, block_number)
}
