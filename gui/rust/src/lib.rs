use godot::prelude::*;
mod gui;
mod action_button;
mod dispatcher;

struct DecktricksGui;

#[gdextension]
unsafe impl ExtensionLibrary for DecktricksGui {}
