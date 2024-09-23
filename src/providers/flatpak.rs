use crate::actions::ActionErrorTEMPORARY;
use crate::prelude::*;
#[cfg(not(test))]
use crate::run_system_command::{system_command_output, system_command_ran_successfully};
use std::io;
use std::marker::PhantomData;
use std::process;
use std::rc::Rc;

struct Flatpak {
    id: String,
}



#[derive(Debug, Clone)]
pub struct FlatpakProviderData {
    id: String,
}

impl KnownProviderData for FlatpakProviderData {}

pub type FlatpakProvider = Provider<FlatpakProviderData>;

pub fn new_flatpak_provider(id: String) -> FlatpakProvider {
    Provider {
        data: Rc::new(FlatpakProviderData { id }),
        state: PhantomData::<DefaultState>,
    }
}

#[cfg(not(test))]
impl<State: KnownState> Provider<FlatpakProviderData, State> {
    fn is_pkg_installed(&self) -> bool {
        system_command_ran_successfully("flatpak", vec!["info", &self.data.id])
    }

    // NOTE: Can handle/track child pid status here, but
    // `flatpak ps` gives us that easily and authoritatively.
    fn flatpak_run(&self) -> io::Result<process::Output> {
        system_command_output("flatpak", vec!["run", &self.data.id])
    }

    fn flatpak_install(&self) -> io::Result<process::Output> {
        system_command_output("flatpak", vec!["install", "-y", &self.data.id])
    }

    fn flatpak_uninstall(&self) -> io::Result<process::Output> {
        system_command_output("flatpak", vec!["uninstall", "-y", &self.data.id])
    }

    // TODO: unit test this logic directly
    fn get_running_flatpak_applications(&self) -> Result<Vec<String>, PLACEHOLDER> {
        // TODO: error handling
        let res = system_command_output("flatpak", vec!["ps", "--columns=application"]);

        match &res {
            Ok(output_obj) => {
                let text = String::from_utf8_lossy(&output_obj.stdout);
                let lines = text.trim().split("\n").map(|s| s.to_string()).collect();
                Ok(lines)
            }
            Err(_) => unimplemented!(),
        }
    }
}

impl<State: KnownState> Provider<FlatpakProviderData, State> {
    fn is_pkg_running(&self) -> bool {
        // TODO: error handling
        for line in self.get_running_flatpak_applications().unwrap() {
            if line == self.data.id {
                return true;
            }
        }
        false
    }
}

#[allow(refining_impl_trait)]
impl<State: KnownState> ProviderChecks<FlatpakProviderData> for Provider<FlatpakProviderData, State>
where
    State: KnownState,
{
    fn is_installable(&self) -> Result<Provider<FlatpakProviderData, IsInstallable>, PLACEHOLDER> {
        // Any flatpaks we explicitly list will be installable.
        success!(self)
    }

    fn is_installed(&self) -> Result<Provider<FlatpakProviderData, IsInstalled>, PLACEHOLDER> {
        if self.is_pkg_installed() {
            success!(self)
        } else {
            // TODO: will these errors ever be seen? or can this just return an Err(()) or such?
            Err(ActionErrorTEMPORARY {
                message: format!("Package {} not installed!", self.data.id),
            })
        }
    }

    fn is_runnable(&self) -> Result<Provider<FlatpakProviderData, IsRunnable>, PLACEHOLDER> {
        if self.is_installed().is_ok() {
            success!(self)
        } else {
            Err(ActionErrorTEMPORARY {
                message: format!("Package {} not runnable!", self.data.id),
            })
        }
    }

    fn is_running(&self) -> Result<Provider<FlatpakProviderData, IsRunning>, PLACEHOLDER> {
        if self.is_pkg_running() {
            success!(self)
        } else {
            Err(ActionErrorTEMPORARY {
                message: format!("Package {} not running!", self.data.id),
            })
        }
    }

    fn is_addable_to_steam(
        &self,
    ) -> Result<Provider<FlatpakProviderData, IsAddableToSteam>, PLACEHOLDER> {
        // Flatpaks are always addable to Steam.
        success!(self)
    }
}

impl Installed for Provider<FlatpakProviderData, IsInstalled> {
    // NOTE!!!!! update takes user input on the command line (so pass -y)
    // , and *often will require a second run* if doing a full update of all packages
    fn update(&self) -> Result<(), DynamicError> {
        unimplemented!()
    }

    fn remove(&self) -> Result<(), DynamicError> {
        self.flatpak_uninstall()?;
        Ok(())
    }

    fn force_reinstall(&self) -> Result<(), DynamicError> {
        unimplemented!()
    }
}

impl Installable for Provider<FlatpakProviderData, IsInstallable> {
    fn install(&self) -> Result<(), DynamicError> {
        self.flatpak_install()?;
        Ok(())
    }
}

impl Runnable for Provider<FlatpakProviderData, IsRunnable> {
    fn run(&self) -> Result<(), DynamicError> {
        // TODO: check return status and return Err if appropriate
        self.flatpak_run()?;
        Ok(())
    }
}

impl Running for Provider<FlatpakProviderData, IsRunning> {
    #[cfg(not(test))]
    fn kill(&self) -> Result<(), DynamicError> {
        // TODO: run 'flatpak kill <id>' here
        system_command_output("flatpak", vec!["kill", &self.data.id])?;
        Ok(())
    }

    #[cfg(test)]
    fn kill(&self) -> Result<(), DynamicError> {
        // TODO: any further testing needed here?
        Ok(())
    }
}

impl AddableToSteam for Provider<FlatpakProviderData, IsAddableToSteam> {
    fn add_to_steam(&self) -> Result<(), DynamicError> {
        unimplemented!()
    }
}

#[cfg(test)]
impl<State: KnownState> Provider<FlatpakProviderData, State> {
    fn is_pkg_installed(&self) -> bool {
        self.data.id == "test_pkg_installed"
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

    fn get_running_flatpak_applications(&self) -> Result<Vec<String>, PLACEHOLDER> {
        Ok(vec!["test_pkg".into(), "test_pkg2".into()])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_flatpak_provider() {
        let provider = new_flatpak_provider("test_pkg".into());

        assert_eq!(provider.data.id, "test_pkg");
    }

    #[test]
    fn test_is_pkg_installed_true() {
        let provider = new_flatpak_provider("test_pkg_installed".into());
        assert!(provider.is_installed().is_ok());
    }

    #[test]
    fn test_is_pkg_installed_false() {
        let provider = new_flatpak_provider("test_pkg_not_installed".into());
        assert!(!provider.is_installed().is_ok());
    }

    #[test]
    fn test_installable() {
        let provider = new_flatpak_provider("test_pkg".into());
        let installable = provider.is_installable();
        assert!(installable.is_ok());
    }

    #[test]
    fn test_is_pkg_running_true() {
        let provider = new_flatpak_provider("test_pkg".into());
        assert!(provider.is_running().is_ok());
    }

    #[test]
    fn test_is_pkg_running_false() {
        let provider = new_flatpak_provider("jfdklsajfds".into());
        assert!(!provider.is_running().is_ok());
    }
}
