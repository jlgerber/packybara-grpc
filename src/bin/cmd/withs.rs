use super::args::find::PbFind;
use packybara_grpc::client as pbclient;
use packybara_grpc::client::Client;
use packybara_grpc::utils::truncate;
use prettytable::{cell, format, row, table};

pub(crate) async fn find(
    mut client: Client,
    cmd: PbFind,
) -> Result<(), Box<dyn std::error::Error>> {
    if let PbFind::Withs {
        package,
        level,
        role,
        platform,
        site,
        order_by,
        order_direction,
        ..
    } = cmd
    {
        let response = client
            .get_withs(
                pbclient::get_withs::Options::new()
                    .package_opt(Some(package))
                    .level_opt(level)
                    .role_opt(role)
                    .platform_opt(platform)
                    .site_opt(site)
                    .order_by_opt(order_by)
                    .order_direction_opt(order_direction),
            )
            .await?;
        let mut table =
            table!([bFg => "PIN ID", "DISTRIBUTION", "ROLE", "LEVEL", "PLATFORM", "SITE"]);
        for response in response {
            table.add_row(row![
                response.versionpin_id,
                response.distribution,
                response.coords.role,
                response.coords.level,
                response.coords.platform,
                response.coords.site,
            ]);
        }
        table.set_format(*format::consts::FORMAT_CLEAN); //FORMAT_NO_LINESEP_WITH_TITLE  FORMAT_NO_BORDER_LINE_SEPARATOR
        table.printstd();
    }
    Ok(())
}
