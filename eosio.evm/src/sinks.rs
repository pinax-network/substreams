use substreams::errors::Error;
use substreams_antelope::pb::Block;
use substreams_entity_change::pb::entity::EntityChanges;
use substreams_sink_kv::pb::sf::substreams::sink::kv::v1::KvOperations;

use crate::maps;

#[substreams::handlers::map]
pub fn graph_out(block: Block) -> Result<EntityChanges, Error> {
    let mut entity_changes: EntityChanges = Default::default();
    Ok(entity_changes)
}

#[substreams::handlers::map]
pub fn kv_out(block: Block) -> Result<KvOperations, Error> {
    let mut kv_ops: KvOperations = Default::default();
    Ok(kv_ops)
}
