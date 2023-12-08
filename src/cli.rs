use clap::{Parser, Subcommand};
use crate::config::Config;
use crate::error::Error;
use crate::minecraft::jars::{load_jars};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    #[command(subcommand)]
    pub dj: DJ,
}

#[derive(Subcommand, Debug)]
pub enum DJ {
    Create {
        name: String,
        jar: String,
        location: String,
        version: String,
        build: Option<String>,
    },
    Start {
        name: String,
    },
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
    Server {
        #[command(subcommand)]
        action: ServerAction,
    }
}

#[derive(Subcommand, Debug)]
pub enum ConfigAction {
    Info
}

#[derive(Subcommand, Debug)]
pub enum ServerAction {
    List,
    Info {
        name: String,
    },
    Start {
        name: String,
    },
    Delete {
        name: String,
    },
    Plugins {
        name: String,
    },
}

// Actions

pub(crate) fn execute(args: Args, mut config: Config) -> Result<(), Error> {
    match args.dj {
        DJ::Create { name, jar, version, build, location } => {
            // Print with emoji
            println!("🔥 Creating server...");
            let jars = load_jars()?;
            let jar = jars.get_jar(&jar).expect("😧 Jar not found");
            let build = build.unwrap_or_else(|| jar.get_latest_build(version.clone()).expect("😧 Failed to get latest build"));
            let server = jar.download(&version, &build, &name, location).expect("😧 Failed to download jar (Check that the version and build exist)");
            config.add_server(&server);
        }
        DJ::Start { name } => {
            let server = config.get_server(&name).unwrap();
            server.run();
        }
        DJ::Config { action } => {
            match action {
                ConfigAction::Info => { config.print_info(); }
            }
        }
        DJ::Server { action } => {
            match action {
                ServerAction::List => {
                    config.print_info();
                }
                ServerAction::Info { name } => {
                    let server = config.get_server(&name).unwrap();
                    server.print_info();
                }
                ServerAction::Start { name } => {
                    let server = config.get_server(&name).unwrap();
                    server.run();
                }
                ServerAction::Delete { name } => {
                    let server = config.get_server(&name).unwrap();
                    server.delete();
                }
                ServerAction::Plugins { name } => {
                    let server = config.get_server(&name).unwrap();
                    for plugin in server.plugins() {
                        println!("{}", plugin);
                    }
                }
            }
        }
    }
    Ok(())
}