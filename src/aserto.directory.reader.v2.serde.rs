// @generated
impl serde::Serialize for CheckPermissionRequest {
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
        if self.permission.is_some() {
            len += 1;
        }
        if self.object.is_some() {
            len += 1;
        }
        if self.trace {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.reader.v2.CheckPermissionRequest", len)?;
        if let Some(v) = self.subject.as_ref() {
            struct_ser.serialize_field("subject", v)?;
        }
        if let Some(v) = self.permission.as_ref() {
            struct_ser.serialize_field("permission", v)?;
        }
        if let Some(v) = self.object.as_ref() {
            struct_ser.serialize_field("object", v)?;
        }
        if self.trace {
            struct_ser.serialize_field("trace", &self.trace)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckPermissionRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subject",
            "permission",
            "object",
            "trace",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Subject,
            Permission,
            Object,
            Trace,
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
                            "permission" => Ok(GeneratedField::Permission),
                            "object" => Ok(GeneratedField::Object),
                            "trace" => Ok(GeneratedField::Trace),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CheckPermissionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.reader.v2.CheckPermissionRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CheckPermissionRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut subject__ = None;
                let mut permission__ = None;
                let mut object__ = None;
                let mut trace__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Subject => {
                            if subject__.is_some() {
                                return Err(serde::de::Error::duplicate_field("subject"));
                            }
                            subject__ = map.next_value()?;
                        }
                        GeneratedField::Permission => {
                            if permission__.is_some() {
                                return Err(serde::de::Error::duplicate_field("permission"));
                            }
                            permission__ = map.next_value()?;
                        }
                        GeneratedField::Object => {
                            if object__.is_some() {
                                return Err(serde::de::Error::duplicate_field("object"));
                            }
                            object__ = map.next_value()?;
                        }
                        GeneratedField::Trace => {
                            if trace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trace"));
                            }
                            trace__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(CheckPermissionRequest {
                    subject: subject__,
                    permission: permission__,
                    object: object__,
                    trace: trace__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.reader.v2.CheckPermissionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CheckPermissionResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.check {
            len += 1;
        }
        if !self.trace.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.reader.v2.CheckPermissionResponse", len)?;
        if self.check {
            struct_ser.serialize_field("check", &self.check)?;
        }
        if !self.trace.is_empty() {
            struct_ser.serialize_field("trace", &self.trace)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckPermissionResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "check",
            "trace",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Check,
            Trace,
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
                            "check" => Ok(GeneratedField::Check),
                            "trace" => Ok(GeneratedField::Trace),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CheckPermissionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.reader.v2.CheckPermissionResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CheckPermissionResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut check__ = None;
                let mut trace__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Check => {
                            if check__.is_some() {
                                return Err(serde::de::Error::duplicate_field("check"));
                            }
                            check__ = Some(map.next_value()?);
                        }
                        GeneratedField::Trace => {
                            if trace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trace"));
                            }
                            trace__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(CheckPermissionResponse {
                    check: check__.unwrap_or_default(),
                    trace: trace__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.reader.v2.CheckPermissionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CheckRelationRequest {
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
        if self.trace {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.reader.v2.CheckRelationRequest", len)?;
        if let Some(v) = self.subject.as_ref() {
            struct_ser.serialize_field("subject", v)?;
        }
        if let Some(v) = self.relation.as_ref() {
            struct_ser.serialize_field("relation", v)?;
        }
        if let Some(v) = self.object.as_ref() {
            struct_ser.serialize_field("object", v)?;
        }
        if self.trace {
            struct_ser.serialize_field("trace", &self.trace)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckRelationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "subject",
            "relation",
            "object",
            "trace",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Subject,
            Relation,
            Object,
            Trace,
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
                            "trace" => Ok(GeneratedField::Trace),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CheckRelationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.reader.v2.CheckRelationRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CheckRelationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut subject__ = None;
                let mut relation__ = None;
                let mut object__ = None;
                let mut trace__ = None;
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
                        GeneratedField::Trace => {
                            if trace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trace"));
                            }
                            trace__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(CheckRelationRequest {
                    subject: subject__,
                    relation: relation__,
                    object: object__,
                    trace: trace__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.reader.v2.CheckRelationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CheckRelationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.check {
            len += 1;
        }
        if !self.trace.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.reader.v2.CheckRelationResponse", len)?;
        if self.check {
            struct_ser.serialize_field("check", &self.check)?;
        }
        if !self.trace.is_empty() {
            struct_ser.serialize_field("trace", &self.trace)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckRelationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "check",
            "trace",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Check,
            Trace,
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
                            "check" => Ok(GeneratedField::Check),
                            "trace" => Ok(GeneratedField::Trace),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CheckRelationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.reader.v2.CheckRelationResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CheckRelationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut check__ = None;
                let mut trace__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Check => {
                            if check__.is_some() {
                                return Err(serde::de::Error::duplicate_field("check"));
                            }
                            check__ = Some(map.next_value()?);
                        }
                        GeneratedField::Trace => {
                            if trace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trace"));
                            }
                            trace__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(CheckRelationResponse {
                    check: check__.unwrap_or_default(),
                    trace: trace__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.reader.v2.CheckRelationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for CheckResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.check {
            len += 1;
        }
        if !self.trace.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.reader.v2.CheckResponse", len)?;
        if self.check {
            struct_ser.serialize_field("check", &self.check)?;
        }
        if !self.trace.is_empty() {
            struct_ser.serialize_field("trace", &self.trace)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for CheckResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "check",
            "trace",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Check,
            Trace,
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
                            "check" => Ok(GeneratedField::Check),
                            "trace" => Ok(GeneratedField::Trace),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = CheckResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.reader.v2.CheckResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<CheckResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut check__ = None;
                let mut trace__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Check => {
                            if check__.is_some() {
                                return Err(serde::de::Error::duplicate_field("check"));
                            }
                            check__ = Some(map.next_value()?);
                        }
                        GeneratedField::Trace => {
                            if trace__.is_some() {
                                return Err(serde::de::Error::duplicate_field("trace"));
                            }
                            trace__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(CheckResponse {
                    check: check__.unwrap_or_default(),
                    trace: trace__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.reader.v2.CheckResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetGraphRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.anchor.is_some() {
            len += 1;
        }
        if self.subject.is_some() {
            len += 1;
        }
        if self.relation.is_some() {
            len += 1;
        }
        if self.object.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.reader.v2.GetGraphRequest", len)?;
        if let Some(v) = self.anchor.as_ref() {
            struct_ser.serialize_field("anchor", v)?;
        }
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
impl<'de> serde::Deserialize<'de> for GetGraphRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "anchor",
            "subject",
            "relation",
            "object",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Anchor,
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
                            "anchor" => Ok(GeneratedField::Anchor),
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
            type Value = GetGraphRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.reader.v2.GetGraphRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetGraphRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut anchor__ = None;
                let mut subject__ = None;
                let mut relation__ = None;
                let mut object__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Anchor => {
                            if anchor__.is_some() {
                                return Err(serde::de::Error::duplicate_field("anchor"));
                            }
                            anchor__ = map.next_value()?;
                        }
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
                Ok(GetGraphRequest {
                    anchor: anchor__,
                    subject: subject__,
                    relation: relation__,
                    object: object__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.reader.v2.GetGraphRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetGraphResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.results.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.reader.v2.GetGraphResponse", len)?;
        if !self.results.is_empty() {
            struct_ser.serialize_field("results", &self.results)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetGraphResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "results",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Results,
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
                            "results" => Ok(GeneratedField::Results),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetGraphResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.reader.v2.GetGraphResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetGraphResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut results__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Results => {
                            if results__.is_some() {
                                return Err(serde::de::Error::duplicate_field("results"));
                            }
                            results__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GetGraphResponse {
                    results: results__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.reader.v2.GetGraphResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetObjectManyRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.param.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.reader.v2.GetObjectManyRequest", len)?;
        if !self.param.is_empty() {
            struct_ser.serialize_field("param", &self.param)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetObjectManyRequest {
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
            type Value = GetObjectManyRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.reader.v2.GetObjectManyRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetObjectManyRequest, V::Error>
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
                            param__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GetObjectManyRequest {
                    param: param__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.reader.v2.GetObjectManyRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetObjectManyResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.results.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.reader.v2.GetObjectManyResponse", len)?;
        if !self.results.is_empty() {
            struct_ser.serialize_field("results", &self.results)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetObjectManyResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "results",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Results,
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
                            "results" => Ok(GeneratedField::Results),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetObjectManyResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.reader.v2.GetObjectManyResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetObjectManyResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut results__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Results => {
                            if results__.is_some() {
                                return Err(serde::de::Error::duplicate_field("results"));
                            }
                            results__ = Some(map.next_value()?);
                        }
                    }
                }
                Ok(GetObjectManyResponse {
                    results: results__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.reader.v2.GetObjectManyResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetObjectRequest {
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
        if self.page.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.reader.v2.GetObjectRequest", len)?;
        if let Some(v) = self.param.as_ref() {
            struct_ser.serialize_field("param", v)?;
        }
        if let Some(v) = self.with_relations.as_ref() {
            struct_ser.serialize_field("withRelations", v)?;
        }
        if let Some(v) = self.page.as_ref() {
            struct_ser.serialize_field("page", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetObjectRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "param",
            "with_relations",
            "withRelations",
            "page",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Param,
            WithRelations,
            Page,
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
                            "page" => Ok(GeneratedField::Page),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetObjectRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.reader.v2.GetObjectRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetObjectRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut param__ = None;
                let mut with_relations__ = None;
                let mut page__ = None;
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
                        GeneratedField::Page => {
                            if page__.is_some() {
                                return Err(serde::de::Error::duplicate_field("page"));
                            }
                            page__ = map.next_value()?;
                        }
                    }
                }
                Ok(GetObjectRequest {
                    param: param__,
                    with_relations: with_relations__,
                    page: page__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.reader.v2.GetObjectRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetObjectResponse {
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
        if !self.relations.is_empty() {
            len += 1;
        }
        if self.page.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.reader.v2.GetObjectResponse", len)?;
        if let Some(v) = self.result.as_ref() {
            struct_ser.serialize_field("result", v)?;
        }
        if !self.relations.is_empty() {
            struct_ser.serialize_field("relations", &self.relations)?;
        }
        if let Some(v) = self.page.as_ref() {
            struct_ser.serialize_field("page", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetObjectResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "result",
            "relations",
            "page",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Result,
            Relations,
            Page,
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
                            "relations" => Ok(GeneratedField::Relations),
                            "page" => Ok(GeneratedField::Page),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetObjectResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.reader.v2.GetObjectResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetObjectResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut result__ = None;
                let mut relations__ = None;
                let mut page__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Result => {
                            if result__.is_some() {
                                return Err(serde::de::Error::duplicate_field("result"));
                            }
                            result__ = map.next_value()?;
                        }
                        GeneratedField::Relations => {
                            if relations__.is_some() {
                                return Err(serde::de::Error::duplicate_field("relations"));
                            }
                            relations__ = Some(map.next_value()?);
                        }
                        GeneratedField::Page => {
                            if page__.is_some() {
                                return Err(serde::de::Error::duplicate_field("page"));
                            }
                            page__ = map.next_value()?;
                        }
                    }
                }
                Ok(GetObjectResponse {
                    result: result__,
                    relations: relations__.unwrap_or_default(),
                    page: page__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.reader.v2.GetObjectResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetObjectTypeRequest {
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
        let mut struct_ser = serializer.serialize_struct("aserto.directory.reader.v2.GetObjectTypeRequest", len)?;
        if let Some(v) = self.param.as_ref() {
            struct_ser.serialize_field("param", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetObjectTypeRequest {
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
            type Value = GetObjectTypeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.reader.v2.GetObjectTypeRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetObjectTypeRequest, V::Error>
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
                Ok(GetObjectTypeRequest {
                    param: param__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.reader.v2.GetObjectTypeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetObjectTypeResponse {
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
        let mut struct_ser = serializer.serialize_struct("aserto.directory.reader.v2.GetObjectTypeResponse", len)?;
        if let Some(v) = self.result.as_ref() {
            struct_ser.serialize_field("result", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetObjectTypeResponse {
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
            type Value = GetObjectTypeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.reader.v2.GetObjectTypeResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetObjectTypeResponse, V::Error>
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
                Ok(GetObjectTypeResponse {
                    result: result__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.reader.v2.GetObjectTypeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetObjectTypesRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.page.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.reader.v2.GetObjectTypesRequest", len)?;
        if let Some(v) = self.page.as_ref() {
            struct_ser.serialize_field("page", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetObjectTypesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "page",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Page,
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
                            "page" => Ok(GeneratedField::Page),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetObjectTypesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.reader.v2.GetObjectTypesRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetObjectTypesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut page__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Page => {
                            if page__.is_some() {
                                return Err(serde::de::Error::duplicate_field("page"));
                            }
                            page__ = map.next_value()?;
                        }
                    }
                }
                Ok(GetObjectTypesRequest {
                    page: page__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.reader.v2.GetObjectTypesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetObjectTypesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.results.is_empty() {
            len += 1;
        }
        if self.page.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.reader.v2.GetObjectTypesResponse", len)?;
        if !self.results.is_empty() {
            struct_ser.serialize_field("results", &self.results)?;
        }
        if let Some(v) = self.page.as_ref() {
            struct_ser.serialize_field("page", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetObjectTypesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "results",
            "page",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Results,
            Page,
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
                            "results" => Ok(GeneratedField::Results),
                            "page" => Ok(GeneratedField::Page),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetObjectTypesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.reader.v2.GetObjectTypesResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetObjectTypesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut results__ = None;
                let mut page__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Results => {
                            if results__.is_some() {
                                return Err(serde::de::Error::duplicate_field("results"));
                            }
                            results__ = Some(map.next_value()?);
                        }
                        GeneratedField::Page => {
                            if page__.is_some() {
                                return Err(serde::de::Error::duplicate_field("page"));
                            }
                            page__ = map.next_value()?;
                        }
                    }
                }
                Ok(GetObjectTypesResponse {
                    results: results__.unwrap_or_default(),
                    page: page__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.reader.v2.GetObjectTypesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetObjectsRequest {
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
        if self.page.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.reader.v2.GetObjectsRequest", len)?;
        if let Some(v) = self.param.as_ref() {
            struct_ser.serialize_field("param", v)?;
        }
        if let Some(v) = self.page.as_ref() {
            struct_ser.serialize_field("page", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetObjectsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "param",
            "page",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Param,
            Page,
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
                            "page" => Ok(GeneratedField::Page),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetObjectsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.reader.v2.GetObjectsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetObjectsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut param__ = None;
                let mut page__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Param => {
                            if param__.is_some() {
                                return Err(serde::de::Error::duplicate_field("param"));
                            }
                            param__ = map.next_value()?;
                        }
                        GeneratedField::Page => {
                            if page__.is_some() {
                                return Err(serde::de::Error::duplicate_field("page"));
                            }
                            page__ = map.next_value()?;
                        }
                    }
                }
                Ok(GetObjectsRequest {
                    param: param__,
                    page: page__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.reader.v2.GetObjectsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetObjectsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.results.is_empty() {
            len += 1;
        }
        if self.page.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.reader.v2.GetObjectsResponse", len)?;
        if !self.results.is_empty() {
            struct_ser.serialize_field("results", &self.results)?;
        }
        if let Some(v) = self.page.as_ref() {
            struct_ser.serialize_field("page", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetObjectsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "results",
            "page",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Results,
            Page,
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
                            "results" => Ok(GeneratedField::Results),
                            "page" => Ok(GeneratedField::Page),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetObjectsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.reader.v2.GetObjectsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetObjectsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut results__ = None;
                let mut page__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Results => {
                            if results__.is_some() {
                                return Err(serde::de::Error::duplicate_field("results"));
                            }
                            results__ = Some(map.next_value()?);
                        }
                        GeneratedField::Page => {
                            if page__.is_some() {
                                return Err(serde::de::Error::duplicate_field("page"));
                            }
                            page__ = map.next_value()?;
                        }
                    }
                }
                Ok(GetObjectsResponse {
                    results: results__.unwrap_or_default(),
                    page: page__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.reader.v2.GetObjectsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetPermissionRequest {
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
        let mut struct_ser = serializer.serialize_struct("aserto.directory.reader.v2.GetPermissionRequest", len)?;
        if let Some(v) = self.param.as_ref() {
            struct_ser.serialize_field("param", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetPermissionRequest {
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
            type Value = GetPermissionRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.reader.v2.GetPermissionRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetPermissionRequest, V::Error>
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
                Ok(GetPermissionRequest {
                    param: param__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.reader.v2.GetPermissionRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetPermissionResponse {
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
        let mut struct_ser = serializer.serialize_struct("aserto.directory.reader.v2.GetPermissionResponse", len)?;
        if let Some(v) = self.result.as_ref() {
            struct_ser.serialize_field("result", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetPermissionResponse {
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
            type Value = GetPermissionResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.reader.v2.GetPermissionResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetPermissionResponse, V::Error>
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
                Ok(GetPermissionResponse {
                    result: result__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.reader.v2.GetPermissionResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetPermissionsRequest {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if self.page.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.reader.v2.GetPermissionsRequest", len)?;
        if let Some(v) = self.page.as_ref() {
            struct_ser.serialize_field("page", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetPermissionsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "page",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Page,
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
                            "page" => Ok(GeneratedField::Page),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetPermissionsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.reader.v2.GetPermissionsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetPermissionsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut page__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Page => {
                            if page__.is_some() {
                                return Err(serde::de::Error::duplicate_field("page"));
                            }
                            page__ = map.next_value()?;
                        }
                    }
                }
                Ok(GetPermissionsRequest {
                    page: page__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.reader.v2.GetPermissionsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetPermissionsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.results.is_empty() {
            len += 1;
        }
        if self.page.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.reader.v2.GetPermissionsResponse", len)?;
        if !self.results.is_empty() {
            struct_ser.serialize_field("results", &self.results)?;
        }
        if let Some(v) = self.page.as_ref() {
            struct_ser.serialize_field("page", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetPermissionsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "results",
            "page",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Results,
            Page,
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
                            "results" => Ok(GeneratedField::Results),
                            "page" => Ok(GeneratedField::Page),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetPermissionsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.reader.v2.GetPermissionsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetPermissionsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut results__ = None;
                let mut page__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Results => {
                            if results__.is_some() {
                                return Err(serde::de::Error::duplicate_field("results"));
                            }
                            results__ = Some(map.next_value()?);
                        }
                        GeneratedField::Page => {
                            if page__.is_some() {
                                return Err(serde::de::Error::duplicate_field("page"));
                            }
                            page__ = map.next_value()?;
                        }
                    }
                }
                Ok(GetPermissionsResponse {
                    results: results__.unwrap_or_default(),
                    page: page__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.reader.v2.GetPermissionsResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetRelationRequest {
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
        if self.with_objects.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.reader.v2.GetRelationRequest", len)?;
        if let Some(v) = self.param.as_ref() {
            struct_ser.serialize_field("param", v)?;
        }
        if let Some(v) = self.with_objects.as_ref() {
            struct_ser.serialize_field("withObjects", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetRelationRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "param",
            "with_objects",
            "withObjects",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Param,
            WithObjects,
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
                            "withObjects" | "with_objects" => Ok(GeneratedField::WithObjects),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetRelationRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.reader.v2.GetRelationRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetRelationRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut param__ = None;
                let mut with_objects__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Param => {
                            if param__.is_some() {
                                return Err(serde::de::Error::duplicate_field("param"));
                            }
                            param__ = map.next_value()?;
                        }
                        GeneratedField::WithObjects => {
                            if with_objects__.is_some() {
                                return Err(serde::de::Error::duplicate_field("withObjects"));
                            }
                            with_objects__ = map.next_value()?;
                        }
                    }
                }
                Ok(GetRelationRequest {
                    param: param__,
                    with_objects: with_objects__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.reader.v2.GetRelationRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetRelationResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.results.is_empty() {
            len += 1;
        }
        if !self.objects.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.reader.v2.GetRelationResponse", len)?;
        if !self.results.is_empty() {
            struct_ser.serialize_field("results", &self.results)?;
        }
        if !self.objects.is_empty() {
            struct_ser.serialize_field("objects", &self.objects)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetRelationResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "results",
            "objects",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Results,
            Objects,
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
                            "results" => Ok(GeneratedField::Results),
                            "objects" => Ok(GeneratedField::Objects),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetRelationResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.reader.v2.GetRelationResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetRelationResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut results__ = None;
                let mut objects__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Results => {
                            if results__.is_some() {
                                return Err(serde::de::Error::duplicate_field("results"));
                            }
                            results__ = Some(map.next_value()?);
                        }
                        GeneratedField::Objects => {
                            if objects__.is_some() {
                                return Err(serde::de::Error::duplicate_field("objects"));
                            }
                            objects__ = Some(
                                map.next_value::<std::collections::HashMap<_, _>>()?
                            );
                        }
                    }
                }
                Ok(GetRelationResponse {
                    results: results__.unwrap_or_default(),
                    objects: objects__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.reader.v2.GetRelationResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetRelationTypeRequest {
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
        let mut struct_ser = serializer.serialize_struct("aserto.directory.reader.v2.GetRelationTypeRequest", len)?;
        if let Some(v) = self.param.as_ref() {
            struct_ser.serialize_field("param", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetRelationTypeRequest {
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
            type Value = GetRelationTypeRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.reader.v2.GetRelationTypeRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetRelationTypeRequest, V::Error>
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
                Ok(GetRelationTypeRequest {
                    param: param__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.reader.v2.GetRelationTypeRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetRelationTypeResponse {
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
        let mut struct_ser = serializer.serialize_struct("aserto.directory.reader.v2.GetRelationTypeResponse", len)?;
        if let Some(v) = self.result.as_ref() {
            struct_ser.serialize_field("result", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetRelationTypeResponse {
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
            type Value = GetRelationTypeResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.reader.v2.GetRelationTypeResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetRelationTypeResponse, V::Error>
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
                Ok(GetRelationTypeResponse {
                    result: result__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.reader.v2.GetRelationTypeResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetRelationTypesRequest {
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
        if self.page.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.reader.v2.GetRelationTypesRequest", len)?;
        if let Some(v) = self.param.as_ref() {
            struct_ser.serialize_field("param", v)?;
        }
        if let Some(v) = self.page.as_ref() {
            struct_ser.serialize_field("page", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetRelationTypesRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "param",
            "page",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Param,
            Page,
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
                            "page" => Ok(GeneratedField::Page),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetRelationTypesRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.reader.v2.GetRelationTypesRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetRelationTypesRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut param__ = None;
                let mut page__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Param => {
                            if param__.is_some() {
                                return Err(serde::de::Error::duplicate_field("param"));
                            }
                            param__ = map.next_value()?;
                        }
                        GeneratedField::Page => {
                            if page__.is_some() {
                                return Err(serde::de::Error::duplicate_field("page"));
                            }
                            page__ = map.next_value()?;
                        }
                    }
                }
                Ok(GetRelationTypesRequest {
                    param: param__,
                    page: page__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.reader.v2.GetRelationTypesRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetRelationTypesResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.results.is_empty() {
            len += 1;
        }
        if self.page.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.reader.v2.GetRelationTypesResponse", len)?;
        if !self.results.is_empty() {
            struct_ser.serialize_field("results", &self.results)?;
        }
        if let Some(v) = self.page.as_ref() {
            struct_ser.serialize_field("page", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetRelationTypesResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "results",
            "page",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Results,
            Page,
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
                            "results" => Ok(GeneratedField::Results),
                            "page" => Ok(GeneratedField::Page),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetRelationTypesResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.reader.v2.GetRelationTypesResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetRelationTypesResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut results__ = None;
                let mut page__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Results => {
                            if results__.is_some() {
                                return Err(serde::de::Error::duplicate_field("results"));
                            }
                            results__ = Some(map.next_value()?);
                        }
                        GeneratedField::Page => {
                            if page__.is_some() {
                                return Err(serde::de::Error::duplicate_field("page"));
                            }
                            page__ = map.next_value()?;
                        }
                    }
                }
                Ok(GetRelationTypesResponse {
                    results: results__.unwrap_or_default(),
                    page: page__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.reader.v2.GetRelationTypesResponse", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetRelationsRequest {
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
        if self.page.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.reader.v2.GetRelationsRequest", len)?;
        if let Some(v) = self.param.as_ref() {
            struct_ser.serialize_field("param", v)?;
        }
        if let Some(v) = self.page.as_ref() {
            struct_ser.serialize_field("page", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetRelationsRequest {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "param",
            "page",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Param,
            Page,
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
                            "page" => Ok(GeneratedField::Page),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetRelationsRequest;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.reader.v2.GetRelationsRequest")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetRelationsRequest, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut param__ = None;
                let mut page__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Param => {
                            if param__.is_some() {
                                return Err(serde::de::Error::duplicate_field("param"));
                            }
                            param__ = map.next_value()?;
                        }
                        GeneratedField::Page => {
                            if page__.is_some() {
                                return Err(serde::de::Error::duplicate_field("page"));
                            }
                            page__ = map.next_value()?;
                        }
                    }
                }
                Ok(GetRelationsRequest {
                    param: param__,
                    page: page__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.reader.v2.GetRelationsRequest", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for GetRelationsResponse {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.results.is_empty() {
            len += 1;
        }
        if self.page.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("aserto.directory.reader.v2.GetRelationsResponse", len)?;
        if !self.results.is_empty() {
            struct_ser.serialize_field("results", &self.results)?;
        }
        if let Some(v) = self.page.as_ref() {
            struct_ser.serialize_field("page", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for GetRelationsResponse {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "results",
            "page",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Results,
            Page,
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
                            "results" => Ok(GeneratedField::Results),
                            "page" => Ok(GeneratedField::Page),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = GetRelationsResponse;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct aserto.directory.reader.v2.GetRelationsResponse")
            }

            fn visit_map<V>(self, mut map: V) -> std::result::Result<GetRelationsResponse, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut results__ = None;
                let mut page__ = None;
                while let Some(k) = map.next_key()? {
                    match k {
                        GeneratedField::Results => {
                            if results__.is_some() {
                                return Err(serde::de::Error::duplicate_field("results"));
                            }
                            results__ = Some(map.next_value()?);
                        }
                        GeneratedField::Page => {
                            if page__.is_some() {
                                return Err(serde::de::Error::duplicate_field("page"));
                            }
                            page__ = map.next_value()?;
                        }
                    }
                }
                Ok(GetRelationsResponse {
                    results: results__.unwrap_or_default(),
                    page: page__,
                })
            }
        }
        deserializer.deserialize_struct("aserto.directory.reader.v2.GetRelationsResponse", FIELDS, GeneratedVisitor)
    }
}
