use crate::prelude::*;
use crate::run_system_command::{system_command_output, system_command_ran_successfully};

impl Flatpak {
    pub fn new<S: Into<String>>(id: S) -> Self {
        Flatpak { id: id.into() }
    }
}

impl TrickProvider for Flatpak {
    //fn get_provider_info(&self) -> ProviderInfo;
    //fn specific_actions(&self) -> Vec<TrickActionID>;
    //fn always_allowed_actions(&self) -> Vec<TrickActionID> {
    //vec![TrickActionID::Info]
    //}

    //fn possible(&self) -> Vec<TrickActionID> {
    //[self.specific_actions(), self.always_allowed_actions()].concat()
    //}
}

#[cfg(not(test))]
impl Flatpak {
    fn is_pkg_installed(&self) -> Result<bool, KnownError> {
        system_command_ran_successfully("flatpak", vec!["info", &self.id])
    }

    // NOTE: Can handle/track child pid status here, but
    // `flatpak ps` gives us that easily and authoritatively.
    fn flatpak_run(&self) -> Result<ActionSuccess, KnownError> {
        system_command_output("flatpak", vec!["run", &self.id])
    }

    fn flatpak_install(&self) -> Result<ActionSuccess, KnownError> {
        system_command_output("flatpak", vec!["install", "-y", &self.id])
    }

    fn flatpak_uninstall(&self) -> Result<ActionSuccess, KnownError> {
        system_command_output("flatpak", vec!["uninstall", "-y", &self.id])
    }

    fn flatpak_kill(&self) -> Result<ActionSuccess, KnownError> {
        system_command_output("flatpak", vec!["kill", &self.id])
    }

    fn flatpak_update(&self) -> Result<ActionSuccess, KnownError> {
        system_command_output("flatpak", vec!["update", &self.id])
    }

    fn flatpak_ps(&self) -> Result<ActionSuccess, KnownError> {
        // NOTE: to see what this actually sees here, pipe it to cat.
        system_command_output("flatpak", vec!["ps", "--columns=application"])
    }
}

impl Flatpak {
    fn is_pkg_running(&self) -> Result<bool, KnownError> {
        // TODO: error handling
        for line in self.get_running_flatpak_applications()? {
            if line == self.id {
                return Ok(true);
            }
        }
        Ok(false)
    }

    fn get_running_flatpak_applications(&self) -> Result<Vec<String>, KnownError> {
        // TODO: error handling
        let ps_output = self.flatpak_ps();

        match &ps_output {
            Ok(output_obj) => {
                let text = output_obj.get_message_or_blank();
                let lines = text.trim().split("\n").map(|s| s.to_string()).collect();
                Ok(lines)
            }
            Err(e) => Err(KnownError::SystemCommandParse(Box::new(
                DeckTricksError {
                    message: format!("Failed to parse 'flatpak ps' output: {:?}", e),
                },
            ))),
        }
    }
}

impl ProviderChecks for Flatpak {
    fn is_installable(&self) -> Result<bool, KnownError> {
        Ok(!self.is_installed()?)
    }

    fn is_uninstallable(&self) -> Result<bool, KnownError> {
        self.is_installed()
    }

    fn is_installed(&self) -> Result<bool, KnownError> {
        self.is_pkg_installed()
    }

    fn is_runnable(&self) -> Result<bool, KnownError> {
        self.is_installed()
    }

    fn is_running(&self) -> Result<bool, KnownError> {
        Ok(self.is_pkg_running()?)
    }

    fn is_killable(&self) -> Result<bool, KnownError> {
        self.is_running()
    }

    fn is_updateable(&self) -> Result<bool, KnownError> {
        self.is_installed()
    }

    fn is_addable_to_steam(&self) -> Result<bool, KnownError> {
        self.is_installed()
    }
}

impl ProviderActions for Flatpak {
    // NOTE!!!!! update takes user input on the command line (so pass -y)
    // , and *often will require a second run* if doing a full update of all packages
    //    fn update(&self) -> Result<ActionSuccess, KnownError> {
    //        unimplemented!()
    //    }

    fn uninstall(&self) -> Result<ActionSuccess, KnownError> {
        self.flatpak_uninstall()?;
        success!("\"{}\" uninstalled successfully.", self.id)
    }

    fn install(&self) -> Result<ActionSuccess, KnownError> {
        self.flatpak_install()?;
        success!("\"{}\" installed successfully.", self.id)
    }

    fn run(&self) -> Result<ActionSuccess, KnownError> {
        // TODO: check return status and return Err if appropriate
        self.flatpak_run()
    }

    fn kill(&self) -> Result<ActionSuccess, KnownError> {
        self.flatpak_kill()?;
        success!()
    }

    fn update(&self) -> Result<ActionSuccess, KnownError> {
        self.flatpak_update()?;
        success!()
    }

    fn add_to_steam(&self, _ctx: AddToSteamContext) -> Result<ActionSuccess, KnownError> {
        unimplemented!()
    }
}

#[cfg(test)]
impl Flatpak {
    fn is_pkg_installed(&self) -> Result<bool, KnownError> {
        Ok(self.id == "test_pkg_installed")
    }

    fn flatpak_run(&self) -> Result<ActionSuccess, KnownError> {
        success!("flatpak run success in test")
    }

    fn flatpak_kill(&self) -> Result<ActionSuccess, KnownError> {
        success!("flatpak kill success in test")
    }

    fn flatpak_update(&self) -> Result<ActionSuccess, KnownError> {
        success!("flatpak update success in test")
    }

    fn flatpak_install(&self) -> Result<ActionSuccess, KnownError> {
        success!("flatpak install success in test")
    }

    fn flatpak_uninstall(&self) -> Result<ActionSuccess, KnownError> {
        success!("flatpak uninstall success in test")
    }

    fn flatpak_ps(&self) -> Result<ActionSuccess, KnownError> {
        success!("running_package\nrunning_package2")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // To soothe CoC's jump-to issues for things that are cfg(not(test))
    fn _shutup() {
        let _ = (system_command_output, system_command_ran_successfully);
    }

    #[test]
    fn test_new_flatpak_provider() -> Result<(), KnownError> {
        let provider = Flatpak::new("test_pkg");
        assert_eq!(provider.id, "test_pkg");
        Ok(())
    }

    #[test]
    fn test_is_pkg_installed_true() -> Result<(), KnownError> {
        let provider = Flatpak::new("test_pkg_installed");
        assert!(provider.is_installed()?);
        let provider = Flatpak::new("test_pkg_not_installed");
        assert!(!provider.is_installed()?);
        Ok(())
    }

    #[test]
    fn test_installable() -> Result<(), KnownError> {
        let provider = Flatpak::new("RANDOM_NAME_FROM_NOWHERE");
        assert!(provider.is_installable()?);
        Ok(())
    }

    #[test]
    fn test_updateable() -> Result<(), KnownError> {
        let provider = Flatpak::new("test_pkg_installed");
        assert!(provider.is_updateable()?);
        let provider = Flatpak::new("test_pkg_not_installed");
        assert!(!provider.is_updateable()?);
        Ok(())
    }

    #[test]
    fn test_is_pkg_running() -> Result<(), KnownError> {
        let provider = Flatpak::new("running_package");
        assert!(provider.is_running()?);
        let provider = Flatpak::new("not_running_package");
        assert!(!provider.is_running()?);
        Ok(())
    }
}
