use crate::prelude::*;
use std::process::Command;

// TODO: make this a trait with an implementation, and make a mock implementation for tests

pub(crate) fn system_command_ran_successfully(cmdname: &str, args: Vec<&str>) -> DeckResult<bool> {
    assert!(
        !am_in_test(),
        "Attempted to run system command {:?} in test!",
        (cmdname, args)
    );

    let mut cmd = Command::new(cmdname);
    cmd.args(args.clone());

    let res = cmd
        .output()
        .map_err(KnownError::SystemCommandRun)?;


    if is_debug() {
        let output = String::from_utf8_lossy(&res.stdout);
        debug!("== EXTERNAL COMMAND STATUS: {cmdname} {args:?} {res:?} \"{output}\"");
    }

    Ok(res.status.success())
}

// For now, return nothing.
// TODO: run in a thread and report back (for gui), run and wait (for CLI)
// TODO: return child process info, store it with provider?
// TODO: pass Result back to calling functions
//
// https://doc.rust-lang.org/std/process/struct.Child.html#method.try_wait
//pub(crate) fn spawn_system_command(cmdname: &str, args: Vec<&str>) {
//    Command::new(cmdname)
//        .args(args)
//        .spawn();
//}

pub(crate) fn system_command_output(cmdname: &str, args: Vec<&str>) -> DeckResult<ActionSuccess> {
    assert!(
        !am_in_test(),
        "Attempted to run system command {:?} in test!",
        (cmdname, args)
    );

    let result = Command::new(cmdname)
        .args(args)
        .output()
        .map_err(KnownError::SystemCommandRun)?;
    
    if result.status.success() {
        success!(String::from_utf8_lossy(&result.stdout))
    } else {
        Err(KnownError::SystemCommandFailed(result))
    }
}
