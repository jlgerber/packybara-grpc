use crate::{
    url as grpcurl, ChangesQueryReply, ChangesQueryRequest, ChangesQueryRow, Coords,
    DistributionsQueryReply, DistributionsQueryRequest, DistributionsQueryRow, LevelsQueryReply,
    LevelsQueryRequest, LevelsQueryRow, PackagesQueryReply, PackagesQueryRequest, PackagesQueryRow,
    PackybaraClient, PkgCoordsQueryReply, PkgCoordsQueryRequest, PkgCoordsQueryRow,
    PlatformsQueryReply, PlatformsQueryRequest, PlatformsQueryRow, RevisionsQueryReply,
    RevisionsQueryRequest, RevisionsQueryRow, RolesQueryReply, RolesQueryRequest, RolesQueryRow,
    SitesQueryReply, SitesQueryRequest, SitesQueryRow, VersionPinQueryReply,
    VersionPinQueryRequest, VersionPinWithsQueryReply, VersionPinWithsQueryRequest,
    VersionPinWithsQueryRow, VersionPinsQueryReply, VersionPinsQueryRequest, VersionPinsQueryRow,
    WithsQueryReply, WithsQueryRequest, WithsQueryRow,
};
use chrono::{DateTime, Local};
use packybara::db::find::versionpins::FindVersionPinsRow;
use packybara::db::find::withs::FindWithsRow;
use packybara::db::find_all::changes::ChangeAction;
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
use std::convert::TryFrom;
use std::str::FromStr;
use tonic::transport::{Channel, Endpoint};

pub mod changes;
pub use changes::*;

pub mod revisions;
pub use revisions::*;

pub mod withs;
pub use withs::*;

pub mod pkgcoords;
pub use pkgcoords::*;

pub mod distributions;
pub use distributions::*;

pub mod packages;
pub use packages::*;

pub mod platforms;
pub use platforms::*;

pub mod sites;
pub use sites::*;

pub mod roles;
pub use roles::*;

pub mod levels;
pub use levels::*;

pub mod versionpins;
pub use versionpins::*;

pub mod versionpin;
pub use versionpin::*;

pub mod versionpin_withs;
pub use versionpin_withs::*;

// NOTE:: this has some implications for applications that want to communicate
// in multiple channels. If this becomes a requirement, we will have to
// put an arc around client
pub struct ClientService {
    client: PackybaraClient<Channel>,
}

impl ClientService {
    /// create a new client instance , given a url
    pub async fn new(url: grpcurl::GrpcUrl) -> Result<Self, Box<dyn std::error::Error>> {
        let url = url.as_str().to_string();
        let endpoint = Endpoint::try_from(url)?;
        let client = PackybaraClient::connect(endpoint).await?;
        Ok(ClientService { client })
    }

    pub fn client(&self) -> &PackybaraClient<Channel> {
        &self.client
    }

    pub fn client_mut(&mut self) -> &mut PackybaraClient<Channel> {
        &mut self.client
    }
    /// Retrieve versionpin from server, given GetVersionPinOptions instance
    ///
    /// # Arguments
    ///
    /// * `options` - get_versionpin::Options instance, encapsulating the query parameters
    ///   
    /// # Returns
    ///
    /// * Result
    /// - Ok - FindVersionPinsRow
    /// - Err - Boxed std::error::Error
    ///
    /// # Example
    ///
    /// ```ignore
    /// let results = client.get_version_in(GetVersionPinOptions::new("maya").role("model")).await?;
    /// ```
    pub async fn get_version_pin(
        &mut self,
        options: get_versionpin::Options,
    ) -> Result<FindVersionPinsRow, Box<dyn std::error::Error>> {
        get_versionpin::cmd(self, options).await
    }

    pub async fn get_version_pins(
        &mut self,
        options: get_versionpins::Options,
    ) -> Result<Vec<FindAllVersionPinsRow>, Box<dyn std::error::Error>> {
        get_versionpins::cmd(self, options).await
    }

    pub async fn get_version_pin_withs(
        &mut self,
        versionpin_id: i64,
    ) -> Result<Vec<FindAllWithsRow>, Box<dyn std::error::Error>> {
        get_versionpin_withs::cmd(self, versionpin_id).await
    }

    pub async fn get_levels(
        &mut self,
        options: get_levels::Options,
    ) -> Result<Vec<FindAllLevelsRow>, Box<dyn std::error::Error>> {
        get_levels::cmd(self, options).await
    }

    pub async fn get_sites(
        &mut self,
        options: get_sites::Options,
    ) -> Result<Vec<FindAllSitesRow>, Box<dyn std::error::Error>> {
        get_sites::cmd(self, options).await
    }

    pub async fn get_roles(
        &mut self,
        options: get_roles::Options,
    ) -> Result<Vec<FindAllRolesRow>, Box<dyn std::error::Error>> {
        get_roles::cmd(self, options).await
    }

    pub async fn get_platforms(
        &mut self,
        options: get_platforms::Options,
    ) -> Result<Vec<FindAllPlatformsRow>, Box<dyn std::error::Error>> {
        get_platforms::cmd(self, options).await
    }

    pub async fn get_packages(
        &mut self,
        options: get_packages::Options,
    ) -> Result<Vec<FindAllPackagesRow>, Box<dyn std::error::Error>> {
        get_packages::cmd(self, options).await
    }

    pub async fn get_distributions(
        &mut self,
        options: get_distributions::Options,
    ) -> Result<Vec<FindAllDistributionsRow>, Box<dyn std::error::Error>> {
        get_distributions::cmd(self, options).await
    }

    pub async fn get_pkgcoords(
        &mut self,
        options: get_pkgcoords::Options,
    ) -> Result<Vec<FindAllPkgCoordsRow>, Box<dyn std::error::Error>> {
        get_pkgcoords::cmd(self, options).await
    }

    pub async fn get_withs(
        &mut self,
        options: get_withs::Options,
    ) -> Result<Vec<FindWithsRow>, Box<dyn std::error::Error>> {
        get_withs::cmd(self, options).await
    }

    pub async fn get_revisions(
        &mut self,
        options: get_revisions::Options,
    ) -> Result<Vec<FindAllRevisionsRow>, Box<dyn std::error::Error>> {
        get_revisions::cmd(self, options).await
    }

    pub async fn get_changes(
        &mut self,
        options: get_changes::Options,
    ) -> Result<Vec<FindAllChangesRow>, Box<dyn std::error::Error>> {
        get_changes::cmd(self, options).await
    }

    //-----------------------------
    //            ADD

    pub async fn add_packages(
        &mut self,
        options: add_packages::Options,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        add_packages::cmd(self, options).await
    }
}
