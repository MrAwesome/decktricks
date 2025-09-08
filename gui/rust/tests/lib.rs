use ctor::ctor;
use std::io::BufRead;
use std::io::BufReader;
use std::path::Path;
use std::path::PathBuf;
use std::process::Stdio;
use std::sync::LazyLock;
use std::time::Instant;
use std::{process::Command, time::Duration};
use wait_timeout::ChildExt;

const GODOT_BASE_DIR: &str = "../godot";
const GODOT_BUILD_BASE_DIR: &str = "../godot/build";
const SMOKE_CONFIG_PATH: &str = "../../config-smoke.json";
const INPUTS_DELIM: &str = "|DELIM|";

// If the GUI hasn't started in 5 seconds on a decently-fast system,
// something is wrong
const GUI_MAXIMUM_STARTUP_TIME_MS: u64 = 5000;

// If the GUI starts faster than half a second on a decently-fast system,
// something is wrong
const GUI_MINIMUM_STARTUP_TIME_MS: u64 = 500;

static GODOT_BINARY_PATH: LazyLock<PathBuf> =
    LazyLock::new(|| Path::new(GODOT_BUILD_BASE_DIR).join("decktricks-gui"));

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

fn apply_smoke_env<'a>(cmd: &'a mut Command, inputs: &str, post_delay_ms: u64) -> &'a mut Command {
    cmd.env("DECKTRICKS_CONFIG", SMOKE_CONFIG_PATH)
        .env("DECKTRICKS_GUI_EXIT_AFTER_INPUTS", "true")
        .env(
            "DECKTRICKS_GUI_INPUTS_POST_DELAY_MS",
            post_delay_ms.to_string(),
        )
        .env("DECKTRICKS_GUI_TEST_INPUTS", inputs);
    cmd
}

fn read_output_strings(output: &std::process::Output) -> (String, String) {
    let stdout = String::from_utf8_lossy(&output.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&output.stderr).into_owned();
    (stdout, stderr)
}

fn inputs_seq(actions: &[&str]) -> String {
    actions.join(INPUTS_DELIM)
}

fn assert_no_errors(stdout: &str, stderr: &str) {
    assert!(!stdout.contains("ERROR"));
    assert!(!stderr.contains("ERROR"));
}

fn assert_error_in_stderr_only(stdout: &str, stderr: &str) {
    assert!(!stdout.contains("ERROR"));
    assert!(stderr.contains("ERROR"));
}

fn assert_not_contains(haystack: &str, needles: &[&str]) {
    for n in needles {
        assert!(!haystack.contains(n));
    }
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
        .env(
            "DECKTRICKS_GUI_TEST_COMMAND_ONLY",
            "run-system-command|DELIM|--|DELIM|echo|DELIM|THISISMYTESTSTRINGYES",
        )
        .output()
        .unwrap();

    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    println!("STDOUT: {}", stdout);
    println!("STDERR: {}", stderr);
    assert!(!stdout.contains("ERROR"));
    assert!(!stderr.contains("ERROR"));
    assert!(stdout.contains("THISISMYTESTSTRINGYES"));
    assert!(stdout
        .trim_end()
        .ends_with("Decktricks GUI initialization complete!"));
    assert_eq!(output.status.code().unwrap(), 0);
}

// Smoke tests using an override config via DECKTRICKS_CONFIG.
#[test]
fn smoke_bbb_writevar() {
    use tempfile::NamedTempFile;
    let mut cmd = get_godot_cmd();
    let tmp = NamedTempFile::new().unwrap();
    let dest_path = tmp.path().to_string_lossy().to_string();
    let unique = format!(
        "decktricks-smoke-{}",
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_millis()
    );
    let input_seq = inputs_seq(&["ui_down", "ui_accept"]);
    let output = apply_smoke_env(&mut cmd, &input_seq, 600)
        .env("DECKTRICKS_BBB_WRITEVAR_CONTENTS", &unique)
        .env("DECKTRICKS_BBB_WRITEVAR_DESTINATION", &dest_path)
        .output()
        .unwrap();

    let (stdout, stderr) = read_output_strings(&output);
    let written = std::fs::read_to_string(&dest_path).expect("failed to read destination file");
    assert_eq!(written, unique);
    assert_no_errors(&stdout, &stderr);
}

