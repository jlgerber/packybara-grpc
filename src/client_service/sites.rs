//! `sites` module exports `get_sites::Options` providing query parameters for `ClientService.get_sites`
//! and `add_sites::Options` providing parameters for `ClientService.add_sites`
use super::*;

/// Exports `Options`, used to set query parameters for `ClientService.get_sites`
pub mod get_sites {
    /// Encapsulate the query parameters
    pub struct Options {
        pub name: Option<String>,
    }

    impl Options {
        /// New up an instance of get_sites::Options.
        ///
        /// # Example
        /// ```
        /// use packybara_grpc::get_sites;
        ///
        /// # fn dox() -> std::io::Result<()> {
        ///
        /// let options = get_sites::Options::new();
        /// # Ok(())
        /// # }
        /// ```
        pub fn new() -> Self {
            Self { name: None }
        }

        /// Set the optional `name` parameter to either `None` or `Some(value)`
        ///
        /// # Example
        /// ```
        /// use packybara_grpc::get_sites;
        ///
        /// # fn dox() -> std::io::Result<()> {
        ///
        /// let options = get_sites::Options::new().name_opt(Some("porltand"));
        /// # Ok(())
        /// # }
        /// ```
        pub fn name_opt<I>(mut self, name: Option<I>) -> Self
        where
            I: Into<String>,
        {
            self.name = name.map(|n| n.into());
            self
        }
    }
}
pub(crate) mod get_sites_impl {
    use super::*;
    /// Given a mutable reference to the ClientService instance and
    /// query parameters defined by `get_sites::Options`, retrieve all matching
    /// sites from pakcybara
    pub async fn cmd(
        grpc_client: &mut ClientService,
        options: get_sites::Options,
    ) -> Result<Vec<FindAllSitesRow>, Box<dyn std::error::Error>> {
        let get_sites::Options { name } = options;
        let request = tonic::Request::new(SitesQueryRequest { name });
        let response = grpc_client.client.get_sites(request).await?;
        let SitesQueryReply { names } = response.into_inner();

        let results = names
            .into_iter()
            .map(|site| {
                let SitesQueryRow { name } = site;
                FindAllSitesRow::from_parts(&name)
            })
            .collect::<Vec<_>>();

        Ok(results)
    }
}

/// Exports `Options`, used to set  parameters for `ClientService.add_sites`
pub mod add_sites {
    /// Encapsulate the parameters for adding sites
    pub struct Options {
        pub names: Vec<String>,
        pub author: String,
        pub comment: Option<String>,
    }

    impl Options {
        /// New up an instance of add_sites::Options given a vector of names and the author
        ///
        /// # Example
        /// ```
        /// use packybara_grpc::get_sites;
        ///
        /// # fn dox() -> std::io::Result<()> {
        ///
        /// let options = get_sites::Options::new(vec!["portland", "hyderabad"], "jgerber");
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

        /// Update comment with an optional value
        ///
        /// # Example
        /// ```
        /// use packybara_grpc::get_sites;
        ///
        /// # fn dox() -> std::io::Result<()> {
        ///
        /// let options = get_sites::Options::new(vec!["portland", "hyderabad"], "jgerber")
        ///                             .comment_opt("adds portland and hyderabad");
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
pub(crate) mod add_sites_impl {
    use super::*;
    use crate::{AddReply, SitesAddRequest};
    pub(crate) async fn cmd(
        grpc_client: &mut ClientService,
        options: add_sites::Options,
    ) -> Result<u64, Box<dyn std::error::Error>> {
        let add_sites::Options {
            names,
            author,
            comment,
        } = options;
        let request = tonic::Request::new(SitesAddRequest {
            names,
            author,
            comment,
        });
        let response = grpc_client.client.add_sites(request).await?;
        let AddReply { updates } = response.into_inner();

        Ok(updates)
    }
}
