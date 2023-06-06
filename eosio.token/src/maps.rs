use substreams::errors::Error;
use substreams_antelope::Block;
use substreams::log;

use crate::abi;
use crate::eosio_token::*;
use crate::utils;
use antelope::{Asset, Name, SymbolCode};

#[substreams::handlers::map]
fn map_accounts(params: String, block: Block) -> Result<Accounts, Error> {
    let mut items = vec![];

    // query-params
    let filter_account = utils::create_filters(params.as_str(), "account");
    let filter_symcode = utils::create_filters(params.as_str(), "symcode");
    let filter_contract = utils::create_filters(params.as_str(), "contract");
    let filter_balancelt = utils::create_filters(params.as_str(), "balance_lt");
    let filter_balancegt = utils::create_filters(params.as_str(), "balance_gt");

    for trx in block.all_transaction_traces() {
        for db_op in &trx.db_ops {
            if db_op.table_name != "accounts" { continue; }

            let contract = db_op.code.clone();
            let raw_primary_key = Name::from(db_op.primary_key.as_str()).value;
            let symcode = SymbolCode::from(raw_primary_key).to_string();
            let account = db_op.scope.clone();

            // filter by params
            if !filter_account.is_empty() && !filter_account.contains(&account) { continue; }
            if !filter_symcode.is_empty() && !filter_symcode.contains(&symcode) { continue; }
            if !filter_contract.is_empty() && !filter_contract.contains(&contract) { continue; }

            match abi::Account::try_from(db_op.new_data_json.as_str()) {
                Ok(data) => {
                    // filter by params
                    if !filter_balancelt.is_empty() && !utils::is_balance_lt(&data.balance, &filter_balancelt) { continue; }
                    if !filter_balancegt.is_empty() && !utils::is_balance_gt(&data.balance, &filter_balancegt) { continue; }

                    items.push(Account {
                        contract,
                        account,
                        symcode,
                        balance: data.balance,
                    });
                }
                Err(_) => continue,
            }
        }
    }
    Ok(Accounts { items })
}

#[substreams::handlers::map]
fn map_stat(params: String, block: Block) -> Result<Stats, Error> {
    let mut items = vec![];

    // query-params
    let filter_symcode = utils::create_filters(params.as_str(), "symcode");
    let filter_contract = utils::create_filters(params.as_str(), "contract");
    let filter_amountlt = utils::create_filters(params.as_str(), "amount_lt");
    let filter_amountgt = utils::create_filters(params.as_str(), "amount_gt");

    for trx in block.all_transaction_traces() {
        for db_op in &trx.db_ops {
            if db_op.table_name != "stat" { continue; }

            let contract = db_op.code.clone();
            let raw_primary_key = Name::from(db_op.primary_key.as_str()).value;
            let symcode = SymbolCode::from(raw_primary_key).to_string();

            // filter by params
            if !filter_symcode.is_empty() && !filter_symcode.contains(&symcode) { continue; }
            if !filter_contract.is_empty() && !filter_contract.contains(&contract) { continue; }


            match abi::CurrencyStats::try_from(db_op.new_data_json.as_str()) {
                Ok(data) => {
                    let supply = Asset::from(data.supply.as_str());
                    let precision = supply.symbol.precision().into();

                    // filter by params
                    if !filter_amountlt.is_empty() && !utils::is_amount_lt(supply.amount, &filter_amountlt) { continue; }
                    if !filter_amountgt.is_empty() && !utils::is_amount_gt(supply.amount, &filter_amountgt) { continue; }
                    

                    items.push(Stat {
                        // trace information
                        trx_id: trx.id.clone(),
                        action_index: db_op.action_index,

                        // contract & scope
                        contract,
                        symcode,

                        // payload
                        issuer: data.issuer,
                        max_supply: data.max_supply,
                        supply: data.supply,

                        // extras
                        precision,
                        amount: supply.amount,
                        value: supply.amount as f64 / 10_i64.pow(precision) as f64,

                    });
                }
                Err(_) => continue,
            }
        }
    }
    Ok(Stats { items })
}

#[substreams::handlers::map]
fn map_transfers(params: String, block: Block) -> Result<TransferEvents, Error> {
    let mut response = vec![];

    // query-params
    let filter_from = utils::create_filters(params.as_str(), "from");
    let filter_to = utils::create_filters(params.as_str(), "to");
    let filter_symcode = utils::create_filters(params.as_str(), "symcode");
    let filter_contract = utils::create_filters(params.as_str(), "contract");
    let filter_tofrom = utils::create_filters(params.as_str(), "to_or_from");
    let filter_amountlt = utils::create_filters(params.as_str(), "amount_lt");
    let filter_amountgt = utils::create_filters(params.as_str(), "amount_gt");

    for trx in block.all_transaction_traces() {
        // action traces
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
                    if !filter_tofrom.is_empty() && !(filter_tofrom.contains(&data.to) || filter_tofrom.contains(&data.from)) { continue; }
                    // if amount is greater than filter_amountlt, skip
                    if !filter_amountgt.is_empty() && !utils::is_amount_gt(amount, &filter_amountgt) { continue; }
                    // if amount is less than filter_amountlt, skip
                    if !filter_amountlt.is_empty() && !utils::is_amount_lt(amount, &filter_amountlt) { continue; }

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
                        value: amount as f64 / 10_i64.pow(precision) as f64,
                    });
                }
                Err(_) => continue,
            }
        }
    }
    Ok(TransferEvents { items: response })
}

#[substreams::handlers::map]
fn map_balance_delta(params: String, block: Block) -> Result<Accounts, Error> {
    
    let mut items = vec![];

    let mut balance_changes: Vec<f64> = Vec::new();
    for trx in block.all_transaction_traces() {
        for db_op in &trx.db_ops {            
            let table_name = db_op.clone().table_name;
            if table_name != "accounts" { continue; }

            let contract = db_op.code.clone();
            let raw_primary_key = Name::from(db_op.primary_key.as_str()).value;
            let symcode = SymbolCode::from(raw_primary_key).to_string();
            let account = db_op.scope.clone();

            match abi::Account::try_from(db_op.old_data_json.as_str()) {
                Ok(data) => {
                    let old_balance_numeric: String = data.balance.chars().filter(|c| c.is_numeric() || *c == '.').collect();
                    let old_balance: Result<f64, _> = old_balance_numeric.parse();
                    if old_balance.is_err() { continue; }

                    balance_changes.push(old_balance.unwrap());
                }
                Err(_) => { continue;}
            }

            match abi::Account::try_from(db_op.new_data_json.as_str()) {
                Ok(data) => {
                    let new_balance_numeric: String = data.balance.chars().filter(|c| c.is_numeric() || *c == '.').collect();
                    let new_balance: Result<f64, _> = new_balance_numeric.parse();
                    if new_balance.is_err() { continue; }
                    
                    balance_changes.push(new_balance.unwrap());

                    let balance_delta = balance_changes.last().unwrap() - balance_changes.first().unwrap();

                    items.push(Account {
                        contract,
                        account,
                        symcode,
                        balance: balance_delta.to_string(),
                    });
                }
                Err(_) => continue,
            }
        }
    }

    Ok(Accounts { items })
}