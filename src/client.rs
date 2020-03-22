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

// NOTE:: this has some implications for applications that want to communicate
// in multiple channels. If this becomes a requirement, we will have to
// put an arc around client
pub struct Client {
    client: PackybaraClient<Channel>,
}

impl Client {
    /// create a new client instance , given a url
    pub async fn new(url: grpcurl::GrpcUrl) -> Result<Self, Box<dyn std::error::Error>> {
        let url = url.as_str().to_string();
        let endpoint = Endpoint::try_from(url)?;
        let client = PackybaraClient::connect(endpoint).await?;
        Ok(Client { client })
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
        let get_versionpin::Options {
            package,
            level,
            role,
            platform,
            site,
        } = options;
        let request = tonic::Request::new(VersionPinQueryRequest {
            package,
            level,
            role,
            platform,
            site,
        });
        let response = self.client.get_version_pin(request).await?;
        let VersionPinQueryReply {
            versionpin_id,
            distribution,
            coords:
                Coords {
                    level,
                    role,
                    platform,
                    site,
                },
            withs,
        } = response.into_inner();

        let withs = if withs.len() > 0 { Some(withs) } else { None };

        let response = FindVersionPinsRow::from_parts(
            versionpin_id as i32,
            distribution.as_str(),
            level.as_str(),
            role.as_str(),
            platform.as_str(),
            &site,
            withs,
        );
        Ok(response)
    }

    pub async fn get_version_pins(
        &mut self,
        options: get_versionpins::Options,
    ) -> Result<Vec<FindAllVersionPinsRow>, Box<dyn std::error::Error>> {
        let get_versionpins::Options {
            package,
            version,
            level,
            role,
            platform,
            site,
            isolate_facility,
            search_mode,
            order_by,
            order_direction,
        } = options;
        let request = tonic::Request::new(VersionPinsQueryRequest {
            package,
            version,
            level,
            role,
            platform,
            site,
            isolate_facility,
            search_mode,
            order_by,
            order_direction,
            limit: None,
        });
        let response = self.client.get_version_pins(request).await?;
        let VersionPinsQueryReply { vpins } = response.into_inner();

        let results = vpins
            .into_iter()
            .map(|vpin| {
                let VersionPinsQueryRow {
                    versionpin_id,
                    distribution_id,
                    pkgcoord_id,
                    distribution,
                    coords:
                        Coords {
                            level,
                            role,
                            platform,
                            site,
                        },
                    withs,
                } = vpin;
                let withs = if withs.len() > 0 { Some(withs) } else { None };
                FindAllVersionPinsRow::from_parts(
                    versionpin_id as i32,
                    distribution_id as i32,
                    pkgcoord_id as i32,
                    &distribution,
                    &level,
                    &role,
                    &platform,
                    &site,
                    withs,
                )
            })
            .collect::<Vec<_>>();

        Ok(results)
    }

    pub async fn get_version_pin_withs(
        &mut self,
        versionpin_id: i64,
    ) -> Result<Vec<FindAllWithsRow>, Box<dyn std::error::Error>> {
        let request = tonic::Request::new(VersionPinWithsQueryRequest { versionpin_id });
        let response = self.client.get_version_pin_withs(request).await?;
        let VersionPinWithsQueryReply { withs } = response.into_inner();
        let withs = withs
            .into_iter()
            .map(|vpin| {
                let VersionPinWithsQueryRow {
                    id,
                    vpin_id,
                    with,
                    order,
                } = vpin;
                //let withs = if withs.len() > 0 { Some(withs) } else { None };
                FindAllWithsRow::from_parts(id as i32, vpin_id as i32, with, order as i32)
            })
            .collect::<Vec<FindAllWithsRow>>();
        Ok(withs)
        //Err("problem")
    }

    pub async fn get_levels(
        &mut self,
        options: get_levels::Options,
    ) -> Result<Vec<FindAllLevelsRow>, Box<dyn std::error::Error>> {
        let get_levels::Options {
            level,
            show,
            depth,
            order_by,
        } = options;
        let request = tonic::Request::new(LevelsQueryRequest {
            level,
            show,
            depth: depth.map(|x| x as u32),
            order_by,
        });
        let response = self.client.get_levels(request).await?;
        let LevelsQueryReply { levels } = response.into_inner();

        let results = levels
            .into_iter()
            .map(|level| {
                let LevelsQueryRow { level, show } = level;
                FindAllLevelsRow::from_parts(&level, &show)
            })
            .collect::<Vec<_>>();

        Ok(results)
    }

