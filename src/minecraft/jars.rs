#![warn(clippy::pedantic, clippy::complexity, clippy::perf, clippy::style)]

use crate::error::Error;
use crate::get_exec_time;
use crate::minecraft::server::Server;
use crate::utils::colorize;
use crate::utils::{download, Color};
use serde::Deserialize;
use std::fmt::Display;
use std::fs::File;
use std::path::Path;

const JARS_TOML: &str = include_str!("../../jars.toml");

pub fn load() -> Result<JarManager, Error> {
    let jars: JarManager = toml::from_str(JARS_TOML)?;

    Ok(jars)
}

#[derive(Deserialize)]
pub struct JarManager {
    pub jars: Vec<Jar>,
}

impl JarManager {
    pub fn get_jar(&self, name: &str) -> Option<&Jar> {
        self.jars
            .iter()
            .find(|jar| jar.name.to_lowercase() == name.to_lowercase())
    }

    pub fn print_info(&self) {
        println!("🗃️ Jar info:");
        println!("  💾 Jars:");
        if self.jars.is_empty() {
            println!("      No jars!");
        }
        for jar in &self.jars {
            println!("      ➥ 📦 {}", jar.name);
        }
    }
}

#[derive(Deserialize)]
pub struct PaperJarBuildInfo {
    pub project_id: String,
    pub project_name: String,
    pub version: String,
    pub builds: Vec<u32>,
}

#[derive(Deserialize)]
pub struct PurpurJarBuildInfo {
    pub project: String,
    pub version: String,
    pub builds: PurpurBuildsInfo,
}

impl JarBuildInfo for PaperJarBuildInfo {
    fn get_builds(&mut self) -> Result<Vec<u32>, Error> {
        Ok(self.builds.clone())
    }
}

impl JarBuildInfo for PurpurJarBuildInfo {
    fn get_builds(&mut self) -> Result<Vec<u32>, Error> {
        Ok(self
            .builds
            .all
            .clone()
            .iter()
            .map(|s| s.parse::<u32>().unwrap())
            .collect())
    }
}

#[derive(Deserialize)]
pub struct PurpurBuildsInfo {
    pub latest: String,
    pub all: Vec<String>,
}

#[derive(Deserialize)]
pub struct JarProjectInfo {
    pub project: Option<String>,
    pub project_id: Option<String>,
    pub project_name: Option<String>,
    pub version_groups: Option<Vec<String>>,
    pub versions: Vec<String>,
}

#[derive(Deserialize)]
pub struct Jar {
    pub name: String,
    pub download_url: String,
    pub builds_url: String,
    pub versions_url: String,
}

pub trait JarBuildInfo {
    fn get_builds(&mut self) -> Result<Vec<u32>, Error>;
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

    pub fn get_latest_build(&self, version: &str) -> Option<u32> {
        let builds = self.get_builds(version).ok()?;
        builds.first().copied()
    }

    pub fn get_builds(&self, version: &str) -> Result<Vec<u32>, Error> {
        let url = self.builds_url.clone().replace("{version}", version);
        let response = reqwest::blocking::get(url)?;
        let lower_name = self.name.to_lowercase();
        let mut builds: Vec<u32>;
        if &lower_name == "paper" || &lower_name == "waterfall" || &lower_name == "velocity" {
            let mut body = response.json::<PaperJarBuildInfo>()?;
            builds = body.get_builds()?;
        } else if &lower_name == "purpur" {
            let mut body = response.json::<PurpurJarBuildInfo>()?;
            builds = body.get_builds()?;
        } else {
            return Err(Error::ResourceNotFound(
                "Jar build info not found".to_string(),
            ));
        }
        builds.reverse();
        Ok(builds)
    }

    pub fn download(
        &self,
        version: &str,
        build: &str,
        server_name: &str,
        location: &Path,
    ) -> Result<Server, Error> {
        println!("🗂️  Downloading {}...", colorize(&self.name, Color::Green));
        let server: Server;
        let exec_time = get_exec_time!({
            let download_url = self
                .download_url
                .clone()
                .replace("{version}", version)
                .replace("{build}", build);
            println!(
                "🗂️  Downloading from {}...",
                colorize(&download_url, Color::LightPurple)
            );
            let response = reqwest::blocking::get(&download_url)?;
            let mut file = File::create(location.join(format!("{}-{}.jar", self.name, version)))?;

            download(response, &mut file);

            server = Server::new(
                server_name,
                self.name.clone(),
                version.to_string(),
                build.to_string(),
                location,
            );
        });
        println!(
            "🗂️  Downloaded {}! ({} elapsed)",
            colorize(&self.name, Color::Green),
            colorize(exec_time.as_str(), Color::Yellow)
        );
        Ok(server)
    }

    pub fn get_versions(&self) -> Result<Vec<String>, Error> {
        let response = reqwest::blocking::get(&self.versions_url)?;
        let mut body: JarProjectInfo = response.json()?;
        body.versions.reverse();
        Ok(body.versions)
    }
}

impl Display for Jar {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.name, self.download_url)
    }
}
