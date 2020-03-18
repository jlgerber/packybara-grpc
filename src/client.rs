use crate::{
    url as grpcurl, Coords, LevelsQueryReply, LevelsQueryRequest, LevelsQueryRow, PackybaraClient,
    VersionPinQueryReply, VersionPinQueryRequest, VersionPinWithsQueryReply,
    VersionPinWithsQueryRequest, VersionPinWithsQueryRow, VersionPinsQueryReply,
    VersionPinsQueryRequest, VersionPinsQueryRow,
};
use packybara::db::find::versionpins::FindVersionPinsRow;
use packybara::db::find_all::levels::FindAllLevelsRow;
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
        /// New up an instance of VetVersionPinOptions given a package name
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
        /// New up an instance of VetVersionPinOptions given a package name
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
