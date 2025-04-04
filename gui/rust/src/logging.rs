use std::time::Duration;
use std::collections::HashMap;
use std::fmt::Display;
use std::sync::{mpsc, Arc, LazyLock, RwLock};
use std::time::Instant;

use decktricks::logging::DecktricksLogger;
use decktricks::prelude::*;
use godot::prelude::*;

// NOTE: the logic in this file is not godot-specific, and could easily be reused in another gui

const NUM_LOG_STORAGE_READ_RETRIES: u8 = 10;
const DEFAULT_GODOT_LOG_LEVEL: LogType = LogType::Info;
type LogsWithTimestamps = HashMap<LogChannel, Vec<StoredLogEntry>>;

#[derive(Debug)]
pub struct ParsedLogs {
    pub all: String,
    pub general: String,
    pub tricks: Vec<(TrickID, String)>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
struct StoredLogEntry(Instant, String);

impl Display for StoredLogEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // At the moment, we don't display timestamps when we print out a log entry
        write!(f, "{}", self.1)
    }
}

static LOG_STORAGE: LazyLock<Arc<RwLock<LogsWithTimestamps>>> =
    LazyLock::new(|| Arc::new(RwLock::new(HashMap::new())));

#[derive(Debug)]
pub struct DecktricksGodotLogger {
    log_level: LogType,
    sender: mpsc::Sender<(LogChannel, String)>,
}

impl DecktricksGodotLogger {
    #[must_use]
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        std::thread::spawn(move || {
            for (log_channel, text) in receiver {
                let now = Instant::now();
                match LOG_STORAGE.write() {
                    Ok(mut hm) => {
                        hm.entry(log_channel)
                            .or_default()
                            .push(StoredLogEntry(now, text));
                    }
                    Err(err) => {
                        godot_error!("Write lock poisoned! Error: {err}");
                        godot_print!("Original message: {text}");
                    }
                };
            }
        });
        Self {
            sender,
            log_level: DEFAULT_GODOT_LOG_LEVEL,
        }
    }

    #[allow(clippy::unused_self)]
    pub fn get_logs(&self) -> ParsedLogs {
        let mut read_result = LOG_STORAGE.try_read();
        let mut delay_ms = 1;
        for _ in 0..NUM_LOG_STORAGE_READ_RETRIES {
            if read_result.is_err() {
                std::thread::sleep(Duration::from_millis(delay_ms));
                delay_ms *= 2;

                read_result = LOG_STORAGE.try_read();
            } else {
                break;
            }
        }
        let unprepped_logs = match LOG_STORAGE.try_read() {
            Ok(unprepped) => (*unprepped).clone(),
            Err(err) => {
                godot_error!("Failed to get read lock on logs! This is a serious error, please report it: {err}");
                HashMap::default()
            }
        };
        prep_logs_for_display(unprepped_logs)
    }
}

impl Default for DecktricksGodotLogger {
    fn default() -> Self {
        Self::new()
    }
}

impl DecktricksLogger for DecktricksGodotLogger {
    fn get_log_level(&self) -> LogType {
        self.log_level
    }

    fn set_log_level(&mut self, log_type: LogType) {
        self.log_level = log_type;
    }

    fn actual_print(&self, text: String) {
        godot_print!("{text}");
    }

    fn actual_print_debug(&self, text: String) {
        godot_print!("{text}");
    }

    fn actual_print_info(&self, text: String) {
        godot_print!("{text}");
    }

    fn actual_print_warn(&self, text: String) {
        godot_warn!("{text}");
    }

    fn actual_print_error(&self, text: String) {
        godot_error!("{text}");
    }

    fn store(&self, ctx: ExecutionContext, text: String) {
        let channel = ctx.get_log_channel().clone();
        self.sender
            .send((channel, text.to_string()))
            .unwrap_or_else(|e| {
                self.actual_print_error(format!("Error sending to log storage: {e}"));
            });
    }
}

fn prep_logs_for_display(unparsed: LogsWithTimestamps) -> ParsedLogs {
    let mut all_entries = vec![];
    let mut general_log_text = String::new();
    let mut trickid_to_log_text = Vec::new();
    for (log_channel, entries) in unparsed {
        let channel_entries_combined_as_text: String = entries
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>()
            .join("\n");
        match log_channel {
            LogChannel::General => {
                general_log_text = channel_entries_combined_as_text;
            }
            LogChannel::TrickID(trick_id) => {
                trickid_to_log_text.push((trick_id, channel_entries_combined_as_text));
            }
            LogChannel::IgnoreCompletelyAlways => {}
        };

        entries
            .into_iter()
            .for_each(|entry| all_entries.push(entry));
    }

    // Sort our assortment of trick logs to be displayed alphabetically by trick id
    trickid_to_log_text.sort();

    // Sort log entries to be displayed chronologically, regardless of where they came from
    all_entries.sort();

    let all = all_entries
        .into_iter()
        .map(|x| x.to_string())
        .collect::<Vec<_>>()
        .join("\n");

    ParsedLogs {
        all,
        general: general_log_text,
        tricks: trickid_to_log_text,
    }
}

#[test]
fn log_entry_display() {
    let teapot = "I am... a little teapot.";
    assert!(StoredLogEntry(Instant::now(), teapot.into())
        .to_string()
        .contains(teapot));
}

//type LogsWithTimestamps = HashMap<LogChannel, Vec<(Instant, String)>>;
//type ParsedLogs = HashMap<LogTypeKey, String>;
