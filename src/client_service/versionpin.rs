//! `versionpin` exports the `get_versionpin::Options` struct, used to provide query
//! parameters to the `ClientService.get_versionpin` method.
use super::*;

/// `get_versionpin::Option` provides query parameters to the `ClientService.get_versionpin`
/// method
pub mod get_versionpin {
    /// Encapsulate the query parameters for `ClientService.get_versionpin`
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
        /// # Example
        /// ```
        /// # fn dox() -> std::io::Result<()> {
        /// use packybara_grpc::get_versionpin;
        /// let options = get_versionpin::Options::new("maya");
        /// # Ok(())
        /// #}
        /// ```
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
        /// # Example
        /// ```
        /// # fn dox() -> std::io::Result<()> {
        /// use packybara_grpc::get_versionpin;
        /// let options = get_versionpin::Options::new("maya")
        ///                               .level_opt(Some("dev01"));
        /// # Ok(())
        /// #}
        /// ```
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
        /// # Example
        /// ```
        /// # fn dox() -> std::io::Result<()> {
        /// use packybara_grpc::get_versionpin;
        /// let options = get_versionpin::Options::new("maya")
        ///                               .level_opt(Some("dev01"))
        ///                               .role_opt(Some("model"));
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

        /// Given a mutable instance of Self and an Option wrapped platform,
        /// set platform and return Self, following the common builder pattern.
        ///
        /// # Example
        /// ```
        /// # fn dox() -> std::io::Result<()> {
        /// use packybara_grpc::get_versionpin;
        /// let options = get_versionpin::Options::new("maya")
        ///                               .level_opt(Some("dev01"))
        ///                               .platform_opt(Some("cent7_64"));
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

        /// Given a mutable instance of Self and an Option wrapped site,
        /// set site and return Self, following the common builder pattern.
        ///
        /// # Example
        /// ```
        /// # fn dox() -> std::io::Result<()> {
        /// use packybara_grpc::get_versionpin;
        /// let options = get_versionpin::Options::new("maya")
        ///                               .level_opt(Some("dev01"))
        ///                               .site_opt(Some("portland"));
        /// # Ok(())
        /// #}
        /// ```
        pub fn site_opt<I>(mut self, site: Option<I>) -> Self
        where
            I: Into<String>,
        {
            self.site = site.map(|x| x.into());
            self
        }
    }
}
pub(crate) mod get_versionpin_impl {
    use super::*;
    pub async fn cmd(
        grpc_client: &mut ClientService,
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
        let response = grpc_client.client.get_version_pin(request).await?;
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
}
