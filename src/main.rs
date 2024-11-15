use decktricks::prelude::*;

fn main() -> DeckResult<()> {
    pretty_env_logger::try_init().map_err(KnownError::LoggerInitializationFail)?;

    let cmd = DecktricksCommand::parse();

    let executor = Executor::new(&cmd)?;
    let results = executor.execute(&cmd.action);

    let mut experienced_error = false;
    results.iter().for_each(|res| match res {
        Ok(action_success) => {
            action_success.get_message().inspect(|m| println!("{}", m));
        }
        Err(known_error) => {
            experienced_error = true;
            error!("{}", known_error);
        }
    });

    if experienced_error {
        std::process::exit(1);
    } else {
        Ok(())
    }
}
