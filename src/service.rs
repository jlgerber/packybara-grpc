use log;
use packybara::coords::Coords as PCoords;
use packybara::db::find::versionpins::FindVersionPinsRow;
use packybara::db::find::withs::FindWithsRow;
use packybara::db::find_all::changes::FindAllChangesRow;
use packybara::db::find_all::distributions::FindAllDistributionsRow;
use packybara::db::find_all::levels::FindAllLevelsRow;
use packybara::db::find_all::packages::FindAllPackagesRow;
use packybara::db::find_all::pkgcoords::FindAllPkgCoordsRow;
use packybara::db::find_all::platforms::FindAllPlatformsRow;
use packybara::db::find_all::revisions::FindAllRevisionsRow;
use packybara::db::find_all::roles::FindAllRolesRow;
use packybara::db::find_all::sites::FindAllSitesRow;
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
    url::GrpcUrl, ChangesQueryReply, ChangesQueryRequest, ChangesQueryRow, Coords,
    DistributionsQueryReply, DistributionsQueryRequest, DistributionsQueryRow, LevelsQueryReply,
    LevelsQueryRequest, LevelsQueryRow, PackagesQueryReply, PackagesQueryRequest, PackagesQueryRow,
    Packybara, PackybaraServer, PkgCoordsQueryReply, PkgCoordsQueryRequest, PkgCoordsQueryRow,
    PlatformsQueryReply, PlatformsQueryRequest, PlatformsQueryRow, RevisionsQueryReply,
    RevisionsQueryRequest, RevisionsQueryRow, RolesQueryReply, RolesQueryRequest, RolesQueryRow,
    SitesQueryReply, SitesQueryRequest, SitesQueryRow, VersionPinQueryReply,
    VersionPinQueryRequest, VersionPinWithsQueryReply, VersionPinWithsQueryRequest,
    VersionPinWithsQueryRow, VersionPinsQueryReply, VersionPinsQueryRequest, VersionPinsQueryRow,
    WithsQueryReply, WithsQueryRequest, WithsQueryRow,
};

mod changes;
mod distributions;
mod levels;
mod packages;
mod pkgcoords;
mod platforms;
mod revisions;
mod roles;
mod sites;
mod version_pin;
mod version_pin_withs;
mod version_pins;
mod withs;

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
        version_pin::get_version_pin(&self, request).await
    }

    async fn get_version_pins(
        &self,
        request: Request<VersionPinsQueryRequest>,
    ) -> Result<Response<VersionPinsQueryReply>, Status> {
        version_pins::get_version_pins(&self, request).await
    }

    async fn get_version_pin_withs(
        &self,
        request: Request<VersionPinWithsQueryRequest>,
    ) -> Result<Response<VersionPinWithsQueryReply>, Status> {
        version_pin_withs::get_version_pin_withs(&self, request).await
        //Err(Status::new(Code::Internal, "not implemented"))
    }
    async fn get_levels(
        &self,
        request: Request<LevelsQueryRequest>,
    ) -> Result<Response<LevelsQueryReply>, Status> {
        levels::get_levels(&self, request).await
        //Err(Status::new(Code::Internal, "not implemented"))
    }
    async fn get_roles(
        &self,
        request: Request<RolesQueryRequest>,
    ) -> Result<Response<RolesQueryReply>, Status> {
        roles::get_roles(&self, request).await
        //Err(Status::new(Code::Internal, "not implemented"))
    }
    async fn get_sites(
        &self,
        request: Request<SitesQueryRequest>,
    ) -> Result<Response<SitesQueryReply>, Status> {
        sites::get_sites(&self, request).await
        //Err(Status::new(Code::Internal, "not implemented"))
    }
    async fn get_platforms(
        &self,
        request: Request<PlatformsQueryRequest>,
    ) -> Result<Response<PlatformsQueryReply>, Status> {
        platforms::get_platforms(&self, request).await
        //Err(Status::new(Code::Internal, "not implemented"))
    }
    async fn get_packages(
        &self,
        request: Request<PackagesQueryRequest>,
    ) -> Result<Response<PackagesQueryReply>, Status> {
        packages::get_packages(&self, request).await
    }

    async fn get_distributions(
        &self,
        request: Request<DistributionsQueryRequest>,
    ) -> Result<Response<DistributionsQueryReply>, Status> {
        distributions::get_distributions(&self, request).await
    }

    async fn get_pkg_coords(
        &self,
        request: Request<PkgCoordsQueryRequest>,
    ) -> Result<Response<PkgCoordsQueryReply>, Status> {
        pkgcoords::get_pkgcoords(&self, request).await
    }

    async fn get_withs(
        &self,
        request: Request<WithsQueryRequest>,
    ) -> Result<Response<WithsQueryReply>, Status> {
        withs::get_withs(&self, request).await
    }

    async fn get_revisions(
        &self,
        request: Request<RevisionsQueryRequest>,
    ) -> Result<Response<RevisionsQueryReply>, Status> {
        revisions::get_revisions(&self, request).await
    }

    async fn get_changes(
        &self,
        request: Request<ChangesQueryRequest>,
    ) -> Result<Response<ChangesQueryReply>, Status> {
        changes::get_changes(&self, request).await
    }
}
