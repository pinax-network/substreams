use substreams::errors::Error;
use substreams::log;
use substreams_antelope::{Block, ProducerAuthoritySchedule};

use crate::eosmechanics::ProducerUsage;
use crate::sinks::PrometheusMetrics;

#[substreams::handlers::map]
pub fn map_producer_usage(block: Block) -> Result<ProducerUsage, Error> {

    // Producer is found in the block header
    let producer = block.clone().header.unwrap().producer;
    let active_schedule: Vec<String> = schedule_to_accounts(block.clone().active_schedule_v2.unwrap());
    let pending_schedule: Vec<String> = schedule_to_accounts(block.clone().pending_schedule.unwrap().schedule_v2.unwrap());
    
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
                active_schedule,
                pending_schedule,
            })
        }
    }

    // If no transaction trace contains `eosmechanics:cpu` action, return default value
    Ok(Default::default())
}

pub fn schedule_to_accounts(schedule: ProducerAuthoritySchedule) -> Vec<String> {
    let accounts = schedule.producers.iter().map(|p| p.account_name.to_string()).collect();
    accounts
}

#[substreams::handlers::map]
pub fn prom_out(producer_usage: ProducerUsage) -> Result<PrometheusMetrics, Error> {
    log::debug!("producer_usage={:?}", producer_usage);
    
    //  TO-DO push generic prometheus changes here
    //  this requires that we define a prometheus changes protobuf first. I (Fred) will help you on that task
    return Ok(PrometheusMetrics::default());
}
