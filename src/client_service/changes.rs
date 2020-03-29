//! `changes` module exports `Options` struct to provide query parameters for
//! the `client.get_changes` method.
use super::*;

/// Module provides the `get_changes::Options` struct used to pass query parameters
/// to the server when retrieving changes via the ClientServer.get_changes` method.
pub mod get_changes {
    //use super::*;
    /// Encapsulate the query parameters used to retrieve changes
    pub struct Options {
        /// An optional transaction id. Each creation or update
        /// triggered by a user is associated with a unique transaction
        /// id. Here we specify a specfic transaction id to fetch info about
        pub transaction_id: Option<i64>,
    }

    impl Options {
        /// New up an instance of get_pkgcoords::Options given a name, order_by
        /// order_direction, and limit
        pub fn new() -> Self {
            Self {
                transaction_id: None,
            }
        }
        /// Set an optional transaction_id.
        ///
        /// # Example
        /// ```
        /// use packybara_grpc::client_service::get_changes;
        ///
        /// # fn dox() -> std::io::Result<()> {
        ///
        /// let options = get_changes::Options::new()
        ///                            .transaction_id_opt(Some(434));
        ///
        /// # Ok(())
        /// # }
        pub fn transaction_id_opt(mut self, transaction_id: Option<i64>) -> Self {
            self.transaction_id = transaction_id;
            self
        }
    }
}

pub(crate) mod get_changes_impl {
    use super::*;
    /// Given a mutable reference to a ClientService instance and a `get_changes::Options` instance,
    /// return the list of changes which match the query parameters provides by said options.
    pub(crate) async fn cmd(
        grpc_client: &mut ClientService,
        options: get_changes::Options,
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
