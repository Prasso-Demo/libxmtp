use std::sync::Weak;

use crate::{
    conversation::{ConversationError, SecretConversation},
    networking::XmtpApiClient,
    Client,
};

pub struct Conversations<A>
where
    A: XmtpApiClient,
{
    client: Weak<Client<A>>,
}

impl<A> Conversations<A>
where
    A: XmtpApiClient,
{
    pub fn new(client: Weak<Client<A>>) -> Self {
        Self { client }
    }

    pub async fn new_secret_conversation(
        &self,
        wallet_address: String,
    ) -> Result<SecretConversation<A>, ConversationError> {
        let client = self.client.upgrade().expect("Client deallocated");
        let contacts = client.get_contacts(wallet_address.as_str()).await?;
        let conversation = SecretConversation::new(self.client.clone(), wallet_address, contacts);

        Ok(conversation)
    }
}

#[cfg(test)]
mod tests {
    use crate::{conversations::Conversations, ClientBuilder};
    use std::sync::Arc;

    #[tokio::test]
    async fn create_secret_conversation() {
        let mut alice_client = ClientBuilder::new_test().build().unwrap();
        alice_client.init().await.unwrap();
        let mut bob_client = ClientBuilder::new_test().build().unwrap();
        bob_client.init().await.unwrap();

        let strong_alice_client = Arc::new(alice_client);
        let conversations = Conversations::new(Arc::downgrade(&strong_alice_client));
        let conversation = conversations
            .new_secret_conversation(bob_client.wallet_address().to_string())
            .await
            .unwrap();

        assert_eq!(conversation.peer_address(), bob_client.wallet_address());
        conversation.initialize().await.unwrap();
    }
}
