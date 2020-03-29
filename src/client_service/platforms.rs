//! `platforms` module exports `get_platforms::Options` providing query parameters for `ClientService.get_platforms`
//! and `add_platforms::Options` providing parameters for `ClientService.add_platforms`
use super::*;

/// Exports `Options`, used to set query parameters for `ClientService.get_platforms`
pub mod get_platforms {
    /// Encapsulate the query parameters
    pub struct Options {
        pub name: Option<String>,
        pub order_by: Option<String>,
        pub order_direction: Option<String>,
        pub limit: Option<i32>,
    }

    impl Options {
        /// New up an instance of get_platforms::Options
        ///
        /// # Example
        /// ```
        /// use packybara_grpc::get_platforms;
        ///
        /// # fn dox() -> std::io::Result<()> {
        ///
        /// let options - get_platforms::Options::new();
        /// # Ok(())
        /// # }
        /// ```
        pub fn new() -> Self {
            Self {
                name: None,
                order_direction: None,
                order_by: None,
                limit: None,
            }
        }

        /// Set an optional name of the platform or platforms we are querying for. May
        /// use `%` in the query as a wildcard.
        ///
        /// # Example
        /// ```
        /// use packybara_grpc::get_platforms;
        ///
        /// # fn dox() -> std::io::Result<()> {
        ///
        /// let options - get_platforms::Options::new()
        ///                              .name_opt(Some("maya%"));
        /// # Ok(())
        /// # }
        /// ```
        pub fn name_opt(mut self, name: Option<String>) -> Self {
            self.name = name;
            self
        }

        /// Set the direction that results will be returned by the server.
        ///
        /// # Example
        /// ```
        /// use packybara_grpc::get_platforms;
        ///
        /// # fn dox() -> std::io::Result<()> {
        ///
        /// let options - get_platforms::Options::new()
        ///                              .package_opt(Some("maya"))
        ///                              .order_direction_opt(Some("asc"));
        /// # Ok(())
        /// # }
        /// ```
        pub fn order_direction_opt(mut self, direction: Option<String>) -> Self {
            self.order_direction = direction;
            self
        }
        /// Set the optional field to order the results of our query by. This
        /// corresponds with the sql `ORDER BY` fragment.
        ///
        /// # Example
        /// ```
        /// use packybara_grpc::get_platforms;
        ///
        /// # fn dox() -> std::io::Result<()> {
        ///
        /// let options - get_platforms::Options::new()
        ///                              .package_opt(Some("maya"))
        ///                              .order_by_opt(Some("package"));
        /// # Ok(())
        /// # }
        /// ```
        pub fn order_by_opt(mut self, order_by: Option<String>) -> Self {
            self.order_by = order_by;
            self
        }
        /// Set an optional limit on the number of results returnd by the
        /// `ClientService.get_platforms` method.
        ///
        /// # Example
        /// ```
        /// use packybara_grpc::get_platformss;
        ///
        /// # fn dox() -> std::io::Result<()> {
        ///
        /// let options - get_platforms::Options::new()
        ///                              .limit_opt(Some(2));
        /// # Ok(())
        /// # }
        /// ```
        pub fn limit_opt(mut self, limit: Option<i32>) -> Self {
            self.limit = limit;
            self
        }
    }
}
pub(crate) mod get_platforms_impl {
    use super::*;

    pub async fn cmd(
        grpc_client: &mut ClientService,
        options: get_platforms::Options,
    ) -> Result<Vec<FindAllPlatformsRow>, Box<dyn std::error::Error>> {
        let get_platforms::Options {
            name,
            order_by,
            order_direction,
            limit,
        } = options;
        let request = tonic::Request::new(PlatformsQueryRequest {
            name,
            order_by,
            order_direction,
            limit: limit.map(|x| x as i32),
        });
        let response = grpc_client.client.get_platforms(request).await?;
        let PlatformsQueryReply { names } = response.into_inner();

        let results = names
            .into_iter()
            .map(|name| {
                let PlatformsQueryRow { name } = name;
                FindAllPlatformsRow::from_parts(&name)
            })
            .collect::<Vec<_>>();

        Ok(results)
    }
}

/// Exports `Options`, used to set parameters for `ClientService.add_platforms`
pub mod add_platforms {
    /// Encapsulate the query parameter for adding platforms
    pub struct Options {
        pub names: Vec<String>,
        pub author: String,
        pub comment: Option<String>,
    }

    impl Options {
        /// New up an instance of add_platforms::Options given a name, order_by
        /// order_direction, and limit
        ///
        /// # Example
        /// ```
        /// use packybara_grpc::add_platforms;
        ///
        /// # fn dox() -> std::io::Result<()> {
        ///
        /// let options - add_platforms::Options::new(vec!["cent7_64", "cent8_64"]);
        /// # Ok(())
        /// # }
        /// ```
        pub fn new<I>(names: Vec<I>, author: I) -> Self
        where
            I: Into<String>,
        {
            let names = names.into_iter().map(|n| n.into()).collect::<Vec<_>>();
            //

            Self {
                names,
                author: author.into(),
                comment: None,
            }
        }

        /// Update comment with option wrapped type implementing
        /// Into<String>
        ///
        /// # Example
        /// ```
        /// use packybara_grpc::add_platforms;
        ///
        /// # fn dox() -> std::io::Result<()> {
        ///
        /// let options - add_platforms::Options::new(vec!["cent7_64", "cent8_64"])
        ///                                 .comment_opt("adding new linux platforms");
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
pub(crate) mod add_platforms_impl {
    use super::*;
    use crate::{AddReply, PlatformsAddRequest};
    pub async fn cmd(
        grpc_client: &mut ClientService,
        options: add_platforms::Options,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        let add_platforms::Options {
            names,
            author,
            comment,
        } = options;
        let request = tonic::Request::new(PlatformsAddRequest {
            names,
            author,
            comment,
        });
        let response = grpc_client.client.add_platforms(request).await?;
        let AddReply { updates } = response.into_inner();

        Ok(updates)
    }
}
