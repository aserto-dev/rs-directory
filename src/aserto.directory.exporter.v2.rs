// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportRequest {
    /// data export options mask
    #[prost(uint32, tag="1")]
    pub options: u32,
    /// start export from timestamp (UTC)
    #[prost(message, optional, tag="20")]
    pub start_from: ::core::option::Option<::pbjson_types::Timestamp>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExportResponse {
    #[prost(oneof="export_response::Msg", tags="2, 3, 4, 5, 6")]
    pub msg: ::core::option::Option<export_response::Msg>,
}
/// Nested message and enum types in `ExportResponse`.
pub mod export_response {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Msg {
        /// object instance (data)
        #[prost(message, tag="2")]
        Object(super::super::super::common::v2::Object),
        /// object type instance (metadata)
        #[prost(message, tag="3")]
        ObjectType(super::super::super::common::v2::ObjectType),
        /// relation instance (data)
        #[prost(message, tag="4")]
        Relation(super::super::super::common::v2::Relation),
        /// relation type instance (metadata)
        #[prost(message, tag="5")]
        RelationType(super::super::super::common::v2::RelationType),
        /// permission instance (metadata)
        #[prost(message, tag="6")]
        Permission(super::super::super::common::v2::Permission),
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Option {
    /// nothing selected (default initialization value)
    Unknown = 0,
    /// object type metadata
    MetadataObjectTypes = 1,
    /// relation type metadata
    MetadataRelationTypes = 2,
    /// permission metadata
    MetadataPermissions = 4,
    /// all metadata = OPTION_METADATA_OBJECT_TYPES | OPTION_METADATA_RELATION_TYPES | OPTION_METADATA_PERMISSIONS
    Metadata = 7,
    /// object instances
    DataObjects = 8,
    /// relation instances
    DataRelations = 16,
    /// relation instances with key values
    DataRelationsWithKeys = 32,
    /// all data = OPTION_DATA_OBJECTS | OPTION_DATA_RELATIONS
    Data = 24,
    /// all data with keys = OPTION_DATA_OBJECTS | OPTION_DATA_RELATIONS_WITH_KEYS
    DataWithKeys = 40,
    /// all metadata and data = OPTION_METADATA | OPTION_DATA
    All = 31,
    /// all metadata and data with keys = OPTION_METADATA | OPTION_DATA_WITH_KEYS
    AllWithKeys = 47,
}
impl Option {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Option::Unknown => "OPTION_UNKNOWN",
            Option::MetadataObjectTypes => "OPTION_METADATA_OBJECT_TYPES",
            Option::MetadataRelationTypes => "OPTION_METADATA_RELATION_TYPES",
            Option::MetadataPermissions => "OPTION_METADATA_PERMISSIONS",
            Option::Metadata => "OPTION_METADATA",
            Option::DataObjects => "OPTION_DATA_OBJECTS",
            Option::DataRelations => "OPTION_DATA_RELATIONS",
            Option::DataRelationsWithKeys => "OPTION_DATA_RELATIONS_WITH_KEYS",
            Option::Data => "OPTION_DATA",
            Option::DataWithKeys => "OPTION_DATA_WITH_KEYS",
            Option::All => "OPTION_ALL",
            Option::AllWithKeys => "OPTION_ALL_WITH_KEYS",
        }
    }
}
include!("aserto.directory.exporter.v2.serde.rs");
include!("aserto.directory.exporter.v2.tonic.rs");
// @@protoc_insertion_point(module)