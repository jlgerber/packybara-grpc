use super::args::find::PbFind;
use packybara_grpc::client_service as pbclient;
use packybara_grpc::client_service::ClientService;
use prettytable::{cell, format, row, table};

pub(crate) async fn find(
    mut client: ClientService,
    cmd: PbFind,
) -> Result<(), Box<dyn std::error::Error>> {
    if let PbFind::Revisions {
        id,
        transaction_id,
        author,
        order_by,
        order_direction,
        limit,
    } = cmd
    {
        let results = client
            .get_revisions(
                pbclient::get_revisions::Options::new()
                    .id_opt(id.map(|x| x as i64))
                    .transaction_id_opt(transaction_id)
                    .author_opt(author)
                    .order_by_opt(order_by)
                    .order_direction_opt(order_direction)
                    .limit_opt(limit),
            )
            .await?;
        let mut table = table!([bFg => "ID", "TX ID", "AUTHOR", "TIMESTAMP", "COMMENT"]);
        for result in results {
            table.add_row(row![
                result.id,
                result.transaction_id,
                result.author,
                result.datetime.format("%F %r"),
                result.comment
            ]);
        }
        table.set_format(*format::consts::FORMAT_CLEAN); //FORMAT_NO_LINESEP_WITH_TITLE  FORMAT_NO_BORDER_LINE_SEPARATOR
        table.printstd();
    }
    Ok(())
}
