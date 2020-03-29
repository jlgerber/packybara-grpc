//! `revisions` module exports `get_revisions::Options` providing query parameters for `ClientService.get_revisions`
//! and `add_revisions::Options` providing parameters for `ClientService.add_revisions`
use super::*;

/// Exports `Options`, used to set query parameters for `ClientService.get_revisions`
pub mod get_revisions {
    use super::*;
    /// Encapsulate the query parameters
    pub struct Options {
        pub id: Option<i64>,
        pub transaction_id: Option<i64>,
        pub author: Option<String>,
        pub order_by: Option<String>,
        pub order_direction: Option<String>,
        pub limit: Option<i32>,
    }

    impl Options {
        /// New up an instance of get_revisions::Options
        ///
        /// # Example
        /// ```
        /// use packybara_grpc::get_revisions;
        ///
        /// # fn dox() -> std::io::Result<()> {
        ///
        /// let options - get_revisions::Options::new();
        /// # Ok(())
        /// # }
        /// ```
        pub fn new() -> Self {
            Self {
                id: None,
                transaction_id: None,
                author: None,
                order_by: None,
                order_direction: None,
                limit: None,
            }
        }

        /// Set an optional id used to query for revisons by id.
        ///
        /// # Example
        /// ```
        /// use packybara_grpc::get_revisions;
        ///
        /// # fn dox() -> std::io::Result<()> {
        ///
        /// let options - get_revisions::Options::new()
        ///                              .id_opt(Some(2345566));
        /// # Ok(())
        /// # }
        /// ```
        pub fn id_opt(mut self, id: Option<i64>) -> Self {
            self.id = id;
            self
        }
        /// Set an optional transaction id. Used to query for revisons by their transaction id.
        ///
        /// # Example
        /// ```
        /// use packybara_grpc::get_revisions;
        ///
        /// # fn dox() -> std::io::Result<()> {
        ///
        /// let options - get_revisions::Options::new()
        ///                              .transaction_id_opt(Some(321));
        /// # Ok(())
        /// # }
        /// ```
        pub fn transaction_id_opt(mut self, transaction_id: Option<i64>) -> Self {
            self.transaction_id = transaction_id;
            self
        }
        /// Set an optional author. Used to query for revisons by their author.
        ///
        /// # Example
        /// ```
        /// use packybara_grpc::get_revisions;
        ///
        /// # fn dox() -> std::io::Result<()> {
        ///
        /// let options - get_revisions::Options::new()
        ///                              .author_opt(Some("jgerber"));
        /// # Ok(())
        /// # }
        /// ```
        pub fn author_opt<I>(mut self, author: Option<I>) -> Self
        where
            I: Into<String>,
        {
            self.author = author.map(|a| a.into());
            self
        }
        /// Set an optional ordering. Used to set the field that results are sorted by on return.
        ///
        /// # Example
        /// ```
        /// use packybara_grpc::get_revisions;
        ///
        /// # fn dox() -> std::io::Result<()> {
        ///
        /// let options - get_revisions::Options::new()
        ///                              .author_opt(Some("jgerber"))
        ///                              .order_by_opt(Some("author"));
        /// # Ok(())
        /// # }
        /// ```
        pub fn order_by_opt(mut self, order_by: Option<String>) -> Self {
            self.order_by = order_by;
            self
        }
        /// Set an optional order direction. Used to control whether sorting is ascending or descending.
        ///
        /// # Example
        /// ```
        /// use packybara_grpc::get_revisions;
        ///
        /// # fn dox() -> std::io::Result<()> {
        ///
        /// let options - get_revisions::Options::new()
        ///                              .author_opt(Some("jgerber"))
        ///                              .order_by_opt(Some("author"))
        ///                              .order_direction_opt(Some("asc"));
        /// # Ok(())
        /// # }
        /// ```
        pub fn order_direction_opt(mut self, direction: Option<String>) -> Self {
            self.order_direction = direction;
            self
        }
        /// Set an optional limit. Used to limit the max number of returned values.
        ///
        /// # Example
        /// ```
        /// use packybara_grpc::get_revisions;
        ///
        /// # fn dox() -> std::io::Result<()> {
        ///
        /// let options - get_revisions::Options::new()
        ///                              .author_opt(Some("jgerber"))
        ///                              .order_by_opt(Some("author"))
        ///                              .limit_opt(Some(2));
        /// # Ok(())
        /// # }
        /// ```
        pub fn limit_opt(mut self, limit: Option<i32>) -> Self {
            self.limit = limit;
            self
        }
    }
}
pub(crate) mod get_revisions_impl {
    use super::*;
    pub(crate) async fn cmd(
        grpc_client: &mut ClientService,
        options: get_revisions::Options,
    ) -> Result<Vec<FindAllRevisionsRow>, Box<dyn std::error::Error>> {
        let get_revisions::Options {
            id,
            transaction_id,
            author,
            order_by,
            order_direction,
            limit,
        } = options;
        let request = tonic::Request::new(RevisionsQueryRequest {
            id,
            transaction_id,
            author,
            order_by,
            order_direction,
            limit,
        });
        let response = grpc_client.client.get_revisions(request).await?;
        let RevisionsQueryReply { revisions } = response.into_inner();

        let results = revisions
            .into_iter()
            .map(|rev| {
                let RevisionsQueryRow {
                    id,
                    transaction_id,
                    author,
                    comment,
                    datetime,
                } = rev;
                //println!("DATETIME {}", datetime);
                //2020-01-25 19:23:26.672258 -08:00
                //let dt = DateTime::<Utc>::from_utc(NaiveDateTime::from_timestamp(61, 0), Utc);
                //let dt: DateTime<Local> = DateTime::from(dt);
                let dt = DateTime::parse_from_str(&datetime, "%F %T%.3f %z")
                    .expect("unable to unwrap time");
                let dt: DateTime<Local> = DateTime::from(dt);
                FindAllRevisionsRow::from_parts(
                    id as i32,
                    transaction_id,
                    &author,
                    dt,
                    //&datetime,
                    &comment,
                )
            })
            .collect::<Vec<_>>();

        Ok(results)
    }
}
