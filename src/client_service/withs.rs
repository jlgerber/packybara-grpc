//! `withs` exports `get_withs::Options`, used to provide query parameters to
//! the `ClientService.get_withs` method, `add_withs::Options`, used to provide
//! parameters to the `ClientService.add_withs` method, and `set_withs::Options` used to
//! provide parameters to the `ClientService.set_withs` method.
use super::*;
/// Exports `Options` struct, used to pass query parameters to `ClientService.get_withs` method
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
        /// New up an instance of get_withs::Options
        ///
        /// # Example
        /// ```
        /// # fn dox() -> std::io::Result<()> {
        /// use packybara_grpc::get_withs;
        /// let options = get_withs::Options::new();
        ///                              
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
                //search_mode: None,
                limit: None,
                order_by: None,
                order_direction: None,
            }
        }
        /// Set package option on `Option` instance, following the owned builder
        /// pattern
        ///
        /// # Example
        /// ```
        /// # fn dox() -> std::io::Result<()> {
        /// use packybara_grpc::get_withs;
        /// let options = get_withs::Options::new()
        ///                                .package_opt(Some("maya"));
        ///                              
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
        /// Set level option on `Option` instance, following the owned builder
        /// pattern
        ///
        /// # Example
        /// ```
        /// # fn dox() -> std::io::Result<()> {
        /// use packybara_grpc::get_withs;
        /// let options = get_withs::Options::new()
        ///                                .level_opt(Some("dev01"));
        ///                              
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
        /// Set role option on `Option` instance.
        ///
        /// # Example
        /// ```
        /// # fn dox() -> std::io::Result<()> {
        /// use packybara_grpc::get_withs;
        /// let options = get_withs::Options::new()
        ///                                .role_opt(Some("model"));
        ///                              
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
        /// Set platform option on `Option` instance, limiting the withs search to those
        /// results set for the provided platform, if Some; otherwise searching for any results
        /// with any platform.
        ///
        /// # Example
        /// ```
        /// # fn dox() -> std::io::Result<()> {
        /// use packybara_grpc::get_withs;
        /// let options = get_withs::Options::new()
        ///                                .platform_opt(Some("cent7_64"));
        ///                              
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
        /// Set site option for the query.
        ///
        /// # Example
        /// ```
        /// # fn dox() -> std::io::Result<()> {
        /// use packybara_grpc::get_withs;
        /// let options = get_withs::Options::new()
        ///                                .site_opt(Some("playa"));
        ///                              
        /// # Ok(())
        /// # }
        /// ```
        pub fn site_opt<I>(mut self, site: Option<I>) -> Self
        where
            I: Into<String>,
        {
            self.site = site.map(|s| s.into());
            self
        }
        // pub fn search_mode_opt(mut self, search_mode: Option<String>) -> Self {
        //     self.search_mode = search_mode;
        //     self
        // }
        /// Set limit option, providing the max number of results returned by the
        /// query
        ///
        /// pattern
        ///
        /// # Example
        /// ```
        /// # fn dox() -> std::io::Result<()> {
        /// use packybara_grpc::get_withs;
        /// let options = get_withs::Options::new()
        ///                                .limit_opt(Some(1));
        ///                              
        /// # Ok(())
        /// # }
        /// ```
        pub fn limit_opt(mut self, limit: Option<i32>) -> Self {
            self.limit = limit;
            self
        }
        /// Set order_by option, providing the field that the results will be sorted on.
        ///
        /// pattern
        ///
        /// # Example
        /// ```
        /// # fn dox() -> std::io::Result<()> {
        /// use packybara_grpc::get_withs;
        /// let options = get_withs::Options::new()
        ///                                .order_by_opt(Some("package"));
        ///                              
        /// # Ok(())
        /// # }
        /// ```
        pub fn order_by_opt<I>(mut self, order_by: Option<I>) -> Self
        where
            I: Into<String>,
        {
            self.order_by = order_by.map(|o| o.into());
            self
        }
        /// Set order_direction option, providing the direction that results will be sorted
        /// in, working in concert with `order_by`.
        ///
        /// pattern
        ///
        /// # Example
        /// ```
        /// # fn dox() -> std::io::Result<()> {
        /// use packybara_grpc::get_withs;
        /// let options = get_withs::Options::new()
        ///                                .order_direction_opt(Some("asc"));
        ///                              
        /// # Ok(())
        /// # }
        /// ```
        pub fn order_direction_opt<I>(mut self, direction: Option<I>) -> Self
        where
            I: Into<String>,
        {
            self.order_direction = direction.map(|d| d.into());
            self
        }
    }
}
pub(crate) mod get_withs_impl {
    use super::*;
    pub(crate) async fn cmd(
        grpc_client: &mut ClientService,
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
        let response = grpc_client.client.get_withs(request).await?;
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
}

/// Exports `Options` struct, used to pass parameters to the `ClientService.add_withs` method
pub mod add_withs {
    /// Encapsulate the parameters for adding withs
    pub struct Options {
        pub vpin_id: i64,
        pub withs: Vec<String>,
        pub author: String,
        pub comment: Option<String>,
    }

    impl Options {
        /// New up an instance of add_withs::Options given a versionpin id, a vector of with package names, and author
        ///
        /// pattern
        ///
        /// # Example
        /// ```
        /// # fn dox() -> std::io::Result<()> {
        /// use packybara_grpc::add_withs;
        /// let options = add_withs::Options::new(15321, vec!["modelpipeline", "modelpublish"], "jgerber");
        /// # Ok(())
        /// # }
        /// ```
        pub fn new<I>(vpin_id: i64, withs: Vec<I>, author: I) -> Self
        where
            I: Into<String>,
        {
            let withs = withs.into_iter().map(|n| n.into()).collect::<Vec<_>>();

            Self {
                vpin_id,
                withs,
                author: author.into(),
                comment: None,
            }
        }

        /// Add an optional comment to the Option instance
        ///
        /// pattern
        ///
        /// # Example
        /// ```
        /// # fn dox() -> std::io::Result<()> {
        /// use packybara_grpc::add_withs;
        /// let options = add_withs::Options::new(15321, vec!["modelpipeline", "modelpublish"], "jgerber")
        ///                             .comment_opt(Some("adding modelpipeline and publish as withs to 15321"));
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
pub(crate) mod add_withs_impl {
    use super::*;
    use crate::{AddReply, WithsAddRequest};
    pub async fn cmd(
        grpc_client: &mut ClientService,
        options: add_withs::Options,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        let add_withs::Options {
            vpin_id,
            withs,
            author,
            comment,
        } = options;
        let request = tonic::Request::new(WithsAddRequest {
            vpin_id,
            withs,
            author,
            comment,
        });
        let response = grpc_client.client.add_withs(request).await?;
        let AddReply { updates } = response.into_inner();

        Ok(updates)
    }
}
