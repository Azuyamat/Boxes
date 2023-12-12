use clap::Subcommand;
use crate::config::Config;
use crate::error::Error;

#[derive(Subcommand, Debug)]
pub enum ConfigAction {
    Info,
}

pub fn manage_config_action(action: ConfigAction, config: &Config) -> Result<(), Error> {
    match action {
        ConfigAction::Info => {
            config.print_info();
        }
    }
    Ok(())
}