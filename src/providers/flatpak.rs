use crate::actions::ActionSuccess;
use crate::prelude::*;
#[cfg(not(test))]
use crate::run_system_command::{system_command_output, system_command_ran_successfully};
use std::io;
use std::process;
use crate::tricks_config::Flatpak;

impl Flatpak {
    pub fn new<S: Into<String>>(id: S) -> Self {
        Flatpak {
            id: id.into()
        }
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
    fn is_pkg_installed(&self) -> bool {
        system_command_ran_successfully("flatpak", vec!["info", &self.id])
    }

    // NOTE: Can handle/track child pid status here, but
    // `flatpak ps` gives us that easily and authoritatively.
    fn flatpak_run(&self) -> io::Result<process::Output> {
        system_command_output("flatpak", vec!["run", &self.id])
    }

    fn flatpak_install(&self) -> io::Result<process::Output> {
        system_command_output("flatpak", vec!["install", "-y", &self.id])
    }

    fn flatpak_uninstall(&self) -> io::Result<process::Output> {
        system_command_output("flatpak", vec!["uninstall", "-y", &self.id])
    }

    // TODO: unit test this logic directly
    fn get_running_flatpak_applications(&self) -> Result<Vec<String>, DynamicError> {
        // TODO: error handling
        let res = system_command_output("flatpak", vec!["ps", "--columns=application"]);

        match &res {
            Ok(output_obj) => {
                let text = String::from_utf8_lossy(&output_obj.stdout);
                let lines = text.trim().split("\n").map(|s| s.to_string()).collect();
                Ok(lines)
            },
            Err(e) => Err(Box::new(ActionErrorTEMPORARY { message: format!("Failed to parse 'flatpak ps' output: {:?}", e) })),
        }
    }
}

impl Flatpak {
    fn is_pkg_running(&self) -> Result<bool, DynamicError> {
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
    fn is_installable(&self) -> Result<(), DynamicError> {
        // Any flatpaks we explicitly list will be installable.
        Ok(())
    }

    fn is_installed(&self) -> Result<(), DynamicError> {
        if self.is_pkg_installed() {
            Ok(())
        } else {
            // TODO: will these errors ever be seen? or can this just return an Err(()) or such?
            Err(Box::new(ActionErrorTEMPORARY {
                message: format!("Package {} not installed!", self.id),
            }))
        }
    }

    fn is_runnable(&self) -> Result<(), DynamicError> {
        if self.is_installed().is_ok() {
            Ok(())
        } else {
            Err(Box::new(ActionErrorTEMPORARY {
                message: format!("Package {} not runnable!", self.id),
            }))
        }
    }

    fn is_running(&self) -> Result<(), DynamicError> {
        if self.is_pkg_running()? {
            Ok(())
        } else {
            Err(Box::new(ActionErrorTEMPORARY {
                message: format!("Package {} not running!", self.id),
            }))
        }
    }

    fn is_addable_to_steam(
        &self,
    ) -> Result<(), DynamicError> {
        // Flatpaks are always addable to Steam.
        Ok(())
    }
}

impl ProviderActions for Flatpak {
    // NOTE!!!!! update takes user input on the command line (so pass -y)
    // , and *often will require a second run* if doing a full update of all packages
    fn update(&self) -> Result<ActionSuccess, DynamicError> {
        unimplemented!()
    }

    fn uninstall(&self) -> Result<ActionSuccess, DynamicError> {
        self.flatpak_uninstall()?;
        success!("\"{}\" uninstalled successfully.", self.id)
    }

    fn force_reinstall(&self) -> Result<ActionSuccess, DynamicError> {
        unimplemented!()
    }

    fn install(&self) -> Result<ActionSuccess, DynamicError> {
        self.flatpak_install()?;
        success!("\"{}\" installed successfully.", self.id)
    }

    fn run(&self) -> Result<ActionSuccess, DynamicError> {
        // TODO: check return status and return Err if appropriate
        self.flatpak_run()?;
        success!()
    }

    #[cfg(not(test))]
    fn kill(&self) -> Result<ActionSuccess, DynamicError> {
        // TODO: run 'flatpak kill <id>' here
        system_command_output("flatpak", vec!["kill", &self.id])?;
        success!()
    }

    #[cfg(test)]
    fn kill(&self) -> Result<ActionSuccess, DynamicError> {
        // TODO: any further testing needed here?
        success!()
    }

    fn add_to_steam(&self) -> Result<ActionSuccess, DynamicError> {
        unimplemented!()
    }
}

#[cfg(test)]
impl Flatpak {
    fn is_pkg_installed(&self) -> bool {
        self.id == "test_pkg_installed"
    }

    fn flatpak_run(&self) -> io::Result<process::Output> {
        Ok(process::Output {
            status: process::ExitStatus::default(),
            stdout: "flatpak run success in test".as_bytes().into(),
            stderr: [].into(),
        })
    }

    fn flatpak_install(&self) -> io::Result<process::Output> {
        Ok(process::Output {
            status: process::ExitStatus::default(),
            stdout: "flatpak install success in test".as_bytes().into(),
            stderr: [].into(),
        })
    }

    fn flatpak_uninstall(&self) -> io::Result<process::Output> {
        Ok(process::Output {
            status: process::ExitStatus::default(),
            stdout: "flatpak uninstall success in test".as_bytes().into(),
            stderr: [].into(),
        })
    }

    fn get_running_flatpak_applications(&self) -> Result<Vec<String>, DynamicError> {
        Ok(vec!["test_pkg".into(), "test_pkg2".into()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_flatpak_provider() {
        let provider = Flatpak::new("test_pkg");

        assert_eq!(provider.id, "test_pkg");
    }

    #[test]
    fn test_is_pkg_installed_true() {
        let provider = Flatpak::new("test_pkg_installed");
        assert!(provider.is_installed().is_ok());
    }

    #[test]
    fn test_is_pkg_installed_false() {
        let provider = Flatpak::new("test_pkg_not_installed");
        assert!(!provider.is_installed().is_ok());
    }

    #[test]
    fn test_installable() {
        let provider = Flatpak::new("test_pkg");
        let installable = provider.is_installable();
        assert!(installable.is_ok());
    }

    #[test]
    fn test_is_pkg_running_true() {
        let provider = Flatpak::new("test_pkg");
        assert!(provider.is_running().is_ok());
    }

    #[test]
    fn test_is_pkg_running_false() {
        let provider = Flatpak::new("jfdklsajfds");
        assert!(!provider.is_running().is_ok());
    }
}
