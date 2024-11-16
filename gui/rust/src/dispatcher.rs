use decktricks::prelude::*;
use godot::prelude::*;
use decktricks::rayon::spawn;

#[derive(GodotClass)]
#[class(init,base=Node)]
struct DecktricksDispatcher {
    base: Base<Node>,
}

// TODO: just rewrite the action buttons in rust so that they can call rust funcs directly
#[godot_api]
impl DecktricksDispatcher {
    #[func]
    fn sync_run_with_decktricks(gargs: Array<GString>) -> GString {
        godot_print!("Running command synchronously with decktricks: {gargs}");
        let args = gargs_to_args(gargs);
        run_with_decktricks(args).unwrap_or("".into())
    }

    #[func]
    fn async_run_with_decktricks(gargs: Array<GString>) {
        godot_print!("Dispatching command to decktricks: {gargs}");
        // TODO: use async-rust or tokio or whatever, if we're using it elsewhere
        let args = gargs_to_args(gargs);
        spawn(move || {
            run_with_decktricks(args).unwrap_or("".into());
        });
    }
}

fn gargs_to_args(gargs: Array<GString>) -> Vec<String> {
    let vecgargs: Vec<GString> = (&gargs)
        .into();

    vecgargs
        .into_iter()
        .map(|arg| arg.into())
        .collect()
}

fn run_with_decktricks(args: Vec<String>) -> Result<GString, ()> {
    let args_with_cmd = vec!["decktricks".into()].into_iter().chain(args.clone());
    let maybe_cmd = DecktricksCommand::try_parse_from(args_with_cmd);

    match maybe_cmd {
        Ok(cmd) => {
            let maybe_executor = Executor::new(&cmd);

            match maybe_executor {
                Ok(executor) => {
                    let mut experienced_error = false;
                    let results = executor.execute(&cmd.action);

                    let mut outputs = vec![];

                    results.iter().for_each(|res| match res {
                        Ok(action_success) => {
                            let msg = action_success.get_message().unwrap_or("".into());
                            godot_print!("Decktricks command {args:?} had success: {msg}");
                            outputs.push(msg);
                        }
                        Err(known_error) => {
                            experienced_error = true;
                            godot_error!(
                                "Decktricks command {args:?} encountered an error: {known_error}"
                            );
                        }
                    });

                    if experienced_error {
                        Err(())
                    } else {
                        Ok(outputs.join("\n").into())
                    }
                }
                Err(executor_known_err) => {
                    godot_print!("Decktricks command {args:?} encountered an executor error: {executor_known_err}");
                    Err(())
                }
            }
        }
        Err(cmd_parse_err) => {
            godot_print!(
                "Decktricks command {args:?} encountered a parse error: {cmd_parse_err:?}"
            );
            Err(())
        }
    }
}
