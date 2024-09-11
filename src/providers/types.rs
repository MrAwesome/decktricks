//use crate::prelude::*;

use std::marker::PhantomData;
use std::rc::Rc;

pub struct DefaultState;
pub struct InstalledState;
pub struct InstallableState;
pub struct RunnableState;
pub struct RunningState;
pub struct AddableToSteamState;

#[derive(Debug)]
pub struct PLACEHOLDER {}

pub struct Provider<Data, State> {
    // TODO: ensure this Rc usage won't cause memory leaks
    pub data: Rc<Data>,
    pub state: PhantomData<State>,
}

pub trait ProviderChecks<'a, Data> {
    fn installable(&self) -> Result<Provider<Data, InstallableState>, PLACEHOLDER>;
    fn installed(&self) -> Result<Provider<Data, InstalledState>, PLACEHOLDER>;
    fn runnable(&self) -> Result<Provider<Data, RunnableState>, PLACEHOLDER>;
    fn running(&self) -> Result<Provider<Data, RunningState>, PLACEHOLDER>;
    fn addable_to_steam(&self) -> Result<Provider<Data, AddableToSteamState>, PLACEHOLDER>;
}

pub trait Runnable {
    fn run(&self) -> Result<(), PLACEHOLDER>;
}

pub trait Running {
    fn kill(&self) -> Result<(), PLACEHOLDER>;
}

pub trait Installable {
    fn install(&self) -> Result<(), PLACEHOLDER>;
}

pub trait Installed {
    fn update(&self) -> Result<(), PLACEHOLDER>;
    fn remove(&self) -> Result<(), PLACEHOLDER>;
    fn force_reinstall(&self) -> Result<(), PLACEHOLDER>;
}

pub trait AddableToSteam {
    fn add_to_steam(&self) -> Result<(), PLACEHOLDER>;
    //TODO: someday
    //fn remove_from_steam(&self) -> Result<(), PLACEHOLDER>;
}
