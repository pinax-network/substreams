use std::collections::HashMap;

use substreams::errors::Error;
use substreams::log;
use substreams_sink_prometheus::{PrometheusOperations, Counter, Gauge};

use crate::eosmechanics::{ProducerUsage, ScheduleChange};

#[substreams::handlers::map]
pub fn prom_out(
    producer_usage: ProducerUsage,
    schedule_change: ScheduleChange
) -> Result<PrometheusOperations, Error> {

    let mut prom_out = PrometheusOperations::default();
    let producer = producer_usage.producer.clone();

    // SET producer CPU usage
    if !producer.is_empty() {
        log::info!("SET producer_usage={:?}", producer_usage);
        let labels = HashMap::from([("producer".to_string(), producer)]);
        let mut gauge = Gauge::from("producer_usage").with(labels);
        prom_out.push(gauge.set(producer_usage.cpu_usage as f64));

        // INC action count
        prom_out.push(Counter::from("action_count").inc());
    }

    // SET schedule version
    if !schedule_change.pending_schedule.is_empty() {
        prom_out.push(Gauge::from("schedule_version").set(schedule_change.schedule_version as f64));
    }

    // RESET remove producer
    // any producers that are no longer in the active schedule
    // must be declared after SET or else producer could stay in schedule indefinitely
    for remove_producer in schedule_change.remove_from_schedule {
        log::info!("RESET remove_producer={}", remove_producer);
        // prom_out.push_reset("producer_usage", vec!["remove_producer"]) ;
    }

    Ok(prom_out)
}
