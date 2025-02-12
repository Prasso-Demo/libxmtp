// @generated
impl serde::Serialize for MultiRemoteAttachment {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.attachments.is_empty() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("xmtp.mls.message_contents.content_types.MultiRemoteAttachment", len)?;
        if !self.attachments.is_empty() {
            struct_ser.serialize_field("attachments", &self.attachments)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for MultiRemoteAttachment {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "attachments",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Attachments,
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
                            "attachments" => Ok(GeneratedField::Attachments),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = MultiRemoteAttachment;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct xmtp.mls.message_contents.content_types.MultiRemoteAttachment")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<MultiRemoteAttachment, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut attachments__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Attachments => {
                            if attachments__.is_some() {
                                return Err(serde::de::Error::duplicate_field("attachments"));
                            }
                            attachments__ = Some(map_.next_value()?);
                        }
                    }
                }
                Ok(MultiRemoteAttachment {
                    attachments: attachments__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("xmtp.mls.message_contents.content_types.MultiRemoteAttachment", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for ReactionAction {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "REACTION_ACTION_UNSPECIFIED",
            Self::Added => "REACTION_ACTION_ADDED",
            Self::Removed => "REACTION_ACTION_REMOVED",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ReactionAction {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "REACTION_ACTION_UNSPECIFIED",
            "REACTION_ACTION_ADDED",
            "REACTION_ACTION_REMOVED",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReactionAction;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "REACTION_ACTION_UNSPECIFIED" => Ok(ReactionAction::Unspecified),
                    "REACTION_ACTION_ADDED" => Ok(ReactionAction::Added),
                    "REACTION_ACTION_REMOVED" => Ok(ReactionAction::Removed),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ReactionSchema {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let variant = match self {
            Self::Unspecified => "REACTION_SCHEMA_UNSPECIFIED",
            Self::Unicode => "REACTION_SCHEMA_UNICODE",
            Self::Shortcode => "REACTION_SCHEMA_SHORTCODE",
            Self::Custom => "REACTION_SCHEMA_CUSTOM",
        };
        serializer.serialize_str(variant)
    }
}
impl<'de> serde::Deserialize<'de> for ReactionSchema {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "REACTION_SCHEMA_UNSPECIFIED",
            "REACTION_SCHEMA_UNICODE",
            "REACTION_SCHEMA_SHORTCODE",
            "REACTION_SCHEMA_CUSTOM",
        ];

        struct GeneratedVisitor;

        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReactionSchema;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(formatter, "expected one of: {:?}", &FIELDS)
            }

            fn visit_i64<E>(self, v: i64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Signed(v), &self)
                    })
            }

            fn visit_u64<E>(self, v: u64) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                i32::try_from(v)
                    .ok()
                    .and_then(|x| x.try_into().ok())
                    .ok_or_else(|| {
                        serde::de::Error::invalid_value(serde::de::Unexpected::Unsigned(v), &self)
                    })
            }

            fn visit_str<E>(self, value: &str) -> std::result::Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                match value {
                    "REACTION_SCHEMA_UNSPECIFIED" => Ok(ReactionSchema::Unspecified),
                    "REACTION_SCHEMA_UNICODE" => Ok(ReactionSchema::Unicode),
                    "REACTION_SCHEMA_SHORTCODE" => Ok(ReactionSchema::Shortcode),
                    "REACTION_SCHEMA_CUSTOM" => Ok(ReactionSchema::Custom),
                    _ => Err(serde::de::Error::unknown_variant(value, FIELDS)),
                }
            }
        }
        deserializer.deserialize_any(GeneratedVisitor)
    }
}
impl serde::Serialize for ReactionV2 {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.reference.is_empty() {
            len += 1;
        }
        if !self.reference_inbox_id.is_empty() {
            len += 1;
        }
        if self.action != 0 {
            len += 1;
        }
        if !self.content.is_empty() {
            len += 1;
        }
        if self.schema != 0 {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("xmtp.mls.message_contents.content_types.ReactionV2", len)?;
        if !self.reference.is_empty() {
            struct_ser.serialize_field("reference", &self.reference)?;
        }
        if !self.reference_inbox_id.is_empty() {
            struct_ser.serialize_field("referenceInboxId", &self.reference_inbox_id)?;
        }
        if self.action != 0 {
            let v = ReactionAction::try_from(self.action)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.action)))?;
            struct_ser.serialize_field("action", &v)?;
        }
        if !self.content.is_empty() {
            struct_ser.serialize_field("content", &self.content)?;
        }
        if self.schema != 0 {
            let v = ReactionSchema::try_from(self.schema)
                .map_err(|_| serde::ser::Error::custom(format!("Invalid variant {}", self.schema)))?;
            struct_ser.serialize_field("schema", &v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for ReactionV2 {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "reference",
            "reference_inbox_id",
            "referenceInboxId",
            "action",
            "content",
            "schema",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            Reference,
            ReferenceInboxId,
            Action,
            Content,
            Schema,
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
                            "reference" => Ok(GeneratedField::Reference),
                            "referenceInboxId" | "reference_inbox_id" => Ok(GeneratedField::ReferenceInboxId),
                            "action" => Ok(GeneratedField::Action),
                            "content" => Ok(GeneratedField::Content),
                            "schema" => Ok(GeneratedField::Schema),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = ReactionV2;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct xmtp.mls.message_contents.content_types.ReactionV2")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<ReactionV2, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut reference__ = None;
                let mut reference_inbox_id__ = None;
                let mut action__ = None;
                let mut content__ = None;
                let mut schema__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::Reference => {
                            if reference__.is_some() {
                                return Err(serde::de::Error::duplicate_field("reference"));
                            }
                            reference__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ReferenceInboxId => {
                            if reference_inbox_id__.is_some() {
                                return Err(serde::de::Error::duplicate_field("referenceInboxId"));
                            }
                            reference_inbox_id__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Action => {
                            if action__.is_some() {
                                return Err(serde::de::Error::duplicate_field("action"));
                            }
                            action__ = Some(map_.next_value::<ReactionAction>()? as i32);
                        }
                        GeneratedField::Content => {
                            if content__.is_some() {
                                return Err(serde::de::Error::duplicate_field("content"));
                            }
                            content__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Schema => {
                            if schema__.is_some() {
                                return Err(serde::de::Error::duplicate_field("schema"));
                            }
                            schema__ = Some(map_.next_value::<ReactionSchema>()? as i32);
                        }
                    }
                }
                Ok(ReactionV2 {
                    reference: reference__.unwrap_or_default(),
                    reference_inbox_id: reference_inbox_id__.unwrap_or_default(),
                    action: action__.unwrap_or_default(),
                    content: content__.unwrap_or_default(),
                    schema: schema__.unwrap_or_default(),
                })
            }
        }
        deserializer.deserialize_struct("xmtp.mls.message_contents.content_types.ReactionV2", FIELDS, GeneratedVisitor)
    }
}
impl serde::Serialize for RemoteAttachmentInfo {
    #[allow(deprecated)]
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        use serde::ser::SerializeStruct;
        let mut len = 0;
        if !self.content_digest.is_empty() {
            len += 1;
        }
        if !self.secret.is_empty() {
            len += 1;
        }
        if !self.nonce.is_empty() {
            len += 1;
        }
        if !self.salt.is_empty() {
            len += 1;
        }
        if !self.scheme.is_empty() {
            len += 1;
        }
        if !self.url.is_empty() {
            len += 1;
        }
        if self.content_length_kb.is_some() {
            len += 1;
        }
        if self.filename.is_some() {
            len += 1;
        }
        let mut struct_ser = serializer.serialize_struct("xmtp.mls.message_contents.content_types.RemoteAttachmentInfo", len)?;
        if !self.content_digest.is_empty() {
            struct_ser.serialize_field("contentDigest", &self.content_digest)?;
        }
        if !self.secret.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("secret", pbjson::private::base64::encode(&self.secret).as_str())?;
        }
        if !self.nonce.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("nonce", pbjson::private::base64::encode(&self.nonce).as_str())?;
        }
        if !self.salt.is_empty() {
            #[allow(clippy::needless_borrow)]
            #[allow(clippy::needless_borrows_for_generic_args)]
            struct_ser.serialize_field("salt", pbjson::private::base64::encode(&self.salt).as_str())?;
        }
        if !self.scheme.is_empty() {
            struct_ser.serialize_field("scheme", &self.scheme)?;
        }
        if !self.url.is_empty() {
            struct_ser.serialize_field("url", &self.url)?;
        }
        if let Some(v) = self.content_length_kb.as_ref() {
            struct_ser.serialize_field("contentLengthKb", v)?;
        }
        if let Some(v) = self.filename.as_ref() {
            struct_ser.serialize_field("filename", v)?;
        }
        struct_ser.end()
    }
}
impl<'de> serde::Deserialize<'de> for RemoteAttachmentInfo {
    #[allow(deprecated)]
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        const FIELDS: &[&str] = &[
            "content_digest",
            "contentDigest",
            "secret",
            "nonce",
            "salt",
            "scheme",
            "url",
            "content_length_kb",
            "contentLengthKb",
            "filename",
        ];

