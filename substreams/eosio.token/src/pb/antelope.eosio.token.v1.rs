// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Actions {
    #[prost(message, repeated, tag="1")]
    pub actions: ::prost::alloc::vec::Vec<Action>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Action {
    #[prost(uint32, tag="1")]
    pub block_num: u32,
    #[prost(message, optional, tag="2")]
    pub timestamp: ::core::option::Option<::prost_types::Timestamp>,
    #[prost(string, tag="3")]
    pub transaction_id: ::prost::alloc::string::String,
    #[prost(string, tag="4")]
    pub account: ::prost::alloc::string::String,
    #[prost(string, tag="5")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag="6")]
    pub json_data: ::prost::alloc::string::String,
}
/// Encoded file descriptor set for the `antelope.eosio.token.v1` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xd4, 0x06, 0x0a, 0x0d, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x12, 0x17, 0x61, 0x6e, 0x74, 0x65, 0x6c, 0x6f, 0x70, 0x65, 0x2e, 0x65, 0x6f, 0x73,
    0x69, 0x6f, 0x2e, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x2e, 0x76, 0x31, 0x1a, 0x1f, 0x67, 0x6f, 0x6f,
    0x67, 0x6c, 0x65, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2f, 0x74, 0x69, 0x6d,
    0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0x44, 0x0a, 0x07,
    0x41, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x73, 0x12, 0x39, 0x0a, 0x07, 0x61, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x73, 0x18, 0x01, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x1f, 0x2e, 0x61, 0x6e, 0x74, 0x65, 0x6c,
    0x6f, 0x70, 0x65, 0x2e, 0x65, 0x6f, 0x73, 0x69, 0x6f, 0x2e, 0x74, 0x6f, 0x6b, 0x65, 0x6e, 0x2e,
    0x76, 0x31, 0x2e, 0x41, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x07, 0x61, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x73, 0x22, 0xd1, 0x01, 0x0a, 0x06, 0x41, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x1b, 0x0a,
    0x09, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x5f, 0x6e, 0x75, 0x6d, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0d,
    0x52, 0x08, 0x62, 0x6c, 0x6f, 0x63, 0x6b, 0x4e, 0x75, 0x6d, 0x12, 0x38, 0x0a, 0x09, 0x74, 0x69,
    0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x1a, 0x2e,
    0x67, 0x6f, 0x6f, 0x67, 0x6c, 0x65, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x62, 0x75, 0x66, 0x2e,
    0x54, 0x69, 0x6d, 0x65, 0x73, 0x74, 0x61, 0x6d, 0x70, 0x52, 0x09, 0x74, 0x69, 0x6d, 0x65, 0x73,
    0x74, 0x61, 0x6d, 0x70, 0x12, 0x25, 0x0a, 0x0e, 0x74, 0x72, 0x61, 0x6e, 0x73, 0x61, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x5f, 0x69, 0x64, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x0d, 0x74, 0x72,
    0x61, 0x6e, 0x73, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x49, 0x64, 0x12, 0x18, 0x0a, 0x07, 0x61,
    0x63, 0x63, 0x6f, 0x75, 0x6e, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x07, 0x61, 0x63,
    0x63, 0x6f, 0x75, 0x6e, 0x74, 0x12, 0x12, 0x0a, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x05, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x04, 0x6e, 0x61, 0x6d, 0x65, 0x12, 0x1b, 0x0a, 0x09, 0x6a, 0x73, 0x6f,
    0x6e, 0x5f, 0x64, 0x61, 0x74, 0x61, 0x18, 0x06, 0x20, 0x01, 0x28, 0x09, 0x52, 0x08, 0x6a, 0x73,
    0x6f, 0x6e, 0x44, 0x61, 0x74, 0x61, 0x4a, 0xe6, 0x03, 0x0a, 0x06, 0x12, 0x04, 0x00, 0x00, 0x11,
    0x01, 0x0a, 0x08, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x00, 0x00, 0x12, 0x0a, 0x08, 0x0a, 0x01, 0x02,
    0x12, 0x03, 0x02, 0x00, 0x20, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12, 0x03, 0x04, 0x00, 0x29,
    0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x06, 0x00, 0x08, 0x01, 0x0a, 0x0a, 0x0a, 0x03,
    0x04, 0x00, 0x01, 0x12, 0x03, 0x06, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00,
    0x12, 0x03, 0x07, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x04, 0x12, 0x03,
    0x07, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12, 0x03, 0x07, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x07, 0x12, 0x19, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x07, 0x1c, 0x1d, 0x0a, 0x0a, 0x0a,
    0x02, 0x04, 0x01, 0x12, 0x04, 0x0a, 0x00, 0x11, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01,
    0x12, 0x03, 0x0a, 0x08, 0x0e, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x0b,
    0x02, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x05, 0x12, 0x03, 0x0b, 0x02, 0x08,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0b, 0x09, 0x12, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0b, 0x15, 0x16, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x02, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x01, 0x06, 0x12, 0x03, 0x0c, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01,
    0x12, 0x03, 0x0c, 0x1c, 0x25, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03,
    0x0c, 0x28, 0x29, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x0d, 0x02, 0x1c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x0d, 0x02, 0x08, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x09, 0x17, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0d, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01,
    0x02, 0x03, 0x12, 0x03, 0x0e, 0x02, 0x15, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x05,
    0x12, 0x03, 0x0e, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x0e, 0x09, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03, 0x0e, 0x13,
    0x14, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x04, 0x12, 0x03, 0x0f, 0x02, 0x12, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x05, 0x12, 0x03, 0x0f, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x0f, 0x09, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x04, 0x03, 0x12, 0x03, 0x0f, 0x10, 0x11, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x05,
    0x12, 0x03, 0x10, 0x02, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x05, 0x12, 0x03,
    0x10, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x01, 0x12, 0x03, 0x10, 0x09,
    0x12, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x05, 0x03, 0x12, 0x03, 0x10, 0x15, 0x16, 0x62,
    0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
// @@protoc_insertion_point(module)