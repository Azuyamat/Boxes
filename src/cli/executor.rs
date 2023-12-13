use std::path::Path;
use crate::cli::constructor::{Args, DJ, ServerAction};
use crate::cli::generator;
use crate::config::Config;
use crate::error::Error;
use crate::minecraft::jars::load_jars;
use crate::utils::read_line;

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
            let jar = jars.get_jar(&jar).ok_or(Error::ResourceNotFound("Jar not found".to_string()))?;
            let build = match build {
                Some(build) => build,
                None => jar.get_latest_build(version.clone()).ok_or(Error::ResourceNotFound("Jar build not found".to_string()))?.to_string(),
            };
            let mut path = Path::new(&location);
            while !path.exists() {
                println!("🚨 Path does not exist!");
                location = read_line("🎚️ Please enter the server location:")?;
                path = Path::new(&location);
            }
            let server = jar
                .download(&version, &build, &name, path.to_path_buf())?;
            config.add_server(&server);
        }
        DJ::Start { name } => {
            let server = config.get_server(&name).ok_or(Error::ResourceNotFound("Server not found".to_string()))?;
            server.run()?;
        }
        DJ::Config { action } => {
            crate::config_cli::manage_config_action(action, &config)?;
        }
        DJ::Server { action } => match action {
            ServerAction::List => {
                config.print_info()?;
            }
            ServerAction::Generate => {
                generator::prompt_wizard(&mut config)?;
            }
            ServerAction::Info { name } => {
                let server = config.get_server(&name).ok_or(Error::ResourceNotFound("Server not found".to_string()))?;
                server.print_info();
            }
            ServerAction::Start { name } => {
                let server = config.get_server(&name).ok_or(Error::ResourceNotFound("Server not found".to_string()))?;
                server.run()?;
            }
            ServerAction::Delete { name } => {
                let server = config.get_server(&name).ok_or(Error::ResourceNotFound("Server not found".to_string()))?;
                server.delete();
            }
            ServerAction::Add { location } => {
                let server = crate::minecraft::server::Server::from_path(&location)?;
                config.add_server(&server);
            }
            ServerAction::Plugins { name } => {
                let server = config.get_server(&name).ok_or(Error::ResourceNotFound("Server not found".to_string()))?;
                let plugins = server.plugins();
                println!("📝 Getting plugins for {} ({} plugin(s))...", name, plugins.len());
                for plugin in plugins {
                    println!(" - {plugin}");
                }
            }
            ServerAction::AssignIP { name, ip } => {
                let server = config.get_server(&name).ok_or(Error::ResourceNotFound("Server not found".to_string()))?;
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