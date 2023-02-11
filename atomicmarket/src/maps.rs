use substreams::errors::Error;
use substreams_antelope::{Block, ActionTraces};

#[substreams::handlers::map]
fn map_actions(block: Block) -> Result<ActionTraces, Error> {
    let mut action_traces = vec![];

    for trx in block.clone().all_transaction_traces() {
        for trace in &trx.action_traces {
            let action_trace = trace.action.as_ref().unwrap().clone();
            if action_trace.account != "atomicmarket" { continue; }
            if action_trace.account != trace.receiver { continue; }
            action_traces.push(trace.clone());
        }
    }
    Ok(ActionTraces { action_traces })
}
