//! # Introduction
//!
//! The packybara-grpc crate provides an async gRPC api for packybara with a postgres connection
//! pool. The client provides a very similar experience to the non-gRPC api, returning the same data
//! types as packybara.
//!
//! Packybara-grpc uses best-of-breed crates to provide a good performing implementation which should be able to
//! sustain top throughput via async and connection pooling. If necessary, one may also mate the server
//! with an independent connection pool like pgbouncer or pgpooler, although I have not yet done any
//! tests.
//!
//! For gRPC, I use [Tonic](https://github.com/hyperium/tonic), which relies on `prost` for the protocol
//! buffer implementation, `hyper` for http2, and `tokio` for the async implementation.
pub use pb::packybara_client::PackybaraClient;
pub use pb::packybara_server::{Packybara, PackybaraServer};
pub use pb::{
    AddReply, LevelsAddRequest, PackagesAddReply, PackagesAddRequest, PlatformsAddRequest,
    RolesAddRequest, SitesAddRequest, VersionPinsAddRequest, WithsAddRequest,
};
pub use pb::{
    ChangesQueryReply, ChangesQueryRequest, ChangesQueryRow, Coords, DistributionsQueryReply,
    DistributionsQueryRequest, DistributionsQueryRow, LevelsQueryReply, LevelsQueryRequest,
    LevelsQueryRow, PackagesQueryReply, PackagesQueryRequest, PackagesQueryRow,
    PkgCoordsQueryReply, PkgCoordsQueryRequest, PkgCoordsQueryRow, PlatformsQueryReply,
    PlatformsQueryRequest, PlatformsQueryRow, RevisionsQueryReply, RevisionsQueryRequest,
    RevisionsQueryRow, RolesQueryReply, RolesQueryRequest, RolesQueryRow, SitesQueryReply,
    SitesQueryRequest, SitesQueryRow, VersionPinQueryReply, VersionPinQueryRequest,
    VersionPinWithsQueryReply, VersionPinWithsQueryRequest, VersionPinWithsQueryRow,
    VersionPinsQueryReply, VersionPinsQueryRequest, VersionPinsQueryRow, WithsQueryReply,
    WithsQueryRequest, WithsQueryRow,
};
pub mod pb {
    tonic::include_proto!("packybara");
}

pub mod service;
pub use service::PackybaraService;
pub mod client_service;
pub use client_service::ClientService;
pub mod database_config;
pub mod url;
pub mod url_builder;
pub mod utils;
pub use database_config::DatabaseConfig;
