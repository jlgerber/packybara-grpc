use super::*;

pub(crate) async fn get_sites(
    service: &PackybaraService,
    request: Request<SitesQueryRequest>,
) -> Result<Response<SitesQueryReply>, Status> {
    let mut pbd = PackratDb::new();
    // TODO: fix order_by in packybara or remove.
    let SitesQueryRequest { name } = request.into_inner();

    let client = service
        .client()
        .await
        .map_err(|e| Status::new(Code::Internal, format!("{}", e)))?;

    let results = pbd
        .find_all_sites()
        .name_opt(name.as_deref())
        .query(&client)
        .await
        .map_err(|e| Status::new(Code::Internal, format!("{}", e)))?;

    let mut names = Vec::new();
    for result in results {
        let FindAllSitesRow { name } = result;

        let reply = SitesQueryRow { name };
        names.push(reply);
    }
    Ok(Response::new(SitesQueryReply { names }))
}
