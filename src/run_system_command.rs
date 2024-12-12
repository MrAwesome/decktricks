use crate::prelude::*;
use std::sync::Arc;

// TODO: prevent Command module-wide

#[cfg(test)]
use std::os::unix::process::ExitStatusExt;

// Export for unit tests in Godot
#[cfg(any(test, feature = "test"))]
pub type Runner = MockTestActualRunner;
#[cfg(not(any(test, feature = "test")))]
pub type Runner = LiveActualRunner;

pub type RunnerRc = Arc<Runner>;

#[derive(Debug)]
pub struct SysCommandRunError {
    pub cmd: SysCommand,
    pub error: std::io::Error,
}

#[derive(Debug, Clone)]
pub struct SysCommand {
    pub ctx: ExecutionContext,
    pub cmd: String,
    pub args: Vec<String>,
    pub desired_env_vars: Vec<(String, String)>,
}

impl PartialEq for SysCommand {
    fn eq(&self, other: &Self) -> bool {
        self.cmd == other.cmd
            && self.args == other.args
            && self.desired_env_vars == other.desired_env_vars
    }
}

impl SysCommand {
    #[allow(clippy::needless_pass_by_value)]
    pub fn new<S, SS, I>(ctx: &impl ExecCtx, cmd: S, args: I) -> Self
    where
        I: IntoIterator<Item = SS>,
        S: StringType,
        SS: StringType,
    {
        Self {
            ctx: ctx.as_ctx(),
            cmd: cmd.to_string(),
            args: args.into_iter().map(|x| x.to_string()).collect(),
            desired_env_vars: Vec::default(),
        }
    }

    pub fn new_no_args<S: StringType>(ctx: &impl ExecCtx, cmd: S) -> Self {
        Self::new(ctx, cmd, Vec::<String>::new())
    }
}

pub(crate) trait SysCommandRunner {
    fn get_cmd(&self) -> &SysCommand;
    fn get_ctx(&self) -> &ExecutionContext;

    fn env(&mut self, varname: &str, value: &str) -> &Self;

    fn run(&self) -> DeckResult<SysCommandResult> {
        let sys_command = self.get_cmd();
        self.get_ctx().get_runner().run(sys_command)
    }
}

impl SysCommandRunner for SysCommand {
    fn get_ctx(&self) -> &ExecutionContext {
        &self.ctx
    }

    fn get_cmd(&self) -> &SysCommand {
        self
    }

    fn env(&mut self, varname: &str, value: &str) -> &Self {
        self.desired_env_vars.push((varname.into(), value.into()));
        self
    }
}

#[derive(Clone, PartialEq)]
pub struct SysCommandResult {
    sys_command: SysCommand,
    raw_output: std::process::Output,
}

#[cfg(not(test))]
impl SysCommandResult {
    fn new(sys_command: SysCommand, output: std::process::Output) -> Self {
        Self {
            sys_command,
            raw_output: output,
        }
    }
}

impl std::fmt::Debug for SysCommandResult {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_string())
    }
}

#[cfg(test)]
impl SysCommandResult {
    pub(crate) fn fake_success() -> Self {
        Self {
            sys_command: SysCommand::new(
                &ExecutionContext::general_for_test(),
                "nothingburger".to_string(),
                ["you should not care about this"],
            ),
            raw_output: std::process::Output {
                status: std::process::ExitStatus::from_raw(0),
                stdout: b"".to_vec(),
                stderr: b"".to_vec(),
            },
        }
    }
    pub(crate) fn fake_for_test(
        cmd: &str,
        args: Vec<&str>,
        code: i32,
        stdout: &str,
        stderr: &str,
    ) -> Self {
        Self {
            sys_command: SysCommand::new(
                &ExecutionContext::general_for_test(),
                cmd.to_string(),
                args,
            ),

            raw_output: std::process::Output {
                status: std::process::ExitStatus::from_raw(code),
                stdout: stdout.as_bytes().to_vec(),
                stderr: stderr.as_bytes().to_vec(),
            },
        }
    }

