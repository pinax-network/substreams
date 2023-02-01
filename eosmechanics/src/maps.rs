use substreams::errors::Error;
use substreams::{log, prelude::*};
use substreams_antelope::pb::antelope::Block;

use crate::eosmechanics::{KeyValues, KeyValue, BlockStats};

#[substreams::handlers::map]
pub fn map_block_stats(_block: Block) -> Result<BlockStats, Error> {
    Ok(BlockStats {
        cpu_usage: 1, // TO-DO
        producer: "producer".to_string(), // TO-DO
    })
}

#[substreams::handlers::map]
pub fn map_stores(store_cpu_usage: Deltas<DeltaInt64>, store_producer_count: Deltas<DeltaInt64>) -> Result<KeyValues, Error> {
    let mut items = Vec::new();

    for delta in store_cpu_usage.deltas {
        log::debug!("cpu_usage delta={:?}", delta);

        let key = format!("cpu_usage:{}", delta.key);
        let value = delta.new_value;
        if value > 0 {
            items.push(KeyValue { key, value })
        }
    }

    for delta in store_producer_count.deltas {
        log::debug!("producer_count delta={:?}", delta);

        let key = format!("producer_count:{}", delta.key);
        let value = delta.new_value;
        if value > 0 {
            items.push(KeyValue { key, value })
        }
    }
    Ok(KeyValues { items })
}
