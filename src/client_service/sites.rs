use super::*;

pub mod get_sites {
    /// Encapsulate the query parameters
    pub struct Options {
        pub name: Option<String>,
    }

    impl Options {
        /// New up an instance of get_roles::Options given a role, category, order_by
        /// order_direction, and limit
        ///
        /// # Arguments
        ///
        /// * `role` - the optional name of the role
        /// * `category` - the optional name of the category
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
    }

    use super::*;

    pub async fn cmd(
        grpc_client: &mut ClientService,
        options: get_sites::Options,
    ) -> Result<Vec<FindAllSitesRow>, Box<dyn std::error::Error>> {
        let get_sites::Options { name } = options;
        let request = tonic::Request::new(SitesQueryRequest { name });
        let response = grpc_client.client.get_sites(request).await?;
        let SitesQueryReply { names } = response.into_inner();

        let results = names
            .into_iter()
            .map(|site| {
                let SitesQueryRow { name } = site;
                FindAllSitesRow::from_parts(&name)
            })
            .collect::<Vec<_>>();

        Ok(results)
    }
}

pub mod add_sites {
    /// Encapsulate the query parameter for adding sites
    pub struct Options {
        pub names: Vec<String>,
        pub author: String,
        pub comment: Option<String>,
    }

    impl Options {
        /// New up an instance of add_sites::Options given a name, order_by
        /// order_direction, and limit
        ///
        /// # Arguments
        ///
        /// * `names` - vector of sites names
        /// * `author` - name of the person who authored the new sites
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
    use crate::{AddReply, SitesAddRequest};
    pub async fn cmd(
        grpc_client: &mut ClientService,
        options: Options,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        let Options {
            names,
            author,
            comment,
        } = options;
        let request = tonic::Request::new(SitesAddRequest {
            names,
            author,
            comment,
        });
        let response = grpc_client.client.add_sites(request).await?;
        let AddReply { updates } = response.into_inner();

        Ok(updates)
    }
}
