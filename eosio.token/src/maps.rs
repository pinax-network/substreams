use substreams::errors::Error;
use substreams_antelope::Block;

use crate::abi;
use crate::eosio_token::*;
use antelope::{Asset, Name, SymbolCode};

#[substreams::handlers::map]
fn map_accounts(block: Block) -> Result<Accounts, Error> {
    let mut items = vec![];
    for trx in block.all_transaction_traces() {
        for db_op in &trx.db_ops {
            if db_op.table_name != "accounts" {
                continue;
            }

            let contract = db_op.code.clone();
            let raw_primary_key = Name::from(db_op.primary_key.as_str()).value;
            let symcode = SymbolCode::from(raw_primary_key).to_string();
            let account = db_op.scope.clone();
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
fn map_stat(block: Block) -> Result<Stats, Error> {
    let mut items = vec![];
    for trx in block.all_transaction_traces() {
        for db_op in &trx.db_ops {
            if db_op.table_name != "stat" {
                continue;
            }

            let contract = db_op.code.clone();
            let raw_primary_key = Name::from(db_op.primary_key.as_str()).value;
            let symcode = SymbolCode::from(raw_primary_key).to_string();
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

    // TO-DO Yaro
    // Build query-params
    // "to=swap.defi,swap.rome&symcode=EOS"
    // all EOS symcode transfers to these two accounts
    // log::debug!("map_params: {:?}", accounts);
    // let from = params.split(";").collect::<Vec<&str>>()[0];

    for trx in block.all_transaction_traces() {
        // action traces
        for trace in &trx.action_traces {
            let action_trace = trace.action.as_ref().unwrap();
            if action_trace.account != trace.receiver {
                continue;
            }
            if action_trace.name != "transfer" {
                continue;
            }

            // if params == "all" || accounts.iter().any(|&i| i == transfer.from) || accounts.iter().any(|&i| i == transfer.to) {
            match abi::Transfer::try_from(action_trace.json_data.as_str()) {
                Ok(data) => {
                    let quantity = Asset::from(data.quantity.as_str());
                    let symcode = quantity.symbol.code().to_string();
                    let precision = quantity.symbol.precision().into();
                    let amount = quantity.amount;
                    let account = action_trace.account.clone();

                    // TO-DO Yaro
                    // filter by params
                    // prevent push if does not match query-param

                    response.push(TransferEvent {
                        // trace information
                        trx_id: trx.id.clone(),
                        action_ordinal: trace.action_ordinal,

                        // contract & scope
                        account,
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
