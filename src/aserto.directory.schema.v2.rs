// @generated
/// Properties of "group" objects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GroupProperties {
    /// ID of the IDP connection the group instance is associated with.
    #[prost(string, tag="1")]
    pub connection_id: ::prost::alloc::string::String,
}
/// Properties of "identity" objects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct IdentityProperties {
    /// identity kind \[email|username|uid|pid|dn|phone\]
    #[prost(enumeration="IdentityKind", tag="1")]
    pub kind: i32,
    /// identity provider name
    #[prost(string, tag="2")]
    pub provider: ::prost::alloc::string::String,
    /// identity has been verified (false when not explicitly specified)
    #[prost(bool, tag="3")]
    pub verified: bool,
    /// IDP connection id which owns the object instance
    #[prost(string, optional, tag="4")]
    pub connection_id: ::core::option::Option<::prost::alloc::string::String>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum IdentityKind {
    /// undefined state
    Unknown = 0,
    /// provider unique identifier
    Pid = 1,
    /// email address
    Email = 2,
    /// username
    Username = 3,
    /// distinguished name format RFC1779
    Dn = 4,
    /// phone number using the format described in RFC3966, using the E.164 recommendation
    Phone = 5,
    /// employee identifier
    Empid = 6,
}
impl IdentityKind {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            IdentityKind::Unknown => "IDENTITY_KIND_UNKNOWN",
            IdentityKind::Pid => "IDENTITY_KIND_PID",
            IdentityKind::Email => "IDENTITY_KIND_EMAIL",
            IdentityKind::Username => "IDENTITY_KIND_USERNAME",
            IdentityKind::Dn => "IDENTITY_KIND_DN",
            IdentityKind::Phone => "IDENTITY_KIND_PHONE",
            IdentityKind::Empid => "IDENTITY_KIND_EMPID",
        }
    }
}
/// Properties of a tenant object
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TenantProperties {
    /// The kind of tenant.
    #[prost(enumeration="TenantKind", tag="1")]
    pub kind: i32,
    /// Whether or not the v2 directory experience enabled for this tenant.
    #[prost(bool, tag="2")]
    pub directory_v2: bool,
    /// If true, the tenant cannot be reverted to the v1 directory experience.
    #[prost(bool, tag="3")]
    pub directory_v2_only: bool,
    /// Additional properties that are only set accounts (personal tennats).
    #[prost(message, optional, tag="4")]
    pub account: ::core::option::Option<AccountProperties>,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AccountProperties {
    /// Maximum number of organizations that can be created in this account.
    /// If -1, there is no limit.
    #[prost(int32, tag="1")]
    pub max_orgs: i32,
    /// Tracks the account owner's progress through the getting-started guide.
    #[prost(message, optional, tag="3")]
    pub getting_started: ::core::option::Option<GuideState>,
    /// The default organization for the account
    #[prost(string, tag="4")]
    pub default_tenant_id: ::prost::alloc::string::String,
}
/// The state of a user's progress through the console's getting started guide.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GuideState {
    /// Whether or not to display the getting started guide.
    #[prost(bool, tag="1")]
    pub show: bool,
    /// Progress information about individual steps in the guide.
    #[prost(message, optional, tag="2")]
    pub steps: ::core::option::Option<::pbjson_types::Struct>,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum TenantKind {
    Unknown = 0,
    Organization = 1,
    /// personal tenant
    Account = 2,
}
impl TenantKind {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            TenantKind::Unknown => "TENANT_KIND_UNKNOWN",
            TenantKind::Organization => "TENANT_KIND_ORGANIZATION",
            TenantKind::Account => "TENANT_KIND_ACCOUNT",
        }
    }
}
/// Properties of "user" objects.
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UserProperties {
    /// main email address of user
    #[prost(string, tag="1")]
    pub email: ::prost::alloc::string::String,
    /// URL to user's picture
    #[prost(string, tag="2")]
    pub picture: ::prost::alloc::string::String,
    /// user lifecycle status
    #[prost(enumeration="UserStatus", tag="3")]
    pub status: i32,
    /// enabled (false prevents the user from accessing anything)
    #[prost(bool, tag="4")]
    pub enabled: bool,
    /// ID of the IDP connection the user instance is associated with.
    #[prost(string, tag="5")]
    pub connection_id: ::prost::alloc::string::String,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum UserStatus {
    /// User status undefined
    Unknown = 0,
    /// Staged status, is when the user object is first created, before the activation flow is initiated, or if there is a pending admin action.
    Staged = 1,
    /// Provisioned status, is when the user object is provisioned, but the user has not provided verification by clicking through the activation email or provided a password.
    Provisioned = 2,
    /// Active status, is when:
    ///   * An admin adds a user and sets the user password without requiring email verification.
    ///   * An admin adds a user, sets the user password, and requires the user to set their password when they first sign-in.
    ///   * A user self-registers into a custom app or IDP and email verification is not required.
    ///   * An admin explicitly activates the user.
    Active = 3,
    /// Recovery status, when the user requests a password reset or an admin initiates one on their behalf.
    Recovery = 4,
    /// Password expired, status when the users' password has expired and the account requires an update to the password before a user is granted access.
    PasswordExpired = 5,
    /// Locked out status, is when the user exceeds the number of login attempts defined in the login policy.
    LockedOut = 6,
    /// Suspended status, when an admin explicitly suspends the user account.
    Suspended = 7,
    /// Deprovisioned status, is when an administrator explicitly deactivates or deprovisions/deletes the account.
    Deprovisioned = 8,
}
impl UserStatus {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            UserStatus::Unknown => "USER_STATUS_UNKNOWN",
            UserStatus::Staged => "USER_STATUS_STAGED",
            UserStatus::Provisioned => "USER_STATUS_PROVISIONED",
            UserStatus::Active => "USER_STATUS_ACTIVE",
            UserStatus::Recovery => "USER_STATUS_RECOVERY",
            UserStatus::PasswordExpired => "USER_STATUS_PASSWORD_EXPIRED",
            UserStatus::LockedOut => "USER_STATUS_LOCKED_OUT",
            UserStatus::Suspended => "USER_STATUS_SUSPENDED",
            UserStatus::Deprovisioned => "USER_STATUS_DEPROVISIONED",
        }
    }
}
include!("aserto.directory.schema.v2.serde.rs");
// @@protoc_insertion_point(module)