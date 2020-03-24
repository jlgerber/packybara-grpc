use super::args::add::PbAdd;
use super::args::find::PbFind;
use packybara_grpc::client_service as pbclient;
use packybara_grpc::client_service::ClientService;
use prettytable::{cell, format, row, table};

pub(crate) async fn find(
    mut client: ClientService,
    cmd: PbFind,
) -> Result<(), Box<dyn std::error::Error>> {
    if let PbFind::Sites { site } = cmd {
        let results = client
            .get_sites(pbclient::get_sites::Options::new().name_opt(site))
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

pub(crate) async fn add(
    mut client: ClientService,
    cmd: PbAdd,
) -> Result<u64, Box<dyn std::error::Error>> {
    if let PbAdd::Sites { names, comment } = cmd {
        let username = whoami::username();
        let opts = pbclient::add_sites::Options::new(names, username).comment_opt(Some(
            comment.unwrap_or("Auto Comment - Sites Added".to_string()),
        ));

        let results = client.add_sites(opts).await?;

        println!("{}", results);
        return Ok(results);
    }
    Ok(0)
}
