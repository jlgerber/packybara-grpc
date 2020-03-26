use super::*;

pub mod export_packagesxml {
    /// Encapsulate the query parameter for adding platforms
    pub struct Options {
        pub show: String,
        pub path: String,
    }

    impl Options {
        /// New up an instance of add_platforms::Options given a name, order_by
        /// order_direction, and limit
        ///
        /// # Arguments
        ///
        /// * `show` - vector of platforms names
        /// * `path` - name of the person who authored the new platforms
        ///
        /// # Returns
        ///
        /// * Option instance
        pub fn new<I>(show: I, path: I) -> Self
        where
            I: Into<String>,
        {
            Self {
                show: show.into(),
                path: path.into(),
            }
        }
    }

    use super::*;
    use crate::{PackagesXmlExportReply, PackagesXmlExportRequest};
    pub async fn cmd(
        grpc_client: &mut ClientService,
        options: Options,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let Options { show, path } = options;
        let request = tonic::Request::new(PackagesXmlExportRequest { show, path });
        let response = grpc_client.client.export_packages(request).await?;
        let PackagesXmlExportReply { result } = response.into_inner();

        Ok(result)
    }
}
