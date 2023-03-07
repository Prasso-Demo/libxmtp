use std::collections::HashMap;

use chrono::NaiveDateTime;

use protobuf;
use protobuf::{Message, MessageField};

mod conversation;
mod ecdh;
mod encryption;
mod ethereum_utils;
mod invitation;
mod keys;
mod proto;
mod topic;
use invitation::InvitationV1;
use keys::{
    key_bundle::{PrivateKeyBundle, PublicKeyBundle, SignedPublicKeyBundle},
    private_key::SignedPrivateKey,
    public_key,
};

use conversation::{InvitationContext, TopicData};

use base64::{engine::general_purpose, Engine as _};

pub struct Keystore {
    // Private key bundle powers most operations
    private_key_bundle: Option<PrivateKeyBundle>,
    // Topic Keys
    topic_keys: HashMap<String, TopicData>,

    num_sets: u32,
}

impl Keystore {
    // new() is a constructor for the Keystore struct
    pub fn new() -> Self {
        Keystore {
            // Empty option for private key bundle
            private_key_bundle: None,
            // Topic keys
            topic_keys: HashMap::new(),
            num_sets: 0,
        }
    }

    // == Keystore methods ==
    // Set private identity key from protobuf bytes
    pub fn set_private_key_bundle(&mut self, private_key_bundle: &[u8]) -> Result<(), String> {
        // Deserialize protobuf bytes into a SignedPrivateKey struct
        let private_key_result: protobuf::Result<proto::private_key::PrivateKeyBundle> =
            protobuf::Message::parse_from_bytes(private_key_bundle);
        if private_key_result.is_err() {
            return Err("could not parse private key bundle".to_string());
        }
        // Get the private key from the result
        let private_key = private_key_result.as_ref().unwrap();
        let private_key_bundle = private_key.v2();

        // If the deserialization was successful, set the privateIdentityKey field
        if private_key_result.is_ok() {
            self.private_key_bundle =
                Some(PrivateKeyBundle::from_proto(&private_key_bundle).unwrap());
            self.num_sets += 1;
            return Ok(());
        } else {
            return Err("could not parse private key bundle".to_string());
        }
    }

    // Process proto::keystore::DecryptV1Request
    pub fn decrypt_v1(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        // Decode request bytes into proto::keystore::DecryptV1Request
        let request_result: protobuf::Result<proto::keystore::DecryptV1Request> =
            protobuf::Message::parse_from_bytes(request_bytes);
        if request_result.is_err() {
            return Err("could not parse decrypt v1 request".to_string());
        }
        let request = request_result.as_ref().unwrap();
        // Create a list of responses
        let mut responses = Vec::new();

        let private_key_bundle = self.private_key_bundle.as_ref().unwrap();

        // Iterate over the requests
        for request in &request.requests {
            let payload = &request.payload;
            let peer_keys = &request.peer_keys;
            let header_bytes = &request.header_bytes;
            let is_sender = &request.is_sender;

            let mut response = proto::keystore::decrypt_response::Response::new();

            // Extract XMTP-like X3DH secret
            let secret_result = private_key_bundle.derive_shared_secret_xmtp(
                &PublicKeyBundle::from_proto(&peer_keys)
                    .unwrap()
                    .to_fake_signed_public_key_bundle(),
                &private_key_bundle.pre_keys[0].public_key,
                !is_sender,
            );
            if secret_result.is_err() {
                return Err("could not derive shared secret".to_string());
            }
            let secret = secret_result.unwrap();

            let ciphertext = &payload.aes256_gcm_hkdf_sha256();

            let decrypt_result = encryption::decrypt_v1(
                ciphertext.payload.as_slice(),
                ciphertext.hkdf_salt.as_slice(),
                ciphertext.gcm_nonce.as_slice(),
                &secret,
                Some(header_bytes.as_slice()),
            );

            match decrypt_result {
                Ok(decrypted) => {
                    let mut success_response =
                        proto::keystore::decrypt_response::response::Success::new();
                    success_response.decrypted = decrypted;
                    response.response = Some(
                        proto::keystore::decrypt_response::response::Response::Result(
                            success_response,
                        ),
                    );
                }
                Err(e) => {
                    let mut error_response = proto::keystore::KeystoreError::new();
                    error_response.message = e.to_string();

                    error_response.code = protobuf::EnumOrUnknown::new(
                        proto::keystore::ErrorCode::ERROR_CODE_UNSPECIFIED,
                    );
                    response.response = Some(
                        proto::keystore::decrypt_response::response::Response::Error(
                            error_response,
                        ),
                    );
                }
            }
            responses.push(response);
        }
        let mut response_proto = proto::keystore::DecryptResponse::new();
        response_proto.responses = responses;
        return Ok(response_proto.write_to_bytes().unwrap());
    }

    // Process proto::keystore::DecryptV2Request
    pub fn decrypt_v2(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        // Decode request bytes into proto::keystore::DecryptV2Request
        let request_result: protobuf::Result<proto::keystore::DecryptV2Request> =
            protobuf::Message::parse_from_bytes(request_bytes);
        if request_result.is_err() {
            return Err("could not parse decrypt v2 request".to_string());
        }
        let request = request_result.unwrap();
        // Create a list of responses
        let mut responses = Vec::new();

        // For each request in the request list
        for request in request.requests {
            // TODO: validate the object

            // Extract the payload, headerBytes and contentTopic
            // const { payload, headerBytes, contentTopic } = req
            let payload = request.payload;
            let header_bytes = request.header_bytes;
            let content_topic = request.content_topic;

            // Try to get the topic data
            // const topicData = this.topicKeys.get(contentTopic)
            let topic_data = self.topic_keys.get(&content_topic);
            if topic_data.is_none() {
                // Error with the content_topic
                return Err("could not find topic data".to_string());
            }
            let topic_data = topic_data.unwrap();

            let ciphertext = payload.unwrap().aes256_gcm_hkdf_sha256().clone();

            // Try to decrypt the payload
            let decrypt_result = encryption::decrypt_v1(
                ciphertext.payload.as_slice(),
                ciphertext.hkdf_salt.as_slice(),
                ciphertext.gcm_nonce.as_slice(),
                &topic_data.key,
                Some(header_bytes.as_slice()),
            );

            let mut response = proto::keystore::decrypt_response::Response::new();

            // If decryption was successful, return the decrypted payload
            // If decryption failed, return an error
            match decrypt_result {
                Ok(decrypted) => {
                    let mut success_response =
                        proto::keystore::decrypt_response::response::Success::new();
                    success_response.decrypted = decrypted;
                    response.response = Some(
                        proto::keystore::decrypt_response::response::Response::Result(
                            success_response,
                        ),
                    );
                }
                Err(e) => {
                    let mut error_response = proto::keystore::KeystoreError::new();
                    error_response.message = e;
                    error_response.code = protobuf::EnumOrUnknown::new(
                        proto::keystore::ErrorCode::ERROR_CODE_UNSPECIFIED,
                    );
                    response.response = Some(
                        proto::keystore::decrypt_response::response::Response::Error(
                            error_response,
                        ),
                    );
                }
            }
            responses.push(response);
        }
        let mut response_proto = proto::keystore::DecryptResponse::new();
        response_proto.responses = responses;
        return Ok(response_proto.write_to_bytes().unwrap());
    }

