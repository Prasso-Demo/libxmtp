//! Api Client Traits

use http_body_util::BodyExt;
use prost::bytes::Bytes;
use std::borrow::Cow;
use thiserror::Error;
use xmtp_common::{retry_async, retryable, BoxedRetry, RetryableError};

use crate::{ApiEndpoint, Code, ProtoError, XmtpApiError};

pub trait Endpoint {
    type Output: prost::Message + Default;

    fn http_endpoint(&self) -> Cow<'static, str>;

    fn grpc_endpoint(&self) -> Cow<'static, str>;

    fn body(&self) -> Result<Vec<u8>, BodyError>;
}
/*
/// Stream
pub struct Streaming<S, E>
where
    S: Stream<Item = Result<Bytes, ApiError<E>>>,
{
    inner: S,
}
*/

#[derive(thiserror::Error, Debug)]
enum MockE {}
use futures::Future;
pub type BoxedClient = Box<
    dyn Client<
        Error = ApiError<MockE>,
        Stream = futures::stream::Once<Box<dyn Future<Output = ()>>>,
    >,
>;

fn try_to_call(c: BoxedClient) {
    // c.request(Default::default(), vec![]).unwrap()
}

#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
pub trait Client {
    type Error: std::error::Error + Send + Sync + 'static;
    type Stream: futures::Stream;

