#[allow(dead_code)]
pub const ACCOUNT: Option<&'static str> = Some("blend.gems");
pub mod types {
    use substreams_antelope::types::*;
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct BlendsRow {
        pub id: NftExtra,
        #[serde(deserialize_with = "substreams_antelope::decoder::vec_str_or_u64")]
        pub recipe_ids: Vec<Uint64>,
        pub description: String,
        pub plugin: Name,
        pub quantity: ExtendedAsset,
        pub start_time: TimePointSec,
        pub end_time: TimePointSec,
    }
    impl std::str::FromStr for BlendsRow {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct CollectionsRow {
        pub collection_names: Vec<Name>,
    }
    impl std::str::FromStr for CollectionsRow {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct ConfigRow {
        pub status: Name,
        pub protocol_fee: Uint16,
        pub fee_account: Name,
    }
    impl std::str::FromStr for ConfigRow {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct LimitsRow {
        pub template_id: Int32,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub mint_assets: Int64,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub max_mint_assets: Int64,
    }
    impl std::str::FromStr for LimitsRow {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Nft {
        pub collection_name: Name,
        pub template_id: Int32,
    }
    impl std::str::FromStr for Nft {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct NftExtra {
        pub collection_name: Name,
        pub template_id: Int32,
        pub schema_name: Name,
    }
    impl std::str::FromStr for NftExtra {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct OrdersRow {
        pub id: Nft,
        pub quantity: ExtendedAsset,
    }
    impl std::str::FromStr for OrdersRow {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct RecipesRow {
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_u64")]
        pub id: Uint64,
        pub templates: Vec<NftExtra>,
    }
    impl std::str::FromStr for RecipesRow {
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
}
pub mod actions {
    use substreams_antelope::types::*;
    use substreams_antelope::decoder::decode;
    #[allow(unused_imports)]
    use super::types::*;
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Addrecipe {
        pub collection_name: Name,
        pub template_id: Int32,
        pub templates: Vec<Nft>,
    }
    impl std::str::FromStr for Addrecipe {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Addrecipe {
        const NAME: &'static str = "addrecipe";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Blendlog {
        pub owner: Name,
        pub description: String,
        #[serde(deserialize_with = "substreams_antelope::decoder::vec_str_or_u64")]
        pub in_asset_ids: Vec<Uint64>,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_u64")]
        pub out_asset_id: Uint64,
        pub in_templates: Vec<NftExtra>,
        pub out_template: NftExtra,
        pub total_mint: Int32,
        pub total_burn: Int32,
    }
    impl std::str::FromStr for Blendlog {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Blendlog {
        const NAME: &'static str = "blendlog";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Cancel {
        pub owner: Name,
        pub template_id: Int32,
    }
    impl std::str::FromStr for Cancel {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Cancel {
        const NAME: &'static str = "cancel";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Delblend {
        pub collection_name: Name,
        pub template_id: Int32,
    }
    impl std::str::FromStr for Delblend {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Delblend {
        const NAME: &'static str = "delblend";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Dellimit {
        pub collection_name: Name,
        pub template_id: Int32,
    }
    impl std::str::FromStr for Dellimit {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Dellimit {
        const NAME: &'static str = "dellimit";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Delrecipe {
        pub collection_name: Name,
        pub template_id: Int32,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_u64")]
        pub recipe_id: Uint64,
    }
    impl std::str::FromStr for Delrecipe {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Delrecipe {
        const NAME: &'static str = "delrecipe";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Reset {
        pub table: Name,
        pub scope: Name,
    }
    impl std::str::FromStr for Reset {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Reset {
        const NAME: &'static str = "reset";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Setblend {
        pub collection_name: Name,
        pub template_id: Int32,
        pub description: String,
        pub plugin: Name,
        pub quantity: ExtendedAsset,
        pub start_time: TimePointSec,
        pub end_time: TimePointSec,
    }
    impl std::str::FromStr for Setblend {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Setblend {
        const NAME: &'static str = "setblend";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Setfee {
        pub protocol_fee: Uint16,
        pub fee_account: Name,
    }
    impl std::str::FromStr for Setfee {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Setfee {
        const NAME: &'static str = "setfee";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Setlimit {
        pub collection_name: Name,
        pub template_id: Int32,
        #[serde(deserialize_with = "substreams_antelope::decoder::str_or_i64")]
        pub max_mint_assets: Int64,
    }
    impl std::str::FromStr for Setlimit {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Setlimit {
        const NAME: &'static str = "setlimit";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Setrecipes {
        pub collection_name: Name,
        pub template_id: Int32,
        #[serde(deserialize_with = "substreams_antelope::decoder::vec_str_or_u64")]
        pub recipe_ids: Vec<Uint64>,
    }
    impl std::str::FromStr for Setrecipes {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Setrecipes {
        const NAME: &'static str = "setrecipes";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
    #[derive(Debug, Clone, PartialEq, serde::Deserialize)]
    #[serde(deny_unknown_fields)]
    pub struct Setstatus {
        pub status: Name,
    }
    impl std::str::FromStr for Setstatus {
        type Err = substreams_antelope::Error;
        fn from_str(value: &str) -> Result<Self, Self::Err> {
            substreams_antelope::decoder::decode::<Self>(value)
        }
    }
    impl substreams_antelope::Action for Setstatus {
        const NAME: &'static str = "setstatus";
        fn decode(
            trace: &substreams_antelope::pb::ActionTrace,
        ) -> Result<Self, substreams_antelope::Error> {
            decode::<Self>(&trace.action.as_ref().unwrap().json_data)
        }
    }
}