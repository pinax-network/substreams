use substreams::errors::Error;
use substreams_antelope::{Block, ActionTraces};

#[substreams::handlers::map]
pub fn map_actions(block: Block) -> Result<ActionTraces, Error> {
    Ok(Default::default())
}
