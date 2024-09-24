use crate::actions::ActionErrorTEMPORARY;
use crate::prelude::*;

use std::rc::Rc;

#[derive(Debug, Copy, Clone)]
pub struct SimpleCommandProvider {
    command: String,
    args: Vec<String>,
}

pub type SimpleCommandProvider = Provider<SimpleCommandProviderData>;

#[allow(refining_impl_trait)]
impl<State: KnownState> ProviderChecks<SimpleCommandProviderData>
    for Provider<SimpleCommandProviderData, State>
where
    State: KnownState,
{
    fn is_installable(
        &self,
    ) -> Result<Provider<SimpleCommandProviderData, IsInstallable>, ActionErrorTEMPORARY> {
        // TODO: boxing up an error just to say the same thing every time is Bad, should switch to
        // using enum errors and just have a general error catchall for boxed errors, so that
        // functions like this can just return IsNotInstalledError
        Err(Errs::IsNotInstalledError)
    }

    fn is_installed(
        &self,
    ) -> Result<Provider<SimpleCommandProviderData, IsInstalled>, ActionErrorTEMPORARY> {
        success!(self)
    }
    fn is_runnable(&self) -> Result<Provider<SimpleCommandProviderData, IsRunnable>, ActionErrorTEMPORARY> {
        success!(self)
    }
    fn is_running(&self) -> Result<Provider<SimpleCommandProviderData, IsRunning>, ActionErrorTEMPORARY> {
        success!(self)
    }
    fn is_addable_to_steam(
        &self,
    ) -> Result<Provider<SimpleCommandProviderData, IsAddableToSteam>, ActionErrorTEMPORARY> {
        success!(self)
    }
}

impl Installed for Provider<SimpleCommandProviderData, IsInstalled> {
    fn update(&self) -> Result<ActionSuccess, DynamicError> {
        unimplemented!()
    }

    fn remove(&self) -> Result<ActionSuccess, DynamicError> {
        unimplemented!()
    }

    fn force_reinstall(&self) -> Result<ActionSuccess, DynamicError> {
        unimplemented!()
    }
}

impl Installable for Provider<SimpleCommandProviderData, IsInstallable> {
    fn install(&self) -> Result<ActionSuccess, DynamicError> {
        unimplemented!()
    }
}

impl Runnable for Provider<SimpleCommandProviderData, IsRunnable> {
    fn run(&self) -> Result<ActionSuccess, DynamicError> {
        unimplemented!()
    }
}

impl Running for Provider<SimpleCommandProviderData, IsRunning> {
    fn kill(&self) -> Result<ActionSuccess, DynamicError> {
        unimplemented!()
    }
}

impl AddableToSteam for Provider<SimpleCommandProviderData, IsAddableToSteam> {
    fn add_to_steam(&self) -> Result<ActionSuccess, DynamicError> {
        unimplemented!()
    }
}
