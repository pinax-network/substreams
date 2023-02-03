#[path = "pb/antelope.accounts.v1.rs"]
#[allow(dead_code)]
mod accounts;
mod abi;

// use substreams::{log};
use substreams::{proto, store::StoreGet};
use substreams_sink_kv::pb::kv::KvOperations;
use substreams::errors::Error;
use substreams::{prelude::*};

/// Extracts new account events from the contract
#[substreams::handlers::map]
fn map_accounts(blk: substreams_antelope::pb::Block) -> Result<accounts::Accounts, substreams::errors::Error> {
    let mut accounts = vec![];

    for trx in blk.clone().all_transaction_traces() {
        let mut ordinal = 0_u32;
        for trace in &trx.action_traces {
            let action = trace.action.as_ref().unwrap().clone();
            // log::debug!("Found action {}::{} action in block #{}", action.account, action.name, blk.number);
            if action.account == "eosio" && action.name == "newaccount" {
                if let Ok(params) = action.json_data.parse::<abi::NewAccount>() {
                    accounts.push(accounts::Account {
                        name: params.name,
                        creator: params.creator,
                        timestamp: Some(blk.header.as_ref().unwrap().timestamp.as_ref().unwrap().clone()),
                        ram_bytes: 0,
                        owner_public_key: params.owner.keys[0].key.clone(),
                        active_public_key: params.active.keys[0].key.clone(),
                        trx_id: trace.transaction_id.clone(),
                        ordinal,
                        counter: 0,
                    });
                    ordinal += 1;
                }
            }

            if action.account == "eosio" && action.name == "buyrambytes" {
                if let Ok(params) = action.json_data.parse::<abi::BuyRamBytes>() {
                    if let Some(last) = accounts.last_mut() {
                        if last.name == params.receiver {
                            last.ram_bytes = params.bytes;
                        }
                    }
                }
            }
        }
    }
    Ok(accounts::Accounts { accounts })
}

#[substreams::handlers::store]
fn store_account_counter(accs: accounts::Accounts, s: substreams::store::StoreAddInt64) {
    let mut i = 1;
    for _ in accs.accounts {
        substreams::store::StoreAdd::add(&s, i, "account_counter", 1);
        i += 1;
    }
}


#[substreams::handlers::map]
pub fn kv_out(
    accounts: accounts::Accounts,
    store_account_count: substreams::store::StoreGetInt64
) -> Result<KvOperations, Error> {
    let mut kv_ops: KvOperations = Default::default();
    let mut i = 1;
    for mut account in accounts.accounts {
        account.counter = store_account_count.get_at(i, "account_counter").unwrap();
        let key = account.name.clone();
        let value = proto::encode(&account).unwrap();
        let ordinal = account.ordinal as u64;
        kv_ops.push_new(key, value, ordinal);
        substreams::log::debug!("Account {} minted #{} in trx: {}", account.name, account.counter, account.trx_id);
        i += 1;
    }

    Ok(kv_ops)
}

