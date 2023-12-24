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

use std::env::current_dir;
use crate::cli::constructor::{Args, ServerAction, DJ, ServerPluginAction};
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
use notch::servers::manipulator::Manipulator;
use notch::hangar::{fetch_plugins, HangarProject};
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
            let mut path = Path::new(&location).to_path_buf();
            while !path.exists() {
                println!("🚨 Path does not exist!");
                location = read_line("🎚️ Please enter the server location:")?;
                path = Path::new(&location).to_path_buf();
            }
            if path.is_relative() {
                path = current_dir()?.join(path);
            }
            let jar = Jar {
                name: jar.name.clone(),
                version: Some(version),
                build: Some(build),
            };
            let server = Server::new(name, jar, path)?;
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
            let server = Server::from_path(location)?;
            config.add_server(&server, true);
        }
        ServerAction::Plugins { action } => handle_server_plugin_action(action, config)?,
    }
    Ok(())
}

fn handle_server_plugin_action(action: ServerPluginAction, config: &Config) -> Result<(), Error> {
    match action {
        ServerPluginAction::List { name } => {
            let server = config
                .get_server(&name)
                .ok_or(Error::ResourceNotFound("Server not found".to_string()))?;
            let manipulator = Manipulator::new(&server);
            let plugins = manipulator.plugins()?;
            println!("📃 Plugins for {} ({}) :", server.name, plugins.len());
            if plugins.is_empty() {
                println!(" - No plugins found");
            }
            for plugin in plugins {
                println!(" - {plugin}");
            }
        }
        ServerPluginAction::Add { name, plugin } => {
            let server = config
                .get_server(&name)
                .ok_or(Error::ResourceNotFound("Server not found".to_string()))?;
            let manipulator = Manipulator::new(&server);
            let plugins = fetch_plugins(&plugin)?;

            // Make a select menu with inquire for the user to select a plugin
            let plugins_select = plugins
                .iter()
                .map(|plugin| plugin.name.clone())
                .collect::<Vec<String>>();
            if plugins_select.is_empty() {
                println!("🚨 No plugins found with the name {plugin}");
                return Ok(());
            }
            let select = inquire::Select::new("🎚️ Select a plugin:", plugins_select);
            let plugin = select.prompt()?;
            let plugin: &HangarProject = plugins
                .iter()
                .find(|p| p.name == plugin)
                .ok_or(Error::PluginNotFound)?;
            println!("🗂️ Downloading plugin {}...", plugin.name);
            let author = &plugin.namespace.owner;
            let slug = &plugin.namespace.slug;
            println!("📦 Author: {author}");
            println!("🪪 Slug: {slug}");
            println!("📁 Category: {}", plugin.category);
            println!("📩 Downloads: {}", plugin.stats.downloads);
            println!("👁️ Views: {}", plugin.stats.views);
            println!("⭐  Stars: {}", plugin.stats.stars);
            manipulator.download_plugin(author, slug)?;
            println!("👌 Done!");
        }
        ServerPluginAction::Remove { name, plugin } => {
            let server = config
                .get_server(&name)
                .ok_or(Error::ResourceNotFound("Server not found".to_string()))?;
            let manipulator = Manipulator::new(&server);
            manipulator.remove_plugin(&plugin)?;
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
