use crate::prelude::*;
#[cfg(not(test))]
use crate::run_system_command::{
    spawn_system_command, system_command_output, system_command_ran_successfully,
};
use std::rc::Rc;

use std::marker::PhantomData;

#[derive(Debug, Clone)]
pub struct FlatpakProviderData<'a> {
    id: &'a str,
    debug: bool,
}

impl<'a> Provider<FlatpakProviderData<'a>, DefaultState> {
    pub fn flatpak(id: &'a str, debug: bool) -> Provider<FlatpakProviderData<'a>, DefaultState> {
        Provider {
            data: Rc::new(FlatpakProviderData { id, debug }),
            state: PhantomData::<DefaultState>,
        }
    }
}

// NOTE: could separate into self.checks and self.actions

#[cfg(not(test))]
impl<'a, State> Provider<FlatpakProviderData<'a>, State> {
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
        if self.data.debug {
            dbg!(&res);
        }

        match &res {
            Ok(output_obj) => {
                let text = String::from_utf8_lossy(&output_obj.stdout);
                let lines = text.trim().split("\n").map(|s| s.to_string()).collect();
                if self.data.debug {
                    dbg!(&lines);
                }
                return Ok(lines);
            }
            Err(_) => Err(PLACEHOLDER{}),
        }
    }
}

impl<'a, State> Provider<FlatpakProviderData<'a>, State> {
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
impl<'a, State> Provider<FlatpakProviderData<'a>, State> {
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
impl<'a, State> ProviderChecks<'a, FlatpakProviderData<'a>> for Provider<FlatpakProviderData<'a>, State> {
    fn installable(&self) -> Result<Provider<FlatpakProviderData<'a>, InstallableState>, PLACEHOLDER> {
        success_provider!(self, FlatpakProvider)
    }

    fn installed(&self) -> Result<Provider<FlatpakProviderData<'a>, InstalledState>, PLACEHOLDER> {
        if self.is_pkg_installed() {
            success_provider!(self, FlatpakProvider)
        } else {
            Err(PLACEHOLDER {})
        }
    }

    fn runnable(&self) -> Result<Provider<FlatpakProviderData<'a>, RunnableState>, PLACEHOLDER> {
        if self.installed().is_ok() {
            success_provider!(self, FlatpakProvider)
        } else {
            Err(PLACEHOLDER {})
        }
    }

    fn running(&self) -> Result<Provider<FlatpakProviderData<'a>, RunningState>, PLACEHOLDER> {
        if self.is_pkg_running() {
            success_provider!(self, FlatpakProvider)
        } else {
            Err(PLACEHOLDER {})
        }
    }

    fn addable_to_steam(&self) -> Result<Provider<FlatpakProviderData<'a>, AddableToSteamState>, PLACEHOLDER> {
        // Flatpaks are always addable to Steam.
        success_provider!(self, FlatpakProvider)
    }
}

impl<'a> Installed for Provider<FlatpakProviderData<'a>, InstalledState> {
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

impl<'a> Installable for Provider<FlatpakProviderData<'a>, InstallableState> {
    fn install(&self) -> Result<(), PLACEHOLDER> {
        Err(PLACEHOLDER {})
    }
}

impl<'a> Runnable for Provider<FlatpakProviderData<'a>, RunnableState> {
    fn run(&self) -> Result<(), PLACEHOLDER> {
        // TODO: check return status and return Err if appropriate
        self.flatpak_run();
        Ok(())
    }
}

impl<'a> Running for Provider<FlatpakProviderData<'a>, RunningState> {
    fn kill(&self) -> Result<(), PLACEHOLDER> {
        // TODO: run 'flatpak kill <id>' here
        Err(PLACEHOLDER {})
    }
}

impl<'a> AddableToSteam for Provider<FlatpakProviderData<'a>, AddableToSteamState> {
    fn add_to_steam(&self) -> Result<(), PLACEHOLDER> {
        Err(PLACEHOLDER {})
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_flatpak_provider() {
        let provider = Provider::flatpak("test_pkg", true);

        assert_eq!(provider.data.id, "test_pkg");
    }

    #[test]
    fn test_is_pkg_installed_true() {
        let provider = Provider::flatpak("test_pkg_installed", true);
        assert!(provider.installed().is_ok());
    }

    #[test]
    fn test_is_pkg_installed_false() {
        let provider = Provider::flatpak("test_pkg_not_installed", true);
        assert!(!provider.installed().is_ok());
    }

    #[test]
    fn test_installable() {
        let provider = Provider::flatpak("test_pkg", true);
        let installable = provider.installable();
        assert!(installable.is_ok());
    }

    #[test]
    fn test_is_pkg_running_true() {
        let provider = Provider::flatpak("test_pkg", true);
        let _ = provider.data.debug;
        assert!(provider.running().is_ok());
    }

    #[test]
    fn test_is_pkg_running_false() {
        let provider = Provider::flatpak("jfdklsajfds", true);
        assert!(!provider.running().is_ok());
    }
}
