use std::process::{Command, Stdio};
use std::process;
use std::io;

// TODO: prevent command usage in tests
// TODO: redirect

pub fn system_command_ran_successfully(cmdname: &str, args: Vec<&str>, debug: bool) -> bool {

    let mut cmd = Command::new(cmdname);
    cmd.args(args.clone());

    if !debug {
        cmd.stdout(Stdio::null())
            .stderr(Stdio::null());
    }

    let res = cmd.status().is_ok_and(|x| x.success());

    if debug {
        println!("== EXTERNAL COMMAND STATUS: {} {:?} {}", cmdname, args, res);
    }

    res
}

// For now, return nothing.
// TODO: run in a thread and report back (for gui), run and wait (for CLI)
// TODO: return child process info, store it with provider?
// TODO: pass Result back to calling functions
//
// https://doc.rust-lang.org/std/process/struct.Child.html#method.try_wait
pub fn spawn_system_command(cmdname: &str, args: Vec<&str>) {
    Command::new(cmdname)
        .args(args)
        .spawn().unwrap();
}

pub fn system_command_output(cmdname: &str, args: Vec<&str>) -> io::Result<process::Output> {
    Command::new(cmdname).args(args).output()

}