    fn get_conversation_from_topic(
        &self,
        topic: &str,
    ) -> Result<proto::keystore::ConversationReference, String> {
        let topic_result = self.topic_keys.get(topic);
        if topic_result.is_none() {
            return Err("could not find topic data".to_string());
        }
        // Finally, if we have the topic data then add success + conversation object
        let topic_data = topic_result.unwrap();
        let mut success_conversation = proto::keystore::ConversationReference::new();
        success_conversation.topic = topic.to_string();
        success_conversation.created_ns = topic_data.created;
        // Create invitation context from topic data context
        let mut invitation_context = proto::invitation::invitation_v1::Context::new();
        if topic_data.context.is_some() {
            let context = topic_data.context.as_ref().unwrap();
            invitation_context.conversation_id = context.conversation_id.clone();
            for (key, value) in context.metadata.iter() {
                invitation_context
                    .metadata
                    .insert(key.to_string(), value.to_string());
            }
            success_conversation.context = Some(invitation_context).into();
        }
        return Ok(success_conversation);
    }

    // Create invite
    pub fn create_invite(&mut self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        // if no self.private_key_bundle, then return error
        if self.private_key_bundle.is_none() {
            return Err("no private key bundle".to_string());
        }
        // Decode request bytes into proto::keystore::CreateInviteRequest
        let invite_request_result = InvitationV1::invite_request_from_bytes(request_bytes);
        if invite_request_result.is_err() {
            return Err("could not parse invite request".to_string());
        }
        let invite_request = invite_request_result.unwrap();

        // Validate the request
        if invite_request.recipient.is_none() {
            return Err("missing recipient".to_string());
        }
        // Try parsing the recipient into a SignedPublicKeyBundle for validation
        let validation_parse_result =
            SignedPublicKeyBundle::from_proto(invite_request.recipient.as_ref().unwrap());
        if validation_parse_result.is_err() {
            return Err("Could not validate recipient bundle".to_string());
        }
        let recipient = invite_request.recipient.unwrap();

        // Create a random invitation
        let invitation = InvitationV1::create_random(invite_request.context);

        // Create a sealed invitation
        let mut sealed_invitation_header = proto::invitation::SealedInvitationHeaderV1::new();
        let self_private_key_ref = self.private_key_bundle.as_ref().unwrap();
        sealed_invitation_header.sender =
            Some(self_private_key_ref.signed_public_key_bundle()).into();
        sealed_invitation_header.recipient = Some(recipient).into();
        sealed_invitation_header.created_ns = invite_request.created_ns;

        // Now seal the invitation with our self_private_key_ref
        let sealed_invitation_result =
            self_private_key_ref.seal_invitation(&sealed_invitation_header, &invitation);
        if sealed_invitation_result.is_err() {
            return Err("could not seal invitation".to_string());
        }

        let sealed_invitation = sealed_invitation_result.unwrap();
        // Get the header again and deserialize it to print for debugging

        // Add the conversation from the invite
        let save_result = self.save_invitation(&sealed_invitation.write_to_bytes().unwrap());
        if save_result.is_err() {
            return Err("could not save own created invitation".to_string());
        }
        let topic = save_result.unwrap();
        let conversation_result = self.get_conversation_from_topic(&topic);
        if conversation_result.is_err() {
            return Err("could not get conversation from topic".to_string());
        }

        // Create the response
        let mut response = proto::keystore::CreateInviteResponse::new();
        response.conversation = Some(conversation_result.unwrap()).into();
        response.payload = sealed_invitation.write_to_bytes().unwrap();

        return Ok(response.write_to_bytes().unwrap());
    }

    // Save invites keystore impl
    pub fn save_invites(&mut self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        // Decode request bytes into proto::keystore::SaveInvitesRequest
        let request_result: protobuf::Result<proto::keystore::SaveInvitesRequest> =
            protobuf::Message::parse_from_bytes(request_bytes);
        if request_result.is_err() {
            return Err("could not parse save invites request".to_string());
        }
        let request = request_result.unwrap();

        let mut full_response = proto::keystore::SaveInvitesResponse::new();
        // For each request, process the sealed invite + other data to save a conversation
        for request in request.requests {
            let sealed_invitation_bytes = request.payload;
            let save_result = self.save_invitation(&sealed_invitation_bytes);
            let mut response = proto::keystore::save_invites_response::Response::new();
            if save_result.is_err() {
                let mut error_response = proto::keystore::KeystoreError::new();
                error_response.message = save_result.err().unwrap();
                error_response.code = protobuf::EnumOrUnknown::new(
                    proto::keystore::ErrorCode::ERROR_CODE_UNSPECIFIED,
                );
                response.response = Some(
                    proto::keystore::save_invites_response::response::Response::Error(
                        error_response,
                    ),
                );
                full_response.responses.push(response);
                continue;
            }
            // Do not use the request.content_topic as it's not tamper proof, instead use the
            // returned unsealed topic
            let unsealed_topic = save_result.unwrap();
            // Check if topic_keys has the content_topic
            let topic_data = self.topic_keys.get(unsealed_topic.as_str());
            // If not, then return an error
            if topic_data.is_none() {
                let mut error_response = proto::keystore::KeystoreError::new();
                error_response.message =
                    format!("could not find topic data for {}", request.content_topic);
                error_response.code = protobuf::EnumOrUnknown::new(
                    proto::keystore::ErrorCode::ERROR_CODE_UNSPECIFIED,
                );
                response.response = Some(
                    proto::keystore::save_invites_response::response::Response::Error(
                        error_response,
                    ),
                );
                full_response.responses.push(response);
                continue;
            }

            // Finally, if we have the topic data then add success + conversation object
            let topic_data = topic_data.unwrap();
            let mut success_conversation = proto::keystore::ConversationReference::new();
            success_conversation.topic = unsealed_topic;
            success_conversation.created_ns = topic_data.created;
            // Create invitation context from topic data context
            let mut invitation_context = proto::invitation::invitation_v1::Context::new();
            if topic_data.context.is_some() {
                let context = topic_data.context.as_ref().unwrap();
                invitation_context.conversation_id = context.conversation_id.clone();
                for (key, value) in context.metadata.iter() {
                    invitation_context
                        .metadata
                        .insert(key.to_string(), value.to_string());
                }
                success_conversation.context = Some(invitation_context).into();
            }
            let mut success = proto::keystore::save_invites_response::response::Success::new();
            success.conversation = Some(success_conversation).into();

            let success_response =
                proto::keystore::save_invites_response::response::Response::Result(success);
            response.response = Some(success_response);

            full_response.responses.push(response);
        }
        return Ok(full_response.write_to_bytes().unwrap());
    }

    // Save single invitation
    pub fn save_invitation(&mut self, sealed_invitation_bytes: &[u8]) -> Result<String, String> {
        // Check that self.private_key_bundle is set, otherwise return an error
        if self.private_key_bundle.is_none() {
            return Err("private key bundle not set yet".to_string());
        }

        // Deserialize invitation bytes into a protobuf::invitation::InvitationV1 struct
        let invitation_result = InvitationV1::sealed_invitation_from_bytes(sealed_invitation_bytes);
        if invitation_result.is_err() {
            return Err("could not parse invitation".to_string());
        }
        let invitation = invitation_result.unwrap();

        // Need to parse the header_bytes as protobuf::invitation::SealedInvitationHeaderV1
        let header_result: protobuf::Result<proto::invitation::SealedInvitationHeaderV1> =
            protobuf::Message::parse_from_bytes(&invitation.header_bytes);
        if header_result.is_err() {
            return Err("could not parse invitation header".to_string());
        }
        // Get the invitation header from the result
        let invitation_header = header_result.as_ref().unwrap();

        // Check the header time from the sealed invite
        // TODO: check header time from the sealed invite
        let header_time = invitation_header.created_ns;

        // Attempt to decrypt the invitation
        let decrypt_result = self
            .private_key_bundle
            .as_ref()
            .unwrap()
            .unseal_invitation(&invitation, &invitation_header);
        if decrypt_result.is_err() {
            return Err("could not decrypt invitation".to_string());
        }
        // Get the decrypted invitation from the result
        let decrypted_invitation = decrypt_result.unwrap();

        // Encryption field should contain the key bytes
        let key_bytes = decrypted_invitation
            .aes256_gcm_hkdf_sha256()
            .key_material
            .as_slice();

        // Context field should contain conversationId
        let conversation_id = &decrypted_invitation.context.conversation_id;
        let mut context_fields = HashMap::new();
        // Iterate through metadata map and add to context_fields
        for key in decrypted_invitation.context.metadata.keys() {
            context_fields.insert(
                key.to_string(),
                decrypted_invitation.context.metadata[key].to_string(),
            );
        }

        let topic = &decrypted_invitation.topic;

        let optional_context = if decrypted_invitation.context.is_some() {
            Some(InvitationContext {
                conversation_id: conversation_id.to_string(),
                metadata: context_fields,
            })
        } else {
            None
        };
        self.topic_keys.insert(
            topic.to_string(),
            TopicData {
                key: key_bytes.to_vec(),
                // If the invitation has a context, then use the context, otherwise use None
                context: optional_context,
                created: header_time,
            },
        );

        return Ok(topic.to_string());
    }

