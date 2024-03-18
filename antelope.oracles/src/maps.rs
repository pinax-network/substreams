use substreams::errors::Error;
use substreams::log;
use substreams_antelope::pb::Block;

use crate::abi;
use crate::antelope_oracles::*;
use antelope::{Name, SymbolCode};

#[substreams::handlers::map]
fn map_prices(_params: String, block: Block) -> Result<Prices, Error> {
    let mut response = vec![];

    for trx in block.executed_transaction_traces() {
        for db_op in &trx.db_ops {
            let contract = db_op.code.clone();
            let raw_primary_key = Name::from(db_op.primary_key.as_str()).value;
            let _symcode = SymbolCode::from(raw_primary_key);
            let _account = db_op.scope.clone();

            if contract == "oracle.defi" && db_op.table_name == "prices" {
                //log::debug!("contract={:?} / table_name={:?}", contract, db_op.table_name);
                //log::debug!("raw_primary_key={:?} / symcode={:?} / account={:?}", raw_primary_key, symcode, account);
                //log::debug!("new_data_json={:?}", db_op.new_data_json);

                /*for trace in &trx.action_traces {
                    let action_trace = trace.action.as_ref().unwrap();
                    log::debug!("action_trace={:?}", action_trace);
                }*/

                match abi::Price::try_from(db_op.new_data_json.as_str()) {
                    Ok(price) => {
                        //log::debug!("price={:?}", price);
                        response.push(Price {
                            id: price.id,
                            contract: price.contract,
                            coin: price.coin,
                            precision: price.precision,
                            acc_price: price.acc_price,
                            last_price: price.last_price,
                            avg_price: price.avg_price,
                            last_update: price.last_update,
                        });
                    }
                    Err(e) => {
                        log::debug!("error={:?}", e);
                        continue;
                    }
                }
            }
        }
    }

    Ok(Prices { prices: response })
}

#[substreams::handlers::map]
fn map_quotes(_params: String, block: Block) -> Result<Quotes, Error> {
    let mut response = vec![];

    for trx in block.executed_transaction_traces() {
        for db_op in &trx.db_ops {
            let contract = db_op.code.clone();
            let pair = db_op.scope.clone();

            if contract == "delphioracle" && db_op.table_name == "datapoints" {
                //log::debug!("new_data_json={:?}", db_op.new_data_json);

                match abi::Datapoints::try_from(db_op.new_data_json.as_str()) {
                    Ok(datapoint) => {
                        //log::debug!("pair={:?}", pair);
                        //log::debug!("datapoint={:?}", datapoint);
                        response.push(Quote {
                            pair,
                            value: Some(Datapoint {
                                id: datapoint.id,
                                median: datapoint.median,
                                owner: datapoint.owner.as_str().to_string(),
                                timestamp: datapoint.timestamp.as_str().to_string(),
                                value: datapoint.value,
                            }),
                        });
                    }
                    Err(e) => {
                        log::debug!("error={:?}", e);
                        continue;
                    }
                }
            }
        }
    }

    Ok(Quotes { quotes: response })
}
