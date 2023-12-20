use notch::servers::server::Server;

mod config_cli;
pub mod constructor;
pub(crate) mod executor;
pub(crate) mod generator;

pub fn print_server_info(server: &Server) {
    println!("📦 Server info:");
    println!("  ➥ 📦 Name: {}", server.name);
    println!("  ➥ 📦 Jar: {}", server.jar.name);
    println!("  ➥ 📦 Version: {}", server.jar.version.as_ref().unwrap());
    println!("  ➥ 📦 Build: {}", server.jar.build.as_ref().unwrap());
    println!("  ➥ 📦 Location: {}", server.location.display());
    println!("  ➥ 📦 Initial memory: {}", server.settings.initial_memory);
    println!("  ➥ 📦 Max memory: {}", server.settings.max_memory);
    println!("  ➥ 📦 GUI: {}", server.settings.gui);
}
