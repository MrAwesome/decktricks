use godot::classes::Tween;
use crate::dispatcher::DecktricksDispatcher;
use crate::early_log_ctx;
use crate::CRATE_DECKTRICKS_DEFAULT_LOGGER;
use decktricks::prelude::*;
use decktricks::rayon::spawn;
use decktricks::tricks_status::ActionDisplayStatus;
use decktricks::tricks_status::AllTricksStatus;

use godot::classes::{Button, IButton};
use godot::obj::WithBaseField;
use godot::prelude::*;

// TODO: fix black backgrounds behind tab containers
// TODO: shorten the gate (?)
// TODO: figure out the architecture of this, where it should be called from
// TODO: have actions happen in a thread
//
// TODO: show is_ongoing status
// TODO: handle updates over time

// NOTE: This should not be initialized directly, use the factory functions below
//       This is only class(init) because class(no_init) breaks hot reloading right now:
//       https://github.com/godot-rust/gdext/issues/539
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
        // TODO: This doesn't work because of the way button text is updated from Godot, fix it:
        // if matches!(action, SpecificAction::AddToSteam { .. }) {
        //    self.base_mut().call_deferred("set_text", &[Variant::from("Added to Steam...")]);
        //    DecktricksDispatcher::emit_added_to_steam();
        //}
        spawn(move || {
            // TODO: run DecktricksCommand instead of SpecificAction just to have access to flags?
            // TODO: move back into DecktricksDispatcher?
            let _ = action
                .do_with(
                    &DecktricksDispatcher::get_executor().clone(),
                    LogType::Info,
                    CRATE_DECKTRICKS_DEFAULT_LOGGER.clone(),
                );
        });
    }
}

impl ActionButton {
    pub fn from_action_display_status(action_display_status: ActionDisplayStatus) -> Gd<Self> {
        Gd::from_init_fn(|base: Base<_>| Self {
            base,
            info: action_display_status,
            button_known_ongoing_state: false,
            button_tween: None,
            button_original_color: Default::default(),
        })
    }

    fn update_appearance(&mut self) {
        let info = &self.info;
        let display_text = info.action_id.get_display_name(info.is_ongoing);
        let action_id = info.action_id.to_string();
        let is_available = info.is_available;
        let is_ongoing = info.is_ongoing;

        DecktricksDispatcher::emit_update_action_button(
            self.to_gd(),
            action_id,
            display_text.into(),
            is_available,
            is_ongoing,
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
                early_log_ctx(),
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
                early_log_ctx(),
                "Action \"{action_id}\" status for \"{trick_id}\" not found! This is a serious error, please report it at {}!",
                GITHUB_ISSUES_LINK
            );
            return;
        };

        self.info = action_display_status.clone();
        self.update_appearance();
    }
}
