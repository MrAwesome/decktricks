use crate::prelude::*;

use std::rc::Rc;
use std::marker::PhantomData;

pub struct SimpleCommandProvider<State> {
    pub data: Rc<SimpleCommandProviderData>,
    pub state: PhantomData<State>,
}

#[derive(Debug, Copy, Clone)]
pub struct SimpleCommandProviderData {}

#[allow(refining_impl_trait)]
impl<'a, State> ProviderChecks<'a> for SimpleCommandProvider<State> {
    fn installable(&self) -> Result<SimpleCommandProvider<InstallableState>, PLACEHOLDER> {
        success_provider!(self, SimpleCommandProvider)
    }

    fn installed(&self) -> Result<impl Installed, PLACEHOLDER> {
        success_provider!(self, SimpleCommandProvider)
    }
    fn runnable(&self) -> Result<impl Runnable, PLACEHOLDER> {
        success_provider!(self, SimpleCommandProvider)
    }
    fn running(&self) -> Result<impl Running, PLACEHOLDER> {
        success_provider!(self, SimpleCommandProvider)
    }
    fn addable_to_steam(&self) -> Result<impl AddableToSteam, PLACEHOLDER> {
        success_provider!(self, SimpleCommandProvider)
    }
}

impl Installed for SimpleCommandProvider<InstalledState> {
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

impl Installable for SimpleCommandProvider<InstallableState> {
    fn install(&self) -> Result<(), PLACEHOLDER> {
        Err(PLACEHOLDER {})
    }
}

impl Runnable for SimpleCommandProvider<RunnableState> {
    fn run(&self) -> Result<(), PLACEHOLDER> {
        Err(PLACEHOLDER {})
    }
}

impl Running for SimpleCommandProvider<RunningState> {
    fn kill(&self) -> Result<(), PLACEHOLDER> {
        Err(PLACEHOLDER {})
    }
}

impl AddableToSteam for SimpleCommandProvider<AddableToSteamState> {
    fn add_to_steam(&self) -> Result<(), PLACEHOLDER> {
        Err(PLACEHOLDER {})
    }
}
