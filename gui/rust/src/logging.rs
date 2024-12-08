use std::collections::HashMap;
use std::sync::{mpsc, Arc, LazyLock, RwLock};
use std::time::Instant;

use decktricks::logging::DecktricksLogger;
use decktricks::prelude::*;
use godot::prelude::*;

static LOG_STORAGE: LazyLock<Arc<RwLock<HashMap<LogChannel, Vec<(Instant, String)>>>>> =
    LazyLock::new(|| Arc::new(RwLock::new(HashMap::new())));

pub(crate) struct DecktricksGodotLogger {
    sender: mpsc::Sender<(LogChannel, String)>,
}

impl DecktricksGodotLogger {
    pub(crate) fn new() -> Self {
        let (sender, receiver) = mpsc::channel();
        std::thread::spawn(move || {
            for (log_channel, text) in receiver {
                let now = Instant::now();
                godot_error!("MESSAGE RECEIVED YO");
                match LOG_STORAGE.write() {
                    Ok(mut hm) => {
                        hm.entry(log_channel).or_default().push((now, text));
                    }
                    Err(err) => {
                        godot_error!("Write lock poisoned! Error: {err}");
                        godot_print!("Original message: {text}");
                    }
                };
                godot_print!("JJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJJ {:?}", LOG_STORAGE);
            }
        });
        Self { sender }
    }
}

impl DecktricksLogger for DecktricksGodotLogger {
    fn actual_print<S: StringType>(&self, text: S) {
        godot_print!("{text}");
    }

    fn actual_print_debug<S: StringType>(&self, text: S) {
        godot_print!("{text}");
    }

    fn actual_print_info<S: StringType>(&self, text: S) {
        godot_print!("{text}");
    }

    fn actual_print_warn<S: StringType>(&self, text: S) {
        godot_warn!("{text}");
    }

    fn actual_print_error<S: StringType>(&self, text: S) {
        godot_error!("{text}");
    }

    fn store<S: StringType>(&self, ctx: &impl ExecCtx, text: S) {
        let channel = ctx.get_log_channel().clone();
        self.sender
            .send((channel, text.to_string()))
            .unwrap_or_else(|e| {
                self.actual_print_error(format!("Error sending to log storage: {e}"));
            });
    }
}
