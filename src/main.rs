use decktricks::decktricks_logging_init;
use decktricks::prelude::*;

decktricks_logging_init!(LogType::Warn);

fn main() -> DeckResult<()> {
    let cmd = DecktricksCommand::parse();

    let logger = CRATE_DECKTRICKS_DEFAULT_LOGGER.clone();
    let log_level = cmd.log_level.unwrap_or(get_log_level());
    let executor = Executor::create_with_gather(ExecutorMode::OnceOff, log_level, logger.clone(), Some(&cmd));
    let (_ctx, results) = executor.execute(&cmd, logger);

    let mut experienced_error = false;
    results.iter().for_each(|res| match res {
        Ok(action_success) => {
            action_success.get_message().inspect(|m| println!("{}", m));
        }
        Err(known_error) => {
            experienced_error = true;
            eprintln!("{}", known_error);
        }
    });

    if experienced_error {
        std::process::exit(1);
    } else {
        Ok(())
    }
}
