use std::path::Path;
use inquire::Select;
use crate::config::Config;
use crate::error::Error;
use crate::minecraft::jars::load_jars;
use crate::utils::read_line;

pub fn prompt_wizard(config: &mut Config) -> Result<(), Error> {
    let jars = load_jars()?;
    println!("🎚️ Welcome to the server generator");
    let mut server_name;
    loop {
        server_name = read_line("🎚️ Please enter the server name:")?;
        if config.get_server(&server_name).is_some() {
            println!("⚠️ A server with the same name already exists! Please enter a different name:");
            continue;
        }
        let length = server_name.len() as u8;
        if !(1..=100).contains(&length) {
            println!("⚠️ Server name must be within 1 and 100 characters. Please enter a different name:");
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
    let jar = jars.get_jar(jar_name).ok_or(Error::ResourceNotFound("Jar not found".to_string()))?;
    let version = Select::new(
        "🎚️ Please enter the server version",
        jar.get_versions()?,
    )
        .prompt()?;
    let builds = jar.get_builds(&version)?;
    let latest = builds.first().ok_or(Error::ResourceNotFound("Jar build not found".to_string()))?;
    let build = Select::new(
        &format!("🎚️ Please enter the jar build ({} is latest)", latest),
        builds,
    )
        .prompt()?
        .to_string();
    let mut location = read_line("🎚️ Please enter the server location:")?;
    let mut path = Path::new(&location);
    if location.is_empty() {
        // Set path to location where the command was run
        location = std::env::current_dir()?.to_str().unwrap().to_string();
        path = Path::new(&location);
        println!("🎚️ Using current directory as server location ({}).", location);
    }
    while !path.exists() {
        println!("🚨 Path does not exist!");
        location = read_line("🎚️ Please enter the server location:")?;
        path = Path::new(&location);
    }
    let server = jar
        .download(&version, &build, &server_name, path.to_path_buf())?;
    println!("🎛️ Server generated!");
    config.add_server(&server);
    Ok(())
}