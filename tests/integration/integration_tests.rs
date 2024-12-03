use steam_shortcuts_util::Shortcut;
use crate::decktricks_cli;
use crate::integration::utils::run_cli_with_args;
use crate::integration::utils::BINARY_NAME;
use decktricks::prelude::DynamicError;
use std::collections::HashMap;
use std::{
    process::Command,
    time::{Duration, Instant},
};

// TODO:
// [] install/uninstall flatpak (optional, with network)
// [] test -c/-C behavior with test_config.json and test_config2.json

// TODO: move to utils when super error is fixed
// NOTE: running this with the default config as another layer of validation
#[test]
fn can_run_see_all_available_actions() -> Result<(), DynamicError> {
    decktricks_cli!["actions"]?;
    Ok(())
}

#[test]
fn broken_command_gives_error() -> Result<(), DynamicError> {
    let res = decktricks_cli!["JFKLDSJFDOISNFOIS"];
    assert!(res.is_err());
    Ok(())
}

// NOTE: running this with the default config to check the actual time taken in the real world
#[test]
fn time_see_all_available_actions() -> Result<(), DynamicError> {
    let see_all_max_time = Duration::from_millis(100);
    let start = Instant::now();
    decktricks_cli!["actions"]?;
    let time_taken = start.elapsed();
    if time_taken.gt(&see_all_max_time) {
        panic!(
            "ERROR: `actions` took too long! Taken: {:?} / Max: {:?}",
            time_taken, see_all_max_time
        );
    }
    Ok(())
}

// Run a "echo HARBLGARBL" through simple-command and make sure
// it returns successfully and doesn't have any chatty stderr
#[test]
fn simple_command_harblgarbl() -> Result<(), DynamicError> {
    let output = Command::new(BINARY_NAME)
        .args(vec![
            "-c",
            "tests/integration/test_config.json",
            "run",
            "print-HARBLGARBL",
        ])
        .output()?;

    if !output.status.success() {
        panic!("{:#?}", output);
    }

    // Ensure we don't have extraneous stderr happening
    let stderr = String::from_utf8_lossy(&output.stderr);
    assert_eq!("", stderr);

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert_eq!("HARBLGARBL", stdout.trim()); // stdout seems to have double newlines here?
    Ok(())
}

#[test]
fn test_config_exclusivity() -> Result<(), DynamicError> {
    let output = decktricks_cli!["-c", "tests/integration/test_config.json", "actions"]?;
    assert!(output.contains("print-HARBLGARBL"));
    assert!(output.contains("run"));
    assert!(output.contains("info"));
    assert!(!output.contains("decky"));

    Ok(())
}

#[test]
fn broken_config_gives_error() -> Result<(), DynamicError> {
    let res = decktricks_cli!["-c", "tests/integration/broken_config.json", "actions"];
    assert!(res.is_err());
    Ok(())
}

#[test]
fn missing_config_gives_error() -> Result<(), DynamicError> {
    let res = decktricks_cli!["-c", "tests/integration/jkfldjsaifdosaj.json", "actions"];
    assert!(res.is_err());
    Ok(())
}

#[test]
fn help_shown() -> Result<(), DynamicError> {
    let no_args_res = decktricks_cli![];
    assert!(no_args_res.is_err());
    let no_args_text = no_args_res.err().unwrap().to_string();
    assert!(no_args_text.contains("Commands:"));

    let shortname_res = decktricks_cli!["-h"];
    assert!(shortname_res.is_ok());
    let shortname_text = shortname_res?;
    assert!(shortname_text.contains("Commands:"));

    let longname_res = decktricks_cli!["--help"];
    assert!(longname_res.is_ok());
    let longname_text = longname_res?;
    assert!(longname_text.contains("Commands:"));
    Ok(())
}

#[test]
fn bad_arg() -> Result<(), DynamicError> {
    let badarg_res = decktricks_cli!["--hosidahfodiash"];
    assert!(badarg_res.is_err());
    let badarg_text = badarg_res.err().unwrap().to_string();
    assert!(badarg_text.contains("unexpected argument"));
    Ok(())
}

#[test]
fn can_add_to_steam() -> Result<(), DynamicError> {
    let file = tempfile::NamedTempFile::new()?;
    let filename = file.path().to_str().unwrap();

    let envs = HashMap::from([(
        "DECKTRICKS_OVERRIDE_STEAM_SHORTCUTS_FILE".into(),
        filename.into(),
    )]);
    run_cli_with_args(
        vec![
            "-c",
            "tests/integration/test_config.json",
            "add-to-steam",
            "print-HARBLGARBL",
        ],
        Some(envs.clone()),
    )?;

    let single_shortcut_file_content = std::fs::read(filename)?;
    let expected_single_shortcut_file_content = std::fs::read("tests/integration/shortcut_single.vdf")?;

    let single_shortcuts = steam_shortcuts_util::parse_shortcuts(&single_shortcut_file_content)?;
    let expected_single_shortcuts = steam_shortcuts_util::parse_shortcuts(&expected_single_shortcut_file_content)?;

    assert_eq!(single_shortcuts.len(), 1);
    assert_eq!(expected_single_shortcuts.len(), 1);

    shortcut_eq(&single_shortcuts[0], &expected_single_shortcuts[0]);

    run_cli_with_args(
        vec![
            "-c",
            "tests/integration/test_config.json",
            "add-to-steam",
            "test-package",
        ],
        Some(envs),
    )?;

    let double_shortcut_file_content = std::fs::read(filename)?;
    let expected_double_shortcut_file_content = std::fs::read("tests/integration/shortcut_double.vdf")?;

    let double_shortcuts = steam_shortcuts_util::parse_shortcuts(&double_shortcut_file_content)?;
    let expected_double_shortcuts = steam_shortcuts_util::parse_shortcuts(&expected_double_shortcut_file_content)?;

    assert_eq!(double_shortcuts.len(), 2);
    assert_eq!(expected_double_shortcuts.len(), 2);

    shortcut_eq(&double_shortcuts[0], &expected_double_shortcuts[0]);

    file.close()?;

    Ok(())
}

#[cfg(test)]
fn shortcut_eq(input: &Shortcut, expected: &Shortcut) {
    assert_eq!(input.order, expected.order);
    // NOTE: this will fail to match in GitHub Actions
    //       if the command paths in the config json do not match
    assert_eq!(input.app_id, expected.app_id);
    assert_eq!(input.app_name, expected.app_name);
    assert_eq!(input.exe, expected.exe);
    assert_eq!(input.start_dir, expected.start_dir);
    assert_eq!(input.icon, expected.icon);
    assert_eq!(input.launch_options, expected.launch_options);
    assert_eq!(input.shortcut_path, expected.shortcut_path);
}
