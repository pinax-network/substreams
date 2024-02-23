mod pb;

use pb::eth::blobs::v1::{Blob, Blobs};
use pb::sf::beacon::r#type::v1::{block::Body::*, Block as BeaconBlock};
use substreams_sink_kv::pb::sf::substreams::sink::kv::v1::KvOperations;

#[substreams::handlers::map]
fn map_blobs(blk: BeaconBlock) -> Result<Blobs, substreams::errors::Error> {
    let blobs = match blk.body.unwrap() {
        Deneb(body) => body
            .embedded_blobs
            .into_iter()
            // .inspect(|_| substreams::log::info!("blk.timestamp: {}", blk.timestamp.clone().unwrap_or_default()))
            .map(|b| Blob {
                slot: blk.slot,
                timestamp: body.execution_payload.as_ref().cloned().unwrap_or_default().timestamp, // blk.timestamp.clone(),
                block_number: body.execution_payload.as_ref().cloned().unwrap_or_default().block_number,
                index: b.index as u32,
                length: b.blob.len() as u32,
                data: b.blob,
                kzg_commitment: b.kzg_commitment,
                kzg_proof: b.kzg_proof,
            })
            .collect(),
        _ => vec![],
    };
    Ok(Blobs { blobs })
}

#[substreams::handlers::map]
fn kv_out(blobs: Blobs) -> Result<KvOperations, substreams::errors::Error> {
    let mut kv_ops: KvOperations = Default::default();

    for blob in blobs.blobs {
        let key = format!("slot:{}:{}", blob.slot, blob.index);
        let value = substreams::proto::encode(&blob).expect("unable to encode blob");
        kv_ops.push_new(key, value, 1);
    }

    Ok(kv_ops)
}
