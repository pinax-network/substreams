use substreams::errors::Error;
use substreams_antelope::Block;
use substreams::log;

use antelope::{Asset, Name, SymbolCode};

//use serde_json::Result;
use serde_json::Value;

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

#[substreams::handlers::map]
fn map_balances(block: Block) -> Result<eosio_evm::Pushbalances, Error> {
    let mut events = vec![];
    for trx in block.all_transaction_traces() {
        for db_op in &trx.db_ops {
            let contract = db_op.code.clone();

            if contract == "eosio.evm" &&  db_op.table_name == "balances" {
                log::debug!("new_data_json={:?}", db_op.new_data_json);
                
                match serde_json::from_str::<Value>(&db_op.new_data_json) {
                    Ok(parsed_value) => {
                        log::debug!("Parsed JSON: {:?}", parsed_value);

                        events.push(eosio_evm::Pushbalance {
                            miner: "testing12345".to_string(),
                            balance: 1.0001,
                            symcode: "EOS".to_string(),
                        });

                    }
                    Err(e) => {
                        log::debug!("Failed to parse JSON: {}", e);
                    }
                }

            }
       }
    }
    Ok(eosio_evm::Pushbalances { events })
}
