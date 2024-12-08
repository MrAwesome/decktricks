use decktricks::decktricks_logging_init;
use decktricks::prelude::*;

decktricks_logging_init!();

fn main() -> DeckResult<()> {
    let cmd = DecktricksCommand::parse();

    let executor = Executor::new(ExecutorMode::OnceOff, &cmd)?;
    let results = executor.execute(&cmd.action);

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
