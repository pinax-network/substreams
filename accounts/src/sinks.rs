use substreams::{log, proto};
use substreams_sink_kv::pb::kv::KvOperations;
use substreams::errors::Error;

use crate::accounts::Accounts;

#[substreams::handlers::map]
pub fn kv_out( accounts: Accounts) -> Result<KvOperations, Error> {
    let mut kv_ops: KvOperations = Default::default();

    for account in accounts.accounts {
        log::debug!("account={:?}", account);
        let value = proto::encode(&account).unwrap();
        kv_ops.push_new(account.name, value, 1);
    }

    Ok(kv_ops)
}

// TO-DO - Prometheus Sink (counter for newly created accounts)
// use EOSMechanics as example