#![allow(clippy::needless_pass_by_value)]

use crate::action_button::ActionButton;
use crate::early_log_ctx;
use crate::logging::get_log_level;
use crate::CRATE_DECKTRICKS_DEFAULT_LOGGER;
use decktricks::rayon::spawn;
use decktricks::run_system_command::SysCommand;
use decktricks::run_system_command::SysCommandRunner;
use decktricks::tricks_config::DEFAULT_CONFIG_CONTENTS;
use decktricks::utils::get_decktricks_update_log_file_location;
use decktricks::{inner_print, prelude::*};
use godot::classes::HBoxContainer;
use godot::classes::Label;
use godot::classes::PanelContainer;
use godot::classes::ScrollContainer;
use godot::classes::TabContainer;
use godot::classes::VBoxContainer;
use godot::prelude::*;
use std::sync::LazyLock;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use std::time::Instant;

const UI_REFRESH_DELAY_MILLIS: u64 = 200;
const NUM_EXECUTOR_READ_RETRIES: u8 = 10;

// TODO: just initialize an executor here (and panic/fail/log if it doesn't work?)
static EXECUTOR_GUARD: LazyLock<RwLock<Arc<Executor>>> = LazyLock::new(|| {
    RwLock::new(Arc::new(Executor::create_with_gather(
        ExecutorMode::Continuous,
        get_log_level(),
        CRATE_DECKTRICKS_DEFAULT_LOGGER.clone(),
        None,
    )))
});

static STARTUP: LazyLock<Instant> = LazyLock::new(Instant::now);

#[derive(GodotClass)]
#[class(init,base=Object)]
pub struct DecktricksDispatcher {
    base: Base<Object>,
}

#[godot_api]
impl DecktricksDispatcher {
    // TODO: figure out how to send parsed tricksconfig?
    #[signal]
    fn actions(actions_json_string: GString);

    #[signal]
    fn show_info_window(info: Dictionary);

    #[signal]
    fn context_was_updated();

    #[func]
    fn run_startup_logic() {
        let log_file_location = get_decktricks_update_log_file_location();
        if !log_file_location.exists() {
            warn!(
                early_log_ctx(),
                "Updates log file not found at {}",
                log_file_location.to_str().unwrap()
            );
        }
    }

    fn get_singleton() -> Gd<Object> {
        godot::classes::Engine::singleton()
            .get_singleton(&StringName::from("DecktricksDispatcher"))
            .expect("Could not get DecktricksDispatcher singleton!")
    }

    #[func]
    fn async_refresh_system_context() {
        spawn(move || {
            Self::spawn_executor_refresh_inner();
            Self::notify_godot_of_new_context();
        });
    }

    #[func]
    fn wait_for_executor() {
        let _unused = EXECUTOR_GUARD.read();
    }
    #[func]
    fn get_time_passed_ms(section: GString) -> GString {
        let time_passed_ms = STARTUP.elapsed().as_millis();
        info!(
            early_log_ctx(),
            "[{section}] Time passed: {}", time_passed_ms
        );
        time_passed_ms.to_string().into()
    }

    #[func]
    fn sync_run_with_decktricks(gargs: Array<GString>) -> GString {
        info!(
            early_log_ctx(),
            "Running command synchronously with decktricks: {gargs}"
        );
        let args = gargs_to_args(gargs);

        run_with_decktricks(Self::get_executor(), args).unwrap_or_else(|()| "".into())
    }

    //    // NOTE: do not run this from executor refresh, because it will cause a loop
    //    #[func]
    //    fn async_run_with_decktricks(gargs: Array<GString>) {
    //        info!(
    //            early_log_ctx(),
    //            "Dispatching command to decktricks: {gargs}"
    //        );
    //        let args = gargs_to_args(gargs);
    //
    //        let maybe_executor_ptr = Self::get_executor();
    //
    //        if let Some(executor_ptr) = maybe_executor_ptr {
    //            spawn(move || {
    //                run_with_decktricks(executor_ptr, args).unwrap_or("".into());
    //            });
    //            spawn(|| {
    //                std::thread::sleep(Duration::from_millis(UI_REFRESH_DELAY_MILLIS));
    //                Self::async_executor_refresh();
    //            });
    //        } else {
    //            error!(
    //                early_log_ctx(),
    //                "No executor found! This is a very serious error, please report it at: {}",
    //                GITHUB_ISSUES_LINK
    //            );
    //        }
    //    }

    fn async_run_action(action: TypedAction) {
        let log_ctx = early_log_ctx();
        info!(log_ctx, "Dispatching command to decktricks: {action:?}");

        let executor = Self::get_executor();

        // Actually run the decktricks command:
        spawn(move || {
            action.clone().do_with(
                executor.as_ref(),
                log_ctx.get_current_log_level(),
                log_ctx.get_logger(),
            );
        });

        // Wait a short amount of time for the command to start running,
        // then trigger a UI update with new system context
        spawn(|| {
            std::thread::sleep(Duration::from_millis(UI_REFRESH_DELAY_MILLIS));
            Self::async_refresh_system_context();
        });
    }

