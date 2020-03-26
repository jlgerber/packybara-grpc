use super::*;
use packybara::io::packages_xml::xml;

pub(crate) async fn export_packagesxml(
    client: Client,
    request: Request<PackagesXmlExportRequest>,
) -> Result<Response<PackagesXmlExportReply>, Status> {
    let mut db = PackratDb::new();
    let PackagesXmlExportRequest { show, path } = request.into_inner();
    xml::write_xml(&mut db, &client, show, path)
        .await
        .map_err(|e| Status::new(Code::Internal, format!("{}", e)))?;
    Ok(Response::new(PackagesXmlExportReply { result: true }))
}
