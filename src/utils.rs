use crate::prelude::*;

#[allow(clippy::unnecessary_wraps)]
#[cfg(test)]
pub(crate) fn run_remote_script(url: &str, local_filename: &str) -> Result<(), DynamicError> {
    warn!("Not running run_remote_script({url}, {local_filename}) from test...");
    Ok(())
}

#[cfg(not(test))]
pub(crate) fn run_remote_script(url: &str, local_filename: &str) -> Result<(), DynamicError> {
    use std::fs::File;
    use std::io::copy;
    use std::os::unix::fs::PermissionsExt;
    use std::process::Command;

    let response = reqwest::blocking::get(url)?;

    // These are in blocks to ensure that files are closed out
    // before attempting to do further changes
    {
        let mut dest = File::create(local_filename)?;
        copy(&mut response.bytes()?.as_ref(), &mut dest)?;
    }

    {
        std::fs::set_permissions(local_filename, std::fs::Permissions::from_mode(0o755))?;
    }

    Command::new(local_filename).status()?;
    Ok(())
}

pub(crate) fn get_homedir() -> String {
    std::env::var("HOME").unwrap_or_else(|_| "/home/deck".to_string())
}

pub(crate) fn exists_and_executable(runner: &RunnerRc, path: &str) -> bool {
    // Using this instead of rust-native code to piggyback on the test-friendliness of SysCommand
    let res = SysCommand::new("/bin/test", vec!["-x", path])
        .run_with(runner);

    match res {
        Ok(cmdres) => cmdres.ran_successfully(),
        Err(_) => false
    }

}

pub(crate) fn kill_pids(runner: &RunnerRc, pids: &[ProcessID]) -> DeckResult<ActionSuccess> {
    let mut outputs = vec![];
    let string_pids: Vec<String> = pids.iter().map(ToString::to_string).collect();
    for pid in string_pids {
        let res = SysCommand::new("kill", vec![&pid])
        .run_with(runner)?
        .as_success();

        if res.is_ok() {
            outputs.push(format!("Successfully killed pid '{pid}'"));
        } else {
            warn!("Failed to kill pid '{pid}'!");
        }
    }
    let output = outputs.join("\n");
    success!(output)
}
