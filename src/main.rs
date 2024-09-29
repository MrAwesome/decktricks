use clap::Parser;
use decktricks::actions::Cli;
use decktricks::executor::Executor;
use decktricks::prelude::*;

fn main() -> DeckResult<()> {
    let action = &Cli::parse().command;
    let executor = Executor::new()?;
    let action_success = executor.execute(action)?;
    action_success.get_message().inspect(|m| println!("{}", m));

    Ok(())
}