    pub async fn get_sites(
        &mut self,
        options: get_sites::Options,
    ) -> Result<Vec<FindAllSitesRow>, Box<dyn std::error::Error>> {
        let get_sites::Options { name } = options;
        let request = tonic::Request::new(SitesQueryRequest { name });
        let response = self.client.get_sites(request).await?;
        let SitesQueryReply { names } = response.into_inner();

        let results = names
            .into_iter()
            .map(|site| {
                let SitesQueryRow { name } = site;
                FindAllSitesRow::from_parts(&name)
            })
            .collect::<Vec<_>>();

        Ok(results)
    }

    pub async fn get_roles(
        &mut self,
        options: get_roles::Options,
    ) -> Result<Vec<FindAllRolesRow>, Box<dyn std::error::Error>> {
        let get_roles::Options {
            role,
            category,
            order_by,
            order_direction,
            limit,
        } = options;
        let request = tonic::Request::new(RolesQueryRequest {
            role,
            category,
            order_by,
            order_direction,
            limit: limit.map(|x| x as i32),
        });
        let response = self.client.get_roles(request).await?;
        let RolesQueryReply { roles } = response.into_inner();

        let results = roles
            .into_iter()
            .map(|role| {
                let RolesQueryRow { role, category } = role;
                FindAllRolesRow::from_parts(&role, &category)
            })
            .collect::<Vec<_>>();

        Ok(results)
    }

    pub async fn get_platforms(
        &mut self,
        options: get_platforms::Options,
    ) -> Result<Vec<FindAllPlatformsRow>, Box<dyn std::error::Error>> {
        let get_platforms::Options {
            name,
            order_by,
            order_direction,
            limit,
        } = options;
        let request = tonic::Request::new(PlatformsQueryRequest {
            name,
            order_by,
            order_direction,
            limit: limit.map(|x| x as i32),
        });
        let response = self.client.get_platforms(request).await?;
        let PlatformsQueryReply { names } = response.into_inner();

        let results = names
            .into_iter()
            .map(|name| {
                let PlatformsQueryRow { name } = name;
                FindAllPlatformsRow::from_parts(&name)
            })
            .collect::<Vec<_>>();

        Ok(results)
    }

    pub async fn get_packages(
        &mut self,
        options: get_packages::Options,
    ) -> Result<Vec<FindAllPackagesRow>, Box<dyn std::error::Error>> {
        let get_packages::Options { name } = options;
        let request = tonic::Request::new(PackagesQueryRequest { name });
        let response = self.client.get_packages(request).await?;
        let PackagesQueryReply { names } = response.into_inner();

        let results = names
            .into_iter()
            .map(|name| {
                let PackagesQueryRow { name } = name;
                FindAllPackagesRow::from_parts(&name)
            })
            .collect::<Vec<_>>();

        Ok(results)
    }

    pub async fn get_distributions(
        &mut self,
        options: get_distributions::Options,
    ) -> Result<Vec<FindAllDistributionsRow>, Box<dyn std::error::Error>> {
        let get_distributions::Options {
            package,
            version,
            order_direction,
        } = options;
        let request = tonic::Request::new(DistributionsQueryRequest {
            package,
            version,
            order_direction,
        });
        let response = self.client.get_distributions(request).await?;
        let DistributionsQueryReply { distributions } = response.into_inner();

        let results = distributions
            .into_iter()
            .map(|name| {
                let DistributionsQueryRow {
                    id,
                    package,
                    version,
                } = name;
                FindAllDistributionsRow::from_parts(id as i32, &package, &version)
            })
            .collect::<Vec<_>>();

        Ok(results)
    }

    pub async fn get_pkgcoords(
        &mut self,
        options: get_pkgcoords::Options,
    ) -> Result<Vec<FindAllPkgCoordsRow>, Box<dyn std::error::Error>> {
        let get_pkgcoords::Options {
            package,
            level,
            role,
            platform,
            site,
            search_mode,
            order_by,
        } = options;
        let request = tonic::Request::new(PkgCoordsQueryRequest {
            package,
            level,
            role,
            platform,
            site,
            search_mode,
            order_by,
        });
        let response = self.client.get_pkg_coords(request).await?;
        let PkgCoordsQueryReply { pkgcoords } = response.into_inner();

        let results = pkgcoords
            .into_iter()
            .map(|coord| {
                let PkgCoordsQueryRow {
                    id,
                    package,
                    level,
                    role,
                    platform,
                    site,
                } = coord;
                FindAllPkgCoordsRow::from_parts(
                    id as i32, &package, &level, &role, &platform, &site,
                )
            })
            .collect::<Vec<_>>();

        Ok(results)
    }

