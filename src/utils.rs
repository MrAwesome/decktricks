use crate::prelude::*;
use std::fs::File;
use std::io::copy;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

pub(crate) fn run_remote_script(url: &str, local_filename: &str) -> Result<(), DynamicError> {
    // TODO: use SysCommand
    // TODO: prevent Command module-wide
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
    std::env::var("HOME")
        .unwrap_or_else(|_| "/home/deck".to_string())
}

pub(crate) fn exists_and_executable(path: &str) -> bool {
    // TODO: use SysCommand
    // /bin/test -x
    std::fs::metadata(path)
        .map(|meta| meta.permissions().mode() & 0o111 != 0)
        .unwrap_or(false)
}
