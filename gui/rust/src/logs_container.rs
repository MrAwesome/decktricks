use std::path::Path;
use decktricks::utils::get_decktricks_dir;
use crate::early_log_ctx;
use crate::CRATE_DECKTRICKS_DEFAULT_LOGGER;
use decktricks::prelude::*;
use godot::classes::ColorRect;
use std::fmt::Display;

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
            error!(early_log_ctx(), "Failure when populating logs! {err:?}");
        });
    }

    fn inner_populate_logs(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        let parsed = CRATE_DECKTRICKS_DEFAULT_LOGGER.clone().get_logs();

        let log_channel_scene: Gd<PackedScene> =
            try_load::<PackedScene>("res://scenes/log_channel.tscn")?;

        self.make_or_update_log_channel(&log_channel_scene, "all".into(), parsed.all)?;
        self.make_or_update_log_channel(&log_channel_scene, "general".into(), parsed.general)?;

        let log_file_location = Path::join(&get_decktricks_dir(), "logs/decktricks-update.log");
        if log_file_location.exists() {
            let updates_text = std::fs::read_to_string(log_file_location)?;
            self.make_or_update_log_channel(&log_channel_scene, "updates".into(), updates_text)?;
        } else {
            warn!(early_log_ctx(), "Updates log file not found at {}", log_file_location.to_str().unwrap());
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
        text: String,
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
                    // Given that logs are always appends, just checking length should be enough always:
                    if textedit.get_text().len() != text.len() {
                        // NOTE: modifications to GDScript strings always return a new string, so it
                        // seems unlikely there's a way to just append updated log text here
                        textedit.set_text(&text);
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
