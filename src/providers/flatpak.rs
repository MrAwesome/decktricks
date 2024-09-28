use super::flatpak_helpers::get_running_flatpak_applications;
use crate::prelude::*;
use crate::run_system_command::{system_command_output, system_command_ran_successfully};

impl Flatpak {
    pub fn new<S: Into<String>>(id: S) -> Self {
        Flatpak { id: id.into() }
    }
}

impl TrickProvider for Flatpak {}

#[cfg(not(test))]
impl Flatpak {
    fn is_pkg_installed(&self) -> DeckResult<bool> {
        system_command_ran_successfully("flatpak", vec!["info", &self.id])
    }

    // NOTE: Can handle/track child pid status here, but
    // `flatpak ps` gives us that easily and authoritatively.
    fn flatpak_run(&self) -> DeckResult<ActionSuccess> {
        system_command_output("flatpak", vec!["run", &self.id])
    }

    fn flatpak_install(&self) -> DeckResult<ActionSuccess> {
        system_command_output("flatpak", vec!["install", "-y", &self.id])
    }

    fn flatpak_uninstall(&self) -> DeckResult<ActionSuccess> {
        system_command_output("flatpak", vec!["uninstall", "-y", &self.id])
    }

    fn flatpak_kill(&self) -> DeckResult<ActionSuccess> {
        system_command_output("flatpak", vec!["kill", &self.id])
    }

    fn flatpak_update(&self) -> DeckResult<ActionSuccess> {
        system_command_output("flatpak", vec!["update", &self.id])
    }
}

impl Flatpak {
    fn is_pkg_running(&self) -> DeckResult<bool> {
        // TODO: error handling
        for line in get_running_flatpak_applications()? {
            if line == self.id {
                return Ok(true);
            }
        }
        Ok(false)
    }
}

impl ProviderChecks for Flatpak {
    fn is_installable(&self) -> DeckResult<bool> {
        Ok(!self.is_installed()?)
    }

    fn is_uninstallable(&self) -> DeckResult<bool> {
        self.is_installed()
    }

    fn is_installed(&self) -> DeckResult<bool> {
        self.is_pkg_installed()
    }

    fn is_runnable(&self) -> DeckResult<bool> {
        self.is_installed()
    }

    fn is_running(&self) -> DeckResult<bool> {
        self.is_pkg_running()
    }

    fn is_killable(&self) -> DeckResult<bool> {
        self.is_running()
    }

    fn is_updateable(&self) -> DeckResult<bool> {
        self.is_installed()
    }

    fn is_addable_to_steam(&self) -> DeckResult<bool> {
        self.is_installed()
    }
}

impl ProviderActions for Flatpak {
    // NOTE!!!!! update takes user input on the command line (so pass -y)
    // , and *often will require a second run* if doing a full update of all packages
    //    fn update(&self) -> DeckResult<ActionSuccess> {
    //        unimplemented!()
    //    }

    fn uninstall(&self) -> DeckResult<ActionSuccess> {
        self.flatpak_uninstall()?;
        success!("\"{}\" uninstalled successfully.", self.id)
    }

    fn install(&self) -> DeckResult<ActionSuccess> {
        self.flatpak_install()?;
        success!("\"{}\" installed successfully.", self.id)
    }

    fn run(&self) -> DeckResult<ActionSuccess> {
        // TODO: check return status and return Err if appropriate
        self.flatpak_run()
    }

    fn kill(&self) -> DeckResult<ActionSuccess> {
        self.flatpak_kill()?;
        success!()
    }

    fn update(&self) -> DeckResult<ActionSuccess> {
        self.flatpak_update()?;
        success!()
    }

    fn add_to_steam(&self, _ctx: AddToSteamContext) -> DeckResult<ActionSuccess> {
        unimplemented!()
    }
}

#[cfg(test)]
impl Flatpak {
    #[allow(clippy::unnecessary_wraps)]
    fn is_pkg_installed(&self) -> DeckResult<bool> {
        Ok(self.id == "test_pkg_installed")
    }

    #[allow(clippy::unused_self)]
    fn flatpak_run(&self) -> DeckResult<ActionSuccess> {
        success!("flatpak run success in test")
    }

    #[allow(clippy::unused_self)]
    fn flatpak_kill(&self) -> DeckResult<ActionSuccess> {
        success!("flatpak kill success in test")
    }

    #[allow(clippy::unused_self)]
    fn flatpak_update(&self) -> DeckResult<ActionSuccess> {
        success!("flatpak update success in test")
    }

    #[allow(clippy::unused_self)]
    fn flatpak_install(&self) -> DeckResult<ActionSuccess> {
        success!("flatpak install success in test")
    }

    #[allow(clippy::unused_self)]
    fn flatpak_uninstall(&self) -> DeckResult<ActionSuccess> {
        success!("flatpak uninstall success in test")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // To soothe CoC's jump-to issues for things that are cfg(not(test))
    fn _shutup() {
        let _ = (system_command_output, system_command_ran_successfully);
    }

    #[allow(clippy::unnecessary_wraps)]
    #[test]
    fn test_new_flatpak_provider() -> DeckResult<()> {
        let provider = Flatpak::new("test_pkg");
        assert_eq!(provider.id, "test_pkg");
        Ok(())
    }

    #[test]
    fn test_is_pkg_installed_true() -> DeckResult<()> {
        let provider = Flatpak::new("test_pkg_installed");
        assert!(provider.is_installed()?);
        let provider = Flatpak::new("test_pkg_not_installed");
        assert!(!provider.is_installed()?);
        Ok(())
    }

    #[test]
    fn test_installable() -> DeckResult<()> {
        let provider = Flatpak::new("RANDOM_NAME_FROM_NOWHERE");
        assert!(provider.is_installable()?);
        Ok(())
    }

    #[test]
    fn test_updateable() -> DeckResult<()> {
        let provider = Flatpak::new("test_pkg_installed");
        assert!(provider.is_updateable()?);
        let provider = Flatpak::new("test_pkg_not_installed");
        assert!(!provider.is_updateable()?);
        Ok(())
    }

    #[test]
    fn test_is_pkg_running() -> DeckResult<()> {
        let provider = Flatpak::new("running_package");
        assert!(provider.is_running()?);
        let provider = Flatpak::new("not_running_package");
        assert!(!provider.is_running()?);
        Ok(())
    }
}
