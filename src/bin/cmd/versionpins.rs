use super::args::find::PbFind;
use packybara_grpc::client_service as pbclient;
use packybara_grpc::client_service::ClientService;
use packybara_grpc::utils::truncate;
use prettytable::{cell, format, row, table};

pub(crate) async fn find(
    mut client: ClientService,
    cmd: PbFind,
) -> Result<(), Box<dyn std::error::Error>> {
    if let PbFind::VersionPins {
        package,
        version,
        level,
        role,
        platform,
        site,
        isolate_facility,
        search_mode,
        order_by,
        order_direction,
        limit,
        full_withs,
    } = cmd
    {
        let response = client
            .get_version_pins(
                pbclient::get_versionpins::Options::new()
                    .package_opt(package)
                    .version_opt(version)
                    .level_opt(level)
                    .role_opt(role)
                    .platform_opt(platform)
                    .site_opt(site)
                    .isolate_facility_opt(Some(isolate_facility))
                    .search_mode_opt(search_mode)
                    .order_direction_opt(order_direction)
                    .order_by_opt(order_by),
            )
            .await?;
        let mut table =
            table!([bFg => "PIN ID", "DISTRIBUTION", "ROLE", "LEVEL", "PLATFORM", "SITE", "WITHS"]);
        for response in response {
            let withs = response.withs.unwrap_or(Vec::new());
            let withs = if withs.len() > 0 {
                if full_withs {
                    format!("[{}]", withs.join(","))
                } else {
                    format!("[{}...]", truncate(withs.join(",").as_ref(), 40))
                }
            } else {
                "[]".to_string()
            };
            table.add_row(row![
                response.versionpin_id,
                response.distribution,
                response.coords.role,
                response.coords.level,
                response.coords.platform,
                response.coords.site,
                withs,
            ]);
        }
        table.set_format(*format::consts::FORMAT_CLEAN); //FORMAT_NO_LINESEP_WITH_TITLE  FORMAT_NO_BORDER_LINE_SEPARATOR
        table.printstd();
    }
    Ok(())
}
