mod pb;

use crate::pb::eosio_token;
use std::collections::HashSet;

#[substreams::handlers::map]
fn map_actions(blk: substreams_antelope_core::pb::antelope::Block) -> Result<eosio_token::Actions, substreams::errors::Error> {
    let mut actions = vec![];
    let events = HashSet::from(["create","issue","retire","transfer","open","close"]);

    for trx in blk.clone().all_transaction_traces() {
        for trace in &trx.action_traces {
            let action = trace.action.as_ref().unwrap().clone();
            if !events.contains(action.name.as_str()) { continue; }

            actions.push(eosio_token::Action {
                block_num: blk.number,
                timestamp: Some(blk.header.as_ref().unwrap().timestamp.as_ref().unwrap().clone()),
                account: action.account,
                name: action.name,
                json_data: action.json_data,
            })
        }
    }
    Ok(eosio_token::Actions { actions })
}
