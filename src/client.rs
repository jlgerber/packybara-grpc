use crate::{
    url as grpcurl, Coords, DistributionsQueryReply, DistributionsQueryRequest,
    DistributionsQueryRow, LevelsQueryReply, LevelsQueryRequest, LevelsQueryRow,
    PackagesQueryReply, PackagesQueryRequest, PackagesQueryRow, PackybaraClient,
    PkgCoordsQueryReply, PkgCoordsQueryRequest, PkgCoordsQueryRow, PlatformsQueryReply,
    PlatformsQueryRequest, PlatformsQueryRow, RevisionsQueryReply, RevisionsQueryRequest,
    RevisionsQueryRow, RolesQueryReply, RolesQueryRequest, RolesQueryRow, SitesQueryReply,
    SitesQueryRequest, SitesQueryRow, VersionPinQueryReply, VersionPinQueryRequest,
    VersionPinWithsQueryReply, VersionPinWithsQueryRequest, VersionPinWithsQueryRow,
    VersionPinsQueryReply, VersionPinsQueryRequest, VersionPinsQueryRow, WithsQueryReply,
    WithsQueryRequest, WithsQueryRow,
};
use chrono::{DateTime, FixedOffset, Local, NaiveDateTime, TimeZone, Utc};
use packybara::db::find::versionpins::FindVersionPinsRow;
use packybara::db::find::withs::FindWithsRow;
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
use tonic::transport::{Channel, Endpoint};

// this has some implications for applications that want to communicate
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
}

pub mod get_versionpin {
    /// Encapsulate the query parameters
    pub struct Options {
        pub package: String,
        pub level: Option<String>,
        pub role: Option<String>,
        pub platform: Option<String>,
        pub site: Option<String>,
    }

    impl Options {
        /// New up an instance of VetVersionPinOptions given a package name
        ///
        /// # Arguments
        ///
        /// * `package` - the name of the package
        ///
        /// # Returns
        ///
        /// * GetVersionPinOptions instance
        pub fn new<I>(package: I) -> Self
        where
            I: Into<String>,
        {
            Self {
                package: package.into(),
                level: None,
                role: None,
                platform: None,
                site: None,
            }
        }

        /// Given a mutable instance of Self and an Option wrapped level,
        /// set level and return Self, following the common builder pattern.
        ///
        /// # Arguments
        ///
        /// * `level` - An option wrapped type that implements Into<String>
        ///
        /// # Returns
        ///
        /// * Self
        pub fn level_opt<I>(mut self, level: Option<I>) -> Self
        where
            I: Into<String>,
        {
            self.level = level.map(|x| x.into());
            self
        }

        /// Given a mutable instance of Self and an Option wrapped role,
        /// set role and return Self, following the common builder pattern.
        ///
        /// # Arguments
        ///
        /// * `role` - An option wrapped type that implements Into<String>
        ///
        /// # Returns
        ///
        /// * Self
        pub fn role_opt<I>(mut self, role: Option<I>) -> Self
        where
            I: Into<String>,
        {
            self.role = role.map(|x| x.into());
            self
        }

        /// Given a mutable instance of Self and an Option wrapped platform,
        /// set platform and return Self, following the common builder pattern.
        ///
        /// # Arguments
        ///
        /// * `platform` - An option wrapped type that implements Into<String>
        ///
        /// # Returns
        ///
        /// * Self
        pub fn platform_opt<I>(mut self, platform: Option<I>) -> Self
        where
            I: Into<String>,
        {
            self.platform = platform.map(|x| x.into());
            self
        }

        /// Given a mutable instance of Self and an Option wrapped site,
        /// set site and return Self, following the common builder pattern.
        ///
        /// # Arguments
        ///
        /// * `site` - An option wrapped type that implements Into<String>
        ///
        /// # Returns
        ///
        /// * Self
        pub fn site_opt<I>(mut self, site: Option<I>) -> Self
        where
            I: Into<String>,
        {
            self.site = site.map(|x| x.into());
            self
        }
    }
}

