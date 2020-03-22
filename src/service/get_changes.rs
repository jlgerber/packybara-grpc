use super::*;

pub(crate) async fn get_changes(
    service: &PackybaraService,
    request: Request<ChangesQueryRequest>,
) -> Result<Response<ChangesQueryReply>, Status> {
    let mut pbd = PackratDb::new();
    // TODO: fix order_by in packybara or remove.
    let ChangesQueryRequest { transaction_id } = request.into_inner();
    let results = pbd
        .find_all_changes()
        .transaction_id_opt(transaction_id)
        .query(service.client())
        .await
        .map_err(|e| Status::new(Code::Internal, format!("{}", e)))?;

    let mut changes = Vec::new();
    for result in results {
        let FindAllChangesRow {
            id,
            transaction_id,
            action,
            level,
            role,
            platform,
            site,
            package,
            old,
            new,
        } = result;

        let coords = Coords {
            level: level.to_string(),
            role: role.to_string(),
            platform: platform.to_string(),
            site: site.to_string(),
        };
        let reply = ChangesQueryRow {
            id: id as i64,
            transaction_id: transaction_id as i64,
            action: action.to_string(),
            coords,
            package,
            old: old.map(|x| x.distribution().to_string()),
            new: new.distribution().to_string(),
        };
        changes.push(reply);
    }
    Ok(Response::new(ChangesQueryReply { changes }))
}
