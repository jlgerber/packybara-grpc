use super::*;

pub mod get_levels {
    /// Encapsulate the query parameters
    pub struct Options {
        pub level: Option<String>,
        pub show: Option<String>,
        pub depth: Option<u8>,
        pub order_by: Option<String>,
    }

    impl Options {
        /// New up an instance of get_levels::Option
        ///
        /// # Arguments
        ///
        /// * `level` - the name of the level
        /// * `show` - the name of the show
        /// * `depth` - the optional depth
        /// * `order_by` - the optional field to order by
        ///
        /// # Returns
        ///
        /// * Option instance
        pub fn new() -> Self {
            Self {
                level: None,
                show: None,
                depth: None,
                order_by: None,
            }
        }

        pub fn level_opt(mut self, level: Option<String>) -> Self {
            self.level = level;
            self
        }

        pub fn show_opt(mut self, show: Option<String>) -> Self {
            self.show = show;
            self
        }

        pub fn depth_opt(mut self, depth: Option<u8>) -> Self {
            self.depth = depth;
            self
        }

        pub fn order_by_opt(mut self, order_by: Option<String>) -> Self {
            self.order_by = order_by;
            self
        }
    }

    use super::*;

    pub async fn cmd(
        grpc_client: &mut Client,
        options: get_levels::Options,
    ) -> Result<Vec<FindAllLevelsRow>, Box<dyn std::error::Error>> {
        let get_levels::Options {
            level,
            show,
            depth,
            order_by,
        } = options;
        let request = tonic::Request::new(LevelsQueryRequest {
            level,
            show,
            depth: depth.map(|x| x as u32),
            order_by,
        });
        let response = grpc_client.client.get_levels(request).await?;
        let LevelsQueryReply { levels } = response.into_inner();

        let results = levels
            .into_iter()
            .map(|level| {
                let LevelsQueryRow { level, show } = level;
                FindAllLevelsRow::from_parts(&level, &show)
            })
            .collect::<Vec<_>>();

        Ok(results)
    }
}
