use env_logger;
use env_logger::Env;
use packybara_grpc::{url_builder, url_builder::UrlBuilder, DatabaseConfig, PackybaraService};
use std::env;
use structopt::StructOpt;
use tokio;

#[derive(StructOpt, Debug, PartialEq)]
pub struct GrpcServerArgs {
    /// Set the log level. This may target one or more
    /// specific modules or be general.
    /// (levels: trace, debug, info, warn, error)
    #[structopt(long)]
    pub loglevel: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = GrpcServerArgs::from_args();
    if let GrpcServerArgs {
        loglevel: Some(ref level),
        ..
    } = opt
    {
        env::set_var("RUST_LOG", level);
    }
    env_logger::from_env(Env::default().default_filter_or("warn")).init();
    let dbconfig = DatabaseConfig::new();
    let url = UrlBuilder::new()
        .host(url_builder::Host::Localhost)
        .port(50051)
        .build();
    PackybaraService::run(url, dbconfig).await?;
    Ok(())
}
