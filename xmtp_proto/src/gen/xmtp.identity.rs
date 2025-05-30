// @generated
// This file is @generated by prost-build.
/// A credential that can be used in MLS leaf nodes
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MlsCredential {
    #[prost(string, tag="1")]
    pub inbox_id: ::prost::alloc::string::String,
}
impl ::prost::Name for MlsCredential {
const NAME: &'static str = "MlsCredential";
const PACKAGE: &'static str = "xmtp.identity";
fn full_name() -> ::prost::alloc::string::String { "xmtp.identity.MlsCredential".into() }fn type_url() -> ::prost::alloc::string::String { "/xmtp.identity.MlsCredential".into() }}
/// Encoded file descriptor set for the `xmtp.identity` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xc5, 0x03, 0x0a, 0x19, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x2f, 0x63, 0x72,
    0x65, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x61, 0x6c, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x0d,
    0x78, 0x6d, 0x74, 0x70, 0x2e, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x22, 0x2a, 0x0a,
    0x0d, 0x4d, 0x6c, 0x73, 0x43, 0x72, 0x65, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x61, 0x6c, 0x12, 0x19,
    0x0a, 0x08, 0x69, 0x6e, 0x62, 0x6f, 0x78, 0x5f, 0x69, 0x64, 0x18, 0x01, 0x20, 0x01, 0x28, 0x09,
    0x52, 0x07, 0x69, 0x6e, 0x62, 0x6f, 0x78, 0x49, 0x64, 0x42, 0x9f, 0x01, 0x0a, 0x11, 0x63, 0x6f,
    0x6d, 0x2e, 0x78, 0x6d, 0x74, 0x70, 0x2e, 0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x42,
    0x0f, 0x43, 0x72, 0x65, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x61, 0x6c, 0x50, 0x72, 0x6f, 0x74, 0x6f,
    0x50, 0x01, 0x5a, 0x24, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x78,
    0x6d, 0x74, 0x70, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x76, 0x33, 0x2f, 0x67, 0x6f, 0x2f,
    0x69, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0xa2, 0x02, 0x03, 0x58, 0x49, 0x58, 0xaa, 0x02,
    0x0d, 0x58, 0x6d, 0x74, 0x70, 0x2e, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0xca, 0x02,
    0x0d, 0x58, 0x6d, 0x74, 0x70, 0x5c, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0xe2, 0x02,
    0x19, 0x58, 0x6d, 0x74, 0x70, 0x5c, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x5c, 0x47,
    0x50, 0x42, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea, 0x02, 0x0e, 0x58, 0x6d, 0x74,
    0x70, 0x3a, 0x3a, 0x49, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x74, 0x79, 0x4a, 0xc2, 0x01, 0x0a, 0x06,
    0x12, 0x04, 0x01, 0x00, 0x0b, 0x01, 0x0a, 0x17, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x01, 0x00, 0x12,
    0x1a, 0x0d, 0x20, 0x43, 0x72, 0x65, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x61, 0x6c, 0x73, 0x0a, 0x0a,
    0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x03, 0x00, 0x16, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03,
    0x05, 0x00, 0x3b, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x05, 0x00, 0x3b, 0x0a, 0x3d,
    0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x09, 0x00, 0x0b, 0x01, 0x1a, 0x31, 0x20, 0x41, 0x20, 0x63,
    0x72, 0x65, 0x64, 0x65, 0x6e, 0x74, 0x69, 0x61, 0x6c, 0x20, 0x74, 0x68, 0x61, 0x74, 0x20, 0x63,
    0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x4d, 0x4c,
    0x53, 0x20, 0x6c, 0x65, 0x61, 0x66, 0x20, 0x6e, 0x6f, 0x64, 0x65, 0x73, 0x0a, 0x0a, 0x0a, 0x0a,
    0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x09, 0x08, 0x15, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x0a, 0x02, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12,
    0x03, 0x0a, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0a,
    0x09, 0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0a, 0x14, 0x15,
    0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("xmtp.identity.serde.rs");
// @@protoc_insertion_point(module)