    pub(crate) fn success_output(stdout: &str) -> Self {
        Self {
            sys_command: SysCommand::new(
                &ExecutionContext::general_for_test(),
                "nothingburger".to_string(),
                ["you should not care about this"],
            ),
            raw_output: std::process::Output {
                status: std::process::ExitStatus::from_raw(0),
                stdout: stdout.as_bytes().to_vec(),
                stderr: b"".to_vec(),
            },
        }
    }
}

pub(crate) trait SysCommandResultChecker {
    fn raw_output(&self) -> &std::process::Output;
    fn sys_command(&self) -> &SysCommand;
    fn as_concrete(&self) -> SysCommandResult;

    fn ran_successfully(&self) -> bool {
        if is_debug() {
            debug!(
                self.sys_command().get_ctx(),
                "== EXTERNAL COMMAND STATUS: {}",
                self.as_string()
            );
        }

        self.raw_output().status.success()
    }

    fn as_success(&self) -> DeckResult<ActionSuccess> {
        if self.ran_successfully() {
            success!(String::from_utf8_lossy(&self.raw_output().stdout))
        } else {
            Err(KnownError::SystemCommandFailed(Box::new(
                self.as_concrete(),
            )))
        }
    }

    fn as_string(&self) -> String {
        let cmd = &self.sys_command().cmd;
        let args = &self.sys_command().args;
        let output = self.raw_output();
        let status = output.status;
        let stdout = String::from_utf8_lossy(&output.stdout);
        let stderr = String::from_utf8_lossy(&output.stderr);
        format!("{cmd} {args:?}\nSTATUS: {status:?}\nSTDOUT:\"{stdout}\"\nSTDERR:\"{stderr}\"")
    }
}

impl SysCommandResultChecker for SysCommandResult {
    fn sys_command(&self) -> &SysCommand {
        &self.sys_command
    }

    fn raw_output(&self) -> &std::process::Output {
        &self.raw_output
    }

    fn as_concrete(&self) -> SysCommandResult {
        self.clone()
    }
}

pub(crate) trait ActualRunner {
    fn run(&self, sys_command: &SysCommand) -> DeckResult<SysCommandResult>;
}

#[cfg(any(test, feature = "test"))]
use mockall::mock;
#[cfg(any(test, feature = "test"))]
mock! {
    #[derive(Debug, Clone, Copy)]
    pub TestActualRunner {}

    impl ActualRunner for TestActualRunner {
        fn run(&self, sys_command: &SysCommand) -> DeckResult<SysCommandResult>;
    }
}

#[cfg(not(test))]
#[derive(Debug, Clone, Copy, Default)]
pub struct LiveActualRunner {}

#[cfg(not(test))]
impl LiveActualRunner {
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(not(test))]
impl ActualRunner for LiveActualRunner {
    #[cfg(test)]
    fn run(&self, sys_command: &SysCommand) -> DeckResult<std::process::Output> {
        panic!("Attempted to run system command {:?} in test!", sys_command)
    }

    #[cfg(not(test))]
    fn run(&self, sys_command: &SysCommand) -> DeckResult<SysCommandResult> {
        let cmd = &sys_command.cmd;
        let args = &sys_command.args;

        let mut command = std::process::Command::new(cmd);
        command.args(args);

        for (var, val) in &sys_command.desired_env_vars {
            command.env(var, val);
        }

        let output = command.output().map_err(|e| {
            KnownError::SystemCommandRunFailure(Box::new(SysCommandRunError {
                cmd: sys_command.clone(),
                error: e,
            }))
        })?;

        info!(
            sys_command.get_ctx(),
            "Command {sys_command:#?} ran successfully with output:\n\n{}",
            String::from_utf8_lossy(&output.stdout)
        );

        Ok(SysCommandResult::new(sys_command.clone(), output))
    }
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
