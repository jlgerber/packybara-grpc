use super::*;

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

    use super::*;
    pub async fn cmd(
        grpc_client: &mut ClientService,
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
        let response = grpc_client.client.get_version_pins(request).await?;
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
}

pub mod add_versionpins {
    /// Encapsulate the query parameter for adding sites
    pub struct Options {
        pub distribution: String,
        pub levels: Vec<String>,
        pub roles: Vec<String>,
        pub platforms: Vec<String>,
        pub sites: Vec<String>,
        pub author: String,
        pub comment: Option<String>,
    }

    impl Options {
        /// New up an instance of add_sites::Options given a name, order_by
        /// order_direction, and limit
        ///
        /// # Arguments
        ///
        /// * `distribution` - package-version
        /// * `levels` - zero or more levels to create version pin at
        /// * `roles` - zero or more roles to create version pin at
        /// * `platforms` - zero or more platforms to create version pin at
        /// * `author` - name of the person who authored the new sites
        ///
        /// # Returns
        ///
        /// * Option instance
        pub fn new<I>(distribution: I, author: I) -> Self
        where
            I: Into<String>,
        {
            Self {
                distribution: distribution.into(),
                levels: Vec::new(),
                roles: Vec::new(),
                platforms: Vec::new(),
                sites: Vec::new(),
                author: author.into(),
                comment: None,
            }
        }

        pub fn levels<I>(mut self, levels: Vec<I>) -> Self
        where
            I: Into<String>,
        {
            let levels = levels.into_iter().map(|l| l.into()).collect::<Vec<_>>();
            self.levels = levels;
            self
        }

        pub fn roles<I>(mut self, roles: Vec<I>) -> Self
        where
            I: Into<String>,
        {
            let roles = roles.into_iter().map(|l| l.into()).collect::<Vec<_>>();
            self.roles = roles;
            self
        }

        pub fn platforms<I>(mut self, platforms: Vec<I>) -> Self
        where
            I: Into<String>,
        {
            let platforms = platforms.into_iter().map(|l| l.into()).collect::<Vec<_>>();
            self.platforms = platforms;
            self
        }

        pub fn sites<I>(mut self, sites: Vec<I>) -> Self
        where
            I: Into<String>,
        {
            let sites = sites.into_iter().map(|l| l.into()).collect::<Vec<_>>();
            self.sites = sites;
            self
        }
        /// Update comment with option wrapped type implementing
        /// Into<String>
        ///
        /// # Arguments
        ///
        /// * `comment` - The optional comment associated with the commit
        pub fn comment_opt<I>(mut self, comment: Option<I>) -> Self
        where
            I: Into<String>,
        {
            let comment = comment.map(|c| c.into());
            self.comment = comment;
            self
        }
    }

    use super::*;
    use crate::{AddReply, VersionPinsAddRequest};
    pub async fn cmd(
        grpc_client: &mut ClientService,
        options: Options,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        let Options {
            distribution,
            levels,
            roles,
            platforms,
            sites,
            author,
            comment,
        } = options;
        let request = tonic::Request::new(VersionPinsAddRequest {
            distribution,
            levels,
            roles,
            platforms,
            sites,
            author,
            comment,
        });
        let response = grpc_client.client.add_version_pins(request).await?;
        let AddReply { updates } = response.into_inner();

        Ok(updates)
    }
}
