use crate::config_cli::ConfigAction;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    #[command(subcommand)]
    pub dj: DJ,
}

#[derive(Subcommand, Debug)]
pub enum DJ {
    /// Create a server with all arguments provided (name, jar, version, build, location)
    Create {
        name: String,
        jar: String,
        location: String,
        version: String,
        build: Option<String>,
    },
    /// Start a server with the given name
    Start { name: String },
    /// Config commands
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
    /// Server commands
    Server {
        #[command(subcommand)]
        action: ServerAction,
    },
}

#[derive(Subcommand, Debug)]
pub enum ServerAction {
    /// List all servers
    List,
    /// Generate a server with a wizard
    Generate,
    /// Get info about a server with the given name
    Info { name: String },
    /// Start a server with the given name
    Start { name: String },
    /// Delete a server with the given name
    Delete { name: String },
    /// Add a server with the given location
    Add { location: String },
    /// Get plugins for a server with the given name
    Plugins { name: String },
    /// Assign an IP to a server with the given name
    AssignIP { name: String, ip: String },
}
