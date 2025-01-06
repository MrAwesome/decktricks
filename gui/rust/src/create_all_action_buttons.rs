use decktricks::tricks_status::{ActionDisplayStatus, TrickStatus};
use decktricks::prelude::*;
use godot::builtin::VariantArray;

// TODO: instead of doing this, create the action buttons yourself

fn convert_trick_status_to_godot_types(
    all_tricks: Vec<(CategoryID, Vec<(TrickID, TrickStatus)>)> ) -> VariantArray {

    // trick: id, display_name
    // actions: [action: action_id, is_available, is_ongoing]

    let g_all_tricks = VariantArray::new();
    for (category, tricks_and_status) in all_tricks {
        let g_tricks_and_status = VariantArray::new();
        for (trick_id, trick_status) in tricks_and_status {
            

        }
    }

    g_all_tricks
}
