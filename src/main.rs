#![allow(dead_code)]

use clap::Parser;
use crate::config::Config;

mod error;
mod minecraft;
mod utils;
mod config;
mod cli;
mod config_cli;

fn main() -> Result<(), error::Error> {
    let args = cli::Args::parse();
    let config = Config::load()?;
    cli::execute(args, config)?;
    Ok(())
}