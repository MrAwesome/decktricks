// TODO: improve performance when large amounts of log text is present!

use crate::logging::StoredLogEntry;
use crate::logging::log_type_to_godot_color;
use crate::CRATE_DECKTRICKS_LOGGER;
use decktricks::prelude::*;
use godot::classes::ColorRect;
use std::fmt::Display;
use std::time::Instant;

use godot::classes::RichTextLabel;
use godot::classes::TabContainer;
use godot::prelude::*;

#[derive(Debug)]
struct LogLoadingError(String);

impl std::error::Error for LogLoadingError {}

impl Display for LogLoadingError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(GodotClass)]
#[class(init,base=TabContainer)]
struct Logs {
    base: Base<TabContainer>,
}

#[godot_api]
impl Logs {
    // Probably easier to just have populate_logs and just create/insert the logs from here
    #[func]
    pub fn populate_logs(&mut self) {
        self.inner_populate_logs().unwrap_or_else(|err| {
            error!(crate::initial_setup::early_log_ctx(), "Failure when populating logs! {err:?}");
        });
    }

    fn inner_populate_logs(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let parsed = CRATE_DECKTRICKS_LOGGER.clone().get_latest_logs_and_wipe();

        let log_channel_scene: Gd<PackedScene> =
            try_load::<PackedScene>("res://scenes/log_channel.tscn")?;

        self.make_or_update_log_channel(&log_channel_scene, "all".into(), parsed.all)?;
        self.make_or_update_log_channel(&log_channel_scene, "general".into(), parsed.general)?;

        if let Some(updates_text) = parsed.updates {
            self.make_or_update_log_channel(&log_channel_scene, "updates".into(), vec![StoredLogEntry(Instant::now(), LogType::Info, updates_text)])?;
        }
        for (trick_id, trick_logtext) in parsed.tricks {
            self.make_or_update_log_channel(&log_channel_scene, trick_id, trick_logtext)?;
        }

        Ok(())
    }

    fn make_or_update_log_channel(
        &mut self,
        scene: &Gd<PackedScene>,
        name: String,
        entries: Vec<StoredLogEntry>,
    ) -> Result<Gd<ColorRect>, Box<dyn std::error::Error>> {
        // find_child() didn't seem to work, so do it by hand:
        let found_child: Option<Gd<Node>> = self
            .base()
            .get_children()
            .iter_shared()
            .find(|c| c.get_name() == name.clone().into());

        let color_rect = match found_child {
            Some(node_child) => match node_child.try_cast::<ColorRect>() {
                Ok(scroll_child) => scroll_child,
                Err(err) => Err(Box::new(LogLoadingError(format!(
                    "Failed to cast to ColorRect: {err}"
                ))))?,
            },
            None => match scene.try_instantiate_as::<ColorRect>() {
                Some(mut scroll) => {
                    scroll.set_name(&name);
                    self.base_mut().add_child(&scroll);
                    scroll
                }
                None => Err(Box::new(LogLoadingError(
                    "Failure instantiating log channel scene!".into(),
                )))?,
            },
        };

        match color_rect
            .get_child(0)
            .and_then(|scroll_container| scroll_container.get_child(0))
            .and_then(|margin_container| margin_container.get_child(0))
        {
            Some(child) => match child.try_cast::<RichTextLabel>() {
                Ok(mut textedit) => {
                    for StoredLogEntry(_time, log_type, text) in entries {
                        textedit.push_color(log_type_to_godot_color(log_type));
                        textedit.append_text(&text);
                        textedit.append_text("\n");
                        textedit.pop();
                    }
                }
                Err(err) => Err(Box::new(LogLoadingError(format!(
                    "Failed to cast to RichTextLabel: {err}"
                ))))?,
            },
            None => Err(Box::new(LogLoadingError(
                "Failed to get child of ScrollContainer!".into(),
            )))?,
        };
        Ok(color_rect)
    }
}
