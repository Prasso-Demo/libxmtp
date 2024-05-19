use log::debug;
use thiserror::Error;

use xmtp_cryptography::signature::AddressValidationError;

use crate::{
    api::ApiClientWrapper,
    client::Client,
    identity::{Identity, IdentityStrategy},
    identity_updates::load_identity_updates,
    retry::Retry,
    storage::EncryptedMessageStore,
    StorageError, XmtpApi,
};

#[derive(Error, Debug)]
pub enum ClientBuilderError {
    #[error(transparent)]
    AddressValidation(#[from] AddressValidationError),

    #[error("Missing parameter: {parameter}")]
    MissingParameter { parameter: &'static str },
    #[error(transparent)]
    ClientError(#[from] crate::client::ClientError),

    // #[error("Failed to serialize/deserialize state for persistence: {source}")]
    // SerializationError { source: serde_json::Error },
    #[error("Required identity was not found in cache.")]
    RequiredIdentityNotFound,

    #[error("Database was configured with a different wallet")]
    StoredIdentityMismatch,

    #[error("Inbox ID mismatch with address")]
    InboxIdMismatch,
    #[error("Uncovered Case")]
    UncoveredCase,
    #[error("Storage Error")]
    StorageError(#[from] StorageError),
    #[error(transparent)]
    Identity(#[from] crate::identity::IdentityError),
    #[error(transparent)]
    WrappedApiError(#[from] crate::api::WrappedApiError),
}

pub struct ClientBuilder<ApiClient> {
    api_client: Option<ApiClient>,
    identity: Option<Identity>,
    store: Option<EncryptedMessageStore>,
    identity_strategy: IdentityStrategy,
}

impl<ApiClient> ClientBuilder<ApiClient>
where
    ApiClient: XmtpApi,
{
    pub fn new(strat: IdentityStrategy) -> Self {
        Self {
            api_client: None,
            identity: None,
            store: None,
            identity_strategy: strat,
        }
    }

    pub fn api_client(mut self, api_client: ApiClient) -> Self {
        self.api_client = Some(api_client);
        self
    }

    pub fn identity(mut self, identity: Identity) -> Self {
        self.identity = Some(identity);
        self
    }

    pub fn store(mut self, store: EncryptedMessageStore) -> Self {
        self.store = Some(store);
        self
    }

    pub async fn build(mut self) -> Result<Client<ApiClient>, ClientBuilderError> {
        debug!("Building client");
        let api_client = self
            .api_client
            .take()
            .ok_or(ClientBuilderError::MissingParameter {
                parameter: "api_client",
            })?;
        let api_client_wrapper = ApiClientWrapper::new(api_client, Retry::default());
        let store = self
            .store
            .take()
            .ok_or(ClientBuilderError::MissingParameter { parameter: "store" })?;
        debug!("Initializing identity");
        let identity = self
            .identity_strategy
            .initialize_identity(&api_client_wrapper, &store)
            .await?;

        // get sequence_id from identity updates loaded into the DB
        load_identity_updates(
            &api_client_wrapper,
            &store.conn()?,
            vec![identity.clone().inbox_id],
        )
        .await?;

        Ok(Client::new(api_client_wrapper, identity, store))
    }
}

#[cfg(test)]
mod tests {
    use xmtp_api_grpc::grpc_api_helper::Client as GrpcClient;
    use xmtp_cryptography::utils::generate_local_wallet;
    use xmtp_id::associations::RecoverableEcdsaSignature;

    use super::{ClientBuilder, IdentityStrategy};
    use crate::{
        storage::{EncryptedMessageStore, StorageOption},
        utils::test::tmp_path,
        Client, InboxOwner,
    };

    async fn get_local_grpc_client() -> GrpcClient {
        GrpcClient::create("http://localhost:5556".to_string(), false)
            .await
            .unwrap()
    }

    async fn register_client(client: &Client<GrpcClient>, owner: &impl InboxOwner) {
        let mut signature_request = client.context.signature_request().unwrap();
        let signature_text = signature_request.signature_text();
        signature_request
            .add_signature(Box::new(RecoverableEcdsaSignature::new(
                signature_text.clone(),
                owner.sign(&signature_text).unwrap().into(),
            )))
            .await
            .unwrap();

        client.register_identity(signature_request).await.unwrap();
    }

    impl ClientBuilder<GrpcClient> {
        pub async fn local_grpc(self) -> Self {
            self.api_client(get_local_grpc_client().await)
        }

        fn temp_store(self) -> Self {
            let tmpdb = tmp_path();
            self.store(
                EncryptedMessageStore::new_unencrypted(StorageOption::Persistent(tmpdb)).unwrap(),
            )
        }

        pub async fn new_test_client(owner: &impl InboxOwner) -> Client<GrpcClient> {
            let client = Self::new(IdentityStrategy::CreateIfNotFound(
                owner.get_address(),
                None,
            ))
            .temp_store()
            .local_grpc()
            .await
            .build()
            .await
            .unwrap();

            register_client(&client, owner).await;

            client
        }
    }

    #[tokio::test]
    async fn builder_test() {
        let wallet = generate_local_wallet();
        let client = ClientBuilder::new_test_client(&wallet).await;
        assert!(!client.installation_public_key().is_empty());
    }

    #[tokio::test]
    async fn identity_persistence_test() {
        let tmpdb = tmp_path();
        let wallet = &generate_local_wallet();

        // Generate a new Wallet + Store
        let store_a =
            EncryptedMessageStore::new_unencrypted(StorageOption::Persistent(tmpdb.clone()))
                .unwrap();

        let client_a = ClientBuilder::new(IdentityStrategy::CreateIfNotFound(
            wallet.get_address(),
            None,
        ))
        .local_grpc()
        .await
        .store(store_a)
        .build()
        .await
        .unwrap();

        register_client(&client_a, wallet).await;

        let keybytes_a = client_a.installation_public_key();
        drop(client_a);

        // Reload the existing store and wallet
        let store_b =
            EncryptedMessageStore::new_unencrypted(StorageOption::Persistent(tmpdb.clone()))
                .unwrap();

        let client_b = ClientBuilder::new(IdentityStrategy::CreateIfNotFound(
            wallet.get_address(),
            None,
        ))
        .local_grpc()
        .await
        .store(store_b)
        .build()
        .await
        .unwrap();
        let keybytes_b = client_b.installation_public_key();
        drop(client_b);

        // Ensure the persistence was used to store the generated keys
        assert_eq!(keybytes_a, keybytes_b);

        // Create a new wallet and store
        let store_c =
            EncryptedMessageStore::new_unencrypted(StorageOption::Persistent(tmpdb.clone()))
                .unwrap();

        ClientBuilder::new(IdentityStrategy::CreateIfNotFound(
            generate_local_wallet().get_address(),
            None,
        ))
        .local_grpc()
        .await
        .store(store_c)
        .build()
        .await
        .expect_err("Testing expected mismatch error");

        // Use cached only strategy
        let store_d =
            EncryptedMessageStore::new_unencrypted(StorageOption::Persistent(tmpdb.clone()))
                .unwrap();
        let client_d = ClientBuilder::new(IdentityStrategy::CachedOnly)
            .local_grpc()
            .await
            .store(store_d)
            .build()
            .await
            .unwrap();
        assert_eq!(client_d.installation_public_key(), keybytes_a);
    }
}
