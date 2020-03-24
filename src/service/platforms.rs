use super::*;
use packybara::OrderDirection;
use packybara::OrderPlatformBy;

pub(crate) async fn get_platforms(
    service: &PackybaraService,
    request: Request<PlatformsQueryRequest>,
) -> Result<Response<PlatformsQueryReply>, Status> {
    let mut pbd = PackratDb::new();
    // TODO: fix order_by in packybara or remove.
    let PlatformsQueryRequest {
        name,
        order_by,
        order_direction,
        limit,
    } = request.into_inner();

    let order_by = order_by.map(|d| {
        d.split('.')
            .filter_map(|o| OrderPlatformBy::from_str(o).ok())
            .collect::<Vec<_>>()
    });
    let order_dir = order_direction
        .map(|v| OrderDirection::from_str(&v).ok())
        .flatten();
    let client = service
        .client()
        .await
        .map_err(|e| Status::new(Code::Internal, format!("{}", e)))?;
    let results = pbd
        .find_all_platforms()
        .name_opt(name.as_deref())
        .order_by_opt(order_by)
        .order_direction_opt(order_dir)
        .limit_opt(limit)
        .query(&client)
        .await
        .map_err(|e| Status::new(Code::Internal, format!("{}", e)))?;

    let mut names = Vec::new();
    for result in results {
        let FindAllPlatformsRow { name } = result;

        let reply = PlatformsQueryRow { name };
        names.push(reply);
    }
    Ok(Response::new(PlatformsQueryReply { names }))
}

pub(crate) async fn add_platforms(
    mut client: Client,
    request: Request<PlatformsAddRequest>,
) -> Result<Response<AddReply>, Status> {
    let pbd = PackratDb::new();

    let mut tx = pbd
        .transaction(&mut client)
        .await
        .map_err(|e| Status::new(Code::Internal, format!("{}", e)))?;
    let PlatformsAddRequest {
        mut names,
        author,
        comment,
    } = request.into_inner();
    let comment = comment.unwrap_or("Auto Comment - Add Packages".to_string());
    let results = PackratDb::add_platforms()
        .platforms(&mut names)
        .create(&mut tx)
        .await
        .map_err(|e| Status::new(Code::Internal, format!("{}", e)))?
        .commit(&author, &comment, tx)
        .await
        .map_err(|e| Status::new(Code::Internal, format!("{}", e)))?;

    Ok(Response::new(AddReply { updates: results }))
}
