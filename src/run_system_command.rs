use crate::prelude::*;
use std::process::{Command, Stdio};

pub fn system_command_ran_successfully(cmdname: &str, args: Vec<&str>) -> DeckResult<bool> {
    if TEST_SAFETY {
        panic!(
            "Attempted to run system command {:?} in test!",
            (cmdname, args)
        );
    }

    let mut cmd = Command::new(cmdname);
    cmd.args(args.clone());

    if !DEBUG {
        cmd.stdout(Stdio::null()).stderr(Stdio::null());
    }

    let res = cmd.status();

    if DEBUG {
        println!(
            "== EXTERNAL COMMAND STATUS: {} {:?} {:?}",
            cmdname, args, res
        );
    }

    res.map(|x| x.success())
        .map_err(|e| KnownError::SystemCommandRun(e))
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

pub fn system_command_output(cmdname: &str, args: Vec<&str>) -> DeckResult<ActionSuccess> {
    if TEST_SAFETY {
        panic!(
            "Attempted to run system command {:?} in test!",
            (cmdname, args)
        );
    }

    let output = Command::new(cmdname)
        .args(args)
        .output()
        .map_err(|e| KnownError::SystemCommandRun(e))?
        .stdout;
    success!(String::from_utf8_lossy(&output))
}
