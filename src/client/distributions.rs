use super::*;

pub mod get_distributions {
    /// Encapsulate the query parameters
    pub struct Options {
        pub package: Option<String>,
        pub version: Option<String>,
        pub order_direction: Option<String>,
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
                package: None,
                version: None,
                order_direction: None,
            }
        }

        pub fn package_opt(mut self, package: Option<String>) -> Self {
            self.package = package;
            self
        }

        pub fn version_opt(mut self, version: Option<String>) -> Self {
            self.version = version;
            self
        }
        pub fn order_direction_opt(mut self, direction: Option<String>) -> Self {
            self.order_direction = direction;
            self
        }

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
