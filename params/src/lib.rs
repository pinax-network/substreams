use substreams::log;
use substreams::errors::Error;
use substreams_ethereum::pb::eth::v2::Block;

#[substreams::handlers::map]
pub fn map_params(params: String, block: Block) -> Result<Block, Error> {
    log::debug!("map_params: {:?}", params);
    Ok(Default::default())
}
