use log;
use packybara::coords::Coords as PCoords;
use packybara::db::find::versionpins::FindVersionPinsRow;
use packybara::db::find_all::levels::FindAllLevelsRow;
use packybara::db::find_all::versionpin_withs::FindAllWithsRow;
use packybara::db::find_all::versionpins::FindAllVersionPinsRow;
use packybara::db::traits::*;
use packybara::packrat::{Client, PackratDb};
use packybara::LtreeSearchMode;
use packybara::{OrderDirection, SearchAttribute};
use std::str::FromStr;
use tokio_postgres::NoTls;
use tonic::transport::Server;
use tonic::{Code, Request, Response, Status};

use crate::{
    url::GrpcUrl, Coords, LevelsQueryReply, LevelsQueryRequest, LevelsQueryRow, Packybara,
    PackybaraServer, VersionPinQueryReply, VersionPinQueryRequest, VersionPinWithsQueryReply,
    VersionPinWithsQueryRequest, VersionPinWithsQueryRow, VersionPinsQueryReply,
    VersionPinsQueryRequest, VersionPinsQueryRow,
};

mod get_levels;
mod get_version_pin;
mod get_version_pin_withs;
mod get_version_pins;

#[derive(Debug)]
pub struct PackybaraService {
    client: Client,
}

impl PackybaraService {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
    // TODO:: Add configuration as run argument
    /// Run the server as a service.
    ///
    /// # Examples
    /// ```
    /// use tokio;
    /// use packybara_grpc::PackybaraService;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     PackybaraService::run().await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn run(url: GrpcUrl) -> Result<(), Box<dyn std::error::Error>> {
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
        let addr = url.to_socket_addr()?; //"[::1]:50051".parse()?;
        let packy = PackybaraService::new(client);
        Server::builder()
            .add_service(PackybaraServer::new(packy))
            .serve(addr)
            .await?;

        Ok(())
    }

    pub fn client(&self) -> &Client {
        &self.client
    }
}

#[tonic::async_trait]
impl Packybara for PackybaraService {
    /// Regrieve the version pin, given a VersionPinQueryRequest struct
    async fn get_version_pin(
        &self,
        request: Request<VersionPinQueryRequest>,
    ) -> Result<Response<VersionPinQueryReply>, Status> {
        get_version_pin::get_version_pin(&self, request).await
    }

    async fn get_version_pins(
        &self,
        request: Request<VersionPinsQueryRequest>,
    ) -> Result<Response<VersionPinsQueryReply>, Status> {
        get_version_pins::get_version_pins(&self, request).await
    }

    async fn get_version_pin_withs(
        &self,
        request: Request<VersionPinWithsQueryRequest>,
    ) -> Result<Response<VersionPinWithsQueryReply>, Status> {
        get_version_pin_withs::get_version_pin_withs(&self, request).await
        //Err(Status::new(Code::Internal, "not implemented"))
    }
    async fn get_levels(
        &self,
        request: Request<LevelsQueryRequest>,
    ) -> Result<Response<LevelsQueryReply>, Status> {
        get_levels::get_levels(&self, request).await
        //Err(Status::new(Code::Internal, "not implemented"))
    }
}
