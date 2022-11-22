// @generated
impl serde::Serialize for ExportRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.options != 0 {
            len += 1;
        }
        if self.start_from.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.exporter.v2.ExportRequest", len)?;
        if self.options != 0 {
            struct_ser.serialize_field("options", &self.options)?;
        }
        if let Some(v) = self.start_from.as_ref() {
            struct_ser.serialize_field("startFrom", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExportRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "options",
            "start_from",
            "startFrom",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Options,
            StartFrom,
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
                            "options" => Ok(GeneratedField::Options),
                            "startFrom" | "start_from" => Ok(GeneratedField::StartFrom),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExportRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.exporter.v2.ExportRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExportRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut options__ = None;
                let mut start_from__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Options => {
                            if options__.is_some() {
                                return Err(serde::de::Error::duplicate_field("options"));
                            }
                            options__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::StartFrom => {
                            if start_from__.is_some() {
                                return Err(serde::de::Error::duplicate_field("startFrom"));
                            }
                            start_from__ = map.next_value()?;
                        }
                    }
                }
                Ok(ExportRequest {
                    options: options__.unwrap_or_default(),
                    start_from: start_from__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.exporter.v2.ExportRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ExportResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.msg.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.exporter.v2.ExportResponse", len)?;
        if let Some(v) = self.msg.as_ref() {
            match v {
                export_response::Msg::Object(v) => {
                    struct_ser.serialize_field("object", v)?;
                }
                export_response::Msg::ObjectType(v) => {
                    struct_ser.serialize_field("objectType", v)?;
                }
                export_response::Msg::Relation(v) => {
                    struct_ser.serialize_field("relation", v)?;
                }
                export_response::Msg::RelationType(v) => {
                    struct_ser.serialize_field("relationType", v)?;
                }
                export_response::Msg::Permission(v) => {
                    struct_ser.serialize_field("permission", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ExportResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "object",
            "object_type",
            "objectType",
            "relation",
            "relation_type",
            "relationType",
            "permission",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Object,
            ObjectType,
            Relation,
            RelationType,
            Permission,
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
                            "object" => Ok(GeneratedField::Object),
                            "objectType" | "object_type" => Ok(GeneratedField::ObjectType),
                            "relation" => Ok(GeneratedField::Relation),
                            "relationType" | "relation_type" => Ok(GeneratedField::RelationType),
                            "permission" => Ok(GeneratedField::Permission),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ExportResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.exporter.v2.ExportResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ExportResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut msg__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Object => {
                            if msg__.is_some() {
                                return Err(serde::de::Error::duplicate_field("object"));
                            }
                            msg__ = map.next_value::<::std::option::Option<_>>()?.map(export_response::Msg::Object)
;
                        }
                        GeneratedField::ObjectType => {
                            if msg__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objectType"));
                            }
                            msg__ = map.next_value::<::std::option::Option<_>>()?.map(export_response::Msg::ObjectType)
;
                        }
                        GeneratedField::Relation => {
                            if msg__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relation"));
                            }
                            msg__ = map.next_value::<::std::option::Option<_>>()?.map(export_response::Msg::Relation)
;
                        }
                        GeneratedField::RelationType => {
                            if msg__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relationType"));
                            }
                            msg__ = map.next_value::<::std::option::Option<_>>()?.map(export_response::Msg::RelationType)
;
                        }
                        GeneratedField::Permission => {
                            if msg__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permission"));
                            }
                            msg__ = map.next_value::<::std::option::Option<_>>()?.map(export_response::Msg::Permission)
;
                        }
                    }
                }
                Ok(ExportResponse {
                    msg: msg__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.exporter.v2.ExportResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Option {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "OPTION_UNKNOWN",
            Self::MetadataObjectTypes => "OPTION_METADATA_OBJECT_TYPES",
            Self::MetadataRelationTypes => "OPTION_METADATA_RELATION_TYPES",
            Self::MetadataPermissions => "OPTION_METADATA_PERMISSIONS",
            Self::Metadata => "OPTION_METADATA",
            Self::DataObjects => "OPTION_DATA_OBJECTS",
            Self::DataRelations => "OPTION_DATA_RELATIONS",
            Self::DataRelationsWithKeys => "OPTION_DATA_RELATIONS_WITH_KEYS",
            Self::Data => "OPTION_DATA",
            Self::DataWithKeys => "OPTION_DATA_WITH_KEYS",
            Self::All => "OPTION_ALL",
            Self::AllWithKeys => "OPTION_ALL_WITH_KEYS",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for Option {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "OPTION_UNKNOWN",
            "OPTION_METADATA_OBJECT_TYPES",
            "OPTION_METADATA_RELATION_TYPES",
            "OPTION_METADATA_PERMISSIONS",
            "OPTION_METADATA",
            "OPTION_DATA_OBJECTS",
            "OPTION_DATA_RELATIONS",
            "OPTION_DATA_RELATIONS_WITH_KEYS",
            "OPTION_DATA",
            "OPTION_DATA_WITH_KEYS",
            "OPTION_ALL",
            "OPTION_ALL_WITH_KEYS",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Option;

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
                    .and_then(Option::from_i32)
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
                    .and_then(Option::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "OPTION_UNKNOWN" => Ok(Option::Unknown),
                    "OPTION_METADATA_OBJECT_TYPES" => Ok(Option::MetadataObjectTypes),
                    "OPTION_METADATA_RELATION_TYPES" => Ok(Option::MetadataRelationTypes),
                    "OPTION_METADATA_PERMISSIONS" => Ok(Option::MetadataPermissions),
                    "OPTION_METADATA" => Ok(Option::Metadata),
                    "OPTION_DATA_OBJECTS" => Ok(Option::DataObjects),
                    "OPTION_DATA_RELATIONS" => Ok(Option::DataRelations),
                    "OPTION_DATA_RELATIONS_WITH_KEYS" => Ok(Option::DataRelationsWithKeys),
                    "OPTION_DATA" => Ok(Option::Data),
                    "OPTION_DATA_WITH_KEYS" => Ok(Option::DataWithKeys),
                    "OPTION_ALL" => Ok(Option::All),
                    "OPTION_ALL_WITH_KEYS" => Ok(Option::AllWithKeys),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