    async fn request<T>(
        &self,
        request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<http::Response<T>, ApiError<Self::Error>>
    where
        T: Default + prost::Message + 'static,
        Self: Sized;

    async fn stream(
        &self,
        request: http::request::Builder,
        body: Vec<u8>,
    ) -> Result<http::Response<Self::Stream>, ApiError<Self::Error>>;
}

// query can return a Wrapper XmtpResponse<T> that implements both Future and Stream. If stream is used on singular response, just a stream of one item. This lets us re-use query for everything.
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
pub trait Query<T, C>
where
    C: Client + Send + Sync,
    T: Send,
{
    async fn query(&self, client: &C) -> Result<T, ApiError<C::Error>>;

    async fn query_retryable(&self, client: &C, retry: BoxedRetry) -> Result<T, ApiError<C::Error>>
    where
        C::Error: RetryableError,
    {
        retry_async!(retry, (async { self.query(client).await }))
    }
}

// blanket Query implementation for a bare Endpoint
#[cfg_attr(not(target_arch = "wasm32"), async_trait::async_trait)]
#[cfg_attr(target_arch = "wasm32", async_trait::async_trait(?Send))]
impl<E, T, C> Query<T, C> for E
where
    E: Endpoint<Output = T> + Sync,
    C: Client + Sync + Send,
    T: Default + prost::Message + 'static,
    // TODO: figure out how to get conversions right
    // T: TryFrom<E::Output>,
    // ApiError<<C as Client>::Error>: From<<T as TryFrom<E::Output>>::Error>,
{
    async fn query(&self, client: &C) -> Result<T, ApiError<C::Error>> {
        let mut request = http::Request::builder();
        let endpoint = if cfg!(feature = "http-api") {
            request = request.header("Content-Type", "application/x-protobuf");
            request = request.header("Accept", "application/x-protobuf");
            self.http_endpoint()
        } else {
            self.grpc_endpoint()
        };
        let request = request.uri(endpoint.as_ref());
        /*let rsp: http::Response<
            http_body_util::combinators::UnsyncBoxBody<prost::bytes::Bytes, ApiError<C::Error>>,
        > = client.request(request, self.body()?).await?;*/
        let rsp = client.request::<T>(request, self.body()?).await?;
        Ok(rsp.into_body())
        // let b = http_body_util::BodyStream::new(rsp.into_body());
        // let bytes = BodyExt::collect(b).await.map(|b| b.to_bytes())?;

        /*
                let bytes = BodyExt::collect(rsp.into_body())
                    .await
                    .map(|b| b.to_bytes());
        */
        // let body = rsp.into_body();
        // println!("body: {}", hex::encode(&body));

        // Ok(prost::Message::decode(bytes)?)
        // Ok(prost::Message::decode(rsp.into_body())?)
        // let mut final_rsp: E::Output = Default::default();
        // final_rsp.merge(rsp.into_body()).map(Option::Some)?;
        // println!("{:?}", final_rsp);
        // Ok(final_rsp)
    }
}

#[derive(Debug, Error)]
#[non_exhaustive]
pub enum ApiError<E>
where
    E: std::error::Error + Send + Sync + 'static,
{
    /// The client encountered an error.
    #[error("client error: {}", source)]
    Client {
        /// The client error.
        source: E,
    },
    #[error(transparent)]
    Http(#[from] http::Error),
    #[error(transparent)]
    Body(#[from] BodyError),
    #[error(transparent)]
    DecodeError(#[from] prost::DecodeError),
    #[error(transparent)]
    Conversion(#[from] crate::ConversionError),
    #[error(transparent)]
    ProtoError(#[from] ProtoError),
}

impl<E> XmtpApiError for ApiError<E>
where
    E: std::error::Error + Send + Sync + RetryableError + 'static,
{
    fn api_call(&self) -> Option<ApiEndpoint> {
        None
    }

    fn code(&self) -> Option<Code> {
        None
    }

    fn grpc_message(&self) -> Option<&str> {
        None
    }
}

impl<E> RetryableError for ApiError<E>
where
    E: RetryableError + std::error::Error + Send + Sync + 'static,
{
    fn is_retryable(&self) -> bool {
        use ApiError::*;
        match self {
            Client { source } => retryable!(source),
            Body(e) => retryable!(e),
            Http(_) => true,
            DecodeError(_) => false,
            Conversion(_) => false,
            ProtoError(_) => false,
        }
    }
}

// Infallible errors by definition can never occur
impl<E: Send + Sync + std::error::Error> From<std::convert::Infallible> for ApiError<E> {
    fn from(_v: std::convert::Infallible) -> ApiError<E> {
        unreachable!()
    }
}

#[derive(Debug, Error)]
pub enum BodyError {
    #[error("placeholder")]
    Placeholder,
}

impl RetryableError for BodyError {
    fn is_retryable(&self) -> bool {
        false
    }
}
/*
#[cfg(any(test, feature = "test-utils"))]
pub mod mock {
    use super::*;

    pub struct MockClient;
    pub struct MockStream;

    #[derive(thiserror::Error, Debug)]
    pub enum MockError {}

    type Repeat = Box<dyn (FnMut() -> prost::bytes::Bytes)>;
    type MockStreamT = futures::stream::RepeatWith<Repeat>;
    #[cfg(not(target_arch = "wasm32"))]
    mockall::mock! {
        pub MockClient {}

        #[async_trait::async_trait]
        impl Client for MockClient {
            type Error = MockError;
            type Stream = MockStreamT;
            async fn request<T>(
                &self,
                request: http::request::Builder,
                body: Vec<u8>,
            ) -> Result<http::Response<T>, ApiError<MockError>> where Self: Sized;

            async fn stream(
                &self,
                request: http::request::Builder,
                body: Vec<u8>,
            ) -> Result<http::Response<MockStreamT>, ApiError<MockError>>;
        }
    }

    #[cfg(target_arch = "wasm32")]
    mockall::mock! {
        pub MockClient {}

        #[async_trait::async_trait(?Send)]
        impl Client for MockClient {
            type Error = MockError;
            type Stream = MockStreamT;
            async fn request(
                &self,
                request: http::request::Builder,
                body: Vec<u8>,
            ) -> Result<http::Response<Bytes>, ApiError<MockError>>;

            async fn stream(
                &self,
                request: http::request::Builder,
                body: Vec<u8>,
            ) -> Result<http::Response<MockStreamT>, ApiError<MockError>>;
        }
    }
}
*/
