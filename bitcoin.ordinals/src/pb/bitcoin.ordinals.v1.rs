// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Inscriptions {
    #[prost(message, repeated, tag="1")]
    pub items: ::prost::alloc::vec::Vec<Inscription>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Inscription {
    #[prost(string, tag="1")]
    pub inscription_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub inscribed_by: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub owned_by: ::prost::alloc::string::String,
    #[prost(int64, tag="5")]
    pub time: i64,
    #[prost(int64, tag="6")]
    pub height: i64,
    #[prost(string, tag="8")]
    pub content_type: ::prost::alloc::string::String,
    #[prost(uint32, tag="9")]
    pub content_length: u32,
    #[prost(bytes="vec", tag="10")]
    pub content: ::prost::alloc::vec::Vec<u8>,
    #[prost(uint32, tag="11")]
    pub inscription_fee: u32,
}
// @@protoc_insertion_point(module)
