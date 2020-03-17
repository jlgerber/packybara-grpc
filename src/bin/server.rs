use tokio;

use packybara_grpc::{url_builder, url_builder::UrlBuilder, PackybaraService};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let url = UrlBuilder::new()
        .host(url_builder::Host::Localhost)
        .port(50051)
        .build();
    PackybaraService::run(url).await?;
    Ok(())
}
