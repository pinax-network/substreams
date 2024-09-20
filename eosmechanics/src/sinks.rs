use std::collections::HashMap;

use substreams::log;
use substreams::{errors::Error, skip_empty_output};
use substreams_sink_prometheus::{Counter, Gauge, Histogram, PrometheusOperations, Summary};

use crate::eosmechanics::{ProducerUsage, ScheduleChange};

#[substreams::handlers::map]
pub fn prom_out(producer_usage: ProducerUsage, schedule_change: ScheduleChange) -> Result<PrometheusOperations, Error> {
    skip_empty_output();

    let mut prom_out = PrometheusOperations::default();
    let producer = producer_usage.producer.clone();
    let labels = HashMap::from([("producer".to_string(), producer.clone())]);

    // SET producer CPU usage
    if !producer.is_empty() {
        log::info!("producer_usage={:?}", producer_usage);
        let mut gauge = Gauge::from("producer_usage").with(labels.clone());
        prom_out.push(gauge.set(producer_usage.cpu_usage as f64));

        // Histogram & Summary
        let mut histogram = Histogram::from("histogram_producer_usage");
        prom_out.push(histogram.observe(producer_usage.cpu_usage as f64));

        let mut summary = Summary::from("summary_producer_usage");
        prom_out.push(summary.observe(producer_usage.cpu_usage as f64));

        // INC action count
        prom_out.push(Counter::from("cpu_actions_total").inc());
        prom_out.push(Counter::from("cpu_actions").with(labels.clone()).inc());
    }

    // SET schedule version
    if !schedule_change.pending_schedule.is_empty() {
        log::info!("schedule_change={:?}", schedule_change);
        prom_out.push(Counter::from("schedule_changes").inc());
        prom_out.push(Gauge::from("schedule_version").set(schedule_change.schedule_version as f64));
    }

    // RESET remove producer
    // any producers that are no longer in the active schedule
    // must be declared after SET or else producer could stay in schedule indefinitely
    for remove_producer in schedule_change.remove_from_schedule {
        log::info!("RESET remove_producer={}", remove_producer);
        let remove_labels = HashMap::from([("producer".to_string(), remove_producer.clone())]);
        prom_out.push(Gauge::from("producer_usage").remove(remove_labels.clone()));
    }

    Ok(prom_out)
}
