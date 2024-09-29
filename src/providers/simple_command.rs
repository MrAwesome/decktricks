use crate::prelude::*;

impl SimpleCommand {
    pub fn new<S: Into<String>>(command: S, args: Vec<S>) -> Self {
        Self {
            command: command.into(),
            args: args.into_iter().map(Into::into).collect(),
        }
    }
}

impl TrickProvider for SimpleCommand {}

impl ProviderChecks for SimpleCommand {
    fn is_installable(&self) -> bool {
        // These are meant to be simple system commands which are always known to be installed
        false
    }

    fn is_uninstallable(&self) -> bool {
        false
    }

    fn is_installed(&self) -> bool {
        false
    }
    fn is_runnable(&self) -> bool {
        true
    }
    fn is_running(&self) -> bool {
        // NOTE: for now, we aren't going to implement this until it's needed
        // (an easy way to implement this would be to have 'unique_grep_regex' for each command)
        false
    }
    fn is_killable(&self) -> bool {
        self.is_running()
    }

    fn is_updateable(&self) -> bool {
        false
    }

    fn is_addable_to_steam(&self) -> bool {
        // For now, we'll assume these aren't commands people will want to run through Steam
        false
    }
}

impl ProviderActions for SimpleCommand {
    fn uninstall(&self) -> DeckResult<ActionSuccess> {
        unimplemented!()
    }

    fn install(&self) -> DeckResult<ActionSuccess> {
        unimplemented!()
    }

    fn run(&self) -> DeckResult<ActionSuccess> {
        unimplemented!()
    }

    fn kill(&self) -> DeckResult<ActionSuccess> {
        unimplemented!()
    }

    fn update(&self) -> DeckResult<ActionSuccess> {
        unimplemented!()
    }

    fn add_to_steam(&self, _ctx: AddToSteamContext) -> DeckResult<ActionSuccess> {
        unimplemented!()
    }
}

#[test]
fn basic_expectations() {
    let sc = SimpleCommand::new("echo", vec!["lol"]);
    assert!(!sc.is_installable());
    assert!(!sc.is_installed());
    assert!(sc.is_runnable());
    assert!(!sc.is_running());
    assert!(!sc.is_addable_to_steam());
}
