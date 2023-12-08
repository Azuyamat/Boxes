use serde::{Serialize, Deserialize};
use crate::minecraft::server::Server;

#[derive(Deserialize, Serialize, Clone)]
pub struct Config {
    pub servers: Vec<Server>,
}

impl Config {
    pub fn print_info(&self) {
        println!("🗃️ Config info:");
        println!("  💾 Servers:");
        for server in &self.servers {
            println!("      ➥ 📦 {}", server.server_name);
        }
    }

    pub fn load() -> Self {
        let mut config: Self = confy::load("config", None).expect("🚨 Config file could not be loaded!");
        let mut changed = false;
        for server in config.clone().servers.iter() {
            if !server.location.exists() {
                println!("⚠️ Server {} does not exist! Removing from config...", server.server_name);
                config.servers.remove(config.servers.iter().position(|s| s.location == server.location).unwrap());
                println!("⚠️ Server {} was removed from config!", server.server_name);
                changed = true;
            }
        }
        if changed { confy::store("config", None, config.clone()).expect("🚨 Config file could not be saved!"); }
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
        self.servers.push(server.clone());
        confy::store("config", None, self).expect("🚨 Config file could not be saved!");
        println!("📝 Added server to config!");
    }

    pub fn get_server(&self, server_name: &str) -> Option<&Server> {
        self.servers.iter().find(|s| s.server_name.to_lowercase() == server_name.to_lowercase())
    }
}

impl Default for Config {
    fn default() -> Self {
        Self {
            servers: vec![],
        }
    }
}