use derive_builder::Builder;
use prost::Message;
use std::borrow::Cow;
use xmtp_proto::traits::{BodyError, Endpoint};
use xmtp_proto::xmtp::identity::api::v1::{
    VerifySmartContractWalletSignatureRequestSignature, VerifySmartContractWalletSignaturesRequest,
    VerifySmartContractWalletSignaturesResponse, FILE_DESCRIPTOR_SET,
};

#[derive(Debug, Builder, Default)]
#[builder(setter(strip_option))]
pub struct VerifySmartContractWalletSignatures {
    #[builder(setter(into))]
    pub signatures: Vec<VerifySmartContractWalletSignatureRequestSignature>,
}

impl VerifySmartContractWalletSignatures {
    pub fn builder() -> VerifySmartContractWalletSignaturesBuilder {
        Default::default()
    }
}

impl Endpoint for VerifySmartContractWalletSignatures {
    type Output = VerifySmartContractWalletSignaturesResponse;
    fn http_endpoint(&self) -> Cow<'static, str> {
        todo!()
    }

    fn grpc_endpoint(&self) -> Cow<'static, str> {
        crate::path_and_query::<VerifySmartContractWalletSignaturesRequest>(FILE_DESCRIPTOR_SET)
    }

    fn body(&self) -> Result<Vec<u8>, BodyError> {
        Ok(VerifySmartContractWalletSignaturesRequest {
            signatures: self.signatures.clone(),
        }
        .encode_to_vec())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use xmtp_proto::prelude::*;

    #[test]
    fn test_file_descriptor() {
        let pnq = crate::path_and_query::<VerifySmartContractWalletSignaturesRequest>(
            FILE_DESCRIPTOR_SET,
        );
        println!("{}", pnq);
    }

    #[tokio::test]
    async fn test_verify_smart_contract_wallet_signatures() {
        let client = crate::TestClient::create_local();
        let client = client.build().await.unwrap();
        let endpoint = VerifySmartContractWalletSignatures::builder()
            .signatures(vec![VerifySmartContractWalletSignatureRequestSignature {
                account_id: "".into(),
                block_number: None,
                hash: vec![],
                signature: vec![],
            }])
            .build()
            .unwrap();

        let result: VerifySmartContractWalletSignaturesResponse =
            endpoint.query(&client).await.unwrap();
        assert_eq!(result.responses.len(), 0);
    }
}
