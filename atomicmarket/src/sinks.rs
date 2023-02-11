use substreams::prelude::*;
use std::collections::HashMap;

use substreams::errors::Error;
use substreams_sink_prometheus::{PrometheusOperations, Gauge};

#[substreams::handlers::map]
pub fn prom_out(store_new_sales: Deltas<DeltaInt64>) -> Result<PrometheusOperations, Error> {
    let mut prom_out = PrometheusOperations::default();

    for delta in store_new_sales.deltas {
        let labels = HashMap::from([("collection_name".to_string(), delta.key)]);
        let mut gauge = Gauge::from("newsale").with(labels);
        prom_out.push(gauge.set(delta.new_value as f64));
    }
    Ok(prom_out)
}