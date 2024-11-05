use super::*;
use crate::storage::group::GroupQueryArgs;
use crate::storage::group_message::MsgQueryArgs;
use crate::XmtpApi;
use crate::{storage::group::StoredGroup, Client};
use xmtp_id::scw_verifier::SmartContractSignatureVerifier;

impl<ApiClient, V> Client<ApiClient, V>
where
    ApiClient: XmtpApi,
    V: SmartContractSignatureVerifier,
{
    pub(super) fn syncable_groups(
        &self,
        conn: &DbConnection,
    ) -> Result<Vec<Syncable>, DeviceSyncError> {
        let groups = conn
            .find_groups(GroupQueryArgs::default().conversation_type(ConversationType::Group))?
            .into_iter()
            .map(Syncable::Group)
            .collect();
        Ok(groups)
    }

    pub(super) fn syncable_messages(
        &self,
        conn: &DbConnection,
    ) -> Result<Vec<Syncable>, DeviceSyncError> {
        let groups =
            conn.find_groups(GroupQueryArgs::default().conversation_type(ConversationType::Group))?;

        let mut all_messages = vec![];
        for StoredGroup { id, .. } in groups.into_iter() {
            let messages = conn.get_group_messages(&id, &MsgQueryArgs::default())?;
            for msg in messages {
                all_messages.push(Syncable::GroupMessage(msg));
            }
        }

        Ok(all_messages)
    }
}

