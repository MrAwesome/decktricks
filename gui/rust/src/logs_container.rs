use crate::CRATE_DECKTRICKS_DEFAULT_LOGGER;
use crate::early_log_ctx;
use decktricks::prelude::*;
use godot::classes::ScrollContainer;
use std::fmt::Display;

use godot::classes::TabContainer;
use godot::classes::TextEdit;
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

        godot_print!("Adding all logs... {:?}", parsed.all.clone());

        self.make_or_update_log_channel(
            &log_channel_scene,
            "all".into(),
            parsed.all,
        )?;
        self.make_or_update_log_channel(
            &log_channel_scene,
            "general".into(),
            parsed.general,
        )?;

        for (trick_id, trick_logtext) in parsed.tricks {
            self.make_or_update_log_channel(
                &log_channel_scene,
                trick_id,
                trick_logtext,
            )?;
        }

        Ok(())
    }

    fn make_or_update_log_channel(
        &mut self,
        scene: &Gd<PackedScene>,
        name: String,
        text: String,
    ) -> Result<Gd<ScrollContainer>, Box<dyn std::error::Error>> {
        // find_child() didn't seem to work, so do it by hand:
        let found_child: Option<Gd<Node>> = self.base().get_children().iter_shared().find(|c| c.get_name() == name.clone().into());

        let scroll = match found_child {
            Some(node_child) => match node_child.try_cast::<ScrollContainer>() {
                Ok(scroll_child) => scroll_child,
                Err(err) => Err(Box::new(LogLoadingError(format!(
                                "Failed to cast to ScrollContainer: {err}"
                ))))?,
            }
            None => {
                match scene.try_instantiate_as::<ScrollContainer>() {
                    Some(mut scroll) => {
                        scroll.set_name(&name);
                        self.base_mut().add_child(&scroll);
                        scroll
                    },
                    None => Err(Box::new(LogLoadingError(
                                "Failure instantiating log channel scene!".into(),
                    )))?,
                }
            }
        };


        match scroll.get_child(0) {
            Some(child) => match child.try_cast::<TextEdit>() {
                Ok(mut textedit) => {
                    // NOTE: modifications to GDScript strings always return a new string, so it
                    // seems unlikely there's a way to just append updated log text here
                    textedit.set_text(&text);
                }
                Err(err) => Err(Box::new(LogLoadingError(format!(
                                "Failed to cast to TextEdit: {err}"
                ))))?,
            },
            None => Err(Box::new(LogLoadingError(
                        "Failed to get child of ScrollContainer!".into(),
            )))?,
        };
        Ok(scroll)
    }
}