#[test]
fn smoke_aaa_fail_direct_path() {
    let mut cmd = get_godot_cmd();
    let input_seq = inputs_seq(&["ui_accept"]);
    let output = apply_smoke_env(cmd.arg("--verbose"), &input_seq, 400)
        .output()
        .unwrap();

    let (stdout, stderr) = read_output_strings(&output);
    assert_not_contains(&stdout, &["BBB", "ZZZ"]);
    assert_not_contains(&stderr, &["BBB", "ZZZ"]);
    assert!(stdout.contains("AAA-fail ran, alright"));
    assert_error_in_stderr_only(&stdout, &stderr);
}

#[test]
fn smoke_aaa_fail_down_up() {
    let mut cmd = get_godot_cmd();
    let input_seq = inputs_seq(&["ui_down", "ui_up", "ui_accept"]);
    let output = apply_smoke_env(cmd.arg("--verbose"), &input_seq, 400)
        .output()
        .unwrap();

    let (stdout, stderr) = read_output_strings(&output);
    assert_not_contains(&stdout, &["BBB", "ZZZ"]);
    assert_not_contains(&stderr, &["BBB", "ZZZ"]);
    assert!(stdout.contains("AAA-fail ran, alright"));
    assert_error_in_stderr_only(&stdout, &stderr);
}

#[test]
fn smoke_zzz_fail_direct_path() {
    let mut cmd = get_godot_cmd();
    let input_seq = inputs_seq(&["ui_down", "ui_down", "ui_down", "ui_accept"]);
    let output = apply_smoke_env(&mut cmd, &input_seq, 400).output().unwrap();

    let (stdout, stderr) = read_output_strings(&output);
    assert_not_contains(&stdout, &["AAA", "BBB"]);
    assert_not_contains(&stderr, &["AAA", "BBB"]);
    assert!(stdout.contains("ZZZ-fail ran, alright"));
    assert_error_in_stderr_only(&stdout, &stderr);
}

#[test]
fn smoke_zzz_fail_many_downs() {
    let mut cmd = get_godot_cmd();
    let input_seq = inputs_seq(&[
        "ui_down",
        "ui_down",
        "ui_down",
        "ui_down",
        "ui_down",
        "ui_down",
        "ui_down",
        "ui_down",
        "ui_accept",
    ]);
    let output = apply_smoke_env(&mut cmd, &input_seq, 400).output().unwrap();

    let (stdout, stderr) = read_output_strings(&output);
    assert_not_contains(&stdout, &["AAA", "BBB"]);
    assert_not_contains(&stderr, &["AAA", "BBB"]);
    assert!(stdout.contains("ZZZ-fail ran, alright"));
    assert_error_in_stderr_only(&stdout, &stderr);
}

#[test]
fn smoke_many_cancels_plus_accept_hits_exit_button() {
    let mut cmd = get_godot_cmd();
    let mut child = cmd
        .env("DECKTRICKS_CONFIG", "../../config-smoke.json")
        .env("DECKTRICKS_GUI_EXIT_AFTER_INPUTS", "false")
        .env("DECKTRICKS_GUI_INPUTS_POST_DELAY_MS", "400")
        .env("DECKTRICKS_GUI_TEST_INPUTS", "ui_down|DELIM|ui_cancel|DELIM|ui_cancel|DELIM|ui_cancel|DELIM|ui_cancel|DELIM|ui_cancel|DELIM|ui_cancel|DELIM|ui_cancel|DELIM|ui_accept")
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    match child
        .wait_timeout(Duration::from_secs(GUI_MAXIMUM_STARTUP_TIME_MS))
        .unwrap()
    {
        Some(status) => {
            let stdout_reader = BufReader::new(child.stdout.take().expect("taking stdout"));
            let stderr_reader = BufReader::new(child.stderr.take().expect("taking stderr"));
            let stdout_lines: Vec<String> = stdout_reader.lines().map(|x| x.unwrap()).collect();
            let did_exit_with_exit_button = stdout_lines
                .iter()
                .any(|l| l == "EXITING BY PRESSING EXIT BUTTON");
            assert!(did_exit_with_exit_button);
            let saw_error = stdout_lines.iter().any(|l| l.contains("ERROR"))
                || stderr_reader.lines().any(|l| l.unwrap().contains("ERROR"));
            assert!(!saw_error);
            assert!(status.success());
        }
        None => {
            child.kill().unwrap();
            child.wait().unwrap();
            panic!("Did not press exit button in time, or exit button did not work!");
        }
    };
}

