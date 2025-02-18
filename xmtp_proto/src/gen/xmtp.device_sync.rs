// @generated
// This file is @generated by prost-build.
/// Union type representing everything that can be serialied and saved in a backup archive.
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupElement {
    #[prost(oneof="backup_element::Element", tags="1, 2, 3, 4")]
    pub element: ::core::option::Option<backup_element::Element>,
}
/// Nested message and enum types in `BackupElement`.
pub mod backup_element {
    #[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Element {
        #[prost(message, tag="1")]
        Metadata(super::BackupMetadataSave),
        #[prost(message, tag="2")]
        Group(super::group_backup::GroupSave),
        #[prost(message, tag="3")]
        GroupMessage(super::message_backup::GroupMessageSave),
        #[prost(message, tag="4")]
        Consent(super::consent_backup::ConsentSave),
    }
}
impl ::prost::Name for BackupElement {
const NAME: &'static str = "BackupElement";
const PACKAGE: &'static str = "xmtp.device_sync";
fn full_name() -> ::prost::alloc::string::String { "xmtp.device_sync.BackupElement".into() }fn type_url() -> ::prost::alloc::string::String { "/xmtp.device_sync.BackupElement".into() }}
/// Proto representation of backup metadata
/// (Backup version is explicitly missing - it's stored as a header.)
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct BackupMetadataSave {
    #[prost(enumeration="BackupElementSelection", repeated, tag="2")]
    pub elements: ::prost::alloc::vec::Vec<i32>,
    #[prost(int64, tag="3")]
    pub exported_at_ns: i64,
    #[prost(int64, optional, tag="4")]
    pub start_ns: ::core::option::Option<i64>,
    #[prost(int64, optional, tag="5")]
    pub end_ns: ::core::option::Option<i64>,
}
impl ::prost::Name for BackupMetadataSave {
const NAME: &'static str = "BackupMetadataSave";
const PACKAGE: &'static str = "xmtp.device_sync";
fn full_name() -> ::prost::alloc::string::String { "xmtp.device_sync.BackupMetadataSave".into() }fn type_url() -> ::prost::alloc::string::String { "/xmtp.device_sync.BackupMetadataSave".into() }}
/// Elements selected for backup
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum BackupElementSelection {
    Unspecified = 0,
    Messages = 1,
    Consent = 2,
}
impl BackupElementSelection {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            BackupElementSelection::Unspecified => "BACKUP_ELEMENT_SELECTION_UNSPECIFIED",
            BackupElementSelection::Messages => "BACKUP_ELEMENT_SELECTION_MESSAGES",
            BackupElementSelection::Consent => "BACKUP_ELEMENT_SELECTION_CONSENT",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "BACKUP_ELEMENT_SELECTION_UNSPECIFIED" => Some(Self::Unspecified),
            "BACKUP_ELEMENT_SELECTION_MESSAGES" => Some(Self::Messages),
            "BACKUP_ELEMENT_SELECTION_CONSENT" => Some(Self::Consent),
            _ => None,
        }
    }
}
/// Encoded file descriptor set for the `xmtp.device_sync` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xd9, 0x0f, 0x0a, 0x1d, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x5f, 0x73, 0x79, 0x6e, 0x63,
    0x2f, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x5f, 0x73, 0x79, 0x6e, 0x63, 0x2e, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x12, 0x10, 0x78, 0x6d, 0x74, 0x70, 0x2e, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x5f,
    0x73, 0x79, 0x6e, 0x63, 0x1a, 0x20, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x5f, 0x73, 0x79, 0x6e,
    0x63, 0x2f, 0x63, 0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x74, 0x5f, 0x62, 0x61, 0x63, 0x6b, 0x75, 0x70,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x1e, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x5f, 0x73,
    0x79, 0x6e, 0x63, 0x2f, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x5f, 0x62, 0x61, 0x63, 0x6b, 0x75, 0x70,
    0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x1a, 0x20, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x5f, 0x73,
    0x79, 0x6e, 0x63, 0x2f, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x5f, 0x62, 0x61, 0x63, 0x6b,
    0x75, 0x70, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x22, 0xc4, 0x02, 0x0a, 0x0d, 0x42, 0x61, 0x63,
    0x6b, 0x75, 0x70, 0x45, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x12, 0x42, 0x0a, 0x08, 0x6d, 0x65,
    0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x24, 0x2e, 0x78,
    0x6d, 0x74, 0x70, 0x2e, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x5f, 0x73, 0x79, 0x6e, 0x63, 0x2e,
    0x42, 0x61, 0x63, 0x6b, 0x75, 0x70, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x53, 0x61,
    0x76, 0x65, 0x48, 0x00, 0x52, 0x08, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x12, 0x40,
    0x0a, 0x05, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x28, 0x2e,
    0x78, 0x6d, 0x74, 0x70, 0x2e, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x5f, 0x73, 0x79, 0x6e, 0x63,
    0x2e, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x5f, 0x62, 0x61, 0x63, 0x6b, 0x75, 0x70, 0x2e, 0x47, 0x72,
    0x6f, 0x75, 0x70, 0x53, 0x61, 0x76, 0x65, 0x48, 0x00, 0x52, 0x05, 0x67, 0x72, 0x6f, 0x75, 0x70,
    0x12, 0x58, 0x0a, 0x0d, 0x67, 0x72, 0x6f, 0x75, 0x70, 0x5f, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x31, 0x2e, 0x78, 0x6d, 0x74, 0x70, 0x2e, 0x64,
    0x65, 0x76, 0x69, 0x63, 0x65, 0x5f, 0x73, 0x79, 0x6e, 0x63, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x5f, 0x62, 0x61, 0x63, 0x6b, 0x75, 0x70, 0x2e, 0x47, 0x72, 0x6f, 0x75, 0x70, 0x4d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x53, 0x61, 0x76, 0x65, 0x48, 0x00, 0x52, 0x0c, 0x67, 0x72,
    0x6f, 0x75, 0x70, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x12, 0x48, 0x0a, 0x07, 0x63, 0x6f,
    0x6e, 0x73, 0x65, 0x6e, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0b, 0x32, 0x2c, 0x2e, 0x78, 0x6d,
    0x74, 0x70, 0x2e, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x5f, 0x73, 0x79, 0x6e, 0x63, 0x2e, 0x63,
    0x6f, 0x6e, 0x73, 0x65, 0x6e, 0x74, 0x5f, 0x62, 0x61, 0x63, 0x6b, 0x75, 0x70, 0x2e, 0x43, 0x6f,
    0x6e, 0x73, 0x65, 0x6e, 0x74, 0x53, 0x61, 0x76, 0x65, 0x48, 0x00, 0x52, 0x07, 0x63, 0x6f, 0x6e,
    0x73, 0x65, 0x6e, 0x74, 0x42, 0x09, 0x0a, 0x07, 0x65, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x22,
    0xd4, 0x01, 0x0a, 0x12, 0x42, 0x61, 0x63, 0x6b, 0x75, 0x70, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61,
    0x74, 0x61, 0x53, 0x61, 0x76, 0x65, 0x12, 0x44, 0x0a, 0x08, 0x65, 0x6c, 0x65, 0x6d, 0x65, 0x6e,
    0x74, 0x73, 0x18, 0x02, 0x20, 0x03, 0x28, 0x0e, 0x32, 0x28, 0x2e, 0x78, 0x6d, 0x74, 0x70, 0x2e,
    0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x5f, 0x73, 0x79, 0x6e, 0x63, 0x2e, 0x42, 0x61, 0x63, 0x6b,
    0x75, 0x70, 0x45, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x53, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x69,
    0x6f, 0x6e, 0x52, 0x08, 0x65, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x12, 0x24, 0x0a, 0x0e,
    0x65, 0x78, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x5f, 0x61, 0x74, 0x5f, 0x6e, 0x73, 0x18, 0x03,
    0x20, 0x01, 0x28, 0x03, 0x52, 0x0c, 0x65, 0x78, 0x70, 0x6f, 0x72, 0x74, 0x65, 0x64, 0x41, 0x74,
    0x4e, 0x73, 0x12, 0x1e, 0x0a, 0x08, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x6e, 0x73, 0x18, 0x04,
    0x20, 0x01, 0x28, 0x03, 0x48, 0x00, 0x52, 0x07, 0x73, 0x74, 0x61, 0x72, 0x74, 0x4e, 0x73, 0x88,
    0x01, 0x01, 0x12, 0x1a, 0x0a, 0x06, 0x65, 0x6e, 0x64, 0x5f, 0x6e, 0x73, 0x18, 0x05, 0x20, 0x01,
    0x28, 0x03, 0x48, 0x01, 0x52, 0x05, 0x65, 0x6e, 0x64, 0x4e, 0x73, 0x88, 0x01, 0x01, 0x42, 0x0b,
    0x0a, 0x09, 0x5f, 0x73, 0x74, 0x61, 0x72, 0x74, 0x5f, 0x6e, 0x73, 0x42, 0x09, 0x0a, 0x07, 0x5f,
    0x65, 0x6e, 0x64, 0x5f, 0x6e, 0x73, 0x2a, 0x8f, 0x01, 0x0a, 0x16, 0x42, 0x61, 0x63, 0x6b, 0x75,
    0x70, 0x45, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x53, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x12, 0x28, 0x0a, 0x24, 0x42, 0x41, 0x43, 0x4b, 0x55, 0x50, 0x5f, 0x45, 0x4c, 0x45, 0x4d,
    0x45, 0x4e, 0x54, 0x5f, 0x53, 0x45, 0x4c, 0x45, 0x43, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x55, 0x4e,
    0x53, 0x50, 0x45, 0x43, 0x49, 0x46, 0x49, 0x45, 0x44, 0x10, 0x00, 0x12, 0x25, 0x0a, 0x21, 0x42,
    0x41, 0x43, 0x4b, 0x55, 0x50, 0x5f, 0x45, 0x4c, 0x45, 0x4d, 0x45, 0x4e, 0x54, 0x5f, 0x53, 0x45,
    0x4c, 0x45, 0x43, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x4d, 0x45, 0x53, 0x53, 0x41, 0x47, 0x45, 0x53,
    0x10, 0x01, 0x12, 0x24, 0x0a, 0x20, 0x42, 0x41, 0x43, 0x4b, 0x55, 0x50, 0x5f, 0x45, 0x4c, 0x45,
    0x4d, 0x45, 0x4e, 0x54, 0x5f, 0x53, 0x45, 0x4c, 0x45, 0x43, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x43,
    0x4f, 0x4e, 0x53, 0x45, 0x4e, 0x54, 0x10, 0x02, 0x42, 0x84, 0x01, 0x0a, 0x14, 0x63, 0x6f, 0x6d,
    0x2e, 0x78, 0x6d, 0x74, 0x70, 0x2e, 0x64, 0x65, 0x76, 0x69, 0x63, 0x65, 0x5f, 0x73, 0x79, 0x6e,
    0x63, 0x42, 0x0f, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x53, 0x79, 0x6e, 0x63, 0x50, 0x72, 0x6f,
    0x74, 0x6f, 0x50, 0x01, 0xa2, 0x02, 0x03, 0x58, 0x44, 0x58, 0xaa, 0x02, 0x0f, 0x58, 0x6d, 0x74,
    0x70, 0x2e, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x53, 0x79, 0x6e, 0x63, 0xca, 0x02, 0x0f, 0x58,
    0x6d, 0x74, 0x70, 0x5c, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x53, 0x79, 0x6e, 0x63, 0xe2, 0x02,
    0x1b, 0x58, 0x6d, 0x74, 0x70, 0x5c, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x53, 0x79, 0x6e, 0x63,
    0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea, 0x02, 0x10, 0x58,
    0x6d, 0x74, 0x70, 0x3a, 0x3a, 0x44, 0x65, 0x76, 0x69, 0x63, 0x65, 0x53, 0x79, 0x6e, 0x63, 0x4a,
    0x82, 0x08, 0x0a, 0x06, 0x12, 0x04, 0x01, 0x00, 0x20, 0x01, 0x0a, 0x23, 0x0a, 0x01, 0x0c, 0x12,
    0x03, 0x01, 0x00, 0x12, 0x1a, 0x19, 0x20, 0x44, 0x65, 0x66, 0x69, 0x6e, 0x69, 0x74, 0x69, 0x6f,
    0x6e, 0x73, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x62, 0x61, 0x63, 0x6b, 0x75, 0x70, 0x73, 0x0a, 0x0a,
    0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x02, 0x00, 0x19, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x00, 0x12,
    0x03, 0x04, 0x00, 0x2a, 0x0a, 0x09, 0x0a, 0x02, 0x03, 0x01, 0x12, 0x03, 0x05, 0x00, 0x28, 0x0a,
    0x09, 0x0a, 0x02, 0x03, 0x02, 0x12, 0x03, 0x06, 0x00, 0x2a, 0x0a, 0x65, 0x0a, 0x02, 0x04, 0x00,
    0x12, 0x04, 0x09, 0x00, 0x10, 0x01, 0x1a, 0x59, 0x20, 0x55, 0x6e, 0x69, 0x6f, 0x6e, 0x20, 0x74,
    0x79, 0x70, 0x65, 0x20, 0x72, 0x65, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x69, 0x6e, 0x67,
    0x20, 0x65, 0x76, 0x65, 0x72, 0x79, 0x74, 0x68, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x61, 0x74,
    0x20, 0x63, 0x61, 0x6e, 0x20, 0x62, 0x65, 0x20, 0x73, 0x65, 0x72, 0x69, 0x61, 0x6c, 0x69, 0x65,
    0x64, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x73, 0x61, 0x76, 0x65, 0x64, 0x20, 0x69, 0x6e, 0x20, 0x61,
    0x20, 0x62, 0x61, 0x63, 0x6b, 0x75, 0x70, 0x20, 0x61, 0x72, 0x63, 0x68, 0x69, 0x76, 0x65, 0x2e,
    0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12, 0x03, 0x09, 0x08, 0x15, 0x0a, 0x0c, 0x0a,
    0x04, 0x04, 0x00, 0x08, 0x00, 0x12, 0x04, 0x0a, 0x02, 0x0f, 0x03, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x08, 0x00, 0x01, 0x12, 0x03, 0x0a, 0x08, 0x0f, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x0b, 0x04, 0x24, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x0b, 0x04, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x0b,
    0x17, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x0b, 0x22, 0x23,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x0c, 0x04, 0x36, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x01, 0x06, 0x12, 0x03, 0x0c, 0x04, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x0c, 0x2c, 0x31, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x0c, 0x34, 0x35, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12,
    0x03, 0x0d, 0x04, 0x47, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x0d,
    0x04, 0x34, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x0d, 0x35, 0x42,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x0d, 0x45, 0x46, 0x0a, 0x0b,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x0e, 0x04, 0x3c, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x00, 0x02, 0x03, 0x06, 0x12, 0x03, 0x0e, 0x04, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x01, 0x12, 0x03, 0x0e, 0x30, 0x37, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03,
    0x12, 0x03, 0x0e, 0x3a, 0x3b, 0x0a, 0x78, 0x0a, 0x02, 0x04, 0x01, 0x12, 0x04, 0x14, 0x00, 0x19,
    0x01, 0x1a, 0x6c, 0x20, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x20, 0x72, 0x65, 0x70, 0x72, 0x65, 0x73,
    0x65, 0x6e, 0x74, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f, 0x66, 0x20, 0x62, 0x61, 0x63, 0x6b,
    0x75, 0x70, 0x20, 0x6d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0x0a, 0x20, 0x28, 0x42, 0x61,
    0x63, 0x6b, 0x75, 0x70, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x20, 0x69, 0x73, 0x20,
    0x65, 0x78, 0x70, 0x6c, 0x69, 0x63, 0x69, 0x74, 0x6c, 0x79, 0x20, 0x6d, 0x69, 0x73, 0x73, 0x69,
    0x6e, 0x67, 0x20, 0x2d, 0x20, 0x69, 0x74, 0x27, 0x73, 0x20, 0x73, 0x74, 0x6f, 0x72, 0x65, 0x64,
    0x20, 0x61, 0x73, 0x20, 0x61, 0x20, 0x68, 0x65, 0x61, 0x64, 0x65, 0x72, 0x2e, 0x29, 0x0a, 0x0a,
    0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x14, 0x08, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04,
    0x01, 0x02, 0x00, 0x12, 0x03, 0x15, 0x02, 0x2f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x04, 0x12, 0x03, 0x15, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x06, 0x12,
    0x03, 0x15, 0x0b, 0x21, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x15,
    0x22, 0x2a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x15, 0x2d, 0x2e,
    0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x16, 0x02, 0x1b, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x01, 0x05, 0x12, 0x03, 0x16, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04,
    0x01, 0x02, 0x01, 0x01, 0x12, 0x03, 0x16, 0x08, 0x16, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x01, 0x03, 0x12, 0x03, 0x16, 0x19, 0x1a, 0x0a, 0x0b, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12,
    0x03, 0x17, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x04, 0x12, 0x03, 0x17,
    0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x17, 0x0b, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x17, 0x11, 0x19, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x01, 0x02, 0x02, 0x03, 0x12, 0x03, 0x17, 0x1c, 0x1d, 0x0a, 0x0b, 0x0a, 0x04,
    0x04, 0x01, 0x02, 0x03, 0x12, 0x03, 0x18, 0x02, 0x1c, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02,
    0x03, 0x04, 0x12, 0x03, 0x18, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x05,
    0x12, 0x03, 0x18, 0x0b, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03,
    0x18, 0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03, 0x18, 0x1a,
    0x1b, 0x0a, 0x2a, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x1c, 0x00, 0x20, 0x01, 0x1a, 0x1e, 0x20,
    0x45, 0x6c, 0x65, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x73, 0x65, 0x6c, 0x65, 0x63, 0x74, 0x65,
    0x64, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x62, 0x61, 0x63, 0x6b, 0x75, 0x70, 0x0a, 0x0a, 0x0a, 0x0a,
    0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x1c, 0x05, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x1d, 0x02, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x1d, 0x02, 0x26, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x1d,
    0x29, 0x2a, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x1e, 0x02, 0x28, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x1e, 0x02, 0x23, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x1e, 0x26, 0x27, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x00, 0x02, 0x02, 0x12, 0x03, 0x1f, 0x02, 0x27, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x1f, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12,
    0x03, 0x1f, 0x25, 0x26, 0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("xmtp.device_sync.serde.rs");
// @@protoc_insertion_point(module)