// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Logs {
    #[prost(message, repeated, tag="1")]
    pub tradelogs: ::prost::alloc::vec::Vec<TradeLog>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TradeLog {
    #[prost(uint64, tag="1")]
    pub block_num: u64,
    #[prost(string, tag="2")]
    pub trx_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="3")]
    pub trx_index: u32,
    #[prost(message, optional, tag="4")]
    pub block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag="5")]
    pub producer: ::prost::alloc::string::String,
    #[prost(uint32, tag="6")]
    pub cpu_usage: u32,
    #[prost(string, tag="7")]
    pub contract: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub executor: ::prost::alloc::string::String,
    #[prost(message, optional, tag="9")]
    pub borrow: ::core::option::Option<Asset>,
    #[prost(message, optional, tag="10")]
    pub profit: ::core::option::Option<Asset>,
    #[prost(message, repeated, tag="11")]
    pub quantities: ::prost::alloc::vec::Vec<Asset>,
    #[prost(string, repeated, tag="12")]
    pub codes: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Asset {
    #[prost(double, tag="1")]
    pub value: f64,
    #[prost(uint64, tag="2")]
    pub amount: u64,
    #[prost(string, tag="3")]
    pub symbol: ::prost::alloc::string::String,
    #[prost(uint32, tag="4")]
    pub precision: u32,
}
// @@protoc_insertion_point(module)