// TODO: Future tests:
// 1) in config-smoke.json, create CCC-pass: same logic as AAA-fail, but inverted (expect a pass and particular content on stdout). run it here.
// 2) ui_next_subtab->ui_accept should run ZZZ
// 3) ui_next_main_tab->ui_prev_main_tab->ui_down->ui_down->ui_left(x3)->ui_accept should run AAA
// 4) another exit button test, this time by pressing ui_next_main_tab 7 times and then ui_down and then ui_accept

#[test]
fn smoke_ccc_pass_direct_path() {
    let mut cmd = get_godot_cmd();
    let input_seq = inputs_seq(&["ui_down", "ui_down", "ui_accept"]);
    let output = apply_smoke_env(cmd.arg("--verbose"), &input_seq, 400)
        .output()
        .unwrap();

    let (stdout, stderr) = read_output_strings(&output);
    assert!(stdout.contains("CCC-pass ran, alright"));
    assert_no_errors(&stdout, &stderr);
    assert!(output.status.success());
}

#[test]
fn smoke_zzz_fail_next_subtab_then_accept() {
    let mut cmd = get_godot_cmd();
    let input_seq = inputs_seq(&["ui_next_subtab", "ui_down", "ui_left", "ui_left", "ui_accept"]);
    let output = apply_smoke_env(cmd.arg("--verbose"), &input_seq, 400)
        .output()
        .unwrap();

    let (stdout, stderr) = read_output_strings(&output);
    assert!(stdout.contains("ZZZ-fail ran, alright"));
    assert_error_in_stderr_only(&stdout, &stderr);
}

#[test]
fn smoke_aaa_fail_complex_navigation() {
    let mut cmd = get_godot_cmd();
    let input_seq = inputs_seq(&[
        "ui_next_main_tab",
        "ui_prev_main_tab",
        "ui_down",
        "ui_down",
        "ui_left",
        "ui_left",
        "ui_left",
        "ui_accept",
    ]);
    let output = apply_smoke_env(cmd.arg("--verbose"), &input_seq, 400)
        .output()
        .unwrap();

    let (stdout, stderr) = read_output_strings(&output);
    assert!(stdout.contains("AAA-fail ran, alright"));
    assert_error_in_stderr_only(&stdout, &stderr);
}

#[test]
fn smoke_exit_button_via_many_next_main_tabs() {
    let mut cmd = get_godot_cmd();
    let next_tabs = vec![
        "ui_next_main_tab",
        "ui_next_main_tab",
        "ui_next_main_tab",
        "ui_next_main_tab",
        "ui_next_main_tab",
        "ui_next_main_tab",
        "ui_next_main_tab",
        "ui_down",
        "ui_accept",
    ];
    let inputs = inputs_seq(&next_tabs);

    let mut child = cmd
        .env("DECKTRICKS_CONFIG", "../../config-smoke.json")
        .env("DECKTRICKS_GUI_EXIT_AFTER_INPUTS", "false")
        .env("DECKTRICKS_GUI_INPUTS_POST_DELAY_MS", "400")
        .env("DECKTRICKS_GUI_TEST_INPUTS", &inputs)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();

    match child
        .wait_timeout(Duration::from_secs(GUI_MAXIMUM_STARTUP_TIME_MS))
        .unwrap()
    {
        Some(status) => {
            let stdout_reader = BufReader::new(child.stdout.take().expect("taking stdout"));
            let stderr_reader = BufReader::new(child.stderr.take().expect("taking stderr"));
            let stdout_lines: Vec<String> = stdout_reader.lines().map(|x| x.unwrap()).collect();
            let did_exit_with_exit_button = stdout_lines
                .iter()
                .any(|l| l == "EXITING BY PRESSING EXIT BUTTON");
            assert!(did_exit_with_exit_button);
            let saw_error = stdout_lines.iter().any(|l| l.contains("ERROR"))
                || stderr_reader.lines().any(|l| l.unwrap().contains("ERROR"));
            assert!(!saw_error);
            assert!(status.success());
        }
        None => {
            child.kill().unwrap();
            child.wait().unwrap();
            panic!(
                "Did not press exit button in time, or exit button did not work!"
            );
        }
    };
}
