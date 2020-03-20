use super::*;

pub(crate) async fn get_version_pin_withs(
    service: &PackybaraService,
    request: Request<VersionPinWithsQueryRequest>,
) -> Result<Response<VersionPinWithsQueryReply>, Status> {
    let mut pbd = PackratDb::new();

    let VersionPinWithsQueryRequest { versionpin_id } = request.into_inner();

    let results = pbd
        .find_all_versionpin_withs(versionpin_id as i32)
        .query(service.client())
        .await
        .map_err(|e| Status::new(Code::Internal, format!("{}", e)))?;

    let mut withs = Vec::new();
    for result in results {
        let FindAllWithsRow {
            id,
            vpin_id,
            with,
            order,
        } = result;

        let reply = VersionPinWithsQueryRow {
            id: id as i64,
            vpin_id: vpin_id as i64,
            with,
            order: order as i64,
        };
        withs.push(reply);
    }
    Ok(Response::new(VersionPinWithsQueryReply { withs }))
}
