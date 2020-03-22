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
        grpc_client: &mut Client,
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
