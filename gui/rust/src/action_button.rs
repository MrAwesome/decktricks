use crate::dispatcher::get_ctx;
use crate::dispatcher::DecktricksDispatcher;
use decktricks::prelude::*;
use decktricks::rayon::spawn;
use decktricks::tricks_status::ActionDisplayStatus;
use decktricks::tricks_status::AllTricksStatus;
use godot::classes::Tween;
use std::time::Duration;

use godot::classes::{Button, IButton};
use godot::obj::WithBaseField;
use godot::prelude::*;

// NOTE: This should not be initialized directly, use the factory functions below
//       This is only class(init) because class(no_init) breaks hot reloading right now:
//       https://github.com/godot-rust/gdext/issues/539
//
//       looks like adding "reloadable = true" to the gdextension file will fix that, from ^
//
//       TODO: now that that is added, check that this is fixed

#[derive(GodotClass)]
#[class(init,base=Button)]
#[derive(Debug)]
pub struct ActionButton {
    base: Base<Button>,
    info: ActionDisplayStatus,
    #[var]
    button_known_ongoing_state: bool,
    #[var]
    button_tween: Option<Gd<Tween>>,
    #[var]
    button_original_color: Color,
    #[var]
    override_text: GString,
}

#[godot_api]
impl IButton for ActionButton {
    fn ready(&mut self) {
        self.update_appearance();
    }

    fn pressed(&mut self) {
        let info = &self.info;

        let trick_id = info.trick.id.clone();
        let action = info.action_id.as_action(trick_id.clone());
        if matches!(action, SpecificAction::Info { .. }) {
            let info_dict = dict! {
                "title": info.trick.display_name.clone(),
                "text": info.trick.description.clone(),
            };
            DecktricksDispatcher::emit_show_info_window(info_dict);
            return;
        }

        if matches!(action, SpecificAction::AddToSteam { .. }) {
            DecktricksDispatcher::emit_added_to_steam();
        }

        spawn(move || {
            // TODO: run DecktricksCommand instead of SpecificAction just to have access to flags?
            // TODO: move back into DecktricksDispatcher?
            let mut cmd = DecktricksCommand::new(action.clone().into());
            // NOTE: this log level override is very important, as it is used to make sure
            // we see output in the GUI logs
            cmd.log_level = Some(LogType::Info);

            let executor = &DecktricksDispatcher::get_executor().clone();
            let (maybe_ctx, res) = executor.execute(&cmd);

            // Shared logic with debugging run code in src/dispatcher.rs
            let ctx = maybe_ctx.map(|c| c.as_ctx()).unwrap_or_else(|| {
                executor
                    .get_new_general_execution_context(LogType::Info)
                    .as_ctx()
            });

            // This unwrap is safe, as we know that SpecificActions will always have a single result
            match res.iter().next().unwrap() {
                Ok(action_success) => {
                    let msg = action_success.get_message().unwrap_or_else(String::default);
                    log!(
                        &ctx,
                        "Decktricks command {action:?} finished with success:\n\n{msg}"
                    );
                }
                Err(known_error) => {
                    error!(
                        &ctx,
                        "Decktricks command {action:?} encountered an error:\n\n{known_error}"
                    );
                }
            }
        });

        // Wait a short amount of time for the command to start running,
        // then trigger a UI update with new system context
        spawn(|| {
            std::thread::sleep(Duration::from_millis(crate::UI_REFRESH_DELAY_MILLIS));
            DecktricksDispatcher::async_refresh_system_context();
        });
    }
}

impl ActionButton {
    pub fn initialize_from_action_display_status(
        action_display_status: ActionDisplayStatus,
    ) -> Gd<Self> {
        let action_button = Gd::from_init_fn(|base: Base<_>| Self {
            base,
            info: action_display_status,
            button_known_ongoing_state: false,
            button_tween: None,
            button_original_color: Default::default(),
            override_text: Default::default(),
        });

        DecktricksDispatcher::emit_initialize_action_button(action_button.clone());

        action_button
    }

    fn update_appearance(&mut self) {
        let info = &self.info;
        let is_available = info.is_available;
        let is_ongoing = info.is_ongoing;
        let is_completed = info.is_completed;
        let display_text = info.action_id.get_display_name(is_ongoing, is_completed);
        let action_id = info.action_id.to_string();

        DecktricksDispatcher::emit_update_action_button(
            self.to_gd(),
            action_id,
            display_text.into(),
            is_available,
            is_ongoing,
            is_completed,
        );

        //        let mut base = self.base_mut();
        //        base.call_deferred("set_name", &[Variant::from(GString::from(action_id))]);
        //        base.call_deferred("set_text", &[Variant::from(GString::from(display_text))]);
        //        base.call_deferred("set_visible", &[Variant::from(is_available)]);
        //
        // TODO: prevent clicking when ongoing

        // base.connect(
        //     &StringName::from("focus_entered"),
        //     &Callable::from_local_fn("remember_focused_node", move |_| {
        //         // Remember node here
        //     }),
        // );
    }

    pub fn update_from(&mut self, all_tricks_status: &AllTricksStatus) {
        let info = &self.info;

        let trick_id = &info.trick.id;
        let action_id = &info.action_id;
        let Some(trick_status) = all_tricks_status.get(trick_id) else {
            error!(
                get_ctx(),
                "Trick \"{trick_id}\" not found! This is a serious error, please report it at {}!",
                GITHUB_ISSUES_LINK
            );
            return;
        };

        let res = trick_status
            .actions
            .iter()
            .find(|a| a.action_id == *action_id);

        let Some(action_display_status) = res else {
            error!(
                get_ctx(),
                "Action \"{action_id}\" status for \"{trick_id}\" not found! This is a serious error, please report it at {}!",
                GITHUB_ISSUES_LINK
            );
            return;
        };

        self.info = action_display_status.clone();
        self.update_appearance();
    }
}
