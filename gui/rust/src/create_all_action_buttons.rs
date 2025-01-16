use decktricks::prelude::*;
use decktricks::tricks_status::TrickStatus;
use godot::classes::Button;
use godot::prelude::*;
use std::fmt::Display;

// TODO: instead of doing this, create the action buttons yourself

#[derive(Debug)]
struct GdCastError(String);

impl std::error::Error for GdCastError {}

impl Display for GdCastError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

fn convert_trick_status_to_godot_types(
    all_tricks: Vec<(CategoryID, Vec<(TrickID, TrickStatus)>)>,
) -> VariantArray {
    // trick: id, display_name
    // actions: [action: action_id, is_available, is_ongoing]

    let g_all_tricks = VariantArray::new();
    for (category, tricks_and_status) in all_tricks {
        let g_tricks_and_status = VariantArray::new();
        for (trick_id, trick_status) in tricks_and_status {}
    }

    g_all_tricks
}

fn outer() {
    let log_channel_scene: Gd<PackedScene> =
        try_load::<PackedScene>("res://scenes/action_button.tscn").expect("FIXME");
}

fn lol(action_button_scene: Gd<PackedScene>) -> Result<(), Box<dyn std::error::Error>> {
    let button: Gd<Button> = match action_button_scene.try_instantiate_as::<Button>() {
        Some(button) => button,
        None => Err(Box::new(GdCastError(
            "Failed to cast action_button.tscn as Button!".into(),
        )))?,
    };

    // func create_action_button(action: String, trick_id: String, ongoing: bool):
    // 	var button: Button = ACTION_BUTTON.instantiate()
    // 	button.name = action
    // 	if ongoing and action == "install":
    // 		button.text = "Installing..."
    // 	elif ongoing and action == "run":
    // 		button.text = "Running..."
    // 	else:
    // 		button.text = display_name_mapping[action]
    // 	button.focus_entered.connect(focus_button.bind(button, action, trick_id))
    //
    // 	if ongoing:
    // 		var tween = create_tween()
    // 		tween.set_loops()
    // 		tween.tween_interval(0.1)
    // 		var trans = Tween.TRANS_QUAD
    // 		tween.tween_property(button, "modulate", Color.GREEN, 2) \
    // 			.set_ease(Tween.EASE_IN_OUT).set_trans(trans)
    // 		tween.tween_property(button, "modulate", Color.FOREST_GREEN, 2) \
    // 			.set_ease(Tween.EASE_IN_OUT).set_trans(trans)
    // 		tween.bind_node(button)
    // 	else:
    // 		# If the action is ongoing, don't let the user click it again
    // 		button.pressed.connect(take_action.bind(action, trick_id))
    //
    // 	return button

    Ok(())
}