    // Get serialized keystore.ConversationReference
    pub fn get_v2_conversations(&self) -> Result<Vec<Vec<u8>>, String> {
        let mut conversations = Vec::new();
        for (topic, topic_data) in self.topic_keys.iter() {
            let mut conversation = proto::keystore::ConversationReference::new();
            conversation.topic = topic.clone();
            conversation.created_ns = topic_data.created;
            if topic_data.context.is_some() {
                let context = topic_data.context.as_ref().unwrap();
                let mut invitation_context = proto::invitation::invitation_v1::Context::new();
                invitation_context.conversation_id = context.conversation_id.clone();
                for (key, value) in context.metadata.iter() {
                    invitation_context
                        .metadata
                        .insert(key.to_string(), value.to_string());
                }
                conversation.context = Some(invitation_context).into();
            }
            conversations.push(conversation.write_to_bytes().unwrap());
        }
        // Sort the conversations by created_ns
        conversations.sort_by(|a, b| {
            let a_conversation = proto::keystore::ConversationReference::parse_from_bytes(a)
                .unwrap()
                .created_ns;
            let b_conversation = proto::keystore::ConversationReference::parse_from_bytes(b)
                .unwrap()
                .created_ns;
            a_conversation.cmp(&b_conversation)
        });
        return Ok(conversations);
    }

    pub fn get_topic_key(&self, topic_id: &str) -> Option<Vec<u8>> {
        let topic_data = self.topic_keys.get(topic_id);
        if topic_data.is_none() {
            return None;
        }
        return Some(topic_data.unwrap().key.clone());
    }

    fn create_unspecified_keystore_err(message: &str) -> proto::keystore::KeystoreError {
        let mut error_response = proto::keystore::KeystoreError::new();
        error_response.message = "Recipient is empty".to_string();

        error_response.code =
            protobuf::EnumOrUnknown::new(proto::keystore::ErrorCode::ERROR_CODE_UNSPECIFIED);
        return error_response;
    }

    // Process proto::keystore::EncryptV1Request
    pub fn encrypt_v1(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        // Decode request bytes into proto::keystore::EncryptV1Request
        let request_result: protobuf::Result<proto::keystore::EncryptV1Request> =
            protobuf::Message::parse_from_bytes(request_bytes);
        if request_result.is_err() {
            return Err("could not parse encrypt v1 request".to_string());
        }
        let request = request_result.as_ref().unwrap();
        // Create a list of responses
        let mut responses = Vec::new();

        let private_key_bundle = self.private_key_bundle.as_ref().unwrap();

        // Iterate over the requests
        for request in &request.requests {
            let mut response = proto::keystore::encrypt_response::Response::new();

            // Extract recipient, payload, header_bytes
            // assert that they're not empty otherwise log error and continue
            if request.recipient.is_none() {
                response.response = Some(
                    proto::keystore::encrypt_response::response::Response::Error(
                        Keystore::create_unspecified_keystore_err(
                            "Missing recipient in encrypt request",
                        ),
                    ),
                );
                responses.push(response);
                continue;
            }
            let recipient = request.recipient.as_ref().unwrap();
            let payload = request.payload.as_ref();
            let header_bytes = request.header_bytes.as_ref();

            // TODO: STOPSHIP: hack: massage the recipient PublicKeyBundle into a fake SignedPublicKeyBundle
            // so that we can use the existing sharedSecret function
            let public_key_bundle_result = PublicKeyBundle::from_proto(&recipient);
            if public_key_bundle_result.is_err() {
                response.response = Some(
                    proto::keystore::encrypt_response::response::Response::Error(
                        Keystore::create_unspecified_keystore_err("Could not parse recipient"),
                    ),
                );
                responses.push(response);
                continue;
            }
            let public_key_bundle = public_key_bundle_result.unwrap();
            let signed_public_key_bundle = public_key_bundle.to_fake_signed_public_key_bundle();

            // Extract XMTP-like X3DH secret
            let secret_result = private_key_bundle.derive_shared_secret_xmtp(
                &signed_public_key_bundle,
                &private_key_bundle.pre_keys[0].public_key,
                false, // sender is doing the encrypting
            );
            if secret_result.is_err() {
                response.response = Some(
                    proto::keystore::encrypt_response::response::Response::Error(
                        Keystore::create_unspecified_keystore_err(
                            &secret_result.as_ref().err().unwrap(),
                        ),
                    ),
                );
                responses.push(response);
                continue;
            }
            let secret = secret_result.unwrap();

            // Encrypt the payload
            let encrypt_result = encryption::encrypt_v1(&payload, &secret, Some(&header_bytes));

            match encrypt_result {
                Ok(encrypted) => {
                    // TODO: this can be modularized away
                    let mut success_response =
                        proto::keystore::encrypt_response::response::Success::new();
                    let mut aes256_gcm_hkdf_sha256 =
                        proto::ciphertext::ciphertext::Aes256gcmHkdfsha256::new();
                    aes256_gcm_hkdf_sha256.payload = encrypted.payload;
                    aes256_gcm_hkdf_sha256.hkdf_salt = encrypted.hkdf_salt;
                    aes256_gcm_hkdf_sha256.gcm_nonce = encrypted.gcm_nonce;
                    let mut ciphertext = proto::ciphertext::Ciphertext::new();
                    ciphertext.set_aes256_gcm_hkdf_sha256(aes256_gcm_hkdf_sha256);
                    success_response.encrypted = Some(ciphertext).into();
                    response.response = Some(
                        proto::keystore::encrypt_response::response::Response::Result(
                            success_response,
                        ),
                    );
                }
                Err(e) => {
                    let mut error_response = proto::keystore::KeystoreError::new();
                    error_response.message = e.to_string();

                    error_response.code = protobuf::EnumOrUnknown::new(
                        proto::keystore::ErrorCode::ERROR_CODE_UNSPECIFIED,
                    );
                    response.response = Some(
                        proto::keystore::encrypt_response::response::Response::Error(
                            error_response,
                        ),
                    );
                }
            }
            responses.push(response);
        }
        let mut response_proto = proto::keystore::EncryptResponse::new();
        response_proto.responses = responses;
        return Ok(response_proto.write_to_bytes().unwrap());
    }

