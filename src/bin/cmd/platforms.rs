use super::args::find::PbFind;
use packybara_grpc::client as pbclient;
use packybara_grpc::client::Client;
use prettytable::{cell, format, row, table};

pub(crate) async fn find(
    mut client: Client,
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
