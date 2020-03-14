use tonic::{Request, Response, Status};

use packybara::coords::Coords as PCoords;
use packybara::db::find::versionpins::FindVersionPinsRow;
use packybara::db::traits::*;
use packybara::packrat::{Client, PackratDb};

use crate::{Coords, Packybara, VersionPinQueryReply, VersionPinQueryRequest};
#[derive(Debug)]
pub struct PackybaraService {
    client: Client,
}

impl PackybaraService {
    pub fn new(client: Client) -> Self {
        Self { client }
    }

    pub fn client(&self) -> &Client {
        &self.client
    }
}

#[tonic::async_trait]
impl Packybara for PackybaraService {
    async fn get_version_pin(
        &self,
        request: Request<VersionPinQueryRequest>, // Accept request of type HelloRequest
    ) -> Result<Response<VersionPinQueryReply>, Status> {
        // Return an instance of type HelloReply

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
        return Ok(Response::new(reply)); // Send back our formatted greeting
    }
}
