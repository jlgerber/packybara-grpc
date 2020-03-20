use super::args::find::PbFind;
use packybara_grpc::client as pbclient;
use packybara_grpc::client::Client;
use prettytable::{cell, format, row, table};

pub(crate) async fn find(
    mut client: Client,
    cmd: PbFind,
) -> Result<(), Box<dyn std::error::Error>> {
    if let PbFind::Distributions {
        package, version, ..
    } = cmd
    {
        let results = client
            .get_distributions(
                pbclient::get_distributions::Options::new()
                    .package_opt(package)
                    .version_opt(version),
            )
            .await?;
        let mut table = table!([bFg => "PACKAGE", "VERSION"]);
        results
            .iter()
            //.filter(|x| x.name != "any")
            .for_each(|result| {
                table.add_row(row![result.package, result.version]);
            });
        table.set_format(*format::consts::FORMAT_CLEAN); //FORMAT_NO_LINESEP_WITH_TITLE  FORMAT_NO_BORDER_LINE_SEPARATOR
        table.printstd();
    }
    Ok(())
}
