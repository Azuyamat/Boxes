#![allow(dead_code)]

use clap::Parser;
use crate::config::Config;

mod error;
mod minecraft;
mod utils;
mod config;
mod cli;

use cli::constructor;
use cli::executor;

fn main() -> Result<(), error::Error> {
    let args = constructor::Args::parse();
    let config = Config::load()?;
    executor::execute(args, config)?;
    Ok(())
}