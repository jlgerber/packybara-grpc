use packybara::types::IdType;
use structopt::StructOpt;

#[derive(StructOpt, Debug, PartialEq)]
#[structopt(about = "PackybaraDb Add")]
pub enum PbAdd {
    /// Add one or more packages to the database.
    #[structopt(display_order = 1, name = "packages")]
    Packages {
        /// list of packages
        #[structopt(name = "PACKAGES")]
        names: Vec<String>,
        /// add a comment
        #[structopt(short, long = "versionpin-id", display_order = 1)]
        comment: Option<String>,
    },
    /// Add one or more levels to the database.
    #[structopt(display_order = 2, name = "levels")]
    Levels {
        #[structopt(name = "LEVELS")]
        names: Vec<String>,
        /// add a comment
        #[structopt(short, long = "versionpin-id", display_order = 1)]
        comment: Option<String>,
    },
    /// Add one or more roles to the database.
    #[structopt(display_order = 3, name = "roles")]
    Roles {
        #[structopt(name = "ROLES")]
        names: Vec<String>,
        /// add a comment
        #[structopt(short, long = "versionpin-id", display_order = 1)]
        comment: Option<String>,
    },
    /// Add one or more roles to the database.
    #[structopt(display_order = 4, name = "platforms")]
    Platforms {
        #[structopt(name = "PLATFORMS")]
        names: Vec<String>,
        /// add a comment
        #[structopt(short, long = "versionpin-id", display_order = 1)]
        comment: Option<String>,
    },
    /// Add one or more roles to the database.
    #[structopt(display_order = 5, name = "withs")]
    Withs {
        #[structopt(short, long = "versionpin-id", display_order = 1)]
        vpin_id: IdType,
        #[structopt(short, long, display_order = 2)]
        comment: String,
        #[structopt(name = "WITHS")]
        withs: Vec<String>,
        /// add a comment
        #[structopt(short, long = "versionpin-id", display_order = 1)]
        comment: Option<String>,
    },
    #[structopt(display_order = 6, name = "versionpins")]
    VersionPins {
        #[structopt(short, long = "distribution", display_order = 1)]
        distribution: String,
        #[structopt(short, long, display_order = 2)]
        level: Option<String>,
        #[structopt(short, long, display_order = 3)]
        role: Option<String>,
        #[structopt(short, long, display_order = 4)]
        platform: Option<String>,
        #[structopt(short, long, display_order = 5)]
        site: Option<String>,
        /// add a comment
        #[structopt(short, long = "versionpin-id", display_order = 1)]
        comment: Option<String>,
    },
}
