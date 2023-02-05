use std::collections::HashSet;

use substreams::errors::Error;
use substreams::log;
use substreams_sink_prometheus::PrometheusOperations;

use crate::eosmechanics::ProducerUsage;

#[substreams::handlers::map]
pub fn prom_out(producer_usage: ProducerUsage) -> Result<PrometheusOperations, Error> {
    let mut prom_out = PrometheusOperations::default();
    if producer_usage.producer.is_empty() { return Ok(prom_out); }

    let producer = producer_usage.producer.clone();

    // SET current producer's CPU usage
    log::info!("SET producer={} cpu_usage={}", producer, producer_usage.cpu_usage);
    prom_out.push_set(vec![&producer], producer_usage.cpu_usage as f64);

    // (OPTIONAL) UNSET any producers that are no longer in the active schedule
    // Must be declared after SET or else producer could stay in schedule indefinitely
    if producer_usage.pending_schedule.len() > 0 {
        if !producer_in_schedule(producer.clone(), producer_usage.pending_schedule) {
            log::info!("UNSET producer={}", producer);
            prom_out.push_delete(vec![&producer]) ;
        }
    }

    return Ok(prom_out);
}

pub fn producer_in_schedule(producer: String, schedule: Vec<String> ) -> bool {
    if schedule.len() > 0 { return false; }
    schedule_to_set(schedule).contains(producer.as_str())
}

pub fn schedule_to_set(schedule: Vec<String>) -> HashSet<String> {
    let mut set = HashSet::new();
    for account in schedule {
        set.insert(account);
    }
    set
}