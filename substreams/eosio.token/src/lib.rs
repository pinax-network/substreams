use substreams::prelude::*;
use substreams::errors::Error; 
use substreams_antelope::pb::antelope::Block;

// local modules
mod abi;
mod pb;
mod keyer;
use crate::pb::eosio_token::{Account, CurrencyStats, TransferEvent, TransferEvents};
use eosio::{Asset};

#[substreams::handlers::map]
fn map_transfers(block: Block) -> Result<TransferEvents, Error> {
    let mut response = vec![];

    for trx in block.clone().all_transaction_traces() {
        // action traces
        for trace in &trx.action_traces {
            let action_trace = trace.action.as_ref().unwrap().clone();
            if action_trace.account != trace.receiver { continue; }
            if action_trace.name != "transfer"  { continue; }
            if !abi::is_transfer(&action_trace.json_data) { continue; }

            // parse action data
            let data: abi::Transfer = serde_json::from_str(&action_trace.json_data).unwrap();
            let quantity = data.quantity.parse::<Asset>().unwrap();

            response.push(TransferEvent {
                // trace information
                block_num: block.number,
                timestamp: block.header.clone().unwrap().timestamp,
                trx_id: trx.id.clone(),
                action_ordinal: trace.action_ordinal.clone(),

                // action data
                account: action_trace.account,
                symcode: quantity.symbol.code().to_string(),
                precision: quantity.symbol.precision().into(),
                from: data.from,
                to: data.to,
                amount: quantity.amount,
                memo: data.memo,
            });
        }
    }
    Ok(TransferEvents { items: response })
}

#[substreams::handlers::store]
fn store_accounts(block: Block, output: StoreSetProto<Account>) {
    for trx in block.clone().all_transaction_traces() {
        for db_op in &trx.db_ops {
            if db_op.table_name != "accounts" { continue; }

            // key: code:scope:primary_key (owner:contract::SYMCODE)
            let contract = &db_op.code;
            let owner = &db_op.scope;
            let symcode = &db_op.primary_key;
            let key = keyer::store_accounts(&owner, &contract, &symcode);

            // update store
            output.set(1, key, &Account {
                account: db_op.new_data.clone(),
            });
        }
    }
}

#[substreams::handlers::store]
fn store_stat(block: Block, output: StoreSetProto<CurrencyStats>) {
    for trx in block.clone().all_transaction_traces() {
        for db_op in &trx.db_ops {
            if db_op.table_name != "stat" { continue; }

            let contract = &db_op.code;
            let symcode = &db_op.scope;
            let key = keyer::store_stat(&contract, &symcode);

            // update store
            output.set(1, key, &CurrencyStats {
                currency_stats: db_op.new_data.clone(),
            });
        }
    }
}