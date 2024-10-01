use clap::Parser;
use decktricks::actions::Cli;
use decktricks::executor::Executor;
use decktricks::prelude::*;

fn main() -> DeckResult<()> {
    pretty_env_logger::try_init().map_err(KnownError::LoggerInitializationFail)?;

    let cli = Cli::parse();
    let action = &cli.command;

    let executor = Executor::new()?;
    let action_success = executor.execute(action)?;
    action_success.get_message().inspect(|m| println!("{}", m));

    Ok(())
}
