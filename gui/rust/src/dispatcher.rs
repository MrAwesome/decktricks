#![allow(clippy::needless_pass_by_value)]

use crate::action_button::ActionButton;
use crate::early_log_ctx;
use crate::logging::get_log_level;
use crate::utils::gderr;
use crate::CRATE_DECKTRICKS_DEFAULT_LOGGER;
use decktricks::rayon::spawn;
use decktricks::run_system_command::SysCommand;
use decktricks::run_system_command::SysCommandRunner;
use decktricks::utils::get_decktricks_update_log_file_location;
use decktricks::{inner_print, prelude::*};
use godot::classes::ColorRect;
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

// TODO: reduce bloat

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
    #[signal]
    fn added_to_steam();

    #[signal]
    fn show_info_window(info: Dictionary);

    #[signal]
    fn context_was_updated();

    #[signal]
    fn initialize_action_button(action_button: Gd<ActionButton>);

    #[signal]
    fn update_action_button(
        action_button: Gd<ActionButton>,
        identifier: GString,
        display_text: GString,
        is_available: bool,
        is_ongoing: bool,
    );

    #[func]
    fn run_startup_logic() {
        // Start gathering for the executor ASAP so it's happening during godot initialization
        spawn(|| {
            let _unused = EXECUTOR_GUARD.try_read();
        });

        let log_file_location = get_decktricks_update_log_file_location();
        if !log_file_location.exists() {
            warn!(
                early_log_ctx(),
                "Updates log file not found at {}",
                log_file_location.to_string_lossy()
            );
        }
    }

    fn get_singleton() -> Gd<Object> {
        godot::classes::Engine::singleton()
            .get_singleton(&StringName::from("DecktricksDispatcher"))
            .expect("Could not get DecktricksDispatcher singleton!")
    }

    #[func]
    pub(crate) fn async_refresh_system_context() {
        spawn(move || {
            Self::spawn_executor_refresh_inner();
            Self::notify_godot_of_new_context();
        });
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

    #[func]
    fn restart_steam() {
        // TODO: move this into Command on the rust side
        // TODO: if in Desktop mode, actually restart steam
        let _ = SysCommand::new(early_log_ctx(), "steam", ["-shutdown"]).run();
    }

    #[func]
    fn log(log_level: u8, message: GString) {
        let log_type = LogType::from(log_level);
        inner_print!(log_type, early_log_ctx(), "{}", message);
    }

    #[func]
    fn populate_categories(categories_tabcontainer: Gd<TabContainer>) {
        if let Err(err) = Self::populate_categories_inner(categories_tabcontainer) {
            error!(
                early_log_ctx(),
                "Error encountered while populating categories! Error: {err:?}"
            );
        }
    }

    fn populate_categories_inner(
        mut categories_tabcontainer: Gd<TabContainer>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let executor = Self::get_executor();

        let map = executor.get_full_map_for_all_categories(early_log_ctx().get_logger().clone());

        let trickslist_packed: Gd<PackedScene> =
            try_load::<PackedScene>("res://scenes/tricks_list.tscn")?;
        let row_outer_packed: Gd<PackedScene> =
            try_load::<PackedScene>("res://scenes/row_outer.tscn")?;
        let actions_row_outer_packed: Gd<PackedScene> =
            try_load::<PackedScene>("res://scenes/actions_row.tscn")?;
        let label_outer_packed: Gd<PackedScene> =
            try_load::<PackedScene>("res://scenes/label_outer.tscn")?;

        let mut first_button_was_marked = false;

        // We focus on the second tab
        let mut i = 0;
        let desired_first_tab_index = 1;

        for (category_id, category_trick_map_unsorted) in map {
            let mut trickslist_background: Gd<ColorRect> = trickslist_packed
                .try_instantiate_as::<ColorRect>()
                .ok_or("background not found")?;
            trickslist_background.set_name(&category_id);
            let trickslist_scroller: Gd<ScrollContainer> = trickslist_background
                .get_child(0)
                .ok_or("scroller not found")?
                .try_cast::<ScrollContainer>()
                .map_err(gderr)?;
            let mut trickslist: Gd<VBoxContainer> = trickslist_scroller
                .get_child(0)
                .ok_or("vboxcontainer not found")?
                .try_cast::<VBoxContainer>()
                .map_err(gderr)?;

            // NOTE: it's inefficient to do this sort here, but for clarity of mind
            //       easier to keep tricks sorted by ID until we want them sorted by
            //       display name for actually showing to the user
            let mut category_trick_map = category_trick_map_unsorted.clone();
            category_trick_map.sort_by_key(|t| t.1.trick.display_name.clone());

            for (_, trick_status) in category_trick_map {
                let mut row_outer: Gd<PanelContainer> = row_outer_packed
                    .try_instantiate_as::<PanelContainer>()
                    .ok_or("panelcontainer not found")?;
                row_outer.set_name(&trick_status.trick.id);
                let mut row_outer_vbox: Gd<VBoxContainer> = row_outer
                    .get_child(1)
                    .ok_or("vboxcontainer second child not found")?
                    .get_child(0)
                    .ok_or("inner vboxcontainer not found")?
                    .try_cast::<VBoxContainer>()
                    .map_err(gderr)?;

                let label_outer: Gd<PanelContainer> = label_outer_packed
                    .try_instantiate_as::<PanelContainer>()
                    .ok_or("label_outer not found")?;
                let mut label: Gd<Label> = label_outer
                    .get_child(1)
                    .ok_or("label not found")?
                    .try_cast::<Label>()
                    .map_err(gderr)?;
                label.set_text(&trick_status.trick.display_name);

                row_outer_vbox.add_child(&label_outer);

                let actions_row_outer: Gd<PanelContainer> = actions_row_outer_packed
                    .try_instantiate_as::<PanelContainer>()
                    .ok_or("actions_row_outer not found")?;
                let mut actions_row: Gd<HBoxContainer> = actions_row_outer
                    .get_child(1)
                    .ok_or("actions_row_outer second child not found")?
                    .get_child(0)
                    .ok_or("actions_row hbox not found")?
                    .try_cast::<HBoxContainer>()
                    .map_err(gderr)?;

                for action in trick_status.actions {
                    let is_available = action.is_available;
                    let mut action_button =
                        ActionButton::initialize_from_action_display_status(action);
                    if !first_button_was_marked && is_available {
                        if i == desired_first_tab_index {
                            action_button.add_to_group("first_button");
                            first_button_was_marked = true;
                        }
                    }
                    action_button.add_to_group("action_buttons");
                    actions_row.add_child(&action_button);
                }

                row_outer_vbox.add_child(&actions_row_outer);
                trickslist.add_child(&row_outer);
            }

            categories_tabcontainer.add_child(&trickslist_background);
            i += 1;
        }

        Ok(())
    }


    // TODO: clean up all unwraps
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
    pub fn emit_added_to_steam() {
        let mut singleton = Self::get_singleton();
        singleton.emit_signal(&StringName::from("added_to_steam"), &[]);
    }

    pub fn emit_show_info_window(info: Dictionary) {
        let mut singleton = Self::get_singleton();
        singleton.emit_signal(
            &StringName::from("show_info_window"),
            &[Variant::from(info)],
        );
    }

    pub fn emit_initialize_action_button(action_button: Gd<ActionButton>) {
        let mut singleton = Self::get_singleton();
        singleton.emit_signal(
            &StringName::from("initialize_action_button"),
            &[Variant::from(action_button)],
        );
    }

    pub fn emit_update_action_button(
        action_button: Gd<ActionButton>,
        identifier: String,
        display_text: String,
        is_available: bool,
        is_ongoing: bool,
        is_completed: bool,
    ) {
        let mut singleton = Self::get_singleton();
        singleton.emit_signal(
            &StringName::from("update_action_button"),
            &[
                Variant::from(action_button),
                Variant::from(GString::from(identifier)),
                Variant::from(GString::from(display_text)),
                Variant::from(is_available),
                Variant::from(is_ongoing),
                Variant::from(is_completed),
            ],
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
