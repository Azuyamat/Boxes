#![warn(
    clippy::cognitive_complexity,
    clippy::debug_assert_with_mut_call,
    clippy::doc_markdown,
    clippy::enum_glob_use,
    clippy::pedantic,
    clippy::complexity
)]

use crate::error::Error;
use crate::minecraft::jars;
use crate::minecraft::server::Server;
use crate::utils::{colorize, Color};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;
use std::process::Command;

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct Config {
    pub servers: Vec<ServerInfo>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct ServerInfo {
    pub server_name: String,
    pub location: PathBuf,
}

impl ServerInfo {
    pub fn from_server(server: &Server) -> Self {
        Self {
            server_name: server.server_name.clone(),
            location: server.location.clone(),
        }
    }
}

impl Config {
    pub fn print_info(&self) -> Result<(), Error> {
        println!("🗃️ Config info:");
        println!("  💾 Servers:");
        if self.servers.is_empty() {
            println!("      No servers!");
        }
        let running_servers = Command::new("jps").arg("-v").output()?;
        let running_servers = String::from_utf8(running_servers.stdout).unwrap_or_default();
        for server in &self.servers {
            let running =
                running_servers.contains(format!("-Dname={}", server.server_name.trim()).as_str());
            let running = if running {
                colorize("Running", Color::Green)
            } else {
                colorize("Stopped", Color::Red)
            };
            println!(
                "      ➥ 📦 {} ({}) ({})",
                server.server_name,
                server.location.display(),
                running
            );
        }
        jars::load().unwrap().print_info();
        Ok(())
    }

    pub fn load() -> Result<Self, Error> {
        let mut config: Self = confy::load("boxes", None)?;
        let mut changed = false;
        for server in &config.clone().servers {
            if !server.location.exists() {
                println!(
                    "⚠️ Server {} does not exist! Removing from config...",
                    server.server_name
                );
                config.servers.remove(
                    config
                        .servers
                        .iter()
                        .position(|s| s.location == server.location)
                        .unwrap(),
                );
                println!("⚠️ Server {} was removed from config!", server.server_name);
                changed = true;
            }
        }
        if changed {
            confy::store("boxes", None, config.clone())?;
        }
        Ok(config)
    }

    pub fn delete() -> Result<(), Error> {
        println!("📝 Deleting config...");
        let path = confy::get_configuration_file_path("config", None)?;
        fs::remove_file(path).expect("🚨 Config file could not be deleted!");
        println!("📝 Deleted config!");
        Ok(())
    }

    pub fn add_server(&mut self, server: &Server, save: bool) {
        println!("📝 Adding server to config... {}", server.server_name);
        if self.servers.iter().any(|s| s.location == server.location) {
            println!("⚠️ A server with the same location already exists! Overriding...");
            self.servers.remove(
                self.servers
                    .iter()
                    .position(|s| s.location == server.location)
                    .ok_or(Error::ResourceNotFound("Server not found".to_string()))
                    .unwrap(),
            );
            println!("⚠️ A server was overridden!");
        }
        let server_info = ServerInfo::from_server(server);
        self.servers.push(server_info);
        if save {
            confy::store("boxes", None, self).expect("🚨 Config file could not be saved!");
        }
        println!("📝 Added server to config!");
    }

    pub fn get_server(&self, server_name: &str) -> Option<Server> {
        let server_info = self
            .servers
            .iter()
            .find(|s| s.server_name.to_lowercase() == server_name.to_lowercase())?;
        let location_str = server_info.location.to_str()?;
        Server::from_path(location_str).ok()
    }

    pub fn save_server(&mut self, server: &Server) -> Result<(), Error> {
        println!("📝 Saving server to config...");
        let index = self
            .servers
            .iter()
            .position(|s| s.server_name == server.server_name)
            .ok_or(Error::ResourceNotFound("Server not found".to_string()))?;
        self.servers.remove(index);
        let server_info = ServerInfo::from_server(server);
        self.servers.push(server_info);
        confy::store("boxes", None, self)?;
        println!("📝 Saved server to config!");
        Ok(())
    }
}
