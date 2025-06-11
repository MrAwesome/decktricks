use crate::prelude::*;
use std::sync::Arc;

// TODO: create the LiveSysCommandWatcher
// TODO: add the necessary functions to logging logic

const LIVE_COMMAND_WATCH_INTERVAL_MILLIS: u64 = 1000;

#[derive(Debug)]
pub enum LiveOutputLine {
    Stdout(String),
    Stderr(String),
}

pub enum LiveRunMessage {
    Error(KnownError),
    Started,
    Running,
}

pub enum LiveProcHandlingCommand {
    Kill,
}

pub struct LiveSysCommandWatcher {
    sys_command: SysCommand,

    // NOTE: this reader handle is actively being checked via lines() and eventually
    //       via try_wait(), so be careful to only call kill() and not lines() elsewhere
    reader_handle: Arc<duct::ReaderHandle>,

}

impl LiveSysCommandWatcher {
    pub fn new(sys_command: SysCommand, reader_handle: Arc<duct::ReaderHandle>) -> Self {
        Self { sys_command, reader_handle }
    }

    // NOTE: this kill is not recursive per the duct docs for kill(), so the stdout reader thread
    // can be stuck if the grandchildren keep stdout open for the kill target
    fn kill(&self) -> DeckResult<()> {
        self.reader_handle.kill().map_err(|e| KnownError::LiveSystemCommandKillError(e))
    }

    fn pids(&self) -> Vec<u32> {
        self.reader_handle.pids()
    }

    fn get_is_completed(&self) -> DeckResult<Option<SysCommandResult>> {
        match self.reader_handle.try_wait() {
            // Process is still running
            Ok(None) => Ok(None),

            Ok(Some(output)) => Ok(Some(SysCommandResult::new(self.sys_command.clone(), output.clone()))),
            Err(err) => Err(KnownError::LiveSystemCommandStatusCheckError(err)),
        }
    }
}
