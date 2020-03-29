//! `versionpins` exports `get_versionpins::Options`, used to provide query parameters to
//! the `ClientService.get_versionpins` method, `add_versionpins::Options`, used to provide
//! parameters to the `ClientService.add_versionpins` method, and `set_versionpins::Options` used to
//! provide parameters to the `ClientService.set_versionpins` method.
use super::*;

/// Exports `Options` struct, used to pass query parameters to `ClientService.get_versionpins` method
pub mod get_versionpins {
    /// Encapsulate the query parameters for `ClientService.get_versionpins`
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
        /// # Example
        /// ```
        /// # fn dox() -> std::io::Result<()> {
        /// use packybara_grpc::get_versionpins;
        /// let options = get_versionpins::Options::new();
        ///                              
        /// # Ok(())
        /// # }
        /// ```
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
        /// set package option on `Options` instance, following builder pattern.
        ///
        /// # Example
        /// ```
        /// # fn dox() -> std::io::Result<()> {
        /// use packybara_grpc::get_versionpins;
        /// let options = get_versionpins::Options::new()
        ///                                .package_opt(Some("maya"));
        ///                              
        /// # Ok(())
        /// # }
        /// ```
        pub fn package_opt<I>(mut self, package: Option<I>) -> Self
        where
            I: Into<String>,
        {
            self.package = package.map(|x| x.into());
            self
        }
        /// Set version option on `Option` instance, following owned builder
        /// pattern
        ///
        /// # Example
        /// ```
        /// # fn dox() -> std::io::Result<()> {
        /// use packybara_grpc::get_versionpins;
        /// let options = get_versionpins::Options::new()
        ///                                .package_opt(Some("maya"))
        ///                                .versoin_opt(Some("2018.3.4"));
        ///                              
        /// # Ok(())
        /// # }
        /// ```
        pub fn version_opt<I>(mut self, version: Option<I>) -> Self
        where
            I: Into<String>,
        {
            self.version = version.map(|x| x.into());
            self
        }
        /// set level option on `Option` instance, following owned builder pattern
        ///
        /// # Example
        /// ```
        /// # fn dox() -> std::io::Result<()> {
        /// use packybara_grpc::get_versionpins;
        /// let options = get_versionpins::Options::new()
        ///                                .package_opt(Some("maya"))
        ///                                .level_opt("dev01");
        ///                              
        /// # Ok(())
        /// # }
        /// ```
        pub fn level_opt<I>(mut self, level: Option<I>) -> Self
        where
            I: Into<String>,
        {
            self.level = level.map(|x| x.into());
            self
        }

        /// set role option on `Option` instance, following owned builder pattern
        ///
        /// # Example
        /// ```
        /// # fn dox() -> std::io::Result<()> {
        /// use packybara_grpc::get_versionpins;
        /// let options = get_versionpins::Options::new()
        ///                                .package_opt(Some("maya"))
        ///                                .level_opt("dev01")
        ///                                .role_opt("model");
        ///                              
        /// # Ok(())
        /// # }
        /// ```
        pub fn role_opt<I>(mut self, role: Option<I>) -> Self
        where
            I: Into<String>,
        {
            self.role = role.map(|x| x.into());
            self
        }

        /// set platform option on `Option` instance, following owned builder pattern
        ///
        /// # Example
        /// ```
        /// # fn dox() -> std::io::Result<()> {
        /// use packybara_grpc::get_versionpins;
        /// let options = get_versionpins::Options::new()
        ///                                .package_opt(Some("maya"))
        ///                                .level_opt("dev01")
        ///                                .platform_opt("cent7_64");
        ///                              
        /// # Ok(())
        /// # }
        /// ```
        pub fn platform_opt<I>(mut self, platform: Option<I>) -> Self
        where
            I: Into<String>,
        {
            self.platform = platform.map(|x| x.into());
            self
        }

