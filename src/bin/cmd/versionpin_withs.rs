use super::args::find::PbFind;
use packybara_grpc::client::Client;
use prettytable::{cell, format, row, table};

pub(crate) async fn find(
    mut client: Client,
    cmd: PbFind,
) -> Result<(), Box<dyn std::error::Error>> {
    if let PbFind::VersionPinWiths { versionpin_id } = cmd {
        let results = client.get_version_pin_withs(versionpin_id as i64).await?;
        let mut table = table!([bFg => "ID", "VERSIONPIN ID", "WITH", "ORDER"]);
        for result in results {
            table.add_row(row![result.id, result.vpin_id, result.with, result.order]);
        }
        table.set_format(*format::consts::FORMAT_CLEAN); //FORMAT_NO_LINESEP_WITH_TITLE  FORMAT_NO_BORDER_LINE_SEPARATOR
        table.printstd();
    }
    Ok(())
}
