use substreams_antelope::ActionTraces;
use std::collections::HashMap;

use substreams::errors::Error;
use substreams_sink_prometheus::{PrometheusOperations, Counter};
use crate::abi;

#[substreams::handlers::map]
pub fn prom_out(action_traces: ActionTraces) -> Result<PrometheusOperations, Error> {
    let mut prom_out = PrometheusOperations::default();
    for action_trace in action_traces.action_traces {
        match abi::parse_lognewsale(&action_trace) {
            Some(new_sale) => {
                let labels = HashMap::from([("collection_name".to_string(), new_sale.collection_name)]);
                let mut counter = Counter::from("newsale").with(labels);
                prom_out.push(counter.add(new_sale.listen_price as f64));
            },
            None => (),
        }
    }
    Ok(prom_out)
}