use substreams::errors::Error;
use substreams_antelope::{Block, BlockHeader, ActionTraces, BlockRootMerkle, TransactionTraces, DbOps };

use crate::utils;

#[substreams::handlers::map]
fn map_block_header(block: Block) -> Result<BlockHeader, Error> {
    Ok(block.header.unwrap())
}

#[substreams::handlers::map]
fn map_blockroot_merkle(block: Block) -> Result<BlockRootMerkle, Error> {
    Ok(block.blockroot_merkle.unwrap() )
}

#[substreams::handlers::map]
fn map_transaction_traces(params: String, block: Block) -> Result<TransactionTraces, Error> {
    let mut transaction_traces = vec![];

    let filter_contract = utils::create_filters(params.as_str(), "contract");
    let filter_action = utils::create_filters(params.as_str(), "action");
    let filter_receiver = utils::create_filters(params.as_str(), "receiver");

    for trx in block.all_transaction_traces() {
        if !filter_contract.is_empty() || !filter_action.is_empty() {
            let mut has_contract = false;
            let mut has_action = false;
            let mut has_receiver = false;
            for trace in &trx.action_traces {
                let action = trace.action.as_ref().unwrap();

                // filter by params
                if !filter_contract.is_empty() && filter_contract.contains(&action.account) { has_contract = true; }
                if !filter_action.is_empty() && filter_action.contains(&action.name) { has_action = true; }
                if !filter_receiver.is_empty() && filter_receiver.contains(&trace.receiver) { has_receiver = true; }
                if has_contract || has_action || has_receiver { break; }
            }
            // don't include transaction
            if !has_contract && !has_action && !has_receiver { continue; }
        }
        transaction_traces.push(trx.clone());
    }
    Ok(TransactionTraces { transaction_traces })
}

#[substreams::handlers::map]
fn map_action_traces(params: String, block: Block) -> Result<ActionTraces, Error> {
    let mut action_traces = vec![];

    let filter_contract = utils::create_filters(params.as_str(), "contract");
    let filter_action = utils::create_filters(params.as_str(), "action");
    let filter_receiver = utils::create_filters(params.as_str(), "receiver");

    for trx in block.all_transaction_traces() {
        for trace in &trx.action_traces {
            let action = trace.action.as_ref().unwrap();

            // filter by params
            if !filter_contract.is_empty() && !filter_contract.contains(&action.account) { continue; }
            if !filter_action.is_empty() && !filter_action.contains(&action.name) { continue; }
            if !filter_receiver.is_empty() && !filter_receiver.contains(&trace.receiver) { continue; }

            action_traces.push(trace.clone());
        }
    }
    Ok(ActionTraces { action_traces })
}

#[substreams::handlers::map]
fn map_db_ops(params: String, block: Block) -> Result<DbOps, Error> {
    let mut db_ops = vec![];

    let filter_contract = utils::create_filters(params.as_str(), "contract");
    let filter_table = utils::create_filters(params.as_str(), "table");

    for trx in block.all_transaction_traces() {
        for db_op in &trx.db_ops {

            // filter by params
            if !filter_contract.is_empty() && !filter_contract.contains(&db_op.code) { continue; }
            if !filter_table.is_empty() && !filter_table.contains(&db_op.table_name) { continue; }

            db_ops.push(db_op.clone());
        }
    }
    Ok(DbOps { db_ops })
}
