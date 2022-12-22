// standard modules
use std::collections::HashSet;

// substream modules
use substreams_antelope_core::pb::antelope::{Block};
use substreams::errors::Error;

// local modules
mod abi;
mod pb;
use crate::pb::actions::{Action, Actions};

pub const ACTIONS: [&str; 6] = [
    "create",
    "issue",
    "retire",
    "transfer",
    "open",
    "close",
];

#[substreams::handlers::map]
fn map_actions(block: Block) -> Result<Actions, Error> {
    let mut actions = vec![];
    let filter_by = HashSet::from(ACTIONS);

    for trx in block.clone().all_transaction_traces() {
        for trace in &trx.action_traces {
            let action = trace.action.as_ref().unwrap().clone();
            if !filter_by.contains(action.name.as_str()) { continue; }
            if trace.receiver != action.account { continue; } // skip extra receivers

            // validate ABIs
            let name = action.name;
            let json_data = action.json_data;
            if name == "transfer" && abi::is_transfer(&json_data).is_err() { continue; }
            if name == "issue" && abi::is_issue(&json_data).is_err() { continue; }
            if name == "create" && abi::is_create(&json_data).is_err() { continue; }
            if name == "close" && abi::is_close(&json_data).is_err() { continue; }
            if name == "open" && abi::is_open(&json_data).is_err() { continue; }
            if name == "retire" && abi::is_retire(&json_data).is_err() { continue; }

            actions.push(Action {
                block_num: block.number,
                timestamp: Some(block.header.as_ref().unwrap().timestamp.as_ref().unwrap().clone()),
                transaction_id: trace.transaction_id.clone(),
                account: action.account,
                name,
                json_data,
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

// #[substreams::handlers::store]
// fn store_net_usage_words(actions: Actions, s: StoreAddInt64) {
//     log::debug!("block {}: adding net_usage_words {}", blocktivity.block_num, blocktivity.net_usage_words);
//     s.add(1, get_key(blocktivity.block_num.clone()).to_string(), blocktivity.net_usage_words.clone() as i64)
// }