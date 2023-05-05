use substreams::errors::Error;
use substreams::log;
use substreams_antelope::Block;

use crate::abi;
use crate::eosio_token::*;
use antelope::{Asset, Name, SymbolCode};

#[substreams::handlers::map]
fn map_accounts(block: Block) -> Result<Accounts, Error> {
    let mut items = vec![];
    for trx in block.all_transaction_traces() {
        for db_op in &trx.db_ops {
            if db_op.table_name != "accounts" { continue; }

            let contract = db_op.code.clone();
            let raw_primary_key = Name::from(db_op.primary_key.as_str()).value;
            let symcode = SymbolCode::from(raw_primary_key).to_string();
            let account = db_op.scope.clone();
            let balance = abi::Account::try_from(db_op.new_data_json.as_str());

            match balance {
                Ok(data) => {
                    items.push(Account {
                        contract,
                        account,
                        symcode,
                        balance: data.balance,
                    });
                },
                Err(_) => continue,
            }
        }
    }
    Ok(Accounts{items})
}

#[substreams::handlers::map]
fn map_stat(block: Block) -> Result<Stats, Error> {
    let mut items = vec![];
    for trx in block.all_transaction_traces() {
        for db_op in &trx.db_ops {
            if db_op.table_name != "stat" { continue; }

            let contract = db_op.code.clone();
            let raw_primary_key = Name::from(db_op.primary_key.as_str()).value;
            let symcode = SymbolCode::from(raw_primary_key).to_string();
            let currency_stats = abi::CurrencyStats::try_from(db_op.new_data_json.as_str());

            match currency_stats {
                Ok(data) => {
                    items.push(Stat {
                        contract,
                        symcode,
                        issuer: data.issuer,
                        max_supply: data.max_supply,
                        supply: data.supply,
                    });
                },
                Err(_) => continue,
            }
        }
    }
    Ok(Stats{items})
}

#[substreams::handlers::map]
fn map_transfers(params: String, block: Block) -> Result<TransferEvents, Error> {
    let mut response = vec![];
    let accounts: Vec<&str> = params.split(";").collect();
    log::debug!("map_params: {:?}", accounts);

    for trx in block.all_transaction_traces() {
        // action traces
        for trace in &trx.action_traces {
            let action_trace = trace.action.as_ref().unwrap();
            if action_trace.account != trace.receiver { continue; }
            if action_trace.name != "transfer"  { continue; }

            let transfer = match abi::Transfer::try_from(action_trace.json_data.as_str()) {
                Ok(action) => action,
                Err(_) => continue,
            };


            if params == "all" || accounts.iter().any(|&i| i == transfer.from) || accounts.iter().any(|&i| i == transfer.to) {
                let quantity = Asset::from(transfer.quantity.as_str());
                response.push(TransferEvent {
                    // trace information
                    trx_id: trx.id.clone(),
                    action_ordinal: trace.action_ordinal,

                    // contract & scope
                    account: action_trace.account.clone(),
                    symcode: quantity.symbol.code().to_string(),

                    // payload
                    from: transfer.from,
                    to: transfer.to,
                    quantity: quantity.to_string(),
                    memo: transfer.memo,

                    // extras
                    precision: quantity.symbol.precision().into(),
                    amount: quantity.amount,
                });
            }
        }
    }
    Ok(TransferEvents { items: response })
}
