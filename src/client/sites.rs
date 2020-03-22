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
}
