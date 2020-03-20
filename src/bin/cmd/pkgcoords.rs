use super::args::find::PbFind;
use packybara_grpc::client as pbclient;
use packybara_grpc::client::Client;
use prettytable::{cell, format, row, table};

pub(crate) async fn find(
    mut client: Client,
    cmd: PbFind,
) -> Result<(), Box<dyn std::error::Error>> {
    if let PbFind::PkgCoords {
        package,
        level,
        role,
        platform,
        site,
        search_mode,
        order_by,
    } = cmd
    {
        let results = client
            .get_pkgcoords(
                pbclient::get_pkgcoords::Options::new()
                    .package_opt(package)
                    .level_opt(level)
                    .role_opt(role)
                    .platform_opt(platform)
                    .site_opt(site)
                    .search_mode_opt(search_mode)
                    .order_by_opt(order_by),
            )
            .await?;
        let mut table = table!([bFg => "ID", "PACKAGE", "LEVEL", "ROLE", "PLATFORM", "SITE"]);
        for result in results {
            table.add_row(row![
                result.id,
                result.package,
                result.level,
                result.role,
                result.platform,
                result.site
            ]);
        }
        table.set_format(*format::consts::FORMAT_CLEAN); //FORMAT_NO_LINESEP_WITH_TITLE  FORMAT_NO_BORDER_LINE_SEPARATOR
        table.printstd();
    }
    Ok(())
}
