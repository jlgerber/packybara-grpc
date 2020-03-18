use super::args::find::PbFind;
use packybara_grpc::client as pbclient;
use packybara_grpc::client::Client;
use packybara_grpc::utils::truncate;
use prettytable::{cell, format, row, table};

pub(crate) async fn find(
    mut client: Client,
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
