
use std::collections::HashMap;
use substreams::errors::Error;



use substreams_sink_prometheus::{PrometheusOperations, Counter};


use crate::eosio_cpu::{TransferEvents};

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