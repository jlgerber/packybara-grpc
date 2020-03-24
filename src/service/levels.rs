use super::*;

pub(crate) async fn get_levels(
    service: &PackybaraService,
    request: Request<LevelsQueryRequest>,
) -> Result<Response<LevelsQueryReply>, Status> {
    let mut pbd = PackratDb::new();
    // TODO: fix order_by in packybara or remove.
    let LevelsQueryRequest {
        level,
        show,
        depth,
        order_by,
    } = request.into_inner();

    let client = service
        .client()
        .await
        .map_err(|e| Status::new(Code::Internal, format!("{}", e)))?;

    let results = pbd
        .find_all_levels()
        .level_opt(level.as_deref())
        .show_opt(show.as_deref())
        .depth_opt(depth.map(|x| x as u8))
        //.order_by_opt(order_by)
        .query(&client)
        .await
        .map_err(|e| Status::new(Code::Internal, format!("{}", e)))?;

    let mut levels = Vec::new();
    for result in results {
        let FindAllLevelsRow { level, show } = result;

        let reply = LevelsQueryRow { level, show };
        levels.push(reply);
    }
    Ok(Response::new(LevelsQueryReply { levels }))
}

pub(crate) async fn add_levels(
    mut client: Client,
    request: Request<LevelsAddRequest>,
) -> Result<Response<AddReply>, Status> {
    let pbd = PackratDb::new();

    let mut tx = pbd
        .transaction(&mut client)
        .await
        .map_err(|e| Status::new(Code::Internal, format!("{}", e)))?;
    let LevelsAddRequest {
        mut names,
        author,
        comment,
    } = request.into_inner();
    let comment = comment.unwrap_or("Auto Comment - Add Packages".to_string());
    let results = PackratDb::add_levels()
        .levels(&mut names)
        .create(&mut tx)
        .await
        .map_err(|e| Status::new(Code::Internal, format!("{}", e)))?
        .commit(&author, &comment, tx)
        .await
        .map_err(|e| Status::new(Code::Internal, format!("{}", e)))?;

    Ok(Response::new(AddReply { updates: results }))
}