#[cfg(all(not(target_arch = "wasm32"), test))]
pub(crate) mod tests {
    #[cfg(target_arch = "wasm32")]
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_dedicated_worker);

    const HISTORY_SERVER_HOST: &str = "0.0.0.0";
    const HISTORY_SERVER_PORT: u16 = 5558;

    use std::time::{Duration, Instant};

    use super::*;
    use crate::{assert_ok, builder::ClientBuilder, groups::GroupMetadataOptions};
    use mockito;
    use xmtp_cryptography::utils::generate_local_wallet;
    use xmtp_id::InboxOwner;

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_message_history_sync() {
        let options = mockito::ServerOpts {
            host: HISTORY_SERVER_HOST,
            port: HISTORY_SERVER_PORT + 1,
            ..Default::default()
        };
        let mut server = mockito::Server::new_with_opts_async(options).await;

        let _m = server
            .mock("POST", "/upload")
            .with_status(201)
            .with_body("12345")
            .create();

        let history_sync_url =
            format!("http://{}:{}", HISTORY_SERVER_HOST, HISTORY_SERVER_PORT + 1);

        let wallet = generate_local_wallet();
        let mut amal_a = ClientBuilder::new_test_client(&wallet).await;
        let amal_a_provider = amal_a.mls_provider().unwrap();
        amal_a.history_sync_url = Some(history_sync_url);
        assert_ok!(amal_a.enable_sync(&amal_a_provider).await);

        let amal_a_provider = amal_a.mls_provider().unwrap();
        let amal_a_conn = amal_a_provider.conn_ref();

        // Create an alix client.
        let alix_wallet = generate_local_wallet();
        let alix = ClientBuilder::new_test_client(&alix_wallet).await;

        // Have amal_a create a group and add alix to that group, then send a message.
        let group = amal_a
            .create_group(None, GroupMetadataOptions::default())
            .unwrap();
        group
            .add_members_by_inbox_id(&[alix.inbox_id()])
            .await
            .unwrap();
        group.send_message(&[1, 2, 3]).await.unwrap();

        // Ensure that groups and messages now exists.
        let syncable_groups = amal_a.syncable_groups(amal_a_conn).unwrap();
        assert_eq!(syncable_groups.len(), 1);
        let syncable_messages = amal_a.syncable_messages(amal_a_conn).unwrap();
        assert_eq!(syncable_messages.len(), 2); // welcome message, and message that was just sent

        // Create a second installation for amal.
        let amal_b = ClientBuilder::new_test_client(&wallet).await;
        let amal_b_provider = amal_b.mls_provider().unwrap();
        let amal_b_conn = amal_b_provider.conn_ref();
        assert_ok!(amal_b.enable_sync(&amal_b_provider).await);

        let groups_b = amal_b.syncable_groups(&amal_b_conn).unwrap();
        assert_eq!(groups_b.len(), 0);

        let old_group_id = amal_a.get_sync_group().unwrap().group_id;
        // Check for new welcomes to new groups in the first installation (should be welcomed to a new sync group from amal_b).
        amal_a
            .sync_welcomes(amal_a_conn)
            .await
            .expect("sync_welcomes");
        let new_group_id = amal_a.get_sync_group().unwrap().group_id;
        // group id should have changed to the new sync group created by the second installation
        assert_ne!(old_group_id, new_group_id);

        // recreate the encrypted payload that was uploaded to our mock server using the same encryption key...
        let amal_a_groups = amal_a.syncable_groups(amal_a_conn).unwrap();
        let amal_a_messages = amal_a.syncable_messages(amal_a_conn).unwrap();
        let (enc_payload, _key) = encrypt_syncables_with_key(
            &[amal_a_groups, amal_a_messages],
            // tests always give the same enc key
            DeviceSyncKeyType::new_aes_256_gcm_key(),
        )
        .unwrap();

        // have the mock server reply with the payload
        server
            .mock("GET", &*format!("/files/12345"))
            .with_status(200)
            .with_body(&enc_payload)
            .create();

        // Have the second installation request for a consent sync.
        amal_b
            .send_sync_request(&amal_b_provider, DeviceSyncKind::MessageHistory)
            .await
            .unwrap();

        // Have amal_a receive the message (and auto-process)
        let amal_a_sync_group = amal_a.get_sync_group().unwrap();
        assert_ok!(amal_a_sync_group.sync_with_conn(&amal_a_provider).await);

        // Wait for up to 3 seconds for the reply on amal_b (usually is almost instant)
        let start = Instant::now();
        let mut reply = None;
        while reply.is_none() {
            reply = amal_b
                .sync_reply(&amal_b_provider, DeviceSyncKind::MessageHistory)
                .await
                .unwrap();
            if start.elapsed() > Duration::from_secs(3) {
                panic!("Did not receive consent reply.");
            }
        }

        // Load consents of both installations
        amal_b.sync_welcomes(&amal_b_conn).await.unwrap();
        let groups_a = amal_a.syncable_groups(&amal_a_conn).unwrap();
        let groups_b = amal_b.syncable_groups(&amal_b_conn).unwrap();
        let messages_a = amal_a.syncable_messages(&amal_a_conn).unwrap();
        let messages_b = amal_b.syncable_messages(&amal_b_conn).unwrap();

        // Ensure the consent is synced.
        assert_eq!(groups_a.len(), 1);
        assert_eq!(groups_b.len(), 1);

        assert_eq!(messages_a.len(), 2);
        assert_eq!(messages_b.len(), 2);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 1)]
    async fn test_prepare_groups_to_sync() {
        let wallet = generate_local_wallet();
        let amal_a = ClientBuilder::new_test_client(&wallet).await;
        let _group_a = amal_a
            .create_group(None, GroupMetadataOptions::default())
            .expect("create group");
        let _group_b = amal_a
            .create_group(None, GroupMetadataOptions::default())
            .expect("create group");

        let result = amal_a
            .syncable_groups(&amal_a.store().conn().unwrap())
            .unwrap();
        assert_eq!(result.len(), 2);
    }

    #[tokio::test]
    async fn test_externals_cant_join_sync_group() {
        let wallet = generate_local_wallet();
        let amal = ClientBuilder::new_test_client(&wallet).await;
        let amal_provider = amal.mls_provider().unwrap();
        assert_ok!(amal.enable_sync(&amal_provider).await);
        amal.sync_welcomes(&amal.store().conn().unwrap())
            .await
            .expect("sync welcomes");

        let external_wallet = generate_local_wallet();
        let external_client = ClientBuilder::new_test_client(&external_wallet).await;
        let external_provider = external_client.mls_provider().unwrap();
        assert_ok!(external_client.enable_sync(&external_provider).await);
        external_client
            .sync_welcomes(&external_client.store().conn().unwrap())
            .await
            .expect("sync welcomes");

        let amal_sync_group = amal
            .store()
            .conn()
            .unwrap()
            .latest_sync_group()
            .expect("find sync group");
        assert!(amal_sync_group.is_some());
        let amal_sync_group = amal_sync_group.unwrap();

        // try to join amal's sync group
        let sync_group_id = amal_sync_group.id.clone();
        let created_at_ns = amal_sync_group.created_at_ns;

        let external_client_group = MlsGroup::new(
            external_client.clone(),
            sync_group_id.clone(),
            created_at_ns,
        );
        let result = external_client_group
            .add_members(&[external_wallet.get_address()])
            .await;
        assert!(result.is_err());
    }

    #[test]
    fn test_new_pin() {
        let pin = new_pin();
        assert!(pin.chars().all(|c| c.is_numeric()));
        assert_eq!(pin.len(), 4);
    }

    #[test]
    fn test_new_request_id() {
        let request_id = new_request_id();
        assert_eq!(request_id.len(), ENC_KEY_SIZE);
    }

    #[test]
    fn test_new_key() {
        let sig_key = DeviceSyncKeyType::new_aes_256_gcm_key();
        let enc_key = DeviceSyncKeyType::new_aes_256_gcm_key();
        assert_eq!(sig_key.len(), ENC_KEY_SIZE);
        assert_eq!(enc_key.len(), ENC_KEY_SIZE);
        // ensure keys are different (seed isn't reused)
        assert_ne!(sig_key, enc_key);
    }

    #[test]
    fn test_generate_nonce() {
        let nonce_1 = generate_nonce();
        let nonce_2 = generate_nonce();
        assert_eq!(nonce_1.len(), NONCE_SIZE);
        // ensure nonces are different (seed isn't reused)
        assert_ne!(nonce_1, nonce_2);
    }
}