    // Process proto::keystore::EncryptV2Request
    pub fn encrypt_v2(&self, request_bytes: &[u8]) -> Result<Vec<u8>, String> {
        // Decode request bytes into proto::keystore::EncryptV2Request
        let request_result: protobuf::Result<proto::keystore::EncryptV2Request> =
            protobuf::Message::parse_from_bytes(request_bytes);
        if request_result.is_err() {
            return Err("could not parse encrypt v2 request".to_string());
        }
        let request = request_result.unwrap();
        // Create a list of responses
        let mut responses = Vec::new();

        // For each request in the request list
        for request in request.requests {
            // TODO: validate the object

            // Extract the payload, headerBytes and contentTopic
            // const { payload, headerBytes, contentTopic } = req
            let payload = request.payload.as_ref();
            let header_bytes = request.header_bytes;
            let content_topic = request.content_topic;

            // Try to get the topic data
            // const topicData = this.topicKeys.get(contentTopic)
            let topic_data = self.topic_keys.get(&content_topic);
            if topic_data.is_none() {
                // Error with the content_topic
                return Err("could not find topic data".to_string());
            }
            let topic_data = topic_data.unwrap();

            // Try to encrypt the payload, (note: yes it says encrypt_v1, need to rename)
            let encrypt_result =
                encryption::encrypt_v1(payload, &topic_data.key, Some(header_bytes.as_slice()));

            let mut response = proto::keystore::encrypt_response::Response::new();

            // If encryption was successful, return the encrypted payload
            // If encryption failed, return an error
            match encrypt_result {
                Ok(encrypted) => {
                    let mut success_response =
                        proto::keystore::encrypt_response::response::Success::new();
                    let mut aes256_gcm_hkdf_sha256 =
                        proto::ciphertext::ciphertext::Aes256gcmHkdfsha256::new();
                    aes256_gcm_hkdf_sha256.payload = encrypted.payload;
                    aes256_gcm_hkdf_sha256.hkdf_salt = encrypted.hkdf_salt;
                    aes256_gcm_hkdf_sha256.gcm_nonce = encrypted.gcm_nonce;
                    let mut ciphertext = proto::ciphertext::Ciphertext::new();
                    ciphertext.set_aes256_gcm_hkdf_sha256(aes256_gcm_hkdf_sha256);
                    success_response.encrypted = Some(ciphertext).into();
                    response.response = Some(
                        proto::keystore::encrypt_response::response::Response::Result(
                            success_response,
                        ),
                    );
                }
                Err(e) => {
                    let mut error_response = proto::keystore::KeystoreError::new();
                    error_response.message = e;
                    error_response.code = protobuf::EnumOrUnknown::new(
                        proto::keystore::ErrorCode::ERROR_CODE_UNSPECIFIED,
                    );
                    response.response = Some(
                        proto::keystore::encrypt_response::response::Response::Error(
                            error_response,
                        ),
                    );
                }
            }
            responses.push(response);
        }
        let mut response_proto = proto::keystore::EncryptResponse::new();
        response_proto.responses = responses;
        return Ok(response_proto.write_to_bytes().unwrap());
    }

    pub fn get_public_key_bundle(&self) -> Result<Vec<u8>, String> {
        if self.private_key_bundle.is_none() {
            return Err("public key bundle is none".to_string());
        }
        // Go from private_key_bundle to public_key_bundle
        let private_key_bundle = self
            .private_key_bundle
            .as_ref()
            .unwrap()
            .signed_public_key_bundle();
        return Ok(private_key_bundle.write_to_bytes().unwrap());
    }

    pub fn get_account_address(&self) -> Result<String, String> {
        if self.private_key_bundle.is_none() {
            return Err("private key bundle is none".to_string());
        }
        self.private_key_bundle.as_ref().unwrap().eth_address()
    }
    // == end keystore api ==
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hkdf_simple() {
        // Test Vectors generated with xmtp-js
        // Test 1
        let secret1 = hex::decode("aff491a0fe153a4ac86065b4b4f6953a4cb33477aa233facb94d5fb88c82778c39167f453aa0690b5358abe9e027ddca5a6185bce3699d8b2ac7efa30510a7991b").unwrap();
        let salt1 = hex::decode("e3412c112c28353088c99bd5c7350c81b1bc879b4d08ea1192ec3c03202ff337")
            .unwrap();
        let expected1 =
            hex::decode("0159d9ad511263c3754a8e2045fadc657c0016b1801720e67bbeb2661c60f176")
                .unwrap();
        let derived1_result = encryption::hkdf(&secret1, &salt1);
        // Check result
        assert!(derived1_result.is_ok());
        assert_eq!(derived1_result.unwrap().to_vec(), expected1);

        // Test 2
        let secret2 = hex::decode("af43ad68d9fcf40967f194497246a6e30515b6c4f574ee2ff58e31df32f5f18040812188cfb5ce34e74ae27b73be08dca626b3eb55c55e6733f32a59dd1b8e021c").unwrap();
        let salt2 = hex::decode("a8500ae6f90a7ccaa096adc55857b90c03508f7d5f8d103a49d58e69058f0c3c")
            .unwrap();
        let expected2 =
            hex::decode("6181d0905f3f31cc3940336696afe1337d9e4d7f6655b9a6eaed2880be38150c")
                .unwrap();
        let derived2_result = encryption::hkdf(&secret2, &salt2);
        // Check result
        assert!(derived2_result.is_ok());
        assert_eq!(derived2_result.unwrap().to_vec(), expected2);
    }

    #[test]
    fn test_hkdf_error() {
        let secret1 = hex::decode("bff491a0fe153a4ac86065b4b4f6953a4cb33477aa233facb94d5fb88c82778c39167f453aa0690b5358abe9e027ddca5a6185bce3699d8b2ac7efa30510a7991b").unwrap();
        let salt1 = hex::decode("e3412c112c28353088c99bd5c7350c81b1bc879b4d08ea1192ec3c03202ff337")
            .unwrap();
        let expected1 =
            hex::decode("0159d9ad511263c3754a8e2045fadc657c0016b1801720e67bbeb2661c60f176")
                .unwrap();
        let derived1_result = encryption::hkdf(&secret1, &salt1);
        // Check result
        assert!(derived1_result.is_ok());
        // Assert not equal
        assert_ne!(derived1_result.unwrap().to_vec(), expected1);
    }

    #[test]
    fn test_hkdf_invalid_key() {
        let secret1 = hex::decode("").unwrap();
        let salt1 = hex::decode("").unwrap();
        let derived1_result = encryption::hkdf(&secret1, &salt1);
        // Check result
        assert!(derived1_result.is_ok());
    }

