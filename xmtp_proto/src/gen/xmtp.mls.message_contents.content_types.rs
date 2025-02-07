// @generated
// This file is @generated by prost-build.
/// MultiRemoteAttachment message type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MultiRemoteAttachment {
    /// A 32 byte array for decrypting the remote content payload
    #[prost(bytes="vec", tag="1")]
    pub secret: ::prost::alloc::vec::Vec<u8>,
    /// A byte array for the salt used to encrypt the remote content payload
    #[prost(bytes="vec", tag="2")]
    pub salt: ::prost::alloc::vec::Vec<u8>,
    /// Array of attachment information
    #[prost(message, repeated, tag="3")]
    pub attachments: ::prost::alloc::vec::Vec<RemoteAttachmentInfo>,
    /// The number of attachments in the attachments array
    #[prost(uint32, optional, tag="4")]
    pub num_attachments: ::core::option::Option<u32>,
    /// The maximum content length of an attachment in the attachments array
    #[prost(uint32, optional, tag="5")]
    pub max_attachment_content_length: ::core::option::Option<u32>,
}
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoteAttachmentInfo {
    /// The SHA256 hash of the remote content
    #[prost(string, tag="1")]
    pub content_digest: ::prost::alloc::string::String,
    /// A byte array for the nonce used to encrypt the remote content payload
    #[prost(bytes="vec", tag="2")]
    pub nonce: ::prost::alloc::vec::Vec<u8>,
    /// The scheme of the URL. Must be "<https://">
    #[prost(string, tag="3")]
    pub scheme: ::prost::alloc::string::String,
    /// The URL of the remote content
    #[prost(string, tag="4")]
    pub url: ::prost::alloc::string::String,
    /// The filename of the remote content
    #[prost(string, tag="5")]
    pub filename: ::prost::alloc::string::String,
}
/// Reaction message type
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ReactionV2 {
    /// The message ID being reacted to
    #[prost(string, tag="1")]
    pub reference: ::prost::alloc::string::String,
    /// The inbox ID of the user who sent the message being reacted to
    /// Optional for group messages
    #[prost(string, tag="2")]
    pub reference_inbox_id: ::prost::alloc::string::String,
    /// The action of the reaction (added or removed)
    #[prost(enumeration="ReactionAction", tag="3")]
    pub action: i32,
    /// The content of the reaction 
    #[prost(string, tag="4")]
    pub content: ::prost::alloc::string::String,
    /// The schema of the reaction content
    #[prost(enumeration="ReactionSchema", tag="5")]
    pub schema: i32,
}
/// Action enum to represent reaction states
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ReactionAction {
    Unspecified = 0,
    Added = 1,
    Removed = 2,
}
impl ReactionAction {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ReactionAction::Unspecified => "REACTION_ACTION_UNSPECIFIED",
            ReactionAction::Added => "REACTION_ACTION_ADDED",
            ReactionAction::Removed => "REACTION_ACTION_REMOVED",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "REACTION_ACTION_UNSPECIFIED" => Some(Self::Unspecified),
            "REACTION_ACTION_ADDED" => Some(Self::Added),
            "REACTION_ACTION_REMOVED" => Some(Self::Removed),
            _ => None,
        }
    }
}
/// Schema enum to represent reaction content types
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum ReactionSchema {
    Unspecified = 0,
    Unicode = 1,
    Shortcode = 2,
    Custom = 3,
}
impl ReactionSchema {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            ReactionSchema::Unspecified => "REACTION_SCHEMA_UNSPECIFIED",
            ReactionSchema::Unicode => "REACTION_SCHEMA_UNICODE",
            ReactionSchema::Shortcode => "REACTION_SCHEMA_SHORTCODE",
            ReactionSchema::Custom => "REACTION_SCHEMA_CUSTOM",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "REACTION_SCHEMA_UNSPECIFIED" => Some(Self::Unspecified),
            "REACTION_SCHEMA_UNICODE" => Some(Self::Unicode),
            "REACTION_SCHEMA_SHORTCODE" => Some(Self::Shortcode),
            "REACTION_SCHEMA_CUSTOM" => Some(Self::Custom),
            _ => None,
        }
    }
}
/// Encoded file descriptor set for the `xmtp.mls.message_contents.content_types` package
pub const FILE_DESCRIPTOR_SET: &[u8] = &[
    0x0a, 0xb0, 0x13, 0x0a, 0x40, 0x6d, 0x6c, 0x73, 0x2f, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x5f, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x73, 0x2f, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e,
    0x74, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2f, 0x6d, 0x75, 0x6c, 0x74, 0x69, 0x5f, 0x72, 0x65,
    0x6d, 0x6f, 0x74, 0x65, 0x5f, 0x61, 0x74, 0x74, 0x61, 0x63, 0x68, 0x6d, 0x65, 0x6e, 0x74, 0x2e,
    0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x27, 0x78, 0x6d, 0x74, 0x70, 0x2e, 0x6d, 0x6c, 0x73, 0x2e,
    0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x5f, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x73,
    0x2e, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x73, 0x22, 0xd0,
    0x02, 0x0a, 0x15, 0x4d, 0x75, 0x6c, 0x74, 0x69, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x41, 0x74,
    0x74, 0x61, 0x63, 0x68, 0x6d, 0x65, 0x6e, 0x74, 0x12, 0x16, 0x0a, 0x06, 0x73, 0x65, 0x63, 0x72,
    0x65, 0x74, 0x18, 0x01, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x06, 0x73, 0x65, 0x63, 0x72, 0x65, 0x74,
    0x12, 0x12, 0x0a, 0x04, 0x73, 0x61, 0x6c, 0x74, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c, 0x52, 0x04,
    0x73, 0x61, 0x6c, 0x74, 0x12, 0x5f, 0x0a, 0x0b, 0x61, 0x74, 0x74, 0x61, 0x63, 0x68, 0x6d, 0x65,
    0x6e, 0x74, 0x73, 0x18, 0x03, 0x20, 0x03, 0x28, 0x0b, 0x32, 0x3d, 0x2e, 0x78, 0x6d, 0x74, 0x70,
    0x2e, 0x6d, 0x6c, 0x73, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x5f, 0x63, 0x6f, 0x6e,
    0x74, 0x65, 0x6e, 0x74, 0x73, 0x2e, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x5f, 0x74, 0x79,
    0x70, 0x65, 0x73, 0x2e, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x41, 0x74, 0x74, 0x61, 0x63, 0x68,
    0x6d, 0x65, 0x6e, 0x74, 0x49, 0x6e, 0x66, 0x6f, 0x52, 0x0b, 0x61, 0x74, 0x74, 0x61, 0x63, 0x68,
    0x6d, 0x65, 0x6e, 0x74, 0x73, 0x12, 0x2c, 0x0a, 0x0f, 0x6e, 0x75, 0x6d, 0x5f, 0x61, 0x74, 0x74,
    0x61, 0x63, 0x68, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x18, 0x04, 0x20, 0x01, 0x28, 0x0d, 0x48, 0x00,
    0x52, 0x0e, 0x6e, 0x75, 0x6d, 0x41, 0x74, 0x74, 0x61, 0x63, 0x68, 0x6d, 0x65, 0x6e, 0x74, 0x73,
    0x88, 0x01, 0x01, 0x12, 0x46, 0x0a, 0x1d, 0x6d, 0x61, 0x78, 0x5f, 0x61, 0x74, 0x74, 0x61, 0x63,
    0x68, 0x6d, 0x65, 0x6e, 0x74, 0x5f, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x5f, 0x6c, 0x65,
    0x6e, 0x67, 0x74, 0x68, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0d, 0x48, 0x01, 0x52, 0x1a, 0x6d, 0x61,
    0x78, 0x41, 0x74, 0x74, 0x61, 0x63, 0x68, 0x6d, 0x65, 0x6e, 0x74, 0x43, 0x6f, 0x6e, 0x74, 0x65,
    0x6e, 0x74, 0x4c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x88, 0x01, 0x01, 0x42, 0x12, 0x0a, 0x10, 0x5f,
    0x6e, 0x75, 0x6d, 0x5f, 0x61, 0x74, 0x74, 0x61, 0x63, 0x68, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x42,
    0x20, 0x0a, 0x1e, 0x5f, 0x6d, 0x61, 0x78, 0x5f, 0x61, 0x74, 0x74, 0x61, 0x63, 0x68, 0x6d, 0x65,
    0x6e, 0x74, 0x5f, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x5f, 0x6c, 0x65, 0x6e, 0x67, 0x74,
    0x68, 0x22, 0x99, 0x01, 0x0a, 0x14, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x41, 0x74, 0x74, 0x61,
    0x63, 0x68, 0x6d, 0x65, 0x6e, 0x74, 0x49, 0x6e, 0x66, 0x6f, 0x12, 0x25, 0x0a, 0x0e, 0x63, 0x6f,
    0x6e, 0x74, 0x65, 0x6e, 0x74, 0x5f, 0x64, 0x69, 0x67, 0x65, 0x73, 0x74, 0x18, 0x01, 0x20, 0x01,
    0x28, 0x09, 0x52, 0x0d, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x44, 0x69, 0x67, 0x65, 0x73,
    0x74, 0x12, 0x14, 0x0a, 0x05, 0x6e, 0x6f, 0x6e, 0x63, 0x65, 0x18, 0x02, 0x20, 0x01, 0x28, 0x0c,
    0x52, 0x05, 0x6e, 0x6f, 0x6e, 0x63, 0x65, 0x12, 0x16, 0x0a, 0x06, 0x73, 0x63, 0x68, 0x65, 0x6d,
    0x65, 0x18, 0x03, 0x20, 0x01, 0x28, 0x09, 0x52, 0x06, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x65, 0x12,
    0x10, 0x0a, 0x03, 0x75, 0x72, 0x6c, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52, 0x03, 0x75, 0x72,
    0x6c, 0x12, 0x1a, 0x0a, 0x08, 0x66, 0x69, 0x6c, 0x65, 0x6e, 0x61, 0x6d, 0x65, 0x18, 0x05, 0x20,
    0x01, 0x28, 0x09, 0x52, 0x08, 0x66, 0x69, 0x6c, 0x65, 0x6e, 0x61, 0x6d, 0x65, 0x42, 0xc1, 0x02,
    0x0a, 0x2b, 0x63, 0x6f, 0x6d, 0x2e, 0x78, 0x6d, 0x74, 0x70, 0x2e, 0x6d, 0x6c, 0x73, 0x2e, 0x6d,
    0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x5f, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x73, 0x2e,
    0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x73, 0x42, 0x1a, 0x4d,
    0x75, 0x6c, 0x74, 0x69, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x41, 0x74, 0x74, 0x61, 0x63, 0x68,
    0x6d, 0x65, 0x6e, 0x74, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50, 0x01, 0x5a, 0x3e, 0x67, 0x69, 0x74,
    0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x78, 0x6d, 0x74, 0x70, 0x2f, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x2f, 0x76, 0x33, 0x2f, 0x67, 0x6f, 0x2f, 0x6d, 0x6c, 0x73, 0x2f, 0x6d, 0x65, 0x73,
    0x73, 0x61, 0x67, 0x65, 0x5f, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x73, 0x2f, 0x63, 0x6f,
    0x6e, 0x74, 0x65, 0x6e, 0x74, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x73, 0xa2, 0x02, 0x04, 0x58, 0x4d,
    0x4d, 0x43, 0xaa, 0x02, 0x25, 0x58, 0x6d, 0x74, 0x70, 0x2e, 0x4d, 0x6c, 0x73, 0x2e, 0x4d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x43, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x73, 0x2e, 0x43, 0x6f,
    0x6e, 0x74, 0x65, 0x6e, 0x74, 0x54, 0x79, 0x70, 0x65, 0x73, 0xca, 0x02, 0x25, 0x58, 0x6d, 0x74,
    0x70, 0x5c, 0x4d, 0x6c, 0x73, 0x5c, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x43, 0x6f, 0x6e,
    0x74, 0x65, 0x6e, 0x74, 0x73, 0x5c, 0x43, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x54, 0x79, 0x70,
    0x65, 0x73, 0xe2, 0x02, 0x31, 0x58, 0x6d, 0x74, 0x70, 0x5c, 0x4d, 0x6c, 0x73, 0x5c, 0x4d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x43, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x73, 0x5c, 0x43, 0x6f,
    0x6e, 0x74, 0x65, 0x6e, 0x74, 0x54, 0x79, 0x70, 0x65, 0x73, 0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65,
    0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea, 0x02, 0x28, 0x58, 0x6d, 0x74, 0x70, 0x3a, 0x3a, 0x4d,
    0x6c, 0x73, 0x3a, 0x3a, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x43, 0x6f, 0x6e, 0x74, 0x65,
    0x6e, 0x74, 0x73, 0x3a, 0x3a, 0x43, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x54, 0x79, 0x70, 0x65,
    0x73, 0x4a, 0x87, 0x0c, 0x0a, 0x06, 0x12, 0x04, 0x0a, 0x00, 0x32, 0x01, 0x0a, 0xab, 0x02, 0x0a,
    0x01, 0x0c, 0x12, 0x03, 0x0a, 0x00, 0x12, 0x1a, 0xa0, 0x02, 0x20, 0x6d, 0x75, 0x6c, 0x74, 0x69,
    0x5f, 0x72, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x5f, 0x61, 0x74, 0x74, 0x61, 0x63, 0x68, 0x6d, 0x65,
    0x6e, 0x74, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x0a, 0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x66,
    0x69, 0x6c, 0x65, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e, 0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x4d, 0x75, 0x6c, 0x74, 0x69, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x41, 0x74, 0x74, 0x61, 0x63,
    0x68, 0x6d, 0x65, 0x6e, 0x74, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x74, 0x79,
    0x70, 0x65, 0x20, 0x61, 0x6e, 0x64, 0x20, 0x69, 0x73, 0x20, 0x61, 0x73, 0x73, 0x6f, 0x63, 0x69,
    0x61, 0x74, 0x65, 0x64, 0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x6f,
    0x6c, 0x6c, 0x6f, 0x77, 0x69, 0x6e, 0x67, 0x20, 0x43, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x54,
    0x79, 0x70, 0x65, 0x49, 0x64, 0x3a, 0x0a, 0x20, 0x0a, 0x20, 0x43, 0x6f, 0x6e, 0x74, 0x65, 0x6e,
    0x74, 0x54, 0x79, 0x70, 0x65, 0x49, 0x64, 0x20, 0x7b, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x61,
    0x75, 0x74, 0x68, 0x6f, 0x72, 0x69, 0x74, 0x79, 0x5f, 0x69, 0x64, 0x3a, 0x20, 0x22, 0x78, 0x6d,
    0x74, 0x70, 0x2e, 0x6f, 0x72, 0x67, 0x22, 0x2c, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x74, 0x79,
    0x70, 0x65, 0x5f, 0x69, 0x64, 0x3a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x22, 0x6d, 0x75, 0x6c,
    0x74, 0x69, 0x52, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x53, 0x74, 0x61, 0x74, 0x69, 0x63, 0x43, 0x6f,
    0x6e, 0x74, 0x65, 0x6e, 0x74, 0x22, 0x2c, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x76, 0x65, 0x72,
    0x73, 0x69, 0x6f, 0x6e, 0x5f, 0x6d, 0x61, 0x6a, 0x6f, 0x72, 0x3a, 0x20, 0x31, 0x2c, 0x0a, 0x20,
    0x20, 0x20, 0x20, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x5f, 0x6d, 0x69, 0x6e, 0x6f,
    0x72, 0x3a, 0x20, 0x30, 0x2c, 0x0a, 0x20, 0x7d, 0x0a, 0x0a, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12,
    0x03, 0x0c, 0x00, 0x30, 0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0e, 0x00, 0x55, 0x0a, 0x09,
    0x0a, 0x02, 0x08, 0x0b, 0x12, 0x03, 0x0e, 0x00, 0x55, 0x0a, 0x30, 0x0a, 0x02, 0x04, 0x00, 0x12,
    0x04, 0x12, 0x00, 0x21, 0x01, 0x1a, 0x24, 0x20, 0x4d, 0x75, 0x6c, 0x74, 0x69, 0x52, 0x65, 0x6d,
    0x6f, 0x74, 0x65, 0x41, 0x74, 0x74, 0x61, 0x63, 0x68, 0x6d, 0x65, 0x6e, 0x74, 0x20, 0x6d, 0x65,
    0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x74, 0x79, 0x70, 0x65, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04,
    0x00, 0x01, 0x12, 0x03, 0x12, 0x08, 0x1d, 0x0a, 0x48, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12,
    0x03, 0x14, 0x02, 0x13, 0x1a, 0x3b, 0x20, 0x41, 0x20, 0x33, 0x32, 0x20, 0x62, 0x79, 0x74, 0x65,
    0x20, 0x61, 0x72, 0x72, 0x61, 0x79, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x64, 0x65, 0x63, 0x72, 0x79,
    0x70, 0x74, 0x69, 0x6e, 0x67, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x6d, 0x6f, 0x74, 0x65,
    0x20, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x20, 0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x14, 0x02, 0x07, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x14, 0x08, 0x0e, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x14, 0x11, 0x12, 0x0a, 0x53, 0x0a, 0x04, 0x04,
    0x00, 0x02, 0x01, 0x12, 0x03, 0x17, 0x02, 0x11, 0x1a, 0x46, 0x20, 0x41, 0x20, 0x62, 0x79, 0x74,
    0x65, 0x20, 0x61, 0x72, 0x72, 0x61, 0x79, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x73, 0x61, 0x6c, 0x74, 0x20, 0x75, 0x73, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x20, 0x65, 0x6e, 0x63,
    0x72, 0x79, 0x70, 0x74, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x20,
    0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x20, 0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x05, 0x12, 0x03, 0x17, 0x02, 0x07, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x17, 0x08, 0x0c, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x01, 0x03, 0x12, 0x03, 0x17, 0x0f, 0x10, 0x0a, 0x2e, 0x0a, 0x04, 0x04, 0x00,
    0x02, 0x02, 0x12, 0x03, 0x1a, 0x02, 0x30, 0x1a, 0x21, 0x20, 0x41, 0x72, 0x72, 0x61, 0x79, 0x20,
    0x6f, 0x66, 0x20, 0x61, 0x74, 0x74, 0x61, 0x63, 0x68, 0x6d, 0x65, 0x6e, 0x74, 0x20, 0x69, 0x6e,
    0x66, 0x6f, 0x72, 0x6d, 0x61, 0x74, 0x69, 0x6f, 0x6e, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x02, 0x04, 0x12, 0x03, 0x1a, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02,
    0x06, 0x12, 0x03, 0x1a, 0x0b, 0x1f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12,
    0x03, 0x1a, 0x20, 0x2b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x1a,
    0x2e, 0x2f, 0x0a, 0x41, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x1d, 0x02, 0x26, 0x1a,
    0x34, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6e, 0x75, 0x6d, 0x62, 0x65, 0x72, 0x20, 0x6f, 0x66, 0x20,
    0x61, 0x74, 0x74, 0x61, 0x63, 0x68, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x69, 0x6e, 0x20, 0x74,
    0x68, 0x65, 0x20, 0x61, 0x74, 0x74, 0x61, 0x63, 0x68, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x61,
    0x72, 0x72, 0x61, 0x79, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x04, 0x12, 0x03,
    0x1d, 0x02, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x05, 0x12, 0x03, 0x1d, 0x0b,
    0x11, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01, 0x12, 0x03, 0x1d, 0x12, 0x21, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03, 0x1d, 0x24, 0x25, 0x0a, 0x53, 0x0a,
    0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x20, 0x02, 0x34, 0x1a, 0x46, 0x20, 0x54, 0x68, 0x65,
    0x20, 0x6d, 0x61, 0x78, 0x69, 0x6d, 0x75, 0x6d, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74,
    0x20, 0x6c, 0x65, 0x6e, 0x67, 0x74, 0x68, 0x20, 0x6f, 0x66, 0x20, 0x61, 0x6e, 0x20, 0x61, 0x74,
    0x74, 0x61, 0x63, 0x68, 0x6d, 0x65, 0x6e, 0x74, 0x20, 0x69, 0x6e, 0x20, 0x74, 0x68, 0x65, 0x20,
    0x61, 0x74, 0x74, 0x61, 0x63, 0x68, 0x6d, 0x65, 0x6e, 0x74, 0x73, 0x20, 0x61, 0x72, 0x72, 0x61,
    0x79, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x04, 0x12, 0x03, 0x20, 0x02, 0x0a,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x05, 0x12, 0x03, 0x20, 0x0b, 0x11, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x20, 0x12, 0x2f, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x20, 0x32, 0x33, 0x0a, 0x0a, 0x0a, 0x02, 0x04, 0x01,
    0x12, 0x04, 0x23, 0x00, 0x32, 0x01, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x01, 0x01, 0x12, 0x03, 0x23,
    0x08, 0x1c, 0x0a, 0x34, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x00, 0x12, 0x03, 0x25, 0x02, 0x1c, 0x1a,
    0x27, 0x20, 0x54, 0x68, 0x65, 0x20, 0x53, 0x48, 0x41, 0x32, 0x35, 0x36, 0x20, 0x68, 0x61, 0x73,
    0x68, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x20,
    0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00,
    0x05, 0x12, 0x03, 0x25, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x25, 0x09, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x00, 0x03, 0x12, 0x03, 0x25,
    0x1a, 0x1b, 0x0a, 0x54, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x01, 0x12, 0x03, 0x28, 0x02, 0x12, 0x1a,
    0x47, 0x20, 0x41, 0x20, 0x62, 0x79, 0x74, 0x65, 0x20, 0x61, 0x72, 0x72, 0x61, 0x79, 0x20, 0x66,
    0x6f, 0x72, 0x20, 0x74, 0x68, 0x65, 0x20, 0x6e, 0x6f, 0x6e, 0x63, 0x65, 0x20, 0x75, 0x73, 0x65,
    0x64, 0x20, 0x74, 0x6f, 0x20, 0x65, 0x6e, 0x63, 0x72, 0x79, 0x70, 0x74, 0x20, 0x74, 0x68, 0x65,
    0x20, 0x72, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x20,
    0x70, 0x61, 0x79, 0x6c, 0x6f, 0x61, 0x64, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01,
    0x05, 0x12, 0x03, 0x28, 0x02, 0x07, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x01, 0x12,
    0x03, 0x28, 0x08, 0x0d, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x01, 0x03, 0x12, 0x03, 0x28,
    0x10, 0x11, 0x0a, 0x38, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x02, 0x12, 0x03, 0x2b, 0x02, 0x14, 0x1a,
    0x2b, 0x20, 0x54, 0x68, 0x65, 0x20, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x65, 0x20, 0x6f, 0x66, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x55, 0x52, 0x4c, 0x2e, 0x20, 0x4d, 0x75, 0x73, 0x74, 0x20, 0x62, 0x65,
    0x20, 0x22, 0x68, 0x74, 0x74, 0x70, 0x73, 0x3a, 0x2f, 0x2f, 0x22, 0x0a, 0x0a, 0x0c, 0x0a, 0x05,
    0x04, 0x01, 0x02, 0x02, 0x05, 0x12, 0x03, 0x2b, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01,
    0x02, 0x02, 0x01, 0x12, 0x03, 0x2b, 0x09, 0x0f, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x02,
    0x03, 0x12, 0x03, 0x2b, 0x12, 0x13, 0x0a, 0x2c, 0x0a, 0x04, 0x04, 0x01, 0x02, 0x03, 0x12, 0x03,
    0x2e, 0x02, 0x11, 0x1a, 0x1f, 0x20, 0x54, 0x68, 0x65, 0x20, 0x55, 0x52, 0x4c, 0x20, 0x6f, 0x66,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x74,
    0x65, 0x6e, 0x74, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x05, 0x12, 0x03, 0x2e,
    0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x01, 0x12, 0x03, 0x2e, 0x09, 0x0c,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x03, 0x03, 0x12, 0x03, 0x2e, 0x0f, 0x10, 0x0a, 0x31,
    0x0a, 0x04, 0x04, 0x01, 0x02, 0x04, 0x12, 0x03, 0x31, 0x02, 0x16, 0x1a, 0x24, 0x20, 0x54, 0x68,
    0x65, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x6e, 0x61, 0x6d, 0x65, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x72, 0x65, 0x6d, 0x6f, 0x74, 0x65, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74,
    0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x05, 0x12, 0x03, 0x31, 0x02, 0x08, 0x0a,
    0x0c, 0x0a, 0x05, 0x04, 0x01, 0x02, 0x04, 0x01, 0x12, 0x03, 0x31, 0x09, 0x11, 0x0a, 0x0c, 0x0a,
    0x05, 0x04, 0x01, 0x02, 0x04, 0x03, 0x12, 0x03, 0x31, 0x14, 0x15, 0x62, 0x06, 0x70, 0x72, 0x6f,
    0x74, 0x6f, 0x33, 0x0a, 0xc2, 0x11, 0x0a, 0x31, 0x6d, 0x6c, 0x73, 0x2f, 0x6d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x5f, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x73, 0x2f, 0x63, 0x6f, 0x6e,
    0x74, 0x65, 0x6e, 0x74, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2f, 0x72, 0x65, 0x61, 0x63, 0x74,
    0x69, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x12, 0x27, 0x78, 0x6d, 0x74, 0x70, 0x2e,
    0x6d, 0x6c, 0x73, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x5f, 0x63, 0x6f, 0x6e, 0x74,
    0x65, 0x6e, 0x74, 0x73, 0x2e, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x5f, 0x74, 0x79, 0x70,
    0x65, 0x73, 0x22, 0x94, 0x02, 0x0a, 0x0a, 0x52, 0x65, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x56,
    0x32, 0x12, 0x1c, 0x0a, 0x09, 0x72, 0x65, 0x66, 0x65, 0x72, 0x65, 0x6e, 0x63, 0x65, 0x18, 0x01,
    0x20, 0x01, 0x28, 0x09, 0x52, 0x09, 0x72, 0x65, 0x66, 0x65, 0x72, 0x65, 0x6e, 0x63, 0x65, 0x12,
    0x2c, 0x0a, 0x12, 0x72, 0x65, 0x66, 0x65, 0x72, 0x65, 0x6e, 0x63, 0x65, 0x5f, 0x69, 0x6e, 0x62,
    0x6f, 0x78, 0x5f, 0x69, 0x64, 0x18, 0x02, 0x20, 0x01, 0x28, 0x09, 0x52, 0x10, 0x72, 0x65, 0x66,
    0x65, 0x72, 0x65, 0x6e, 0x63, 0x65, 0x49, 0x6e, 0x62, 0x6f, 0x78, 0x49, 0x64, 0x12, 0x4f, 0x0a,
    0x06, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x18, 0x03, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x37, 0x2e,
    0x78, 0x6d, 0x74, 0x70, 0x2e, 0x6d, 0x6c, 0x73, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65,
    0x5f, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x73, 0x2e, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e,
    0x74, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x73, 0x2e, 0x52, 0x65, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e,
    0x41, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x52, 0x06, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x18,
    0x0a, 0x07, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x18, 0x04, 0x20, 0x01, 0x28, 0x09, 0x52,
    0x07, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x12, 0x4f, 0x0a, 0x06, 0x73, 0x63, 0x68, 0x65,
    0x6d, 0x61, 0x18, 0x05, 0x20, 0x01, 0x28, 0x0e, 0x32, 0x37, 0x2e, 0x78, 0x6d, 0x74, 0x70, 0x2e,
    0x6d, 0x6c, 0x73, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x5f, 0x63, 0x6f, 0x6e, 0x74,
    0x65, 0x6e, 0x74, 0x73, 0x2e, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x5f, 0x74, 0x79, 0x70,
    0x65, 0x73, 0x2e, 0x52, 0x65, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x53, 0x63, 0x68, 0x65, 0x6d,
    0x61, 0x52, 0x06, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x2a, 0x69, 0x0a, 0x0e, 0x52, 0x65, 0x61,
    0x63, 0x74, 0x69, 0x6f, 0x6e, 0x41, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x12, 0x1f, 0x0a, 0x1b, 0x52,
    0x45, 0x41, 0x43, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x41, 0x43, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x55,
    0x4e, 0x53, 0x50, 0x45, 0x43, 0x49, 0x46, 0x49, 0x45, 0x44, 0x10, 0x00, 0x12, 0x19, 0x0a, 0x15,
    0x52, 0x45, 0x41, 0x43, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x41, 0x43, 0x54, 0x49, 0x4f, 0x4e, 0x5f,
    0x41, 0x44, 0x44, 0x45, 0x44, 0x10, 0x01, 0x12, 0x1b, 0x0a, 0x17, 0x52, 0x45, 0x41, 0x43, 0x54,
    0x49, 0x4f, 0x4e, 0x5f, 0x41, 0x43, 0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x52, 0x45, 0x4d, 0x4f, 0x56,
    0x45, 0x44, 0x10, 0x02, 0x2a, 0x89, 0x01, 0x0a, 0x0e, 0x52, 0x65, 0x61, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x53, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x12, 0x1f, 0x0a, 0x1b, 0x52, 0x45, 0x41, 0x43, 0x54,
    0x49, 0x4f, 0x4e, 0x5f, 0x53, 0x43, 0x48, 0x45, 0x4d, 0x41, 0x5f, 0x55, 0x4e, 0x53, 0x50, 0x45,
    0x43, 0x49, 0x46, 0x49, 0x45, 0x44, 0x10, 0x00, 0x12, 0x1b, 0x0a, 0x17, 0x52, 0x45, 0x41, 0x43,
    0x54, 0x49, 0x4f, 0x4e, 0x5f, 0x53, 0x43, 0x48, 0x45, 0x4d, 0x41, 0x5f, 0x55, 0x4e, 0x49, 0x43,
    0x4f, 0x44, 0x45, 0x10, 0x01, 0x12, 0x1d, 0x0a, 0x19, 0x52, 0x45, 0x41, 0x43, 0x54, 0x49, 0x4f,
    0x4e, 0x5f, 0x53, 0x43, 0x48, 0x45, 0x4d, 0x41, 0x5f, 0x53, 0x48, 0x4f, 0x52, 0x54, 0x43, 0x4f,
    0x44, 0x45, 0x10, 0x02, 0x12, 0x1a, 0x0a, 0x16, 0x52, 0x45, 0x41, 0x43, 0x54, 0x49, 0x4f, 0x4e,
    0x5f, 0x53, 0x43, 0x48, 0x45, 0x4d, 0x41, 0x5f, 0x43, 0x55, 0x53, 0x54, 0x4f, 0x4d, 0x10, 0x03,
    0x42, 0xb4, 0x02, 0x0a, 0x2b, 0x63, 0x6f, 0x6d, 0x2e, 0x78, 0x6d, 0x74, 0x70, 0x2e, 0x6d, 0x6c,
    0x73, 0x2e, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x5f, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e,
    0x74, 0x73, 0x2e, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x5f, 0x74, 0x79, 0x70, 0x65, 0x73,
    0x42, 0x0d, 0x52, 0x65, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x50, 0x72, 0x6f, 0x74, 0x6f, 0x50,
    0x01, 0x5a, 0x3e, 0x67, 0x69, 0x74, 0x68, 0x75, 0x62, 0x2e, 0x63, 0x6f, 0x6d, 0x2f, 0x78, 0x6d,
    0x74, 0x70, 0x2f, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x2f, 0x76, 0x33, 0x2f, 0x67, 0x6f, 0x2f, 0x6d,
    0x6c, 0x73, 0x2f, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x5f, 0x63, 0x6f, 0x6e, 0x74, 0x65,
    0x6e, 0x74, 0x73, 0x2f, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x5f, 0x74, 0x79, 0x70, 0x65,
    0x73, 0xa2, 0x02, 0x04, 0x58, 0x4d, 0x4d, 0x43, 0xaa, 0x02, 0x25, 0x58, 0x6d, 0x74, 0x70, 0x2e,
    0x4d, 0x6c, 0x73, 0x2e, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x43, 0x6f, 0x6e, 0x74, 0x65,
    0x6e, 0x74, 0x73, 0x2e, 0x43, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x54, 0x79, 0x70, 0x65, 0x73,
    0xca, 0x02, 0x25, 0x58, 0x6d, 0x74, 0x70, 0x5c, 0x4d, 0x6c, 0x73, 0x5c, 0x4d, 0x65, 0x73, 0x73,
    0x61, 0x67, 0x65, 0x43, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x73, 0x5c, 0x43, 0x6f, 0x6e, 0x74,
    0x65, 0x6e, 0x74, 0x54, 0x79, 0x70, 0x65, 0x73, 0xe2, 0x02, 0x31, 0x58, 0x6d, 0x74, 0x70, 0x5c,
    0x4d, 0x6c, 0x73, 0x5c, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x43, 0x6f, 0x6e, 0x74, 0x65,
    0x6e, 0x74, 0x73, 0x5c, 0x43, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x54, 0x79, 0x70, 0x65, 0x73,
    0x5c, 0x47, 0x50, 0x42, 0x4d, 0x65, 0x74, 0x61, 0x64, 0x61, 0x74, 0x61, 0xea, 0x02, 0x28, 0x58,
    0x6d, 0x74, 0x70, 0x3a, 0x3a, 0x4d, 0x6c, 0x73, 0x3a, 0x3a, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67,
    0x65, 0x43, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x73, 0x3a, 0x3a, 0x43, 0x6f, 0x6e, 0x74, 0x65,
    0x6e, 0x74, 0x54, 0x79, 0x70, 0x65, 0x73, 0x4a, 0x96, 0x0a, 0x0a, 0x06, 0x12, 0x04, 0x0a, 0x00,
    0x31, 0x01, 0x0a, 0x81, 0x02, 0x0a, 0x01, 0x0c, 0x12, 0x03, 0x0a, 0x00, 0x12, 0x1a, 0xf6, 0x01,
    0x20, 0x72, 0x65, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x2e, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x0a,
    0x20, 0x54, 0x68, 0x69, 0x73, 0x20, 0x66, 0x69, 0x6c, 0x65, 0x20, 0x64, 0x65, 0x66, 0x69, 0x6e,
    0x65, 0x73, 0x20, 0x74, 0x68, 0x65, 0x20, 0x52, 0x65, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x56,
    0x32, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x74, 0x79, 0x70, 0x65, 0x20, 0x61,
    0x6e, 0x64, 0x20, 0x69, 0x73, 0x20, 0x61, 0x73, 0x73, 0x6f, 0x63, 0x69, 0x61, 0x74, 0x65, 0x64,
    0x20, 0x77, 0x69, 0x74, 0x68, 0x20, 0x74, 0x68, 0x65, 0x20, 0x66, 0x6f, 0x6c, 0x6c, 0x6f, 0x77,
    0x69, 0x6e, 0x67, 0x20, 0x43, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x54, 0x79, 0x70, 0x65, 0x49,
    0x64, 0x3a, 0x0a, 0x20, 0x0a, 0x20, 0x43, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x54, 0x79, 0x70,
    0x65, 0x49, 0x64, 0x20, 0x7b, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x61, 0x75, 0x74, 0x68, 0x6f,
    0x72, 0x69, 0x74, 0x79, 0x5f, 0x69, 0x64, 0x3a, 0x20, 0x22, 0x78, 0x6d, 0x74, 0x70, 0x2e, 0x6f,
    0x72, 0x67, 0x22, 0x2c, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x74, 0x79, 0x70, 0x65, 0x5f, 0x69,
    0x64, 0x3a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x20, 0x22, 0x72, 0x65, 0x61, 0x63, 0x74, 0x69, 0x6f,
    0x6e, 0x22, 0x2c, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20, 0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e,
    0x5f, 0x6d, 0x61, 0x6a, 0x6f, 0x72, 0x3a, 0x20, 0x32, 0x2c, 0x0a, 0x20, 0x20, 0x20, 0x20, 0x20,
    0x76, 0x65, 0x72, 0x73, 0x69, 0x6f, 0x6e, 0x5f, 0x6d, 0x69, 0x6e, 0x6f, 0x72, 0x3a, 0x20, 0x30,
    0x2c, 0x0a, 0x20, 0x7d, 0x0a, 0x0a, 0x0a, 0x08, 0x0a, 0x01, 0x02, 0x12, 0x03, 0x0c, 0x00, 0x30,
    0x0a, 0x08, 0x0a, 0x01, 0x08, 0x12, 0x03, 0x0e, 0x00, 0x55, 0x0a, 0x09, 0x0a, 0x02, 0x08, 0x0b,
    0x12, 0x03, 0x0e, 0x00, 0x55, 0x0a, 0x36, 0x0a, 0x02, 0x05, 0x00, 0x12, 0x04, 0x12, 0x00, 0x16,
    0x01, 0x1a, 0x2a, 0x20, 0x41, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x65, 0x6e, 0x75, 0x6d, 0x20,
    0x74, 0x6f, 0x20, 0x72, 0x65, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x20, 0x72, 0x65, 0x61,
    0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x73, 0x74, 0x61, 0x74, 0x65, 0x73, 0x0a, 0x0a, 0x0a, 0x0a,
    0x03, 0x05, 0x00, 0x01, 0x12, 0x03, 0x12, 0x05, 0x13, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02,
    0x00, 0x12, 0x03, 0x13, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x01, 0x12,
    0x03, 0x13, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x00, 0x02, 0x12, 0x03, 0x13,
    0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x00, 0x02, 0x01, 0x12, 0x03, 0x14, 0x02, 0x1c, 0x0a,
    0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x01, 0x01, 0x12, 0x03, 0x14, 0x02, 0x17, 0x0a, 0x0c, 0x0a,
    0x05, 0x05, 0x00, 0x02, 0x01, 0x02, 0x12, 0x03, 0x14, 0x1a, 0x1b, 0x0a, 0x0b, 0x0a, 0x04, 0x05,
    0x00, 0x02, 0x02, 0x12, 0x03, 0x15, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02,
    0x01, 0x12, 0x03, 0x15, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x00, 0x02, 0x02, 0x02, 0x12,
    0x03, 0x15, 0x1c, 0x1d, 0x0a, 0x3d, 0x0a, 0x02, 0x05, 0x01, 0x12, 0x04, 0x19, 0x00, 0x1e, 0x01,
    0x1a, 0x31, 0x20, 0x53, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x20, 0x65, 0x6e, 0x75, 0x6d, 0x20, 0x74,
    0x6f, 0x20, 0x72, 0x65, 0x70, 0x72, 0x65, 0x73, 0x65, 0x6e, 0x74, 0x20, 0x72, 0x65, 0x61, 0x63,
    0x74, 0x69, 0x6f, 0x6e, 0x20, 0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x20, 0x74, 0x79, 0x70,
    0x65, 0x73, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x05, 0x01, 0x01, 0x12, 0x03, 0x19, 0x05, 0x13, 0x0a,
    0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x00, 0x12, 0x03, 0x1a, 0x02, 0x22, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x01, 0x02, 0x00, 0x01, 0x12, 0x03, 0x1a, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01,
    0x02, 0x00, 0x02, 0x12, 0x03, 0x1a, 0x20, 0x21, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x01,
    0x12, 0x03, 0x1b, 0x02, 0x1e, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x01, 0x12, 0x03,
    0x1b, 0x02, 0x19, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x01, 0x02, 0x12, 0x03, 0x1b, 0x1c,
    0x1d, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01, 0x02, 0x02, 0x12, 0x03, 0x1c, 0x02, 0x20, 0x0a, 0x0c,
    0x0a, 0x05, 0x05, 0x01, 0x02, 0x02, 0x01, 0x12, 0x03, 0x1c, 0x02, 0x1b, 0x0a, 0x0c, 0x0a, 0x05,
    0x05, 0x01, 0x02, 0x02, 0x02, 0x12, 0x03, 0x1c, 0x1e, 0x1f, 0x0a, 0x0b, 0x0a, 0x04, 0x05, 0x01,
    0x02, 0x03, 0x12, 0x03, 0x1d, 0x02, 0x1d, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x1d, 0x02, 0x18, 0x0a, 0x0c, 0x0a, 0x05, 0x05, 0x01, 0x02, 0x03, 0x02, 0x12, 0x03,
    0x1d, 0x1b, 0x1c, 0x0a, 0x23, 0x0a, 0x02, 0x04, 0x00, 0x12, 0x04, 0x21, 0x00, 0x31, 0x01, 0x1a,
    0x17, 0x20, 0x52, 0x65, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61,
    0x67, 0x65, 0x20, 0x74, 0x79, 0x70, 0x65, 0x0a, 0x0a, 0x0a, 0x0a, 0x03, 0x04, 0x00, 0x01, 0x12,
    0x03, 0x21, 0x08, 0x12, 0x0a, 0x2e, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x00, 0x12, 0x03, 0x23, 0x02,
    0x17, 0x1a, 0x21, 0x20, 0x54, 0x68, 0x65, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20,
    0x49, 0x44, 0x20, 0x62, 0x65, 0x69, 0x6e, 0x67, 0x20, 0x72, 0x65, 0x61, 0x63, 0x74, 0x65, 0x64,
    0x20, 0x74, 0x6f, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x05, 0x12, 0x03, 0x23,
    0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x01, 0x12, 0x03, 0x23, 0x09, 0x12,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x00, 0x03, 0x12, 0x03, 0x23, 0x15, 0x16, 0x0a, 0x6a,
    0x0a, 0x04, 0x04, 0x00, 0x02, 0x01, 0x12, 0x03, 0x27, 0x02, 0x20, 0x1a, 0x5d, 0x20, 0x54, 0x68,
    0x65, 0x20, 0x69, 0x6e, 0x62, 0x6f, 0x78, 0x20, 0x49, 0x44, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68,
    0x65, 0x20, 0x75, 0x73, 0x65, 0x72, 0x20, 0x77, 0x68, 0x6f, 0x20, 0x73, 0x65, 0x6e, 0x74, 0x20,
    0x74, 0x68, 0x65, 0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x20, 0x62, 0x65, 0x69, 0x6e,
    0x67, 0x20, 0x72, 0x65, 0x61, 0x63, 0x74, 0x65, 0x64, 0x20, 0x74, 0x6f, 0x0a, 0x20, 0x4f, 0x70,
    0x74, 0x69, 0x6f, 0x6e, 0x61, 0x6c, 0x20, 0x66, 0x6f, 0x72, 0x20, 0x67, 0x72, 0x6f, 0x75, 0x70,
    0x20, 0x6d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x73, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00,
    0x02, 0x01, 0x05, 0x12, 0x03, 0x27, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01,
    0x01, 0x12, 0x03, 0x27, 0x09, 0x1b, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x01, 0x03, 0x12,
    0x03, 0x27, 0x1e, 0x1f, 0x0a, 0x3c, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x02, 0x12, 0x03, 0x2a, 0x02,
    0x1c, 0x1a, 0x2f, 0x20, 0x54, 0x68, 0x65, 0x20, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x6f,
    0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x28,
    0x61, 0x64, 0x64, 0x65, 0x64, 0x20, 0x6f, 0x72, 0x20, 0x72, 0x65, 0x6d, 0x6f, 0x76, 0x65, 0x64,
    0x29, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x06, 0x12, 0x03, 0x2a, 0x02, 0x10,
    0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x01, 0x12, 0x03, 0x2a, 0x11, 0x17, 0x0a, 0x0c,
    0x0a, 0x05, 0x04, 0x00, 0x02, 0x02, 0x03, 0x12, 0x03, 0x2a, 0x1a, 0x1b, 0x0a, 0x2b, 0x0a, 0x04,
    0x04, 0x00, 0x02, 0x03, 0x12, 0x03, 0x2d, 0x02, 0x15, 0x1a, 0x1e, 0x20, 0x54, 0x68, 0x65, 0x20,
    0x63, 0x6f, 0x6e, 0x74, 0x65, 0x6e, 0x74, 0x20, 0x6f, 0x66, 0x20, 0x74, 0x68, 0x65, 0x20, 0x72,
    0x65, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02,
    0x03, 0x05, 0x12, 0x03, 0x2d, 0x02, 0x08, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x01,
    0x12, 0x03, 0x2d, 0x09, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x03, 0x03, 0x12, 0x03,
    0x2d, 0x13, 0x14, 0x0a, 0x31, 0x0a, 0x04, 0x04, 0x00, 0x02, 0x04, 0x12, 0x03, 0x30, 0x02, 0x1c,
    0x1a, 0x24, 0x20, 0x54, 0x68, 0x65, 0x20, 0x73, 0x63, 0x68, 0x65, 0x6d, 0x61, 0x20, 0x6f, 0x66,
    0x20, 0x74, 0x68, 0x65, 0x20, 0x72, 0x65, 0x61, 0x63, 0x74, 0x69, 0x6f, 0x6e, 0x20, 0x63, 0x6f,
    0x6e, 0x74, 0x65, 0x6e, 0x74, 0x0a, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x06, 0x12,
    0x03, 0x30, 0x02, 0x10, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x01, 0x12, 0x03, 0x30,
    0x11, 0x17, 0x0a, 0x0c, 0x0a, 0x05, 0x04, 0x00, 0x02, 0x04, 0x03, 0x12, 0x03, 0x30, 0x1a, 0x1b,
    0x62, 0x06, 0x70, 0x72, 0x6f, 0x74, 0x6f, 0x33,
];
include!("xmtp.mls.message_contents.content_types.serde.rs");
// @@protoc_insertion_point(module)