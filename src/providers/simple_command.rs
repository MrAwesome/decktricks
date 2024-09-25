use crate::prelude::*;

impl SimpleCommand {
    pub fn new<S: Into<String>>(
            command: S,
            args: Vec<S>,
        ) -> Self {
        Self {
            command: command.into(),
            args: args.into_iter().map(|s| s.into()).collect(),
        }
    }
}

impl Provider for SimpleCommand {}

impl ProviderChecks for SimpleCommand {
    fn is_installable(&self) -> Result<bool, KnownError> {
        // These are meant to be simple system commands which are always known to be installed
        Ok(false)
    }

    fn is_installed(&self) -> Result<bool, KnownError> {
        Ok(false)
    }
    fn is_runnable(&self) -> Result<bool, KnownError> {
        Ok(true)
    }
    fn is_running(&self) -> Result<bool, KnownError> {
        // NOTE: for now, we aren't going to implement this until it's needed
        // (an easy way to implement this would be to have 'unique_grep_regex' for each command)
        Ok(false)
    }
    fn is_addable_to_steam(&self) -> Result<bool, KnownError> {
        // For now, we'll assume these aren't commands people will want to run through Steam
        Ok(false)
    }
}

impl ProviderActions for SimpleCommand {
    fn update(&self) -> Result<ActionSuccess, KnownError> {
        unimplemented!()
    }

    fn uninstall(&self) -> Result<ActionSuccess, KnownError> {
        unimplemented!()
    }

    fn force_reinstall(&self) -> Result<ActionSuccess, KnownError> {
        unimplemented!()
    }

    fn install(&self) -> Result<ActionSuccess, KnownError> {
        unimplemented!()
    }

    fn run(&self) -> Result<ActionSuccess, KnownError> {
        unimplemented!()
    }

    fn kill(&self) -> Result<ActionSuccess, KnownError> {
        unimplemented!()
    }

    fn add_to_steam(&self) -> Result<ActionSuccess, KnownError> {
        unimplemented!()
    }
}

#[test]
fn basic_expectations() {
    let sc = SimpleCommand::new("echo", vec!["lol"]);
    assert!(sc.is_installable().is_ok_and(|r| !r));
    assert!(sc.is_installed().is_ok_and(|r| !r));
    assert!(sc.is_runnable().is_ok_and(|r| r));
    assert!(sc.is_running().is_ok_and(|r| !r));
    assert!(sc.is_addable_to_steam().is_ok_and(|r| !r));
}
