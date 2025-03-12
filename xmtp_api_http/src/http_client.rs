use crate::{HttpClientError, XmtpHttpApiClient};
use bytes::Bytes;
use http::{uri::Uri, Response};
use reqwest::Body;
use std::pin::Pin;
use xmtp_proto::traits::{ApiError, Client};

impl From<HttpClientError> for ApiError<HttpClientError> {
    fn from(value: HttpClientError) -> Self {
        ApiError::Client { source: value }
    }
}

impl XmtpHttpApiClient {
    async fn request<T>(
        &self,
        request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<http::Response<T>, HttpClientError>
    where
        T: Default + prost::Message + 'static,
        Self: Sized,
    {
        let mut parts = self.host_url.clone().into_parts();
        parts.path_and_query = request
            .uri_ref()
            .map(|u| u.path_and_query())
            .flatten()
            .cloned();
        let request = request
            .method("POST")
            .uri(Uri::from_parts(parts)?)
            .body(body)?;

        let response: Response<Body> = self.http_client.execute(request.try_into()?).await?.into();
        let (parts, body) = response.into_parts();
        let body = http_body_util::BodyExt::collect(body)
            .await
            .map(|buf| T::decode(buf.to_bytes()))??;
        Ok(http::Response::from_parts(parts, body))
    }
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
impl Client for XmtpHttpApiClient {
    type Error = HttpClientError;
    type Stream = Pin<Box<dyn futures::Stream<Item = Result<Bytes, HttpClientError>> + Send>>;
    async fn request<T>(
        &self,
        request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<http::Response<T>, ApiError<Self::Error>>
    where
        T: Default + prost::Message + 'static,
        Self: Sized,
    {
        Ok(self.request(request, body).await?)
    }

    async fn stream(
        &self,
        _request: http::request::Builder,
        _body: Vec<u8>,
    ) -> Result<http::Response<Self::Stream>, ApiError<Self::Error>> {
        // same as unary but server_streaming method
        todo!()
    }
}
