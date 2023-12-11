use std::{env, thread};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use inquire::Select;
use serde::{Deserialize, Serialize};
use crate::config::Config;
use crate::minecraft::jars;
use crate::read_line;
use crate::utils::*;

#[derive(Deserialize, Serialize, Clone)]
pub struct Server {
    pub server_name: String,
    pub jar_name: String,
    pub version: String,
    pub build: String,

    pub gui: bool,
    pub xms: Option<String>,
    pub xmx: Option<String>,

    pub location: PathBuf,
}

impl Server {
    pub fn new(server_name: &str, jar_name: String, version: String, build: String, location: PathBuf) -> Self {
        if !location.exists() { panic!("Jar file not found!"); }
        let server = Self {
            server_name: server_name.to_string(),
            jar_name,
            version,
            build,
            location: location.clone(),
            gui: false,
            xms: None,
            xmx: None,
        };
        println!("📝 Saving server info...");
        server.write();
        println!("📝 Saved server info!");
        server
    }

    pub fn print_info(&self) {
        let jar_name = colorize(self.jar_name.as_str(), Color::Gold);
        let version = colorize(self.version.as_str(), Color::Gold);
        let build = colorize(self.build.as_str(), Color::Gold);
        let location = colorize(self.location.to_str().unwrap(), Color::Gold);
        let gui = colorize(if self.gui { "true" } else { "false" }, Color::Gold);
        let xms = colorize(self.xms.as_ref().unwrap_or(&"Default".to_string()), Color::Gold);
        let xmx = colorize(self.xmx.as_ref().unwrap_or(&"Default".to_string()), Color::Gold);

        println!("\
            ===================\n\
            📦 Jar name: {}\n\
            📦 Version: {}\n\
            📦 Build: {}\n\
            📦 Location: {}\n\
            📦 GUI: {}\n\
            📦 Xms: {}\n\
            📦 Xmx: {}\n\
            ===================", jar_name, version, build, location, gui, xms, xmx);
    }

    pub fn run(&self) {
        let server_info = self.clone();
        self.print_info();
        println!("🚀 Starting {} server...", self.jar_name);

        let jar_dir = &self.location;
        env::set_current_dir(jar_dir).expect("Failed to change directory");

        println!("📂 Changed directory to {}", jar_dir.to_str().unwrap());
        // Find jar in dir
        let jar_name = format!("{}-{}.jar", self.jar_name, self.version);

        // Run jar
        let mut process = Command::new("java")
            .arg(format!("-Dname={}", server_info.server_name.trim()))
            .arg(format!("-Xms{}", server_info.xms.unwrap_or_else(|| "1G".to_string())))
            .arg(format!("-Xmx{}", server_info.xmx.unwrap_or_else(|| "1G".to_string())))
            .arg("-jar")
            .arg(jar_name)
            .arg(if !server_info.gui { "--nogui" } else { "" })
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to start child");

        if let Some(ref mut stdout) = process.stdout {
            let reader = BufReader::new(stdout);

            for line in reader.lines() {
                let text = line.unwrap();
                if text.contains("You need to agree to the EULA in order to run the server. Go to eula.txt for more info.") {
                    let input = read_line!("🚨 EULA not accepted! Would you like to accept? (y/n)").to_lowercase();
                    if input == "y" {
                        println!("🛑 Stopping server");
                        process.kill().expect("Failed to kill child");
                        self.accept_eula();
                        let server_clone = self.clone();
                        let server_copy = thread::spawn(move || {
                            server_clone.run();
                        });
                        server_copy.join().unwrap();
                        break;
                    } else {
                        println!("🚨 EULA not accepted! Exiting...");
                        break;
                    }
                }
                println!("{}", text);
            }
        }

        process.wait().expect("Failed to wait on child");
    }

    pub fn accept_eula(&self) {
        println!("📝 Accepting EULA...");
        let eula_path = self.location.join("eula.txt");
        if !eula_path.exists() {
            println!("🚨 EULA not found!");
            return;
        }
        let mut eula = std::fs::read_to_string(&eula_path).unwrap();
        eula = eula.replace("eula=false", "eula=true");
        std::fs::write(&eula_path, eula).unwrap();
        println!("📝 Accepted EULA!");
    }

    pub fn from_path(path: &str) -> Self {
        let path = PathBuf::from(path);
        let server_info = path.join("server_box.toml");
        if !server_info.exists() {
            let config = Config::load();
            println!("🚨 Server info not found!");
            let mut server_name;
            loop {
                server_name = read_line!("🎚️ Please enter the server name:");
                if config.get_server(&server_name).is_some() {
                    println!("⚠️ A server with the same name already exists! Please enter a different name:");
                    continue;
                }
                break;
            }
            let jars = jars::load_jars().unwrap();
            let jar_name = Select::new("🎚️ Please enter the server Jar", jars.jars.iter().map(|j| j.name.as_str()).collect::<Vec<&str>>()).prompt().expect("😧 Failed to get jar name");
            let jar = jars.get_jar(jar_name).expect("😧 Jar not found");
            let version = Select::new("🎚️ Please enter the server version", jar.get_versions().unwrap()).prompt().expect("😧 Failed to get jar name");
            let builds = jar.get_builds(&version).unwrap();
            let latest = builds.first().unwrap();
            let build = Select::new(&format!("🎚️ Please enter the jar build ({} is latest)", latest), builds).prompt().expect("😧 Failed to get jar build").to_string();
            let server = Server::new(
                &server_name,
                jar_name.to_string(),
                version,
                build,
                path.clone(),
            );
            server.write();
            return server;
        }
        let server_info = std::fs::read_to_string(server_info).unwrap();
        let server_info: Server = toml::from_str(&server_info).unwrap();
        server_info
    }

    pub fn delete(&self) {
        println!("📝 Deleting server...");
        std::fs::remove_dir_all(&self.location).expect("🚨 Failed to delete server!");
        println!("📝 Deleted server!");
    }

    pub fn plugins(&self) -> Vec<String> {
        let dir = self.location.join("plugins");
        if !dir.exists() {
            println!("🚨 Plugins directory not found!");
            return Vec::new();
        }
        dir.read_dir().unwrap().filter_map(|entry| {
            let name = entry.unwrap().file_name().into_string().unwrap();
            if name.ends_with(".jar") {
                Some(name)
            } else {
                None
            }
        }).collect()
    }

    pub fn write(&self) {
        let server_info = toml::to_string(&self).unwrap();
        let server_info_path = self.location.join("server_box.toml");
        std::fs::write(server_info_path, server_info).unwrap();
    }

    pub fn remove_plugin(&self, plugin: &str) {
        let dir = self.location.join("plugins");
        if !dir.exists() {
            println!("🚨 Plugins directory not found!");
            return;
        }
        let path = dir.join(plugin);
        if !path.exists() {
            println!("🚨 Plugin not found!");
            return;
        }
        std::fs::remove_file(path).unwrap();
        println!("📝 Removed plugin!");
    }
}