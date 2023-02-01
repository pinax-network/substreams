use substreams::errors::Error;

use crate::eosmechanics::KeyValues;
use substreams_sink_kv::pb::kv::KvOperations;

#[substreams::handlers::map]
pub fn kv_out(map_stores: KeyValues) -> Result<KvOperations, Error> {
    let params = "params"; // NOT YET IMPLEMENTED IN SUBSTREAMS
    let mut kv_ops: KvOperations = Default::default();

    // push to KV database (BadgerDB)
    for item in map_stores.items {
        let key = &format!("{}:{}", params, item.key);
        kv_ops.push_new(key, item.value.to_string(), 1);
    }
    Ok(kv_ops)
}
