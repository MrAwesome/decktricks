#![allow(clippy::needless_pass_by_value)]

use crate::logging::get_log_level;
use crate::CRATE_DECKTRICKS_DEFAULT_LOGGER;
use std::time::Duration;
use crate::early_log_ctx;
use decktricks::actions::SpecificActionID;
use decktricks::{inner_print, prelude::*};
use decktricks::rayon::spawn;
use decktricks::tricks_config::DEFAULT_CONFIG_CONTENTS;
use godot::prelude::*;
use std::sync::LazyLock;
use std::sync::{Arc, RwLock};
use std::time::Instant;


// TODO: trick logs not getting added
// TODO: it seems like there can be a condition where the active executor sees a different version
// of the world from the UI? Given that the update timers are so different (5 and 3)
const TODO: i32 = 0;

const UI_REFRESH_DELAY_MILLIS: u64 = 500;

// TODO: just initialize an executor here (and panic/fail/log if it doesn't work?)
static EXECUTOR_GUARD: LazyLock<Arc<RwLock<Arc<Option<Executor>>>>> =
    LazyLock::new(|| Arc::new(RwLock::new(Arc::new(None))));

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

    fn get_singleton() -> Gd<Object> {
        godot::classes::Engine::singleton()
            .get_singleton(&StringName::from("DecktricksDispatcher"))
            .expect("Could not get DecktricksDispatcher singleton!")
    }

    #[func]
    fn sync_executor_refresh() {
        Self::spawn_executor_refresh_inner(true, false);
    }

    #[func]
    fn async_executor_refresh() {
        Self::spawn_executor_refresh_inner(false, false);
    }

    #[func]
    fn initialize_executor_with_lock() {
        Self::spawn_executor_refresh_inner(false, true);
    }

    #[func]
    fn wait_for_executor() {
        let _unused = EXECUTOR_GUARD.read();
    }

    // sync_run: whether to create a new executor in the current thread or spawned elsewhere
    // hold_write_lock: whether to hold the write lock *while creating the executor*
    fn spawn_executor_refresh_inner(sync_run: bool, hold_write_lock: bool) {
        let lock = EXECUTOR_GUARD.clone();
        let task = move || {
            let new_inner_arc: Arc<Option<Executor>> = if hold_write_lock {
                Arc::new(None)
            } else {
                Arc::new(gather_new_executor())
            };

            match lock.write() {
                Ok(mut old_arc) => {
                    let new_inner_arc = match *new_inner_arc {
                        Some(_) => new_inner_arc,
                        None => Arc::new(gather_new_executor())
                    };
                    *old_arc = new_inner_arc;
                },
                Err(err) => error!(early_log_ctx(), "Failed to access executor while writing! This is a serious error, please report it: {err:?}")
                
            }
        };

        if sync_run {
            task();
        } else {
            spawn(task);
        }
    }

    fn get_executor() -> Option<Arc<Option<Executor>>> {
        let read_result = EXECUTOR_GUARD.try_read();
        match read_result {
            Ok(guard) => Some((*guard).clone()),
            Err(err) => {
                error!(early_log_ctx(), "Failed to access executor while reading! This is a serious error, please report it: {err:?}");
                None
            }
        }
    }

    #[func]
    fn get_time_passed_ms(section: GString) -> GString {
        let time_passed_ms = STARTUP.elapsed().as_millis();
        info!(early_log_ctx(), "[{section}] Time passed: {}", time_passed_ms);
        time_passed_ms.to_string().into()
    }

    #[func]
    fn sync_run_with_decktricks(gargs: Array<GString>) -> GString {
        info!(early_log_ctx(), "Running command synchronously with decktricks: {gargs}");
        let args = gargs_to_args(gargs);

        Self::get_executor().map_or_else(
            || "".into(),
            |executor_ptr| run_with_decktricks(executor_ptr, args).unwrap_or_else(|()| "".into()),
        )
    }

    #[func]
    fn async_run_with_decktricks(gargs: Array<GString>) {
        info!(early_log_ctx(), "Dispatching command to decktricks: {gargs}");
        let args = gargs_to_args(gargs);

        let maybe_executor_ptr = Self::get_executor();

        if let Some(executor_ptr) = maybe_executor_ptr {
            spawn(move || {
                // TODO: just log in real time?
                let output = run_with_decktricks(executor_ptr, args).unwrap_or("".into());
            });
            spawn(|| {
                let x = "TODO";
                std::thread::sleep(Duration::from_millis(UI_REFRESH_DELAY_MILLIS));
                Self::async_update_actions();
            });
        }
    }

    // TODO: move this out of dispatcher and into a more general object?
    #[func]
    #[must_use]
    pub fn get_display_name_mapping() -> Dictionary {
        Dictionary::from_iter(SpecificActionID::get_display_name_mapping())
    }

    // This is what actually triggers a UI refresh. This is called on a timer from GDScript.
    #[func]
    fn async_update_actions() {
        let maybe_executor_ptr = Self::get_executor();

        if let Some(executor_ptr) = maybe_executor_ptr {
            spawn(move || {
                let maybe_actions =
                    run_with_decktricks(executor_ptr, vec!["actions".into(), "--json".into()]);

                if let Ok(actions_json_string) = maybe_actions {
                    let mut singleton = Self::get_singleton();
                    singleton.emit_signal(
                        &StringName::from("actions"),
                        &[Variant::from(actions_json_string)],
                    );
                }
            });
        }
    }

    #[func]
    fn get_config_text() -> GString {
        // NOTE: to use custom configs, you'll need to refactor or just
        //       use "get-config" with an executor
        DEFAULT_CONFIG_CONTENTS.into()
    }

    #[func]
    fn log(log_level: u8, message: GString) {
        let log_type = LogType::from(log_level);
        inner_print!(
            log_type,
            early_log_ctx(),
            "{}",
            message
        );
    }
}

