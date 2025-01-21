#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
use crate::dispatcher::DecktricksDispatcher;
use crate::logging::DecktricksGodotLogger;
use decktricks::decktricks_logging_init;
use decktricks::prelude::*;
use godot::classes::Engine;
use godot::prelude::*;
mod action_button;
pub mod dispatcher;
mod gui;
mod utils;
pub mod logging;
pub mod logs_container;

pub(crate) const UI_REFRESH_DELAY_MILLIS: u64 = 200;

decktricks_logging_init!(LogType::Log, DecktricksGodotLogger);

// For use only within this crate, and not within logging.rs:
pub(crate) static EARLY_LOGGING_CONTEXT: LazyLock<Arc<ExecutionContext>> =
    LazyLock::new(|| {
        Arc::new(ExecutionContext::internal_get_for_logging(
            CRATE_DECKTRICKS_DEFAULT_LOG_LEVEL,
            Arc::new(DecktricksGodotLogger::new()),
        ))
    });

// For use only within this crate, and not within logging.rs:
pub(crate) fn early_log_ctx() -> &'static ExecutionContext {
    &EARLY_LOGGING_CONTEXT
}

struct DecktricksGui;

#[gdextension]
unsafe impl ExtensionLibrary for DecktricksGui {
    fn on_level_init(level: InitLevel) {
        if level == InitLevel::Scene {
            // The StringName identifies your singleton and can be
            // used later to access it.
            Engine::singleton().register_singleton(
                &StringName::from("DecktricksDispatcher"),
                &DecktricksDispatcher::new_alloc().upcast::<DecktricksDispatcher>(),
            );
        }
    }

    fn on_level_deinit(level: InitLevel) {
        if level == InitLevel::Scene {
            // Get the `Engine` instance and `StringName` for your singleton.
            let mut engine = Engine::singleton();
            let singleton_name = StringName::from("DecktricksDispatcher");

            // We need to retrieve the pointer to the singleton object,
            // as it has to be freed manually - unregistering singleton
            // doesn't do it automatically.
            let singleton = engine
                .get_singleton(&singleton_name)
                .expect("cannot retrieve the singleton");

            // Unregistering singleton and freeing the object itself is needed
            // to avoid memory leaks and warnings, especially for hot reloading.
            engine.unregister_singleton(&singleton_name);
            singleton.free();
        }
    }
}
