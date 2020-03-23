use super::*;
use crate::utils::extract_coords;

pub(crate) async fn get_version_pins(
    service: &PackybaraService,
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

    let client = service
        .client()
        .await
        .map_err(|e| Status::new(Code::Internal, format!("{}", e)))?;
    let intermediate_results = results
        .query(&client)
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
