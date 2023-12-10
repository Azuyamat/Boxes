use std::fmt::{Display};
use std::fs::File;
use serde::Deserialize;
use crate::error::Error;
use crate::get_exec_time;
use crate::minecraft::server::Server;
use crate::utils::colorize;
use crate::utils::*;

const JARS_TOML: &str = include_str!("../../jars.toml");

pub fn load_jars() -> Result<JarManager, Error> {
    let jars: JarManager = toml::from_str(&JARS_TOML)?;

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
        println!("🗃️ Jar info:");
        println!("  💾 Jars:");
        if self.jars.is_empty() { println!("      No jars!"); }
        for jar in &self.jars {
            println!("      ➥ 📦 {}", jar.name);
        }
    }
}

#[derive(Deserialize)]
pub struct JarBuildInfo {
    pub project_id: String,
    pub project_name: String,
    pub version: String,
    pub builds: Vec<u32>,
}

#[derive(Deserialize)]
pub struct JarProjectInfo {
    pub project_id: String,
    pub project_name: String,
    pub version_groups: Vec<String>,
    pub versions: Vec<String>,
}


#[derive(Deserialize)]
pub struct Jar {
    pub name: String,
    pub download_url: String,
    pub builds_url: String,
    pub versions_url: String,
}

impl Jar {
    fn new(name: String, download_url: String, builds_url: String, versions_url: String) -> Self {
        Self {
            name,
            download_url,
            builds_url,
            versions_url,
        }
    }

    pub fn get_latest_build(&self, version: String) -> Result<String, Error> {
        Ok(self.get_builds(&version).unwrap().first().unwrap().to_string())
    }

    pub fn get_builds(&self, version: &String) -> Result<Vec<u32>, Error> {
        let url = self.builds_url.clone()
            .replace("{version}", version.as_str());
        let response = reqwest::blocking::get(&url)?;
        let mut body: JarBuildInfo = response.json()?;
        body.builds.reverse();
        Ok(body.builds)
    }

    pub fn download(&self, version: &str, build: &str, server_name: &str, location: String) -> Result<Server, Error> {
        println!("🗂️ Downloading {}...", colorize(&self.name, Color::Green));
        let server: Server;
        let exec_time = get_exec_time!({
            let download_url = self.download_url.clone()
                .replace("{version}", version)
                .replace("{build}", build);
            println!("🗂️ Downloading from {}...", colorize(&download_url, Color::LightPurple));
            let response = reqwest::blocking::get(&download_url)?;

            let path = std::path::Path::new(&location).join(format!("{0}", server_name));
            if !path.exists() { std::fs::create_dir_all(&path)?; }
            let mut file = File::create(&path.join(format!("{}-{}.jar", self.name, version)))?;

            download(response, &mut file);

            server = Server::new(
                server_name,
                self.name.clone(),
                version.to_string(),
                build.to_string(),
                path,
            );
        });
        println!("🗂️ Downloaded {}! ({} elapsed)", colorize(&self.name, Color::Green), colorize(exec_time.as_str(), Color::Yellow));
        Ok(server)
    }

    pub fn get_versions(&self) -> Result<Vec<String>, Error> {
        let response = reqwest::blocking::get(&self.versions_url)?;
        let mut body: JarProjectInfo = response.json()?;
        body.versions.reverse();
        Ok(body.versions)
    }

    fn to_string(&self) -> String {
        format!("{}: {}", self.name, self.download_url)
    }
}

impl Display for Jar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.download_url)
    }
}