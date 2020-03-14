use crate::{Coords, PackybaraClient, VersionPinQueryReply, VersionPinQueryRequest};
use packybara::db::find::versionpins::FindVersionPinsRow;
use tonic::transport::Channel;

// this has some implications for applications that want to communicate
// in multiple channels. If this becomes a requirement, we will have to
// put an arc around client
pub struct Client {
    client: PackybaraClient<Channel>,
}

impl Client {
    /// create a new client instance , given a url
    pub async fn new<I>(url: I) -> Result<Self, Box<dyn std::error::Error>>
    where
        I: Into<String>,
    {
        let client = PackybaraClient::connect(url.into()).await?;
        Ok(Client { client })
    }
    /// Retrieve versionpin from server, given GetVersionPinOptions instance
    ///
    /// # Example
    ///
    /// ```ignore
    /// let results = client.get_version_in(GetVersionPinOptions::new("maya").role("model")).await?;
    /// ```
    pub async fn get_version_pin(
        &mut self,
        options: GetVersionPinOptions,
    ) -> Result<FindVersionPinsRow, Box<dyn std::error::Error>> {
        let GetVersionPinOptions {
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
/// Encapsulate the query parameters
pub struct GetVersionPinOptions {
    package: String,
    level: Option<String>,
    role: Option<String>,
    platform: Option<String>,
    site: Option<String>,
}

impl GetVersionPinOptions {
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

    ///
    pub fn level_opt<I>(mut self, level: Option<I>) -> Self
    where
        I: Into<String>,
    {
        self.level = level.map(|x| x.into());
        self
    }

    pub fn role_opt<I>(mut self, role: Option<I>) -> Self
    where
        I: Into<String>,
    {
        self.role = role.map(|x| x.into());
        self
    }

    pub fn platform_opt<I>(mut self, platform: Option<I>) -> Self
    where
        I: Into<String>,
    {
        self.platform = platform.map(|x| x.into());
        self
    }

    pub fn site_opt<I>(mut self, site: Option<I>) -> Self
    where
        I: Into<String>,
    {
        self.site = site.map(|x| x.into());
        self
    }
}
