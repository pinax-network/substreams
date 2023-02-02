use substreams::errors::Error; 
use substreams::log;
use substreams_antelope::Block;

use crate::abi;
use crate::eosio_token::*;
use eosio::Asset;
use antelope::{Name, SymbolCode};

#[substreams::handlers::map]
fn map_accounts(block: Block) -> Result<Block, Error> {
    for trx in block.clone().all_transaction_traces() {
        for db_op in &trx.db_ops {
            if db_op.table_name != "accounts" { continue; }

            let raw_primary_key = Name::from(db_op.primary_key.as_str()).value;            
            let symcode = SymbolCode::from(raw_primary_key).to_string();
            let account = db_op.scope.clone();

            log::debug!("account={} symcode={} new_data_json={} old_data_json={}", account, symcode, db_op.new_data_json, db_op.old_data_json );
        }
    }
    Ok(Default::default())
}

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
