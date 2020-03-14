use tokio;
use tokio_postgres::NoTls;
use tonic::transport::Server;

use packybara_grpc::{PackybaraServer, PackybaraService};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let (client, connection) = tokio_postgres::connect(
        "host=127.0.0.1 user=postgres  dbname=packrat password=example port=5432",
        NoTls,
    )
    .await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    let addr = "[::1]:50051".parse()?;
    let packy = PackybaraService::new(client);

    Server::builder()
        .add_service(PackybaraServer::new(packy))
        .serve(addr)
        .await?;

    Ok(())
}
