#[allow(clippy::all)]
#[allow(warnings)]
mod generated {
    include!("gen/mod.rs");
}
pub use generated::*;

mod error;
pub use error::*;

pub mod api_client;
pub mod traits;

#[macro_use]
extern crate tracing;

#[cfg(feature = "convert")]
pub mod convert;
#[cfg(feature = "convert")]
pub mod types;
#[cfg(feature = "convert")]
pub mod v4_utils;

#[cfg(test)]
pub mod test {
    #[cfg(target_arch = "wasm32")]
    wasm_bindgen_test::wasm_bindgen_test_configure!(run_in_browser);
}

pub mod prelude {
    #[cfg(any(test, feature = "test-utils"))]
    pub use super::api_client::XmtpTestClient;
    pub use super::api_client::{
        ApiBuilder, ArcedXmtpApi, BoxedXmtpApi, XmtpIdentityClient, XmtpMlsClient, XmtpMlsStreams,
    };
    pub use super::traits::{ApiError, Client, Endpoint, Query};
    pub use super::XmtpApiError;
}

pub mod mls {
    pub mod api {
        pub mod v1 {
            pub mod prelude {
                pub use crate::xmtp::mls::api::v1::*;
            }
        }
    }
}

pub mod identity {
    pub mod api {
        pub mod v1 {
            pub mod prelude {
                pub use crate::xmtp::identity::api::v1::*;
            }
        }
    }
}

pub mod xmtpv4 {
    pub mod message_api {
        pub mod prelude {
            pub use crate::xmtp::xmtpv4::message_api::*;
        }
    }
    pub mod metadata_api {
        pub mod prelude {
            pub use crate::xmtp::xmtpv4::metadata_api::*;
        }
    }
    pub mod payer_api {
        pub mod prelude {
            pub use crate::xmtp::xmtpv4::payer_api::*;
        }
    }
}
