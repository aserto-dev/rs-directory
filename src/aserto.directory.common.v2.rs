// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectType {
    /// object type name (unique, lc-string)
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// object type display name
    #[prost(string, tag="3")]
    pub display_name: ::prost::alloc::string::String,
    /// object type is a subject (user|group) (default false)
    #[prost(bool, tag="4")]
    pub is_subject: bool,
    /// sort ordinal (default 0)
    #[prost(int32, tag="5")]
    pub ordinal: i32,
    /// status flag bitmap (default 0)
    #[prost(uint32, tag="6")]
    pub status: u32,
    /// object type schema definition (JSON)
    #[prost(message, optional, tag="10")]
    pub schema: ::core::option::Option<::pbjson_types::Struct>,
    /// created at timestamp (UTC)
    #[prost(message, optional, tag="20")]
    pub created_at: ::core::option::Option<::pbjson_types::Timestamp>,
    /// last updated timestamp (UTC)
    #[prost(message, optional, tag="21")]
    pub updated_at: ::core::option::Option<::pbjson_types::Timestamp>,
    /// object instance hash
    #[prost(string, tag="23")]
    pub hash: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Permission {
    /// permission name (unique, cs-string)
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// permission display name
    #[prost(string, tag="3")]
    pub display_name: ::prost::alloc::string::String,
    /// created at timestamp (UTC)
    #[prost(message, optional, tag="20")]
    pub created_at: ::core::option::Option<::pbjson_types::Timestamp>,
    /// last updated timestamp (UTC)
    #[prost(message, optional, tag="21")]
    pub updated_at: ::core::option::Option<::pbjson_types::Timestamp>,
    /// object instance hash
    #[prost(string, tag="23")]
    pub hash: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelationType {
    /// relation type name selector
    #[prost(string, tag="2")]
    pub name: ::prost::alloc::string::String,
    /// object type referenced by relation
    #[prost(string, tag="3")]
    pub object_type: ::prost::alloc::string::String,
    /// relation display name
    #[prost(string, tag="4")]
    pub display_name: ::prost::alloc::string::String,
    /// sort ordinal (default 0)
    #[prost(int32, tag="5")]
    pub ordinal: i32,
    /// status bitmap (default 0)
    #[prost(uint32, tag="6")]
    pub status: u32,
    /// relations union-ed with relation type instance
    #[prost(string, repeated, tag="7")]
    pub unions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// permissions associated to relation type instance
    #[prost(string, repeated, tag="8")]
    pub permissions: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    /// created at timestamp (UTC)
    #[prost(message, optional, tag="20")]
    pub created_at: ::core::option::Option<::pbjson_types::Timestamp>,
    /// last updated timestamp (UTC)
    #[prost(message, optional, tag="21")]
    pub updated_at: ::core::option::Option<::pbjson_types::Timestamp>,
    /// object instance hash
    #[prost(string, tag="23")]
    pub hash: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Object {
    /// external object key (cs-string)
    #[prost(string, tag="2")]
    pub key: ::prost::alloc::string::String,
    /// object type name
    #[prost(string, tag="3")]
    pub r#type: ::prost::alloc::string::String,
    /// display name object
    #[prost(string, tag="4")]
    pub display_name: ::prost::alloc::string::String,
    /// property bag
    #[prost(message, optional, tag="5")]
    pub properties: ::core::option::Option<::pbjson_types::Struct>,
    /// created at timestamp (UTC)
    #[prost(message, optional, tag="20")]
    pub created_at: ::core::option::Option<::pbjson_types::Timestamp>,
    /// last updated timestamp (UTC)
    #[prost(message, optional, tag="21")]
    pub updated_at: ::core::option::Option<::pbjson_types::Timestamp>,
    /// object instance hash
    #[prost(string, tag="23")]
    pub hash: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Relation {
    /// subject identifier
    #[prost(message, optional, tag="1")]
    pub subject: ::core::option::Option<ObjectIdentifier>,
    /// relation type name
    #[prost(string, tag="2")]
    pub relation: ::prost::alloc::string::String,
    /// object identifier
    #[prost(message, optional, tag="3")]
    pub object: ::core::option::Option<ObjectIdentifier>,
    /// created at timestamp (UTC)
    #[prost(message, optional, tag="20")]
    pub created_at: ::core::option::Option<::pbjson_types::Timestamp>,
    /// last updated timestamp (UTC)
    #[prost(message, optional, tag="21")]
    pub updated_at: ::core::option::Option<::pbjson_types::Timestamp>,
    /// object instance hash
    #[prost(string, tag="23")]
    pub hash: ::prost::alloc::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectDependency {
    /// object type name of source object
    #[prost(string, tag="1")]
    pub object_type: ::prost::alloc::string::String,
    /// object search key of source object
    #[prost(string, tag="4")]
    pub object_key: ::prost::alloc::string::String,
    /// relation identifier
    #[prost(string, tag="5")]
    pub relation: ::prost::alloc::string::String,
    /// object type id of target object
    #[prost(string, tag="7")]
    pub subject_type: ::prost::alloc::string::String,
    /// object search key of target object
    #[prost(string, tag="10")]
    pub subject_key: ::prost::alloc::string::String,
    /// dependency depth
    #[prost(int32, tag="11")]
    pub depth: i32,
    /// dependency cycle
    #[prost(bool, tag="12")]
    pub is_cycle: bool,
    /// dependency path
    #[prost(string, repeated, tag="13")]
    pub path: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
/// ObjectType identifier
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectTypeIdentifier {
    /// object type name (unique, lc-string)
    #[prost(string, optional, tag="2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
}
/// Permission identifier
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PermissionIdentifier {
    /// permission name (unique, cs-string)
    #[prost(string, optional, tag="2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
}
/// RelationType identifier
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelationTypeIdentifier {
    /// relation type name selector
    #[prost(string, optional, tag="2")]
    pub name: ::core::option::Option<::prost::alloc::string::String>,
    /// object type referenced by relation
    #[prost(string, optional, tag="3")]
    pub object_type: ::core::option::Option<::prost::alloc::string::String>,
}
/// Object identifier
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ObjectIdentifier {
    /// object type
    #[prost(string, optional, tag="1")]
    pub r#type: ::core::option::Option<::prost::alloc::string::String>,
    /// external object key (cs-string)
    #[prost(string, optional, tag="3")]
    pub key: ::core::option::Option<::prost::alloc::string::String>,
}
/// Relation identifier
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RelationIdentifier {
    /// subject identifier
    #[prost(message, optional, tag="1")]
    pub subject: ::core::option::Option<ObjectIdentifier>,
    /// relation identifier
    #[prost(message, optional, tag="2")]
    pub relation: ::core::option::Option<RelationTypeIdentifier>,
    /// object identifier
    #[prost(message, optional, tag="3")]
    pub object: ::core::option::Option<ObjectIdentifier>,
}
/// Pagination request
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaginationRequest {
    /// requested page size, valid value between 1-100 rows (default 100)
    #[prost(int32, tag="1")]
    pub size: i32,
    /// pagination start token, default ""
    #[prost(string, tag="2")]
    pub token: ::prost::alloc::string::String,
}
/// Pagination response
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct PaginationResponse {
    /// next page token, when empty there are no more pages to fetch
    #[prost(string, tag="1")]
    pub next_token: ::prost::alloc::string::String,
    /// result size of the page returned
    #[prost(int32, tag="2")]
    pub result_size: i32,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum Flag {
    /// default, no special object behavior
    Unknown = 0,
    /// hidden object
    Hidden = 1,
    /// read-only object
    Readonly = 2,
    /// system object
    System = 4,
    /// shadow object by type+key associated to parent object
    Shadow = 8,
}
impl Flag {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            Flag::Unknown => "FLAG_UNKNOWN",
            Flag::Hidden => "FLAG_HIDDEN",
            Flag::Readonly => "FLAG_READONLY",
            Flag::System => "FLAG_SYSTEM",
            Flag::Shadow => "FLAG_SHADOW",
        }
    }
}
include!("aserto.directory.common.v2.serde.rs");
// @@protoc_insertion_point(module)