    //    // TODO: move this out of dispatcher and into a more general object?
    //    #[func]
    //    #[must_use]
    //    pub fn get_display_name_mapping() -> Dictionary {
    //        Dictionary::from_iter(SpecificActionID::get_display_name_mapping())
    //    }

    //    // This is what actually triggers a UI refresh. This is called on a timer from GDScript.
    //    fn async_update_actions(executor: Arc<Option<Executor>>) {
    //        spawn(move || {
    //            let maybe_actions =
    //                run_with_decktricks(executor, vec!["actions".into(), "--json".into()]);
    //
    //            if let Ok(actions_json_string) = maybe_actions {
    //                let mut singleton = Self::get_singleton();
    //                singleton.emit_signal(
    //                    &StringName::from("actions"),
    //                    &[Variant::from(actions_json_string)],
    //                );
    //            }
    //        });
    //    }

    #[func]
    fn get_config_text() -> GString {
        // NOTE: to use custom configs, you'll need to refactor or just
        //       use "get-config" with an executor
        DEFAULT_CONFIG_CONTENTS.into()
    }

    #[func]
    fn restart_steam() {
        // TODO: move this into Command on the rust side
        // TODO: if in Desktop mode, actually restart steam
        let _ = SysCommand::new(early_log_ctx(), "steam", ["-shutdown"]).run();
    }

    //    fn get_actions_mapping() {
    //        get_full_gui_actions_map(
    //            executor,
    //            get_log_level(),
    //            CRATE_DECKTRICKS_DEFAULT_LOGGER.clone(),
    //        )
    //    }

    #[func]
    fn log(log_level: u8, message: GString) {
        let log_type = LogType::from(log_level);
        inner_print!(log_type, early_log_ctx(), "{}", message);
    }

    #[func]
    fn populate_categories(mut categories_tabcontainer: Gd<TabContainer>) {
        let executor = Self::get_executor();

        // TODO: move to function with errors
        let map = executor.get_full_map_for_all_categories(early_log_ctx().get_logger().clone());

        let trickslist_packed: Gd<PackedScene> =
            try_load::<PackedScene>("res://scenes/tricks_list.tscn").unwrap();
        let row_outer_packed: Gd<PackedScene> =
            try_load::<PackedScene>("res://scenes/row_outer.tscn").unwrap();
        let actions_row_outer_packed: Gd<PackedScene> =
            try_load::<PackedScene>("res://scenes/actions_row.tscn").unwrap();
        let label_outer_packed: Gd<PackedScene> =
            try_load::<PackedScene>("res://scenes/label_outer.tscn").unwrap();

        let mut first_button_was_marked = false;

        for (category_id, category_trick_map) in map {
            let mut trickslist_scroller: Gd<ScrollContainer> = trickslist_packed
                .try_instantiate_as::<ScrollContainer>()
                .unwrap();
            trickslist_scroller.set_name(&category_id);
            let mut trickslist: Gd<VBoxContainer> = trickslist_scroller
                .get_child(0)
                .unwrap()
                .try_cast::<VBoxContainer>()
                .unwrap();

            for (_, trick_status) in category_trick_map {
                let mut row_outer: Gd<PanelContainer> = row_outer_packed
                    .try_instantiate_as::<PanelContainer>()
                    .unwrap();
                row_outer.set_name(&trick_status.trick.id);
                let mut row_outer_vbox: Gd<VBoxContainer> = row_outer
                    .get_child(1)
                    .unwrap()
                    .get_child(0)
                    .unwrap()
                    .try_cast::<VBoxContainer>()
                    .unwrap();

                let label_outer: Gd<PanelContainer> = label_outer_packed
                    .try_instantiate_as::<PanelContainer>()
                    .unwrap();
                let mut label: Gd<Label> = label_outer
                    .get_child(1)
                    .unwrap()
                    .try_cast::<Label>()
                    .unwrap();
                label.set_text(&trick_status.trick.display_name);

                row_outer_vbox.add_child(&label_outer);

                let actions_row_outer: Gd<PanelContainer> = actions_row_outer_packed
                    .try_instantiate_as::<PanelContainer>()
                    .unwrap();
                let mut actions_row: Gd<HBoxContainer> = actions_row_outer
                    .get_child(1)
                    .unwrap()
                    .get_child(0)
                    .unwrap()
                    .try_cast::<HBoxContainer>()
                    .unwrap();

                for action in trick_status.actions {
                    let is_available = action.is_available;
                    let mut action_button = ActionButton::from_action_display_status(action);
                    if !first_button_was_marked && is_available {
                        action_button.add_to_group("first_button");
                        first_button_was_marked = true;
                    }
                    action_button.add_to_group("action_buttons");
                    actions_row.add_child(&action_button);
                }

                row_outer_vbox.add_child(&actions_row_outer);
                trickslist.add_child(&row_outer);
            }

            categories_tabcontainer.add_child(&trickslist_scroller);
        }
    }
    #[func]
    fn update_all_buttons(mut scene_tree: Gd<SceneTree>) {
        let executor = Self::get_executor();
        let all_tricks_status =
            &executor.get_all_tricks_status(CRATE_DECKTRICKS_DEFAULT_LOGGER.clone());

        let nodes = scene_tree.get_nodes_in_group("action_buttons");
        let buttons = nodes
            .iter_shared()
            .map(|node| node.try_cast::<ActionButton>().unwrap());
        for mut button in buttons {
            button.bind_mut().update_from(all_tricks_status);
        }
    }
}

