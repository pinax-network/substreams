use substreams::errors::Error;
use substreams_antelope::Block;

use crate::eosio_evm;
use crate::abi;

#[substreams::handlers::map]
fn map_pushtx(block: Block) -> Result<eosio_evm::Pushtxs, Error> {
    let mut events = vec![];
    for trx in block.all_transaction_traces() {
        for trace in &trx.action_traces {
            let action_trace = trace.action.as_ref().unwrap();
            if action_trace.account != "eosio.evm" { continue; }
            if action_trace.name != "pushtx" { continue; }

            match abi::Pushtx::try_from(action_trace.json_data.as_str()) {
                Ok(pushtx) => {
                    events.push(eosio_evm::Pushtx {
                        trx_id: trx.id.to_string(),
                        miner: pushtx.miner,
                        rlptx: pushtx.rlptx,
                    });
                }
                Err(_) => continue,
            }
        }
    }
    Ok(eosio_evm::Pushtxs { events })
}