    pub async fn get_withs(
        &mut self,
        options: get_withs::Options,
    ) -> Result<Vec<FindWithsRow>, Box<dyn std::error::Error>> {
        let get_withs::Options {
            package,
            level,
            role,
            platform,
            site,
            //search_mode,
            limit,
            order_by,
            order_direction,
        } = options;
        let package = package.unwrap_or("UNKNOWN".to_string());
        let request = tonic::Request::new(WithsQueryRequest {
            package,
            level,
            role,
            platform,
            site,
            //search_mode,
            limit,
            order_by,
            order_direction,
        });
        let response = self.client.get_withs(request).await?;
        let WithsQueryReply { withs } = response.into_inner();

        let results = withs
            .into_iter()
            .map(|with| {
                let WithsQueryRow {
                    versionpin_id,
                    distribution,
                    coords:
                        Coords {
                            level,
                            role,
                            platform,
                            site,
                        },
                } = with;
                FindWithsRow::from_parts(
                    versionpin_id as i32,
                    &distribution,
                    &level,
                    &role,
                    &platform,
                    &site,
                )
            })
            .collect::<Vec<_>>();

        Ok(results)
    }

    pub async fn get_revisions(
        &mut self,
        options: get_revisions::Options,
    ) -> Result<Vec<FindAllRevisionsRow>, Box<dyn std::error::Error>> {
        let get_revisions::Options {
            id,
            transaction_id,
            author,
            order_by,
            order_direction,
            limit,
        } = options;
        let request = tonic::Request::new(RevisionsQueryRequest {
            id,
            transaction_id,
            author,
            order_by,
            order_direction,
            limit,
        });
        let response = self.client.get_revisions(request).await?;
        let RevisionsQueryReply { revisions } = response.into_inner();

        let results = revisions
            .into_iter()
            .map(|rev| {
                let RevisionsQueryRow {
                    id,
                    transaction_id,
                    author,
                    comment,
                    datetime,
                } = rev;
                //println!("DATETIME {}", datetime);
                //2020-01-25 19:23:26.672258 -08:00
                //let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(61, 0), Utc);
                //let dt: DateTime<Local> = DateTime::from(dt);
                let dt = DateTime::parse_from_str(&datetime, "%F %T%.3f %z")
                    .expect("unable to unwrap time");
                let dt: DateTime<Local> = DateTime::from(dt);
                FindAllRevisionsRow::from_parts(
                    id as i32,
                    transaction_id,
                    &author,
                    dt,
                    //&datetime,
                    &comment,
                )
            })
            .collect::<Vec<_>>();

        Ok(results)
    }

    pub async fn get_changes(
        &mut self,
        options: get_changes::Options,
    ) -> Result<Vec<FindAllChangesRow>, Box<dyn std::error::Error>> {
        let get_changes::Options { transaction_id } = options;
        let request = tonic::Request::new(ChangesQueryRequest { transaction_id });
        let response = self.client.get_changes(request).await?;
        let ChangesQueryReply { changes } = response.into_inner();

        let results = changes
            .into_iter()
            .map(|chng| {
                let ChangesQueryRow {
                    id,
                    transaction_id,
                    action,
                    coords:
                        Coords {
                            level,
                            role,
                            platform,
                            site,
                        },
                    package,
                    old,
                    new,
                } = chng;
                let change_action = ChangeAction::from_str(&action)
                    .expect("could not convert to action to ChangeAction");
                let old = old.unwrap_or("".to_string());
                //let new = Distribution::new(new);
                let action = change_action.to_string();

                FindAllChangesRow::from_parts(
                    id,
                    transaction_id,
                    &action,
                    &level,
                    &role,
                    &platform,
                    &site,
                    &package,
                    &old,
                    &new,
                )
            })
            .collect::<Vec<_>>();

        Ok(results)
    }
}
