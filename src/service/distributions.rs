use super::*;
use packybara::OrderDirection;
use std::str::FromStr;

pub(crate) async fn get_distributions(
    service: &PackybaraService,
    request: Request<DistributionsQueryRequest>,
) -> Result<Response<DistributionsQueryReply>, Status> {
    let mut pbd = PackratDb::new();
    // TODO: fix order_by in packybara or remove.
    let DistributionsQueryRequest {
        package,
        version,
        order_direction,
    } = request.into_inner();
    let direction = order_direction
        .as_ref()
        .map(|x| OrderDirection::from_str(x).ok())
        .flatten();

    let client = service
        .client()
        .await
        .map_err(|e| Status::new(Code::Internal, format!("{}", e)))?;

    let results = pbd
        .find_all_distributions()
        .package_opt(package.as_deref())
        .version_opt(version.as_deref())
        .order_direction_opt(direction)
        .query(&client)
        .await
        .map_err(|e| Status::new(Code::Internal, format!("{}", e)))?;

    let mut distributions = Vec::new();
    for result in results {
        let FindAllDistributionsRow {
            id,
            package,
            version,
        } = result;

        let reply = DistributionsQueryRow {
            id: id as i64,
            package,
            version,
        };
        distributions.push(reply);
    }
    Ok(Response::new(DistributionsQueryReply { distributions }))
}