        /// set site option on `Option` instance, following owned builder pattern
        ///
        /// # Example
        /// ```
        /// # fn dox() -> std::io::Result<()> {
        /// use packybara_grpc::get_versionpins;
        /// let options = get_versionpins::Options::new()
        ///                                .package_opt(Some("maya"))
        ///                                .level_opt("dev01")
        ///                                .site_opt("portland");
        ///                              
        /// # Ok(())
        /// # }
        /// ```
        pub fn site_opt<I>(mut self, site: Option<I>) -> Self
        where
            I: Into<String>,
        {
            self.site = site.map(|x| x.into());
            self
        }
        /// set is_isolate_facility option on `Option` instance, following owned builder pattern.
        /// This option affects the query, limiting the extent to the facility regardless of the
        /// direction, if the query is rooted in the facility
        ///
        /// # Example
        /// ```
        /// # fn dox() -> std::io::Result<()> {
        /// use packybara_grpc::get_versionpins;
        /// let options = get_versionpins::Options::new()
        ///                                .package_opt(Some("maya"))
        ///                                .level_opt("dev01")
        ///                                .platform_opt("cent7_64")
        ///                                .isolate_facility_opt(Some(true));
        ///                              
        /// # Ok(())
        /// # }
        /// ```
        pub fn isolate_facility_opt(mut self, isolate: Option<bool>) -> Self {
            self.isolate_facility = isolate;
            self
        }
        /// set search mode option on `Option` instance, following owned builder pattern
        ///
        /// # Example
        /// ```
        /// # fn dox() -> std::io::Result<()> {
        /// use packybara_grpc::get_versionpins;
        /// let options = get_versionpins::Options::new()
        ///                                .package_opt(Some("maya"))
        ///                                .level_opt("dev01")
        ///                                .search_mode_opt("ancestor");
        ///                              
        /// # Ok(())
        /// # }
        /// ```
        pub fn search_mode_opt(mut self, mode: Option<String>) -> Self {
            self.search_mode = mode;
            self
        }
        /// Optionally set the field to order the query results by on the `Option` instance,
        /// following owned builder pattern
        ///
        /// # Example
        /// ```
        /// # fn dox() -> std::io::Result<()> {
        /// use packybara_grpc::get_versionpins;
        /// let options = get_versionpins::Options::new()
        ///                                .package_opt(Some("maya"))
        ///                                .level_opt("dev01")
        ///                                .porder_on_opt("package");
        ///                              
        /// # Ok(())
        /// # }
        /// ```
        pub fn order_by_opt(mut self, order_by: Option<String>) -> Self {
            self.order_by = order_by;
            self
        }
        /// set order_direction option on `Option` instance, following owned builder pattern
        ///
        /// # Example
        /// ```
        /// # fn dox() -> std::io::Result<()> {
        /// use packybara_grpc::get_versionpins;
        /// let options = get_versionpins::Options::new()
        ///                                .package_opt(Some("maya"))
        ///                                .level_opt("dev01")
        ///                                .order_direction_opt("asc");
        ///                              
        /// # Ok(())
        /// # }
        /// ```
        pub fn order_direction_opt(mut self, order_dir: Option<String>) -> Self {
            self.order_direction = order_dir;
            self
        }
    }
}
pub(crate) mod get_versionpins_impl {
    use super::*;
    pub(crate) async fn cmd(
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

/// Exports `Options` struct which provides parameters to the `ClientService.add_versionpins`
/// method.
pub mod add_versionpins {
    /// Encapsulate the parameters for adding sites
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
        /// New up an instance of add_sites::Options a distribution and author
        ///
        /// # Example
        /// ```
        /// # fn dox() -> std::io::Result<()> {
        /// use packybara_grpc::add_versionpins;
        /// let options = add_versionpins::Options::new("wam-2.1.2", "jgerber");
        ///                              
        /// # Ok(())
        /// # }
        /// ```
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
        /// Set one or more levels in which to set versionpins
        ///
        /// # Example
        /// ```
        /// # fn dox() -> std::io::Result<()> {
        /// use packybara_grpc::add_versionpins;
        /// let options = add_versionpins::Options::new("wam-2.3.1", "jgerber")
        ///                                 .levels(vec!["dev01","plasma"]);
        ///                              
        /// # Ok(())
        /// # }
        /// ```
        pub fn levels<I>(mut self, levels: Vec<I>) -> Self
        where
            I: Into<String>,
        {
            let levels = levels.into_iter().map(|l| l.into()).collect::<Vec<_>>();
            self.levels = levels;
            self
        }
        /// Set one or more roles in which to set versionpins
        ///
        /// # Example
        /// ```
        /// # fn dox() -> std::io::Result<()> {
        /// use packybara_grpc::add_versionpins;
        /// let options = add_versionpins::Options::new("wam-2.3.1", "jgerber")
        ///                                 .levels(vec!["dev01","plasma"])
        ///                                 .roles(vec!["model"]);
        ///                              
        /// # Ok(())
        /// # }
        /// ```
        pub fn roles<I>(mut self, roles: Vec<I>) -> Self
        where
            I: Into<String>,
        {
            let roles = roles.into_iter().map(|l| l.into()).collect::<Vec<_>>();
            self.roles = roles;
            self
        }

        /// Set one or more platforms in which to set versionpins
        ///
        /// # Example
        /// ```
        /// # fn dox() -> std::io::Result<()> {
        /// use packybara_grpc::add_versionpins;
        /// let options = add_versionpins::Options::new("wam-2.3.1", "jgerber")
        ///                                 .levels(vec!["dev01","plasma"])
        ///                                 .roles(vec!["model"])
        ///                                 .platforms(vec!["cent6_64", "cent7_64"]);
        ///                              
        /// # Ok(())
        /// # }
        /// ```
        pub fn platforms<I>(mut self, platforms: Vec<I>) -> Self
        where
            I: Into<String>,
        {
            let platforms = platforms.into_iter().map(|l| l.into()).collect::<Vec<_>>();
            self.platforms = platforms;
            self
        }
        /// Set one or more sites in which to set versionpins
        ///
        /// # Example
        ///
        /// ```
        /// # fn dox() -> std::io::Result<()> {
        /// use packybara_grpc::add_versionpins;
        /// let options = add_versionpins::Options::new("wam-2.3.1", "jgerber")
        ///                                 .levels(vec!["dev01","plasma"])
        ///                                 .roles(vec!["model"])
        ///                                 .platforms(vec!["cent6_64", "cent7_64"])
        ///                                 .sites(vec!["portlan", "hyderabad", "playa"]);
        ///                              
        /// # Ok(())
        /// # }
        /// ```
        pub fn sites<I>(mut self, sites: Vec<I>) -> Self
        where
            I: Into<String>,
        {
            let sites = sites.into_iter().map(|l| l.into()).collect::<Vec<_>>();
            self.sites = sites;
            self
        }
        /// Optionally set a comment when adding versionpins
        ///
        /// # Example
        /// ```
        /// # fn dox() -> std::io::Result<()> {
        /// use packybara_grpc::add_versionpins;
        /// let options = add_versionpins::Options::new("wam-2.3.1", "jgerber")
        ///                                 .levels(vec!["dev01","plasma"])
        ///                                 .roles(vec!["model"])
        ///                                 .platforms(vec!["cent6_64", "cent7_64"])
        ///                                 .sites(vec!["portlan", "hyderabad", "playa"])
        ///                                 .comment_opt(Some("creating a versionpin for whatever"));
        ///                              
        /// # Ok(())
        /// # }
        /// ```
        pub fn comment_opt<I>(mut self, comment: Option<I>) -> Self
        where
            I: Into<String>,
        {
            let comment = comment.map(|c| c.into());
            self.comment = comment;
            self
        }
    }
}
pub(crate) mod add_versionpins_impl {
    use super::*;
    use crate::{AddReply, VersionPinsAddRequest};
    pub(crate) async fn cmd(
        grpc_client: &mut ClientService,
        options: add_versionpins::Options,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        let add_versionpins::Options {
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

/// Exports `Options` struct which provides parameters to the `ClientService.set_versionpins`
/// method.
pub mod set_versionpins {
    /// Encapsulate the query parameters for updating sites
    pub struct Options {
        pub vpin_ids: Vec<i64>,
        pub dist_ids: Vec<i64>,
        pub author: String,
        pub comment: String,
    }

    impl Options {
        /// New up an instance of set_versionpins::Options given a vector of versionpin ids,
        /// a vector of distributions ids, and an author and comment
        ///
        /// # Example
        /// ```
        /// # fn dox() -> std::io::Result<()> {
        /// use packybara_grpc::set_versionpins;
        /// let options = set_versionpins::Options::new( vec![12345], vec!["141556"], "jgerber", "stuff and things");
        /// # Ok(())
        /// # }
        /// ```
        pub fn new<I>(vpin_ids: Vec<i64>, dist_ids: Vec<i64>, author: I, comment: I) -> Self
        where
            I: Into<String>,
        {
            Self {
                vpin_ids,
                dist_ids,
                author: author.into(),
                comment: comment.into(),
            }
        }
    }
}
pub(crate) mod set_versionpins_impl {
    use super::*;
    use crate::{VersionPinsSetReply, VersionPinsSetRequest};
    pub(crate) async fn cmd(
        grpc_client: &mut ClientService,
        options: set_versionpins::Options,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let set_versionpins::Options {
            vpin_ids,
            dist_ids,
            author,
            comment,
        } = options;
        let request = tonic::Request::new(VersionPinsSetRequest {
            vpin_ids,
            dist_ids,
            author,
            comment: Some(comment),
        });
        let response = grpc_client.client.set_version_pins(request).await?;
        let VersionPinsSetReply { result } = response.into_inner();

        Ok(result)
    }
}
