use super::{SmartContractSignatureVerifier, ValidationResponse, VerifierError};
use crate::associations::AccountId;
use ethers::types::{BlockNumber, Bytes};

use xmtp_proto::api_client::BoxedApiClient;
use xmtp_proto::xmtp::identity::api::v1::{
    VerifySmartContractWalletSignatureRequestSignature, VerifySmartContractWalletSignaturesRequest,
    VerifySmartContractWalletSignaturesResponse,
};

pub struct RemoteSignatureVerifier {
    identity_client: BoxedApiClient,
}

impl RemoteSignatureVerifier {
    pub fn new(identity_client: BoxedApiClient) -> Self {
        Self { identity_client }
    }
}

#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
impl SmartContractSignatureVerifier for RemoteSignatureVerifier {
    async fn is_valid_signature(
        &self,
        account_id: AccountId,
        hash: [u8; 32],
        signature: Bytes,
        block_number: Option<BlockNumber>,
    ) -> Result<ValidationResponse, VerifierError> {
        let block_number = block_number.and_then(|bn| bn.as_number()).map(|bn| bn.0[0]);

        let result = self
            .identity_client
            .verify_smart_contract_wallet_signatures(VerifySmartContractWalletSignaturesRequest {
                signatures: vec![VerifySmartContractWalletSignatureRequestSignature {
                    account_id: account_id.into(),
                    block_number,
                    signature: signature.to_vec(),
                    hash: hash.to_vec(),
                }],
            })
            .await?;

        let VerifySmartContractWalletSignaturesResponse { responses } = result;

        Ok(responses
            .into_iter()
            .next()
            .expect("Api given one request will return one response")
            .into())
    }
}

impl Clone for RemoteSignatureVerifier {
    fn clone(&self) -> Self {
        Self {
            identity_client: self.identity_client.clone(),
        }
    }
}