pub mod get_versionpins {
    /// Encapsulate the query parameters
    pub struct Options {
        pub package: Option<String>,
        pub version: Option<String>,
        pub level: Option<String>,
        pub role: Option<String>,
        pub platform: Option<String>,
        pub site: Option<String>,
        pub isolate_facility: Option<bool>,
        pub search_mode: Option<String>,
        pub order_by: Option<String>,
        pub order_direction: Option<String>,
    }

    impl Options {
        /// New up an instance of get_versionpins::Option
        ///
        /// # Arguments
        ///
        /// * `package` - the name of the package
        ///
        /// # Returns
        ///
        /// * GetVersionPinOptions instance
        pub fn new() -> Self {
            Self {
                package: None,
                version: None,
                level: None,
                role: None,
                platform: None,
                site: None,
                isolate_facility: None,
                search_mode: None,
                order_by: None,
                order_direction: None,
            }
        }
        pub fn package_opt<I>(mut self, package: Option<I>) -> Self
        where
            I: Into<String>,
        {
            self.package = package.map(|x| x.into());
            self
        }

        pub fn version_opt<I>(mut self, version: Option<I>) -> Self
        where
            I: Into<String>,
        {
            self.version = version.map(|x| x.into());
            self
        }
        /// Given a mutable instance of Self and an Option wrapped level,
        /// set level and return Self, following the common builder pattern.
        ///
        /// # Arguments
        ///
        /// * `level` - An option wrapped type that implements Into<String>
        ///
        /// # Returns
        ///
        /// * Self
        pub fn level_opt<I>(mut self, level: Option<I>) -> Self
        where
            I: Into<String>,
        {
            self.level = level.map(|x| x.into());
            self
        }

        /// Given a mutable instance of Self and an Option wrapped role,
        /// set role and return Self, following the common builder pattern.
        ///
        /// # Arguments
        ///
        /// * `role` - An option wrapped type that implements Into<String>
        ///
        /// # Returns
        ///
        /// * Self
        pub fn role_opt<I>(mut self, role: Option<I>) -> Self
        where
            I: Into<String>,
        {
            self.role = role.map(|x| x.into());
            self
        }

        /// Given a mutable instance of Self and an Option wrapped platform,
        /// set platform and return Self, following the common builder pattern.
        ///
        /// # Arguments
        ///
        /// * `platform` - An option wrapped type that implements Into<String>
        ///
        /// # Returns
        ///
        /// * Self
        pub fn platform_opt<I>(mut self, platform: Option<I>) -> Self
        where
            I: Into<String>,
        {
            self.platform = platform.map(|x| x.into());
            self
        }

        /// Given a mutable instance of Self and an Option wrapped site,
        /// set site and return Self, following the common builder pattern.
        ///
        /// # Arguments
        ///
        /// * `site` - An option wrapped type that implements Into<String>
        ///
        /// # Returns
        ///
        /// * Self
        pub fn site_opt<I>(mut self, site: Option<I>) -> Self
        where
            I: Into<String>,
        {
            self.site = site.map(|x| x.into());
            self
        }

        pub fn isolate_facility_opt(mut self, isolate: Option<bool>) -> Self {
            self.isolate_facility = isolate;
            self
        }

        pub fn search_mode_opt(mut self, mode: Option<String>) -> Self {
            self.search_mode = mode;
            self
        }

        pub fn order_by_opt(mut self, order_by: Option<String>) -> Self {
            self.order_by = order_by;
            self
        }

        pub fn order_direction_opt(mut self, order_dir: Option<String>) -> Self {
            self.order_direction = order_dir;
            self
        }
    }
}

pub mod get_levels {
    /// Encapsulate the query parameters
    pub struct Options {
        pub level: Option<String>,
        pub show: Option<String>,
        pub depth: Option<u8>,
        pub order_by: Option<String>,
    }

    impl Options {
        /// New up an instance of get_levels::Option
        ///
        /// # Arguments
        ///
        /// * `level` - the name of the level
        /// * `show` - the name of the show
        /// * `depth` - the optional depth
        /// * `order_by` - the optional field to order by
        ///
        /// # Returns
        ///
        /// * Option instance
        pub fn new() -> Self {
            Self {
                level: None,
                show: None,
                depth: None,
                order_by: None,
            }
        }

