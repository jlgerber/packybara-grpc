use pb::packybara_client::PackybaraClient;
use pb::VersionPinQueryRequest;

pub mod pb {
    tonic::include_proto!("packybara");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = PackybaraClient::connect("http://[::1]:50051").await?;

    let request = tonic::Request::new(VersionPinQueryRequest {
        package: "maya".into(),
        level: None,
        role: None,
        platform: None,
        site: None,
    });

    let response = client.get_version_pin(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}
