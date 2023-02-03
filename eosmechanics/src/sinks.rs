use std::collections::HashSet;

use substreams::errors::Error;
use substreams::log;

use crate::eosmechanics::ProducerUsage;
use crate::prometheus::PrometheusMetrics;

#[substreams::handlers::map]
pub fn prom_out(producer_usage: ProducerUsage) -> Result<PrometheusMetrics, Error> {
    let producer = producer_usage.producer.clone();

    // UNSET any producers that are no longer in the active schedule
    if producer_usage.pending_schedule.len() > 0 {
        if !producer_in_schedule(producer.clone(), producer_usage.pending_schedule) {
            log::info!("UNSET producer={}", producer);
        }
    }
    // SET current producer's CPU usage
    log::info!("SET producer={} cpu_usage={}", producer, producer_usage.cpu_usage);

    return Ok(PrometheusMetrics::default());
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