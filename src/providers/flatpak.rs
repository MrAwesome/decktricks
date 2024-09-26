use crate::prelude::*;
#[cfg(not(test))]
use crate::run_system_command::{system_command_output, system_command_ran_successfully};
use std::process;

impl Flatpak {
    pub fn new<S: Into<String>>(id: S) -> Self {
        Flatpak { id: id.into() }
    }
}

impl Provider for Flatpak {
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
    fn flatpak_run(&self) -> Result<process::Output, KnownError> {
        system_command_output("flatpak", vec!["run", &self.id])
    }

    fn flatpak_install(&self) -> Result<process::Output, KnownError> {
        system_command_output("flatpak", vec!["install", "-y", &self.id])
    }

    fn flatpak_uninstall(&self) -> Result<process::Output, KnownError> {
        system_command_output("flatpak", vec!["uninstall", "-y", &self.id])
    }

    // TODO: unit test this logic directly
    fn get_running_flatpak_applications(&self) -> Result<Vec<String>, KnownError> {
        // TODO: error handling
        let res = system_command_output("flatpak", vec!["ps", "--columns=application"]);

        match &res {
            Ok(output_obj) => {
                let text = String::from_utf8_lossy(&output_obj.stdout);
                let lines = text.trim().split("\n").map(|s| s.to_string()).collect();
                Ok(lines)
            }
            Err(e) => Err(KnownError::SystemCommandParse(Box::new(
                ActionErrorTEMPORARY {
                    message: format!("Failed to parse 'flatpak ps' output: {:?}", e),
                },
            ))),
        }
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
}

impl ProviderChecks for Flatpak {
    fn is_installable(&self) -> Result<bool, KnownError> {
        // Any flatpaks we explicitly list will be installable.
        Ok(true)
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

    fn is_addable_to_steam(&self) -> Result<bool, KnownError> {
        // Flatpaks are always addable to Steam.
        Ok(true)
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
        self.flatpak_run()?;
        success!()
    }

    #[cfg(not(test))]
    fn kill(&self) -> Result<ActionSuccess, KnownError> {
        // TODO: run 'flatpak kill <id>' here
        system_command_output("flatpak", vec!["kill", &self.id])?;
        success!()
    }

    #[cfg(test)]
    fn kill(&self) -> Result<ActionSuccess, KnownError> {
        // TODO: any further testing needed here?
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

    fn flatpak_run(&self) -> Result<process::Output, KnownError> {
        Ok(process::Output {
            status: process::ExitStatus::default(),
            stdout: "flatpak run success in test".as_bytes().into(),
            stderr: [].into(),
        })
    }

    fn flatpak_install(&self) -> Result<process::Output, KnownError> {
        Ok(process::Output {
            status: process::ExitStatus::default(),
            stdout: "flatpak install success in test".as_bytes().into(),
            stderr: [].into(),
        })
    }

    fn flatpak_uninstall(&self) -> Result<process::Output, KnownError> {
        Ok(process::Output {
            status: process::ExitStatus::default(),
            stdout: "flatpak uninstall success in test".as_bytes().into(),
            stderr: [].into(),
        })
    }

    fn get_running_flatpak_applications(&self) -> Result<Vec<String>, KnownError> {
        Ok(vec!["test_pkg".into(), "test_pkg2".into()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
        Ok(())
    }

    #[test]
    fn test_is_pkg_installed_false() -> Result<(), KnownError> {
        let provider = Flatpak::new("test_pkg_not_installed");
        assert!(!provider.is_installed()?);
        Ok(())
    }

    #[test]
    fn test_installable() -> Result<(), KnownError> {
        let provider = Flatpak::new("test_pkg");
        assert!(provider.is_installable()?);
        Ok(())
    }

    #[test]
    fn test_is_pkg_running_true() -> Result<(), KnownError> {
        let provider = Flatpak::new("test_pkg");
        assert!(provider.is_running()?);
        Ok(())
    }

    #[test]
    fn test_is_pkg_running_false() -> Result<(), KnownError> {
        let provider = Flatpak::new("jfdklsajfds");
        assert!(!provider.is_running()?);
        Ok(())
    }
}
