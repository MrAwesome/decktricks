use clap::Parser;
use decktricks::actions::Cli;
use decktricks::executor::Executor;
use decktricks::prelude::*;

fn main() -> DeckResult<()> {
    pretty_env_logger::try_init().map_err(KnownError::LoggerInitializationFail)?;

    let cli = Cli::parse();
    let action = &cli.command;

    let executor = Executor::new()?;
    let results = executor.execute(action);

    let mut experienced_error = false;
    results.iter().for_each(|res|
        match res {
            Ok(action_success) => {
                action_success.get_message().inspect(|m| println!("{}", m));
            }
            Err(known_error) => {
                experienced_error = true;
                error!("{:?}", known_error);
            }
        }
    );

    Ok(())
}
