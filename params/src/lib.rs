use substreams::log;
use substreams::errors::Error;
use substreams::pb::substreams::Clock;

#[substreams::handlers::map]
pub fn map_params(params: String, clock: Clock) -> Result<Clock, Error> {
    log::debug!("map_params: {:?}", params);
    log::debug!("2nd log map_params: {:?}", params);
    Ok(clock)
}
