use std::env;
use std::ffi::OsStr;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

const GODOT_BASE_DIR: &str = "../godot";
const GODOT_BUILD_BASE_DIR: &str = "../godot/build";
const GODOT_CACHE_DIR: &str = "../godot/.godot";
const GODOT_CDYLIB_NAME: &str = "libdecktricks_godot_gui.so";

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    BuildAndExport,
    BuildLibOnly,
    ExportOnly,
    PrintBinaryPath,
}

fn main() {
    let mut args = env::args().skip(1).collect::<Vec<String>>();
    let mut clean = false;

    // Minimal arg parsing to avoid extra deps.
    // Usage:
    //   gui-tool [--clean] [build-and-export|build-lib|export|print-binary-path]
    // Defaults to build-and-export.
    args.retain(|a| {
        if a == "--clean" {
            clean = true;
            false
        } else {
            true
        }
    });

    let action = match args.get(0).map(|s| s.as_str()) {
        None => Action::BuildAndExport,
        Some("build-and-export") | Some("build") => Action::BuildAndExport,
        Some("build-lib") => Action::BuildLibOnly,
        Some("export") => Action::ExportOnly,
        Some("print-binary-path") => Action::PrintBinaryPath,
        Some(other) => {
            eprintln!(
                "Unknown subcommand: {}\nUsage: gui-tool [--clean] [build-and-export|build-lib|export|print-binary-path]",
                other
            );
            std::process::exit(2);
        }
    };

    if clean {
        clean_build_dirs();
    }

    let is_debug_build = cfg!(debug_assertions);
    match action {
        Action::BuildAndExport => {
            ensure_build_dir();
            cargo_build_cdylib(is_debug_build);
            copy_cdylib_into_godot_build(is_debug_build);
            godot_import();
            godot_export(is_debug_build);
            println!("{}", get_exported_binary_path(is_debug_build).display());
        }
        Action::BuildLibOnly => {
            ensure_build_dir();
            cargo_build_cdylib(is_debug_build);
            copy_cdylib_into_godot_build(is_debug_build);
        }
        Action::ExportOnly => {
            ensure_build_dir();
            godot_import();
            godot_export(is_debug_build);
            println!("{}", get_exported_binary_path(is_debug_build).display());
        }
        Action::PrintBinaryPath => {
            println!("{}", get_exported_binary_path(is_debug_build).display());
        }
    }
}

fn ensure_build_dir() {
    let build_path = Path::new(GODOT_BUILD_BASE_DIR);
    if !build_path.is_dir() {
        fs::create_dir_all(build_path).expect("failed to create Godot build directory");
    }
}

fn clean_build_dirs() {
    let build_path = Path::new(GODOT_BUILD_BASE_DIR);
    if build_path.is_dir() {
        fs::remove_dir_all(build_path).expect("failed to clean Godot build directory");
    }
    let cache_path = Path::new(GODOT_CACHE_DIR);
    if cache_path.is_dir() {
        fs::remove_dir_all(cache_path).expect("failed to clean Godot cache directory");
    }
}

fn cargo_build_cdylib(is_debug_build: bool) {
    let mut cmd = Command::new("cargo");
    cmd.arg("build").arg("--lib");
    if !is_debug_build {
        cmd.arg("--release");
    }
    let human = if is_debug_build { "cargo build (debug)" } else { "cargo build --release" };
    run_cmd(cmd, human);
    let lib_path = cdylib_path(is_debug_build);
    if !lib_path.is_file() {
        eprintln!("Expected cdylib missing: {}", lib_path.display());
        std::process::exit(1);
    }
}

fn copy_cdylib_into_godot_build(is_debug_build: bool) {
    let build_dir = get_build_dir(is_debug_build);
    fs::create_dir_all(&build_dir).expect("failed to create build subdirectory");
    let tmp_path = build_dir.join(format!("{}.new", GODOT_CDYLIB_NAME));
    fs::copy(cdylib_path(is_debug_build), &tmp_path).expect("failed to copy cdylib to tmp path");
    let final_path = build_dir.join(GODOT_CDYLIB_NAME);
    fs::rename(&tmp_path, &final_path).expect("failed to move cdylib into place");
}

