use std::collections::HashMap;
use std::fmt::Display;
use std::sync::{mpsc, Arc, LazyLock, RwLock};
use std::time::Duration;
use std::time::Instant;

use decktricks::logging::DecktricksLogger;
use decktricks::prelude::*;
use godot::prelude::*;

// NOTE: the logic in this file is not godot-specific, and could easily be reused in another gui

const NUM_LOG_STORAGE_READ_RETRIES: u8 = 10;
const DEFAULT_GODOT_LOG_LEVEL: LogType = LogType::Info;

type LogsWithTimestamps = HashMap<LogChannel, Vec<StoredLogEntry>>;
#[derive(Default)]
pub struct LogStorage {
    pending_logs: LogsWithTimestamps,
    all_stored_logs: LogsWithTimestamps,
}

#[derive(Debug)]
pub struct ParsedLogsLatest {
    pub all: Vec<StoredLogEntry>,
    pub general: Vec<StoredLogEntry>,
    pub tricks: Vec<(TrickID, Vec<StoredLogEntry>)>,
}

#[derive(Debug)]
pub struct ParsedLogsForDumps {
    pub all: String,
    pub general: String,
    pub tricks: Vec<(TrickID, String)>,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct StoredLogEntry(pub Instant, pub LogType, pub String);

impl Display for StoredLogEntry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // At the moment, we don't display timestamps when we print out a log entry
        write!(f, "{}", self.2)
    }
}

static LOG_STORAGE: LazyLock<Arc<RwLock<LogStorage>>> =
    LazyLock::new(|| Arc::new(RwLock::new(LogStorage::default())));

// TODO: see if LOG_STORAGE can just live on this directly?
#[derive(Debug)]
pub struct DecktricksGodotLogger {
    log_level: LogType,
    sender: mpsc::Sender<(LogChannel, LogType, String)>,
}

impl DecktricksGodotLogger {
    #[must_use]
    pub fn new() -> Self {
        let (sender, receiver) = mpsc::channel::<(LogChannel, LogType, String)>();
        std::thread::spawn(move || {
            for (log_channel_id, log_type, text) in receiver {
                let now = Instant::now();
                match LOG_STORAGE.write() {
                    Ok(mut hm) => {
                        hm.pending_logs
                            .entry(log_channel_id.clone())
                            .or_default()
                            .push(StoredLogEntry(now, log_type, text.clone()));
                        hm.all_stored_logs
                            .entry(log_channel_id)
                            .or_default()
                            .push(StoredLogEntry(now, log_type, text));
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
    pub fn get_latest_logs_and_wipe(&self) -> ParsedLogsLatest {
        let logs_with_timestamps: LogsWithTimestamps = match LOG_STORAGE.write() {
            Ok(mut hm) => std::mem::take(&mut hm.pending_logs),
            Err(err) => {
                let error_msg = format!("Error: {err}\n\nWrite lock poisoned while trying to get latest logs! This is a serious error, please report it.");
                godot_error!("{error_msg}");
                let logs = HashMap::from([(
                    LogChannel::General,
                    vec![StoredLogEntry(
                        Instant::now(),
                        LogType::Error,
                        error_msg.to_string(),
                    )],
                )]);
                logs
            }
        };
        prep_logs_for_display(logs_with_timestamps)
    }

    #[allow(clippy::unused_self)]
    pub fn get_all_logs_for_dump(&self) -> ParsedLogsForDumps {
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
        let unprepped_logs = match read_result {
            Ok(unprepped) => (*unprepped).all_stored_logs.clone(),
            Err(err) => {
                godot_error!("Failed to get read lock on logs! This is a serious error, please report it: {err}");
                HashMap::default()
            }
        };
        prep_logs_for_dump(unprepped_logs)
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

    fn store(&self, ctx: ExecutionContext, log_type: LogType, text: String) {
        let channel = ctx.get_log_channel().clone();

        self.sender
            .send((channel, log_type, text))
            .unwrap_or_else(|e| {
                self.actual_print_error(format!("Error sending to log storage: {e}"));
            });
    }
}

fn prep_logs_for_dump(unparsed: LogsWithTimestamps) -> ParsedLogsForDumps {
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

    ParsedLogsForDumps {
        all,
        general: general_log_text,
        tricks: trickid_to_log_text,
    }
}

fn prep_logs_for_display(unparsed: LogsWithTimestamps) -> ParsedLogsLatest {
    let mut all_entries = vec![];
    let mut general_entries = vec![];
    let mut trickid_to_log_entries = vec![];

    for (log_channel, entries) in unparsed {
        all_entries.extend(entries.clone());
        match log_channel {
            LogChannel::General => {
                general_entries = entries;
            }
            LogChannel::TrickID(trick_id) => {
                trickid_to_log_entries.push((trick_id, entries));
            }
            LogChannel::IgnoreCompletelyAlways => {}
        }
    }

    trickid_to_log_entries.sort();
    all_entries.sort();

    ParsedLogsLatest {
        all: all_entries,
        general: general_entries,
        tricks: trickid_to_log_entries,
    }
}

pub fn log_type_to_godot_color(log_type: LogType) -> Color {
    match log_type {
        LogType::Debug => Color::DARK_GRAY,
        LogType::Info => Color::GRAY,
        LogType::Log => Color::GREEN,
        LogType::Warn => Color::ORANGE,
        LogType::Error => Color::RED,
    }
}

#[test]
fn log_entry_display() {
    let teapot = "I am... a little teapot.";
    assert!(StoredLogEntry(Instant::now(), LogType::Log, teapot.into())
        .to_string()
        .contains(teapot));
}

//type LogsWithTimestamps = HashMap<LogChannel, Vec<(Instant, String)>>;
//type ParsedLogs = HashMap<LogTypeKey, String>;
