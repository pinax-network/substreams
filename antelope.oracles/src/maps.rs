use antelope::Symbol;
use substreams::log;
use substreams::errors::Error;
use substreams_antelope::Block;

use crate::abi;
use crate::antelope_oracles::*;
use crate::utils;
use antelope::{Asset, Name, SymbolCode};

#[substreams::handlers::map]
fn map_prices(params: String, block: Block) -> Result<Prices, Error> {
    let mut response = vec![];

    for trx in block.all_transaction_traces() {
        for db_op in &trx.db_ops {
            let contract = db_op.code.clone();
            let raw_primary_key = Name::from(db_op.primary_key.as_str()).value;
            let symcode = SymbolCode::from(raw_primary_key);
            let account = db_op.scope.clone();

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
                            last_update: price.last_update
                        });
                    }
                    Err(e) => {
                        log::debug!("error={:?}", e);
                        continue;
                    },
                }
            }
        }
    }

    Ok(Prices { prices: response })
}

#[substreams::handlers::map]
fn map_datapoints(params: String, block: Block) -> Result<Datapoints, Error> {
    let mut response = vec![];

    for trx in block.all_transaction_traces() {
        for db_op in &trx.db_ops {
            let contract = db_op.code.clone();
            let pair = db_op.scope.clone();

            if contract == "delphioracle" && db_op.table_name == "datapoints" {
                //log::debug!("new_data_json={:?}", db_op.new_data_json);

                for trace in &trx.action_traces {
                    let action_trace = trace.action.as_ref().unwrap();
                    //log::debug!("action_trace={:?}", action_trace);
                }

                match abi::Datapoints::try_from(db_op.new_data_json.as_str()) {
                    Ok(datapoint) => {
                        //log::debug!("pair={:?}", pair);
                        //log::debug!("datapoint={:?}", datapoint);
                        response.push(Datapoint {
                            id: datapoint.id,
                            median: datapoint.median,
                            owner: datapoint.owner.as_str().to_string(),
                            timestamp: datapoint.timestamp.as_str().to_string(),
                            value: datapoint.value
                        });
                    }
                    Err(e) => {
                        log::debug!("error={:?}", e);
                        continue;
                    },
                }
            }
        }
    }

    Ok(Datapoints { datapoints: response })
}

// Work In Progress: Extract pairs information from `pairs` table of `delphioracle`
/*#[substreams::handlers::map]
fn map_pairs(params: String, block: Block) -> Result<Pairs, Error> {
    let mut response = vec![];

    // query-params
    /*let filter_from = utils::create_filters(params.as_str(), "from");
    let filter_to = utils::create_filters(params.as_str(), "to");
    let filter_symcode = utils::create_filters(params.as_str(), "symcode");
    let filter_contract = utils::create_filters(params.as_str(), "contract");
    let filter_to_or_from = utils::create_filters(params.as_str(), "to_or_from");

    let filter_quantity_lt = utils::create_i64_filter(params.as_str(), "quantity_lt");
    let filter_quantity_gt = utils::create_i64_filter(params.as_str(), "quantity_gt");
    let filter_quantity_lte = utils::create_i64_filter(params.as_str(), "quantity_lte");
    let filter_quantity_gte = utils::create_i64_filter(params.as_str(), "quantity_gte");*/

    for trx in block.all_transaction_traces() {
        for db_op in &trx.db_ops {
            let contract = db_op.code.clone();
    
            if contract == "delphioracle" {// && db_op.table_name == "pairs" {
                log::debug!("contract={:?} / table_name={:?} / is datapoints ? {:?}", contract, db_op.table_name, db_op.table_name == "datapoints");
                log::debug!("new_data_json={:?}", db_op.new_data_json);

                response.push(Pair {
                            active: false,
                            bounty_awarded: false,
                            bounty_edited_by_custodians: false,
                            proposer: "test".to_string(),
                            name: "test".to_string(),
                            bounty_amount: "test".to_string(),
                            approving_custodians: vec![],
                            approving_oracles: vec![],
                            base_symbol: "test/test".to_string(),
                            base_type: 1,
                            base_contract: "test_contract".to_string(),
                            quote_symbol: "test".to_string(),
                            quote_type: 1,
                            quote_contract: "test_contract".to_string(),
                            quoted_precision: 8
                        });
                /*match abi::Pairs::try_from(db_op.new_data_json.as_str()) {
                    Ok(pair) => {
                        response.push(Pair {
                            active: false,
                            bounty_awarded: false,
                            bounty_edited_by_custodians: false,
                            proposer: "test".to_string(),
                            name: "test".to_string(),
                            bounty_amount: "test".to_string(),
                            approving_custodians: vec![],
                            approving_oracles: vec![],
                            base_symbol: "test/test".to_string(),
                            base_type: 1,
                            base_contract: "test_contract".to_string(),
                            quote_symbol: "test".to_string(),
                            quote_type: 1,
                            quote_contract: "test_contract".to_string(),
                            quoted_precision: 8
                        });
                    }
                    Err(_) => continue,
                }*/
            }
        }

       /* // action traces
        for trace in &trx.action_traces {
            let action_trace = trace.action.as_ref().unwrap();
            if action_trace.account != trace.receiver { continue; }
            if action_trace.name != "transfer" { continue; }

            match abi::Transfer::try_from(action_trace.json_data.as_str()) {
                Ok(data) => {
                    let quantity = Asset::from(data.quantity.as_str());
                    let symcode = quantity.symbol.code().to_string();
                    let precision = quantity.symbol.precision().into();
                    let amount = quantity.amount;
                    let contract = action_trace.account.clone();

                    // filter by params
                    if !filter_from.is_empty() && !filter_from.contains(&data.from) { continue; }
                    if !filter_to.is_empty() && !filter_to.contains(&data.to) { continue; }
                    if !filter_symcode.is_empty() && !filter_symcode.contains(&symcode) { continue; }
                    if !filter_contract.is_empty() && !filter_contract.contains(&contract) { continue; }
                    if !filter_to_or_from.is_empty() && !(filter_to_or_from.contains(&data.to) || filter_to_or_from.contains(&data.from)) { continue; }
                    if filter_quantity_lt.is_some() && !(quantity.amount < filter_quantity_lt.unwrap()) { continue; }
                    if filter_quantity_gt.is_some() && !(quantity.amount > filter_quantity_gt.unwrap()) { continue; }
                    if filter_quantity_lte.is_some() && !(quantity.amount <= filter_quantity_lte.unwrap()) { continue; }
                    if filter_quantity_gte.is_some() && !(quantity.amount >= filter_quantity_gte.unwrap()) { continue; }

                    response.push(TransferEvent {
                        // trace information
                        trx_id: trx.id.clone(),
                        action_ordinal: trace.action_ordinal,

                        // contract & scope
                        contract,
                        action: action_trace.name.clone(),
                        symcode,

                        // payload
                        from: data.from,
                        to: data.to,
                        quantity: data.quantity,
                        memo: data.memo,

                        // extras
                        precision,
                        amount,
                        value: utils::to_value(quantity),
                    });
                }
                Err(_) => continue,
            }
        }*/
    }

    Ok(Pairs { pairs: response })
}*/

/*#[substreams::handlers::map]
fn map_quotes(params: String, block: Block) -> Result<Quotes, Error> {
    let mut response = vec![];

    Ok(Quotes { quotes: response })
}*/