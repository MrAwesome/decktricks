use decktricks::actions::SpecificActionID;
use decktricks::prelude::*;
use decktricks::rayon::spawn;
use godot::prelude::*;
use std::sync::{Arc, RwLock};

#[derive(GodotClass)]
#[class(init,base=Node)]
struct DecktricksDispatcher {
    base: Base<Node>,
    executor: Arc<RwLock<Arc<Option<Executor>>>>,
}

#[godot_api]
impl INode for DecktricksDispatcher {
    fn ready(&mut self) {
        self.spawn_executor_refresh(true)
    }
}

#[godot_api]
impl DecktricksDispatcher {
    // Try sending signal or mpsc with new executor?
    #[func]
    fn on_executor_refresh_timer_timeout(&self) {
        self.spawn_executor_refresh(false)
    }

    fn spawn_executor_refresh(&self, sync_run: bool) {
        let lock = self.executor.clone();
        let task = move || {
            let new_inner_arc = Arc::new(gather_new_executor());
            match lock.write() {
                Ok(mut old_arc) => {
                    *old_arc = new_inner_arc;
                },
                Err(err) =>
                    godot_error!("Failed to access executor while writing! This is a serious error, please report it: {err:?}")
            }
        };

        if sync_run {
            task();
        } else {
            spawn(task);
        }
    }

    fn get_executor(&self) -> Option<Arc<Option<Executor>>> {
        let read_result = self.executor.try_read();
        match read_result {
            Ok(guard) => Some((*guard).clone()),
            Err(err) => {
                godot_error!("Failed to access executor while reading! This is a serious error, please report it: {err:?}");
                None
            }
        }
    }

    #[func]
    fn sync_run_with_decktricks(&self, gargs: Array<GString>) -> GString {
        godot_print!("Running command synchronously with decktricks: {gargs}");
        let args = gargs_to_args(gargs);

        self.get_executor()
            .map(|executor_ptr| run_with_decktricks(executor_ptr, args).unwrap_or("".into()))
            .unwrap_or("".into())
    }

    #[func]
    fn async_run_with_decktricks(&self, gargs: Array<GString>) {
        godot_print!("Dispatching command to decktricks: {gargs}");
        let args = gargs_to_args(gargs);

        let maybe_executor_ptr = self.get_executor();

        if let Some(executor_ptr) = maybe_executor_ptr {
            spawn(move || {
                run_with_decktricks(executor_ptr, args).unwrap_or("".into());
            });
        }
    }

    // TODO: move this out of dispatcher and into a more general object?
    #[func]
    fn get_display_name_mapping() -> Dictionary {
        Dictionary::from_iter(SpecificActionID::get_display_name_mapping())
    }

    // TODO: check these definitions in the editor
    #[signal]
    fn take_action(gargs: Array<GString>) {}
}

fn gargs_to_args(gargs: Array<GString>) -> Vec<String> {
    let vecgargs: Vec<GString> = (&gargs).into();

    vecgargs.into_iter().map(|arg| arg.into()).collect()
}
fn run_with_decktricks(
    maybe_executor: Arc<Option<Executor>>,
    args: Vec<String>,
) -> Result<GString, ()> {
    let args_with_cmd = vec!["decktricks".into()].into_iter().chain(args.clone());
    let maybe_cmd = DecktricksCommand::try_parse_from(args_with_cmd);

    match maybe_cmd {
        Ok(cmd) => match maybe_executor.as_ref().as_ref() {
            Some(executor) => run_with_decktricks_inner(executor, cmd),
            None => {
                godot_error!("No executor found! This is a serious error, please report it.");
                Err(())
            }
        },
        Err(cmd_parse_err) => {
            godot_print!(
                "Decktricks command {args:?} encountered a parse error: {cmd_parse_err:?}"
            );
            Err(())
        }
    }
}

fn run_with_decktricks_inner(executor: &Executor, cmd: DecktricksCommand) -> Result<GString, ()> {
    let mut experienced_error = false;
    let action = &cmd.action;
    let results = executor.execute(action);

    let mut outputs = vec![];

    results.iter().for_each(|res| match res {
        Ok(action_success) => {
            let msg = action_success.get_message().unwrap_or("".into());
            //godot_print!("Decktricks command {action:?} had success: {msg}");
            outputs.push(msg);
        }
        Err(known_error) => {
            experienced_error = true;
            godot_error!("Decktricks command {action:?} encountered an error: {known_error}");
        }
    });

    if experienced_error {
        Err(())
    } else {
        Ok(outputs.join("\n").into())
    }
}

fn gather_new_executor() -> Option<Executor> {
    let seed_command = DecktricksCommand::new(Action::GatherContext);
    let maybe_executor = Executor::new(ExecutorMode::Continuous, &seed_command);

    maybe_executor.map(Option::from).unwrap_or_else(|e| {
        godot_error!(
            "Executor failed to initialize! This is a very serious error, please report it: {e:?}"
        );
        None
    })
}
