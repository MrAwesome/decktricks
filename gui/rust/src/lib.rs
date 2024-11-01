mod gdext_plugin {
    use godot::prelude::*;

    #[derive(GodotClass)]
    #[class(base=Node)]
    pub struct GDExtPlugin;

    #[godot_api]
    impl NodeVirtual for GDExtPlugin {
        fn init(base: Base<Node>) -> Self {
            GDExtPlugin
        }

        fn ready(&mut self) {
            let actions = self.get_actions();
            godot_print!("{}", actions.get("chromium").unwrap());

            let file = File::new();
            if file.open("res://assets/config.json", File::READ).is_ok() {
                let config_data = file.get_as_text();
                file.close();

                if let Ok(json) = Json::parse(&config_data) {
                    if let Some(tricks) = json.get("tricks") {
                        if let Some(tricks) = tricks.try_iter() {
                            for trick in tricks {
                                let trick_id = trick.get("id");
                                let display_name = trick.get("display_name");
                                let icon_path = trick.get("icon");
                                let available_actions = actions.get(trick_id).unwrap();

                                let temp_row_label = Label::new();
                                temp_row_label.set_text(display_name);
                                if let Some(games) = self.get_node("Games") {
                                    games.add_child(temp_row_label);
                                    let trick_row = self.create_row(trick_id, available_actions, display_name, icon_path);
                                    games.add_child(trick_row);
                                }
                            }
                        }
                    }
                }
            }

            // TODO: improve
            if let Some(games) = self.get_node("Games") {
                if let Some(first_row) = games.get_child(1) {
                    if let Some(first_button) = first_row.get_child(0) {
                        first_button.grab_focus();
                    }
                }
            }
        }

        fn input(&mut self, event: Ref<InputEvent>) {
            if let Some(key_event) = event.cast::<InputEventKey>() {
                if key_event.is_pressed() && key_event.keycode() == Keycode::KEY_DELETE {
                    self.get_tree().unwrap().quit();
                }
            }
        }
    }

    impl GDExtPlugin {
        fn create_action_button(&self, action: &str, target: &str, contents: &str) -> Ref<Button> {
            let button = Button::new();
            button.set_name(action);
            button.set_text(contents);
            let action_copy = action.to_owned();
            let target_copy = target.to_owned();
            let button_ref = button.claim();
            button.connect(
                Button::pressed,
                Callable::from_function(move |_, _: ()| {
                    Self::take_action(&action_copy, &target_copy);
                }),
            );
            button_ref
        }

        fn take_action(action: &str, target: &str) {
            godot_print!("Running: ./decktricks {action} {target}");
            let output = Exec::run("./decktricks", &[action, target], ExecFlags::USE_STDOUT);
            // TODO: check output
            godot_print!("{}", output.unwrap().stdout());
        }

        fn create_row(&self, target: &str, available_actions: &Variant, _display_name: &str, _icon_path: &str) -> Ref<HBoxContainer> {
            let row = HBoxContainer::new();
            let mut did_first = false;

            if let Some(actions) = available_actions.try_iter() {
                for action in actions {
                    let action_name = action.to_string();
                    let button = self.create_action_button(&action_name, target, &action_name);
                    if !did_first {
                        button.set_name("first_button");
                        did_first = true;
                    }
                    row.add_child(button);
                }
            }
            row.claim()
        }

        fn get_actions(&self) -> Variant {
            let output = Exec::run("./decktricks", &["actions", "--json"], ExecFlags::USE_STDOUT);
            let response = output.unwrap();
            godot_print!("{}", response.stdout());

            // Dummy return value, replace with real parsing.
            Variant::new()
        }
    }

    fn init(handle: InitHandle) {
        handle.add_class::<GDExtPlugin>();
    }

    godot_init!(init);
}