    #[test]
    fn test_private_key_from_v2_bundle() {
        // = test vectors generated with xmtp-js =
        let private_key_bundle_raw = "EpYDCsgBCMDw7ZjWtOygFxIiCiAvph+Hg/Gk9G1g2EoW1ZDlWVH1nCkn6uRL7GBG3iNophqXAQpPCMDw7ZjWtOygFxpDCkEEeH4w/gK5HMaKu51aec/jiosmqDduIaEA67V7Lbox1cPhz9SIEi6sY/6jVQQXeIjKxzsZSVrM0LXCXjc0VkRmxhJEEkIKQNSujk9ApV5gIKltm0CFhLLuN3Xt2fjkKZBoUH/mswjTaUMTc3qZZzde3ZKMfkNVZYqns4Sn0sgopXzpjQGgjyUSyAEIwPXBtNa07KAXEiIKIOekWIyRJCelxqX+mR8i76KuDO2QV3e42nv8CxJQL0DXGpcBCk8IwPXBtNa07KAXGkMKQQTIePKpkAHxREbLbXfn6XCOwx9YqQWmqLuTHAnqRNj1q5xDLpbgkiyAORFZmVOK8iVq3dT/PWm6WMasPrqdzD7iEkQKQgpAqIj/yKx2wn8VjeWV6wm/neNDEQ6282p3CeJsPDKS56B11Nqc5Y5vUPKcrC1nB2dqBkwvop0fU49Yx4k0CB2evQ==";
        let message = "hello world!";
        let digest = "dQnlvaDHYtK6x/kNdYtbImP6Acy8VCq1498WO+CObKk=";
        let signature_proto_raw = "CkQKQAROtHwYeoBT4LhZEVM6dYaPCDDVy4/9dYSZBvKizAk7J+9f29+1OkAZoGw+FLCHWr/G9cKGfiZf3ln7bTssuIkQAQ==";
        let expected_address = "0xf4c3d5f8f04da9d5eaa7e92f7a6e7f990450c88b";
        // =====

        // For debugging, the secret key is hex encoded bigint:
        // BigInt('0x2fa61f8783f1a4f46d60d84a16d590e55951f59c2927eae44bec6046de2368a6')
        // > 21552218103791599555364469821754606161148148489927333195317013913723696539814n

        let proto_encoded = general_purpose::STANDARD
            .decode(private_key_bundle_raw)
            .unwrap();
        // Deserialize the proto bytes into proto::private_key::PrivateKeyBundleV2
        let signed_private_key: proto::private_key::PrivateKeyBundle =
            protobuf::Message::parse_from_bytes(&proto_encoded).unwrap();
        let private_key_bundle = signed_private_key.v2();

        // Decode signature proto
        let signature: proto::signature::Signature = protobuf::Message::parse_from_bytes(
            &general_purpose::STANDARD
                .decode(signature_proto_raw)
                .unwrap(),
        )
        .unwrap();
        let key_bundle_result = PrivateKeyBundle::from_proto(private_key_bundle);
        assert!(key_bundle_result.is_ok());
        let key_bundle = key_bundle_result.unwrap();
        // Do a raw byte signature verification
        let signature_verified = &key_bundle
            .identity_key
            .verify_signature(message.as_bytes(), &signature.ecdsa_compact().bytes);
        assert!(signature_verified.is_ok());
        // Calculate the eth wallet address from public key
        let eth_address = &key_bundle.identity_key.eth_address().unwrap();
        assert_eq!(eth_address, expected_address);
    }

    #[test]
    fn test_verify_wallet_signature() {
        // = test vectors generated with xmtp-js =
        let address = "0x2Fb28c95E110C6Bb188B41f9E7d6850ccbE48e61";
        let signature_proto_result: proto::signature::Signature = protobuf::Message::parse_from_bytes(&general_purpose::STANDARD.decode("EkIKQKOfb+lUwNCnJrMWQapvY1YNtFheYXa5gH5jZ+IpHPxrIAtWyvMPTMW7WpBb4Mscrie9yRap7H8XbzPPbJKEybI=").unwrap()).unwrap();
        let bytes_to_sign = general_purpose::STANDARD.decode("CIC07umj5I+hFxpDCkEEE27Yj8R97eSoWjEwE35U3pB439S9OSfdrPrDjGH9/JQ5CCb8rjFK1vxxhbHGM2bq1v0PXdk6k/tkbhXmn2WEmw==").unwrap();
        // Encode string as bytes
        let xmtp_identity_signature_payload =
            ethereum_utils::EthereumUtils::xmtp_identity_key_payload(&bytes_to_sign);
        let personal_signature_message =
            SignedPrivateKey::ethereum_personal_sign_payload(&xmtp_identity_signature_payload);
        let signature_verified = SignedPrivateKey::verify_wallet_signature(
            address,
            &personal_signature_message,
            &signature_proto_result,
        );
        assert!(signature_verified.is_ok());
    }

    #[test]
    fn test_recover_wallet_signature() {
        // = test vectors generated with xmtp-js =
        let hex_public_key = "08b8cff59ae3301a430a4104ac471e1ff54947e91e30a4640fe093e6dcb9ac097330b2e2506135d42980454e83bdc639ef7ae4de3debf82aa6800bdd4d1a635d0cdeeab8ed2401d64de22dde";
        let xmtp_test_message = "XMTP : Create Identity\n08b8cff59ae3301a430a4104ac471e1ff54947e91e30a4640fe093e6dcb9ac097330b2e2506135d42980454e83bdc639ef7ae4de3debf82aa6800bdd4d1a635d0cdeeab8ed2401d64de22dde\n\nFor more info: https://xmtp.org/signatures/";
        let xmtp_test_digest = "LDK+7DM/jgDncHBEegvPq0fM9sirQXNHcuNcEPLe5E4=";
        let xmtp_test_address = "0x9DaBcF16c361493e41192BF5901DB1E4E7E7Ca30";

        let xmtp_identity_signature_payload =
            ethereum_utils::EthereumUtils::xmtp_identity_key_payload(
                &hex::decode(hex_public_key).unwrap(),
            );

        assert_eq!(
            xmtp_identity_signature_payload,
            xmtp_test_message.as_bytes()
        );

        let derived_digest =
            SignedPrivateKey::ethereum_personal_digest(xmtp_test_message.as_bytes());
        assert_eq!(
            xmtp_test_digest,
            general_purpose::STANDARD.encode(&derived_digest)
        );
    }

    #[test]
    fn test_simple_decryption() {
        let secret_hex = "7ce6121ed4756aaf8dd0b116ceb7f44ab2f11d4f4caf5924e4bd070353739e6a3c8b039cde75edc2134c7ff76bca5d7ade3fe59bd791f3e73edc97e188c1e4521c";
        let ciphertext_hex = "0ace030a208b2d6b2957ad0fa3fa0ec298c8b4e2308cc6015d50fd40f429450f8bc54dbd35120c37f568081c6b294c36a6b3b71a9b031da6127e0e33bad3d84b1803894d532ea27d8ab3b77d605d46395fcf55c7b49805ee39b8fab9207e324f9a5c326b7807075a131f7c60589291758c1993ac3b1ed5a4bb35e2300093f6fe7ac2abf6f83e3eb08e00e65f0de2d78fedeb693b8b5749b010f068078e1c7be2e4b307ff463d4605dc1427f96ef2262a0e4ad613e87f9d719597b9129517b4fc3e1f1ff95a264d18bc266f8f1f894649508d91f8619e35279cb3879ede9475a528fed2428a878d9f500da9eccadfb2b988c09eed9d6ba2cf6fe40e3730bf7cbec930c2ad5263df7c671e4f8baeeab9e9b45b35f8c4bce74de59009fab8739228eed987b31ce31ff6cbdd688c2055ba3b919b205c59c3b3240d15dc4b527e3b3ebb3ebccb05130e6b42ec80e7b9b49f0d46baf5ae55d1dc5b734c2dee798da6cd6656ba90113fdd0a27aebdb6fbd7de66b0cffbe912d1d9e27b22e77ca8eb13f82bfd2b3adfd8e59c46f115a49727fd1a104d8010ed248bfac0e23632a9b5120fb385c25ff8e76d715df1bc02e6534f2792209796b60c070c4997bfe6aa49f934c8b042624a0377e3ef495c50510f63b934";
        let plaintext_hex = "0a88030ac00108b08b90bfe53012220a20b1d1ae465df4258351c462ea592723753a366263146c69120b4901e4c7a56c8b1a920108b08b90bfe53012440a420a401051d42da81190bbbe080f0cef3356cb476ecf87b112b22a4623f1d22ac358fa08a6160720051acf6ac651335c9114a052a7885ecfaf7c9725f9700075ac22b11a430a41046520443dc4358499e8f0269567bcc27d7264771de694eb84d5c5334e152ede227f3a1606b6dd47129d7c999a6655855cb02dc2b32ee9bf02c01578277dd4ddeb12c20108d88b90bfe53012220a20744cabc19d4d84d9753eed7091bc3047d2e46578cce75193add548f530c7f1d31a940108d88b90bfe53012460a440a409e12294d043420f762ed24e7d21f26328f0f787a964d07f7ebf288f2ab9f750b76b820339ff8cffd4be83adf7177fd29265c4479bf9ab4dc8ed9e5af399a9fab10011a430a4104e0f94416fc0431050a7f4561f8dfdd89e23d24c1d05c50710ef0524316a3bd5ed938c0f111133348fc2aeff399838ce3bd8505182e8582efc6beda0d5144330f";

        // protobuf deseriaize the ciphertext
        let ciphertext_result: proto::ciphertext::Ciphertext =
            protobuf::Message::parse_from_bytes(&hex::decode(ciphertext_hex).unwrap()).unwrap();
        let aes_ciphertext = ciphertext_result.aes256_gcm_hkdf_sha256();
        assert_eq!(aes_ciphertext.gcm_nonce.len(), 12);
        assert_eq!(aes_ciphertext.hkdf_salt.len(), 32);
        assert_eq!(aes_ciphertext.payload.len(), 411);

        // Invoke decrypt_v1 on the ciphertext
        let decrypt_result = encryption::decrypt_v1(
            aes_ciphertext.payload.as_slice(),
            aes_ciphertext.hkdf_salt.as_slice(),
            aes_ciphertext.gcm_nonce.as_slice(),
            hex::decode(secret_hex).unwrap().as_slice(),
            None,
        );

        assert!(decrypt_result.is_ok());
        assert_eq!(hex::encode(decrypt_result.unwrap()), plaintext_hex);
    }

