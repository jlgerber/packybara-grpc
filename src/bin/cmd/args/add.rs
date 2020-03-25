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
    /// Add one or more platforms to the database.
    #[structopt(display_order = 4, name = "platforms")]
    Platforms {
        #[structopt(name = "PLATFORMS")]
        names: Vec<String>,
        /// add a comment
        #[structopt(short, long = "versionpin-id", display_order = 1)]
        comment: Option<String>,
    },
    /// Add one or more Sites to the database.
    #[structopt(display_order = 5, name = "sites")]
    Sites {
        #[structopt(name = "SITES")]
        names: Vec<String>,
        /// add a comment
        #[structopt(short, long = "versionpin-id", display_order = 1)]
        comment: Option<String>,
    },
    /// Add one or more roles to the database.
    #[structopt(display_order = 6, name = "withs")]
    Withs {
        /// set the version pin id of the distribution to associate withs with
        #[structopt(short, long = "versionpin-id", display_order = 1)]
        vpin_id: IdType,
        /// add a comment
        #[structopt(short, long, display_order = 2)]
        comment: Option<String>,
        /// Provide one or more package names as withs
        #[structopt(name = "WITHS")]
        withs: Vec<String>,
    },
    #[structopt(display_order = 7, name = "version-pins")]
    VersionPins {
        #[structopt(short, long = "distribution", display_order = 1)]
        distribution: String,
        /// set one or more levels
        #[structopt(short, long, display_order = 2)]
        levels: Vec<String>,
        /// set one or more roles
        #[structopt(short, long, display_order = 3)]
        roles: Vec<String>,
        /// Set one or more platforms
        #[structopt(short, long, display_order = 4)]
        platforms: Vec<String>,
        /// set one or more sites
        #[structopt(short, long, display_order = 5)]
        sites: Vec<String>,
        /// add a comment
        #[structopt(short, long = "versionpin-id", display_order = 1)]
        comment: Option<String>,
    },
}
