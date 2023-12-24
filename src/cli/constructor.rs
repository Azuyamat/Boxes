use crate::cli::config_cli::ConfigAction;
use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    #[command(subcommand)]
    pub dj: DJ,

    #[clap(short, long)]
    pub verbose: bool, // 🗨️
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
    /// Start an existing server by providing its name
    Start { name: String },
    /// Commands related to server configuration settings
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
    /// Commands related to server management
    Server {
        #[command(subcommand)]
        action: ServerAction,
    },
    /// Commands related to theme configuration
    Theme {
        #[command(subcommand)]
        action: ThemeAction,
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
    Plugins {
        #[command(subcommand)]
        action: ServerPluginAction
    },
    /// Remove a server with the given name from the config
    Remove { name: String },
}

#[derive(Subcommand, Debug)]
pub enum ServerPluginAction {
    /// List all plugins for a server with the given name
    List { name: String },
    /// Add a plugin to a server with the given name
    Add { name: String, plugin: String },
    /// Remove a plugin from a server with the given name
    Remove { name: String, plugin: String },
}

#[derive(Subcommand, Debug)]
pub enum ThemeAction {
    /// List all available themes
    List,
    /// Create a new theme
    Create,
    /// Delete a theme
    Delete,
    /// Get info about a theme with the given name
    Info { name: String },
}
