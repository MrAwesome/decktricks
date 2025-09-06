use std::path::PathBuf;
use std::path::Path;
use std::time::Instant;
use std::process::Stdio;
use std::{process::Command, time::Duration};
use wait_timeout::ChildExt;
use ctor::ctor;
use std::sync::LazyLock;

const GODOT_BASE_DIR: &str = "../godot";
const GODOT_BUILD_BASE_DIR: &str = "../godot/build";

// If the GUI hasn't started in 5 seconds on a decently-fast system,
// something is wrong
const GUI_MAXIMUM_STARTUP_TIME_MS: u64 = 5000;

// If the GUI starts faster than half a second on a decently-fast system,
// something is wrong
const GUI_MINIMUM_STARTUP_TIME_MS: u64 = 500;

static GODOT_BINARY_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    // Tests use debug or release based on build mode
    let is_debug = cfg!(debug_assertions);
    let build_dir = if is_debug {
        Path::new(GODOT_BUILD_BASE_DIR).join("debug")
    } else {
        Path::new(GODOT_BUILD_BASE_DIR).join("release")
    };
    build_dir.join("decktricks-gui")
});

#[ctor]
fn build_gui_once() {
    // Build/export the GUI once before any tests run.
    let is_debug = cfg!(debug_assertions);
    let mut build_cmd = Command::new("cargo");
    build_cmd.arg("run");
    if !is_debug {
        build_cmd.arg("--release");
    }
    build_cmd.args(["--bin", "gui-tool", "--", "build-and-export"]);
    let status = build_cmd.status().expect("failed to run gui-tool");
    assert!(status.success(), "gui-tool build/export failed");
}

fn get_godot_cmd() -> Command {
    let path = &*GODOT_BINARY_PATH;
    println!("Path: {}", path.to_str().unwrap());
    let mut cmd = Command::new(path);
    cmd.arg("--headless").current_dir(GODOT_BASE_DIR);
    cmd
}

#[test]
fn test_run_godot() {
    std::thread::sleep(Duration::from_secs(1));
    let mut cmd = get_godot_cmd();
    cmd.env("DECKTRICKS_GUI_EXIT_IMMEDIATELY", "true")
        .output()
        .unwrap();
}

// If the GUI was still running after 5 seconds, we can be sure that we
// at least finished _ready(), because test_gui_startup_speed
// ensures we start up faster than this
//
// Note the lack of DECKTRICKS_GUI_EXIT_IMMEDIATELY here - we want the
// GUI to run normally
#[test]
fn test_gui_timeout() {
    let dur = Duration::from_millis(GUI_MAXIMUM_STARTUP_TIME_MS);
    let mut cmd = get_godot_cmd();
    let mut child = cmd
        // If you are encountering issues, change these to Stdio::inherit() to debug
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    let status_code = match child.wait_timeout(dur).unwrap() {
        Some(status) => panic!("GUI exited early with status {status}!"),
        None => {
            // child hasn't exited yet, this is what we want
            child.kill().unwrap();
            child.wait().unwrap().code()
        }
    };
    assert!(status_code.is_none());
}

#[test]
fn test_gui_startup_speed() {
    std::thread::sleep(Duration::from_secs(2));
    let mut cmd = get_godot_cmd();
    let start = Instant::now();
    cmd.env("DECKTRICKS_GUI_EXIT_IMMEDIATELY", "true")
        .output()
        .unwrap();
    let dur = start.elapsed().as_millis();
    println!("DURATION: {}", dur);
    assert!(dur > (GUI_MINIMUM_STARTUP_TIME_MS as u128));
    // A full second lower than the timeout time, to avoid any ambiguity on whether or
    // not we're starting up quickly enough
    assert!(dur < ((GUI_MAXIMUM_STARTUP_TIME_MS - 1) as u128));
}

// Ensures that a command is passed through to the dispatcher,
// parsed, run, and returned successfully
#[test]
fn test_dispatcher_e2e() {
    std::thread::sleep(Duration::from_secs(3));
    let mut cmd = get_godot_cmd();
    let output = cmd
        .env("DECKTRICKS_GUI_EXIT_IMMEDIATELY", "true")
        .env("DECKTRICKS_GUI_TEST_COMMAND_ONLY", "run-system-command|DELIM|--|DELIM|echo|DELIM|THISISMYTESTSTRINGYES")
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("STDOUT: {}", stdout);
    println!("STDERR: {}", stderr);
    assert!(!stdout.contains("ERROR"));
    assert!(!stderr.contains("ERROR"));
    assert!(stdout.contains("THISISMYTESTSTRINGYES"));
    assert!(stdout.trim_end().ends_with("Decktricks GUI initialization complete!"));
    assert_eq!(output.status.code().unwrap(), 0);
}
