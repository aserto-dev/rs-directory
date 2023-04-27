// @generated
impl serde::Serialize for GroupProperties {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.connection_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.schema.v2.GroupProperties", len)?;
        if !self.connection_id.is_empty() {
            struct_ser.serialize_field("connectionId", &self.connection_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GroupProperties {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "connection_id",
            "connectionId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ConnectionId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "connectionId" | "connection_id" => Ok(GeneratedField::ConnectionId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GroupProperties;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.schema.v2.GroupProperties")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GroupProperties, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut connection_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ConnectionId => {
                            if connection_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("connectionId"));
                            }
                            connection_id__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GroupProperties {
                    connection_id: connection_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.schema.v2.GroupProperties", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for IdentityKind {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "IDENTITY_KIND_UNKNOWN",
            Self::Pid => "IDENTITY_KIND_PID",
            Self::Email => "IDENTITY_KIND_EMAIL",
            Self::Username => "IDENTITY_KIND_USERNAME",
            Self::Dn => "IDENTITY_KIND_DN",
            Self::Phone => "IDENTITY_KIND_PHONE",
            Self::Empid => "IDENTITY_KIND_EMPID",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for IdentityKind {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "IDENTITY_KIND_UNKNOWN",
            "IDENTITY_KIND_PID",
            "IDENTITY_KIND_EMAIL",
            "IDENTITY_KIND_USERNAME",
            "IDENTITY_KIND_DN",
            "IDENTITY_KIND_PHONE",
            "IDENTITY_KIND_EMPID",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IdentityKind;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(IdentityKind::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(IdentityKind::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "IDENTITY_KIND_UNKNOWN" => Ok(IdentityKind::Unknown),
                    "IDENTITY_KIND_PID" => Ok(IdentityKind::Pid),
                    "IDENTITY_KIND_EMAIL" => Ok(IdentityKind::Email),
                    "IDENTITY_KIND_USERNAME" => Ok(IdentityKind::Username),
                    "IDENTITY_KIND_DN" => Ok(IdentityKind::Dn),
                    "IDENTITY_KIND_PHONE" => Ok(IdentityKind::Phone),
                    "IDENTITY_KIND_EMPID" => Ok(IdentityKind::Empid),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for IdentityProperties {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.kind != 0 {
            len += 1;
        }
        if !self.provider.is_empty() {
            len += 1;
        }
        if self.verified {
            len += 1;
        }
        if self.connection_id.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.schema.v2.IdentityProperties", len)?;
        if self.kind != 0 {
            let v = IdentityKind::from_i32(self.kind)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.kind)))?;
            struct_ser.serialize_field("kind", &v)?;
        }
        if !self.provider.is_empty() {
            struct_ser.serialize_field("provider", &self.provider)?;
        }
        if self.verified {
            struct_ser.serialize_field("verified", &self.verified)?;
        }
        if let Some(v) = self.connection_id.as_ref() {
            struct_ser.serialize_field("connectionId", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for IdentityProperties {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "kind",
            "provider",
            "verified",
            "connection_id",
            "connectionId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Kind,
            Provider,
            Verified,
            ConnectionId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "kind" => Ok(GeneratedField::Kind),
                            "provider" => Ok(GeneratedField::Provider),
                            "verified" => Ok(GeneratedField::Verified),
                            "connectionId" | "connection_id" => Ok(GeneratedField::ConnectionId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = IdentityProperties;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.schema.v2.IdentityProperties")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<IdentityProperties, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut kind__ = None;
                let mut provider__ = None;
                let mut verified__ = None;
                let mut connection_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Kind => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind__ = Some(map.next_value::<IdentityKind>()? as i32);
                        }
                        GeneratedField::Provider => {
                            if provider__.is_some() {
                                return Err(serde::de::Error::duplicate_field("provider"));
                            }
                            provider__ = Some(map.next_value()?);
                        }
                        GeneratedField::Verified => {
                            if verified__.is_some() {
                                return Err(serde::de::Error::duplicate_field("verified"));
                            }
                            verified__ = Some(map.next_value()?);
                        }
                        GeneratedField::ConnectionId => {
                            if connection_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("connectionId"));
                            }
                            connection_id__ = map.next_value()?;
                        }
                    }
                }
                Ok(IdentityProperties {
                    kind: kind__.unwrap_or_default(),
                    provider: provider__.unwrap_or_default(),
                    verified: verified__.unwrap_or_default(),
                    connection_id: connection_id__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.schema.v2.IdentityProperties", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserProperties {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.email.is_empty() {
            len += 1;
        }
        if !self.picture.is_empty() {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        if self.enabled {
            len += 1;
        }
        if !self.connection_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.schema.v2.UserProperties", len)?;
        if !self.email.is_empty() {
            struct_ser.serialize_field("email", &self.email)?;
        }
        if !self.picture.is_empty() {
            struct_ser.serialize_field("picture", &self.picture)?;
        }
        if self.status != 0 {
            let v = UserStatus::from_i32(self.status)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.status)))?;
            struct_ser.serialize_field("status", &v)?;
        }
        if self.enabled {
            struct_ser.serialize_field("enabled", &self.enabled)?;
        }
        if !self.connection_id.is_empty() {
            struct_ser.serialize_field("connectionId", &self.connection_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for UserProperties {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "email",
            "picture",
            "status",
            "enabled",
            "connection_id",
            "connectionId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Email,
            Picture,
            Status,
            Enabled,
            ConnectionId,
        }
        impl<'de> serde::Deserialize<'de> for GeneratedField {
            fn deserialize<D>(deserializer: D) -> std::result::Result<GeneratedField, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                struct GeneratedVisitor;

                impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
                    type Value = GeneratedField;

                    fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                        write!(formatter, "expected one of: {:?}", &FIELDS)
                    }

                    #[allow(unused_variables)]
                    fn visit_str<E>(self, value: &str) -> std::result::Result<GeneratedField, E>
                    where
                        E: serde::de::Error,
                    {
                        match value {
                            "email" => Ok(GeneratedField::Email),
                            "picture" => Ok(GeneratedField::Picture),
                            "status" => Ok(GeneratedField::Status),
                            "enabled" => Ok(GeneratedField::Enabled),
                            "connectionId" | "connection_id" => Ok(GeneratedField::ConnectionId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UserProperties;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.schema.v2.UserProperties")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<UserProperties, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut email__ = None;
                let mut picture__ = None;
                let mut status__ = None;
                let mut enabled__ = None;
                let mut connection_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Email => {
                            if email__.is_some() {
                                return Err(serde::de::Error::duplicate_field("email"));
                            }
                            email__ = Some(map.next_value()?);
                        }
                        GeneratedField::Picture => {
                            if picture__.is_some() {
                                return Err(serde::de::Error::duplicate_field("picture"));
                            }
                            picture__ = Some(map.next_value()?);
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = Some(map.next_value::<UserStatus>()? as i32);
                        }
                        GeneratedField::Enabled => {
                            if enabled__.is_some() {
                                return Err(serde::de::Error::duplicate_field("enabled"));
                            }
                            enabled__ = Some(map.next_value()?);
                        }
                        GeneratedField::ConnectionId => {
                            if connection_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("connectionId"));
                            }
                            connection_id__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(UserProperties {
                    email: email__.unwrap_or_default(),
                    picture: picture__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                    enabled: enabled__.unwrap_or_default(),
                    connection_id: connection_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.schema.v2.UserProperties", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for UserStatus {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "USER_STATUS_UNKNOWN",
            Self::Staged => "USER_STATUS_STAGED",
            Self::Provisioned => "USER_STATUS_PROVISIONED",
            Self::Active => "USER_STATUS_ACTIVE",
            Self::Recovery => "USER_STATUS_RECOVERY",
            Self::PasswordExpired => "USER_STATUS_PASSWORD_EXPIRED",
            Self::LockedOut => "USER_STATUS_LOCKED_OUT",
            Self::Suspended => "USER_STATUS_SUSPENDED",
            Self::Deprovisioned => "USER_STATUS_DEPROVISIONED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for UserStatus {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "USER_STATUS_UNKNOWN",
            "USER_STATUS_STAGED",
            "USER_STATUS_PROVISIONED",
            "USER_STATUS_ACTIVE",
            "USER_STATUS_RECOVERY",
            "USER_STATUS_PASSWORD_EXPIRED",
            "USER_STATUS_LOCKED_OUT",
            "USER_STATUS_SUSPENDED",
            "USER_STATUS_DEPROVISIONED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = UserStatus;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(UserStatus::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                use std::convert::TryFrom;
                i32::try_from(v)
                    .ok()
                    .and_then(UserStatus::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "USER_STATUS_UNKNOWN" => Ok(UserStatus::Unknown),
                    "USER_STATUS_STAGED" => Ok(UserStatus::Staged),
                    "USER_STATUS_PROVISIONED" => Ok(UserStatus::Provisioned),
                    "USER_STATUS_ACTIVE" => Ok(UserStatus::Active),
                    "USER_STATUS_RECOVERY" => Ok(UserStatus::Recovery),
                    "USER_STATUS_PASSWORD_EXPIRED" => Ok(UserStatus::PasswordExpired),
                    "USER_STATUS_LOCKED_OUT" => Ok(UserStatus::LockedOut),
                    "USER_STATUS_SUSPENDED" => Ok(UserStatus::Suspended),
                    "USER_STATUS_DEPROVISIONED" => Ok(UserStatus::Deprovisioned),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
