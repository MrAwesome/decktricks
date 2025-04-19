use std::sync::Arc;
use crate::prelude::*;

use std::sync::mpsc;
use std::process::Child;
use std::sync::mpsc::{Receiver, TryRecvError};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub enum LiveOutputLine {
    Stdout(String),
    Stderr(String),
}

// TODO: determine where the logic that (does stuff to each line) should live
pub enum LiveRunMessage {
    Error(KnownError)
}

pub enum LiveProcHandlingCommand {
    Kill
}

pub struct LiveSysCommandWatcher {
    output_recv: mpsc::Receiver<LiveRunMessage>,
    child: Arc<Child>,
    sys_command: SysCommand,
    line_chan: Receiver<LiveOutputLine>,
    completed_chan: Receiver<SysCommandResult>,
    line_printing_func: Box<dyn Fn(LiveOutputLine) + Send + Sync + 'static>,
}

impl LiveSysCommandWatcher {
    #[must_use]
    fn get_latest_lines(&self) -> Vec<LiveOutputLine> {
        self.line_chan.try_iter().collect()
    }

    #[must_use]
    fn get_is_completed(&self) -> DeckResult<Option<SysCommandResult>> {
        match self.completed_chan.try_recv() {
            Ok(res) => Ok(Some(res)),
            Err(TryRecvError::Empty) => Ok(None),
            Err(TryRecvError::Disconnected) => Err(KnownError::SystemCommandThreadError(format!(
                "Received disconnect from live runner thread for command: {:?}",
                self.sys_command
            ))),
        }
    }
}

fn WATCHME(watcher: LiveSysCommandWatcher) -> DeckResult<SysCommandResult> {
    loop {
        let result = watcher.get_is_completed();

        let latest_lines = watcher.get_latest_lines();
        for line in latest_lines {
            todo!("print line {:?}", line);
        }

        // Handle result here, after you've handled any final lines that might have come through
        // TODO

        match result {
            Ok(Some(sys_command_result)) => return Ok(sys_command_result),
            Ok(None) => thread::sleep(Duration::from_millis(500)),
            Err(err) => return Err(err),
        }
    }
}
