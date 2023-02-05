use substreams::errors::Error;
use substreams_antelope::{Block, ProducerAuthoritySchedule};

use crate::eosmechanics::ProducerUsage;


/// Map a block to a ProducerUsage struct
/// 
/// This function is called for every block that is received from the blockchain.
/// It will return a ProducerUsage struct if the block contains a transaction trace
/// that contains an `eosmechanics:cpu` action. Otherwise, it will return a default
/// ProducerUsage struct.
/// 
/// The ProducerUsage struct contains the producer name, the CPU usage of the
/// transaction trace, and the active and pending schedule.
/// 
/// The active and pending schedule are used to determine if the producer is in the
/// pending schedule. If the producer is in the pending schedule, then the CPU usage
/// is not counted towards the producer's CPU usage.
/// 
/// The active and pending schedule are returned as a vector of strings, where each
/// string is the account name of the producer.
/// 
/// The CPU usage is returned as a i64, which is the number of microseconds of CPU
/// used by the transaction trace.
/// 
/// The producer name is returned as a string, which is the account name of the
/// producer.
#[substreams::handlers::map]
pub fn map_producer_usage(block: Block) -> Result<ProducerUsage, Error> {

    // Producer is found in the block header
    let producer = block.header.as_ref().unwrap().producer.clone();
    let active_schedule: Vec<String> = schedule_to_accounts(block.active_schedule_v2.clone().unwrap());
    let pending_schedule: Vec<String> = schedule_to_accounts(block.pending_schedule.as_ref().unwrap().schedule_v2.clone().unwrap());

    for trx in block.all_transaction_traces() {
        // CPU usage is found in the transaction receipt
        let cpu_usage = trx.receipt.as_ref().unwrap().cpu_usage_micro_seconds as i64;

        // Only return a value if the transaction trace contains `eosmechanics:cpu` action
        for trace in &trx.action_traces {
            let action_trace = trace.action.as_ref().unwrap();
            if action_trace.account != "eosmechanics" { continue; }
            if action_trace.name != "cpu"  { continue; }
            return Ok(ProducerUsage{
                producer,
                cpu_usage,
                active_schedule,
                pending_schedule
            })
        }
    }

    // If no transaction trace contains `eosmechanics:cpu` action, return default value
    Ok(Default::default())
}

pub fn schedule_to_accounts(schedule: ProducerAuthoritySchedule) -> Vec<String> {
    schedule.producers.iter().map(|p| p.account_name.clone()).collect()
}
