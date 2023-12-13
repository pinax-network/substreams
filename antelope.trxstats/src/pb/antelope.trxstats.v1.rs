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
    #[prost(uint64, tag="4")]
    pub block_num: u64,
    #[prost(message, optional, tag="5")]
    pub block_time: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(uint32, tag="10")]
    pub action_count: u32,
    #[prost(int32, tag="11")]
    pub cpu: i32,
    #[prost(uint32, tag="12")]
    pub net: u32,
}
// @@protoc_insertion_point(module)
