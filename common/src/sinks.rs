use substreams::errors::Error;
use substreams_sink_kv::pb::sf::substreams::sink::kv::v1::KvOperations;
use substreams_antelope::{Block};

#[substreams::handlers::map]
pub fn kv_out(block: Block) -> Result<KvOperations, Error> {
    let mut kv_ops: KvOperations = Default::default();

    // block.id
    let key = format!("block.id:{}", block.number);
    kv_ops.push_new(key, block.id.to_string(), 1);

    // block.timestamp
    let key = format!("block.timestamp:{}", block.number);
    let timestamp = block.header.unwrap().timestamp.unwrap().seconds.to_string();
    kv_ops.push_new(key, timestamp, 1);

    Ok(kv_ops)
}
