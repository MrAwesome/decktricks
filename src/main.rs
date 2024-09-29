use clap::Parser;
use decktricks::actions::Cli;
use decktricks::executor::Executor;
use decktricks::prelude::*;

fn init_logger() -> DeckResult<()> {
    pretty_env_logger::try_init().map_err(KnownError::LoggerInitializationFail)
}

fn main() -> DeckResult<()> {
    let cli = Cli::parse();
    let action = &cli.command;
    let debug = cli.debug;

    init_logger()?;

    if debug {
        //debug!("Running in debug mode!");
    }

    let executor = Executor::new()?;
    let action_success = executor.execute(action)?;
    action_success.get_message().inspect(|m| println!("{}", m));

    Ok(())
}
