//! `packages_xml` module exports the `export_packagesxml::Options` struct, used to set options for
//! the `ClientService.export_packagesxml` method.
use super::*;

/// Exports the `Options` struct, used to set parameters for the `ClientService.export_packgesxml` method.
pub mod export_packagesxml {
    /// Encapsulate the query parameter for adding platforms
    pub struct Options {
        pub show: String,
        pub path: String,
    }

    impl Options {
        /// New up an instance of add_platforms::Options given a name, order_by
        /// order_direction, and limit.
        ///
        /// # Example
        /// ```
        /// # fn dox() -> std::io::Result<()> {
        /// use packybara_grpc::client_service::export_packagesxml;
        ///
        /// let options = export_packagesxml::Options::new("dev01", "/var/tmp/packages.xml");
        ///
        /// # Ok(())
        /// # }
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
}
pub mod export_packagesxml_impl {
    use super::*;
    use crate::{PackagesXmlExportReply, PackagesXmlExportRequest};
    pub(crate) async fn cmd(
        grpc_client: &mut ClientService,
        options: export_packagesxml::Options,
    ) -> Result<bool, Box<dyn std::error::Error>> {
        let export_packagesxml::Options { show, path } = options;
        let request = tonic::Request::new(PackagesXmlExportRequest { show, path });
        let response = grpc_client.client.export_packages(request).await?;
        let PackagesXmlExportReply { result } = response.into_inner();

        Ok(result)
    }
}
