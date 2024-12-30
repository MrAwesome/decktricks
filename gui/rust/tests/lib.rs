use std::path::PathBuf;
use std::path::Path;
use std::time::Instant;
use std::process::Stdio;
use std::{process::Command, time::Duration};
use wait_timeout::ChildExt;
use std::sync::LazyLock;
use std::fs;

const GODOT_BASE_DIR: &str = "../godot";
const GODOT_BUILD_DIR: &str = "../godot/build";

// If the GUI hasn't started in 5 seconds on a decently-fast system,
// something is wrong
const GUI_MAXIMUM_STARTUP_TIME_MS: u64 = 5000;

// If the GUI starts faster than half a second on a decently-fast system,
// something is wrong
const GUI_MINIMUM_STARTUP_TIME_MS: u64 = 500;

// The pre-test build steps.
//
// You MUST access this variable, using &*GODOT_BINARY_PATH, in each test here to ensure that
// the dylib/binary is actually built and placed, since test ordering is not guaranteed.
// `get_godot_cmd()` will do this for you.
//
// NOTE: the first test to run will trigger this, and will be much slower as a result.
static GODOT_BINARY_PATH: LazyLock<PathBuf> = LazyLock::new(|| {
    if cfg!(debug_assertions) {
        panic!("GUI tests should not be run in debug mode! Use `cargo test --release`.");
    }

    let path = Path::new(GODOT_BUILD_DIR);
    if path.is_dir() {
        fs::remove_dir_all(path).unwrap();
    }

    // NOTE: for production builds, you want to delete the ../godot/.godot directory
    //       as well. That is handled in gui.sh in CI, but if you're encountering
    //       weird behavior in local builds after major changes you can try that as well.

    Command::new("cargo")
        .args(["build", "--release"])
        .output()
        .unwrap();

    fs::create_dir_all(path).unwrap();

    fs::copy(
        "target/release/libdecktricks_godot_gui.so",
        Path::join(path, Path::new("libdecktricks_godot_gui.so")),
    )
    .unwrap();

    let output = Command::new("godot")
        .current_dir(GODOT_BASE_DIR)
        .args(["--headless", "--export-release", "Linux"])
        .output()
        .unwrap();

    // We can't trust Godot to exit on an error, so we manually check for ERROR
    // in the output of the build, as well as a run of the binary below
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    if stdout.contains("ERROR") || stderr.contains("ERROR") {
        println!("STDOUT: {}", stdout);
        println!("STDERR: {}", stderr);
    }
    assert!(!stdout.contains("ERROR"));
    assert!(!stderr.contains("ERROR"));

    Path::join(path, Path::new("decktricks-gui"))
});

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
