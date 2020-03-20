use super::*;
use packybara::LtreeSearchMode;
use packybara::SearchMode;

pub(crate) async fn get_pkgcoords(
    service: &PackybaraService,
    request: Request<PkgCoordsQueryRequest>,
) -> Result<Response<PkgCoordsQueryReply>, Status> {
    let mut pbd = PackratDb::new();
    // TODO: fix order_by in packybara or remove.
    let PkgCoordsQueryRequest {
        package,
        level,
        role,
        platform,
        site,
        search_mode,
        order_by,
    } = request.into_inner();

    let search_mode = search_mode
        .map(|v| SearchMode::try_from_str(&v).ok())
        .flatten();

    // let order_by = order_by.map(|d| {
    //     d.split('.')
    //         .filter_map(|o| OrderPkgCoordsBy::from_str(o).ok())
    //         .collect::<Vec<_>>()
    // });
    let results = pbd
        .find_pkgcoords()
        .package_opt(package.as_deref())
        .level_opt(level.as_deref())
        .role_opt(role.as_deref())
        .platform_opt(platform.as_deref())
        .site_opt(site.as_deref())
        .search_mode(search_mode.unwrap_or(SearchMode::Ltree(LtreeSearchMode::Ancestor)))
        .order_by_opt(order_by.as_deref())
        .query(service.client())
        .await
        .map_err(|e| Status::new(Code::Internal, format!("{}", e)))?;

    let mut pkgcoords = Vec::new();
    for result in results {
        let FindAllPkgCoordsRow {
            id,
            package,
            level,
            role,
            platform,
            site,
        } = result;

        let reply = PkgCoordsQueryRow {
            id: id as i64,
            package,
            level,
            role,
            platform,
            site,
        };
        pkgcoords.push(reply);
    }
    Ok(Response::new(PkgCoordsQueryReply { pkgcoords }))
}
