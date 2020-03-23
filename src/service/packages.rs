use super::*;

pub(crate) async fn get_packages(
    service: &PackybaraService,
    request: Request<PackagesQueryRequest>,
) -> Result<Response<PackagesQueryReply>, Status> {
    let mut pbd = PackratDb::new();
    // TODO: fix order_by in packybara or remove.
    let PackagesQueryRequest { name } = request.into_inner();

    let client = service
        .client()
        .await
        .map_err(|e| Status::new(Code::Internal, format!("{}", e)))?;

    let results = pbd
        .find_all_packages()
        .name_opt(name.as_deref())
        .query(&client)
        .await
        .map_err(|e| Status::new(Code::Internal, format!("{}", e)))?;

    let mut names = Vec::new();
    for result in results {
        let FindAllPackagesRow { name } = result;

        let reply = PackagesQueryRow { name };
        names.push(reply);
    }
    Ok(Response::new(PackagesQueryReply { names }))
}

pub(crate) async fn add_packages(
    mut client: Client,
    request: Request<PackagesAddRequest>,
) -> Result<Response<PackagesAddReply>, Status> {
    let pbd = PackratDb::new();

    let mut tx = pbd
        .transaction(&mut client)
        .await
        .map_err(|e| Status::new(Code::Internal, format!("{}", e)))?;
    let PackagesAddRequest {
        mut names,
        author,
        comment,
    } = request.into_inner();
    let comment = comment.unwrap_or("Auto Comment - Add Packages".to_string());
    let results = PackratDb::add_packages()
        .packages(&mut names)
        .create(&mut tx)
        .await
        .map_err(|e| Status::new(Code::Internal, format!("{}", e)))?
        .commit(&author, &comment, tx)
        .await
        .map_err(|e| Status::new(Code::Internal, format!("{}", e)))?;

    Ok(Response::new(PackagesAddReply { updates: results }))
}