    #[test]
    fn test_xmtp_x3dh_simple() {
        let peer_bundle =  "CpQBCkwIs46U3eUwGkMKQQSp/qE9WdVygIo8+sb45OtE43s68RCqPz+RikceMh+FLuvPp1FcpNiLqURwSrL0o1p/T4HmG4qHn2Mk0lPZqKIBEkQSQgpA416oJdOWzEAQzGiKgDt9ejOkZAtCJ0EN3b2LyapXv+wZPfTlQSI95Db3tTWb/xz1vO/Of3tHDQ0L4bRIqgTVrhKUAQpMCNWOlN3lMBpDCkEEzR0hsrKL6oZeOAabEo3LDYycTjnZ6HSns5Tl9vg3RQ1iEWLrd0GQ4IN8CwwDlGWRUDqcUZNKmqOVXiicDEATuBJECkIKQJiZjxTenDCM/0dMFvqz0d9g2iyGFOM10mi/jaDSxpdUMYm2ZMyNEh94Jq1kYUpptcixuTtb528dnDKlax8B1SE=";
        let my_pre_key_public =  "CkwIy4yU3eUwGkMKQQRibzecVrKk6rgCPNSPyybJib3lKBk1GrI8r/v1yHXcoVuhtmOKffZcoZ3yYl7R1q8+kx61GhwgBQtihzlDyGrKEkQKQgpALqg2w0lg9uhGApJMtgtKrW5qxNgYDNL2BwvnYCHsE15fu9KOdKq0kYKy9TSL9T0Ue0rCYwonA/Qr6lhnFmbh1A==";
        let my_identity_bundle =  "EpADCsUBCMCvgomt/JiiFxIiCiA8iMJ0t2Kc+ilGyAIDtnQOgeQ19RNQzuZuj3J29d+iPxqUAQpMCKeMlN3lMBpDCkEEtrRkcEuQsvY3c6Hwbpyuzk8lbsZK7YgsxSAdmrWft1DM38oM/rrDswhqKUbrMKobt/lN7ShP5JQV+Q2ypvks0RJEEkIKQJgwindCu1V5K46WxWiibrdqodLii2rxgIF/qbSNVREacZ2GSonzXMOlHTMTTo4sy6nw9W1iwAfukqElUZy7J9QSxQEIwNGXmq38mKIXEiIKIF6tvfEObqASql4MbqwWwdvcB1AtHbx6km21Tk6VwCX5GpQBCkwIy4yU3eUwGkMKQQRibzecVrKk6rgCPNSPyybJib3lKBk1GrI8r/v1yHXcoVuhtmOKffZcoZ3yYl7R1q8+kx61GhwgBQtihzlDyGrKEkQKQgpALqg2w0lg9uhGApJMtgtKrW5qxNgYDNL2BwvnYCHsE15fu9KOdKq0kYKy9TSL9T0Ue0rCYwonA/Qr6lhnFmbh1A==";
        let is_recipient = false;
        let pre_key_private =  "CMDRl5qt/JiiFxIiCiBerb3xDm6gEqpeDG6sFsHb3AdQLR28epJttU5OlcAl+RqUAQpMCMuMlN3lMBpDCkEEYm83nFaypOq4AjzUj8smyYm95SgZNRqyPK/79ch13KFbobZjin32XKGd8mJe0davPpMetRocIAULYoc5Q8hqyhJECkIKQC6oNsNJYPboRgKSTLYLSq1uasTYGAzS9gcL52Ah7BNeX7vSjnSqtJGCsvU0i/U9FHtKwmMKJwP0K+pYZxZm4dQ=";
        let secret =  "BNOBBknXpaz9LWs2izeKYFAh3KRS8a7Mibefi38yhyunt3stLHjgvSYPWScBQ4E9VlzTFzOKzR2mnyYhAYrUDSgECK29BC8qeTsusEWZVZso3AC9jFDXV+T7Oyl4+p+pdHMXher5S4xAhJLNEqfGdBLn1Y436cVkppLF/kQjqE8DTwTTxG8VheDyy6sv9PFHZN1C0T6xJ01HH6yVMeZLIOkS13fibjhZ2SUNDYA+/muMyB9AnuG8UN3MNOGLQSPkcW3O";

        let mut x = Keystore::new();
        let res = x.set_private_key_bundle(
            &general_purpose::STANDARD
                .decode(my_identity_bundle)
                .unwrap(),
        );
        assert!(res.is_ok());

        let peer_bundle_proto: proto::public_key::SignedPublicKeyBundle =
            protobuf::Message::parse_from_bytes(
                &general_purpose::STANDARD.decode(peer_bundle).unwrap(),
            )
            .unwrap();
        let peer_bundle_object = SignedPublicKeyBundle::from_proto(&peer_bundle_proto).unwrap();

        let pre_key_proto: proto::public_key::SignedPublicKey =
            protobuf::Message::parse_from_bytes(
                &general_purpose::STANDARD.decode(my_pre_key_public).unwrap(),
            )
            .unwrap();
        let pre_key_object = public_key::signed_public_key_from_proto(&pre_key_proto).unwrap();

        // Do a x3dh shared secret derivation
        let shared_secret_result = x
            .private_key_bundle
            .expect("Must be present for test")
            .derive_shared_secret_xmtp(&peer_bundle_object, &pre_key_object, is_recipient);
        assert!(shared_secret_result.is_ok());
        let shared_secret = shared_secret_result.unwrap();
        assert_eq!(
            shared_secret,
            general_purpose::STANDARD.decode(secret).unwrap()
        );
    }

