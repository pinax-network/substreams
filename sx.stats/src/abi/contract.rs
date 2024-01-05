#[allow(dead_code)]
pub const ACCOUNT: Option<&'static str> = None;
pub mod types {
    use substreams_antelope::types::*;
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct FlashRow {
        pub contract: Name,
        pub last_modified: TimePointSec,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_u64")]
        pub transactions: Uint64,
        pub borrow: Vec<PairSymbolCodeAsset>,
        pub fees: Vec<PairSymbolCodeAsset>,
        pub reserves: Vec<PairSymbolCodeAsset>,
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct GatewayRow {
        pub contract: Name,
        pub last_modified: TimePointSec,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_u64")]
        pub transactions: Uint64,
        pub ins: Vec<PairSymbolCodePairUint64Asset>,
        pub outs: Vec<PairSymbolCodePairUint64Asset>,
        pub exchanges: Vec<PairNameUint64>,
        pub savings: Vec<PairSymbolCodeAsset>,
        pub fees: Vec<PairSymbolCodeAsset>,
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct PairNameUint64 {
        pub key: Name,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_u64")]
        pub value: Uint64,
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct PairSymbolCodeAsset {
        pub key: SymbolCode,
        pub value: Asset,
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct PairSymbolCodeFloat64 {
        pub key: SymbolCode,
        pub value: Float64,
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct PairSymbolCodePairUint64Asset {
        pub key: SymbolCode,
        pub value: PairUint64Asset,
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct PairSymbolCodeUint64 {
        pub key: SymbolCode,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_u64")]
        pub value: Uint64,
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct PairUint64Asset {
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_u64")]
        pub first: Uint64,
        pub second: Asset,
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct SpotpricesRow {
        pub contract: Name,
        pub last_modified: TimePointSec,
        pub base: SymbolCode,
        pub quotes: Vec<PairSymbolCodeFloat64>,
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct TradesRow {
        pub contract: Name,
        pub last_modified: TimePointSec,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_u64")]
        pub transactions: Uint64,
        pub borrow: Vec<PairSymbolCodeAsset>,
        pub quantities: Vec<PairSymbolCodeAsset>,
        pub codes: Vec<PairNameUint64>,
        pub symcodes: Vec<PairSymbolCodeUint64>,
        pub executors: Vec<PairNameUint64>,
        pub profits: Vec<PairSymbolCodeAsset>,
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct VolumeRow {
        pub contract: Name,
        pub last_modified: TimePointSec,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_u64")]
        pub transactions: Uint64,
        pub volume: Vec<PairSymbolCodeAsset>,
        pub fees: Vec<PairSymbolCodeAsset>,
    }
}
pub mod actions {
    use substreams_antelope::types::*;
    use substreams_antelope::decoder::decode;
    #[allow(unused_imports)]
    use super::types::*;
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Clean {
        pub contract: Name,
    }
    impl substreams_antelope::Action for Clean {
        const NAME: &'static str = "clean";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            Ok(decode::<Self>(&trace.action.as_ref().unwrap().json_data)?)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Erase {
        pub contract: Name,
    }
    impl substreams_antelope::Action for Erase {
        const NAME: &'static str = "erase";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            Ok(decode::<Self>(&trace.action.as_ref().unwrap().json_data)?)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Gatewaylog {
        pub contract: Name,
        #[serde(rename = "in")]
        pub in_: Asset,
        pub out: Asset,
        pub exchanges: Vec<Name>,
        pub savings: Asset,
        pub fee: Asset,
    }
    impl substreams_antelope::Action for Gatewaylog {
        const NAME: &'static str = "gatewaylog";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            Ok(decode::<Self>(&trace.action.as_ref().unwrap().json_data)?)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Swaplog {
        pub contract: Name,
        pub buyer: Name,
        pub amount_in: Asset,
        pub amount_out: Asset,
        pub fee: Asset,
    }
    impl substreams_antelope::Action for Swaplog {
        const NAME: &'static str = "swaplog";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            Ok(decode::<Self>(&trace.action.as_ref().unwrap().json_data)?)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Tradelog {
        pub contract: Name,
        pub executor: Name,
        pub borrow: Asset,
        pub quantities: Vec<Asset>,
        pub codes: Vec<Name>,
        pub profit: Asset,
    }
    impl substreams_antelope::Action for Tradelog {
        const NAME: &'static str = "tradelog";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            Ok(decode::<Self>(&trace.action.as_ref().unwrap().json_data)?)
        }
    }
}