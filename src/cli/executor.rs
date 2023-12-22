#![warn(
    clippy::absolute_paths,
    clippy::cognitive_complexity,
    clippy::debug_assert_with_mut_call,
    clippy::doc_markdown,
    clippy::empty_enum,
    clippy::enum_glob_use,
    clippy::pedantic,
    clippy::complexity
)]

use crate::cli::constructor::{Args, ServerAction, DJ};
use crate::cli::generator;
use crate::cli::{config_cli, print_server_info};
use crate::config::Config;
use crate::error::Error;
use crate::themes::theme::Theme;
use crate::utils::{read_line, start_server};
use notch::jars::jar::Jar;
use notch::jars::manager::JarManager;
use notch::servers::downloader::Downloader;
use notch::servers::server::Server;
use std::path::Path;

use super::constructor::ThemeAction;

pub fn execute(args: Args, mut config: Config, theme: &Theme) -> Result<(), Error> {
    let verbose = args.verbose;
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
            let jars = JarManager::load()?;
            let jar = jars.get_jar(jar)?;
            let build: u32 = match build {
                Some(build) => build.parse::<u32>().unwrap(),
                None => jar.get_latest_build(version.clone())?,
            };
            let mut path = Path::new(&location);
            while !path.exists() {
                println!("🚨 Path does not exist!");
                location = read_line("🎚️ Please enter the server location:")?;
                path = Path::new(&location);
            }
            let jar = Jar {
                name: jar.name.clone(),
                version: Some(version),
                build: Some(build),
            };
            let server = Server::new(name, jar, path.to_path_buf())?;
            let downloader = Downloader::new(&server);
            downloader.download()?;
            config.add_server(&server, true);
        }
        DJ::Start { name } => {
            let server = config
                .get_server(&name)
                .ok_or(Error::ResourceNotFound("Server not found".to_string()))?;
            start_server(&server)?;
        }
        DJ::Config { action } => {
            config_cli::manage_config_action(action, &config)?;
        }

        // Actions
        DJ::Server { action } => handle_server_action(action, &mut config, verbose)?,
        DJ::Theme { action } => handle_theme_action(action, verbose),
    }
    Ok(())
}

fn handle_server_action(
    action: ServerAction,
    config: &mut Config,
    verbose: bool,
) -> Result<(), Error> {
    match action {
        ServerAction::List => {
            config.print_info()?;
        }
        ServerAction::Generate => {
            generator::prompt_wizard(config)?;
        }
        ServerAction::Info { name } => {
            let server = config
                .get_server(&name)
                .ok_or(Error::ResourceNotFound("Server not found".to_string()))?;
            print_server_info(&server);
        }
        ServerAction::Start { name } => {
            let server = config
                .get_server(&name)
                .ok_or(Error::ResourceNotFound("Server not found".to_string()))?;
            start_server(&server)?;
        }
        ServerAction::Delete { name } => {
            let server = config
                .get_server(&name)
                .ok_or(Error::ResourceNotFound("Server not found".to_string()))?;
            server.delete()?;
        }
        ServerAction::Remove { name } => {
            config.remove_server(&name)?;
        }
        ServerAction::Add { location } => {
            let location = Path::new(&location);
            if !location.exists() || location.is_relative() {
                return Err(Error::ResourceNotFound("Path does not exist".to_string()));
            }
            let server = Server::from_path(&location.to_path_buf())?;
            config.add_server(&server, true);
        }
        ServerAction::Plugins { name } => {
            todo!("Plugins")
            // let server = config
            //     .get_server(&name)
            //     .ok_or(Error::ResourceNotFound("Server not found".to_string()))?;
            // let plugins = server.plugins();
            // println!(
            //     "📝 Getting plugins for {} ({} plugin(s))...",
            //     name,
            //     plugins.len()
            // );
            // for plugin in plugins {
            //     let plugin = plugin.to_string_lossy().to_string();
            //     println!(" - {plugin}");
            // }
        }
        ServerAction::AssignIP { name, ip } => {
            todo!("Assign IP")
            // let server = config
            //     .get_server(&name)
            //     .ok_or(Error::ResourceNotFound("Server not found".to_string()))?;
            // let manipulator = ServerManipulator {
            //     server: server.clone(),
            // };
            // if let Some(mut properties) = manipulator.get_server_properties() {
            //     println!("📝 Assigning {name}'s IP to {ip}...");
            //     properties.insert("server-ip".to_string(), ip.clone());
            //     manipulator.save_server_properties(&properties);
            //     println!("📝 Assigned {name}'s IP to {ip}!");
            // }
        }
        ServerAction::Optimize { name } => {
            todo!("Optimize")
            // let server = config
            //     .get_server(&name)
            //     .ok_or(Error::ResourceNotFound("Server not found".to_string()))?;
            // server.optimize(verbose);
        }
        ServerAction::Import { location } => {
            todo!("Import server")
            // let path = Path::new(&location);
            // if !path.exists() {
            //     return Err(Error::ResourceNotFound("Path does not exist".to_string()));
            // }
            // println!("📝 Importing servers from {location}...");
            // for entry in path.read_dir()? {
            //     let entry = entry?;
            //     let path = entry.path();
            //     if !path.is_dir() {
            //         continue;
            //     }
            //     let Ok(server) = Server::from_path(path.to_str().unwrap()) else {
            //         println!(
            //             "⚠️ Failed to create server from path {}",
            //             path.to_str().unwrap()
            //         );
            //         return Err(Error::ResourceNotFound(
            //             "⚠️ Failed to create server from path".to_string(),
            //         ));
            //     };
            //     if config.get_server(&server.server_name).is_some() {
            //         println!("😒 Skipping {}, it already exists.", server.server_name);
            //         continue;
            //     }
            //     config.add_server(&server, true);
            // }
            // confy::store("boxes", None, config).expect("🚨 Config file could not be saved!");
            // println!("📝 Imported servers from {location}!");
        }
    }
    Ok(())
}

fn handle_theme_action(action: ThemeAction, verbose: bool) {
    match action {
        ThemeAction::List => {
            todo!()
        }
        ThemeAction::Create => {
            todo!()
        }
        ThemeAction::Delete => {
            todo!()
        }
        ThemeAction::Info { name } => {
            todo!()
        }
    }
}
