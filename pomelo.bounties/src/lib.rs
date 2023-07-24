#[allow(dead_code)]
mod abi;
mod pb;

use substreams_antelope::Block;
use pb::antelope::pomelo::bounties::v1::{Actions, StateLog};

#[substreams::handlers::map]
fn map_actions(param_account: String, block: Block) -> Result<Actions, substreams::errors::Error> {
    Ok(Actions {
        statelogs: block.all_transaction_traces()
            .flat_map(|trx| trx.action_traces.iter())
            .filter(|trace| trace.action.as_ref().unwrap().name == "statelog")
            .filter(|trace| trace.action.as_ref().unwrap().account == param_account)
            .map(|trace|
                match abi::Statelog::try_from(trace.action.as_ref().unwrap().json_data.as_str()) {
                    Ok(data) =>
                        StateLog {
                            // trx information
                            trx_id: trace.transaction_id.clone(),
                            trx_index: trace.action_ordinal,

                            // action data
                            action: data.action,
                            state: data.status,
                            bounty_id: data.bounty_id,
                        },
                    Err(_) => panic!("Error parsing statelog action data"),
                }
            )
            .collect(),
    })
}