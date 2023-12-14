use std::collections::HashMap;
use std::io::Write;
use crate::minecraft::server::Server;

// Integration to read/write/modify server info files
pub struct ServerManipulator {
    pub server: Server,
}

impl ServerManipulator {
    pub fn get_server_properties(&self) -> Option<HashMap<String, String>> {
        let mut properties = std::collections::HashMap::new();
        let path = self.server.location.join("server.properties");
        if !path.exists() {
            println!("🚨 server.properties not found!");
            return None;
        }
        let file = std::fs::read_to_string(&path).unwrap();
        for line in file.lines() {
            if line.starts_with('#') { continue; }
            let mut split = line.split('=');
            let key = split.next().unwrap();
            let value = split.next().unwrap();
            properties.insert(key.to_string(), value.to_string());
        }
        Some(properties)
    }

    pub fn save_server_properties(&self, properties: &std::collections::HashMap<String, String>) {
        let mut file = std::fs::File::create(self.server.location.join("server.properties")).unwrap();
        for (key, value) in properties {
            file.write_all(format!("{}={}\n", key, value).as_bytes()).unwrap();
        }
    }
}