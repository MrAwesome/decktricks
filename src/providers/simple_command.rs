use crate::prelude::*;

#[derive(Debug)]
pub struct SimpleCommandProvider {
    pub command: String,
    pub args: Vec<String>,
}

impl From<SimpleCommand> for SimpleCommandProvider {
    fn from(sc: SimpleCommand) -> Self {
        Self {
            command: sc.command,
            args: sc.args,
        }
    }
}

#[cfg(test)]
impl SimpleCommandProvider {
    fn new<S: Into<String>>(command: S, args: Vec<S>) -> Self {
        Self {
            command: command.into(),
            args: args.into_iter().map(Into::into).collect(),
        }
    }
}

impl TrickProvider for SimpleCommandProvider {}

impl ProviderChecks for SimpleCommandProvider {
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

impl ProviderActions for SimpleCommandProvider {
    // TODO: generalize these to be default implementations?
    fn uninstall(&self) -> DeckResult<ActionSuccess> {
        not_possible("Simple commands cannot be uninstalled!")
    }

    fn install(&self) -> DeckResult<ActionSuccess> {
        not_possible("Simple commands cannot be installed!")
    }

    #[cfg(not(test))]
    fn run(&self) -> DeckResult<ActionSuccess> {
        crate::run_system_command::system_command_output(
            &self.command,
            self.args.iter().map(AsRef::as_ref).collect(),
        )
    }

    #[cfg(test)]
    fn run(&self) -> DeckResult<ActionSuccess> {
        Ok(ActionSuccess::success(Some("Success in test.")))
    }

    fn kill(&self) -> DeckResult<ActionSuccess> {
        not_implemented("Simple commands cannot be killed yet.")
    }

    fn update(&self) -> DeckResult<ActionSuccess> {
        not_possible("Simple commands cannot be installed!")
    }

    fn add_to_steam(&self, _ctx: AddToSteamContext) -> DeckResult<ActionSuccess> {
        not_implemented("Simple commands cannot be added to Steam yet.")
    }
}

impl GeneralProvider for SimpleCommandProvider {
    fn update_all(&self) -> DeckResult<ActionSuccess> {
        not_implemented("Simple commands cannot be updated!")
    }
}

#[cfg(test)]
mod tests {
    use super::SimpleCommandProvider;
    use crate::prelude::*;

    #[test]
    fn basic_expectations() {
        let sc = SimpleCommandProvider::new("echo", vec!["lol"]);
        assert!(!sc.is_installable());
        assert!(!sc.is_installed());
        assert!(sc.is_runnable());
        assert!(!sc.is_running());
        assert!(!sc.is_addable_to_steam());
    }

    #[test]
    fn expected_failures() {
        let sc = SimpleCommandProvider::new("echo", vec!["lol"]);
        // TODO: generalize these to be default implementations?

        assert!(matches!(sc.run(), Ok(ActionSuccess{..})));
        assert!(matches!(
            sc.uninstall(),
            Err(KnownError::ActionNotPossible(_))
        ));
        assert!(matches!(
            sc.install(),
            Err(KnownError::ActionNotPossible(_))
        ));
        assert!(matches!(sc.kill(), Err(KnownError::ActionNotImplementedYet(_))));
        assert!(matches!(sc.update(), Err(KnownError::ActionNotPossible(_))));
        assert!(matches!(
            sc.add_to_steam(AddToSteamContext::default()),
            Err(KnownError::ActionNotImplementedYet(_))
        ));
    }
}
