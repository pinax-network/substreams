use substreams::pb::substreams::Clock;
use substreams::{log, prelude::*};

use crate::keyer;
use crate::eosmechanics::BlockStats;

#[substreams::handlers::store]
fn store_cpu_usage(clock: Clock, stats: BlockStats, s: StoreAddInt64) {
    let seconds = clock.clone().timestamp.unwrap().seconds;
    log::debug!("clock {:?} stats {:?}", clock, stats);
    s.add(1, keyer::get_key(stats.producer.as_str(), seconds, keyer::INTERVAL), stats.cpu_usage);
}

#[substreams::handlers::store]
fn store_producer_count(clock: Clock, stats: BlockStats, s: StoreAddInt64) {
    let seconds = clock.clone().timestamp.unwrap().seconds;
    log::debug!("clock {:?} stats {:?}", clock, stats);
    if stats.cpu_usage > 0 {
        s.add(1, keyer::get_key(stats.producer.as_str(), seconds, keyer::INTERVAL), 1);
    }
}
