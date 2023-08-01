// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Actions {
    #[prost(message, repeated, tag="1")]
    pub blendlogs: ::prost::alloc::vec::Vec<BlendLog>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BlendLog {
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub trx_index: u32,
    #[prost(string, tag="3")]
    pub owner: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub description: ::prost::alloc::string::String,
    #[prost(uint64, repeated, tag="5")]
    pub in_asset_ids: ::prost::alloc::vec::Vec<u64>,
    #[prost(uint64, tag="6")]
    pub out_asset_id: u64,
    #[prost(message, repeated, tag="7")]
    pub in_templates: ::prost::alloc::vec::Vec<NftExtra>,
    #[prost(message, optional, tag="8")]
    pub out_template: ::core::option::Option<NftExtra>,
    #[prost(int32, tag="9")]
    pub total_mint: i32,
    #[prost(int32, tag="10")]
    pub total_burn: i32,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct NftExtra {
    #[prost(string, tag="1")]
    pub collection_name: ::prost::alloc::string::String,
    #[prost(int32, tag="2")]
    pub template_id: i32,
    #[prost(string, tag="3")]
    pub schema_name: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
