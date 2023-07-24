#[allow(dead_code)]
mod abi;
mod pb;

use substreams_antelope::Block;
use pb::antelope::pomelo::bounties::v1::{Actions, StateLog};

const TRACKED_CONTRACT: &str = "d.w.pomelo";


#[substreams::handlers::map]
fn map_actions(block: Block) -> Result<Actions, substreams::errors::Error> {
    let mut actions = Actions::default();

    for trx in block.all_transaction_traces() {
        // action traces
        for trace in &trx.action_traces {
            let action_trace = trace.action.as_ref().unwrap();
            if action_trace.account != trace.receiver { continue; }
            if action_trace.account != TRACKED_CONTRACT { continue; }
            if action_trace.name != "statelog" { continue; }

            match abi::Statelog::try_from(action_trace.json_data.as_str()) {
                Ok(data) => {
                    actions.statelogs.push(StateLog {
                        // trace information
                        trx_id: trx.id.clone(),
                        trx_index: trace.action_ordinal,

                        // action data
                        action: data.action,
                        state: data.status,
                        bounty_id: data.bounty_id,
                    });
                }
                Err(_) => panic!("Error parsing action data"),
            }
        }
    }
    Ok(actions)
}