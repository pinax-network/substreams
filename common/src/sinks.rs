use substreams::proto;
use substreams::errors::Error;
use substreams::pb::substreams::Clock;
use substreams_sink_kv::pb::sf::substreams::sink::kv::v1::KvOperations;

use crate::common::{BlockId, BlockTimestamp};

#[substreams::handlers::map]
pub fn kv_out(clock: Clock) -> Result<KvOperations, Error> {
    let mut kv_ops: KvOperations = Default::default();

    // From block number
    let key = format!("block.number:{}", clock.number);
    let value = BlockTimestamp { timestamp: clock.timestamp.as_ref().unwrap().to_string() };
    kv_ops.push_new(key, proto::encode(&value).unwrap(), 1);

    // From timestamp
    let key = format!("block.timestamp:{}", clock.timestamp.unwrap());
    let value = BlockId { id: clock.id, number: clock.number };
    kv_ops.push_new(key, proto::encode(&value).unwrap(), 1);

    Ok(kv_ops)
}