fn godot_import() {
    let godot_bin = resolve_godot_bin();
    let mut cmd = Command::new(&godot_bin);
    cmd.current_dir(GODOT_BASE_DIR)
        .args(["--import"]) // ensure import database updated
        .args(["--headless"]) // run without GUI for CI
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit());
    run_cmd(cmd, &format!("{} --import --headless", godot_bin));
}

fn godot_export(is_debug_build: bool) {
    let output_path = get_exported_binary_path(is_debug_build);
    let godot_bin = resolve_godot_bin();
    let mut cmd = Command::new(&godot_bin);
    cmd.current_dir(GODOT_BASE_DIR)
        .arg("--headless");
    if is_debug_build {
        cmd.args(["--export-debug", "linux-debug", &output_path.to_string_lossy()]);
    } else {
        cmd.args(["--export-release", "linux-release", &output_path.to_string_lossy()]);
    }
    let human = if is_debug_build {
        format!("{} --headless --export-debug linux-debug", godot_bin)
    } else {
        format!("{} --headless --export-release linux-release", godot_bin)
    };
    let output = run_cmd_capture(cmd, &human);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    // We can't trust Godot to exit on an error, so we manually check for ERROR
    // in the output of the build, as well as a run of the binary below
    if stdout.contains("ERROR") || stderr.contains("ERROR") {
        eprintln!("Godot export reported errors");
        eprintln!("STDOUT:\n{}", stdout);
        eprintln!("STDERR:\n{}", stderr);
        std::process::exit(1);
    }
    let bin_path = get_exported_binary_path(is_debug_build);
    if !bin_path.is_file() {
        eprintln!("Expected exported binary missing: {}", bin_path.display());
        std::process::exit(1);
    }
}

fn resolve_godot_bin() -> String {
    if let Ok(bin) = env::var("GODOT_BIN") {
        if !bin.trim().is_empty() {
            return bin;
        }
    }
    // Prefer godot4 if available; otherwise fall back to godot
    if Command::new("godot4")
        .arg("--version")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .status()
        .map(|s| s.success())
        .unwrap_or(false)
    {
        return "godot4".to_string();
    }
    "godot".to_string()
}

fn get_exported_binary_path(is_debug_build: bool) -> PathBuf {
    get_build_dir(is_debug_build).join("decktricks-gui")
}

fn get_build_dir(is_debug_build: bool) -> PathBuf {
    if is_debug_build {
        Path::new(GODOT_BUILD_BASE_DIR).join("debug")
    } else {
        Path::new(GODOT_BUILD_BASE_DIR).join("release")
    }
}

fn run_cmd(mut cmd: Command, human: &str) {
    let status = cmd.status().unwrap_or_else(|e| {
        eprintln!("Failed to run {}: {}", human, e);
        std::process::exit(1);
    });
    if !status.success() {
        eprintln!("Command failed ({}): {:?}", human, cmd_debug(&cmd));
        std::process::exit(status.code().unwrap_or(1));
    }
}

fn run_cmd_capture(mut cmd: Command, human: &str) -> std::process::Output {
    let output = cmd.output().unwrap_or_else(|e| {
        eprintln!("Failed to run {}: {}", human, e);
        std::process::exit(1);
    });
    if !output.status.success() {
        eprintln!("Command failed ({}): {:?}", human, cmd_debug(&cmd));
        eprintln!("STDOUT:\n{}", String::from_utf8_lossy(&output.stdout));
        eprintln!("STDERR:\n{}", String::from_utf8_lossy(&output.stderr));
        std::process::exit(output.status.code().unwrap_or(1));
    }
    output
}

fn cdylib_path(is_debug_build: bool) -> PathBuf {
    if is_debug_build {
        Path::new("target/debug").join(GODOT_CDYLIB_NAME)
    } else {
        Path::new("target/release").join(GODOT_CDYLIB_NAME)
    }
}

fn cmd_debug(cmd: &Command) -> String {
    let prog = cmd.get_program();
    let args: Vec<&OsStr> = cmd.get_args().collect();
    format!(
        "{} {}",
        prog.to_string_lossy(),
        args.into_iter()
            .map(|a| a.to_string_lossy().to_string())
            .collect::<Vec<_>>()
            .join(" ")
    )
}


