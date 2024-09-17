use crate::prelude::*;
#[cfg(not(test))]
use crate::run_system_command::{
    spawn_system_command, system_command_output, system_command_ran_successfully,
};
use std::rc::Rc;

use std::marker::PhantomData;

// TODO: use Trick -- Flatpak(Flatpak) instead?
#[derive(Debug, Clone)]
pub struct FlatpakProviderData {
    id: String,
}

pub type FlatpakProvider = Provider<FlatpakProviderData>;

pub fn new_flatpak_provider(id: String) -> FlatpakProvider {
    Provider {
        data: Rc::new(FlatpakProviderData { id }),
        state: PhantomData::<DefaultState>,
    }
}

//impl<State: KnownState> ProviderActions for Provider<FlatpakProviderData, State> where Provider<FlatpakProviderData, State>: Runnable + Running + Installed + Installable + AddableToSteam { }

//impl<State: KnownState> Provider<FlatpakProviderData, State> {
//    pub fn flatpak(id: String) -> Provider<FlatpakProviderData, DefaultState> {
//        Provider {
//            data: Rc::new(FlatpakProviderData { id }),
//            state: PhantomData::<DefaultState>,
//        }
//    }
//}
//
// NOTE: could separate into self.checks and self.actions

#[cfg(not(test))]
impl<State: KnownState> Provider<FlatpakProviderData, State> {
    fn is_pkg_installed(&self) -> bool {
        system_command_ran_successfully("flatpak", vec!["info", &self.data.id], false)
    }

    // NOTE: Can handle/track child pid status here, but
    // `flatpak ps` gives us that easily and authoritatively.
    fn flatpak_run(&self) {
        spawn_system_command("flatpak", vec!["run", &self.data.id])
    }

    // TODO: unit test this logic directly
    fn get_running_flatpak_applications(&self) -> Result<Vec<String>, PLACEHOLDER> {
        // TODO: error handling
        let res = system_command_output("flatpak", vec!["ps", "--columns=application"]);

        match &res {
            Ok(output_obj) => {
                let text = String::from_utf8_lossy(&output_obj.stdout);
                let lines = text.trim().split("\n").map(|s| s.to_string()).collect();
                return Ok(lines);
            }
            Err(_) => Err(PLACEHOLDER {}),
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
        return false;
    }
}

#[cfg(test)]
impl<State: KnownState> Provider<FlatpakProviderData, State> {
    fn is_pkg_installed(&self) -> bool {
        self.data.id == "test_pkg_installed"
    }

    fn flatpak_run(&self) {
        println!("Running flatpak for {}", self.data.id);
    }

    fn get_running_flatpak_applications(&self) -> Result<Vec<String>, PLACEHOLDER> {
        Ok(vec!["test_pkg".into(), "test_pkg2".into()])
    }
}

#[allow(refining_impl_trait)]
impl<State: KnownState> ProviderChecks<FlatpakProviderData> 
    for Provider<FlatpakProviderData, State>
where
    State: KnownState,
{
    fn is_installable(&self) -> Result<Provider<FlatpakProviderData, IsInstallable>, PLACEHOLDER> {
        // Any flatpaks we explicitly list will be installable.
        success_provider!(self, FlatpakProvider)
    }

    fn is_installed(&self) -> Result<Provider<FlatpakProviderData, IsInstalled>, PLACEHOLDER> {
        if self.is_pkg_installed() {
            success_provider!(self, FlatpakProvider)
        } else {
            Err(PLACEHOLDER {})
        }
    }

    fn is_runnable(&self) -> Result<Provider<FlatpakProviderData, IsRunnable>, PLACEHOLDER> {
        if self.is_installed().is_ok() {
            success_provider!(self, FlatpakProvider)
        } else {
            Err(PLACEHOLDER {})
        }
    }

    fn is_running(&self) -> Result<Provider<FlatpakProviderData, IsRunning>, PLACEHOLDER> {
        if self.is_pkg_running() {
            success_provider!(self, FlatpakProvider)
        } else {
            Err(PLACEHOLDER {})
        }
    }

    fn is_addable_to_steam(
        &self,
    ) -> Result<Provider<FlatpakProviderData, IsAddableToSteam>, PLACEHOLDER> {
        // Flatpaks are always addable to Steam.
        success_provider!(self, FlatpakProvider)
    }
}

impl Installed for Provider<FlatpakProviderData, IsInstalled> {
    fn update(&self) -> Result<(), PLACEHOLDER> {
        Err(PLACEHOLDER {})
    }

    fn remove(&self) -> Result<(), PLACEHOLDER> {
        Err(PLACEHOLDER {})
    }

    fn force_reinstall(&self) -> Result<(), PLACEHOLDER> {
        Err(PLACEHOLDER {})
    }
}

impl Installable for Provider<FlatpakProviderData, IsInstallable> {
    fn install(&self) -> Result<(), PLACEHOLDER> {
        Err(PLACEHOLDER {})
    }
}

impl Runnable for Provider<FlatpakProviderData, IsRunnable> {
    fn run(&self) -> Result<(), PLACEHOLDER> {
        // TODO: check return status and return Err if appropriate
        self.flatpak_run();
        Ok(())
    }
}

impl Running for Provider<FlatpakProviderData, IsRunning> {
    fn kill(&self) -> Result<(), PLACEHOLDER> {
        // TODO: run 'flatpak kill <id>' here
        Err(PLACEHOLDER {})
    }
}

impl AddableToSteam for Provider<FlatpakProviderData, IsAddableToSteam> {
    fn add_to_steam(&self) -> Result<(), PLACEHOLDER> {
        Err(PLACEHOLDER {})
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
