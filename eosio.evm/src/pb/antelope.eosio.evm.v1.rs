// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pushtx {
    #[prost(string, tag="1")]
    pub trx_id: ::prost::alloc::string::String,
    #[prost(string, tag="2")]
    pub miner: ::prost::alloc::string::String,
    #[prost(string, tag="3")]
    pub rlptx: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pushtxs {
    #[prost(message, repeated, tag="1")]
    pub events: ::prost::alloc::vec::Vec<Pushtx>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pushbalance {
    #[prost(string, tag="1")]
    pub miner: ::prost::alloc::string::String,
    #[prost(double, tag="2")]
    pub balance: f64,
    #[prost(string, tag="3")]
    pub symcode: ::prost::alloc::string::String,
    #[prost(int64, tag="4")]
    pub dust: i64,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Pushbalances {
    #[prost(message, repeated, tag="1")]
    pub events: ::prost::alloc::vec::Vec<Pushbalance>,
}
/// Encoded file descriptor set for the `antelope.eosio.evm.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xf8, 0x03, 0x0a, 0x0f, 0x65, 0x6f, 0x73, 0x69, 0x6f, 0x2e, 0x65, 0x76, 0x6d, 0x2e, 0x70,
    0x72, 0x6f, 0x74, 0x6f, 0x12, 0x15, 0x61, 0x6e, 0x74, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x2e, 0x65,
    0x6f, 0x73, 0x69, 0x6f, 0x2e, 0x65, 0x76, 0x6d, 0x2e, 0x76, 0x31, 0x22, 0x4b, 0x0a, 0x06, 0x50,
    0x75, 0x73, 0x68, 0x74, 0x78, 0x12, 0x15, 0x0a, 0x06, 0x74, 0x72, 0x78, 0x5f, 0x69, 0x64, 0x18,
    0x01, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x74, 0x72, 0x78, 0x49, 0x64, 0x12, 0x14, 0x0a, 0x05,
    0x6d, 0x69, 0x6e, 0x65, 0x72, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x05, 0x6d, 0x69, 0x6e,
    0x65, 0x72, 0x12, 0x14, 0x0a, 0x05, 0x72, 0x6c, 0x70, 0x74, 0x78, 0x18, 0x03, 0x20, 0x01, 0x28,
    0x09, 0x52, 0x05, 0x72, 0x6c, 0x70, 0x74, 0x78, 0x22, 0x40, 0x0a, 0x07, 0x50, 0x75, 0x73, 0x68,
    0x74, 0x78, 0x73, 0x12, 0x35, 0x0a, 0x06, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x18, 0x01, 0x20,
    0x03, 0x28, 0x0b, 0x32, 0x1d, 0x2e, 0x61, 0x6e, 0x74, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x2e, 0x65,
    0x6f, 0x73, 0x69, 0x6f, 0x2e, 0x65, 0x76, 0x6d, 0x2e, 0x76, 0x31, 0x2e, 0x50, 0x75, 0x73, 0x68,
    0x74, 0x78, 0x52, 0x06, 0x65, 0x76, 0x65, 0x6e, 0x74, 0x73, 0x4a, 0xb6, 0x02, 0x0a, 0x06, 0x12,
    0x04, 0x00, 0x00, 0x0c, 0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a,
    0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x1e, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12,
    0x04, 0x04, 0x00, 0x08, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x04, 0x08,
    0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x05, 0x04, 0x16, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x05, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x05, 0x0b, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x00, 0x03, 0x12, 0x03, 0x05, 0x14, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01,
    0x12, 0x03, 0x06, 0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03,
    0x06, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x06, 0x0b,
    0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x06, 0x13, 0x14, 0x0a,
    0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x07, 0x04, 0x15, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x02, 0x05, 0x12, 0x03, 0x07, 0x04, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x07, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x07, 0x13, 0x14, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x0a, 0x00,
    0x0c, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x0a, 0x08, 0x0f, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0b, 0x04, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x00, 0x04, 0x12, 0x03, 0x0b, 0x04, 0x0c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x00, 0x06, 0x12, 0x03, 0x0b, 0x0d, 0x13, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01,
    0x12, 0x03, 0x0b, 0x14, 0x1a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03,
    0x0b, 0x1d, 0x1e, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)