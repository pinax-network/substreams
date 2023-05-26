use substreams::errors::Error;
use substreams_antelope::Block;
use substreams::log;

use crate::abi;
use crate::eosio_evm::*;
use crate::utils;
use antelope::{Asset, Name, SymbolCode};

#[substreams::handlers::map]
fn map_accounts(params: String, block: Block) -> Result<Accounts, Error> {
    let mut items = vec![];

    log::debug!("map_accounts");

    // query-params
    let filter_account = utils::create_filters(params.as_str(), "account");
    let filter_symcode = utils::create_filters(params.as_str(), "symcode");
    let filter_contract = utils::create_filters(params.as_str(), "contract");

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

    log::debug!("map_stat");

    // query-params
    let filter_symcode = utils::create_filters(params.as_str(), "symcode");
    let filter_contract = utils::create_filters(params.as_str(), "contract");

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
                        precision: supply.symbol.precision().into(),
                        amount: supply.amount,
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

    log::debug!("map_transfers");

    // query-params
    let filter_from = utils::create_filters(params.as_str(), "from");
    let filter_to = utils::create_filters(params.as_str(), "to");
    let filter_symcode = utils::create_filters(params.as_str(), "symcode");
    let filter_contract = utils::create_filters(params.as_str(), "contract");
    let filter_tofrom = utils::create_filters(params.as_str(), "to_or_from");

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

                    response.push(TransferEvent {
                        // trace information
                        trx_id: trx.id.clone(),
                        action_ordinal: trace.action_ordinal,

                        // contract & scope
                        contract,
                        symcode,

                        // payload
                        from: data.from,
                        to: data.to,
                        quantity: data.quantity,
                        memo: data.memo,

                        // extras
                        precision,
                        amount,
                    });
                }
                Err(_) => continue,
            }
        }
    }
    Ok(TransferEvents { items: response })
}
