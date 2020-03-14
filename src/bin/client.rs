use packybara_grpc::client as pbclient;
mod client_cli;
use client_cli::*;
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Pb::from_args();

    let mut client = pbclient::Client::new("http://[::1]:50051").await?;
    let Pb { crud, .. } = opt;
    match crud {
        PbCrud::Find { cmd } => match cmd {
            PbFind::VersionPin {
                package,
                level,
                role,
                platform,
                site,
                ..
            } => {
                let response = client
                    .get_version_pin(
                        pbclient::GetVersionPinOptions::new(package)
                            .level_opt(level)
                            .role_opt(role)
                            .platform_opt(platform)
                            .site_opt(site),
                    )
                    .await?;
                println!("RESPONSE={:#?}", response);
            }
            _ => println!("Not Implemented"), //     PbFind::VersionPins { .. } => {
                                              //         cmd::versionpins::find(client, cmd).await?;
                                              //     }
                                              //     PbFind::Roles { .. } => {
                                              //         cmd::all_roles::find(client, cmd).await?;
                                              //     }
                                              //     PbFind::Platforms { .. } => {
                                              //         cmd::all_platforms::find(client, cmd).await?;
                                              //     }
                                              //     PbFind::Sites { .. } => {
                                              //         cmd::all_sites::find(client, cmd).await?;
                                              //     }
                                              //     PbFind::Levels { .. } => {
                                              //         cmd::all_levels::find(client, cmd).await?;
                                              //     }
                                              //     PbFind::Pins { .. } => {
                                              //         cmd::pins::find(client, cmd).await?;
                                              //     }
                                              //     PbFind::VersionPinWiths { .. } => {
                                              //         cmd::versionpin_withs::find(client, cmd).await?;
                                              //     }
                                              //     PbFind::Withs { .. } => {
                                              //         cmd::withs::find(client, cmd).await?;
                                              //     }
                                              //     PbFind::Packages { .. } => {
                                              //         cmd::all_packages::find(client, cmd).await?;
                                              //     }
                                              //     PbFind::Distributions { .. } => {
                                              //         cmd::all_distributions::find(client, cmd).await?;
                                              //     }
                                              //     PbFind::PkgCoords { .. } => {
                                              //         cmd::pkgcoords::find(client, cmd).await?;
                                              //     }
                                              //     PbFind::Revisions { .. } => {
                                              //         cmd::all_revisions::find(client, cmd).await?;
                                              //     }
                                              //     PbFind::Changes { .. } => {
                                              //         cmd::all_changes::find(client, cmd).await?;
                                              //     }
        },
        // PbCrud::Add { cmd } => match cmd {
        //     PbAdd::Packages { .. } => {
        //         let tx = client.transaction().await?;
        //         cmd::all_packages::add(tx, cmd).await?;
        //     }
        //     PbAdd::Levels { .. } => {
        //         let tx = client.transaction().await?;
        //         cmd::all_levels::add(tx, cmd).await?;
        //     }
        //     PbAdd::Roles { .. } => {
        //         let tx = client.transaction().await?;
        //         cmd::all_roles::add(tx, cmd).await?;
        //     }
        //     PbAdd::Platforms { .. } => {
        //         let tx = client.transaction().await?;
        //         cmd::all_platforms::add(tx, cmd).await?;
        //     }
        //     PbAdd::Withs { .. } => {
        //         let tx = client.transaction().await?;
        //         cmd::withs::add(tx, cmd).await?;
        //     }
        //     PbAdd::VersionPins { .. } => {
        //         let tx = client.transaction().await?;
        //         cmd::versionpins::add(tx, cmd).await?;
        //     }
        // },
        // PbCrud::Set { cmd } => match cmd {
        //     PbSet::VersionPins { .. } => {
        //         let tx = client.transaction().await?;
        //         cmd::versionpins::set(tx, cmd).await?;
        //     }
        // },
        // PbCrud::Export { cmd } => match cmd {
        //     PbExport::PackagesXml { .. } => {
        //         cmd::export::export(client, cmd).await?;
        //     }
        // },
        _ => println!("Not implemented"),
    }

    Ok(())
}
