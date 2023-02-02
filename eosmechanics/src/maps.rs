use substreams::errors::Error;
use substreams::{log, prelude::*};
use substreams_antelope::Block;

use crate::eosmechanics::{BlockResults, ProducerStats};

#[substreams::handlers::map]
pub fn map_block_stats(block: Block) -> Result<BlockResults, Error> {

    // ToDo parse the transaction traces of the block
    // if it contains an action on account "eosmechanics" with the name "cpu" then take the
    // cpu time of the transaction and the name of the producer and return it.

    // the producer can be found on the block header using:
    // block.header.unwrap().producer

    // the cpu usage can be found in the transaction receipt

    Ok(BlockResults{
        producer_stats: vec![]
    })
}

// #[substreams::handlers::map]
// pub fn prom_out(block_results: BlockResults) -> Result<PrometheusChange, Error> {
//
//  ToDo push generic prometheus changes here
//  this requires that we define a prometheus changes protobuf first. I (Fred) will help you on that task
//
// }
