use super::args::find::PbFind;
use packybara_grpc::client_service as pbclient;
use packybara_grpc::client_service::ClientService;
//use packybara_grpc::utils::truncate;
use prettytable::{cell, format, row, table};

pub(crate) async fn find(
    mut client: ClientService,
    cmd: PbFind,
) -> Result<(), Box<dyn std::error::Error>> {
    if let PbFind::Changes { transaction_id } = cmd {
        let response = client
            .get_changes(
                pbclient::get_changes::Options::new().transaction_id_opt(Some(transaction_id)),
            )
            .await?;
        let mut table =
            table!([bFg => "ID", "TX ID", "ACTION", "PACKAGE","LEVEL", "ROLE", "PLATFORM", "SITE"]);
        for result in response {
            table.add_row(row![
                result.id,
                result.transaction_id,
                result.action,
                result.package,
                result.level,
                result.role,
                result.platform,
                result.site
            ]);
        }
        table.set_format(*format::consts::FORMAT_CLEAN);
        table.printstd();
    }
    Ok(())
}
