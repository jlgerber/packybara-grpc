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
        grpc_client: &mut ClientService,
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

pub mod add_levels {
    /// Encapsulate the query parameter for adding levels
    pub struct Options {
        pub names: Vec<String>,
        pub author: String,
        pub comment: Option<String>,
    }

    impl Options {
        /// New up an instance of add_levels::Options given a name, order_by
        /// order_direction, and limit
        ///
        /// # Arguments
        ///
        /// * `names` - vector of levels names
        /// * `author` - name of the person who authored the new levels
        ///
        /// # Returns
        ///
        /// * Option instance
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

        /// Update comment with option wrapped type implementing
        /// Into<String>
        ///
        /// # Arguments
        ///
        /// * `comment` - The optional comment associated with the commit
        pub fn comment_opt<I>(mut self, comment: Option<I>) -> Self
        where
            I: Into<String>,
        {
            let comment = comment.map(|c| c.into());
            self.comment = comment;
            self
        }
    }

    use super::*;
    use crate::{AddReply, LevelsAddRequest};
    pub async fn cmd(
        grpc_client: &mut ClientService,
        options: Options,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        let Options {
            names,
            author,
            comment,
        } = options;
        let request = tonic::Request::new(LevelsAddRequest {
            names,
            author,
            comment,
        });
        let response = grpc_client.client.add_levels(request).await?;
        let AddReply { updates } = response.into_inner();

        Ok(updates)
    }
}
