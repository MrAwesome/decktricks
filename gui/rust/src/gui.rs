use godot::classes::Control;
use godot::prelude::*;

type ActionAndTarget = (String, String);

#[derive(GodotClass)]
#[class(init,base=Control)]
struct Decktricks {
    base: Base<Control>,
    selected_node: ActionAndTarget,
}

// Instead of all of this, could just have a process handler for each trick that keeps track of
// running/etc status (and interfaces with decktricks rs directly instead of through execution)

#[godot_api]
impl Decktricks {
    #[func]
    fn select_active_node(&mut self) {
        godot_print!("{:?}", self.selected_node)
    }
}
