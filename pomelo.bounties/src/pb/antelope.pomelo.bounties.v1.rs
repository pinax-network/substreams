// @generated
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Actions {
    #[prost(message, repeated, tag="1")]
    pub statelogs: ::prost::alloc::vec::Vec<StateLog>,
    #[prost(message, repeated, tag="2")]
    pub applys: ::prost::alloc::vec::Vec<Apply>,
    #[prost(message, repeated, tag="3")]
    pub createlogs: ::prost::alloc::vec::Vec<CreateLog>,
    #[prost(message, repeated, tag="4")]
    pub claimlogs: ::prost::alloc::vec::Vec<ClaimLog>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct StateLog {
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub trx_index: u32,
    #[prost(string, tag="3")]
    pub state: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub action: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub bounty_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Apply {
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub trx_index: u32,
    #[prost(string, tag="3")]
    pub bounty_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub user_id: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CreateLog {
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub trx_index: u32,
    #[prost(string, tag="3")]
    pub bounty_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub author_user_id: ::prost::alloc::string::String,
    #[prost(message, optional, tag="5")]
    pub ext_sym: ::core::option::Option<ExtendedSymbol>,
    #[prost(string, tag="6")]
    pub r#type: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub permissions: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtendedSymbol {
    #[prost(string, tag="1")]
    pub sym: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub contract: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExtendedAsset {
    #[prost(string, tag="1")]
    pub quantity: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub contract: ::prost::alloc::string::String,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ClaimLog {
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="2")]
    pub trx_index: u32,
    #[prost(string, tag="3")]
    pub bounty_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub receiver: ::prost::alloc::string::String,
    #[prost(message, optional, tag="5")]
    pub ext_quantity: ::core::option::Option<ExtendedAsset>,
    #[prost(string, tag="6")]
    pub fee: ::prost::alloc::string::String,
    #[prost(string, tag="7")]
    pub status: ::prost::alloc::string::String,
    #[prost(string, tag="8")]
    pub worker_user_id: ::prost::alloc::string::String,
    #[prost(uint32, tag="9")]
    pub days_since_created: u32,
}
// @@protoc_insertion_point(module)
