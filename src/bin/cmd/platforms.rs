use super::args::add::PbAdd;
use super::args::find::PbFind;
use packybara_grpc::client_service as pbclient;
use packybara_grpc::client_service::ClientService;
use prettytable::{cell, format, row, table};

pub(crate) async fn find(
    mut client: ClientService,
    cmd: PbFind,
) -> Result<(), Box<dyn std::error::Error>> {
    if let PbFind::Platforms {
        platform,
        order_by,
        order_direction,
        limit,
    } = cmd
    {
        let results = client
            .get_platforms(
                pbclient::get_platforms::Options::new()
                    .name_opt(platform)
                    .order_by_opt(order_by)
                    .order_direction_opt(order_direction)
                    .limit_opt(limit),
            )
            .await?;
        let mut table = table!([bFg => "NAME"]);
        results
            .iter()
            .filter(|x| x.name != "any")
            .for_each(|result| {
                table.add_row(row![result.name]);
            });
        table.set_format(*format::consts::FORMAT_CLEAN); //FORMAT_NO_LINESEP_WITH_TITLE  FORMAT_NO_BORDER_LINE_SEPARATOR
        table.printstd();
    }
    Ok(())
}

pub(crate) async fn add(
    mut client: ClientService,
    cmd: PbAdd,
) -> Result<u64, Box<dyn std::error::Error>> {
    if let PbAdd::Platforms { names, comment } = cmd {
        let username = whoami::username();
        let opts = pbclient::add_platforms::Options::new(names, username).comment_opt(Some(
            comment.unwrap_or("Auto Comment - Platforms Added".to_string()),
        ));

        let results = client.add_platforms(opts).await?;

        println!("{}", results);
        return Ok(results);
    }
    Ok(0)
}
