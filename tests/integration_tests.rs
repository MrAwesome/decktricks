use decktricks::prelude::*;
use std::{process::Command, time::{Duration, Instant}};

pub const BINARY_NAME: &str = env!("CARGO_BIN_EXE_decktricks");

// TODO:
// [] see-all-available-actions
// [] time see-all-available-actions
// [] install/uninstall flatpak (optional, with network)
// [] run with -c test_config.json and try:
//     [] simplecommand
// [] test -c/-C behavior with test_config.json and test_config2.json
// [] giving a broken config with -c errors

// NOTE: running this with the default config as another layer of validation
#[test]
fn can_run_see_all_available_actions() -> Result<(), DynamicError> {
    Command::new(BINARY_NAME)
        .args(vec!["see-all-available-actions"])
        .output()?;
    Ok(())
}

// NOTE: running this with the default config to check the actual time taken in the real world
#[test]
fn time_see_all_available_actions() -> Result<(), DynamicError> {
    let see_all_max_time = Duration::from_millis(100);
    let start = Instant::now();
    Command::new(BINARY_NAME)
        .args(vec!["see-all-available-actions"])
        .output()?;
    let time_taken = start.elapsed();
    if time_taken.gt(&see_all_max_time) {
        panic!("ERROR: `see-all-available-actions` took too long! Taken: {:?} / Max: {:?}", time_taken, see_all_max_time);
    }
    Ok(())
}

#[test]
fn simple_command_harblgarbl() -> Result<(), DynamicError> {
    let output = Command::new(BINARY_NAME)
        .args(vec!["-c", "tests/test_config.json", "run", "print-HARBLGARBL"])
        .output()?;

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!("HARBLGARBL", stdout);

    // Ensure we don't have extraneous stderr happening
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!("", stderr);
    panic!()
}
