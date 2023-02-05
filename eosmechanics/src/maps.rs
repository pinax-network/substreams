use std::collections::HashSet;

use substreams::errors::Error;
use substreams_antelope::{Block, ProducerAuthoritySchedule};

use crate::eosmechanics::{ProducerUsage, ScheduleChange};

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
            })
        }
    }
    // If no transaction trace contains `eosmechanics:cpu` action, return default value
    Ok(Default::default())
}

#[substreams::handlers::map]
pub fn map_schedule_change(block: Block) -> Result<ScheduleChange, Error> {
    let active_schedule: Vec<String> = schedule_to_accounts(block.active_schedule_v2.clone().unwrap());
    let pending_schedule: Vec<String> = schedule_to_accounts(block.pending_schedule.as_ref().unwrap().schedule_v2.clone().unwrap());

    // If there is no pending schedule, then there is no schedule change
    if pending_schedule.is_empty() { return Ok(Default::default()); }

    let mut add_to_schedule: Vec<String> = Default::default();
    let mut remove_from_schedule: Vec<String> = Default::default();

    for producer in &active_schedule {
        match producer_in_schedule(producer.clone(), pending_schedule.clone()) {
            Some(true) => (),
            Some(false) => remove_from_schedule.push(producer.clone()),
            None => (),
        }
    }

    for producer in &pending_schedule {
        match producer_in_schedule(producer.clone(), active_schedule.clone()) {
            Some(true) => (),
            Some(false) => add_to_schedule.push(producer.clone()),
            None => (),
        }
    }

    // header
    let header = block.header.as_ref().unwrap();

    Ok(ScheduleChange{
        producer: header.producer.clone(),
        schedule_version: header.schedule_version,
        active_schedule,
        pending_schedule,
        add_to_schedule,
        remove_from_schedule,
    })
}

pub fn schedule_to_accounts(schedule: ProducerAuthoritySchedule) -> Vec<String> {
    schedule.producers.iter().map(|p| p.account_name.clone()).collect()
}

pub fn producer_in_schedule(producer: String, schedule: Vec<String> ) -> Option<bool> {
    if schedule.is_empty() { return None; }
    Some(schedule_to_set(schedule).contains(producer.as_str()))
}

pub fn schedule_to_set(schedule: Vec<String>) -> HashSet<String> {
    let mut set = HashSet::new();
    for account in schedule {
        set.insert(account);
    }
    set
}