fn gargs_to_args(gargs: Array<GString>) -> Vec<String> {
    let vecgargs: Vec<GString> = (&gargs).into();

    vecgargs.into_iter().map(Into::into).collect()
}
fn run_with_decktricks(
    maybe_executor: Arc<Option<Executor>>,
    args: Vec<String>,
) -> Result<GString, ()> {
    let args_with_cmd = vec!["decktricks".into()].into_iter().chain(args.clone());
    let maybe_cmd = DecktricksCommand::try_parse_from(args_with_cmd);
    let todo = "set current log level of command here, and pass through overriding the executor's level";

    match maybe_cmd {
        Ok(mut cmd) => {
            if let Some(executor) = maybe_executor.as_ref().as_ref() {

                // Explicitly show logs for commands we explicitly asked for
                cmd.log_level = Some(LogType::Info);

                run_with_decktricks_inner(executor, cmd)
            } else {
                error!(early_log_ctx(), "No executor found! This is a serious error, please report it.");
                Err(())
            }
        }
        Err(cmd_parse_err) => {
            error!(early_log_ctx(), 
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
            info!(early_log_ctx(), "Decktricks command {action:?} finished with success: '{msg}'");
            outputs.push(msg);
        }
        Err(known_error) => {
            experienced_error = true;
            error!(early_log_ctx(), "Decktricks command {action:?} encountered an error: '{known_error}'");
        }
    });

    if experienced_error {
        Err(())
    } else {
        Ok(outputs.join("\n").into())
    }
}

fn gather_new_executor() -> Option<Executor> {
    let seed_command = DecktricksCommand::new(Action::GatherContext);

    let x = "use this elsewhere? make particular commands override" ;

    let maybe_executor = Executor::create_with_gather(
        ExecutorMode::Continuous,
        &seed_command,
        get_log_level(),
        CRATE_DECKTRICKS_DEFAULT_LOGGER.clone(),
    );

    maybe_executor.map_or_else(
        |e| {
            error!(early_log_ctx(), 
                "Executor failed to initialize! This is a very serious error, please report it: {e:?}"
            );
            None
        },
        Option::from,
    )
}
