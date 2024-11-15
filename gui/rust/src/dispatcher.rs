use decktricks::prelude::*;
use godot::prelude::*;

#[derive(GodotClass)]
#[class(init,base=Node)]
struct DecktricksDispatcher {
    base: Base<Node>,
}

#[godot_api]
impl INode for DecktricksDispatcher {
    fn ready(&mut self) {
        let mut base = self.base_mut();
        base.connect(
            "dispatch_to_decktricks".into(),
            Callable::from_fn("dispatch_to_decktricks", move |signal_args: &[&Variant]| {
                // `signal_args` should look like:
                // [
                //   VariantArray["run", "protonup-qt"]
                // ]
                let args: Vec<String> = signal_args
                    .first()
                    .map(|x| x.to::<Vec<Variant>>())
                    .unwrap_or_else(|| {
                        godot_error!("Empty args list passed to dispatch_to_decktricks");
                        vec![]
                    })
                    .iter()
                    .map(|v| v.to::<String>())
                    .collect();
                run_with_decktricks(args)
            }),
        );
    }
}

fn run_with_decktricks(args: Vec<String>) -> Result<Variant, ()> {
    let mut experienced_error = false;
    let cmd = DecktricksCommand::parse_from(args);
    let maybe_executor = Executor::new(&cmd);

    match maybe_executor {
        Ok(executor) => {
            let results = executor.execute(&cmd.action);

            results.iter().for_each(|res| match res {
                Ok(action_success) => {
                    action_success
                        .get_message()
                        .inspect(|m| godot_print!("{}", m));
                }
                Err(known_error) => {
                    experienced_error = true;
                    godot_error!("{}", known_error);
                }
            });
        }
        Err(err) => {
            godot_error!("{}", err);
            experienced_error = true;
        }
    }

    if experienced_error {
        Err(())
    } else {
        // If needed, you can send back to Godot from here with Variants
        Ok(().to_variant())
    }
}
