use substreams::prelude::*;
use substreams::errors::Error;
use substreams::store::{StoreSetProto};

// local modules
mod abi;
mod pb;
mod keyer;
use crate::pb::eosio_token::{Account, CurrencyStats};
use crate::pb::common::{ActionTraces, DatabaseOperations};

#[substreams::handlers::map]
fn map_action_traces(action_traces: ActionTraces) -> Result<ActionTraces, Error> {
    let mut response = vec![];

    for action_trace in action_traces.action_traces {
        // validate ABIs
        let matched = match action_trace.name.as_str() {
            "transfer" => abi::is_transfer(&action_trace.json_data),
            "issue" => abi::is_issue(&action_trace.json_data),
            "create" => abi::is_create(&action_trace.json_data),
            "close" => abi::is_close(&action_trace.json_data),
            "open" => abi::is_open(&action_trace.json_data),
            "retire" => abi::is_retire(&action_trace.json_data),
            _ => continue,
        };
        if !matched { continue; }
        response.push(action_trace);
    }
    Ok(ActionTraces { action_traces: response })
}

#[substreams::handlers::map]
fn map_db_ops(db_ops: DatabaseOperations) -> Result<DatabaseOperations, Error> {
    let mut response = vec![];

    for db_op in db_ops.db_ops {
        // validate ABIs
        let matched = match db_op.table_name.as_str() {
            "accounts" => abi::is_accounts(&db_op.new_data),
            "stat" => abi::is_stat(&db_op.new_data),
            _ => continue,
        };
        if !matched { continue; }
        response.push(db_op);
    }
    Ok(DatabaseOperations { db_ops: response })
}

#[substreams::handlers::store]
fn store_accounts(db_ops: DatabaseOperations, output: StoreSetProto<Account>) {
    for db_op in db_ops.db_ops {
        if db_op.table_name != "accounts" { continue; }

        // key: code:scope:primary_key (contract:owner:symcode)
        let contract = db_op.code;
        let owner = db_op.scope;
        let symcode = db_op.primary_key;
        let key = keyer::store_accounts(&symcode, &contract, &owner,);
        
        // delete if empty
        if db_op.new_data.is_empty() {
            return output.delete_prefix(1, &key);
        }

        // update store
        output.set(db_op.action_index.into(), key, &Account {
            account: db_op.new_data,
        });
    }
}

#[substreams::handlers::store]
fn store_stat(db_ops: DatabaseOperations, output: StoreSetProto<CurrencyStats>) {
    for db_op in db_ops.db_ops {
        if db_op.table_name != "stat" { continue; }

        let contract = db_op.code;
        let symcode = db_op.scope;
        let key = keyer::store_stat(&symcode, &contract);
        
        // delete if empty
        if db_op.new_data.is_empty() {
            return output.delete_prefix(1, &key);
        }

        // update store
        output.set(db_op.action_index.into(), key, &CurrencyStats {
            currency_stats: db_op.new_data,
        });
    }
}