use substreams::errors::Error;
use substreams::log;
use substreams_antelope::pb::Block;
use substreams_entity_change::pb::entity::EntityChanges;

use crate::abi;
use crate::utils::from_dbop_to_entityop;

#[substreams::handlers::map]
pub fn entity_out(block: Block) -> Result<EntityChanges, Error> {
    let mut entity_changes: EntityChanges = Default::default();

    for trx in block.executed_transaction_traces() {
        for db_op in &trx.db_ops {
            if db_op.code == "app.pomelo" {
                match db_op.table_name.as_str() {
                    "grants" => {
                        let grant = abi::GrantsRow::try_from(db_op.new_data_json.as_str()).unwrap();
                        let op = from_dbop_to_entityop(&db_op.operation());
                        log::info!("Op: {:?}, Grant={:?}", op, grant);
                        entity_changes
                            .push_change("Grant", &grant.id, 1, op)
                            .change("id", grant.id)
                            .change("author_user_id", grant.author_user_id)
                            .change("funding_account", grant.funding_account)
                            .change("status", grant.status)
                            .change("accepted_tokens", grant.accepted_tokens)
                            .change("updated_at", grant.updated_at)
                            .change("trx_id", trx.id.clone());
                    }
                    "transfers" => {
                        let transfer = abi::TransfersRow::try_from(db_op.new_data_json.as_str()).unwrap();
                        let op = from_dbop_to_entityop(&db_op.operation());
                        let transfer_id = format!("{}-{}", transfer.round_id, transfer.transfer_id);
                        log::info!("Op: {:?}, Transfer={:?}", op, transfer);
                        entity_changes
                            .push_change("Transfer", &transfer_id, 1, op)
                            .change("user_id", transfer.user_id)
                            .change("from", transfer.from)
                            .change("to", transfer.to)
                            .change("quantity", transfer.ext_quantity.quantity)
                            .change("contract", transfer.ext_quantity.contract)
                            .change("fee", transfer.fee)
                            .change("value", transfer.value.to_string())
                            .change("memo", transfer.memo)
                            .change("project_id", transfer.project_id)
                            .change("created_at", transfer.created_at)
                            .change("trx_id", transfer.trx_id);
                    }
                    _ => continue,
                }
            }
        }
    }
    Ok(entity_changes)
}
