use std::path::Path;
use clap::{Parser, Subcommand};
use inquire::Select;
use crate::config::Config;
use crate::error::Error;
use crate::minecraft::jars::{load_jars};
use crate::read_line;

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    #[command(subcommand)]
    pub dj: DJ,
}

#[derive(Subcommand, Debug)]
pub enum DJ {
    /// Create a server with all arguments provided (name, jar, version, build, location)
    Create {
        name: String,
        jar: String,
        location: String,
        version: String,
        build: Option<String>,
    },
    /// Start a server with the given name
    Start {
        name: String,
    },
    /// Config commands
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
    /// Server commands
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
    Generate,
    Info {
        name: String,
    },
    Start {
        name: String,
    },
    Delete {
        name: String,
    },
    Add {
        location: String,
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
                ServerAction::Generate => {
                    let jars = load_jars()?;
                    println!("🎚️ Welcome to the server generator");
                    let mut server_name;
                    loop {
                        server_name = read_line!("🎚️ Please enter the server name:");
                        if config.get_server(&server_name).is_some() {
                            println!("⚠️ A server with the same name already exists! Please enter a different name:");
                            continue;
                        }
                        break;
                    }
                    let jar_name = Select::new("🎚️ Please enter the server Jar", jars.jars.iter().map(|j| j.name.as_str()).collect::<Vec<&str>>()).prompt().expect("😧 Failed to get jar name");
                    let jar = jars.get_jar(&jar_name).expect("😧 Jar not found");
                    let version = Select::new("🎚️ Please enter the server version", jar.get_versions().unwrap()).prompt().expect("😧 Failed to get jar name");
                    let builds = jar.get_builds(&version).unwrap();
                    let latest = builds.first().unwrap();
                    let build = Select::new(&*format!("🎚️ Please enter the jar build ({} is latest)", latest), builds).prompt().expect("😧 Failed to get jar build").to_string();
                    let location = read_line!("🎚️ Please enter the server location:");
                    let server = jar.download(&version, &build, &server_name, location).expect("😧 Failed to download jar (Check that the version and build exist)");
                    println!("🎛️ Server generated!");
                    config.add_server(&server);
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
                ServerAction::Add { location } => {
                    let server_info = crate::minecraft::server::ServerInfo::from_path(&Path::new(&location).to_path_buf());
                    let server = server_info.to_server(location);
                    config.add_server(&server);
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