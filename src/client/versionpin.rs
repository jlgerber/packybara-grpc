use super::*;

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