        pub fn level_opt(mut self, level: Option<String>) -> Self {
            self.level = level;
            self
        }

        pub fn show_opt(mut self, show: Option<String>) -> Self {
            self.show = show;
            self
        }

        pub fn depth_opt(mut self, depth: Option<u8>) -> Self {
            self.depth = depth;
            self
        }

        pub fn order_by_opt(mut self, order_by: Option<String>) -> Self {
            self.order_by = order_by;
            self
        }
    }
}

pub mod get_roles {
    /// Encapsulate the query parameters
    pub struct Options {
        pub role: Option<String>,
        pub category: Option<String>,
        pub order_by: Option<String>,
        pub order_direction: Option<String>,
        pub limit: Option<i32>,
    }

    impl Options {
        /// New up an instance of get_roles::Options given a role, category, order_by
        /// order_direction, and limit
        ///
        /// # Arguments
        ///
        /// * `role` - the optional name of the role
        /// * `category` - the optional name of the category
        /// * `order_by` - the optional field to order by
        /// * `order_direction` - the optional direction to order by
        /// * `limit` - the optional limit
        ///
        /// # Returns
        ///
        /// * Option instance
        pub fn new() -> Self {
            Self {
                role: None,
                category: None,
                order_direction: None,
                order_by: None,
                limit: None,
            }
        }

        pub fn role_opt(mut self, role: Option<String>) -> Self {
            self.role = role;
            self
        }

        pub fn category_opt(mut self, category: Option<String>) -> Self {
            self.category = category;
            self
        }

        pub fn order_direction_opt(mut self, direction: Option<String>) -> Self {
            self.order_direction = direction;
            self
        }

        pub fn order_by_opt(mut self, order_by: Option<String>) -> Self {
            self.order_by = order_by;
            self
        }
        pub fn limit_opt(mut self, limit: Option<i32>) -> Self {
            self.limit = limit;
            self
        }
    }
}

pub mod get_sites {
    /// Encapsulate the query parameters
    pub struct Options {
        pub name: Option<String>,
    }

    impl Options {
        /// New up an instance of get_roles::Options given a role, category, order_by
        /// order_direction, and limit
        ///
        /// # Arguments
        ///
        /// * `role` - the optional name of the role
        /// * `category` - the optional name of the category
        /// * `order_by` - the optional field to order by
        /// * `order_direction` - the optional direction to order by
        /// * `limit` - the optional limit
        ///
        /// # Returns
        ///
        /// * Option instance
        pub fn new() -> Self {
            Self { name: None }
        }

        pub fn name_opt(mut self, name: Option<String>) -> Self {
            self.name = name;
            self
        }
    }
}

pub mod get_platforms {
    /// Encapsulate the query parameters
    pub struct Options {
        pub name: Option<String>,
        pub order_by: Option<String>,
        pub order_direction: Option<String>,
        pub limit: Option<i32>,
    }

    impl Options {
        /// New up an instance of get_platform::Options given a name, order_by
        /// order_direction, and limit
        ///
        /// # Arguments
        ///
        /// * `name` - the optional name of the platform
        /// * `order_by` - the optional field to order by
        /// * `order_direction` - the optional direction to order by
        /// * `limit` - the optional limit
        ///
        /// # Returns
        ///
        /// * Option instance
        pub fn new() -> Self {
            Self {
                name: None,
                order_direction: None,
                order_by: None,
                limit: None,
            }
        }

        pub fn name_opt(mut self, name: Option<String>) -> Self {
            self.name = name;
            self
        }

        pub fn order_direction_opt(mut self, direction: Option<String>) -> Self {
            self.order_direction = direction;
            self
        }

        pub fn order_by_opt(mut self, order_by: Option<String>) -> Self {
            self.order_by = order_by;
            self
        }
        pub fn limit_opt(mut self, limit: Option<i32>) -> Self {
            self.limit = limit;
            self
        }
    }
}

