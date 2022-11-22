// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportRequest {
    /// operation Opcode enum value
    #[prost(enumeration="Opcode", tag="1")]
    pub op_code: i32,
    #[prost(oneof="import_request::Msg", tags="2, 3, 4, 5, 6")]
    pub msg: ::core::option::Option<import_request::Msg>,
}
/// Nested message and enum types in `ImportRequest`.
pub mod import_request {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Msg {
        /// object_type import message
        #[prost(message, tag="2")]
        ObjectType(super::super::super::common::v2::ObjectType),
        /// permission import message
        #[prost(message, tag="3")]
        Permission(super::super::super::common::v2::Permission),
        /// relation_type import message
        #[prost(message, tag="4")]
        RelationType(super::super::super::common::v2::RelationType),
        /// object import message
        #[prost(message, tag="5")]
        Object(super::super::super::common::v2::Object),
        /// relation import message
        #[prost(message, tag="6")]
        Relation(super::super::super::common::v2::Relation),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportResponse {
    /// object_type import counter
    #[prost(message, optional, tag="1")]
    pub object_type: ::core::option::Option<ImportCounter>,
    /// object_type import counter
    #[prost(message, optional, tag="2")]
    pub permission: ::core::option::Option<ImportCounter>,
    /// object_type import counter
    #[prost(message, optional, tag="3")]
    pub relation_type: ::core::option::Option<ImportCounter>,
    /// object import counter
    #[prost(message, optional, tag="4")]
    pub object: ::core::option::Option<ImportCounter>,
    /// object_type import counter
    #[prost(message, optional, tag="5")]
    pub relation: ::core::option::Option<ImportCounter>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ImportCounter {
    /// number of messages received
    #[prost(uint64, tag="1")]
    pub recv: u64,
    /// number of messages with OPCODE_SET
    #[prost(uint64, tag="2")]
    pub set: u64,
    /// number of messages with OPCODE_DELETE
    #[prost(uint64, tag="3")]
    pub delete: u64,
    /// number of messages resulting in error
    #[prost(uint64, tag="4")]
    pub error: u64,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Opcode {
    Unknown = 0,
    Set = 1,
    Delete = 2,
}
impl Opcode {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Opcode::Unknown => "OPCODE_UNKNOWN",
            Opcode::Set => "OPCODE_SET",
            Opcode::Delete => "OPCODE_DELETE",
        }
    }
}
include!("aserto.directory.importer.v2.serde.rs");
include!("aserto.directory.importer.v2.tonic.rs");
// @@protoc_insertion_point(module)