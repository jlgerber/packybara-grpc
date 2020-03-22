use super::*;

pub mod get_platforms {
    /// Encapsulate the query parameters
    pub struct Options {
        pub name: Option<String>,
        pub order_by: Option<String>,
        pub order_direction: Option<String>,
        pub limit: Option<i32>,
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
            Self {
                name: None,
                order_direction: None,
                order_by: None,
                limit: None,
            }
        }

        pub fn name_opt(mut self, name: Option<String>) -> Self {
            self.name = name;
            self
        }

        pub fn order_direction_opt(mut self, direction: Option<String>) -> Self {
            self.order_direction = direction;
            self
        }

        pub fn order_by_opt(mut self, order_by: Option<String>) -> Self {
            self.order_by = order_by;
            self
        }
        pub fn limit_opt(mut self, limit: Option<i32>) -> Self {
            self.limit = limit;
            self
        }
    }

    use super::*;

    pub async fn cmd(
        grpc_client: &mut Client,
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
