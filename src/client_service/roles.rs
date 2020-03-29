//! `roles` module exports `get_roles::Options` providing query parameters for `ClientService.get_roles`
//! and `add_roles::Options` providing parameters for `ClientService.add_roles`
use super::*;

/// Exports `Options`, used to set query parameters for `ClientService.get_roles`
pub mod get_roles {
    /// Encapsulate the query parameters for roles
    pub struct Options {
        pub role: Option<String>,
        pub category: Option<String>,
        pub order_by: Option<String>,
        pub order_direction: Option<String>,
        pub limit: Option<i32>,
    }

    impl Options {
        /// New up an instance of get_roles::Options
        ///
        /// # Example
        /// ```
        /// use packybara_grpc::get_roles;
        ///
        /// # fn dox() -> std::io::Result<()> {
        ///
        /// let options - get_roles::Options::new();
        /// # Ok(())
        /// # }
        /// ```
        pub fn new() -> Self {
            Self {
                role: None,
                category: None,
                order_direction: None,
                order_by: None,
                limit: None,
            }
        }

        /// Set the `role` given an `Option<string>`
        ///
        /// # Example
        /// ```
        /// use packybara_grpc::get_roles;
        ///
        /// # fn dox() -> std::io::Result<()> {
        ///
        /// let options - get_roles::Options::new()
        ///                         .role_opt(Some("model"));
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
        /// Set the `category` given an `Option<string>`.
        ///
        /// # Example
        /// ```
        /// use packybara_grpc::get_roles;
        ///
        /// # fn dox() -> std::io::Result<()> {
        ///
        /// let options - get_roles::Options::new()
        ///                         .role_opt(Some("model"))
        ///                         .category_opt(Some("role"));
        /// # Ok(())
        /// # }
        /// ```
        pub fn category_opt<I>(mut self, category: Option<I>) -> Self
        where
            I: Into<String>,
        {
            self.category = category.map(|c| c.into());
            self
        }
        /// Set the `otder_direction`. Used in conjunction with `order_by` to define the order of the sorted
        /// results
        ///
        /// # Example
        /// ```
        /// use packybara_grpc::get_roles;
        ///
        /// # fn dox() -> std::io::Result<()> {
        ///
        /// let options - get_roles::Options::new()
        ///                         .role_opt(Some("model"))
        ///                         .order_direction_opt(Some("asc"));
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
        /// Set `order_by` given an optional field name to sort the results by
        ///
        /// # Example
        /// ```
        /// use packybara_grpc::get_roles;
        ///
        /// # fn dox() -> std::io::Result<()> {
        ///
        /// let options - get_roles::Options::new()
        ///                         .role_opt(Some("model"))
        ///                         .order_by_opt(Some("role"));
        /// # Ok(())
        /// # }
        /// ```
        pub fn order_by_opt(mut self, order_by: Option<String>) -> Self {
            self.order_by = order_by;
            self
        }

        /// set the `limit` of the max number of values returned by the query
        ///
        /// # Example
        /// ```
        /// use packybara_grpc::get_roles;
        ///
        /// # fn dox() -> std::io::Result<()> {
        ///
        /// let options - get_roles::Options::new()
        ///                         .role_opt(Some("model"))
        ///                         .limit_opt(Some(1));
        /// # Ok(())
        /// # }
        /// ```
        pub fn limit_opt(mut self, limit: Option<i32>) -> Self {
            self.limit = limit;
            self
        }
    }
}
pub(crate) mod get_roles_impl {
    use super::*;
    /// Given a mutable reference to the ClientService instance and a
    /// `get_roles::Option` instance, return all the roles which match
    /// the query parameters defined in said option instance.
    pub(crate) async fn cmd(
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

/// Provides parameters for the `ClientService.add_roles` method.
pub mod add_roles {
    /// Encapsulate the parameters for adding roles
    pub struct Options {
        /// Provide the roles to create
        pub names: Vec<String>,
        /// Provide the name of the author making the change
        pub author: String,
        /// Optionally provide a change comment
        pub comment: Option<String>,
    }

    impl Options {
        /// New up an instance of add_roles::Options given a vector of role names and the author
        ///
        /// # Example
        /// ```
        /// use packybara_grpc::add_roles;
        ///
        /// # fn dox() -> std::io::Result<()> {
        ///
        /// let options - add_roles::Options::new(vec!["model", "anim"], "jgerber");
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

        /// Update comment with an optional comment
        ///
        /// # Example
        /// ```
        /// use packybara_grpc::add_roles;
        ///
        /// # fn dox() -> std::io::Result<()> {
        ///
        /// let options = add_roles::Options::new(vec!["model", "anim"], "jgerber").comment_opt(Some("adding some basic roles"));
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
pub(crate) mod add_roles_impl {
    use super::*;
    use crate::{AddReply, RolesAddRequest};
    pub async fn cmd(
        grpc_client: &mut ClientService,
        options: add_roles::Options,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        let add_roles::Options {
            names,
            author,
            comment,
        } = options;
        let request = tonic::Request::new(RolesAddRequest {
            names,
            author,
            comment,
        });
        let response = grpc_client.client.add_roles(request).await?;
        let AddReply { updates } = response.into_inner();

        Ok(updates)
    }
}
