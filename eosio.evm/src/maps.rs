use substreams::errors::Error;
use substreams::log;
use substreams_antelope::pb::Block;

//use serde_json::Result;
use serde_json::Value;

use crate::abi;
use crate::eosio_evm;

#[substreams::handlers::map]
fn map_pushtx(block: Block) -> Result<eosio_evm::Pushtxs, Error> {
    let mut events = vec![];
    for trx in block.executed_transaction_traces() {
        for trace in &trx.action_traces {
            let action_trace = trace.action.as_ref().unwrap();
            if action_trace.account != "eosio.evm" {
                continue;
            }
            if action_trace.name != "pushtx" {
                continue;
            }

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
    for trx in block.executed_transaction_traces() {
        for db_op in &trx.db_ops {
            let contract = db_op.code.clone();

            if contract == "eosio.evm" && db_op.table_name == "balances" {
                match serde_json::from_str::<Value>(&db_op.new_data_json) {
                    Ok(parsed_value) => {
                        let owner = (parsed_value.get("owner").unwrap().to_string()).trim_matches('"').to_string();
                        let balances = (parsed_value.get("balance").unwrap().to_string()).trim_matches('"').to_string();

                        match serde_json::from_str::<Value>(&balances) {
                            Ok(parsed_value_balance) => {
                                let balancesym = (parsed_value_balance.get("balance").unwrap().to_string())
                                    .trim_matches('"')
                                    .to_string();
                                let dust = (parsed_value_balance.get("dust").unwrap().to_string())
                                    .trim_matches('"')
                                    .to_string();

                                let parts: Vec<&str> = balancesym.split_whitespace().collect();
                                if parts.len() == 2 {
                                    let balance = parts[0].parse::<f64>().unwrap_or(0.0);
                                    let sym = String::from(parts[1]);

                                    events.push(eosio_evm::Pushbalance {
                                        miner: owner,
                                        balance: balance,
                                        symcode: sym,
                                        dust: dust.parse::<i64>().ok().unwrap(),
                                    });
                                }
                            }
                            Err(e) => {
                                log::debug!("Failed to parse JSON: {}", e);
                            }
                        }
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