    #[test]
    fn test_decrypt_invite() {
        let alice_private_b64 = "EpYDCsgBCICHs+6ZvJKjFxIiCiCNtoFf4wgcj3UH5Nhy6vHD94+HbVWUAdYlQ9IYGMv5tBqXAQpPCICHs+6ZvJKjFxpDCkEEYYEjMNUf/Eu1hJH8aZJ8bJrfVitQLGCq0P2QFcEsetPpIHHvB7vqZEctGvq13pbQbkx+LTuKUMwT+cYR6OVBQBJEEkIKQPbipTP3/U4jWwRLI8SbrDJMttTFe+2p55buL9+IUOkCM/IYaB2teaprjWXHhs3dNEkOiI1c5dLeGNrAFBfgYHMSyAEIwIfKiZq8kqMXEiIKIAOcJgVnEPy1OPad9KytYnvN+X67I33mqVKlHMqU9qsZGpcBCk8IwIfKiZq8kqMXGkMKQQTU6+Vdl4ZzsJrhRQvz2Nl7+e8CNdMY04OnC1u5JYZ6ECN+Kez0pJwc2YhypqFisyWuq6s5+FhIa83A6RAtI264EkQKQgpAHH18U/ykyjLFg5T59c35tt/TLZ5lnHwWJGDLaRZAlR81UVfW634+SvEijLbS0IWJ5ZZblwbvMarvfjm0G2i0aw==";
        let bob_private_b64 = "EpYDCsgBCID/5qOavJKjFxIiCiAEu89bIFnCDu1NvDUnPrcW/QwVoBD3MBkDmSW8JCb6gxqXAQpPCID/5qOavJKjFxpDCkEETNqXya/QxjDTgOqgUkrxFEmasoNc9GY83nREU6IXWAhbUzWLbpapP6fVN7adTmG97tztFDb/Zo9K4yxtZ54rUxJEEkIKQNZWu3UbXoDzeY2FPXLWBcMtf0dXCTlGppv8jRWNLRyvaBSpfaXc7QdeKtbIWUKq5rgd88OWkHhZjgA0NPGBqMASyAEIwKW5tZq8kqMXEiIKIMoytCr53r3f/k9Wae/QPdGdPWsAPSLQWFwVez5K8ZGxGpcBCk8IwKW5tZq8kqMXGkMKQQRX0e1CP6Jc5kbjtXF1oxgbFciNSt002UlP4ZS6vDmkCYvQyclEtY3TQcrBXSNNK2JbDwu30+1z+h6DqasrMJc6EkQKQgpAw2rkuwL7e0s3XrrtY6+YhEMmh2nijAMFQKXPFa8edKE1LfMqp0IAGhYXBiGlV7A7yPZDXLLasf11Uy4ww2Wiyw==";
        let alice_invite_b64 = "CtgGCvgECrQCCpcBCk8IgIez7pm8kqMXGkMKQQRhgSMw1R/8S7WEkfxpknxsmt9WK1AsYKrQ/ZAVwSx60+kgce8Hu+pkRy0a+rXeltBuTH4tO4pQzBP5xhHo5UFAEkQSQgpA9uKlM/f9TiNbBEsjxJusMky21MV77annlu4v34hQ6QIz8hhoHa15qmuNZceGzd00SQ6IjVzl0t4Y2sAUF+BgcxKXAQpPCMCHyomavJKjFxpDCkEE1OvlXZeGc7Ca4UUL89jZe/nvAjXTGNODpwtbuSWGehAjfins9KScHNmIcqahYrMlrqurOfhYSGvNwOkQLSNuuBJECkIKQBx9fFP8pMoyxYOU+fXN+bbf0y2eZZx8FiRgy2kWQJUfNVFX1ut+PkrxIoy20tCFieWWW5cG7zGq7345tBtotGsStAIKlwEKTwiA/+ajmrySoxcaQwpBBEzal8mv0MYw04DqoFJK8RRJmrKDXPRmPN50RFOiF1gIW1M1i26WqT+n1Te2nU5hve7c7RQ2/2aPSuMsbWeeK1MSRBJCCkDWVrt1G16A83mNhT1y1gXDLX9HVwk5Rqab/I0VjS0cr2gUqX2l3O0HXirWyFlCqua4HfPDlpB4WY4ANDTxgajAEpcBCk8IwKW5tZq8kqMXGkMKQQRX0e1CP6Jc5kbjtXF1oxgbFciNSt002UlP4ZS6vDmkCYvQyclEtY3TQcrBXSNNK2JbDwu30+1z+h6DqasrMJc6EkQKQgpAw2rkuwL7e0s3XrrtY6+YhEMmh2nijAMFQKXPFa8edKE1LfMqp0IAGhYXBiGlV7A7yPZDXLLasf11Uy4ww2WiyxiAqva1mrySoxcS2gEK1wEKIPl1rD6K3Oj8Ps+zIzfp+n2/hUKqE/ORkHOsZ8kJpIFtEgwb7/dw52hTPD37IsYapAGAJTWRotzIHUtMu1bLd7izktJOh3cJ+ZXODtho02lsNp6DuwNIoEXesdoFRtVZCYqvaiOwnctX+nnPsSfemDmQ1mJ/o4sZyvFAF25ufSBaBqRJeyQjUBbfyuJSWYoDiqAAAMzsWPzrPeVJZFXrcOdDSTA11b+MevlfzcFjitqv/0J2j+pcQo4RFOgtpFK9cUkbcIB2xjRBRXOUQL89BuyMQmb+gg==";
        let bob_public_key_b64 = "CpcBCk8IgP/mo5q8kqMXGkMKQQRM2pfJr9DGMNOA6qBSSvEUSZqyg1z0ZjzedERTohdYCFtTNYtulqk/p9U3tp1OYb3u3O0UNv9mj0rjLG1nnitTEkQSQgpA1la7dRtegPN5jYU9ctYFwy1/R1cJOUamm/yNFY0tHK9oFKl9pdztB14q1shZQqrmuB3zw5aQeFmOADQ08YGowBKXAQpPCMClubWavJKjFxpDCkEEV9HtQj+iXOZG47VxdaMYGxXIjUrdNNlJT+GUurw5pAmL0MnJRLWN00HKwV0jTStiWw8Lt9Ptc/oeg6mrKzCXOhJECkIKQMNq5LsC+3tLN1667WOvmIRDJodp4owDBUClzxWvHnShNS3zKqdCABoWFwYhpVewO8j2Q1yy2rH9dVMuMMNloss=";
        let alice_public_key_b64 = "CpcBCk8IgIez7pm8kqMXGkMKQQRhgSMw1R/8S7WEkfxpknxsmt9WK1AsYKrQ/ZAVwSx60+kgce8Hu+pkRy0a+rXeltBuTH4tO4pQzBP5xhHo5UFAEkQSQgpA9uKlM/f9TiNbBEsjxJusMky21MV77annlu4v34hQ6QIz8hhoHa15qmuNZceGzd00SQ6IjVzl0t4Y2sAUF+BgcxKXAQpPCMCHyomavJKjFxpDCkEE1OvlXZeGc7Ca4UUL89jZe/nvAjXTGNODpwtbuSWGehAjfins9KScHNmIcqahYrMlrqurOfhYSGvNwOkQLSNuuBJECkIKQBx9fFP8pMoyxYOU+fXN+bbf0y2eZZx8FiRgy2kWQJUfNVFX1ut+PkrxIoy20tCFieWWW5cG7zGq7345tBtotGs=";
        let expected_key_material_b64 = "pCZEyn0gkwTrNDOlewVGTHYuqXdWzv9s+WKUWCtdFCk=";
        // xmtp-js unit tests generate this random byte array
        let topic_string = "210,86,199,2,239,247,51,208,205,197,32,162,215,110,185,7,115,73,7,223,5,10,75,19,252,160,139,241,4,205,128,152";

        // Create a keystore, then save Alice's private key bundle
        let mut x = Keystore::new();
        let set_private_result = x.set_private_key_bundle(
            &general_purpose::STANDARD
                .decode(alice_private_b64.as_bytes())
                .unwrap(),
        );
        assert!(set_private_result.is_ok());

        // Save an invite for alice
        let save_invite_result = x.save_invitation(
            &general_purpose::STANDARD
                .decode(alice_invite_b64.as_bytes())
                .unwrap(),
        );
        assert!(save_invite_result.is_ok());

        // Assert that the invite was saved for the topic_string
        let get_invite_result = x.get_topic_key(topic_string);
        assert!(get_invite_result.is_some());
    }

