/*******************************************************
 * Copyright (C) 2019,2020 Jonathan Gerber <jlgerber@gmail.com>
 *
 * This file is part of packybara.
 *
 * packybara can not be copied and/or distributed without the express
 * permission of Jonathan Gerber
 *******************************************************/
use super::args::PbExport;
use packybara_grpc::client_service as pbclient;
use packybara_grpc::client_service::ClientService;

/// Given a client and PbExport enum, extract the parameters from the enum and
/// export the provided show's state to disk
///
/// # Arguments
/// * `client` - a ClientService instance
/// * `cmd` - a PbExport instance
///
/// # Returns
/// * A Unit if Ok, or a boxed error if Err
pub async fn export(
    mut client: ClientService,
    cmd: PbExport,
) -> Result<(), Box<dyn std::error::Error>> {
    let PbExport::PackagesXml { show, path, .. } = cmd;
    // let mut db = packrat::PackratDb::new(client);
    // let result = xml::write_xml(&mut db, show, path).await?;

    client
        .export_packagesxml(pbclient::export_packagesxml::Options::new(show, path))
        .await?;

    Ok(())
}
