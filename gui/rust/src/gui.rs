use godot::classes::Control;
use godot::prelude::*;

type ActionAndTarget = (String, String);

#[derive(GodotClass)]
#[class(init,base=Control)]
struct Decktricks {
    base: Base<Control>,
    selected_node: ActionAndTarget,
}

#[godot_api]
impl Decktricks {
    #[func]
    fn select_active_node(&mut self) {
        godot_print!("{:?}", self.selected_node)
    }
}
