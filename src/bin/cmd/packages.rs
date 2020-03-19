use super::args::find::PbFind;
use packybara_grpc::client as pbclient;
use packybara_grpc::client::Client;
//use packybara_grpc::utils::truncate;
use prettytable::{cell, format, row, table};

pub(crate) async fn find(
    mut client: Client,
    cmd: PbFind,
) -> Result<(), Box<dyn std::error::Error>> {
    if let PbFind::Packages { package } = cmd {
        let results = client
            .get_packages(pbclient::get_packages::Options::new().name_opt(package))
            .await?;
        let mut table = table!([bFg => "NAME"]);
        for result in results {
            table.add_row(row![result.name]);
        }
        table.set_format(*format::consts::FORMAT_CLEAN); //FORMAT_NO_LINESEP_WITH_TITLE  FORMAT_NO_BORDER_LINE_SEPARATOR
        table.printstd();
    }
    Ok(())
}
