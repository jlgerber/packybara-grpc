use crate::{
    url as grpcurl, Coords, PackybaraClient, VersionPinQueryReply, VersionPinQueryRequest,
};
use packybara::db::find::versionpins::FindVersionPinsRow;
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
        let response = self.client.get_version_pin_gr(request).await?;
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
