use crate::prelude::*;
use crate::run_system_command::{spawn_system_command, system_command_ran_successfully};
use std::rc::Rc;

use std::marker::PhantomData;

// TODO: try returning self with a pointer instead of a clone of data

#[derive(Debug, Clone)]
pub struct FlatpakProviderData<'a> {
    id: &'a str,
}

#[derive(Debug)]
pub struct FlatpakProvider<'a, State> {
    pub data: Rc<FlatpakProviderData<'a>>,
    pub state: PhantomData<State>,
}

impl<'a> FlatpakProvider<'a, DefaultState> {
    pub fn new(id: &str) -> FlatpakProvider<DefaultState> {
        FlatpakProvider {
            data: Rc::new(FlatpakProviderData { id }),
            state: PhantomData::<DefaultState>,
        }
    }
}


impl<'a, State> FlatpakProvider<'a, State> {
    fn is_pkg_installed(&self) -> bool {
        system_command_ran_successfully("flatpak", vec!["info", &self.data.id], false)
    }

    // NOTE: Can handle/track child pid status here, but
    // `flatpak ps` gives us that easily and authoritatively.
    fn flatpak_run(&self) {
        spawn_system_command("flatpak", vec!["run", &self.data.id])
    }
}

#[allow(refining_impl_trait)]
impl<'a, State> ProviderChecks<'a> for FlatpakProvider<'a, State> {
    fn installable(&self) -> Result<FlatpakProvider<'a, InstallableState>, PLACEHOLDER> {
        success_provider!(self, FlatpakProvider)
    }

    fn installed(&self) -> Result<impl Installed, PLACEHOLDER> {
        if self.is_pkg_installed() {
            success_provider!(self, FlatpakProvider)
        } else {
            Err(PLACEHOLDER {})
        }
    }

    fn runnable(&self) -> Result<FlatpakProvider<RunnableState>, PLACEHOLDER> {
        if self.installed().is_ok() {
            success_provider!(self, FlatpakProvider)
        } else {
            Err(PLACEHOLDER {})
        }
    }

    fn running(&self) -> Result<impl Running, PLACEHOLDER> {
        // TODO: run `flatpak ps --columns=application` and look for the id
        success_provider!(self, FlatpakProvider)
    }

    fn addable_to_steam(&self) -> Result<impl AddableToSteam, PLACEHOLDER> {
        // Flatpaks are always addable to Steam.
        success_provider!(self, FlatpakProvider)
    }
}

impl<'a> Installed for FlatpakProvider<'a, InstalledState> {
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

impl<'a> Installable for FlatpakProvider<'a, InstallableState> {
    fn install(&self) -> Result<(), PLACEHOLDER> {
        Err(PLACEHOLDER {})
    }
}

impl<'a> Runnable for FlatpakProvider<'a, RunnableState> {
    fn run(&self) -> Result<(), PLACEHOLDER> {
        // TODO: check return status and return Err if appropriate
        self.flatpak_run();
        Ok(())
    }
}

impl<'a> Running for FlatpakProvider<'a, RunningState> {
    fn kill(&self) -> Result<(), PLACEHOLDER> {
        // TODO: run 'flatpak kill <id>' here
        Err(PLACEHOLDER {})
    }
}

impl<'a> AddableToSteam for FlatpakProvider<'a, AddableToSteamState> {
    fn add_to_steam(&self) -> Result<(), PLACEHOLDER> {
        Err(PLACEHOLDER {})
    }
}
