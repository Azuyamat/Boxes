use std::path::Path;
use crate::config::Config;
use crate::config_cli::ConfigAction;
use crate::error::Error;
use crate::minecraft::jars::load_jars;
use crate::read_line;
use clap::{Parser, Subcommand};
use inquire::Select;

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
    Start { name: String },
    /// Config commands
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
    /// Server commands
    Server {
        #[command(subcommand)]
        action: ServerAction,
    },
}

#[derive(Subcommand, Debug)]
pub enum ServerAction {
    List,
    Generate,
    Info { name: String },
    Start { name: String },
    Delete { name: String },
    Add { location: String },
    Plugins { name: String },
    AssignIP { name: String, ip: String },
}

// Actions

pub(crate) fn execute(args: Args, mut config: Config) -> Result<(), Error> {
    match args.dj {
        DJ::Create {
            name,
            jar,
            version,
            build,
            mut location,
        } => {
            // Print with emoji
            println!("🔥 Creating server...");
            let jars = load_jars()?;
            let jar = jars.get_jar(&jar).ok_or(Error::JarNotFound{ name: jar })?;
            let build = match build {
                Some(build) => build,
                None => jar.get_latest_build(version.clone())?,
            };
            let mut path = Path::new(&location);
            while !path.exists() {
                println!("🚨 Path does not exist!");
                location = read_line!("🎚️ Please enter the server location:");
                path = Path::new(&location);
            }
            let server = jar
                .download(&version, &build, &name, path.to_path_buf())?;
            config.add_server(&server);
        }
        DJ::Start { name } => {
            let server = config.get_server(&name).ok_or(Error::ServerNotFound{ name })?;
            server.run();
        }
        DJ::Config { action } => {
            crate::config_cli::manage_config_action(action, &config)?;
        }
        DJ::Server { action } => match action {
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
                    let length = server_name.len() as u8;
                    if !(1..=100).contains(&length) {
                        println!("⚠️ Server name must be within 1 and 100 characters. Please enter a different name:");
                        server_name = read_line!("🎚️ Please enter the server name:");
                    }
                    break;
                }
                let jar_name = Select::new(
                    "🎚️ Please enter the server Jar",
                    jars.jars
                        .iter()
                        .map(|j| j.name.as_str())
                        .collect::<Vec<&str>>(),
                )
                .prompt()?;
                let jar = jars.get_jar(jar_name).ok_or(Error::JarNotFound { name: jar_name.to_string() })?;
                let version = Select::new(
                    "🎚️ Please enter the server version",
                    jar.get_versions()?,
                )
                .prompt()?;
                let builds = jar.get_builds(&version)?;
                let latest = builds.first().ok_or(Error::BuildNotFound { name: jar_name.to_string(), build: 0 })?;
                let build = Select::new(
                    &format!("🎚️ Please enter the jar build ({} is latest)", latest),
                    builds,
                )
                .prompt()?
                .to_string();
                let mut location = read_line!("🎚️ Please enter the server location:");
                let mut path = Path::new(&location);
                if location.is_empty() {
                    // Set path to location where the command was run
                    location = std::env::current_dir().unwrap().to_str().unwrap().to_string();
                    path = Path::new(&location);
                    println!("🎚️ Using current directory as server location ({}).", location);
                }
                while !path.exists() {
                    println!("🚨 Path does not exist!");
                    location = read_line!("🎚️ Please enter the server location:");
                    path = Path::new(&location);
                }
                let server = jar
                    .download(&version, &build, &server_name, path.to_path_buf())?;
                println!("🎛️ Server generated!");
                config.add_server(&server);
            }
            ServerAction::Info { name } => {
                let server = config.get_server(&name).ok_or(Error::ServerNotFound{ name })?;
                server.print_info();
            }
            ServerAction::Start { name } => {
                let server = config.get_server(&name).ok_or(Error::ServerNotFound{ name })?;
                server.run();
            }
            ServerAction::Delete { name } => {
                let server = config.get_server(&name).ok_or(Error::ServerNotFound{ name })?;
                server.delete();
            }
            ServerAction::Add { location } => {
                let server = crate::minecraft::server::Server::from_path(&location);
                config.add_server(&server);
            }
            ServerAction::Plugins { name } => {
                let server = config.get_server(&name).ok_or(Error::ServerNotFound{ name })?;
                for plugin in server.plugins() {
                    println!("{plugin}");
                }
            }
            ServerAction::AssignIP { name, ip } => {
                let server = config.get_server(&name).ok_or(Error::ServerNotFound{ name: name.clone() })?;
                let manipulator = crate::minecraft::server_manipulator::ServerManipulator {
                    server: server.clone(),
                };
                let mut properties = manipulator.get_server_properties();
                println!("📝 Assigning {}'s IP to {}...", name, ip);
                properties.insert("server-ip".to_string(), ip.clone());
                manipulator.save_server_properties(&properties);
                println!("📝 Assigned {}'s IP to {}!", name, ip);
            }
        },
    }
    Ok(())
}
