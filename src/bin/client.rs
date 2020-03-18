use packybara_grpc::client as pbclient;
mod cmd;
use cmd::args::*;
use packybara_grpc::url_builder;
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Pb::from_args();
    let url = url_builder::UrlBuilder::new()
        .host(url_builder::Host::Localhost)
        .port(50051)
        .build(); //"http://[::1]:50051"
    let client = pbclient::Client::new(url).await?;
    let Pb { crud, .. } = opt;
    match crud {
        PbCrud::Find { cmd } => match cmd {
            PbFind::VersionPin { .. } => {
                cmd::versionpin::find(client, cmd).await?;
            }
            PbFind::VersionPins { .. } => {
                cmd::versionpins::find(client, cmd).await?;
            }
            _ => println!("Not Implemented"),
            // PbFind::Roles { .. } => {
            //     cmd::all_roles::find(client, cmd).await?;
            // }
            // PbFind::Platforms { .. } => {
            //     cmd::all_platforms::find(client, cmd).await?;
            // }
            // PbFind::Sites { .. } => {
            //     cmd::all_sites::find(client, cmd).await?;
            // }
            // PbFind::Levels { .. } => {
            //     cmd::all_levels::find(client, cmd).await?;
            // }
            // PbFind::Pins { .. } => {
            //     cmd::pins::find(client, cmd).await?;
            // }
            // PbFind::VersionPinWiths { .. } => {
            //     cmd::versionpin_withs::find(client, cmd).await?;
            // }
            // PbFind::Withs { .. } => {
            //     cmd::withs::find(client, cmd).await?;
            // }
            // PbFind::Packages { .. } => {
            //     cmd::all_packages::find(client, cmd).await?;
            // }
            // PbFind::Distributions { .. } => {
            //     cmd::all_distributions::find(client, cmd).await?;
            // }
            // PbFind::PkgCoords { .. } => {
            //     cmd::pkgcoords::find(client, cmd).await?;
            // }
            // PbFind::Revisions { .. } => {
            //     cmd::all_revisions::find(client, cmd).await?;
            // }
            // PbFind::Changes { .. } => {
            //     cmd::all_changes::find(client, cmd).await?;
            // }
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
        //_ => println!("Not implemented"),
    }

    Ok(())
}
