mod endpoints;
pub use endpoints::*;

mod proto_cache;
pub(crate) use proto_cache::*;

pub mod compat;

#[cfg(any(test, feature = "test-utils"))]
pub use tests::*;
#[cfg(any(test, feature = "test-utils"))]
pub mod tests {
    #[cfg(feature = "grpc-api")]
    pub type TestClient = xmtp_api_grpc::grpc_client::GrpcClient;

    #[cfg(feature = "http-api")]
    pub type TestClient = xmtp_api_http::XmtpHttpApiClient;
}
