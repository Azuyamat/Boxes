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

use crate::cli::config_cli;
use crate::cli::constructor::{Args, ServerAction, DJ};
use crate::cli::generator;
use crate::config::Config;
use crate::error::Error;
use crate::minecraft::jars::load;
use crate::minecraft::server::Server;
use crate::minecraft::server_manipulator::ServerManipulator;
use crate::themes::theme::Theme;
use crate::utils::read_line;
use std::path::Path;

use super::constructor::ThemeAction;

pub(crate) fn execute(args: Args, mut config: Config, theme: &Theme) -> Result<(), Error> {
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
            let jars = load()?;
            let jar = jars
                .get_jar(&jar)
                .ok_or(Error::ResourceNotFound("Jar not found".to_string()))?;
            let build = match build {
                Some(build) => build,
                None => jar
                    .get_latest_build(version.as_str())
                    .ok_or(Error::ResourceNotFound("Jar build not found".to_string()))?
                    .to_string(),
            };
            let mut path = Path::new(&location);
            while !path.exists() {
                println!("🚨 Path does not exist!");
                location = read_line("🎚️ Please enter the server location:")?;
                path = Path::new(&location);
            }
            let server = jar.download(&version, &build, &name, path)?;
            config.add_server(&server);
        }
        DJ::Start { name } => {
            let server = config
                .get_server(&name)
                .ok_or(Error::ResourceNotFound("Server not found".to_string()))?;
            server.run()?;
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
            server.print_info();
        }
        ServerAction::Start { name } => {
            let server = config
                .get_server(&name)
                .ok_or(Error::ResourceNotFound("Server not found".to_string()))?;
            server.run()?;
        }
        ServerAction::Delete { name } => {
            let server = config
                .get_server(&name)
                .ok_or(Error::ResourceNotFound("Server not found".to_string()))?;
            server.delete();
        }
        ServerAction::Add { location } => {
            let server = Server::from_path(&location)?;
            config.add_server(&server);
        }
        ServerAction::Plugins { name } => {
            let server = config
                .get_server(&name)
                .ok_or(Error::ResourceNotFound("Server not found".to_string()))?;
            let plugins = server.plugins();
            println!(
                "📝 Getting plugins for {} ({} plugin(s))...",
                name,
                plugins.len()
            );
            for plugin in plugins {
                let plugin = plugin.to_string_lossy().to_string();
                println!(" - {plugin}");
            }
        }
        ServerAction::AssignIP { name, ip } => {
            let server = config
                .get_server(&name)
                .ok_or(Error::ResourceNotFound("Server not found".to_string()))?;
            let manipulator = ServerManipulator {
                server: server.clone(),
            };
            if let Some(mut properties) = manipulator.get_server_properties() {
                println!("📝 Assigning {name}'s IP to {ip}...");
                properties.insert("server-ip".to_string(), ip.clone());
                manipulator.save_server_properties(&properties);
                println!("📝 Assigned {name}'s IP to {ip}!");
            }
        }
        ServerAction::Optimize { name } => {
            let server = config
                .get_server(&name)
                .ok_or(Error::ResourceNotFound("Server not found".to_string()))?;
            server.optimize(verbose);
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
