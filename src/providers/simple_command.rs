use crate::prelude::*;

use std::rc::Rc;

#[derive(Debug, Copy, Clone)]
pub struct SimpleCommandProviderData;

pub type SimpleCommandProvider = Provider<SimpleCommandProviderData>;

#[allow(refining_impl_trait)]
impl<State: KnownState> ProviderChecks<SimpleCommandProviderData>
    for Provider<SimpleCommandProviderData, State>
where
    State: KnownState,
{
    fn is_installable(
        &self,
    ) -> Result<Provider<SimpleCommandProviderData, IsInstallable>, PLACEHOLDER> {
        success_provider!(self, SimpleCommandProvider)
    }

    fn is_installed(
        &self,
    ) -> Result<Provider<SimpleCommandProviderData, IsInstalled>, PLACEHOLDER> {
        success_provider!(self, SimpleCommandProvider)
    }
    fn is_runnable(&self) -> Result<Provider<SimpleCommandProviderData, IsRunnable>, PLACEHOLDER> {
        success_provider!(self, SimpleCommandProvider)
    }
    fn is_running(&self) -> Result<Provider<SimpleCommandProviderData, IsRunning>, PLACEHOLDER> {
        success_provider!(self, SimpleCommandProvider)
    }
    fn is_addable_to_steam(
        &self,
    ) -> Result<Provider<SimpleCommandProviderData, IsAddableToSteam>, PLACEHOLDER> {
        success_provider!(self, SimpleCommandProvider)
    }
}

impl Installed for Provider<SimpleCommandProviderData, IsInstalled> {
    fn update(&self) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }

    fn remove(&self) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }

    fn force_reinstall(&self) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }
}

impl Installable for Provider<SimpleCommandProviderData, IsInstallable> {
    fn install(&self) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }
}

impl Runnable for Provider<SimpleCommandProviderData, IsRunnable> {
    fn run(&self) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }
}

impl Running for Provider<SimpleCommandProviderData, IsRunning> {
    fn kill(&self) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }
}

impl AddableToSteam for Provider<SimpleCommandProviderData, IsAddableToSteam> {
    fn add_to_steam(&self) -> Result<(), Box<dyn std::error::Error>> {
        unimplemented!()
    }
}
