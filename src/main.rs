use clap::Parser;
use decktricks::actions::Cli;
use decktricks::executor::Executor;
use decktricks::prelude::*;
use simplelog::*;

fn init_logger(debug: bool) -> DeckResult<()> {
    TermLogger::init(
        if debug {
            LevelFilter::Debug
        } else {
            LevelFilter::Info
        },
        Config::default(),
        TerminalMode::Mixed,
        ColorChoice::Auto,
    )
    .map_err(KnownError::LoggerInitializationFail)
}

fn main() -> DeckResult<()> {
    let cli = Cli::parse();
    let action = &cli.command;
    let debug = cli.debug;

    init_logger(debug)?;

    if debug {
        //debug!("Running in debug mode!");
    }

    let executor = Executor::new()?;
    let action_success = executor.execute(action)?;
    action_success.get_message().inspect(|m| println!("{}", m));

    Ok(())
}
