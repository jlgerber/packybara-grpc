use super::*;
use packybara::OrderDirection;
use packybara::OrderRoleBy;

pub(crate) async fn get_roles(
    service: &PackybaraService,
    request: Request<RolesQueryRequest>,
) -> Result<Response<RolesQueryReply>, Status> {
    let mut pbd = PackratDb::new();
    // TODO: fix order_by in packybara or remove.
    let RolesQueryRequest {
        role,
        category,
        order_by,
        order_direction,
        limit,
    } = request.into_inner();

    let order_by = order_by.map(|d| {
        d.split('.')
            .filter_map(|o| OrderRoleBy::from_str(o).ok())
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
        .find_all_roles()
        .role_opt(role.as_deref())
        .category_opt(category.as_deref())
        //.depth_opt(depth.map(|x| x as u8))
        .order_by_opt(order_by)
        .order_direction_opt(order_dir)
        .limit_opt(limit)
        .query(&client)
        .await
        .map_err(|e| Status::new(Code::Internal, format!("{}", e)))?;

    let mut roles = Vec::new();
    for result in results {
        let FindAllRolesRow { role, category } = result;

        let reply = RolesQueryRow { role, category };
        roles.push(reply);
    }
    Ok(Response::new(RolesQueryReply { roles }))
}