    #[test]
    fn test_create_invite() {
        let private_key_bundle = "EpIDCscBCID6r/ihgr+kFxIiCiA+69dhptWAhSZL61BrxdSObvBGu8h7LC0sebiEBL2DlBqWAQpMCNTC6MXqMBpDCkEEwyc/GHYo+O59IazB6A6IT7sL8aK8pPVV5woD3KWUW9mamD1BbADIRkj5NhsY12MoV3sV6Cdcy4gCOgLVyrKHohJGEkQKQG16AbOXa/zauUTg/OQ7r4iVwoD/gMSAF1vPXEl2ffN8dcamI9WM8F07RsguQCHlULAUY3510GX0wkS2xNq7fyoQARLFAQiAnMWJooK/pBcSIgognCDebi8hRgi5N3DCwGIIvJRt3GUfrp2dmp2SfyJNDOYalAEKTAj4wujF6jAaQwpBBM2XNmLQBhOiCg/sC08UcbCm0osKghqSJmb6Cfxvcu6gHNBP6KRt9E9gv4AMNu4/BNJo/ExTkydvZGyfSUsL90MSRApCCkCdiq2zIGScoXUEEFn7Fvqv0E5tGSxeNQujFLcSTguo+kmDgYOmN9XjfjZdUTjLBTKYuxeXJCXmFwFuqoAvvC2v";

        let create_invite_request_b64 = "ErACCpQBCkwIm8PoxeowGkMKQQTeI6rFEL1eJh5WofKgzDfjP9TETM61G/heGOZP7vRACfMD0ZAzsQ858uvrmqbD7MCFZpTFM6pztTZm9aJ9tzytEkQSQgpA8BReRxtcqrI+aLLW4UKZiREHTo4ub7std5/Klgi7JAEtQTC9Ppp6ZoDPYmK2GWvbTwVOzCElBiZsM+qtUgsVURKWAQpMCL7D6MXqMBpDCkEEY0sZ7+E4hzrdZTpjWiZhuUJHmlwlf96oK/Nm5OyYgRhKNji0oKPe1JX8sij1bjI7XkFiVzZunNhl/Vkmot9g2hJGCkQKQJN3Z1GDiaUnG6N7NxEAuJFN+HKmNfos2XCHNqBjApzQJrVtQApxBntY0vUjtLZyHFFak/33uKYaxpam3EDlDw8QARiA4O+rooK/pBc=";
        // Create a keystore, then save Alice's private key bundle
        let mut x = Keystore::new();
        let set_private_result = x.set_private_key_bundle(
            &general_purpose::STANDARD
                .decode(private_key_bundle.as_bytes())
                .unwrap(),
        );
        assert!(set_private_result.is_ok());
        // Create invite request
        let create_invite_result = x.create_invite(
            &general_purpose::STANDARD
                .decode(create_invite_request_b64.as_bytes())
                .unwrap(),
        );
        assert!(create_invite_result.err().is_none());
    }

    #[test]
    fn test_get_wallet_address() {
        let private_key_bundle = "EpIDCsUBCMDhxZ6FqsOkFxIiCiDw8Tzi1Ke4pqKSAb1vavGlfZ+AvjO3wODJ+UFZtBwqRxqUAQpMCOvW7c7qMBpDCkEEGoTeu8h3/uy+v5j3lDsNb7NAQoYIthqn2NnsKDJiY1AM0cCujfPDfIfnIE4RlKP6h9B3mzArBPh5gMowHT2d0RJEEkIKQGhQA4lJ+mQS2k966sjf3fkMOmTl9W/XUhstk3QPFM2cTHvSZktpMxqcX8ayRIrVZnb3KCaaUKEli7fsgvqgY0ISxwEIgPajroWqw6QXEiIKIHIogys5c9Cv9J/Qlbmao+4/xpY243vxZ3JoBOzoYKSDGpYBCkwIjNftzuowGkMKQQQ8Rsc0PVa8DOXZpUQutmTB+t2TmCO3inJaHMkdDfnaAf/4La6x1qf8NCUi9xv76CALCTGIGhENjveUdfGxrXNLEkYKRApAEI7tmQXGLSArJIJYpAyaDZPy8RV7Zvf+fat0awNHIGN3y0lDSo2d3xmqquwfodQJHjaoaz+Pe/iABQbq7PeGVBAB";
        let wallet_address = "0xBcF6bEa45762d07025cEc882280675f44d12e41C";
        // Create a keystore, then save Alice's private key bundle
        let mut x = Keystore::new();
        let set_private_result = x.set_private_key_bundle(
            &general_purpose::STANDARD
                .decode(private_key_bundle.as_bytes())
                .unwrap(),
        );
        assert!(set_private_result.is_ok());

        let get_wallet_address_result = x.get_account_address();
        assert!(get_wallet_address_result.as_ref().err().is_none());
        assert_eq!(get_wallet_address_result.as_ref().unwrap(), wallet_address);
    }

    #[test]
    fn test_encrypt_v1_with_invalid_params() {
        let private_key_bundle = "EpIDCsUBCMDhxZ6FqsOkFxIiCiDw8Tzi1Ke4pqKSAb1vavGlfZ+AvjO3wODJ+UFZtBwqRxqUAQpMCOvW7c7qMBpDCkEEGoTeu8h3/uy+v5j3lDsNb7NAQoYIthqn2NnsKDJiY1AM0cCujfPDfIfnIE4RlKP6h9B3mzArBPh5gMowHT2d0RJEEkIKQGhQA4lJ+mQS2k966sjf3fkMOmTl9W/XUhstk3QPFM2cTHvSZktpMxqcX8ayRIrVZnb3KCaaUKEli7fsgvqgY0ISxwEIgPajroWqw6QXEiIKIHIogys5c9Cv9J/Qlbmao+4/xpY243vxZ3JoBOzoYKSDGpYBCkwIjNftzuowGkMKQQQ8Rsc0PVa8DOXZpUQutmTB+t2TmCO3inJaHMkdDfnaAf/4La6x1qf8NCUi9xv76CALCTGIGhENjveUdfGxrXNLEkYKRApAEI7tmQXGLSArJIJYpAyaDZPy8RV7Zvf+fat0awNHIGN3y0lDSo2d3xmqquwfodQJHjaoaz+Pe/iABQbq7PeGVBAB";
        // Create a keystore, then save Alice's private key bundle
        let mut x = Keystore::new();
        let set_private_result = x.set_private_key_bundle(
            &general_purpose::STANDARD
                .decode(private_key_bundle.as_bytes())
                .unwrap(),
        );
        assert!(set_private_result.is_ok());

        let mut encrypt_request = proto::keystore::EncryptV1Request::new();

        let mut single_encrypt_request = proto::keystore::encrypt_v1request::Request::new();
        // Add an empty recipient
        single_encrypt_request.recipient = Some(proto::public_key::PublicKeyBundle::new()).into();

        let mut requests = Vec::new();
        requests.push(single_encrypt_request);
        encrypt_request.requests = requests;
        let res = x.encrypt_v1(&encrypt_request.write_to_bytes().unwrap());
        assert!(res.is_ok());
        // Unwrap response
        let response = res.unwrap();
        let encrypt_response_result = protobuf::Message::parse_from_bytes(&response);
        assert!(encrypt_response_result.is_ok());
        // Assert response.responses length == 1
        let encrypt_response: proto::keystore::EncryptResponse = encrypt_response_result.unwrap();
        assert_eq!(1, encrypt_response.responses.len());
    }
}
