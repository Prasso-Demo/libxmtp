use derive_builder::Builder;
use prost::Message;
use std::borrow::Cow;
use xmtp_proto::traits::{BodyError, Endpoint};
use xmtp_proto::xmtp::identity::api::v1::{
    get_inbox_ids_request::Request, GetInboxIdsRequest, GetInboxIdsResponse, FILE_DESCRIPTOR_SET,
};
use xmtp_proto::xmtp::identity::associations::IdentifierKind;

#[derive(Debug, Builder, Default)]
#[builder(setter(strip_option))]
pub struct GetInboxIds {
    #[builder(setter(into))]
    addresses: Vec<String>,
}

impl GetInboxIds {
    pub fn builder() -> GetInboxIdsBuilder {
        Default::default()
    }
}

impl Endpoint for GetInboxIds {
    type Output = GetInboxIdsResponse;
    fn http_endpoint(&self) -> Cow<'static, str> {
        Cow::from("/identity/v1/get-inbox-ids")
    }

    fn grpc_endpoint(&self) -> Cow<'static, str> {
        crate::path_and_query::<GetInboxIdsRequest>(FILE_DESCRIPTOR_SET)
    }

    fn body(&self) -> Result<Vec<u8>, BodyError> {
        Ok(GetInboxIdsRequest {
            requests: self
                .addresses
                .iter()
                .cloned()
                .map(|i| Request {
                    identifier: i,
                    identifier_kind: IdentifierKind::Ethereum as i32,
                })
                .collect(),
        }
        .encode_to_vec())
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use xmtp_proto::prelude::*;
    use xmtp_proto::xmtp::identity::api::v1::{
        get_inbox_ids_response::Response, GetInboxIdsResponse,
    };

    #[test]
    fn test_file_descriptor() {
        use xmtp_proto::xmtp::identity::api::v1::{GetInboxIdsRequest, FILE_DESCRIPTOR_SET};

        let pnq = crate::path_and_query::<GetInboxIdsRequest>(FILE_DESCRIPTOR_SET);
        println!("{}", pnq);
    }

    #[tokio::test]
    async fn test_get_inbox_ids() {
        let resp = hex::decode("0a6e0a2a307865336564616366376534313830613465343562366666363063623438626163353730626631643339124038383337663861313364633130663932643636343564643537383032313635666561613936333338306439396361653138343036653462616230316364386339").unwrap();
        let t: GetInboxIdsResponse = prost::Message::decode(resp.as_slice()).unwrap();
        info!("{:?}", t);
        let client = crate::TestClient::create_local();
        let client = client.build().await.unwrap();
        let endpoint = GetInboxIds::builder()
            .addresses(vec![
                "0xC2e3f813297E7b42a89e0b2FAa66f2034831984f".to_string()
            ])
            .build()
            .unwrap();

        let result: Result<GetInboxIdsResponse, _> = endpoint.query(&client).await;
        match result {
            Ok(response) => {
                assert_eq!(response.responses.len(), 1);
            }
            Err(err) => {
                panic!("Test failed: {}", err);
            }
        }
    }
}
