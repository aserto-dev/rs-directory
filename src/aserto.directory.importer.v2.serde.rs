// @generated
impl serde::Serialize for ImportCounter {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.recv != 0 {
            len += 1;
        }
        if self.set != 0 {
            len += 1;
        }
        if self.delete != 0 {
            len += 1;
        }
        if self.error != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.importer.v2.ImportCounter", len)?;
        if self.recv != 0 {
            struct_ser.serialize_field("recv", ToString::to_string(&self.recv).as_str())?;
        }
        if self.set != 0 {
            struct_ser.serialize_field("set", ToString::to_string(&self.set).as_str())?;
        }
        if self.delete != 0 {
            struct_ser.serialize_field("delete", ToString::to_string(&self.delete).as_str())?;
        }
        if self.error != 0 {
            struct_ser.serialize_field("error", ToString::to_string(&self.error).as_str())?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ImportCounter {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "recv",
            "set",
            "delete",
            "error",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Recv,
            Set,
            Delete,
            Error,
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
                            "recv" => Ok(GeneratedField::Recv),
                            "set" => Ok(GeneratedField::Set),
                            "delete" => Ok(GeneratedField::Delete),
                            "error" => Ok(GeneratedField::Error),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ImportCounter;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.importer.v2.ImportCounter")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ImportCounter, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut recv__ = None;
                let mut set__ = None;
                let mut delete__ = None;
                let mut error__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Recv => {
                            if recv__.is_some() {
                                return Err(serde::de::Error::duplicate_field("recv"));
                            }
                            recv__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Set => {
                            if set__.is_some() {
                                return Err(serde::de::Error::duplicate_field("set"));
                            }
                            set__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Delete => {
                            if delete__.is_some() {
                                return Err(serde::de::Error::duplicate_field("delete"));
                            }
                            delete__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Error => {
                            if error__.is_some() {
                                return Err(serde::de::Error::duplicate_field("error"));
                            }
                            error__ = 
                                Some(map.next_value::<::pbjson::private::NumberDeserialize<_>>()?.0)
                            ;
                        }
                    }
                }
                Ok(ImportCounter {
                    recv: recv__.unwrap_or_default(),
                    set: set__.unwrap_or_default(),
                    delete: delete__.unwrap_or_default(),
                    error: error__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.importer.v2.ImportCounter", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ImportRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.op_code != 0 {
            len += 1;
        }
        if self.msg.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.importer.v2.ImportRequest", len)?;
        if self.op_code != 0 {
            let v = Opcode::from_i32(self.op_code)
                .ok_or_else(|| serde::ser::Error::custom(format!("Invalid variant {}", self.op_code)))?;
            struct_ser.serialize_field("opCode", &v)?;
        }
        if let Some(v) = self.msg.as_ref() {
            match v {
                import_request::Msg::ObjectType(v) => {
                    struct_ser.serialize_field("objectType", v)?;
                }
                import_request::Msg::Permission(v) => {
                    struct_ser.serialize_field("permission", v)?;
                }
                import_request::Msg::RelationType(v) => {
                    struct_ser.serialize_field("relationType", v)?;
                }
                import_request::Msg::Object(v) => {
                    struct_ser.serialize_field("object", v)?;
                }
                import_request::Msg::Relation(v) => {
                    struct_ser.serialize_field("relation", v)?;
                }
            }
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ImportRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "op_code",
            "opCode",
            "object_type",
            "objectType",
            "permission",
            "relation_type",
            "relationType",
            "object",
            "relation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            OpCode,
            ObjectType,
            Permission,
            RelationType,
            Object,
            Relation,
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
                            "opCode" | "op_code" => Ok(GeneratedField::OpCode),
                            "objectType" | "object_type" => Ok(GeneratedField::ObjectType),
                            "permission" => Ok(GeneratedField::Permission),
                            "relationType" | "relation_type" => Ok(GeneratedField::RelationType),
                            "object" => Ok(GeneratedField::Object),
                            "relation" => Ok(GeneratedField::Relation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ImportRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.importer.v2.ImportRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ImportRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut op_code__ = None;
                let mut msg__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::OpCode => {
                            if op_code__.is_some() {
                                return Err(serde::de::Error::duplicate_field("opCode"));
                            }
                            op_code__ = Some(map.next_value::<Opcode>()? as i32);
                        }
                        GeneratedField::ObjectType => {
                            if msg__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objectType"));
                            }
                            msg__ = map.next_value::<::std::option::Option<_>>()?.map(import_request::Msg::ObjectType)
;
                        }
                        GeneratedField::Permission => {
                            if msg__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permission"));
                            }
                            msg__ = map.next_value::<::std::option::Option<_>>()?.map(import_request::Msg::Permission)
;
                        }
                        GeneratedField::RelationType => {
                            if msg__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relationType"));
                            }
                            msg__ = map.next_value::<::std::option::Option<_>>()?.map(import_request::Msg::RelationType)
;
                        }
                        GeneratedField::Object => {
                            if msg__.is_some() {
                                return Err(serde::de::Error::duplicate_field("object"));
                            }
                            msg__ = map.next_value::<::std::option::Option<_>>()?.map(import_request::Msg::Object)
;
                        }
                        GeneratedField::Relation => {
                            if msg__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relation"));
                            }
                            msg__ = map.next_value::<::std::option::Option<_>>()?.map(import_request::Msg::Relation)
;
                        }
                    }
                }
                Ok(ImportRequest {
                    op_code: op_code__.unwrap_or_default(),
                    msg: msg__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.importer.v2.ImportRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ImportResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.object_type.is_some() {
            len += 1;
        }
        if self.permission.is_some() {
            len += 1;
        }
        if self.relation_type.is_some() {
            len += 1;
        }
        if self.object.is_some() {
            len += 1;
        }
        if self.relation.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.importer.v2.ImportResponse", len)?;
        if let Some(v) = self.object_type.as_ref() {
            struct_ser.serialize_field("objectType", v)?;
        }
        if let Some(v) = self.permission.as_ref() {
            struct_ser.serialize_field("permission", v)?;
        }
        if let Some(v) = self.relation_type.as_ref() {
            struct_ser.serialize_field("relationType", v)?;
        }
        if let Some(v) = self.object.as_ref() {
            struct_ser.serialize_field("object", v)?;
        }
        if let Some(v) = self.relation.as_ref() {
            struct_ser.serialize_field("relation", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ImportResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "object_type",
            "objectType",
            "permission",
            "relation_type",
            "relationType",
            "object",
            "relation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ObjectType,
            Permission,
            RelationType,
            Object,
            Relation,
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
                            "permission" => Ok(GeneratedField::Permission),
                            "relationType" | "relation_type" => Ok(GeneratedField::RelationType),
                            "object" => Ok(GeneratedField::Object),
                            "relation" => Ok(GeneratedField::Relation),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ImportResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.importer.v2.ImportResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<ImportResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut object_type__ = None;
                let mut permission__ = None;
                let mut relation_type__ = None;
                let mut object__ = None;
                let mut relation__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ObjectType => {
                            if object_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objectType"));
                            }
                            object_type__ = map.next_value()?;
                        }
                        GeneratedField::Permission => {
                            if permission__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permission"));
                            }
                            permission__ = map.next_value()?;
                        }
                        GeneratedField::RelationType => {
                            if relation_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relationType"));
                            }
                            relation_type__ = map.next_value()?;
                        }
                        GeneratedField::Object => {
                            if object__.is_some() {
                                return Err(serde::de::Error::duplicate_field("object"));
                            }
                            object__ = map.next_value()?;
                        }
                        GeneratedField::Relation => {
                            if relation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relation"));
                            }
                            relation__ = map.next_value()?;
                        }
                    }
                }
                Ok(ImportResponse {
                    object_type: object_type__,
                    permission: permission__,
                    relation_type: relation_type__,
                    object: object__,
                    relation: relation__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.importer.v2.ImportResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for Opcode {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unknown => "OPCODE_UNKNOWN",
            Self::Set => "OPCODE_SET",
            Self::Delete => "OPCODE_DELETE",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for Opcode {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "OPCODE_UNKNOWN",
            "OPCODE_SET",
            "OPCODE_DELETE",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = Opcode;

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
                    .and_then(Opcode::from_i32)
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
                    .and_then(Opcode::from_i32)
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "OPCODE_UNKNOWN" => Ok(Opcode::Unknown),
                    "OPCODE_SET" => Ok(Opcode::Set),
                    "OPCODE_DELETE" => Ok(Opcode::Delete),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
