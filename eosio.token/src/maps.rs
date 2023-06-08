use substreams::errors::Error;
use substreams_antelope::Block;

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

    let filter_old_balance_lt = utils::create_i64_filter(params.as_str(), "old_balance_lt");
    let filter_old_balance_gt = utils::create_i64_filter(params.as_str(), "old_balance_gt");
    let filter_old_balance_lte = utils::create_i64_filter(params.as_str(), "old_balance_lte");
    let filter_old_balance_gte = utils::create_i64_filter(params.as_str(), "old_balance_gte");

    let filter_new_balance_lt = utils::create_i64_filter(params.as_str(), "new_balance_lt");
    let filter_new_balance_gt = utils::create_i64_filter(params.as_str(), "new_balance_gt");
    let filter_new_balance_lte = utils::create_i64_filter(params.as_str(), "new_balance_lte");
    let filter_new_balance_gte = utils::create_i64_filter(params.as_str(), "new_balance_gte");

    let filter_delta_balance_lt = utils::create_i64_filter(params.as_str(), "delta_balance_lt");
    let filter_delta_balance_gt = utils::create_i64_filter(params.as_str(), "delta_balance_gt");
    let filter_delta_balance_lte = utils::create_i64_filter(params.as_str(), "delta_balance_lte");
    let filter_delta_balance_gte = utils::create_i64_filter(params.as_str(), "delta_balance_gte");

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

            let old_data = abi::Account::try_from(db_op.old_data_json.as_str()).ok();
            let new_data = abi::Account::try_from(db_op.new_data_json.as_str()).ok();
            if old_data.is_none() && new_data.is_none() { continue; } // no accounts data

            let old_balance_str: Option<String> = match old_data {
                Some(data) => Some(data.balance.as_str().into()),
                None => None,
            };
            let new_balance_str: Option<String> = match new_data {
                Some(data) => Some(data.balance.as_str().into()),
                None => None,
            };
            let old_balance = match &old_balance_str {
                Some(data) => Some(Asset::from(data.as_str())),
                None => None,
            };
            let new_balance = match &new_balance_str {
                Some(data) => Some(Asset::from(data.as_str())),
                None => None,
            };
            let old_amount = match old_balance {
                Some(data) => Some(data.amount),
                None => None,
            };
            let new_amount = match new_balance {
                Some(data) => Some(data.amount),
                None => None,
            };
            let old_value = match old_balance {
                Some(data) => Some(utils::to_value(data)),
                None => None,
            };
            let new_value = match new_balance {
                Some(data) => Some(utils::to_value(data)),
                None => None,
            };
            let precision = match old_balance.is_some() {
                true => old_balance.unwrap().symbol.precision(),
                false => new_balance.unwrap().symbol.precision(),
            };
            let delta_amount = match new_amount.is_some() {
                true => match old_amount.is_some() {
                    true => Some(new_amount.unwrap() - old_amount.unwrap()),
                    false => new_amount,
                },
                false => Some(old_amount.unwrap() * -1),
            };

            // filter by params
            if old_balance.is_some() && filter_old_balance_lt.is_some() && !(old_balance.unwrap().amount < filter_old_balance_lt.unwrap()) { continue; }
            if old_balance.is_some() && filter_old_balance_gt.is_some() && !(old_balance.unwrap().amount > filter_old_balance_gt.unwrap()) { continue; }
            if old_balance.is_some() && filter_old_balance_lte.is_some() && !(old_balance.unwrap().amount <= filter_old_balance_lte.unwrap()) { continue; }
            if old_balance.is_some() && filter_old_balance_gte.is_some() && !(old_balance.unwrap().amount >= filter_old_balance_gte.unwrap()) { continue; }

            if new_balance.is_some() && filter_new_balance_lt.is_some() && !(new_balance.unwrap().amount < filter_new_balance_lt.unwrap()) { continue; }
            if new_balance.is_some() && filter_new_balance_gt.is_some() && !(new_balance.unwrap().amount > filter_new_balance_gt.unwrap()) { continue; }
            if new_balance.is_some() && filter_new_balance_lte.is_some() && !(new_balance.unwrap().amount <= filter_new_balance_lte.unwrap()) { continue; }
            if new_balance.is_some() && filter_new_balance_gte.is_some() && !(new_balance.unwrap().amount >= filter_new_balance_gte.unwrap()) { continue; }

            if delta_amount.is_some() && filter_delta_balance_lt.is_some() && !(delta_amount.unwrap() < filter_delta_balance_lt.unwrap()) { continue; }
            if delta_amount.is_some() && filter_delta_balance_gt.is_some() && !(delta_amount.unwrap() > filter_delta_balance_gt.unwrap()) { continue; }
            if delta_amount.is_some() && filter_delta_balance_lte.is_some() && !(delta_amount.unwrap() <= filter_delta_balance_lte.unwrap()) { continue; }
            if delta_amount.is_some() && filter_delta_balance_gte.is_some() && !(delta_amount.unwrap() >= filter_delta_balance_gte.unwrap()) { continue; }

            items.push(Account {
                // trace information
                trx_id: trx.id.clone(),
                action_index: db_op.action_index,

                // contract & scope
                contract,
                symcode,

                // payload
                account: account,
                old_balance: old_balance_str,
                new_balance: new_balance_str,

                // extras
                precision: precision.into(),
                old_amount,
                old_value,
                new_amount,
                new_value,
                delta_amount,
            });
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
    let filter_supply_lt = utils::create_i64_filter(params.as_str(), "supply_lt");
    let filter_supply_gt = utils::create_i64_filter(params.as_str(), "supply_gt");
    let filter_supply_lte = utils::create_i64_filter(params.as_str(), "supply_lte");
    let filter_supply_gte = utils::create_i64_filter(params.as_str(), "supply_gte");

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

                    // filter by params
                    if filter_supply_lt.is_some() && !(supply.amount < filter_supply_lt.unwrap()) { continue; }
                    if filter_supply_gt.is_some() && !(supply.amount > filter_supply_gt.unwrap()) { continue; }
                    if filter_supply_lte.is_some() && !(supply.amount <= filter_supply_lte.unwrap()) { continue; }
                    if filter_supply_gte.is_some() && !(supply.amount >= filter_supply_gte.unwrap()) { continue; }

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
                        value: utils::to_value(supply),
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
    let filter_to_or_from = utils::create_filters(params.as_str(), "to_or_from");
    let filter_quantity_lt = utils::create_i64_filter(params.as_str(), "quantity_lt");
    let filter_quantity_gt = utils::create_i64_filter(params.as_str(), "quantity_gt");
    let filter_quantity_lte = utils::create_i64_filter(params.as_str(), "quantity_lte");
    let filter_quantity_gte = utils::create_i64_filter(params.as_str(), "quantity_gte");

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
        }
    }
    Ok(TransferEvents { items: response })
}
