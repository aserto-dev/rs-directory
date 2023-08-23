// @generated
impl serde::Serialize for Flag {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "FLAG_UNKNOWN",
            Self::Hidden => "FLAG_HIDDEN",
            Self::Readonly => "FLAG_READONLY",
            Self::System => "FLAG_SYSTEM",
            Self::Shadow => "FLAG_SHADOW",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for Flag {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "FLAG_UNKNOWN",
            "FLAG_HIDDEN",
            "FLAG_READONLY",
            "FLAG_SYSTEM",
            "FLAG_SHADOW",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Flag;

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
                    .and_then(Flag::from_i32)
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
                    .and_then(Flag::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "FLAG_UNKNOWN" => Ok(Flag::Unknown),
                    "FLAG_HIDDEN" => Ok(Flag::Hidden),
                    "FLAG_READONLY" => Ok(Flag::Readonly),
                    "FLAG_SYSTEM" => Ok(Flag::System),
                    "FLAG_SHADOW" => Ok(Flag::Shadow),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for Object {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.key.is_empty() {
            len += 1;
        }
        if !self.r#type.is_empty() {
            len += 1;
        }
        if !self.display_name.is_empty() {
            len += 1;
        }
        if self.properties.is_some() {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        if self.updated_at.is_some() {
            len += 1;
        }
        if !self.hash.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.common.v2.Object", len)?;
        if !self.key.is_empty() {
            struct_ser.serialize_field("key", &self.key)?;
        }
        if !self.r#type.is_empty() {
            struct_ser.serialize_field("type", &self.r#type)?;
        }
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
        }
        if let Some(v) = self.properties.as_ref() {
            struct_ser.serialize_field("properties", v)?;
        }
        if let Some(v) = self.created_at.as_ref() {
            struct_ser.serialize_field("createdAt", v)?;
        }
        if let Some(v) = self.updated_at.as_ref() {
            struct_ser.serialize_field("updatedAt", v)?;
        }
        if !self.hash.is_empty() {
            struct_ser.serialize_field("hash", &self.hash)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Object {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "key",
            "type",
            "display_name",
            "displayName",
            "properties",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "hash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Key,
            Type,
            DisplayName,
            Properties,
            CreatedAt,
            UpdatedAt,
            Hash,
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
                            "key" => Ok(GeneratedField::Key),
                            "type" => Ok(GeneratedField::Type),
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
                            "properties" => Ok(GeneratedField::Properties),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "hash" => Ok(GeneratedField::Hash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Object;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.common.v2.Object")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Object, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut key__ = None;
                let mut r#type__ = None;
                let mut display_name__ = None;
                let mut properties__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut hash__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = Some(map.next_value()?);
                        }
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = Some(map.next_value()?);
                        }
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Properties => {
                            if properties__.is_some() {
                                return Err(serde::de::Error::duplicate_field("properties"));
                            }
                            properties__ = map.next_value()?;
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = map.next_value()?;
                        }
                        GeneratedField::UpdatedAt => {
                            if updated_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedAt"));
                            }
                            updated_at__ = map.next_value()?;
                        }
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Object {
                    key: key__.unwrap_or_default(),
                    r#type: r#type__.unwrap_or_default(),
                    display_name: display_name__.unwrap_or_default(),
                    properties: properties__,
                    created_at: created_at__,
                    updated_at: updated_at__,
                    hash: hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.common.v2.Object", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ObjectDependency {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.object_type.is_empty() {
            len += 1;
        }
        if !self.object_key.is_empty() {
            len += 1;
        }
        if !self.relation.is_empty() {
            len += 1;
        }
        if !self.subject_type.is_empty() {
            len += 1;
        }
        if !self.subject_key.is_empty() {
            len += 1;
        }
        if self.depth != 0 {
            len += 1;
        }
        if self.is_cycle {
            len += 1;
        }
        if !self.path.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.common.v2.ObjectDependency", len)?;
        if !self.object_type.is_empty() {
            struct_ser.serialize_field("objectType", &self.object_type)?;
        }
        if !self.object_key.is_empty() {
            struct_ser.serialize_field("objectKey", &self.object_key)?;
        }
        if !self.relation.is_empty() {
            struct_ser.serialize_field("relation", &self.relation)?;
        }
        if !self.subject_type.is_empty() {
            struct_ser.serialize_field("subjectType", &self.subject_type)?;
        }
        if !self.subject_key.is_empty() {
            struct_ser.serialize_field("subjectKey", &self.subject_key)?;
        }
        if self.depth != 0 {
            struct_ser.serialize_field("depth", &self.depth)?;
        }
        if self.is_cycle {
            struct_ser.serialize_field("isCycle", &self.is_cycle)?;
        }
        if !self.path.is_empty() {
            struct_ser.serialize_field("path", &self.path)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ObjectDependency {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "object_type",
            "objectType",
            "object_key",
            "objectKey",
            "relation",
            "subject_type",
            "subjectType",
            "subject_key",
            "subjectKey",
            "depth",
            "is_cycle",
            "isCycle",
            "path",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ObjectType,
            ObjectKey,
            Relation,
            SubjectType,
            SubjectKey,
            Depth,
            IsCycle,
            Path,
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
                            "objectType" | "object_type" => Ok(GeneratedField::ObjectType),
                            "objectKey" | "object_key" => Ok(GeneratedField::ObjectKey),
                            "relation" => Ok(GeneratedField::Relation),
                            "subjectType" | "subject_type" => Ok(GeneratedField::SubjectType),
                            "subjectKey" | "subject_key" => Ok(GeneratedField::SubjectKey),
                            "depth" => Ok(GeneratedField::Depth),
                            "isCycle" | "is_cycle" => Ok(GeneratedField::IsCycle),
                            "path" => Ok(GeneratedField::Path),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ObjectDependency;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.common.v2.ObjectDependency")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ObjectDependency, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut object_type__ = None;
                let mut object_key__ = None;
                let mut relation__ = None;
                let mut subject_type__ = None;
                let mut subject_key__ = None;
                let mut depth__ = None;
                let mut is_cycle__ = None;
                let mut path__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ObjectType => {
                            if object_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objectType"));
                            }
                            object_type__ = Some(map.next_value()?);
                        }
                        GeneratedField::ObjectKey => {
                            if object_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objectKey"));
                            }
                            object_key__ = Some(map.next_value()?);
                        }
                        GeneratedField::Relation => {
                            if relation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relation"));
                            }
                            relation__ = Some(map.next_value()?);
                        }
                        GeneratedField::SubjectType => {
                            if subject_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subjectType"));
                            }
                            subject_type__ = Some(map.next_value()?);
                        }
                        GeneratedField::SubjectKey => {
                            if subject_key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subjectKey"));
                            }
                            subject_key__ = Some(map.next_value()?);
                        }
                        GeneratedField::Depth => {
                            if depth__.is_some() {
                                return Err(serde::de::Error::duplicate_field("depth"));
                            }
                            depth__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::IsCycle => {
                            if is_cycle__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isCycle"));
                            }
                            is_cycle__ = Some(map.next_value()?);
                        }
                        GeneratedField::Path => {
                            if path__.is_some() {
                                return Err(serde::de::Error::duplicate_field("path"));
                            }
                            path__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ObjectDependency {
                    object_type: object_type__.unwrap_or_default(),
                    object_key: object_key__.unwrap_or_default(),
                    relation: relation__.unwrap_or_default(),
                    subject_type: subject_type__.unwrap_or_default(),
                    subject_key: subject_key__.unwrap_or_default(),
                    depth: depth__.unwrap_or_default(),
                    is_cycle: is_cycle__.unwrap_or_default(),
                    path: path__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.common.v2.ObjectDependency", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ObjectIdentifier {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.r#type.is_some() {
            len += 1;
        }
        if self.key.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.common.v2.ObjectIdentifier", len)?;
        if let Some(v) = self.r#type.as_ref() {
            struct_ser.serialize_field("type", v)?;
        }
        if let Some(v) = self.key.as_ref() {
            struct_ser.serialize_field("key", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ObjectIdentifier {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "type",
            "key",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Type,
            Key,
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
                            "type" => Ok(GeneratedField::Type),
                            "key" => Ok(GeneratedField::Key),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ObjectIdentifier;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.common.v2.ObjectIdentifier")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ObjectIdentifier, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut r#type__ = None;
                let mut key__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Type => {
                            if r#type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("type"));
                            }
                            r#type__ = map.next_value()?;
                        }
                        GeneratedField::Key => {
                            if key__.is_some() {
                                return Err(serde::de::Error::duplicate_field("key"));
                            }
                            key__ = map.next_value()?;
                        }
                    }
                }
                Ok(ObjectIdentifier {
                    r#type: r#type__,
                    key: key__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.common.v2.ObjectIdentifier", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ObjectType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.display_name.is_empty() {
            len += 1;
        }
        if self.is_subject {
            len += 1;
        }
        if self.ordinal != 0 {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        if self.schema.is_some() {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        if self.updated_at.is_some() {
            len += 1;
        }
        if !self.hash.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.common.v2.ObjectType", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
        }
        if self.is_subject {
            struct_ser.serialize_field("isSubject", &self.is_subject)?;
        }
        if self.ordinal != 0 {
            struct_ser.serialize_field("ordinal", &self.ordinal)?;
        }
        if self.status != 0 {
            struct_ser.serialize_field("status", &self.status)?;
        }
        if let Some(v) = self.schema.as_ref() {
            struct_ser.serialize_field("schema", v)?;
        }
        if let Some(v) = self.created_at.as_ref() {
            struct_ser.serialize_field("createdAt", v)?;
        }
        if let Some(v) = self.updated_at.as_ref() {
            struct_ser.serialize_field("updatedAt", v)?;
        }
        if !self.hash.is_empty() {
            struct_ser.serialize_field("hash", &self.hash)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ObjectType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "display_name",
            "displayName",
            "is_subject",
            "isSubject",
            "ordinal",
            "status",
            "schema",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "hash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            DisplayName,
            IsSubject,
            Ordinal,
            Status,
            Schema,
            CreatedAt,
            UpdatedAt,
            Hash,
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
                            "name" => Ok(GeneratedField::Name),
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
                            "isSubject" | "is_subject" => Ok(GeneratedField::IsSubject),
                            "ordinal" => Ok(GeneratedField::Ordinal),
                            "status" => Ok(GeneratedField::Status),
                            "schema" => Ok(GeneratedField::Schema),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "hash" => Ok(GeneratedField::Hash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ObjectType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.common.v2.ObjectType")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ObjectType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut display_name__ = None;
                let mut is_subject__ = None;
                let mut ordinal__ = None;
                let mut status__ = None;
                let mut schema__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut hash__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::IsSubject => {
                            if is_subject__.is_some() {
                                return Err(serde::de::Error::duplicate_field("isSubject"));
                            }
                            is_subject__ = Some(map.next_value()?);
                        }
                        GeneratedField::Ordinal => {
                            if ordinal__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ordinal"));
                            }
                            ordinal__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Schema => {
                            if schema__.is_some() {
                                return Err(serde::de::Error::duplicate_field("schema"));
                            }
                            schema__ = map.next_value()?;
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = map.next_value()?;
                        }
                        GeneratedField::UpdatedAt => {
                            if updated_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedAt"));
                            }
                            updated_at__ = map.next_value()?;
                        }
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(ObjectType {
                    name: name__.unwrap_or_default(),
                    display_name: display_name__.unwrap_or_default(),
                    is_subject: is_subject__.unwrap_or_default(),
                    ordinal: ordinal__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                    schema: schema__,
                    created_at: created_at__,
                    updated_at: updated_at__,
                    hash: hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.common.v2.ObjectType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ObjectTypeIdentifier {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.name.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.common.v2.ObjectTypeIdentifier", len)?;
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ObjectTypeIdentifier {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
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
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ObjectTypeIdentifier;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.common.v2.ObjectTypeIdentifier")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ObjectTypeIdentifier, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map.next_value()?;
                        }
                    }
                }
                Ok(ObjectTypeIdentifier {
                    name: name__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.common.v2.ObjectTypeIdentifier", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PaginationRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.size != 0 {
            len += 1;
        }
        if !self.token.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.common.v2.PaginationRequest", len)?;
        if self.size != 0 {
            struct_ser.serialize_field("size", &self.size)?;
        }
        if !self.token.is_empty() {
            struct_ser.serialize_field("token", &self.token)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PaginationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "size",
            "token",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Size,
            Token,
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
                            "size" => Ok(GeneratedField::Size),
                            "token" => Ok(GeneratedField::Token),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PaginationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.common.v2.PaginationRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PaginationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut size__ = None;
                let mut token__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Size => {
                            if size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("size"));
                            }
                            size__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Token => {
                            if token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("token"));
                            }
                            token__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(PaginationRequest {
                    size: size__.unwrap_or_default(),
                    token: token__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.common.v2.PaginationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PaginationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.next_token.is_empty() {
            len += 1;
        }
        if self.result_size != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.common.v2.PaginationResponse", len)?;
        if !self.next_token.is_empty() {
            struct_ser.serialize_field("nextToken", &self.next_token)?;
        }
        if self.result_size != 0 {
            struct_ser.serialize_field("resultSize", &self.result_size)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PaginationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "next_token",
            "nextToken",
            "result_size",
            "resultSize",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            NextToken,
            ResultSize,
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
                            "nextToken" | "next_token" => Ok(GeneratedField::NextToken),
                            "resultSize" | "result_size" => Ok(GeneratedField::ResultSize),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PaginationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.common.v2.PaginationResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PaginationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut next_token__ = None;
                let mut result_size__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::NextToken => {
                            if next_token__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nextToken"));
                            }
                            next_token__ = Some(map.next_value()?);
                        }
                        GeneratedField::ResultSize => {
                            if result_size__.is_some() {
                                return Err(serde::de::Error::duplicate_field("resultSize"));
                            }
                            result_size__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(PaginationResponse {
                    next_token: next_token__.unwrap_or_default(),
                    result_size: result_size__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.common.v2.PaginationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Permission {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.display_name.is_empty() {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        if self.updated_at.is_some() {
            len += 1;
        }
        if !self.hash.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.common.v2.Permission", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
        }
        if let Some(v) = self.created_at.as_ref() {
            struct_ser.serialize_field("createdAt", v)?;
        }
        if let Some(v) = self.updated_at.as_ref() {
            struct_ser.serialize_field("updatedAt", v)?;
        }
        if !self.hash.is_empty() {
            struct_ser.serialize_field("hash", &self.hash)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Permission {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "display_name",
            "displayName",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "hash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            DisplayName,
            CreatedAt,
            UpdatedAt,
            Hash,
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
                            "name" => Ok(GeneratedField::Name),
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "hash" => Ok(GeneratedField::Hash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Permission;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.common.v2.Permission")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Permission, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut display_name__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut hash__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = map.next_value()?;
                        }
                        GeneratedField::UpdatedAt => {
                            if updated_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedAt"));
                            }
                            updated_at__ = map.next_value()?;
                        }
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Permission {
                    name: name__.unwrap_or_default(),
                    display_name: display_name__.unwrap_or_default(),
                    created_at: created_at__,
                    updated_at: updated_at__,
                    hash: hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.common.v2.Permission", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for PermissionIdentifier {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.name.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.common.v2.PermissionIdentifier", len)?;
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for PermissionIdentifier {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
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
                            "name" => Ok(GeneratedField::Name),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = PermissionIdentifier;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.common.v2.PermissionIdentifier")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<PermissionIdentifier, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map.next_value()?;
                        }
                    }
                }
                Ok(PermissionIdentifier {
                    name: name__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.common.v2.PermissionIdentifier", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Relation {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.subject.is_some() {
            len += 1;
        }
        if !self.relation.is_empty() {
            len += 1;
        }
        if self.object.is_some() {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        if self.updated_at.is_some() {
            len += 1;
        }
        if !self.hash.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.common.v2.Relation", len)?;
        if let Some(v) = self.subject.as_ref() {
            struct_ser.serialize_field("subject", v)?;
        }
        if !self.relation.is_empty() {
            struct_ser.serialize_field("relation", &self.relation)?;
        }
        if let Some(v) = self.object.as_ref() {
            struct_ser.serialize_field("object", v)?;
        }
        if let Some(v) = self.created_at.as_ref() {
            struct_ser.serialize_field("createdAt", v)?;
        }
        if let Some(v) = self.updated_at.as_ref() {
            struct_ser.serialize_field("updatedAt", v)?;
        }
        if !self.hash.is_empty() {
            struct_ser.serialize_field("hash", &self.hash)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for Relation {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subject",
            "relation",
            "object",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "hash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Subject,
            Relation,
            Object,
            CreatedAt,
            UpdatedAt,
            Hash,
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
                            "subject" => Ok(GeneratedField::Subject),
                            "relation" => Ok(GeneratedField::Relation),
                            "object" => Ok(GeneratedField::Object),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "hash" => Ok(GeneratedField::Hash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Relation;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.common.v2.Relation")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<Relation, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut subject__ = None;
                let mut relation__ = None;
                let mut object__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut hash__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Subject => {
                            if subject__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject"));
                            }
                            subject__ = map.next_value()?;
                        }
                        GeneratedField::Relation => {
                            if relation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relation"));
                            }
                            relation__ = Some(map.next_value()?);
                        }
                        GeneratedField::Object => {
                            if object__.is_some() {
                                return Err(serde::de::Error::duplicate_field("object"));
                            }
                            object__ = map.next_value()?;
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = map.next_value()?;
                        }
                        GeneratedField::UpdatedAt => {
                            if updated_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedAt"));
                            }
                            updated_at__ = map.next_value()?;
                        }
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(Relation {
                    subject: subject__,
                    relation: relation__.unwrap_or_default(),
                    object: object__,
                    created_at: created_at__,
                    updated_at: updated_at__,
                    hash: hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.common.v2.Relation", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RelationIdentifier {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.subject.is_some() {
            len += 1;
        }
        if self.relation.is_some() {
            len += 1;
        }
        if self.object.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.common.v2.RelationIdentifier", len)?;
        if let Some(v) = self.subject.as_ref() {
            struct_ser.serialize_field("subject", v)?;
        }
        if let Some(v) = self.relation.as_ref() {
            struct_ser.serialize_field("relation", v)?;
        }
        if let Some(v) = self.object.as_ref() {
            struct_ser.serialize_field("object", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RelationIdentifier {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subject",
            "relation",
            "object",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Subject,
            Relation,
            Object,
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
                            "subject" => Ok(GeneratedField::Subject),
                            "relation" => Ok(GeneratedField::Relation),
                            "object" => Ok(GeneratedField::Object),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RelationIdentifier;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.common.v2.RelationIdentifier")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RelationIdentifier, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut subject__ = None;
                let mut relation__ = None;
                let mut object__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Subject => {
                            if subject__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject"));
                            }
                            subject__ = map.next_value()?;
                        }
                        GeneratedField::Relation => {
                            if relation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relation"));
                            }
                            relation__ = map.next_value()?;
                        }
                        GeneratedField::Object => {
                            if object__.is_some() {
                                return Err(serde::de::Error::duplicate_field("object"));
                            }
                            object__ = map.next_value()?;
                        }
                    }
                }
                Ok(RelationIdentifier {
                    subject: subject__,
                    relation: relation__,
                    object: object__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.common.v2.RelationIdentifier", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RelationType {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.name.is_empty() {
            len += 1;
        }
        if !self.object_type.is_empty() {
            len += 1;
        }
        if !self.display_name.is_empty() {
            len += 1;
        }
        if self.ordinal != 0 {
            len += 1;
        }
        if self.status != 0 {
            len += 1;
        }
        if !self.unions.is_empty() {
            len += 1;
        }
        if !self.permissions.is_empty() {
            len += 1;
        }
        if self.created_at.is_some() {
            len += 1;
        }
        if self.updated_at.is_some() {
            len += 1;
        }
        if !self.hash.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.common.v2.RelationType", len)?;
        if !self.name.is_empty() {
            struct_ser.serialize_field("name", &self.name)?;
        }
        if !self.object_type.is_empty() {
            struct_ser.serialize_field("objectType", &self.object_type)?;
        }
        if !self.display_name.is_empty() {
            struct_ser.serialize_field("displayName", &self.display_name)?;
        }
        if self.ordinal != 0 {
            struct_ser.serialize_field("ordinal", &self.ordinal)?;
        }
        if self.status != 0 {
            struct_ser.serialize_field("status", &self.status)?;
        }
        if !self.unions.is_empty() {
            struct_ser.serialize_field("unions", &self.unions)?;
        }
        if !self.permissions.is_empty() {
            struct_ser.serialize_field("permissions", &self.permissions)?;
        }
        if let Some(v) = self.created_at.as_ref() {
            struct_ser.serialize_field("createdAt", v)?;
        }
        if let Some(v) = self.updated_at.as_ref() {
            struct_ser.serialize_field("updatedAt", v)?;
        }
        if !self.hash.is_empty() {
            struct_ser.serialize_field("hash", &self.hash)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RelationType {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "object_type",
            "objectType",
            "display_name",
            "displayName",
            "ordinal",
            "status",
            "unions",
            "permissions",
            "created_at",
            "createdAt",
            "updated_at",
            "updatedAt",
            "hash",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            ObjectType,
            DisplayName,
            Ordinal,
            Status,
            Unions,
            Permissions,
            CreatedAt,
            UpdatedAt,
            Hash,
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
                            "name" => Ok(GeneratedField::Name),
                            "objectType" | "object_type" => Ok(GeneratedField::ObjectType),
                            "displayName" | "display_name" => Ok(GeneratedField::DisplayName),
                            "ordinal" => Ok(GeneratedField::Ordinal),
                            "status" => Ok(GeneratedField::Status),
                            "unions" => Ok(GeneratedField::Unions),
                            "permissions" => Ok(GeneratedField::Permissions),
                            "createdAt" | "created_at" => Ok(GeneratedField::CreatedAt),
                            "updatedAt" | "updated_at" => Ok(GeneratedField::UpdatedAt),
                            "hash" => Ok(GeneratedField::Hash),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RelationType;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.common.v2.RelationType")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RelationType, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut object_type__ = None;
                let mut display_name__ = None;
                let mut ordinal__ = None;
                let mut status__ = None;
                let mut unions__ = None;
                let mut permissions__ = None;
                let mut created_at__ = None;
                let mut updated_at__ = None;
                let mut hash__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = Some(map.next_value()?);
                        }
                        GeneratedField::ObjectType => {
                            if object_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objectType"));
                            }
                            object_type__ = Some(map.next_value()?);
                        }
                        GeneratedField::DisplayName => {
                            if display_name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("displayName"));
                            }
                            display_name__ = Some(map.next_value()?);
                        }
                        GeneratedField::Ordinal => {
                            if ordinal__.is_some() {
                                return Err(serde::de::Error::duplicate_field("ordinal"));
                            }
                            ordinal__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Status => {
                            if status__.is_some() {
                                return Err(serde::de::Error::duplicate_field("status"));
                            }
                            status__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Unions => {
                            if unions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("unions"));
                            }
                            unions__ = Some(map.next_value()?);
                        }
                        GeneratedField::Permissions => {
                            if permissions__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permissions"));
                            }
                            permissions__ = Some(map.next_value()?);
                        }
                        GeneratedField::CreatedAt => {
                            if created_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("createdAt"));
                            }
                            created_at__ = map.next_value()?;
                        }
                        GeneratedField::UpdatedAt => {
                            if updated_at__.is_some() {
                                return Err(serde::de::Error::duplicate_field("updatedAt"));
                            }
                            updated_at__ = map.next_value()?;
                        }
                        GeneratedField::Hash => {
                            if hash__.is_some() {
                                return Err(serde::de::Error::duplicate_field("hash"));
                            }
                            hash__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(RelationType {
                    name: name__.unwrap_or_default(),
                    object_type: object_type__.unwrap_or_default(),
                    display_name: display_name__.unwrap_or_default(),
                    ordinal: ordinal__.unwrap_or_default(),
                    status: status__.unwrap_or_default(),
                    unions: unions__.unwrap_or_default(),
                    permissions: permissions__.unwrap_or_default(),
                    created_at: created_at__,
                    updated_at: updated_at__,
                    hash: hash__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.common.v2.RelationType", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RelationTypeIdentifier {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.name.is_some() {
            len += 1;
        }
        if self.object_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.common.v2.RelationTypeIdentifier", len)?;
        if let Some(v) = self.name.as_ref() {
            struct_ser.serialize_field("name", v)?;
        }
        if let Some(v) = self.object_type.as_ref() {
            struct_ser.serialize_field("objectType", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RelationTypeIdentifier {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "name",
            "object_type",
            "objectType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Name,
            ObjectType,
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
                            "name" => Ok(GeneratedField::Name),
                            "objectType" | "object_type" => Ok(GeneratedField::ObjectType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RelationTypeIdentifier;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.common.v2.RelationTypeIdentifier")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<RelationTypeIdentifier, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut name__ = None;
                let mut object_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Name => {
                            if name__.is_some() {
                                return Err(serde::de::Error::duplicate_field("name"));
                            }
                            name__ = map.next_value()?;
                        }
                        GeneratedField::ObjectType => {
                            if object_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objectType"));
                            }
                            object_type__ = map.next_value()?;
                        }
                    }
                }
                Ok(RelationTypeIdentifier {
                    name: name__,
                    object_type: object_type__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.common.v2.RelationTypeIdentifier", FIELDS, GeneratedVisitor)
    }
}
