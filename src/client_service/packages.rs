use super::*;

pub mod get_packages {
    /// Encapsulate the query parameters
    pub struct Options {
        pub name: Option<String>,
    }

    impl Options {
        /// New up an instance of get_platform::Options given a name, order_by
        /// order_direction, and limit
        ///
        /// # Arguments
        ///
        /// * `name` - the optional name of the platform
        /// * `order_by` - the optional field to order by
        /// * `order_direction` - the optional direction to order by
        /// * `limit` - the optional limit
        ///
        /// # Returns
        ///
        /// * Option instance
        pub fn new() -> Self {
            Self { name: None }
        }

        pub fn name_opt(mut self, name: Option<String>) -> Self {
            self.name = name;
            self
        }

        // pub fn order_direction_opt(mut self, direction: Option<String>) -> Self {
        //     self.order_direction = direction;
        //     self
        // }

        // pub fn order_by_opt(mut self, order_by: Option<String>) -> Self {
        //     self.order_by = order_by;
        //     self
        // }
        // pub fn limit_opt(mut self, limit: Option<i32>) -> Self {
        //     self.limit = limit;
        //     self
        // }
    }

    use super::*;

    pub async fn cmd(
        grpc_client: &mut ClientService,
        options: get_packages::Options,
    ) -> Result<Vec<FindAllPackagesRow>, Box<dyn std::error::Error>> {
        let get_packages::Options { name } = options;
        let request = tonic::Request::new(PackagesQueryRequest { name });
        let response = grpc_client.client.get_packages(request).await?;
        let PackagesQueryReply { names } = response.into_inner();

        let results = names
            .into_iter()
            .map(|name| {
                let PackagesQueryRow { name } = name;
                FindAllPackagesRow::from_parts(&name)
            })
            .collect::<Vec<_>>();

        Ok(results)
    }
}
pub mod add_packages {
    /// Encapsulate the query parameter for adding packages
    pub struct Options {
        pub names: Vec<String>,
        pub author: String,
        pub comment: Option<String>,
    }

    impl Options {
        /// New up an instance of add_packages::Options given a name, order_by
        /// order_direction, and limit
        ///
        /// # Arguments
        ///
        /// * `names` - vector of package names
        /// * `author` - name of the person who authored the new packages
        ///
        /// # Returns
        ///
        /// * Option instance
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
    use crate::{PackagesAddReply, PackagesAddRequest};
    pub async fn cmd(
        grpc_client: &mut ClientService,
        options: Options,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        let Options {
            names,
            author,
            comment,
        } = options;
        let request = tonic::Request::new(PackagesAddRequest {
            names,
            author,
            comment,
        });
        let response = grpc_client.client.add_packages(request).await?;
        let PackagesAddReply { updates } = response.into_inner();

        // let results = names
        //     .into_iter()
        //     .map(|name| {
        //         let PackagesQueryRow { name } = name;
        //         FindAllPackagesRow::from_parts(&name)
        //     })
        //     .collect::<Vec<_>>();

        Ok(updates)
    }
}
