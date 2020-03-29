//! `levels` module exports the `get_levels::Options` struct which provides query
//! parameters for the `ClientService.get_levels` method, and the `add_levels::Options`
//! struct which provides parameters for the `ClientServer.add_levels` method.
use super::*;

/// Provides `Options` for the `ClientService.get_levels` method. Options exposes a
/// builder api, with a variety of methods to set options.
pub mod get_levels {
    /// Encapsulate the query parameters
    pub struct Options {
        /// Optional level name
        pub level: Option<String>,
        /// Optionally set the name of the show for which to retrieve all levels
        pub show: Option<String>,
        /// Optionally set the depth (show, seq, shot) for which to retrieve all
        /// mathcing levels
        pub depth: Option<u8>,
        /// Optionally specify an field to order the return by
        pub order_by: Option<String>,
    }

    impl Options {
        /// New up an instance of get_levels::Option
        pub fn new() -> Self {
            Self {
                level: None,
                show: None,
                depth: None,
                order_by: None,
            }
        }
        /// set an optional level
        ///
        /// # Example
        /// ```
        /// use packybara_grpc::client_service::get_levels;
        ///
        /// # fn dox() -> std::io::Result<()> {
        ///
        /// let options - get_levels::Options::new()
        ///                  .level_opt(Some("dev01.rd".into()));
        ///
        /// # Ok(())
        /// # }
        /// ```
        pub fn level_opt(mut self, level: Option<String>) -> Self {
            self.level = level;
            self
        }
        /// set an optional show
        ///
        /// # Example
        /// ```
        /// use packybara_grpc::client_service::get_levels;
        ///
        /// # fn dox() -> std::io::Result<()> {
        ///
        /// let options - get_levels::Options::new()
        ///                  .show_opt(Some("dev01".into()));
        ///
        /// # Ok(())
        /// # }
        /// ```
        pub fn show_opt(mut self, show: Option<String>) -> Self {
            self.show = show;
            self
        }
        /// set an optional depth. The depth determines what (show,sequence,shot) we are
        /// after
        ///
        ///
        /// # Example
        /// ```
        /// use packybara_grpc::client_service::get_levels;
        ///
        /// # fn dox() -> std::io::Result<()> {
        ///
        /// let options - get_levels::Options::new()
        ///                  .depth_opt(Some(2)));
        ///
        /// # Ok(())
        /// # }
        /// ```
        pub fn depth_opt(mut self, depth: Option<u8>) -> Self {
            self.depth = depth;
            self
        }
        /// Set an optional `order_by` parameter, dictating the field to
        /// order the return by, server size.
        ///
        /// # Example
        /// ```
        /// use packybara_grpc::client_service::get_levels;
        ///
        /// # fn dox() -> std::io::Result<()> {
        ///
        /// let options - get_levels::Options::new()
        ///                   .order_by_opt(Some("depth".into()));
        ///
        /// # Ok(())
        /// # }
        /// ```
        pub fn order_by_opt(mut self, order_by: Option<String>) -> Self {
            self.order_by = order_by;
            self
        }
    }
}
pub(super) mod get_levels_impl {
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

/// Provides parameters for the `ClientService.add_levels` method.
pub mod add_levels {
    /// Encapsulate the  parameter for adding levels
    pub struct Options {
        /// The levels one wants to create
        pub names: Vec<String>,
        /// The author of the change
        pub author: String,
        /// An optional comment
        pub comment: Option<String>,
    }

    impl Options {
        /// New up an instance of add_levels::Options given a name, order_by
        /// order_direction, and limit
        ///
        /// # Example
        /// ```
        /// use packybara_grpc::client_service::add_levels;
        ///
        /// # fn dox() -> std::io::Result<()> {
        /// let options = Options::new(vec!["dev01", "dev01.rd"], "jgerber");
        ///
        /// # Ok(())
        /// # }
        /// ```
        pub fn new<I>(names: Vec<I>, author: I) -> Self
        where
            I: Into<String>,
        {
            let names = names.into_iter().map(|n| n.into()).collect::<Vec<_>>();

            Self {
                names,
                author: author.into(),
                comment: None,
            }
        }

        /// Update comment with option wrapped type implementing
        /// Into<String>
        ///
        /// # Example
        /// ```
        /// use packybara_grpc::client_service::add_levels;
        ///
        /// # fn dox() -> std::io::Result<()> {
        /// let options = Options::new(vec!["dev01", "dev01.rd"], "jgerber")
        ///                        .comment_opt(Some("creates dev01 and the rd level"));
        ///
        /// # Ok(())
        /// # }
        /// ```
        pub fn comment_opt<I>(mut self, comment: Option<I>) -> Self
        where
            I: Into<String>,
        {
            let comment = comment.map(|c| c.into());
            self.comment = comment;
            self
        }
    }
}

pub(super) mod add_levels_impl {
    use super::*;
    use crate::{AddReply, LevelsAddRequest};
    pub async fn cmd(
        grpc_client: &mut ClientService,
        options: add_levels::Options,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        let add_levels::Options {
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
