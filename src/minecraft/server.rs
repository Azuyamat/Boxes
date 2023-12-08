use std::{env, thread};
use std::io::{BufRead, BufReader};
use std::path::PathBuf;
use std::process::{Command, Stdio};
use serde::{Deserialize, Serialize};
use crate::read_line;
use crate::utils::*;

#[derive(Deserialize, Serialize, Clone)]
pub struct Server {
    pub server_name: String,
    pub jar_name: String,
    pub version: String,
    pub build: String,

    pub location: PathBuf,
}

#[derive(Deserialize, Serialize)]
pub struct ServerInfo {
    pub server_name: String,
    pub jar_name: String,
    pub version: String,
    pub build: String,
}

impl ServerInfo {
    pub fn from_server(server: &Server) -> Self {
        Self {
            server_name: server.server_name.clone(),
            jar_name: server.jar_name.clone(),
            version: server.version.clone(),
            build: server.build.clone(),
        }
    }

    pub fn from_path(path: &PathBuf) -> Self {
        let server_info_toml = std::fs::read_to_string(path.parent().unwrap().join("server.toml")).unwrap();
        toml::from_str(&server_info_toml).unwrap()
    }
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
        };
        println!("📝 Saving server info...");
        let server_info = ServerInfo::from_server(&server);
        let server_info_toml = toml::to_string(&server_info).unwrap();
        let server_location = location.join("server.toml");
        std::fs::write(server_location, server_info_toml).unwrap();
        println!("📝 Saved server info!");
        server
    }

    pub fn print_info(&self) {
        let jar_name = colorize(self.jar_name.as_str(), Color::Gold);
        let version = colorize(self.version.as_str(), Color::Gold);
        let build = colorize(self.build.as_str(), Color::Gold);

        println!("\
            ===================\n\
            📦 Jar name: {}\n\
            📦 Version: {}\n\
            📦 Build: {}\n\
            ===================", jar_name, version, build)
    }

    pub fn run(&self) {
        self.print_info();
        println!("🚀 Starting {} server...", self.jar_name);

        let jar_dir = &self.location;
        env::set_current_dir(&jar_dir).expect("Failed to change directory");

        println!("📂 Changed directory to {}", jar_dir.to_str().unwrap());
        // Find jar in dir
        let jar_name = format!("{}-{}.jar", self.jar_name, self.version);

        // Run jar
        let mut child = Command::new("java")
            .arg("-jar")
            .arg(jar_name)
            .stdout(Stdio::piped())
            .spawn()
            .expect("Failed to run server");

        if let Some(ref mut stdout) = child.stdout {
            let reader = BufReader::new(stdout);

            for line in reader.lines() {
                let text = line.unwrap();
                if text.contains("You need to agree to the EULA in order to run the server. Go to eula.txt for more info.") {
                    println!("🚨 EULA not accepted! Would you like to accept? (y/n)");
                    let input = read_line!();
                    if input == "y" {
                        println!("🛑 Stopping server");
                        child.kill().expect("Failed to kill child");
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

        child.wait().expect("Failed to wait on child");
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
        let server_info = ServerInfo::from_path(&path);

        Self {
            server_name: server_info.server_name,
            jar_name: server_info.jar_name,
            version: server_info.version,
            build: server_info.build,
            location: path,
        }
    }
}