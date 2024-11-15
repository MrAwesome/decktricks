use std::sync::mpsc;

use godot::classes::{Button, IButton};
use godot::obj::WithBaseField;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init,base=Button)]
struct ActionButton {
    base: Base<Button>,
    action: String,
    trick_id: String,
    contents: String,
    sender: Option<mpsc::Sender<(String, String)>>,
}

impl ActionButton {
    fn set_sender(sender: mpsc::Sender<(String, String)>) {
        
    }
}

#[godot_api]
impl IButton for ActionButton {
    fn ready(&mut self) {
        let contents = self.contents.clone();
        let action = self.action.clone();
        let trick_id = self.trick_id.clone();
        let sender = self.sender.clone();

        let mut base = self.base_mut();
        base.set_text(contents.into());
        base.set_name(action.clone().into());

        base.connect(
            "focus_entered".into(),
            Callable::from_fn("remember_focused_node", move |_| {
                if let Some(sender) = sender.clone() {
                    sender.send((action.clone(), trick_id.clone()));
                }
                Ok(().to_variant())
            }),
        );
    }

    fn pressed(&mut self) {
        perform_button_action(self.action.as_ref(), self.trick_id.as_ref());
    }

}

#[godot_api]
impl ActionButton {
    #[func]
    fn from_actions(action: String, trick_id: String, contents: String) -> Gd<Self> {
        Gd::from_init_fn(|base: Base<_>| {
            Self {
                base,
                action,
                trick_id,
                contents,
                sender: None,
            }
        })
    }
}

fn perform_button_action(action: &str, trick_id: &str) {
    godot_print!("Taking action: {action} {trick_id}")
}

fn store_focused_node(action: String, trick_id: String) {
    godot_print!("STORING FOCUSED NODE: {action} {trick_id}")
}
