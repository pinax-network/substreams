use substreams::pb::substreams::Clock;

#[substreams::handlers::map]
pub fn map_blocks(clock: Clock) -> Result<Clock, substreams::errors::Error> {
    Ok(clock)
}
