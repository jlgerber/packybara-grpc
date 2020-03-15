pub use pb::packybara_client::PackybaraClient;
pub use pb::packybara_server::{Packybara, PackybaraServer};
pub use pb::{Coords, VersionPinQueryReply, VersionPinQueryRequest};

pub mod pb {
    tonic::include_proto!("packybara");
}

pub mod service;
pub use service::PackybaraService;
pub mod client;
pub mod url_builder;
