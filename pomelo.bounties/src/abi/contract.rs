#[allow(dead_code)]
pub const ACCOUNT: Option<&'static str> = Some("d.w.pomelo");
pub mod types {
    use substreams_antelope::types::*;
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct BountiesRow {
        pub bounty_id: Name,
        pub author_user_id: Name,
        pub funders: Vec<PairNameAsset>,
        pub amount: ExtendedAsset,
        pub fee: ExtendedAsset,
        pub claimed: Asset,
        pub applicant_user_ids: Vec<Name>,
        pub approved_user_id: Name,
        pub status: Name,
        #[serde(rename = "type")]
        pub type_: Name,
        pub permissions: Name,
        pub metadata: Vec<PairNameString>,
        pub created_at: TimePointSec,
        pub updated_at: TimePointSec,
        pub submitted_at: TimePointSec,
        pub completed_at: TimePointSec,
    }
    impl std::str::FromStr for BountiesRow {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct ConfigsRow {
        pub status: Name,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_u64")]
        pub fee: Uint64,
        pub login_contract: Name,
        pub fee_account: Name,
        pub metadata_keys: Vec<Name>,
    }
    impl std::str::FromStr for ConfigsRow {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct ExtendedSymbol {
        pub sym: Symbol,
        pub contract: Name,
    }
    impl std::str::FromStr for ExtendedSymbol {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct PairNameAsset {
        pub first: Name,
        pub second: Asset,
    }
    impl std::str::FromStr for PairNameAsset {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct PairNameString {
        pub first: Name,
        pub second: String,
    }
    impl std::str::FromStr for PairNameString {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct StatusRow {
        pub counters: Vec<Uint32>,
        pub last_updated: TimePointSec,
    }
    impl std::str::FromStr for StatusRow {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct TokensRow {
        pub sym: Symbol,
        pub contract: Name,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_u64")]
        pub min_amount: Uint64,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_u64")]
        pub oracle_id: Uint64,
    }
    impl std::str::FromStr for TokensRow {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct TransfersRow {
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_u64")]
        pub transfer_id: Uint64,
        pub bounty_id: Name,
        pub funder_user_id: Name,
        pub from: Name,
        pub to: Name,
        pub ext_quantity: ExtendedAsset,
        pub fee: Asset,
        pub memo: String,
        pub value: Float64,
        pub trx_id: Checksum256,
        pub created_at: TimePointSec,
    }
    impl std::str::FromStr for TransfersRow {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
}
pub mod actions {
    use substreams_antelope::types::*;
    use substreams_antelope::decoder::decode;
    #[allow(unused_imports)]
    use super::types::*;
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Apply {
        pub bounty_id: Name,
        pub user_id: Name,
    }
    impl std::str::FromStr for Apply {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Apply {
        const NAME: &'static str = "apply";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Approve {
        pub bounty_id: Name,
        pub applicant_user_id: Name,
    }
    impl std::str::FromStr for Approve {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Approve {
        const NAME: &'static str = "approve";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Claim {
        pub bounty_id: Name,
        pub receiver: Name,
    }
    impl std::str::FromStr for Claim {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Claim {
        const NAME: &'static str = "claim";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Claimlog {
        pub bounty_id: Name,
        pub receiver: Name,
        pub ext_quantity: ExtendedAsset,
        pub fee: Asset,
        pub status: Name,
        pub worker_user_id: Name,
        pub days_since_created: Uint32,
    }
    impl std::str::FromStr for Claimlog {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Claimlog {
        const NAME: &'static str = "claimlog";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Close {
        pub bounty_id: Name,
    }
    impl std::str::FromStr for Close {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Close {
        const NAME: &'static str = "close";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Complete {
        pub bounty_id: Name,
    }
    impl std::str::FromStr for Complete {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Complete {
        const NAME: &'static str = "complete";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Create {
        pub author_user_id: Name,
        pub bounty_id: Name,
        pub accepted_token: SymbolCode,
        pub bounty_type: Name,
    }
    impl std::str::FromStr for Create {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Create {
        const NAME: &'static str = "create";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Createlog {
        pub bounty_id: Name,
        pub author_user_id: Name,
        pub ext_sym: ExtendedSymbol,
        #[serde(rename = "type")]
        pub type_: Name,
        pub permissions: Name,
    }
    impl std::str::FromStr for Createlog {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Createlog {
        const NAME: &'static str = "createlog";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Deltoken {
        pub symcode: SymbolCode,
    }
    impl std::str::FromStr for Deltoken {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Deltoken {
        const NAME: &'static str = "deltoken";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Deny {
        pub bounty_id: Name,
    }
    impl std::str::FromStr for Deny {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Deny {
        const NAME: &'static str = "deny";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Depositlog {
        pub bounty_id: Name,
        pub funder_user_id: Name,
        pub from: Name,
        pub ext_quantity: ExtendedAsset,
        pub fee: Asset,
        pub value: Float64,
        pub memo: String,
    }
    impl std::str::FromStr for Depositlog {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Depositlog {
        const NAME: &'static str = "depositlog";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Forfeit {
        pub bounty_id: Name,
    }
    impl std::str::FromStr for Forfeit {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Forfeit {
        const NAME: &'static str = "forfeit";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Publish {
        pub bounty_id: Name,
    }
    impl std::str::FromStr for Publish {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Publish {
        const NAME: &'static str = "publish";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Release {
        pub bounty_id: Name,
    }
    impl std::str::FromStr for Release {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Release {
        const NAME: &'static str = "release";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Setconfig {
        pub status: Name,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_u64")]
        pub fee: Uint64,
        pub login_contract: Name,
        pub fee_account: Name,
        pub metadata_keys: Vec<Name>,
    }
    impl std::str::FromStr for Setconfig {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Setconfig {
        const NAME: &'static str = "setconfig";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Setmetadata {
        pub bounty_id: Name,
        pub metadata_key: Name,
        pub metadata_value: String,
    }
    impl std::str::FromStr for Setmetadata {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Setmetadata {
        const NAME: &'static str = "setmetadata";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Setstate {
        pub bounty_id: Name,
        pub state: Name,
    }
    impl std::str::FromStr for Setstate {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Setstate {
        const NAME: &'static str = "setstate";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Statelog {
        pub bounty_id: Name,
        pub status: Name,
        pub action: Name,
    }
    impl std::str::FromStr for Statelog {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Statelog {
        const NAME: &'static str = "statelog";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Syncbounty {
        pub bounty_id: Name,
        pub state: Name,
        pub applicant_user_ids: Vec<Name>,
        pub approved_user_id: Name,
        pub updated_at: TimePointSec,
        pub submitted_at: TimePointSec,
        pub completed_at: TimePointSec,
    }
    impl std::str::FromStr for Syncbounty {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Syncbounty {
        const NAME: &'static str = "syncbounty";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Token {
        pub sym: Symbol,
        pub contract: Name,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_u64")]
        pub min_amount: Uint64,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_u64")]
        pub oracle_id: Uint64,
    }
    impl std::str::FromStr for Token {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Token {
        const NAME: &'static str = "token";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Withdraw {
        pub bounty_id: Name,
        pub receiver: Name,
    }
    impl std::str::FromStr for Withdraw {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Withdraw {
        const NAME: &'static str = "withdraw";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Withdrawlog {
        pub bounty_id: Name,
        pub status: Name,
        pub author_user_id: Name,
        pub receiver: Name,
        pub refund: ExtendedAsset,
    }
    impl std::str::FromStr for Withdrawlog {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Withdrawlog {
        const NAME: &'static str = "withdrawlog";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
}