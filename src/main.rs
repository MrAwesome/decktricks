use clap::Parser;
use decktricks::actions::Cli;
use decktricks::prelude::*;
use decktricks::tricks_config::TricksLoader;

fn main() -> DeckResult<()> {
    let loader = TricksLoader::from_default_config()?;
    let cli = Cli::parse();
    let action = &cli.command;

    let action_success = action.do_with(&loader)?;

    if let Some(message) = action_success.get_message() {
        println!("{}", message);
    }

    Ok(())
}
