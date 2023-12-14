#![allow(dead_code)]

use crate::config::Config;
use clap::Parser;

mod cli;
mod config;
mod config_cli;
mod error;
mod minecraft;
mod themes;
mod utils;

use cli::constructor;
use cli::executor;
use themes::theme::Theme;

fn main() -> Result<(), error::Error> {
    let args = constructor::Args::parse();
    let config = Config::load()?;
    let theme = Theme::load()?;
    executor::execute(args, config, theme)?;
    Ok(())
}
