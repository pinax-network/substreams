#[path = "pb/eosmechanics.v1.rs"]
#[allow(dead_code)]
pub mod eosmechanics;
pub use self::eosmechanics::*;

#[path = "pb/pinax.sink.prom.v1.rs"]
#[allow(dead_code)]
pub mod sinks;
pub use self::sinks::*;

mod maps;
