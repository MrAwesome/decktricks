use decktricks::run::run_action_with_config;
use decktricks::prelude::*;
use decktricks::tricks_config::TricksConfig;
use clap::Parser;
use decktricks::actions::Cli;

fn main() -> Result<(), DynamicError> {
    let config = TricksConfig::from_default_config()?;
    let cli = Cli::parse();
    let action = &cli.command;

    let action_success = run_action_with_config(action, &config)?;

    if let Some(message) = action_success.message {
        println!("{}", message);
    }
    
    Ok(())
}

