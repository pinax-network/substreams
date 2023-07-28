use antelope::Symbol;
use substreams::errors::Error;
use substreams::log;
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

    let filter_balance_lt = utils::create_i64_filter(params.as_str(), "balance_lt");
    let filter_balance_gt = utils::create_i64_filter(params.as_str(), "balance_gt");
    let filter_balance_lte = utils::create_i64_filter(params.as_str(), "balance_lte");
    let filter_balance_gte = utils::create_i64_filter(params.as_str(), "balance_gte");

    let filter_balance_delta_lt = utils::create_i64_filter(params.as_str(), "balance_delta_lt");
    let filter_balance_delta_gt = utils::create_i64_filter(params.as_str(), "balance_delta_gt");
    let filter_balance_delta_lte = utils::create_i64_filter(params.as_str(), "balance_delta_lte");
    let filter_balance_delta_gte = utils::create_i64_filter(params.as_str(), "balance_delta_gte");

    for trx in block.all_transaction_traces() {
        for db_op in &trx.db_ops {
            if db_op.table_name != "accounts" { continue; }

            let contract = db_op.code.clone();
            let raw_primary_key = Name::from(db_op.primary_key.as_str()).value;
            let symcode = SymbolCode::from(raw_primary_key);
            let account = db_op.scope.clone();

            // filter by params
            if !filter_account.is_empty() && !filter_account.contains(&account) { continue; }
            if !filter_symcode.is_empty() && !filter_symcode.contains(&symcode.to_string()) { continue; }
            if !filter_contract.is_empty() && !filter_contract.contains(&contract) { continue; }

            let old_data = abi::Account::try_from(db_op.old_data_json.as_str()).ok();
            let new_data = abi::Account::try_from(db_op.new_data_json.as_str()).ok();
            if old_data.is_none() && new_data.is_none() { continue; } // no data

            let old_balance = match &old_data {
                Some(data) => Some(Asset::from(data.balance.as_str())),
                None => None,
            };
            let new_balance = match &new_data {
                Some(data) => Some(Asset::from(data.balance.as_str())),
                None => None,
            };
            let precision = match new_balance.is_some() {
                true => new_balance.unwrap().symbol.precision(),
                false => old_balance.unwrap().symbol.precision(),
            };
            let sym = Symbol::from_precision(symcode, precision);
            let balance = match new_balance.is_some() {
                true => new_balance.unwrap(),
                false => Asset::from_amount(0, sym),
            };

            let balance_delta = match old_balance.is_some() {
                true => balance.amount - old_balance.unwrap().amount,
                false => balance.amount,
            };

            // filter by params
            if filter_balance_lt.is_some() && !(balance.amount < filter_balance_lt.unwrap()) { continue; }
            if filter_balance_gt.is_some() && !(balance.amount > filter_balance_gt.unwrap()) { continue; }
            if filter_balance_lte.is_some() && !(balance.amount <= filter_balance_lte.unwrap()) { continue; }
            if filter_balance_gte.is_some() && !(balance.amount >= filter_balance_gte.unwrap()) { continue; }

            if filter_balance_delta_lt.is_some() && !(balance_delta < filter_balance_delta_lt.unwrap()) { continue; }
            if filter_balance_delta_gt.is_some() && !(balance_delta > filter_balance_delta_gt.unwrap()) { continue; }
            if filter_balance_delta_lte.is_some() && !(balance_delta <= filter_balance_delta_lte.unwrap()) { continue; }
            if filter_balance_delta_gte.is_some() && !(balance_delta >= filter_balance_delta_gte.unwrap()) { continue; }

            items.push(Account {
                // trace information
                trx_id: trx.id.clone(),
                action_index: db_op.action_index,

                // contract & scope
                contract,
                symcode: symcode.to_string(),

                // payload
                account: account,
                balance: balance.to_string(),
                balance_delta,

                // extras
                precision: precision.into(),
                amount: balance.amount,
                value: utils::to_value(balance),
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

    let filter_supply_delta_lt = utils::create_i64_filter(params.as_str(), "supply_delta_lt");
    let filter_supply_delta_gt = utils::create_i64_filter(params.as_str(), "supply_delta_gt");
    let filter_supply_delta_lte = utils::create_i64_filter(params.as_str(), "supply_delta_lte");
    let filter_supply_delta_gte = utils::create_i64_filter(params.as_str(), "supply_delta_gte");

    for trx in block.all_transaction_traces() {
        for db_op in &trx.db_ops {
            if db_op.table_name != "stat" { continue; }

            let contract = db_op.code.clone();
            let raw_primary_key = Name::from(db_op.primary_key.as_str()).value;
            let symcode = SymbolCode::from(raw_primary_key);

            // filter by params
            if !filter_contract.is_empty() && !filter_contract.contains(&contract) { continue; }
            if !filter_symcode.is_empty() && !filter_symcode.contains(&symcode.to_string()) { continue; }
            if !filter_contract.is_empty() && !filter_contract.contains(&contract) { continue; }

            let old_data = abi::CurrencyStats::try_from(db_op.old_data_json.as_str()).ok();
            let new_data = abi::CurrencyStats::try_from(db_op.new_data_json.as_str()).ok();
            if old_data.is_none() && new_data.is_none() { continue; } // no data

            let old_supply = match &old_data {
                Some(data) => Some(Asset::from(data.supply.as_str())),
                None => None,
            };
            let new_supply = match &new_data {
                Some(data) => Some(Asset::from(data.supply.as_str())),
                None => None,
            };
            let precision = match new_supply.is_some() {
                true => new_supply.unwrap().symbol.precision(),
                false => old_supply.unwrap().symbol.precision(),
            };
            let sym = Symbol::from_precision(symcode, precision);
            let supply = match new_supply.is_some() {
                true => new_supply.unwrap(),
                false => Asset::from_amount(0, sym),
            };

            let supply_delta = match old_supply.is_some() {
                true => supply.amount - old_supply.unwrap().amount,
                false => supply.amount,
            };

            // filter by params
            if filter_supply_lt.is_some() && !(supply.amount < filter_supply_lt.unwrap()) { continue; }
            if filter_supply_gt.is_some() && !(supply.amount > filter_supply_gt.unwrap()) { continue; }
            if filter_supply_lte.is_some() && !(supply.amount <= filter_supply_lte.unwrap()) { continue; }
            if filter_supply_gte.is_some() && !(supply.amount >= filter_supply_gte.unwrap()) { continue; }

            if filter_supply_delta_lt.is_some() && !(supply_delta < filter_supply_delta_lt.unwrap()) { continue; }
            if filter_supply_delta_gt.is_some() && !(supply_delta > filter_supply_delta_gt.unwrap()) { continue; }
            if filter_supply_delta_lte.is_some() && !(supply_delta <= filter_supply_delta_lte.unwrap()) { continue; }
            if filter_supply_delta_gte.is_some() && !(supply_delta >= filter_supply_delta_gte.unwrap()) { continue; }

            // TO-DO fix in case of new_data is None
            let data = new_data.unwrap();

            items.push(Stat {
                // trace information
                trx_id: trx.id.clone(),
                action_index: db_op.action_index,

                // contract & scope
                contract,
                symcode: symcode.to_string(),

                // payload
                issuer: data.issuer,
                max_supply: data.max_supply,
                supply: supply.to_string(),
                supply_delta,

                // extras
                precision: precision.into(),
                amount: supply.amount,
                value: utils::to_value(supply),
            });
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

                    log::debug!("symcode: {:?}", symcode);

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
