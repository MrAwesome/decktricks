//use crate::prelude::*;

pub struct DefaultState;
pub struct InstalledState;
pub struct InstallableState;
pub struct RunnableState;
pub struct RunningState;
pub struct AddableToSteamState;

#[derive(Debug)]
pub struct PLACEHOLDER {}

pub trait ProviderChecks<'a> {
    fn installable(&self) -> Result<impl Installable, PLACEHOLDER>;
    fn installed(&self) -> Result<impl Installed, PLACEHOLDER>;
    fn runnable(&self) -> Result<impl Runnable, PLACEHOLDER>;
    fn running(&self) -> Result<impl Running, PLACEHOLDER>;
    fn addable_to_steam(&self) -> Result<impl AddableToSteam, PLACEHOLDER>;
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
