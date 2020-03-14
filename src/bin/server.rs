use tokio;

use packybara_grpc::PackybaraService;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    PackybaraService::run().await?;
    Ok(())
}
