use super::*;

pub mod get_changes {
    /// Encapsulate the query parameters
    pub struct Options {
        pub transaction_id: Option<i64>,
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
                transaction_id: None,
            }
        }

        pub fn transaction_id_opt(mut self, transaction_id: Option<i64>) -> Self {
            self.transaction_id = transaction_id;
            self
        }
    }
}
