// @generated
impl serde::Serialize for AccountProperties {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.max_orgs != 0 {
            len += 1;
        }
        if self.getting_started.is_some() {
            len += 1;
        }
        if !self.default_tenant_id.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.schema.v2.AccountProperties", len)?;
        if self.max_orgs != 0 {
            struct_ser.serialize_field("maxOrgs", &self.max_orgs)?;
        }
        if let Some(v) = self.getting_started.as_ref() {
            struct_ser.serialize_field("gettingStarted", v)?;
        }
        if !self.default_tenant_id.is_empty() {
            struct_ser.serialize_field("defaultTenantId", &self.default_tenant_id)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for AccountProperties {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "max_orgs",
            "maxOrgs",
            "getting_started",
            "gettingStarted",
            "default_tenant_id",
            "defaultTenantId",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            MaxOrgs,
            GettingStarted,
            DefaultTenantId,
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
                            "maxOrgs" | "max_orgs" => Ok(GeneratedField::MaxOrgs),
                            "gettingStarted" | "getting_started" => Ok(GeneratedField::GettingStarted),
                            "defaultTenantId" | "default_tenant_id" => Ok(GeneratedField::DefaultTenantId),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = AccountProperties;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.schema.v2.AccountProperties")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<AccountProperties, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut max_orgs__ = None;
                let mut getting_started__ = None;
                let mut default_tenant_id__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::MaxOrgs => {
                            if max_orgs__.is_some() {
                                return Err(serde::de::Error::duplicate_field("maxOrgs"));
                            }
                            max_orgs__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::GettingStarted => {
                            if getting_started__.is_some() {
                                return Err(serde::de::Error::duplicate_field("gettingStarted"));
                            }
                            getting_started__ = map.next_value()?;
                        }
                        GeneratedField::DefaultTenantId => {
                            if default_tenant_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("defaultTenantId"));
                            }
                            default_tenant_id__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(AccountProperties {
                    max_orgs: max_orgs__.unwrap_or_default(),
                    getting_started: getting_started__,
                    default_tenant_id: default_tenant_id__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.schema.v2.AccountProperties", FIELDS, GeneratedVisitor)
    }
}
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
impl serde::Serialize for GuideState {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.show {
            len += 1;
        }
        if self.steps.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.schema.v2.GuideState", len)?;
        if self.show {
            struct_ser.serialize_field("show", &self.show)?;
        }
        if let Some(v) = self.steps.as_ref() {
            struct_ser.serialize_field("steps", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GuideState {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "show",
            "steps",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Show,
            Steps,
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
                            "show" => Ok(GeneratedField::Show),
                            "steps" => Ok(GeneratedField::Steps),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GuideState;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.schema.v2.GuideState")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GuideState, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut show__ = None;
                let mut steps__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Show => {
                            if show__.is_some() {
                                return Err(serde::de::Error::duplicate_field("show"));
                            }
                            show__ = Some(map.next_value()?);
                        }
                        GeneratedField::Steps => {
                            if steps__.is_some() {
                                return Err(serde::de::Error::duplicate_field("steps"));
                            }
                            steps__ = map.next_value()?;
                        }
                    }
                }
                Ok(GuideState {
                    show: show__.unwrap_or_default(),
                    steps: steps__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.schema.v2.GuideState", FIELDS, GeneratedVisitor)
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
impl serde::Serialize for TenantKind {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "TENANT_KIND_UNKNOWN",
            Self::Organization => "TENANT_KIND_ORGANIZATION",
            Self::Account => "TENANT_KIND_ACCOUNT",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for TenantKind {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "TENANT_KIND_UNKNOWN",
            "TENANT_KIND_ORGANIZATION",
            "TENANT_KIND_ACCOUNT",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TenantKind;

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
                    .and_then(TenantKind::from_i32)
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
                    .and_then(TenantKind::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "TENANT_KIND_UNKNOWN" => Ok(TenantKind::Unknown),
                    "TENANT_KIND_ORGANIZATION" => Ok(TenantKind::Organization),
                    "TENANT_KIND_ACCOUNT" => Ok(TenantKind::Account),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for TenantProperties {
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
        if self.directory_v2 {
            len += 1;
        }
        if self.directory_v2_only {
            len += 1;
        }
        if self.account.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.schema.v2.TenantProperties", len)?;
        if self.kind != 0 {
            let v = TenantKind::from_i32(self.kind)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.kind)))?;
            struct_ser.serialize_field("kind", &v)?;
        }
        if self.directory_v2 {
            struct_ser.serialize_field("directoryV2", &self.directory_v2)?;
        }
        if self.directory_v2_only {
            struct_ser.serialize_field("directoryV2Only", &self.directory_v2_only)?;
        }
        if let Some(v) = self.account.as_ref() {
            struct_ser.serialize_field("account", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for TenantProperties {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "kind",
            "directory_v2",
            "directoryV2",
            "directory_v2_only",
            "directoryV2Only",
            "account",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Kind,
            DirectoryV2,
            DirectoryV2Only,
            Account,
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
                            "directoryV2" | "directory_v2" => Ok(GeneratedField::DirectoryV2),
                            "directoryV2Only" | "directory_v2_only" => Ok(GeneratedField::DirectoryV2Only),
                            "account" => Ok(GeneratedField::Account),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = TenantProperties;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.schema.v2.TenantProperties")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<TenantProperties, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut kind__ = None;
                let mut directory_v2__ = None;
                let mut directory_v2_only__ = None;
                let mut account__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Kind => {
                            if kind__.is_some() {
                                return Err(serde::de::Error::duplicate_field("kind"));
                            }
                            kind__ = Some(map.next_value::<TenantKind>()? as i32);
                        }
                        GeneratedField::DirectoryV2 => {
                            if directory_v2__.is_some() {
                                return Err(serde::de::Error::duplicate_field("directoryV2"));
                            }
                            directory_v2__ = Some(map.next_value()?);
                        }
                        GeneratedField::DirectoryV2Only => {
                            if directory_v2_only__.is_some() {
                                return Err(serde::de::Error::duplicate_field("directoryV2Only"));
                            }
                            directory_v2_only__ = Some(map.next_value()?);
                        }
                        GeneratedField::Account => {
                            if account__.is_some() {
                                return Err(serde::de::Error::duplicate_field("account"));
                            }
                            account__ = map.next_value()?;
                        }
                    }
                }
                Ok(TenantProperties {
                    kind: kind__.unwrap_or_default(),
                    directory_v2: directory_v2__.unwrap_or_default(),
                    directory_v2_only: directory_v2_only__.unwrap_or_default(),
                    account: account__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.schema.v2.TenantProperties", FIELDS, GeneratedVisitor)
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
