// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transactions {
    #[prost(message, repeated, tag="1")]
    pub transactions: ::prost::alloc::vec::Vec<Transaction>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Transaction {
    #[prost(string, tag="2")]
    pub trx_id: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub op: ::prost::alloc::string::String,
    #[prost(uint64, tag="4")]
    pub block_num: u64,
    #[prost(message, optional, tag="5")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag="6")]
    pub sender: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub account: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub action: ::prost::alloc::string::String,
    #[prost(string, tag="9")]
    pub json_data: ::prost::alloc::string::String,
    #[prost(string, tag="10")]
    pub parent_trx_id: ::prost::alloc::string::String,
}
// @@protoc_insertion_point(module)
