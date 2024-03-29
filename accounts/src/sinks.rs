use substreams::{log, proto};
use substreams_sink_kv::pb::sf::substreams::sink::kv::v1::KvOperations;
use substreams::errors::Error;
use substreams_sink_prometheus::{PrometheusOperations, Gauge};

use crate::accounts::Accounts;

#[substreams::handlers::map]
pub fn kv_out( accounts: Accounts) -> Result<KvOperations, Error> {
    let mut kv_ops: KvOperations = Default::default();

    let mut ordinal = 1;
    for account in accounts.accounts {
        log::debug!("account={:?}", account);
        let value = proto::encode(&account).unwrap();
        kv_ops.push_new(account.name, value, ordinal);
        ordinal += 1;
    }

    Ok(kv_ops)
}

#[substreams::handlers::map]
pub fn prom_out(accounts: Accounts) -> Result<PrometheusOperations, Error> {

    let mut prom_out = PrometheusOperations::default();
    let total = accounts.accounts.len() as f64;

    if total > 0.0 {
        log::info!("New accounts: {}", total);
        prom_out.push(Gauge::from("accounts_counter").add(total));
    }

    Ok(prom_out)
}
