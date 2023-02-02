use substreams::errors::Error;
use substreams::log;
use substreams_antelope::Block;

use crate::eosmechanics::ProducerUsage;
use crate::sinks::PrometheusMetrics;

#[substreams::handlers::map]
pub fn map_producer_usage(block: Block) -> Result<ProducerUsage, Error> {

    // Producer is found in the block header
    let producer = block.clone().header.unwrap().producer;
    
    for trx in block.clone().all_transaction_traces() { 
        // CPU usage is found in the transaction receipt
        let cpu_usage = trx.clone().receipt.unwrap().cpu_usage_micro_seconds as i64;

        // Only return a value if the transaction trace contains `eosmechanics:cpu` action
        for trace in trx.clone().action_traces {
            let action_trace = trace.action.as_ref().unwrap().clone();
            if action_trace.account != "eosmechanics" { continue; }
            if action_trace.name != "cpu"  { continue; }
            return Ok(ProducerUsage{
                producer,
                cpu_usage,
            })
        }
    }

    // If no transaction trace contains `eosmechanics:cpu` action, return default value
    Ok(Default::default())
}

#[substreams::handlers::map]
pub fn prom_out(producer_usage: ProducerUsage) -> Result<PrometheusMetrics, Error> {
    log::debug!("producer_usage={:?}", producer_usage);
    
    //  TO-DO push generic prometheus changes here
    //  this requires that we define a prometheus changes protobuf first. I (Fred) will help you on that task
    return Ok(PrometheusMetrics::default());
}
