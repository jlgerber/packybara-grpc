use super::args::find::PbFind;
use packybara_grpc::client as pbclient;
use prettytable::{cell, format, row, table};
use serde_json;
//use tokio_postgres::Client;
use packybara_grpc::client::Client;
use packybara_grpc::utils::truncate;

pub(crate) async fn find(
    mut client: Client,
    cmd: PbFind,
) -> Result<(), Box<dyn std::error::Error>> {
    if let PbFind::VersionPin {
        package,
        level,
        role,
        platform,
        site,
        full_withs,
        json,
        ..
    } = cmd
    {
        let response = client
            .get_version_pin(
                pbclient::get_versionpin::Options::new(package)
                    .level_opt(level)
                    .role_opt(role)
                    .platform_opt(platform)
                    .site_opt(site),
            )
            .await?;
        if json {
            let serialized =
                serde_json::to_string_pretty(&response).expect("unable to unwrap response");
            println!("{}", serialized);
        } else {
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
            let mut table = table!([bFg => "PIN ID", "DISTRIBUTION", "ROLE", "LEVEL", "PLATFORM", "SITE", "WITHS"]);
            table.add_row(row![
                response.versionpin_id,
                response.distribution,
                response.coords.role,
                response.coords.level,
                response.coords.platform,
                response.coords.site,
                withs,
            ]);

            table.set_format(*format::consts::FORMAT_CLEAN); //FORMAT_NO_LINESEP_WITH_TITLE  FORMAT_NO_BORDER_LINE_SEPARATOR
            table.printstd();
        }
    }
    Ok(())
}
