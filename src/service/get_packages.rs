use super::*;

pub(crate) async fn get_packages(
    service: &PackybaraService,
    request: Request<PackagesQueryRequest>,
) -> Result<Response<PackagesQueryReply>, Status> {
    let mut pbd = PackratDb::new();
    // TODO: fix order_by in packybara or remove.
    let PackagesQueryRequest { name } = request.into_inner();

    let results = pbd
        .find_all_packages()
        .name_opt(name.as_deref())
        .query(service.client())
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