pub mod get_packages {
    /// Encapsulate the query parameters
    pub struct Options {
        pub name: Option<String>,
    }

    impl Options {
        /// New up an instance of get_platform::Options given a name, order_by
        /// order_direction, and limit
        ///
        /// # Arguments
        ///
        /// * `name` - the optional name of the platform
        /// * `order_by` - the optional field to order by
        /// * `order_direction` - the optional direction to order by
        /// * `limit` - the optional limit
        ///
        /// # Returns
        ///
        /// * Option instance
        pub fn new() -> Self {
            Self { name: None }
        }

        pub fn name_opt(mut self, name: Option<String>) -> Self {
            self.name = name;
            self
        }

        // pub fn order_direction_opt(mut self, direction: Option<String>) -> Self {
        //     self.order_direction = direction;
        //     self
        // }

        // pub fn order_by_opt(mut self, order_by: Option<String>) -> Self {
        //     self.order_by = order_by;
        //     self
        // }
        // pub fn limit_opt(mut self, limit: Option<i32>) -> Self {
        //     self.limit = limit;
        //     self
        // }
    }
}

pub mod get_distributions {
    /// Encapsulate the query parameters
    pub struct Options {
        pub package: Option<String>,
        pub version: Option<String>,
        pub order_direction: Option<String>,
    }

    impl Options {
        /// New up an instance of get_platform::Options given a name, order_by
        /// order_direction, and limit
        ///
        /// # Arguments
        ///
        /// * `name` - the optional name of the platform
        /// * `order_by` - the optional field to order by
        /// * `order_direction` - the optional direction to order by
        /// * `limit` - the optional limit
        ///
        /// # Returns
        ///
        /// * Option instance
        pub fn new() -> Self {
            Self {
                package: None,
                version: None,
                order_direction: None,
            }
        }

        pub fn package_opt(mut self, package: Option<String>) -> Self {
            self.package = package;
            self
        }

        pub fn version_opt(mut self, version: Option<String>) -> Self {
            self.version = version;
            self
        }
        pub fn order_direction_opt(mut self, direction: Option<String>) -> Self {
            self.order_direction = direction;
            self
        }

        // pub fn order_by_opt(mut self, order_by: Option<String>) -> Self {
        //     self.order_by = order_by;
        //     self
        // }
        // pub fn limit_opt(mut self, limit: Option<i32>) -> Self {
        //     self.limit = limit;
        //     self
        // }
    }
}

pub mod get_pkgcoords {
    /// Encapsulate the query parameters
    pub struct Options {
        pub package: Option<String>,
        pub level: Option<String>,
        pub role: Option<String>,
        pub platform: Option<String>,
        pub site: Option<String>,
        pub search_mode: Option<String>,
        pub order_by: Option<String>,
    }

    impl Options {
        /// New up an instance of get_pkgcoords::Options given a name, order_by
        /// order_direction, and limit
        ///
        /// # Arguments
        ///
        /// * `name` - the optional name of the platform
        /// * `level` - the optional field to level by
        /// * `role`
        /// * `platform`
        /// * `site`
        /// * `search_mode`
        /// * `order_by` - the optional direction to order by
        ///
        /// # Returns
        ///
        /// * Option instance
        pub fn new() -> Self {
            Self {
                package: None,
                level: None,
                role: None,
                platform: None,
                site: None,
                search_mode: None,
                order_by: None,
            }
        }

        pub fn package_opt(mut self, package: Option<String>) -> Self {
            self.package = package;
            self
        }

        pub fn level_opt(mut self, level: Option<String>) -> Self {
            self.level = level;
            self
        }
        pub fn role_opt(mut self, role: Option<String>) -> Self {
            self.role = role;
            self
        }
        pub fn platform_opt(mut self, platform: Option<String>) -> Self {
            self.platform = platform;
            self
        }
        pub fn site_opt(mut self, site: Option<String>) -> Self {
            self.site = site;
            self
        }
        pub fn search_mode_opt(mut self, search_mode: Option<String>) -> Self {
            self.search_mode = search_mode;
            self
        }
        pub fn order_by_opt(mut self, order_by: Option<String>) -> Self {
            self.order_by = order_by;
            self
        }
    }
}

