use std::process::{Command, Stdio};

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
// TODO: return child process info, store it with provider?
pub fn spawn_system_command(cmdname: &str, args: Vec<&str>) {
    Command::new(cmdname)
        .args(args)
        .spawn();
}
