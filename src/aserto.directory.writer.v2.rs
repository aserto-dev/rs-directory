// @generated
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetObjectTypeRequest {
    /// 
    #[prost(message, optional, tag="1")]
    pub object_type: ::core::option::Option<super::super::common::v2::ObjectType>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetObjectTypeResponse {
    /// object type instance
    #[prost(message, optional, tag="1")]
    pub result: ::core::option::Option<super::super::common::v2::ObjectType>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteObjectTypeRequest {
    /// 
    #[prost(message, optional, tag="1")]
    pub param: ::core::option::Option<super::super::common::v2::ObjectTypeIdentifier>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteObjectTypeResponse {
    /// empty result
    #[prost(message, optional, tag="1")]
    pub result: ::core::option::Option<::pbjson_types::Empty>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRelationTypeRequest {
    /// 
    #[prost(message, optional, tag="1")]
    pub relation_type: ::core::option::Option<super::super::common::v2::RelationType>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRelationTypeResponse {
    /// relation types instance
    #[prost(message, optional, tag="1")]
    pub result: ::core::option::Option<super::super::common::v2::RelationType>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRelationTypeRequest {
    /// 
    #[prost(message, optional, tag="1")]
    pub param: ::core::option::Option<super::super::common::v2::RelationTypeIdentifier>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRelationTypeResponse {
    /// empty result
    #[prost(message, optional, tag="1")]
    pub result: ::core::option::Option<::pbjson_types::Empty>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPermissionRequest {
    /// 
    #[prost(message, optional, tag="1")]
    pub permission: ::core::option::Option<super::super::common::v2::Permission>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetPermissionResponse {
    /// permission instance
    #[prost(message, optional, tag="1")]
    pub result: ::core::option::Option<super::super::common::v2::Permission>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePermissionRequest {
    /// 
    #[prost(message, optional, tag="1")]
    pub param: ::core::option::Option<super::super::common::v2::PermissionIdentifier>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeletePermissionResponse {
    /// empty result
    #[prost(message, optional, tag="1")]
    pub result: ::core::option::Option<::pbjson_types::Empty>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetObjectRequest {
    /// 
    #[prost(message, optional, tag="1")]
    pub object: ::core::option::Option<super::super::common::v2::Object>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetObjectResponse {
    /// object instance
    #[prost(message, optional, tag="1")]
    pub result: ::core::option::Option<super::super::common::v2::Object>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteObjectRequest {
    /// 
    #[prost(message, optional, tag="1")]
    pub param: ::core::option::Option<super::super::common::v2::ObjectIdentifier>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteObjectResponse {
    /// empty result
    #[prost(message, optional, tag="1")]
    pub result: ::core::option::Option<::pbjson_types::Empty>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRelationRequest {
    /// 
    #[prost(message, optional, tag="1")]
    pub relation: ::core::option::Option<super::super::common::v2::Relation>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct SetRelationResponse {
    /// relation instance
    #[prost(message, optional, tag="1")]
    pub result: ::core::option::Option<super::super::common::v2::Relation>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRelationRequest {
    /// 
    #[prost(message, optional, tag="1")]
    pub param: ::core::option::Option<super::super::common::v2::RelationIdentifier>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct DeleteRelationResponse {
    /// empty result
    #[prost(message, optional, tag="1")]
    pub result: ::core::option::Option<::pbjson_types::Empty>,
}
include!("aserto.directory.writer.v2.serde.rs");
include!("aserto.directory.writer.v2.tonic.rs");
// @@protoc_insertion_point(module)