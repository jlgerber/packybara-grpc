use super::*;

pub mod get_changes {
    use super::*;
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

    pub async fn cmd(
        grpc_client: &mut ClientService,
        options: Options,
    ) -> Result<Vec<FindAllChangesRow>, Box<dyn std::error::Error>> {
        let get_changes::Options { transaction_id } = options;
        let request = tonic::Request::new(ChangesQueryRequest { transaction_id });
        let response = grpc_client.client.get_changes(request).await?;
        let ChangesQueryReply { changes } = response.into_inner();

        let results = changes
            .into_iter()
            .map(|chng| {
                let ChangesQueryRow {
                    id,
                    transaction_id,
                    action,
                    coords:
                        Coords {
                            level,
                            role,
                            platform,
                            site,
                        },
                    package,
                    old,
                    new,
                } = chng;
                let change_action = ChangeAction::from_str(&action)
                    .expect("could not convert to action to ChangeAction");
                let old = old.unwrap_or("".to_string());
                //let new = Distribution::new(new);
                let action = change_action.to_string();

                FindAllChangesRow::from_parts(
                    id,
                    transaction_id,
                    &action,
                    &level,
                    &role,
                    &platform,
                    &site,
                    &package,
                    &old,
                    &new,
                )
            })
            .collect::<Vec<_>>();

        Ok(results)
    }
}
