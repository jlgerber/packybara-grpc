use super::*;

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
                id: None,
                transaction_id: None,
                author: None,
                order_by: None,
                order_direction: None,
                limit: None,
            }
        }

        pub fn id_opt(mut self, id: Option<i64>) -> Self {
            self.id = id;
            self
        }

        pub fn transaction_id_opt(mut self, transaction_id: Option<i64>) -> Self {
            self.transaction_id = transaction_id;
            self
        }
        pub fn author_opt(mut self, author: Option<String>) -> Self {
            self.author = author;
            self
        }
        pub fn order_by_opt(mut self, order_by: Option<String>) -> Self {
            self.order_by = order_by;
            self
        }
        pub fn order_direction_opt(mut self, direction: Option<String>) -> Self {
            self.order_direction = direction;
            self
        }
        pub fn limit_opt(mut self, limit: Option<i32>) -> Self {
            self.limit = limit;
            self
        }
    }

    pub async fn cmd(
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
