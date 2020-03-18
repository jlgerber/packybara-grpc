use log;
use packybara::coords::Coords as PCoords;
use packybara::db::find::versionpins::FindVersionPinsRow;
use packybara::db::find_all::versionpins::FindAllVersionPinsRow;
use packybara::db::traits::*;
use packybara::packrat::{Client, PackratDb};
use packybara::LtreeSearchMode;
use packybara::{OrderDirection, SearchAttribute};
use std::str::FromStr;
use tokio_postgres::NoTls;
use tonic::transport::Server;
use tonic::{Code, Request, Response, Status};

use crate::{
    url::GrpcUrl, Coords, Packybara, PackybaraServer, VersionPinQueryReply, VersionPinQueryRequest,
    VersionPinsQueryReply, VersionPinsQueryRequest, VersionPinsQueryRow,
};
#[derive(Debug)]
pub struct PackybaraService {
    client: Client,
}

impl PackybaraService {
    pub fn new(client: Client) -> Self {
        Self { client }
    }
    // TODO:: Add configuration as run argument
    /// Run the server as a service.
    ///
    /// # Examples
    /// ```
    /// use tokio;
    /// use packybara_grpc::PackybaraService;
    ///
    /// #[tokio::main]
    /// async fn main() -> Result<(), Box<dyn std::error::Error>> {
    ///     PackybaraService::run().await?;
    ///     Ok(())
    /// }
    /// ```
    pub async fn run(url: GrpcUrl) -> Result<(), Box<dyn std::error::Error>> {
        let (client, connection) = tokio_postgres::connect(
            "host=127.0.0.1 user=postgres  dbname=packrat password=example port=5432",
            NoTls,
        )
        .await?;
        tokio::spawn(async move {
            if let Err(e) = connection.await {
                eprintln!("connection error: {}", e);
            }
        });
        let addr = url.to_socket_addr()?; //"[::1]:50051".parse()?;
        let packy = PackybaraService::new(client);
        Server::builder()
            .add_service(PackybaraServer::new(packy))
            .serve(addr)
            .await?;

        Ok(())
    }

    pub fn client(&self) -> &Client {
        &self.client
    }
}

#[tonic::async_trait]
impl Packybara for PackybaraService {
    async fn get_version_pin(
        &self,
        request: Request<VersionPinQueryRequest>,
    ) -> Result<Response<VersionPinQueryReply>, Status> {
        let mut pbd = PackratDb::new();
        let msg = request.get_ref();
        let result = pbd
            .find_versionpin(msg.package.as_str())
            .level(msg.level.as_deref().unwrap_or("facility"))
            .role(msg.role.as_deref().unwrap_or("any"))
            .platform(msg.platform.as_deref().unwrap_or("any"))
            .site(msg.site.as_deref().unwrap_or("any"))
            .query(self.client())
            .await
            .unwrap();

        let FindVersionPinsRow {
            versionpin_id,
            distribution,
            coords:
                PCoords {
                    role,
                    level,
                    platform,
                    site,
                },
            withs,
        } = result;

        let coords = Coords {
            level: level.to_string(),
            role: role.to_string(),
            platform: platform.to_string(),
            site: site.to_string(),
        };
        let reply = VersionPinQueryReply {
            versionpin_id: versionpin_id as i64,
            distribution: distribution.to_string(),
            coords,
            withs: withs
                .unwrap_or(Vec::new())
                .iter()
                .map(|x| x.to_string())
                .collect::<Vec<_>>(),
        };
        return Ok(Response::new(reply));
    }

    async fn get_version_pins(
        &self,
        request: Request<VersionPinsQueryRequest>,
    ) -> Result<Response<VersionPinsQueryReply>, Status> {
        let mut pbd = PackratDb::new();

        let VersionPinsQueryRequest {
            package,
            version,
            level,
            role,
            platform,
            site,
            isolate_facility,
            search_mode,
            order_by,
            order_direction,
            full_withs,
            limit,
        } = request.into_inner();

        let (level, role, platform, site, mode) =
            extract_coords(level, role, platform, site, search_mode);

        let mut results = pbd.find_all_versionpins();
        results
            .some_package(package.as_deref())
            .some_version(version.as_deref())
            .level(level.as_str())
            .isolate_facility(isolate_facility.unwrap_or(false))
            .role(role.as_str())
            .platform(platform.as_str())
            .site(site.as_str())
            .search_mode(
                LtreeSearchMode::from_str(mode.as_str())
                    .map_err(|e| Status::new(Code::Internal, format!("{}", e)))?,
            );
        if let Some(ref order) = order_by {
            let orders = order
                .split(",")
                .map(|x| SearchAttribute::from_str(x).unwrap_or(SearchAttribute::Unknown))
                .collect::<Vec<SearchAttribute>>();
            results.order_by(orders);
        }
        if let Some(ref dir) = order_direction {
            let direction = OrderDirection::from_str(dir);
            if direction.is_ok() {
                let direction = direction.unwrap();
                results.order_direction(direction);
            } else {
                log::warn!("unable to apply search direction request {} to query", dir);
            }
        }
        let intermediate_results = results
            .query(self.client())
            .await
            .map_err(|x| Status::new(Code::Internal, format!("{}", x)))?;
        let mut vpins = Vec::new();
        for result in intermediate_results {
            let FindAllVersionPinsRow {
                versionpin_id,
                distribution_id,
                pkgcoord_id,
                distribution,
                coords:
                    PCoords {
                        role,
                        level,
                        platform,
                        site,
                    },
                withs,
            } = result;

            let coords = Coords {
                level: level.to_string(),
                role: role.to_string(),
                platform: platform.to_string(),
                site: site.to_string(),
            };

            let reply = VersionPinsQueryRow {
                versionpin_id: versionpin_id as i64,
                distribution_id: distribution_id as i64,
                pkgcoord_id: pkgcoord_id as i64,
                distribution: distribution.to_string(),
                coords,
                withs: withs
                    .unwrap_or(Vec::new())
                    .iter()
                    .map(|x| x.to_string())
                    .collect::<Vec<_>>(),
            };
            vpins.push(reply);
        }
        Ok(Response::new(VersionPinsQueryReply { vpins }))
    }
}

/// Build a tuple of coordinates given a their components as Options.
/// This takes care of default initialization
///
/// # Arguments
/// * `level` - A reference to an Option wrapped string
/// * `role` - A reference to an Option wrapped string
/// * `platform` - A reference to an Option wrapped string
/// * `site` - A reference to an Option wrapped String
/// * `mode` - A reference to an Option wrapped string
///
/// # Returns
/// * tuple of strings (level, role, platform, site, mode)
pub fn extract_coords(
    level: Option<String>,
    role: Option<String>,
    platform: Option<String>,
    site: Option<String>,
    mode: Option<String>,
) -> (String, String, String, String, String) {
    let r = role.unwrap_or("any".to_string());
    let l = level.unwrap_or("facility".to_string());
    let p = platform.unwrap_or("any".to_string());
    let s = site.unwrap_or("any".to_string());
    let m = mode.unwrap_or("ancestor".to_string());

    (l, r, p, s, m)
}
