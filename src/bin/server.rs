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
    #[structopt(long, display_order = 1)]
    pub loglevel: Option<String>,
    /// Specify the host database
    #[structopt(long, display_order = 2)]
    pub host: Option<String>,
    /// Specify the database port to connect to
    #[structopt(short, long, display_order = 3)]
    pub port: Option<u16>,
    /// Specify the database user
    #[structopt(short, long, display_order = 4)]
    pub user: Option<String>,
    /// Specify the password
    #[structopt(long, display_order = 5)]
    pub password: Option<String>,
    /// Specify the number of procs in the connection pool
    #[structopt(long, display_order = 6)]
    pub procs: Option<u16>,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = GrpcServerArgs::from_args();
    let GrpcServerArgs {
        loglevel,
        host,
        port,
        user,
        password,
        procs,
    } = opt;

    if let Some(ref level) = loglevel {
        env::set_var("RUST_LOG", level);
    }

    env_logger::from_env(Env::default().default_filter_or("warn")).init();
    let dbconfig = DatabaseConfig::new()
        .host_opt(host)
        .port_opt(port)
        .user_opt(user)
        .password_opt(password)
        .pool_procs_opt(procs);
    let url = UrlBuilder::new()
        .host(url_builder::Host::Localhost)
        .port(50051)
        .build();
    PackybaraService::run(url, dbconfig).await?;
    Ok(())
}
