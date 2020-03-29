//! `pkgcoords` module exports `get_pkgcoords::Options`, used to set query options for `ClientService.get_pkgcoords`,
//! and `add_pkgcoords::Options` used to set parameters for `ClientService.add_pkgcoords`.
use super::*;

/// Exports `Options`, used to set query parameters for `ClientService.get_pkgcoords`
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
        /// New up an instance of get_pkgcoords::Options
        ///
        /// # Example
        /// ```
        /// use packybara_grpc::get_pkgcoords;
        ///
        /// # fn dox() -> std::io::Result<()> {
        ///
        /// let options - get_pkgcoords::Options::new();
        /// # Ok(())
        /// # }
        /// ```
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
        /// Set the optional package name
        ///
        /// # Example
        /// ```
        /// use packybara_grpc::get_pkgcoords;
        ///
        /// # fn dox() -> std::io::Result<()> {
        ///
        /// let options - get_pkgcoords::Options::new()
        ///                              .package_opt(Some("maya"));
        /// # Ok(())
        /// # }
        /// ```
        pub fn package_opt<I>(mut self, package: Option<I>) -> Self
        where
            I: Into<String>,
        {
            self.package = package.map(|p| p.into());
            self
        }
        /// Set the optional level name
        ///
        /// # Example
        /// ```
        /// use packybara_grpc::get_pkgcoords;
        ///
        /// # fn dox() -> std::io::Result<()> {
        ///
        /// let options - get_pkgcoords::Options::new()
        ///                              .package_opt(Some("maya"))
        ///                              .level_opt(Some("dev01"));
        /// # Ok(())
        /// # }
        /// ```
        pub fn level_opt<I>(mut self, level: Option<I>) -> Self
        where
            I: Into<String>,
        {
            self.level = level.map(|l| l.into());
            self
        }
        /// Set the optional role name
        ///
        /// # Example
        /// ```
        /// use packybara_grpc::get_pkgcoords;
        ///
        /// # fn dox() -> std::io::Result<()> {
        ///
        /// let options - get_pkgcoords::Options::new()
        ///                              .role_opt(Some("model"));
        /// # Ok(())
        /// # }
        /// ```
        pub fn role_opt<I>(mut self, role: Option<I>) -> Self
        where
            I: Into<String>,
        {
            self.role = role.map(|r| r.into());
            self
        }
        /// Set the optional platform name
        ///
        /// # Example
        /// ```
        /// use packybara_grpc::get_pkgcoords;
        ///
        /// # fn dox() -> std::io::Result<()> {
        ///
        /// let options - get_pkgcoords::Options::new()
        ///                              .platform_opt(Some("cent7_64"));
        /// # Ok(())
        /// # }
        /// ```
        pub fn platform_opt<I>(mut self, platform: Option<I>) -> Self
        where
            I: Into<String>,
        {
            self.platform = platform.map(|p| p.into());
            self
        }
        /// Set the optional site name
        ///
        /// # Example
        /// ```
        /// use packybara_grpc::get_pkgcoords;
        ///
        /// # fn dox() -> std::io::Result<()> {
        ///
        /// let options - get_pkgcoords::Options::new()
        ///                              .site_opt(Some("platform"));
        /// # Ok(())
        /// # }
        /// ```
        pub fn site_opt<I>(mut self, site: Option<String>) -> Self
        where
            I: Into<String>,
        {
            self.site = site.map(|s| s.into());
            self
        }
        /// Set the optional package name
        ///
        /// # Example
        /// ```
        /// use packybara_grpc::get_pkgcoords;
        ///
        /// # fn dox() -> std::io::Result<()> {
        ///
        /// let options - get_pkgcoords::Options::new()
        ///                              .package_opt(Some("maya"));
        /// # Ok(())
        /// # }
        /// ```
        pub fn search_mode_opt<I>(mut self, search_mode: Option<I>) -> Self
        where
            I: Into<String>,
        {
            self.search_mode = search_mode.map(|s| s.into());
            self
        }
        pub fn order_by_opt(mut self, order_by: Option<String>) -> Self {
            self.order_by = order_by;
            self
        }
    }
}
pub(crate) mod get_pkgcoords_impl {
    use super::*;
    pub async fn cmd(
        grpc_client: &mut ClientService,
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
        let response = grpc_client.client.get_pkg_coords(request).await?;
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
}
