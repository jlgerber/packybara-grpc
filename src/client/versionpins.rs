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
}
