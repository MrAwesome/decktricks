use crate::prelude::*;

use std::rc::Rc;

#[derive(Debug, Copy, Clone)]
pub struct SimpleCommandProviderData {}

#[allow(refining_impl_trait)]
impl<'a, State: KnownState> ProviderChecks<'a, SimpleCommandProviderData> for Provider<SimpleCommandProviderData, State> {
    fn is_installable(&self) -> Result<Provider<SimpleCommandProviderData, IsInstallable>, PLACEHOLDER> {
        success_provider!(self, SimpleCommandProvider)
    }

    fn is_installed(&self) -> Result<Provider<SimpleCommandProviderData, IsInstalled>, PLACEHOLDER> {
        success_provider!(self, SimpleCommandProvider)
    }
    fn is_runnable(&self) -> Result<Provider<SimpleCommandProviderData, IsRunnable>, PLACEHOLDER> {
        success_provider!(self, SimpleCommandProvider)
    }
    fn is_running(&self) -> Result<Provider<SimpleCommandProviderData, IsRunning>, PLACEHOLDER> {
        success_provider!(self, SimpleCommandProvider)
    }
    fn is_addable_to_steam(&self) -> Result<Provider<SimpleCommandProviderData, IsAddableToSteam>, PLACEHOLDER> {
        success_provider!(self, SimpleCommandProvider)
    }
}

impl Installed for Provider<SimpleCommandProviderData, IsInstalled> {
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

impl Installable for Provider<SimpleCommandProviderData, IsInstallable> {
    fn install(&self) -> Result<(), PLACEHOLDER> {
        Err(PLACEHOLDER {})
    }
}

impl Runnable for Provider<SimpleCommandProviderData, IsRunnable> {
    fn run(&self) -> Result<(), PLACEHOLDER> {
        Err(PLACEHOLDER {})
    }
}

impl Running for Provider<SimpleCommandProviderData, IsRunning> {
    fn kill(&self) -> Result<(), PLACEHOLDER> {
        Err(PLACEHOLDER {})
    }
}

impl AddableToSteam for Provider<SimpleCommandProviderData, IsAddableToSteam> {
    fn add_to_steam(&self) -> Result<(), PLACEHOLDER> {
        Err(PLACEHOLDER {})
    }
}
