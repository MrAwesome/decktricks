use crate::provider_types::*;

use std::marker::PhantomData;

pub struct DefaultState;
pub struct InstalledState;
pub struct InstallableState;
pub struct RunnableState;
pub struct RunningState;
pub struct AddableToSteamState;

pub struct SimpleCommandProvider<State> {
    pub data: SimpleCommandProviderData,
    pub state: PhantomData<State>
}

#[derive(Debug, Copy, Clone)]
pub struct SimpleCommandProviderData {
    
}

#[allow(refining_impl_trait)]
impl<State> ProviderChecks for SimpleCommandProvider<State> {
    fn installable(&self) -> Result<SimpleCommandProvider<InstallableState>, PLACEHOLDER> {
        Ok(SimpleCommandProvider{ data: self.data, state: PhantomData })
    }

    fn installed(&self) -> Result<impl Installed, PLACEHOLDER> {
        Ok(SimpleCommandProvider{ data: self.data, state: PhantomData })
        
    }
    fn runnable(&self) -> Result<impl Runnable, PLACEHOLDER> {
        Ok(SimpleCommandProvider{ data: self.data, state: PhantomData })
        
    }
    fn running(&self) -> Result<impl Running, PLACEHOLDER> {
        Ok(SimpleCommandProvider{ data: self.data, state: PhantomData })
        
    }
    fn addable_to_steam(&self) -> Result<impl AddableToSteam, PLACEHOLDER> {
        Ok(SimpleCommandProvider{ data: self.data, state: PhantomData })
        
    }

}

impl Installed for SimpleCommandProvider<InstalledState> {
    fn update(&self) -> Result<(), PLACEHOLDER> {
        Err(PLACEHOLDER{})
    }

    fn remove(&self) -> Result<(), PLACEHOLDER> {
        Err(PLACEHOLDER{})
    }

    fn force_reinstall(&self) -> Result<(), PLACEHOLDER> {
        Err(PLACEHOLDER{})
    }
}

impl Installable for SimpleCommandProvider<InstallableState> {
    fn install(&self) -> Result<(), PLACEHOLDER> {
        Err(PLACEHOLDER{})
    }
}

impl Runnable for SimpleCommandProvider<RunnableState> {
    fn run(&self) -> Result<(), PLACEHOLDER> {
        Err(PLACEHOLDER{})
    }
}

impl Running for SimpleCommandProvider<RunningState> {
    fn kill(&self) -> Result<(), PLACEHOLDER> {
        Err(PLACEHOLDER{})
    }
}

impl AddableToSteam for SimpleCommandProvider<AddableToSteamState> {
    fn add_to_steam(&self) -> Result<(), PLACEHOLDER> {
        Err(PLACEHOLDER{})
    }
}
