use substreams::log;
use substreams::prelude::*;
use substreams_antelope::pb::antelope::Block;

use crate::keyer;
use crate::eosio_token::*;

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
            
            log::debug!("store_accounts: {:?}", db_op.new_data);
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