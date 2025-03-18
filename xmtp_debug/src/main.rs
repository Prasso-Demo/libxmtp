mod app;
mod args;
mod constants;
mod logger;

use clap::Parser;
use color_eyre::eyre::Result;

use xmtp_api_grpc::{grpc_api_helper::Client as GrpcClient, GrpcError};
use xmtp_proto::traits::ApiClientError;

type DbgClientApi = xmtp_proto::api_client::ArcedXmtpApi<ApiClientError<GrpcError>>;
type DbgClient = xmtp_mls::client::Client<DbgClientApi>;

#[macro_use]
extern crate tracing;

#[tokio::main]
async fn main() -> Result<()> {
    color_eyre::install()?;

    let opts = args::AppOpts::parse();
    let mut logger = logger::Logger::from(&opts.log);
    logger.init()?;

    if opts.version {
        info!("Version: {0}", get_version());
        return Ok(());
    }

    let app = app::App::new(opts)?;
    app.run().await?;

    Ok(())
}

pub fn get_version() -> String {
    format!("{}-{}", env!("CARGO_PKG_VERSION"), env!("VERGEN_GIT_SHA"))
}
