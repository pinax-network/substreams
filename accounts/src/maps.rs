use substreams::log;
use substreams::errors::Error;
use substreams_antelope::Block;

use crate::abi;
use crate::accounts;

/// Extracts new account events from the contract
#[substreams::handlers::map]
fn map_accounts(block: Block) -> Result<accounts::Accounts, Error> {
    let mut accounts = vec![];

    for trx in block.clone().all_transaction_traces() {
        for trace in &trx.action_traces {
            let action = trace.action.as_ref().unwrap();

            // parse new account actions
            if action.account == "eosio" && action.name == "newaccount" {
                if let Ok(params) = action.json_data.parse::<abi::NewAccount>() {
                    log::debug!("newaccount={:?}", params);
                    let creator = accounts::Creator {
                        creator: params.creator.clone(),
                        service: None,
                        authorizations: vec![],
                    };
                    let owner = accounts::Authority::from(params.owner);
                    let active = accounts::Authority::from(params.active);
                    accounts.push(accounts::Account {
                        creator: Some(creator),
                        name: params.name,
                        owner: Some(owner),
                        active: Some(active),
                        timestamp: block.header.as_ref().unwrap().timestamp.clone(),
                        trx_id: trace.transaction_id.clone(),
                        block_num: block.number,
                        ..Default::default()
                    });
                }
            }

            // parse buy RAM actions (normally one transaction per new account created)
            // TODO: instead, get the RAM usage and CPU/NET staked from dbOps (this will also cover buyram) BLOCKED: need to replay the blockchain to get decoded JSONs for dbOps
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
    Ok(accounts::Accounts { accounts })
}
