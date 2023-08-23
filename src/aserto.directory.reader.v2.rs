// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectTypeRequest {
    /// object type selector
    #[prost(message, optional, tag="1")]
    pub param: ::core::option::Option<super::super::common::v2::ObjectTypeIdentifier>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectTypeResponse {
    /// object type instance
    #[prost(message, optional, tag="1")]
    pub result: ::core::option::Option<super::super::common::v2::ObjectType>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectTypesRequest {
    /// pagination request
    #[prost(message, optional, tag="9")]
    pub page: ::core::option::Option<super::super::common::v2::PaginationRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectTypesResponse {
    /// array of object types
    #[prost(message, repeated, tag="1")]
    pub results: ::prost::alloc::vec::Vec<super::super::common::v2::ObjectType>,
    /// pagination response
    #[prost(message, optional, tag="9")]
    pub page: ::core::option::Option<super::super::common::v2::PaginationResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRelationTypeRequest {
    /// relation type selector
    #[prost(message, optional, tag="1")]
    pub param: ::core::option::Option<super::super::common::v2::RelationTypeIdentifier>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRelationTypeResponse {
    /// relation type instance
    #[prost(message, optional, tag="1")]
    pub result: ::core::option::Option<super::super::common::v2::RelationType>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRelationTypesRequest {
    /// object type selector
    #[prost(message, optional, tag="1")]
    pub param: ::core::option::Option<super::super::common::v2::ObjectTypeIdentifier>,
    /// pagination request
    #[prost(message, optional, tag="9")]
    pub page: ::core::option::Option<super::super::common::v2::PaginationRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRelationTypesResponse {
    /// array of relation types
    #[prost(message, repeated, tag="1")]
    pub results: ::prost::alloc::vec::Vec<super::super::common::v2::RelationType>,
    /// pagination response
    #[prost(message, optional, tag="9")]
    pub page: ::core::option::Option<super::super::common::v2::PaginationResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectRequest {
    /// object selector
    #[prost(message, optional, tag="1")]
    pub param: ::core::option::Option<super::super::common::v2::ObjectIdentifier>,
    /// materialize the object relations objects
    #[prost(bool, optional, tag="2")]
    pub with_relations: ::core::option::Option<bool>,
    /// pagination request
    #[prost(message, optional, tag="9")]
    pub page: ::core::option::Option<super::super::common::v2::PaginationRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectResponse {
    /// object instance
    #[prost(message, optional, tag="1")]
    pub result: ::core::option::Option<super::super::common::v2::Object>,
    /// object relations
    #[prost(message, repeated, tag="4")]
    pub relations: ::prost::alloc::vec::Vec<super::super::common::v2::Relation>,
    /// pagination response
    #[prost(message, optional, tag="9")]
    pub page: ::core::option::Option<super::super::common::v2::PaginationResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectManyRequest {
    /// object identifier list
    #[prost(message, repeated, tag="1")]
    pub param: ::prost::alloc::vec::Vec<super::super::common::v2::ObjectIdentifier>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectManyResponse {
    /// array of object instances
    #[prost(message, repeated, tag="1")]
    pub results: ::prost::alloc::vec::Vec<super::super::common::v2::Object>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectsRequest {
    /// object type selector
    #[prost(message, optional, tag="1")]
    pub param: ::core::option::Option<super::super::common::v2::ObjectTypeIdentifier>,
    /// pagination request
    #[prost(message, optional, tag="9")]
    pub page: ::core::option::Option<super::super::common::v2::PaginationRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetObjectsResponse {
    /// array of object instances
    #[prost(message, repeated, tag="1")]
    pub results: ::prost::alloc::vec::Vec<super::super::common::v2::Object>,
    /// pagination response
    #[prost(message, optional, tag="9")]
    pub page: ::core::option::Option<super::super::common::v2::PaginationResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRelationRequest {
    /// relation selector
    #[prost(message, optional, tag="1")]
    pub param: ::core::option::Option<super::super::common::v2::RelationIdentifier>,
    /// materialize relation objects
    #[prost(bool, optional, tag="2")]
    pub with_objects: ::core::option::Option<bool>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRelationResponse {
    /// array of relation instances
    #[prost(message, repeated, tag="1")]
    pub results: ::prost::alloc::vec::Vec<super::super::common::v2::Relation>,
    /// map of materialized relation objects
    #[prost(map="string, message", tag="2")]
    pub objects: ::std::collections::HashMap<::prost::alloc::string::String, super::super::common::v2::Object>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRelationsRequest {
    /// relation selector
    #[prost(message, optional, tag="1")]
    pub param: ::core::option::Option<super::super::common::v2::RelationIdentifier>,
    /// pagination request
    #[prost(message, optional, tag="9")]
    pub page: ::core::option::Option<super::super::common::v2::PaginationRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetRelationsResponse {
    /// array of relation instances
    #[prost(message, repeated, tag="1")]
    pub results: ::prost::alloc::vec::Vec<super::super::common::v2::Relation>,
    /// pagination response
    #[prost(message, optional, tag="9")]
    pub page: ::core::option::Option<super::super::common::v2::PaginationResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPermissionRequest {
    /// permission selector
    #[prost(message, optional, tag="1")]
    pub param: ::core::option::Option<super::super::common::v2::PermissionIdentifier>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPermissionResponse {
    /// permission instance
    #[prost(message, optional, tag="1")]
    pub result: ::core::option::Option<super::super::common::v2::Permission>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPermissionsRequest {
    /// pagination request
    #[prost(message, optional, tag="9")]
    pub page: ::core::option::Option<super::super::common::v2::PaginationRequest>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetPermissionsResponse {
    /// array of permissions
    #[prost(message, repeated, tag="1")]
    pub results: ::prost::alloc::vec::Vec<super::super::common::v2::Permission>,
    /// pagination response
    #[prost(message, optional, tag="9")]
    pub page: ::core::option::Option<super::super::common::v2::PaginationResponse>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckPermissionRequest {
    /// subject selector
    #[prost(message, optional, tag="1")]
    pub subject: ::core::option::Option<super::super::common::v2::ObjectIdentifier>,
    /// permission selector
    #[prost(message, optional, tag="2")]
    pub permission: ::core::option::Option<super::super::common::v2::PermissionIdentifier>,
    /// object selector
    #[prost(message, optional, tag="3")]
    pub object: ::core::option::Option<super::super::common::v2::ObjectIdentifier>,
    /// collect trace information
    #[prost(bool, tag="7")]
    pub trace: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckPermissionResponse {
    /// check result
    #[prost(bool, tag="1")]
    pub check: bool,
    /// trace information
    #[prost(string, repeated, tag="2")]
    pub trace: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckRelationRequest {
    /// subject selector
    #[prost(message, optional, tag="1")]
    pub subject: ::core::option::Option<super::super::common::v2::ObjectIdentifier>,
    /// relation selector
    #[prost(message, optional, tag="2")]
    pub relation: ::core::option::Option<super::super::common::v2::RelationTypeIdentifier>,
    /// object selector
    #[prost(message, optional, tag="3")]
    pub object: ::core::option::Option<super::super::common::v2::ObjectIdentifier>,
    /// collect trace information
    #[prost(bool, tag="7")]
    pub trace: bool,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckRelationResponse {
    /// check result
    #[prost(bool, tag="1")]
    pub check: bool,
    /// trace information
    #[prost(string, repeated, tag="2")]
    pub trace: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CheckResponse {
    /// check result
    #[prost(bool, tag="1")]
    pub check: bool,
    /// trace information
    #[prost(string, repeated, tag="2")]
    pub trace: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGraphRequest {
    /// anchor selector
    #[prost(message, optional, tag="1")]
    pub anchor: ::core::option::Option<super::super::common::v2::ObjectIdentifier>,
    /// subject selector
    #[prost(message, optional, tag="2")]
    pub subject: ::core::option::Option<super::super::common::v2::ObjectIdentifier>,
    /// relation selector
    #[prost(message, optional, tag="3")]
    pub relation: ::core::option::Option<super::super::common::v2::RelationTypeIdentifier>,
    /// object selector
    #[prost(message, optional, tag="4")]
    pub object: ::core::option::Option<super::super::common::v2::ObjectIdentifier>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetGraphResponse {
    /// dependency graph
    #[prost(message, repeated, tag="1")]
    pub results: ::prost::alloc::vec::Vec<super::super::common::v2::ObjectDependency>,
}
include!("aserto.directory.reader.v2.serde.rs");
include!("aserto.directory.reader.v2.tonic.rs");
// @@protoc_insertion_point(module)