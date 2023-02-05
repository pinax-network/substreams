use substreams::errors::Error;
use substreams::log;
use substreams_sink_prometheus::PrometheusOperations;

use crate::eosmechanics::{ProducerUsage, ScheduleChange};

#[substreams::handlers::map]
pub fn prom_out(producer_usage: ProducerUsage, schedule_change: ScheduleChange) -> Result<PrometheusOperations, Error> {
    let mut prom_out = PrometheusOperations::default();
    let producer = producer_usage.producer.clone();

    // SET current producer's CPU usage
    if producer.len() > 0 {
        log::info!("SET producer={} cpu_usage={}", producer, producer_usage.cpu_usage);
        prom_out.push_set(vec![&producer], producer_usage.cpu_usage as f64);
    }

    // RESET any producers that are no longer in the active schedule
    // Must be declared after SET or else producer could stay in schedule indefinitely
    for remove_producer in schedule_change.remove_from_schedule {
        log::info!("RESET remove_producer={}", remove_producer);
        prom_out.push_delete(vec![&remove_producer]) ;
    }
    return Ok(prom_out);
}
