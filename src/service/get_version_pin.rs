//use super::PackybaraService;
use super::*;

pub(crate) async fn get_version_pin(
    service: &PackybaraService,
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
        .query(service.client())
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
