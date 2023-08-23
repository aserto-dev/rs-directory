// @generated
impl serde::Serialize for DeleteObjectRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.param.is_some() {
            len += 1;
        }
        if self.with_relations.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.writer.v2.DeleteObjectRequest", len)?;
        if let Some(v) = self.param.as_ref() {
            struct_ser.serialize_field("param", v)?;
        }
        if let Some(v) = self.with_relations.as_ref() {
            struct_ser.serialize_field("withRelations", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteObjectRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "param",
            "with_relations",
            "withRelations",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Param,
            WithRelations,
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
                            "param" => Ok(GeneratedField::Param),
                            "withRelations" | "with_relations" => Ok(GeneratedField::WithRelations),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteObjectRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.writer.v2.DeleteObjectRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DeleteObjectRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut param__ = None;
                let mut with_relations__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Param => {
                            if param__.is_some() {
                                return Err(serde::de::Error::duplicate_field("param"));
                            }
                            param__ = map.next_value()?;
                        }
                        GeneratedField::WithRelations => {
                            if with_relations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("withRelations"));
                            }
                            with_relations__ = map.next_value()?;
                        }
                    }
                }
                Ok(DeleteObjectRequest {
                    param: param__,
                    with_relations: with_relations__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.writer.v2.DeleteObjectRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteObjectResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.result.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.writer.v2.DeleteObjectResponse", len)?;
        if let Some(v) = self.result.as_ref() {
            struct_ser.serialize_field("result", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteObjectResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "result",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Result,
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
                            "result" => Ok(GeneratedField::Result),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteObjectResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.writer.v2.DeleteObjectResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DeleteObjectResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut result__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Result => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("result"));
                            }
                            result__ = map.next_value()?;
                        }
                    }
                }
                Ok(DeleteObjectResponse {
                    result: result__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.writer.v2.DeleteObjectResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteObjectTypeRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.param.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.writer.v2.DeleteObjectTypeRequest", len)?;
        if let Some(v) = self.param.as_ref() {
            struct_ser.serialize_field("param", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteObjectTypeRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "param",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Param,
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
                            "param" => Ok(GeneratedField::Param),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteObjectTypeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.writer.v2.DeleteObjectTypeRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DeleteObjectTypeRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut param__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Param => {
                            if param__.is_some() {
                                return Err(serde::de::Error::duplicate_field("param"));
                            }
                            param__ = map.next_value()?;
                        }
                    }
                }
                Ok(DeleteObjectTypeRequest {
                    param: param__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.writer.v2.DeleteObjectTypeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteObjectTypeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.result.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.writer.v2.DeleteObjectTypeResponse", len)?;
        if let Some(v) = self.result.as_ref() {
            struct_ser.serialize_field("result", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteObjectTypeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "result",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Result,
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
                            "result" => Ok(GeneratedField::Result),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteObjectTypeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.writer.v2.DeleteObjectTypeResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DeleteObjectTypeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut result__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Result => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("result"));
                            }
                            result__ = map.next_value()?;
                        }
                    }
                }
                Ok(DeleteObjectTypeResponse {
                    result: result__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.writer.v2.DeleteObjectTypeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeletePermissionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.param.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.writer.v2.DeletePermissionRequest", len)?;
        if let Some(v) = self.param.as_ref() {
            struct_ser.serialize_field("param", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeletePermissionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "param",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Param,
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
                            "param" => Ok(GeneratedField::Param),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeletePermissionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.writer.v2.DeletePermissionRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DeletePermissionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut param__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Param => {
                            if param__.is_some() {
                                return Err(serde::de::Error::duplicate_field("param"));
                            }
                            param__ = map.next_value()?;
                        }
                    }
                }
                Ok(DeletePermissionRequest {
                    param: param__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.writer.v2.DeletePermissionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeletePermissionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.result.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.writer.v2.DeletePermissionResponse", len)?;
        if let Some(v) = self.result.as_ref() {
            struct_ser.serialize_field("result", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeletePermissionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "result",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Result,
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
                            "result" => Ok(GeneratedField::Result),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeletePermissionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.writer.v2.DeletePermissionResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DeletePermissionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut result__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Result => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("result"));
                            }
                            result__ = map.next_value()?;
                        }
                    }
                }
                Ok(DeletePermissionResponse {
                    result: result__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.writer.v2.DeletePermissionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteRelationRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.param.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.writer.v2.DeleteRelationRequest", len)?;
        if let Some(v) = self.param.as_ref() {
            struct_ser.serialize_field("param", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteRelationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "param",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Param,
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
                            "param" => Ok(GeneratedField::Param),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteRelationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.writer.v2.DeleteRelationRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DeleteRelationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut param__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Param => {
                            if param__.is_some() {
                                return Err(serde::de::Error::duplicate_field("param"));
                            }
                            param__ = map.next_value()?;
                        }
                    }
                }
                Ok(DeleteRelationRequest {
                    param: param__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.writer.v2.DeleteRelationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteRelationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.result.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.writer.v2.DeleteRelationResponse", len)?;
        if let Some(v) = self.result.as_ref() {
            struct_ser.serialize_field("result", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteRelationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "result",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Result,
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
                            "result" => Ok(GeneratedField::Result),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteRelationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.writer.v2.DeleteRelationResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DeleteRelationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut result__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Result => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("result"));
                            }
                            result__ = map.next_value()?;
                        }
                    }
                }
                Ok(DeleteRelationResponse {
                    result: result__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.writer.v2.DeleteRelationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteRelationTypeRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.param.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.writer.v2.DeleteRelationTypeRequest", len)?;
        if let Some(v) = self.param.as_ref() {
            struct_ser.serialize_field("param", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteRelationTypeRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "param",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Param,
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
                            "param" => Ok(GeneratedField::Param),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteRelationTypeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.writer.v2.DeleteRelationTypeRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DeleteRelationTypeRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut param__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Param => {
                            if param__.is_some() {
                                return Err(serde::de::Error::duplicate_field("param"));
                            }
                            param__ = map.next_value()?;
                        }
                    }
                }
                Ok(DeleteRelationTypeRequest {
                    param: param__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.writer.v2.DeleteRelationTypeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for DeleteRelationTypeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.result.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.writer.v2.DeleteRelationTypeResponse", len)?;
        if let Some(v) = self.result.as_ref() {
            struct_ser.serialize_field("result", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for DeleteRelationTypeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "result",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Result,
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
                            "result" => Ok(GeneratedField::Result),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = DeleteRelationTypeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.writer.v2.DeleteRelationTypeResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<DeleteRelationTypeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut result__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Result => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("result"));
                            }
                            result__ = map.next_value()?;
                        }
                    }
                }
                Ok(DeleteRelationTypeResponse {
                    result: result__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.writer.v2.DeleteRelationTypeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetObjectRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.object.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.writer.v2.SetObjectRequest", len)?;
        if let Some(v) = self.object.as_ref() {
            struct_ser.serialize_field("object", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetObjectRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "object",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = SetObjectRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.writer.v2.SetObjectRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SetObjectRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut object__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Object => {
                            if object__.is_some() {
                                return Err(serde::de::Error::duplicate_field("object"));
                            }
                            object__ = map.next_value()?;
                        }
                    }
                }
                Ok(SetObjectRequest {
                    object: object__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.writer.v2.SetObjectRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetObjectResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.result.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.writer.v2.SetObjectResponse", len)?;
        if let Some(v) = self.result.as_ref() {
            struct_ser.serialize_field("result", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetObjectResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "result",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Result,
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
                            "result" => Ok(GeneratedField::Result),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetObjectResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.writer.v2.SetObjectResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SetObjectResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut result__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Result => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("result"));
                            }
                            result__ = map.next_value()?;
                        }
                    }
                }
                Ok(SetObjectResponse {
                    result: result__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.writer.v2.SetObjectResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetObjectTypeRequest {
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
        let mut struct_ser = serializer.serialize_struct("aserto.directory.writer.v2.SetObjectTypeRequest", len)?;
        if let Some(v) = self.object_type.as_ref() {
            struct_ser.serialize_field("objectType", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetObjectTypeRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "object_type",
            "objectType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = SetObjectTypeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.writer.v2.SetObjectTypeRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SetObjectTypeRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut object_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::ObjectType => {
                            if object_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objectType"));
                            }
                            object_type__ = map.next_value()?;
                        }
                    }
                }
                Ok(SetObjectTypeRequest {
                    object_type: object_type__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.writer.v2.SetObjectTypeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetObjectTypeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.result.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.writer.v2.SetObjectTypeResponse", len)?;
        if let Some(v) = self.result.as_ref() {
            struct_ser.serialize_field("result", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetObjectTypeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "result",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Result,
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
                            "result" => Ok(GeneratedField::Result),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetObjectTypeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.writer.v2.SetObjectTypeResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SetObjectTypeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut result__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Result => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("result"));
                            }
                            result__ = map.next_value()?;
                        }
                    }
                }
                Ok(SetObjectTypeResponse {
                    result: result__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.writer.v2.SetObjectTypeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetPermissionRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.permission.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.writer.v2.SetPermissionRequest", len)?;
        if let Some(v) = self.permission.as_ref() {
            struct_ser.serialize_field("permission", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetPermissionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "permission",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = SetPermissionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.writer.v2.SetPermissionRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SetPermissionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut permission__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Permission => {
                            if permission__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permission"));
                            }
                            permission__ = map.next_value()?;
                        }
                    }
                }
                Ok(SetPermissionRequest {
                    permission: permission__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.writer.v2.SetPermissionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetPermissionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.result.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.writer.v2.SetPermissionResponse", len)?;
        if let Some(v) = self.result.as_ref() {
            struct_ser.serialize_field("result", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetPermissionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "result",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Result,
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
                            "result" => Ok(GeneratedField::Result),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetPermissionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.writer.v2.SetPermissionResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SetPermissionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut result__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Result => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("result"));
                            }
                            result__ = map.next_value()?;
                        }
                    }
                }
                Ok(SetPermissionResponse {
                    result: result__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.writer.v2.SetPermissionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetRelationRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.relation.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.writer.v2.SetRelationRequest", len)?;
        if let Some(v) = self.relation.as_ref() {
            struct_ser.serialize_field("relation", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetRelationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "relation",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
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
            type Value = SetRelationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.writer.v2.SetRelationRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SetRelationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut relation__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Relation => {
                            if relation__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relation"));
                            }
                            relation__ = map.next_value()?;
                        }
                    }
                }
                Ok(SetRelationRequest {
                    relation: relation__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.writer.v2.SetRelationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetRelationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.result.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.writer.v2.SetRelationResponse", len)?;
        if let Some(v) = self.result.as_ref() {
            struct_ser.serialize_field("result", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetRelationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "result",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Result,
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
                            "result" => Ok(GeneratedField::Result),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetRelationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.writer.v2.SetRelationResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SetRelationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut result__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Result => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("result"));
                            }
                            result__ = map.next_value()?;
                        }
                    }
                }
                Ok(SetRelationResponse {
                    result: result__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.writer.v2.SetRelationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetRelationTypeRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.relation_type.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.writer.v2.SetRelationTypeRequest", len)?;
        if let Some(v) = self.relation_type.as_ref() {
            struct_ser.serialize_field("relationType", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetRelationTypeRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "relation_type",
            "relationType",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            RelationType,
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
                            "relationType" | "relation_type" => Ok(GeneratedField::RelationType),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetRelationTypeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.writer.v2.SetRelationTypeRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SetRelationTypeRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut relation_type__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::RelationType => {
                            if relation_type__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relationType"));
                            }
                            relation_type__ = map.next_value()?;
                        }
                    }
                }
                Ok(SetRelationTypeRequest {
                    relation_type: relation_type__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.writer.v2.SetRelationTypeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for SetRelationTypeResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.result.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.writer.v2.SetRelationTypeResponse", len)?;
        if let Some(v) = self.result.as_ref() {
            struct_ser.serialize_field("result", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for SetRelationTypeResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "result",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Result,
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
                            "result" => Ok(GeneratedField::Result),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = SetRelationTypeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.writer.v2.SetRelationTypeResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<SetRelationTypeResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut result__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Result => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("result"));
                            }
                            result__ = map.next_value()?;
                        }
                    }
                }
                Ok(SetRelationTypeResponse {
                    result: result__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.writer.v2.SetRelationTypeResponse", FIELDS, GeneratedVisitor)
    }
}
