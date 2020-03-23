use super::*;

pub mod get_roles {
    /// Encapsulate the query parameters
    pub struct Options {
        pub role: Option<String>,
        pub category: Option<String>,
        pub order_by: Option<String>,
        pub order_direction: Option<String>,
        pub limit: Option<i32>,
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
            Self {
                role: None,
                category: None,
                order_direction: None,
                order_by: None,
                limit: None,
            }
        }

        pub fn role_opt(mut self, role: Option<String>) -> Self {
            self.role = role;
            self
        }

        pub fn category_opt(mut self, category: Option<String>) -> Self {
            self.category = category;
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
        grpc_client: &mut ClientService,
        options: get_roles::Options,
    ) -> Result<Vec<FindAllRolesRow>, Box<dyn std::error::Error>> {
        let get_roles::Options {
            role,
            category,
            order_by,
            order_direction,
            limit,
        } = options;
        let request = tonic::Request::new(RolesQueryRequest {
            role,
            category,
            order_by,
            order_direction,
            limit: limit.map(|x| x as i32),
        });
        let response = grpc_client.client.get_roles(request).await?;
        let RolesQueryReply { roles } = response.into_inner();

        let results = roles
            .into_iter()
            .map(|role| {
                let RolesQueryRow { role, category } = role;
                FindAllRolesRow::from_parts(&role, &category)
            })
            .collect::<Vec<_>>();

        Ok(results)
    }
}
