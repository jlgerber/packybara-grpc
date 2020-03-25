use super::*;

pub(crate) async fn get_withs(
    service: &PackybaraService,
    request: Request<WithsQueryRequest>,
) -> Result<Response<WithsQueryReply>, Status> {
    let mut pbd = PackratDb::new();

    let WithsQueryRequest {
        package,
        level,
        role,
        platform,
        site,
        limit,
        order_by,
        order_direction,
    } = request.into_inner();

    let mut results = pbd.find_withs(&package);
    results
        .level_opt(level.as_deref())
        .role_opt(role.as_deref())
        .platform_opt(platform.as_deref())
        .limit_opt(limit)
        .site_opt(site.as_deref());
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

    let mut withs = Vec::new();
    for result in intermediate_results {
        let FindWithsRow {
            versionpin_id,
            distribution,
            coords:
                PCoords {
                    role,
                    level,
                    platform,
                    site,
                },
        } = result;

        let coords = Coords {
            level: level.to_string(),
            role: role.to_string(),
            platform: platform.to_string(),
            site: site.to_string(),
        };

        let reply = WithsQueryRow {
            versionpin_id: versionpin_id as i64,
            distribution: distribution.to_string(),
            coords,
        };
        withs.push(reply);
    }
    Ok(Response::new(WithsQueryReply { withs }))
}

pub(crate) async fn add_withs(
    mut client: Client,
    request: Request<WithsAddRequest>,
) -> Result<Response<AddReply>, Status> {
    let pbd = PackratDb::new();

    let mut tx = pbd
        .transaction(&mut client)
        .await
        .map_err(|e| Status::new(Code::Internal, format!("{}", e)))?;
    let WithsAddRequest {
        vpin_id,
        withs,
        author,
        comment,
    } = request.into_inner();
    let comment = comment.unwrap_or("Auto Comment - Add Sites".to_string());
    let results = PackratDb::add_withs()
        .create(&mut tx, vpin_id as i32, withs)
        .await
        .map_err(|e| Status::new(Code::Internal, format!("{}", e)))?
        .commit(&author, &comment, tx)
        .await
        .map_err(|e| Status::new(Code::Internal, format!("{}", e)))?;

    Ok(Response::new(AddReply { updates: results }))
}
