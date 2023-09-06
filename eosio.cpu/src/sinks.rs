use antelope::Asset;
use std::collections::HashMap;
use substreams::errors::Error;
use substreams_entity_change::pb::entity::{entity_change, EntityChanges};
use substreams_sink_kv::pb::sf::substreams::sink::kv::v1::KvOperations;
use substreams_database_change::pb::database::{table_change, DatabaseChanges};
use substreams_sink_prometheus::{PrometheusOperations, Counter, Gauge, Histogram, Summary};
use substreams::log;

use crate::eosio_cpu::{Accounts, Stats, TransferEvents};

#[substreams::handlers::map]
pub fn prom_out(map_transfers: TransferEvents) -> Result<PrometheusOperations, Error> {

    let mut prom_out = PrometheusOperations::default();
    //log::info!("map_transfers={:?}", map_transfers);

    for transfer in map_transfers.items {

        let transfer_label = HashMap::from([
            ("producer".to_string(), transfer.producer.to_string()),
            //("from".to_string(), transfer.from.to_string())
        ]);

        prom_out.push(Counter::from("cpu_usage").with(transfer_label.clone()).add(transfer.cpu_usage as f64));
        prom_out.push(Counter::from("net_usage").with(transfer_label.clone()).add(transfer.net_usage as f64));
        prom_out.push(Counter::from("tx_count").with(transfer_label.clone()).inc());

    }
    Ok(prom_out)
}