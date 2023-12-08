use std::fmt::{Display};
use std::fs::File;
use std::io::Write;
use std::process::Command;
use serde::Deserialize;
use crate::error::Error;
use crate::get_exec_time;
use crate::minecraft::server::Server;
use crate::utils::colorize;
use crate::utils::*;

pub fn load_jars() -> Result<JarManager, Error> {
    let toml = std::fs::read_to_string("jars.toml")?;
    let jars: JarManager = toml::from_str(&toml)?;

    Ok(jars)
}

#[derive(Deserialize)]
pub struct JarManager {
    pub jars: Vec<Jar>,
}

impl JarManager {
    pub fn get_jar(&self, name: &str) -> Option<&Jar> {
        self.jars.iter().find(|jar| jar.name.to_lowercase() == name.to_lowercase())
    }

    pub fn print_info(&self) {
        let running_jars = Command::new("jps")
            .output()
            .expect("🚨 Failed to get running jars!");
        let running_jars = String::from_utf8(running_jars.stdout).unwrap();
        println!("🗃️ Jar info:");
        println!("  💾 Jars:");
        if self.jars.is_empty() { println!("      No jars!"); }
        for jar in &self.jars {
            if running_jars.contains(&jar.name) {
                println!("      ➥ 📦 {} (Running)", colorize(&jar.name, Color::Green));
                continue;
            }
            println!("      ➥ 📦 {}", jar.name);
        }
    }
}

#[derive(Deserialize)]
pub struct JarInfo {
    pub project_id: String,
    pub project_name: String,
    pub version: String,
    pub builds: Vec<u32>
}

#[derive(Deserialize)]
pub struct Jar {
    pub name: String,
    pub download_url: String,
    pub info_url: String
}

impl Jar {
    fn new(name: String, download_url: String, info_url: String) -> Self {
        Self {
            name,
            download_url,
            info_url
        }
    }

    pub fn get_latest_build(&self, version: String) -> Result<String, Error> {
        let url = self.info_url.clone()
            .replace("{version}", version.as_str());
        let response = reqwest::blocking::get(&url)?;
        let body: JarInfo = response.json()?;
        let build = body.builds.last().unwrap();
        Ok(build.to_string())
    }

    pub fn download(&self, version: &str, build: &str, server_name: &str, location: String) -> Result<Server, Error> {
        println!("Downloading {}...", colorize(&self.name, Color::Green));
        let server: Server;
        let exec_time = get_exec_time!({
            let download_url = self.download_url.clone()
                .replace("{version}", version)
                .replace("{build}", build);
            println!("Downloading from {}...", colorize(&download_url, Color::LightPurple));
            let response = reqwest::blocking::get(&download_url)?;

            let path = std::path::Path::new(&location).join(format!("{0}", server_name));
            if !path.exists() { std::fs::create_dir_all(&path)?; }
            let mut file = File::create(&path.join(format!("{}-{}.jar", self.name, version)))?;
            file.write_all(&response.bytes()?)?;

            server = Server::new(
                server_name,
                self.name.clone(),
                version.to_string(),
                build.to_string(),
                path,
            );
        });
        println!("Downloaded {}! ({} elapsed)", colorize(&self.name, Color::Green), colorize(exec_time.as_str(), Color::Yellow));
        Ok(server)
    }

    fn to_string(&self) -> String {
        format!("{}: {}", self.name, self.download_url)
    }

    fn from_string(string: String) -> Self {
        let split: Vec<&str> = string.split(": ").collect();
        Self::new(split[0].to_string(), split[1].to_string(), "".to_string())
    }
}

impl Display for Jar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.download_url)
    }
}