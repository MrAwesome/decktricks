use decktricks::tricks_config::TricksLoader;
use decktricks::prelude::*;
use clap::Parser;
use decktricks::actions::Cli;

fn main() -> Result<(), KnownError> {
    let loader = TricksLoader::from_default_config()?;
    let cli = Cli::parse();
    let action = &cli.command;

    let action_success = action.do_with(&loader)?;

    if let Some(message) = action_success.get_message() {
        println!("{}", message);
    }
    
    Ok(())
}

