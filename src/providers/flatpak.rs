use crate::prelude::*;
#[cfg(not(test))]
use crate::run_system_command::{system_command_output, system_command_ran_successfully};
use std::io;
use std::process;
use crate::tricks_config::Flatpak;

impl Flatpak {
    pub fn new(id: String) -> Self {
        Flatpak {
            id
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

//#[derive(Debug, Clone)]
//pub struct FlatpakProviderData {
//    id: String,
//}
//
//impl KnownProviderData for FlatpakProviderData {}
//
//pub type FlatpakProvider = FIXME;
//
//pub fn new_flatpak_provider(id: String) -> FlatpakProvider {
//    Provider {
//        data: Rc::new(FlatpakProviderData { id }),
//        state: PhantomData::<DefaultState>,
//    }
//}
//
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
//
//impl<State: KnownState> FIXME {
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
//
//#[allow(refining_impl_trait)]
//impl<State: KnownState> ProviderChecks<FlatpakProviderData> for FIXME
//where
//    State: KnownState,
//{
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
    fn update(&self) -> Result<(), DynamicError> {
        unimplemented!()
    }

    fn uninstall(&self) -> Result<(), DynamicError> {
        self.flatpak_uninstall()?;
        Ok(())
    }

    fn force_reinstall(&self) -> Result<(), DynamicError> {
        unimplemented!()
    }

    fn install(&self) -> Result<(), DynamicError> {
        self.flatpak_install()?;
        Ok(())
    }

    fn run(&self) -> Result<(), DynamicError> {
        // TODO: check return status and return Err if appropriate
        self.flatpak_run()?;
        Ok(())
    }

    #[cfg(not(test))]
    fn kill(&self) -> Result<(), DynamicError> {
        // TODO: run 'flatpak kill <id>' here
        system_command_output("flatpak", vec!["kill", &self.id])?;
        Ok(())
    }

    #[cfg(test)]
    fn kill(&self) -> Result<(), DynamicError> {
        // TODO: any further testing needed here?
        Ok(())
    }

    fn add_to_steam(&self) -> Result<(), DynamicError> {
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

//#[cfg(test)]
//mod tests {
//    use super::*;
//
//    #[test]
//    fn test_new_flatpak_provider() {
//        let provider = new_flatpak_provider("test_pkg".into());
//
//        assert_eq!(provider.id, "test_pkg");
//    }
//
//    #[test]
//    fn test_is_pkg_installed_true() {
//        let provider = new_flatpak_provider("test_pkg_installed".into());
//        assert!(provider.is_installed().is_ok());
//    }
//
//    #[test]
//    fn test_is_pkg_installed_false() {
//        let provider = new_flatpak_provider("test_pkg_not_installed".into());
//        assert!(!provider.is_installed().is_ok());
//    }
//
//    #[test]
//    fn test_installable() {
//        let provider = new_flatpak_provider("test_pkg".into());
//        let installable = provider.is_installable();
//        assert!(installable.is_ok());
//    }
//
//    #[test]
//    fn test_is_pkg_running_true() {
//        let provider = new_flatpak_provider("test_pkg".into());
//        assert!(provider.is_running().is_ok());
//    }
//
//    #[test]
//    fn test_is_pkg_running_false() {
//        let provider = new_flatpak_provider("jfdklsajfds".into());
//        assert!(!provider.is_running().is_ok());
//    }
//}
