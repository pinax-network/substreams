// standard modules
use std::collections::HashSet;
use serde_json;
use serde_json::{Value};

// substream modules
use substreams_antelope_core::pb::antelope::{Block};
use substreams::errors::Error;

// local modules
mod pb;
use crate::pb::eosio_token::{Action, Actions};

#[substreams::handlers::map]
fn map_actions(block: Block) -> Result<Actions, Error> {
    let mut actions = vec![];
    let events = HashSet::from(["create","issue","retire","transfer","open","close"]);

    for trx in block.clone().all_transaction_traces() {
        for trace in &trx.action_traces {
            let action = trace.action.as_ref().unwrap().clone();
            if !events.contains(action.name.as_str()) { continue; }

            actions.push(Action {
                block_num: block.number,
                timestamp: Some(block.header.as_ref().unwrap().timestamp.as_ref().unwrap().clone()),
                account: action.account,
                name: action.name,
                json_data: action.json_data,
            })
        }
    }
    Ok(Actions { actions })
}

#[substreams::handlers::map]
pub fn map_transfers(actions: Actions) -> Result<Actions, Error> {
    let mut response = vec![];

    for action in actions.actions {
        if action.name != "transfer" { continue; }
        response.push(action);
    }

    Ok(Actions { actions: response })
}

#[substreams::handlers::map]
pub fn map_transfers_eosio_token(actions: Actions) -> Result<Actions, Error> {
    let mut response = vec![];

    for action in actions.actions {
        if action.account != "eosio.token" { continue; }
        response.push(action);
    }

    Ok(Actions { actions: response })
}

#[substreams::handlers::map]
pub fn map_transfers_eosnationftw(actions: Actions) -> Result<Actions, Error> {
    let mut response = vec![];

    for action in actions.actions {
        if action.account != "eosio.token" { continue; }
        let data: Value = serde_json::from_str(action.json_data.as_str()).unwrap();
        if !(data["to"] == "eosnationftw" || data["from"] == "eosnationftw") { continue; }
        response.push(action);
    }

    Ok(Actions { actions: response })
}