        #[allow(clippy::enum_variant_names)]
        enum GeneratedField {
            ContentDigest,
            Secret,
            Nonce,
            Salt,
            Scheme,
            Url,
            ContentLengthKb,
            Filename,
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
                            "contentDigest" | "content_digest" => Ok(GeneratedField::ContentDigest),
                            "secret" => Ok(GeneratedField::Secret),
                            "nonce" => Ok(GeneratedField::Nonce),
                            "salt" => Ok(GeneratedField::Salt),
                            "scheme" => Ok(GeneratedField::Scheme),
                            "url" => Ok(GeneratedField::Url),
                            "contentLengthKb" | "content_length_kb" => Ok(GeneratedField::ContentLengthKb),
                            "filename" => Ok(GeneratedField::Filename),
                            _ => Err(serde::de::Error::unknown_field(value, FIELDS)),
                        }
                    }
                }
                deserializer.deserialize_identifier(GeneratedVisitor)
            }
        }
        struct GeneratedVisitor;
        impl<'de> serde::de::Visitor<'de> for GeneratedVisitor {
            type Value = RemoteAttachmentInfo;

            fn expecting(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                formatter.write_str("struct xmtp.mls.message_contents.content_types.RemoteAttachmentInfo")
            }

            fn visit_map<V>(self, mut map_: V) -> std::result::Result<RemoteAttachmentInfo, V::Error>
                where
                    V: serde::de::MapAccess<'de>,
            {
                let mut content_digest__ = None;
                let mut secret__ = None;
                let mut nonce__ = None;
                let mut salt__ = None;
                let mut scheme__ = None;
                let mut url__ = None;
                let mut content_length_kb__ = None;
                let mut filename__ = None;
                while let Some(k) = map_.next_key()? {
                    match k {
                        GeneratedField::ContentDigest => {
                            if content_digest__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contentDigest"));
                            }
                            content_digest__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Secret => {
                            if secret__.is_some() {
                                return Err(serde::de::Error::duplicate_field("secret"));
                            }
                            secret__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Nonce => {
                            if nonce__.is_some() {
                                return Err(serde::de::Error::duplicate_field("nonce"));
                            }
                            nonce__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Salt => {
                            if salt__.is_some() {
                                return Err(serde::de::Error::duplicate_field("salt"));
                            }
                            salt__ = 
                                Some(map_.next_value::<::pbjson::private::BytesDeserialize<_>>()?.0)
                            ;
                        }
                        GeneratedField::Scheme => {
                            if scheme__.is_some() {
                                return Err(serde::de::Error::duplicate_field("scheme"));
                            }
                            scheme__ = Some(map_.next_value()?);
                        }
                        GeneratedField::Url => {
                            if url__.is_some() {
                                return Err(serde::de::Error::duplicate_field("url"));
                            }
                            url__ = Some(map_.next_value()?);
                        }
                        GeneratedField::ContentLengthKb => {
                            if content_length_kb__.is_some() {
                                return Err(serde::de::Error::duplicate_field("contentLengthKb"));
                            }
                            content_length_kb__ = 
                                map_.next_value::<::std::option::Option<::pbjson::private::NumberDeserialize<_>>>()?.map(|x| x.0)
                            ;
                        }
                        GeneratedField::Filename => {
                            if filename__.is_some() {
                                return Err(serde::de::Error::duplicate_field("filename"));
                            }
                            filename__ = map_.next_value()?;
                        }
                    }
                }
                Ok(RemoteAttachmentInfo {
                    content_digest: content_digest__.unwrap_or_default(),
                    secret: secret__.unwrap_or_default(),
                    nonce: nonce__.unwrap_or_default(),
                    salt: salt__.unwrap_or_default(),
                    scheme: scheme__.unwrap_or_default(),
                    url: url__.unwrap_or_default(),
                    content_length_kb: content_length_kb__,
                    filename: filename__,
                })
            }
        }
        deserializer.deserialize_struct("xmtp.mls.message_contents.content_types.RemoteAttachmentInfo", FIELDS, GeneratedVisitor)
    }
}
