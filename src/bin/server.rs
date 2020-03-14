use tokio;
use tokio_postgres::NoTls;
use tonic::transport::Server;

use packybara_grpc::{PackybaraServer, PackybaraService};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    PackybaraService::run().await?;
    Ok(())
}
