#[path = "pb/eosmechanics.v1.rs"]
#[allow(dead_code)]
pub mod eosmechanics;
pub use self::eosmechanics::*;

#[path = "pb/pinax.substreams.sink.prometheus.v1.rs"]
#[allow(dead_code)]
pub mod prometheus;
pub use self::prometheus::*;

mod maps;
mod sinks;
