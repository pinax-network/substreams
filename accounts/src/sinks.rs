use substreams::{log, proto};
use substreams_sink_kv::pb::kv::KvOperations;
use substreams::errors::Error;
use substreams_sink_prometheus::PrometheusOperations;

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

#[substreams::handlers::map]
pub fn prom_out(accounts: Accounts) -> Result<PrometheusOperations, Error> {

    let mut prom_out = PrometheusOperations::default();

    if accounts.accounts.len() > 0 {
        log::info!("New accounts: {}", accounts.accounts.len());
        prom_out.push_add("accounts_counter", accounts.accounts.len() as f64, vec![]);
    }

    Ok(prom_out)
}
