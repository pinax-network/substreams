use crate::ord::envelope::ParsedEnvelope;
use crate::pb::bitcoin::ordinals::v1::{Inscription, Inscriptions};
use bitcoin::{consensus::deserialize, hashes::hex::FromHex, Transaction};
use substreams::errors::Error;
use substreams::log;
use substreams_bitcoin::pb::btc::v1::Block;

#[allow(dead_code)]
mod ord;
mod pb;

#[substreams::handlers::map]
fn map_inscriptions(block: Block) -> Result<Inscriptions, Error> {
    let items = block
        .transactions()
        .into_iter()
        .inspect(|tx| log::info!("{:?}", tx.vin.first().unwrap().txid))
        .filter_map(|trx| {
            // log::info!("{:?}", tx);
            let trx_id = &trx.txid;
            let owner = &trx.vin.first().unwrap().txid;
            let raw_trx = Vec::from_hex(&trx.hex).unwrap();
            let trx: Transaction = deserialize(&raw_trx).unwrap();
            let envelopes = ParsedEnvelope::from_transaction(&trx);
            let inscriptions = envelopes.into_iter().filter_map(move |envelope| {
                Some(Inscription {
                    inscription_id: format!("{}i{}", trx_id, envelope.offset),
                    inscribed_by: owner.into(),
                    owned_by: owner.into(),
                    time: block.time,
                    height: block.height,
                    offset: envelope.offset,
                    content_type: envelope.payload.content_type().unwrap_or_default().to_string(),
                    content_length: envelope.payload.content_length().unwrap_or_default() as u32,
                    content: envelope.payload.body().unwrap_or_default().to_vec(),
                    ..Default::default()
                })
            });
            Some(inscriptions)
        })
        .flatten()
        .take(5)
        .collect::<Vec<_>>();

    Ok(Inscriptions { items })
}
