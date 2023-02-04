use substreams::log;
use substreams::errors::Error;
use substreams_antelope::Block;

use crate::abi;
use crate::accounts::{Accounts, Account, Creator, Authority};

/// Extracts new account events from the contract
#[substreams::handlers::map]
fn map_accounts(block: Block) -> Result<Accounts, Error> {
    let mut accounts = vec![];

    for trx in block.clone().all_transaction_traces() {
        for trace in &trx.action_traces {
            let action = trace.action.as_ref().unwrap().clone();

            // parse new account actions
            if action.account == "eosio" && action.name == "newaccount" {
                if let Ok(params) = action.json_data.parse::<abi::NewAccount>() {
                    log::debug!("newaccount={:?}", params);
                    let creator = Some(Creator::default()); // TO-DO

                    accounts.push(Account {
                        // action details
                        creator,
                        name: params.name,
                        owner: Some(Authority::default()), // TO-DO
                        active: Some(Authority::default()), // TO-DO

                        // transaction details
                        timestamp: block.header.clone().unwrap().timestamp,
                        trx_id: trace.transaction_id.clone(),
                        block_num: block.clone().number,

                        // account details
                        ram_bytes: 0, // will be updated later
                        stake_net_quantity: 0, // will be updated later
                        stake_cpu_quantity: 0, // will be updated later
                        transfer: false, // will be updated later
                    });
                }
            }

            // parse buy RAM actions (normally one transaction per new account created)
            if action.account == "eosio" && action.name == "buyrambytes" {
                if let Ok(params) = action.json_data.parse::<abi::BuyRamBytes>() {
                    if let Some(last) = accounts.last_mut() {
                        log::debug!("buyrambytes={:?}", params);
                        if last.name == params.receiver {
                            last.ram_bytes += params.bytes;
                        }
                    }
                }
            }
        }
    }
    Ok(Accounts { accounts })
}