pub mod get_withs {
    /// Encapsulate the query parameters
    pub struct Options {
        pub package: Option<String>,
        pub level: Option<String>,
        pub role: Option<String>,
        pub platform: Option<String>,
        pub site: Option<String>,
        //pub search_mode: Option<String>,
        pub limit: Option<i32>,
        pub order_by: Option<String>,
        pub order_direction: Option<String>,
    }

    impl Options {
        /// New up an instance of get_pkgcoords::Options given a name, order_by
        /// order_direction, and limit
        ///
        /// # Arguments
        ///
        /// * `name` - the optional name of the platform
        /// * `level` - the optional field to level by
        /// * `role`
        /// * `platform`
        /// * `site`
        // /// * `search_mode`
        /// * `order_by` - the optional direction to order by
        ///
        /// # Returns
        ///
        /// * Option instance
        pub fn new() -> Self {
            Self {
                package: None,
                level: None,
                role: None,
                platform: None,
                site: None,
                //search_mode: None,
                limit: None,
                order_by: None,
                order_direction: None,
            }
        }

        pub fn package_opt(mut self, package: Option<String>) -> Self {
            self.package = package;
            self
        }

        pub fn level_opt(mut self, level: Option<String>) -> Self {
            self.level = level;
            self
        }
        pub fn role_opt(mut self, role: Option<String>) -> Self {
            self.role = role;
            self
        }
        pub fn platform_opt(mut self, platform: Option<String>) -> Self {
            self.platform = platform;
            self
        }
        pub fn site_opt(mut self, site: Option<String>) -> Self {
            self.site = site;
            self
        }
        // pub fn search_mode_opt(mut self, search_mode: Option<String>) -> Self {
        //     self.search_mode = search_mode;
        //     self
        // }
        pub fn limit_opt(mut self, limit: Option<i32>) -> Self {
            self.limit = limit;
            self
        }
        pub fn order_by_opt(mut self, order_by: Option<String>) -> Self {
            self.order_by = order_by;
            self
        }
        pub fn order_direction_opt(mut self, direction: Option<String>) -> Self {
            self.order_direction = direction;
            self
        }
    }
}

pub mod get_revisions {
    /// Encapsulate the query parameters
    pub struct Options {
        pub id: Option<i64>,
        pub transaction_id: Option<i64>,
        pub author: Option<String>,
        pub order_by: Option<String>,
        pub order_direction: Option<String>,
        pub limit: Option<i32>,
    }

    impl Options {
        /// New up an instance of get_pkgcoords::Options given a name, order_by
        /// order_direction, and limit
        ///
        /// # Arguments
        ///
        /// * `name` - the optional name of the platform
        /// * `level` - the optional field to level by
        /// * `role`
        /// * `platform`
        /// * `site`
        // /// * `search_mode`
        /// * `order_by` - the optional direction to order by
        ///
        /// # Returns
        ///
        /// * Option instance
        pub fn new() -> Self {
            Self {
                id: None,
                transaction_id: None,
                author: None,
                order_by: None,
                order_direction: None,
                limit: None,
            }
        }

        pub fn id_opt(mut self, id: Option<i64>) -> Self {
            self.id = id;
            self
        }

        pub fn transaction_id_opt(mut self, transaction_id: Option<i64>) -> Self {
            self.transaction_id = transaction_id;
            self
        }
        pub fn author_opt(mut self, author: Option<String>) -> Self {
            self.author = author;
            self
        }
        pub fn order_by_opt(mut self, order_by: Option<String>) -> Self {
            self.order_by = order_by;
            self
        }
        pub fn order_direction_opt(mut self, direction: Option<String>) -> Self {
            self.order_direction = direction;
            self
        }
        pub fn limit_opt(mut self, limit: Option<i32>) -> Self {
            self.limit = limit;
            self
        }
    }
}
