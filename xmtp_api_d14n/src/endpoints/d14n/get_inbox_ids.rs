use derive_builder::Builder;
use prost::Message;
use std::borrow::Cow;
use xmtp_proto::traits::{BodyError, Endpoint};
use xmtp_proto::xmtp::xmtpv4::message_api::FILE_DESCRIPTOR_SET;
use xmtp_proto::xmtp::xmtpv4::message_api::{
    get_inbox_ids_request, GetInboxIdsRequest, GetInboxIdsResponse,
};

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
        todo!()
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
                .map(|i| get_inbox_ids_request::Request { address: i })
                .collect(),
        }
        .encode_to_vec())
    }
}

#[cfg(all(test, not(target_arch = "wasm32")))]
mod test {
    use crate::d14n::GetInboxIds;
    use xmtp_api_grpc::grpc_client::GrpcClient;
    use xmtp_api_grpc::LOCALHOST_ADDRESS;
    use xmtp_proto::api_client::ApiBuilder;
    use xmtp_proto::traits::Query;
    use xmtp_proto::xmtp::xmtpv4::message_api::{GetInboxIdsRequest, FILE_DESCRIPTOR_SET};

    #[test]
    fn test_file_descriptor() {
        let pnq = crate::path_and_query::<GetInboxIdsRequest>(FILE_DESCRIPTOR_SET);
        println!("{}", pnq);
    }

    #[tokio::test]
    async fn test_get_inbox_ids() {
        let mut client = GrpcClient::builder();
        client.set_app_version("0.0.0".into()).unwrap();
        client.set_tls(false);
        client.set_host(LOCALHOST_ADDRESS.to_string());
        let client = client.build().await.unwrap();

        let endpoint = GetInboxIds::builder()
            .addresses(vec!["".to_string()])
            .build()
            .unwrap();

        //todo: fix later when it was implemented
        let result = endpoint.query(&client).await;
        assert!(result.is_err());
    }
}
