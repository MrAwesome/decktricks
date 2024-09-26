use decktricks::prelude::*;
use decktricks::tricks_config::TricksConfig;
use clap::Parser;
use decktricks::actions::Cli;

fn main() -> Result<(), KnownError> {
    let config = TricksConfig::from_default_config()?;
    let cli = Cli::parse();
    let action = &cli.command;

    let action_success = action.do_with(&config)?;

    if let Some(message) = action_success.get_message() {
        println!("{}", message);
    }
    
    Ok(())
}

