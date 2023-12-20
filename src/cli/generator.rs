use crate::config::Config;
use crate::error::Error;
use crate::utils::{canonize, read_line};
use inquire::Select;
use notch::jars::jar::Jar;
use notch::jars::manager::JarManager;
use notch::servers::downloader::Downloader;
use notch::servers::server::Server;
use std::fs;
use std::path::Path;

pub fn prompt_wizard(config: &mut Config) -> Result<(), Error> {
    let jars = JarManager::load()?;
    println!("🎚️ Welcome to the server generator");
    let mut server_name;
    loop {
        server_name = read_line("🎚️ Please enter the server name:")?;
        if config.get_server(&server_name).is_some() {
            println!(
                "⚠️ A server with the same name already exists! Please enter a different name:"
            );
            continue;
        }
        let length = server_name.len() as u8;
        if !(1..=100).contains(&length) {
            println!(
                "⚠️ Server name must be within 1 and 100 characters. Please enter a different name:"
            );
            server_name = read_line("🎚️ Please enter the server name:")?;
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
    let jar = jars.get_jar(jar_name.to_string())?;
    let version = Select::new("🎚️ Please enter the server version", jar.get_versions()?).prompt()?;
    let builds = jar.get_builds(version.clone())?;
    let latest = builds
        .first()
        .ok_or(Error::ResourceNotFound("Jar build not found".to_string()))?;
    let build: u32 = Select::new(
        &format!("🎚️ Please enter the jar build ({} is latest)", latest),
        builds,
    )
    .prompt()?;
    let mut location = read_line("🎚️ Please enter the server location:")?;
    let mut path = Path::new(&location);
    if location.is_empty() {
        // Set path to location where the command was run
        location = std::env::current_dir()?.to_str().unwrap().to_string();
        path = Path::new(&location);
        println!(
            "🎚️ Using current directory as server location ({}).",
            location
        );
    }
    while !path.exists() {
        println!("🚨 Path does not exist! Should we create it?");
        let should_create = read_line("🎚️ Create dir (Y/N):")?;
        if should_create.to_lowercase() == "y" {
            fs::create_dir_all(path)?;
            break;
        }
        location = read_line("🎚️ Please enter the server location:")?;
        path = Path::new(&location);
    }
    let full_path = canonize(path)?;

    let jar = Jar {
        name: jar_name.to_string(),
        version: Some(version.to_string()),
        build: Some(build),
    };
    let server = Server::new(server_name.clone(), jar, full_path.clone())?;
    let downloader = Downloader::new(&server);
    println!("🎛️ Generating server... (downloading)");
    downloader.download()?;
    println!("🎛️ Server generated!");
    config.add_server(&server, true);
    server.save()?;
    println!("🎛️ Server generated!");
    Ok(())
}
