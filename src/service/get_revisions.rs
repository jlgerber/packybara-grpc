use super::*;
use packybara::OrderDirection;
use packybara::OrderRevisionBy;

pub(crate) async fn get_revisions(
    service: &PackybaraService,
    request: Request<RevisionsQueryRequest>,
) -> Result<Response<RevisionsQueryReply>, Status> {
    let mut pbd = PackratDb::new();
    // TODO: fix order_by in packybara or remove.
    let RevisionsQueryRequest {
        id,
        transaction_id,
        author,
        order_by,
        order_direction,
        limit,
    } = request.into_inner();

    let order_by = order_by.map(|d| {
        d.split('.')
            .filter_map(|o| OrderRevisionBy::from_str(o).ok())
            .collect::<Vec<_>>()
    });
    let order_dir = order_direction
        .map(|v| OrderDirection::from_str(&v).ok())
        .flatten();
    let results = pbd
        .find_all_revisions()
        .id_opt(id.map(|x| x as i32))
        .transaction_id_opt(transaction_id)
        .author_opt(author.as_deref())
        .order_by_opt(order_by)
        .order_direction_opt(order_dir)
        .limit_opt(limit)
        .query(service.client())
        .await
        .map_err(|e| Status::new(Code::Internal, format!("{}", e)))?;

    let mut revisions = Vec::new();
    for result in results {
        let FindAllRevisionsRow {
            id,
            transaction_id,
            author,
            comment,
            datetime,
        } = result;

        let reply = RevisionsQueryRow {
            id: id as i64,
            transaction_id,
            author,
            comment,
            datetime: format!("{}", datetime),
        };
        revisions.push(reply);
    }
    Ok(Response::new(RevisionsQueryReply { revisions }))
}
