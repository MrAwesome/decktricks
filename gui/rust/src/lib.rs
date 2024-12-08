#![warn(clippy::pedantic)]
#![allow(clippy::module_name_repetitions)]
use decktricks::decktricks_logging_init;
use crate::logging::DecktricksGodotLogger;
use crate::dispatcher::DecktricksDispatcher;
use godot::classes::Engine;
use godot::prelude::*;
use decktricks::prelude::*;
mod gui;
mod action_button;
mod dispatcher;
mod logging;

decktricks_logging_init!(DecktricksGodotLogger);

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
