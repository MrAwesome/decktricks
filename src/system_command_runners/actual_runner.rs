use std::io::Read;
use super::system_command::*;
use crate::prelude::*;
use duct::cmd;
use std::io::BufRead;
use std::io::BufReader;
use std::sync::Arc;

// This file contains the actual logic for running commands.

pub fn get_runner() -> Arc<dyn ActualRunner> {
    if cfg!(test) {
        Arc::new(MockTestActualRunner::new())
    } else {
        Arc::new(RealWorldActualRunner::new())
    }
}

pub type RunnerRc = Arc<dyn ActualRunner>;

pub trait ActualRunner: Send + Sync + std::fmt::Debug {
    fn run(&self, sys_command: &SysCommand) -> DeckResult<SysCommandResult>;
    fn run_live(&self, sys_command: &SysCommand) -> DeckResult<LiveSysCommandWatcher>;
}

use mockall::mock;
mock! {
    #[derive(Debug, Clone, Copy)]
    pub TestActualRunner {}

    impl ActualRunner for TestActualRunner {
        fn run(&self, sys_command: &SysCommand) -> DeckResult<SysCommandResult>;
        fn run_live(&self, sys_command: &SysCommand) -> DeckResult<LiveSysCommandWatcher>;
    }
}

// Don't instantiate this directly, use ctx.sys_command() instead
#[derive(Debug, Clone, Default)]
pub struct RealWorldActualRunner {}

impl RealWorldActualRunner {
    #[must_use]
    pub fn new() -> Self {
        if cfg!(test) {
            panic!("Tried to create real world command runner in tests!");
        }
        Self::default()
    }
}

// Helper struct: implements Read for Arc<T: Read>
struct ArcReader<T>(Arc<T>);

impl<T: Read> Read for ArcReader<T> {
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        // Arc does not provide interior mutability for Read, so you usually need to wrap in Mutex if you share it
        // Here, we're just demoing reading from a ref, generally only safe if only used from one place at a time
        Arc::get_mut(&mut self.0).unwrap().read(buf)
    }
}

impl ActualRunner for RealWorldActualRunner {
    fn run_live(&self, sys_command: &SysCommand) -> DeckResult<LiveSysCommandWatcher> {
        if cfg!(test) {
            eprintln!("Culprit command: {sys_command:?}");
            panic!("Tried to run live real command in tests!");
        }
        let first_sys_command = sys_command.clone();
        let second_sys_command = sys_command.clone();
        let final_sys_command = sys_command.clone();

        let mut c = cmd(first_sys_command.cmd, first_sys_command.args);
        for (k, v) in first_sys_command.desired_env_vars {
            c = c.env(k, v);
        }

        // NOTE: ".reader()" actually spawns off the command:
        let reader_handle = c
            .stderr_to_stdout()
            .stdout_capture()
            .reader()
            .map_err(|err| sys_command_error_to_known_error(sys_command, err))?;

        let arc_reader_handle = Arc::new(reader_handle);


        let arcreader = ArcReader(arc_reader_handle.clone());

        let reader = BufReader::new(arcreader);

        // TODO: send across, create live watcher?

        std::thread::spawn(move || {

            let mut lines_iter = reader.lines();

            // NOTE: this thread can be stuck if any grandchildren keep stdout open
            //       when this is killed. if that becomes a problem, pstree out all
            //       child procs and kill them.
            loop {
                match lines_iter.next() {
                    Some(Ok(st)) => crate::CRATE_DECKTRICKS_DEFAULT_LOGGER.decktricks_print_inner(
                        LogType::Log,
                        second_sys_command.ctx.as_ctx(),
                        st,
                    ),
                    Some(Err(err)) => {
                        warn!(second_sys_command.ctx.as_ctx(), "Error while reading stdout/err: {err}");
                        return;
                    },
                    // From the duct docs:
                    // "if `ReaderHandle` has indicated EOF successfully, then it's guaranteed
                    // that this method will return `Ok(Some(_))`",
                    // so we can just return here and let try_wait handle completion for us.
                    None => return,
                }
            }
        });

        Ok(LiveSysCommandWatcher::new(final_sys_command, arc_reader_handle))

        // HELPME: figure out the best way to wait for the program to exit here
        //  1) how often will programs close stdout but keep running?
        //  2) how often will programs close stdout and then reopen it? can they even do that?

    }

    fn run(&self, sys_command: &SysCommand) -> DeckResult<SysCommandResult> {
        if cfg!(test) {
            eprintln!("Culprit command: {sys_command:?}");
            panic!("Tried to run real command in tests!");
        }

        let cmd = &sys_command.cmd;
        let args = &sys_command.args;

        // NOTE: Here is where we finally create the actual Command to be run.
        let mut command = std::process::Command::new(cmd);
        command.args(args);

        for (var, val) in &sys_command.desired_env_vars {
            command.env(var, val);
        }

        let output = command
            .output()
            .map_err(|e| sys_command_error_to_known_error(sys_command, e))?;

        if output.status.success() {
            info!(
                sys_command.get_ctx(),
                "Command {sys_command:#?} ran successfully with output:\n\n{}",
                String::from_utf8_lossy(&output.stdout)
            );
        } else {
            let exit_code = output
                .status
                .code()
                .map(|c| c.to_string())
                .unwrap_or_else(|| "UNKNOWN".into());
            info!(
                sys_command.get_ctx(),
                "Command {sys_command:#?} exited with non-zero exit code {exit_code} with:\n\n\nSTDOUT:\n\n{}\n\nSTDERR:\n\n{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
        }

        Ok(SysCommandResult::new(sys_command.clone(), output))
    }
}

fn sys_command_error_to_known_error(sys_command: &SysCommand, e: std::io::Error) -> KnownError {
    let args = sys_command.args.join(" ");
    error!(
        sys_command.get_ctx(),
        "Error running command \"{} {}\": {e:?}", sys_command.cmd, args
    );
    KnownError::SystemCommandRunFailure(Box::new(SysCommandRunError {
        cmd: sys_command.clone(),
        error: e,
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "Tried to create real world command runner in tests!")]
    fn test_real_world_creation_panics() {
        let _ = RealWorldActualRunner::new();
    }

    #[test]
    #[should_panic(expected = "Tried to run real command in tests!")]
    fn test_real_world_command_panics() {
        let r = RealWorldActualRunner {};
        let sys_command = GeneralExecutionContext::test().sys_command_no_args("echo");
        let _ = r.run(&sys_command);
    }

    #[test]
    #[should_panic(expected = "Tried to run live real command in tests!")]
    fn test_real_world_live_command_panics() {
        let r = RealWorldActualRunner {};
        let sys_command = GeneralExecutionContext::test().sys_command_no_args("echo");
        let _ = r.run_live(&sys_command);
    }
}
