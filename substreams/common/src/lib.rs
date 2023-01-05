use substreams_antelope_core::pb::antelope::{Block, ActionTraces, DbOps};
use substreams::errors::Error;

#[substreams::handlers::map]
fn map_action_traces(block: Block) -> Result<ActionTraces, Error> {
    let mut action_traces = vec![];  

    for trx in block.clone().all_transaction_traces() {
        for trace in trx.action_traces.clone() {
            action_traces.push(trace);
        }
    }
    Ok(ActionTraces { action_traces })
}

#[substreams::handlers::map]
fn map_db_ops(block: Block) -> Result<DbOps, Error> {
    let mut db_ops = vec![];

    for trx in block.clone().all_transaction_traces() {
        for db_op in trx.db_ops.clone() {
            db_ops.push(db_op);
        }
    }
    Ok(DbOps { db_ops })
}
