use substreams::{log};
use substreams::errors::Error;
use substreams_antelope::{Block};
use substreams_entity_change::pb::entity::{entity_change::Operation, EntityChanges};

use crate::abi::SetGrant;

#[substreams::handlers::map]
pub fn entity_out( block: Block) -> Result<EntityChanges, Error> {
    let mut entity_changes: EntityChanges = Default::default();

    for trx in block.clone().all_transaction_traces() {
        for trace in &trx.action_traces {
            let action_trace = trace.action.as_ref().unwrap().clone();
            if action_trace.account != "app.pomelo" { continue; }
            if action_trace.name == "setgrant" {
                let data: SetGrant = serde_json::from_str(&action_trace.json_data).unwrap();
                log::info!("data={:?}", data);
                entity_changes.push_change("Grant", &data.project_id, 1, Operation::Create)
                    .change::<&str, String>("author_id", data.author_id)
                    .change::<&str, String>("project_id", data.project_id)
                    .change::<&str, String>("funding_account", data.funding_account)
                    .change::<&str, String>("accepted_tokens", data.accepted_tokens.join(","));
            }
        }
    }
    Ok( entity_changes )
}