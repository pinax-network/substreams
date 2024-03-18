use substreams::errors::Error;
use substreams::log;
use substreams_antelope::pb::Block;

use crate::abi;
use crate::accounts;

/// Extracts new account events from the contract
#[substreams::handlers::map]
fn map_accounts(block: Block) -> Result<accounts::Accounts, Error> {
    let mut accounts = vec![];

    for trx in block.executed_transaction_traces() {
        for trace in &trx.action_traces {
            let action = trace.action.as_ref().unwrap();

            // parse new account actions
            if action.account == "eosio" && action.name == "newaccount" {
                if let Ok(params) = abi::Newaccount::try_from(action.json_data.as_str()) {
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
            // TODO: instead, get the RAM usage and CPU/NET staked from dbOps (uncomment below) BLOCKED: need to replay blockchain to get dbOps
            if action.account == "eosio" && action.name == "buyrambytes" {
                if let Ok(params) = abi::Buyrambytes::try_from(action.json_data.as_str()) {
                    for acc in accounts.iter_mut() {
                        log::debug!("buyrambytes={:?}", params);
                        if acc.name == params.receiver {
                            acc.ram_bytes += params.bytes
                        }
                    }
                }
            }
        }

        // for db_op in &trx.db_ops {
        //     if db_op.code == "eosio" && db_op.table_name == "userres" {
        //         log::info!("db_op={:?}", db_op);
        //         if let Ok(params) = abi::UserResources::try_from(db_op.new_data_json.as_str()) {
        //             for acc in accounts.iter_mut() {
        //                 if acc.name == params.owner {
        //                     acc.ram_bytes = params.ram_bytes as u32
        //                 }
        //             }
        //         }
        //     }
        // }
    }
    Ok(accounts::Accounts { accounts })
}
