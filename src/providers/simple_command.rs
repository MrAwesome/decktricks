use crate::prelude::*;

use std::rc::Rc;

#[derive(Debug, Copy, Clone)]
pub struct SimpleCommandProviderData {}

#[allow(refining_impl_trait)]
impl<'a, State> ProviderChecks<'a, SimpleCommandProviderData> for Provider<SimpleCommandProviderData, State> {
    fn installable(&self) -> Result<Provider<SimpleCommandProviderData, InstallableState>, PLACEHOLDER> {
        success_provider!(self, SimpleCommandProvider)
    }

    fn installed(&self) -> Result<Provider<SimpleCommandProviderData, InstalledState>, PLACEHOLDER> {
        success_provider!(self, SimpleCommandProvider)
    }
    fn runnable(&self) -> Result<Provider<SimpleCommandProviderData, RunnableState>, PLACEHOLDER> {
        success_provider!(self, SimpleCommandProvider)
    }
    fn running(&self) -> Result<Provider<SimpleCommandProviderData, RunningState>, PLACEHOLDER> {
        success_provider!(self, SimpleCommandProvider)
    }
    fn addable_to_steam(&self) -> Result<Provider<SimpleCommandProviderData, AddableToSteamState>, PLACEHOLDER> {
        success_provider!(self, SimpleCommandProvider)
    }
}

impl Installed for Provider<SimpleCommandProviderData, InstalledState> {
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

impl Installable for Provider<SimpleCommandProviderData, InstallableState> {
    fn install(&self) -> Result<(), PLACEHOLDER> {
        Err(PLACEHOLDER {})
    }
}

impl Runnable for Provider<SimpleCommandProviderData, RunnableState> {
    fn run(&self) -> Result<(), PLACEHOLDER> {
        Err(PLACEHOLDER {})
    }
}

impl Running for Provider<SimpleCommandProviderData, RunningState> {
    fn kill(&self) -> Result<(), PLACEHOLDER> {
        Err(PLACEHOLDER {})
    }
}

impl AddableToSteam for Provider<SimpleCommandProviderData, AddableToSteamState> {
    fn add_to_steam(&self) -> Result<(), PLACEHOLDER> {
        Err(PLACEHOLDER {})
    }
}
