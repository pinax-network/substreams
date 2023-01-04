// substream modules
use substreams_antelope_core::pb::antelope::{Block};
use substreams::errors::Error;

// local modules
mod pb;
use crate::pb::antelope::{ActionTraces, DbOps};

#[substreams::handlers::map]
fn map_action_traces(block: Block) -> Result<ActionTraces, Error> {
    let mut action_traces = vec![];  

    for trx in block.clone().all_transaction_traces() {
        // action traces
        for trace in &trx.action_traces {
            action_traces.push(trace);
        }
    }
    Ok(ActionTraces { action_traces })
}

#[substreams::handlers::map]
fn map_db_ops(block: Block) -> Result<DbOps, Error> {
    let mut db_ops = vec![];
    for trx in block.clone().all_transaction_traces() {
        for db_op in &trx.db_ops {
            db_ops.push(db_op);
        }
    }
    Ok(DbOps { db_ops })
}
