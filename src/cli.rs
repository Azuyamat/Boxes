use clap::{Parser, Subcommand};
use crate::config::Config;
use crate::error::Error;
use crate::minecraft::jars::{load_jars};

#[derive(Debug, Parser)]
#[command(author, version, about)]
pub struct Args {
    #[command(subcommand)]
    pub dj: DJ,
}

#[derive(Subcommand, Debug)]
pub enum DJ {
    Create {
        name: String,
        jar: String,
        version: String,
        build: String,
        location: String,
    },
    Start {
        name: String,
    },
    Config {
        #[command(subcommand)]
        action: ConfigAction,
    },
}

#[derive(Subcommand, Debug)]
pub enum ConfigAction {
    Info
}

// Actions

pub(crate) fn execute(args: Args, mut config: Config) -> Result<(), Error> {
    match args.dj {
        DJ::Create { name, jar, version, build, location } => {
            // Print with emoji
            println!("🔥 Creating server...");
            let jars = load_jars()?;
            let jar = jars.get_jar(&jar).expect("😧 Jar not found");
            let server = jar.download(&version, &build, &name, location).expect("😧 Failed to download jar (Check that the version and build exist)");
            config.add_server(&server);
        }
        DJ::Start { name } => {
            let server = config.get_server(&name).unwrap();
            server.run();
        }
        DJ::Config { action } => {
            match action {
                ConfigAction::Info => { config.print_info(); }
            }
        }
    }
    Ok(())
}