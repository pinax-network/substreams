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
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct ExtendedSymbol {
        pub sym: Symbol,
        pub contract: Name,
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct PairNameAsset {
        pub first: Name,
        pub second: Asset,
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct PairNameString {
        pub first: Name,
        pub second: String,
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct StatusRow {
        pub counters: Vec<Uint32>,
        pub last_updated: TimePointSec,
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
}
pub mod actions {
    use substreams_antelope::types::*;
    use super::types::*;
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Apply {
        pub bounty_id: Name,
        pub user_id: Name,
    }
    impl substreams_antelope::action::Action for Apply {
        const NAME: &'static str = "apply";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Approve {
        pub bounty_id: Name,
        pub applicant_user_id: Name,
    }
    impl substreams_antelope::action::Action for Approve {
        const NAME: &'static str = "approve";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Claim {
        pub bounty_id: Name,
        pub receiver: Name,
    }
    impl substreams_antelope::action::Action for Claim {
        const NAME: &'static str = "claim";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
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
    impl substreams_antelope::action::Action for Claimlog {
        const NAME: &'static str = "claimlog";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Close {
        pub bounty_id: Name,
    }
    impl substreams_antelope::action::Action for Close {
        const NAME: &'static str = "close";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Complete {
        pub bounty_id: Name,
    }
    impl substreams_antelope::action::Action for Complete {
        const NAME: &'static str = "complete";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
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
    impl substreams_antelope::action::Action for Create {
        const NAME: &'static str = "create";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
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
    impl substreams_antelope::action::Action for Createlog {
        const NAME: &'static str = "createlog";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Deltoken {
        pub symcode: SymbolCode,
    }
    impl substreams_antelope::action::Action for Deltoken {
        const NAME: &'static str = "deltoken";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Deny {
        pub bounty_id: Name,
    }
    impl substreams_antelope::action::Action for Deny {
        const NAME: &'static str = "deny";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
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
    impl substreams_antelope::action::Action for Depositlog {
        const NAME: &'static str = "depositlog";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Forfeit {
        pub bounty_id: Name,
    }
    impl substreams_antelope::action::Action for Forfeit {
        const NAME: &'static str = "forfeit";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Publish {
        pub bounty_id: Name,
    }
    impl substreams_antelope::action::Action for Publish {
        const NAME: &'static str = "publish";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Release {
        pub bounty_id: Name,
    }
    impl substreams_antelope::action::Action for Release {
        const NAME: &'static str = "release";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
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
    impl substreams_antelope::action::Action for Setconfig {
        const NAME: &'static str = "setconfig";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Setmetadata {
        pub bounty_id: Name,
        pub metadata_key: Name,
        pub metadata_value: String,
    }
    impl substreams_antelope::action::Action for Setmetadata {
        const NAME: &'static str = "setmetadata";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Setstate {
        pub bounty_id: Name,
        pub state: Name,
    }
    impl substreams_antelope::action::Action for Setstate {
        const NAME: &'static str = "setstate";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Statelog {
        pub bounty_id: Name,
        pub status: Name,
        pub action: Name,
    }
    impl substreams_antelope::action::Action for Statelog {
        const NAME: &'static str = "statelog";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
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
    impl substreams_antelope::action::Action for Syncbounty {
        const NAME: &'static str = "syncbounty";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
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
    impl substreams_antelope::action::Action for Token {
        const NAME: &'static str = "token";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Withdraw {
        pub bounty_id: Name,
        pub receiver: Name,
    }
    impl substreams_antelope::action::Action for Withdraw {
        const NAME: &'static str = "withdraw";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
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
    impl substreams_antelope::action::Action for Withdrawlog {
        const NAME: &'static str = "withdrawlog";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::errors::Error> {
            Ok(
                substreams_antelope::decoder::decode::<
                    Self,
                >(&trace.action.as_ref().unwrap().json_data)?,
            )
        }
    }
}