impl DecktricksDispatcher {
    pub fn emit_show_info_window(info: Dictionary) {
        let mut singleton = Self::get_singleton();
        singleton.emit_signal(
            &StringName::from("show_info_window"),
            &[Variant::from(info)],
        );
    }

    fn notify_godot_of_new_context() {
        let mut singleton = Self::get_singleton();
        singleton.emit_signal(&StringName::from("context_was_updated"), &[]);
    }

    fn spawn_executor_refresh_inner() {
        let executor = Self::get_executor();
        let logger = early_log_ctx().get_logger();

        // Do the work to gather context outside of the write lock, to
        // minimize the amount of time spent locked
        let full_ctx = executor.get_new_system_context(logger);

        match EXECUTOR_GUARD.write() {
            Ok(mut executor) => {
                let mut new_executor = (**executor).clone();
                new_executor.update_system_context(full_ctx);
                *executor = Arc::new(new_executor);
            }
            Err(err) => {
                error!(
                        early_log_ctx(),
                        "Failed to access executor while writing! This is a serious error, please report it at {}\n\nError: {:?}",
                        GITHUB_ISSUES_LINK,
                        err
                    );
            }
        };
    }

    pub fn get_executor() -> Arc<Executor> {
        let mut read_result = EXECUTOR_GUARD.try_read();
        let mut delay_ms = 1;
        for _ in 0..NUM_EXECUTOR_READ_RETRIES {
            if read_result.is_err() {
                std::thread::sleep(Duration::from_millis(delay_ms));
                delay_ms *= 2;

                read_result = EXECUTOR_GUARD.try_read();
            } else {
                break;
            }
        }
        match read_result {
            Ok(guard) => guard.clone(),
            Err(err) => {
                error!(
                    early_log_ctx(),
                    "Failed to access executor while writing! This is a serious error, please report it at {}\n\nError: {:?}",
                    GITHUB_ISSUES_LINK,
                    err
                );
                panic!("Failed to get lock on executor! Panicking now.");
            }
        }
    }
}

fn gargs_to_args(gargs: Array<GString>) -> Vec<String> {
    let vecgargs: Vec<GString> = (&gargs).into();

    vecgargs.into_iter().map(Into::into).collect()
}
fn run_with_decktricks(executor: Arc<Executor>, args: Vec<String>) -> Result<GString, ()> {
    let args_with_cmd = vec!["decktricks".into()].into_iter().chain(args.clone());
    let maybe_cmd = DecktricksCommand::try_parse_from(args_with_cmd);

    match maybe_cmd {
        Ok(mut cmd) => {
            // Explicitly show logs for commands we explicitly asked for
            cmd.log_level = Some(LogType::Info);

            run_with_decktricks_inner(&executor, cmd)
        }
        Err(cmd_parse_err) => {
            error!(
                early_log_ctx(),
                "Decktricks command {args:?} encountered a parse error: {cmd_parse_err:?}"
            );
            Err(())
        }
    }
}

fn run_with_decktricks_inner(executor: &Executor, cmd: DecktricksCommand) -> Result<GString, ()> {
    let mut experienced_error = false;
    let action = &cmd.action;
    let results = executor.execute(&cmd, CRATE_DECKTRICKS_DEFAULT_LOGGER.clone());

    let mut outputs = vec![];

    results.iter().for_each(|res| match res {
        Ok(action_success) => {
            let msg = action_success.get_message().unwrap_or_else(String::default);
            info!(
                early_log_ctx(),
                "Decktricks command {action:?} finished with success: '{msg}'"
            );
            outputs.push(msg);
        }
        Err(known_error) => {
            experienced_error = true;
            error!(
                early_log_ctx(),
                "Decktricks command {action:?} encountered an error: '{known_error}'"
            );
        }
    });

    if experienced_error {
        Err(())
    } else {
        Ok(outputs.join("\n").into())
    }
}

fn gather_new_executor() -> Executor {
    Executor::create_with_gather(
        ExecutorMode::Continuous,
        get_log_level(),
        CRATE_DECKTRICKS_DEFAULT_LOGGER.clone(),
        None,
    )
}
