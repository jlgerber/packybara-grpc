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
}
