use std::path::PathBuf;
use std::process::Command;
use serde::{Serialize, Deserialize};
use crate::minecraft::jars;
use crate::minecraft::server::Server;
use crate::utils::{Color, colorize};

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
    pub fn print_info(&self) {
        println!("🗃️ Config info:");
        println!("  💾 Servers:");
        if self.servers.is_empty() { println!("      No servers!"); }
        let running_servers = Command::new("jps")
            .arg("-v")
            .output().unwrap();
        let running_servers = String::from_utf8(running_servers.stdout).unwrap_or_default();
        for server in &self.servers {
            let running = running_servers.contains(format!("-Dname={}", server.server_name.trim()).as_str());
            let running = if running { colorize("Running", Color::Green) } else { colorize("Stopped", Color::Red) };
            println!("      ➥ 📦 {} ({}) ({})", server.server_name, server.location.display(), running);
        }
        jars::load_jars().unwrap().print_info();
    }

    pub fn load() -> Self {
        let mut config: Self = confy::load("boxes", None).expect("🚨 Config file could not be loaded!");
        let mut changed = false;
        for server in config.clone().servers.iter() {
            if !server.location.exists() {
                println!("⚠️ Server {} does not exist! Removing from config...", server.server_name);
                config.servers.remove(config.servers.iter().position(|s| s.location == server.location).unwrap());
                println!("⚠️ Server {} was removed from config!", server.server_name);
                changed = true;
            }
        }
        if changed { confy::store("boxes", None, config.clone()).expect("🚨 Config file could not be saved!"); }
        config
    }

    pub fn delete() {
        println!("📝 Deleting config...");
        let path = confy::get_configuration_file_path("config", None).expect("🚨 Config file could not be found!");
        std::fs::remove_file(path).expect("🚨 Config file could not be deleted!");
        println!("📝 Deleted config!");
    }

    pub fn add_server(&mut self, server: &Server) {
        println!("📝 Adding server to config...");
        if self.servers.iter().any(|s| s.location == server.location) {
            println!("⚠️ A server with the same location already exists! Overriding...");
            self.servers.remove(self.servers.iter().position(|s| s.location == server.location).unwrap());
            println!("⚠️ A server was overridden!");
        }
        let server_info = ServerInfo::from_server(server);
        self.servers.push(server_info);
        confy::store("boxes", None, self).expect("🚨 Config file could not be saved!");
        println!("📝 Added server to config!");
    }

    pub fn get_server(&self, server_name: &str) -> Option<Server> {
        let server_info = self.servers.iter().find(|s| s.server_name.to_lowercase() == server_name.to_lowercase());
        server_info?;
        Some(Server::from_path(server_info.unwrap().location.to_str().unwrap()))
    }

    pub fn save_server(&mut self, server: &Server) {
        println!("📝 Saving server to config...");
        let index = self.servers.iter().position(|s| s.server_name == server.server_name).unwrap();
        self.servers.remove(index);
        let server_info = ServerInfo::from_server(server);
        self.servers.push(server_info);
        confy::store("boxes", None, self).expect("🚨 Config file could not be saved!");
        println!("📝 Saved server to config!");
    }
}