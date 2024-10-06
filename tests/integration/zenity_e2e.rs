// Comment this out to make LSP features work
#![cfg(feature = "e2e")]

use std::process::Command;
use std::thread;
use std::time::Duration;

use crate::decktricks_cli;
use decktricks::prelude::DeckResult;

fn simulate_keypress(window_id: &str, keyname: &str) {
    let status = Command::new("xdotool")
        .arg("windowfocus")
        .arg(window_id)
        .status()
        .expect("Failed to focus window");

    dbg!(&status);

    Command::new("xdotool")
        .arg("key")
        .arg(keyname)
        .status()
        .expect("Failed to simulate key press");
}

fn get_zenity_window_id() -> String {
    let output = Command::new("xdotool")
        .arg("search")
        .arg("--onlyvisible")
        .arg("--classname")
        .arg("[zZ]enity")
        .output()
        .expect("Failed to search window");

    String::from_utf8(output.stdout)
        .expect("Failed to convert output to string")
        .trim()
        .to_string()
        .lines()
        .next()
        .unwrap()
        .into()
}

#[test]
fn test_zenity_e2e() -> DeckResult<()> {
    let thrd = thread::spawn(|| {
        decktricks_cli!["-c", "tests/integration/test_config.json", "gui", "zenity"]
    });

    thread::sleep(Duration::from_secs(5));

    let window_id = get_zenity_window_id();
    simulate_keypress(window_id.as_ref(), "return");
    thread::sleep(Duration::from_secs(5));

    let window_id = get_zenity_window_id();
    simulate_keypress(window_id.as_ref(), "return");
    thread::sleep(Duration::from_secs(5));

    let window_id = get_zenity_window_id();
    simulate_keypress(window_id.as_ref(), "space");
    thread::sleep(Duration::from_secs(1));

    let output = thrd.join().unwrap().unwrap();
    assert_eq!("HARBLGARBL", output.trim());
    Ok(())
}
