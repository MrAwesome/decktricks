use clap::Parser;
use decktricks::actions::Cli;
use decktricks::prelude::*;
use decktricks::tricks_config::TricksLoader;

fn main() -> DeckResult<()> {
    // TODO: coalesce all of this into a Runner class
    let loader = TricksLoader::from_default_config()?;
    let full_ctx = FullSystemContext::try_gather()?;

    let cli = Cli::parse();
    let action = &cli.command;

    let action_success = action.do_with(&loader, &full_ctx)?;

    if let Some(message) = action_success.get_message() {
        println!("{}", message);
    }

    Ok(())
}
