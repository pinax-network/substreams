// substream modules
use substreams_antelope_core::pb::antelope::{Block};
use substreams::errors::Error;

// local modules
mod pb;
use crate::pb::common::{ActionTrace, ActionTraces, DatabaseOperation, DatabaseOperations};

#[substreams::handlers::map]
fn map_action_traces(block: Block) -> Result<ActionTraces, Error> {
    let mut action_traces = vec![];  

    for trx in block.clone().all_transaction_traces() {
        // action traces
        for trace in &trx.action_traces {
            let trace_action = trace.action.as_ref().unwrap().clone();
            action_traces.push(ActionTrace {
                // trace information
                block_num: block.number,
                timestamp: Some(block.header.as_ref().unwrap().timestamp.as_ref().unwrap().clone()),
                trx_id: trx.id.clone(),
                action_ordinal: trace.action_ordinal.clone(),

                // action
                account: trace_action.account,
                receiver: trace.receiver.clone(),
                name: trace_action.name,

                // action data
                json_data: trace_action.json_data,
            })
        }
    }
    Ok(ActionTraces { action_traces })
}


#[substreams::handlers::map]
fn map_db_ops(block: Block) -> Result<DatabaseOperations, Error> {
    let mut db_ops = vec![];
    for trx in block.clone().all_transaction_traces() {
        // table deltas
        for db_op in &trx.db_ops {

            db_ops.push(DatabaseOperation {
                // trace information
                block_num: block.number,
                timestamp: Some(block.header.as_ref().unwrap().timestamp.as_ref().unwrap().clone()),
                trx_id: trx.id.clone(),
                action_index: db_op.action_index,

                // database operation
                code: db_op.code.clone(),
                table_name: db_op.table_name.clone(),
                scope: db_op.scope.clone(),
                primary_key: db_op.primary_key.clone(),

                // table data
                old_data: db_op.old_data.clone(),
                new_data: db_op.new_data.clone(),
            })
        }
    }
    Ok(DatabaseOperations { db_ops })
}
