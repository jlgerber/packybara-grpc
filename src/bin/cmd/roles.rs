use super::args::add::PbAdd;
use super::args::find::PbFind;
use packybara_grpc::client_service as pbclient;
use packybara_grpc::client_service::ClientService;
//use packybara_grpc::utils::truncate;
use prettytable::{cell, format, row, table};

pub(crate) async fn find(
    mut client: ClientService,
    cmd: PbFind,
) -> Result<(), Box<dyn std::error::Error>> {
    if let PbFind::Roles {
        role,
        category,
        order_by,
        order_direction,
        limit,
    } = cmd
    {
        let results = client
            .get_roles(
                pbclient::get_roles::Options::new()
                    .role_opt(role)
                    .category_opt(category)
                    .order_by_opt(order_by)
                    .order_direction_opt(order_direction)
                    .limit_opt(limit),
            )
            .await?;
        let mut table = table!([bFg => "ROLE", "CATEGORY"]);
        for result in results {
            table.add_row(row![result.role, result.category]);
        }
        table.set_format(*format::consts::FORMAT_CLEAN); //FORMAT_NO_LINESEP_WITH_TITLE  FORMAT_NO_BORDER_LINE_SEPARATOR
        table.printstd();
    }
    Ok(())
}

pub(crate) async fn add(
    mut client: ClientService,
    cmd: PbAdd,
) -> Result<u64, Box<dyn std::error::Error>> {
    if let PbAdd::Roles { names, comment } = cmd {
        let username = whoami::username();
        let opts = pbclient::add_roles::Options::new(names, username).comment_opt(Some(
            comment.unwrap_or("Auto Comment - Levels Added".to_string()),
        ));

        let results = client.add_roles(opts).await?;

        println!("{}", results);
        return Ok(results);
    }
    Ok(0)
}
