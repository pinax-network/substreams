mod pb;

use pb::eth::blobs::v1::{Blob, Blobs};
use pb::sf::beacon::r#type::v1::{block::Body::*, Block as BeaconBlock};

#[substreams::handlers::map]
fn map_blobs(blk: BeaconBlock) -> Result<Blobs, substreams::errors::Error> {
    let blobs = match blk.body.unwrap() {
        Deneb(body) => body
            .embedded_blobs
            .into_iter()
            .inspect(|_| substreams::log::info!("blk.timestamp: {}", blk.timestamp.clone().unwrap_or_default()))
            .map(|b| Blob {
                slot: blk.slot,
                timestamp: blk.timestamp.clone(),
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
