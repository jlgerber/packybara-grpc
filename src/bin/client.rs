use packybara_grpc::client_service as pbclient;
mod cmd;
use cmd::args::*;
use env_logger;
use env_logger::Env;
use packybara_grpc::url_builder;
use std::env;
use structopt::StructOpt;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let opt = Pb::from_args();
    if let Pb {
        loglevel: Some(ref level),
        ..
    } = opt
    {
        env::set_var("RUST_LOG", level);
    }
    env_logger::from_env(Env::default().default_filter_or("warn")).init();
    let url = url_builder::UrlBuilder::new()
        .host(url_builder::Host::Localhost)
        .port(50051)
        .build(); //"http://[::1]:50051"
    let client = pbclient::ClientService::new(url).await?;
    let Pb { crud, .. } = opt;

    match crud {
        PbCrud::Find { cmd } => match cmd {
            PbFind::VersionPin { .. } => {
                cmd::versionpin::find(client, cmd).await?;
            }
            PbFind::VersionPins { .. } => {
                cmd::versionpins::find(client, cmd).await?;
            }
            PbFind::Roles { .. } => {
                cmd::roles::find(client, cmd).await?;
            }
            PbFind::Platforms { .. } => {
                cmd::platforms::find(client, cmd).await?;
            }
            PbFind::Sites { .. } => {
                cmd::sites::find(client, cmd).await?;
            }
            PbFind::Levels { .. } => {
                cmd::levels::find(client, cmd).await?;
            }
            // PbFind::Pins { .. } => {
            //     cmd::pins::find(client, cmd).await?;
            // }
            PbFind::VersionPinWiths { .. } => {
                cmd::versionpin_withs::find(client, cmd).await?;
            }
            PbFind::Withs { .. } => {
                cmd::withs::find(client, cmd).await?;
            }
            PbFind::Packages { .. } => {
                cmd::packages::find(client, cmd).await?;
            }
            PbFind::Distributions { .. } => {
                cmd::distributions::find(client, cmd).await?;
            }
            PbFind::PkgCoords { .. } => {
                cmd::pkgcoords::find(client, cmd).await?;
            }
            PbFind::Revisions { .. } => {
                cmd::revisions::find(client, cmd).await?;
            }
            PbFind::Changes { .. } => {
                cmd::changes::find(client, cmd).await?;
            }
            _ => println!("Not Implemented"),
        },
        PbCrud::Add { cmd } => match cmd {
            PbAdd::Packages { .. } => {
                cmd::packages::add(client, cmd).await?;
            }
            PbAdd::Levels { .. } => {
                cmd::levels::add(client, cmd).await?;
            }
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
            _ => println!("Not Implemented"),
        },
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
