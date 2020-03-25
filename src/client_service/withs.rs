use super::*;

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
        /// New up an instance of get_pkgcoords::Options given a name, order_by
        /// order_direction, and limit
        ///
        /// # Arguments
        ///
        /// * `name` - the optional name of the platform
        /// * `level` - the optional field to level by
        /// * `role`
        /// * `platform`
        /// * `site`
        // /// * `search_mode`
        /// * `order_by` - the optional direction to order by
        ///
        /// # Returns
        ///
        /// * Option instance
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

        pub fn package_opt(mut self, package: Option<String>) -> Self {
            self.package = package;
            self
        }

        pub fn level_opt(mut self, level: Option<String>) -> Self {
            self.level = level;
            self
        }
        pub fn role_opt(mut self, role: Option<String>) -> Self {
            self.role = role;
            self
        }
        pub fn platform_opt(mut self, platform: Option<String>) -> Self {
            self.platform = platform;
            self
        }
        pub fn site_opt(mut self, site: Option<String>) -> Self {
            self.site = site;
            self
        }
        // pub fn search_mode_opt(mut self, search_mode: Option<String>) -> Self {
        //     self.search_mode = search_mode;
        //     self
        // }
        pub fn limit_opt(mut self, limit: Option<i32>) -> Self {
            self.limit = limit;
            self
        }
        pub fn order_by_opt(mut self, order_by: Option<String>) -> Self {
            self.order_by = order_by;
            self
        }
        pub fn order_direction_opt(mut self, direction: Option<String>) -> Self {
            self.order_direction = direction;
            self
        }
    }

    use super::*;
    pub async fn cmd(
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

pub mod add_withs {
    /// Encapsulate the query parameter for adding withs
    pub struct Options {
        pub vpin_id: i64,
        pub withs: Vec<String>,
        pub author: String,
        pub comment: Option<String>,
    }

    impl Options {
        /// New up an instance of add_withs::Options given a name, order_by
        /// order_direction, and limit
        ///
        /// # Arguments
        ///
        /// * `names` - vector of withs names
        /// * `author` - name of the person who authored the new withs
        ///
        /// # Returns
        ///
        /// * Option instance
        pub fn new<I>(vpin_id: i64, withs: Vec<I>, author: I) -> Self
        where
            I: Into<String>,
        {
            let withs = withs.into_iter().map(|n| n.into()).collect::<Vec<_>>();
            //

            Self {
                vpin_id,
                withs,
                author: author.into(),
                comment: None,
            }
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
    use crate::{AddReply, WithsAddRequest};
    pub async fn cmd(
        grpc_client: &mut ClientService,
        options: Options,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        let Options {
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
