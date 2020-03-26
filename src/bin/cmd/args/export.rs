use structopt::StructOpt;

#[derive(StructOpt, Debug, PartialEq)]
#[structopt(about = "PackybaraDb Export")]
pub enum PbExport {
    /// Export a show's state to a packages.xml file
    #[structopt(display_order = 1, name = "packages.xml")]
    PackagesXml {
        /// Set the show to export a packages.xml file from.
        #[structopt(name = "SHOW")]
        show: String,
        /// Export path to the packages.xml file
        #[structopt(name = "PATH")]
        path: String,
    },
}
