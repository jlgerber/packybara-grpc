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
    if let PbFind::Levels {
        level,
        show,
        depth,
        order_by,
    } = cmd
    {
        let response = client
            .get_levels(
                pbclient::get_levels::Options::new()
                    .level_opt(level)
                    .show_opt(show)
                    .depth_opt(depth)
                    .order_by_opt(order_by),
            )
            .await?;
        let mut table = table!([bFg => "LEVEL", "SHOW"]);
        for result in response {
            table.add_row(row![result.level, result.show]);
        }
        table.set_format(*format::consts::FORMAT_CLEAN);
        table.printstd();
    }
    Ok(())
}

pub(crate) async fn add(
    mut client: ClientService,
    cmd: PbAdd,
) -> Result<u64, Box<dyn std::error::Error>> {
    if let PbAdd::Levels { names, comment } = cmd {
        let username = whoami::username();
        let opts = pbclient::add_levels::Options::new(names, username).comment_opt(Some(
            comment.unwrap_or("Auto Comment - Levels Added".to_string()),
        ));

        let results = client.add_levels(opts).await?;

        println!("{}", results);
        return Ok(results);
    }
    Ok(0)
}
