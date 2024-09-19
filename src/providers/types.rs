//use crate::providers::decky_installer::DeckyInstallerProviderData;
//use crate::providers::flatpak::FlatpakProviderData;
use crate::prelude::*;

use crate::actions::ActionErrorTEMPORARY;
use crate::tricks_config::ProviderConfig;
use crate::tricks_config::Trick;
use std::marker::PhantomData;
use std::rc::Rc;

//use super::decky_installer::DeckyInstallerProvider;
use super::flatpak::FlatpakProvider;
use super::flatpak::new_flatpak_provider;
//use super::decky_installer::new_decky_installer_provider;

pub trait KnownProviderData {}

// These are the states that any given trick can be in.
// They can be thought of as tags - any trick can be in multiple states
// at any given time, but we only know once we run the predicate check
// for a state in that moment.
pub struct DefaultState;
pub struct IsInstalled;
pub struct IsInstallable;
pub struct IsRunnable;
pub struct IsRunning;
pub struct IsAddableToSteam;

pub trait KnownState: sealed::Sealed {}

mod sealed {
    pub trait Sealed {}

    impl Sealed for super::DefaultState {}
    impl Sealed for super::IsInstalled {}
    impl Sealed for super::IsInstallable {}
    impl Sealed for super::IsRunnable {}
    impl Sealed for super::IsRunning {}
    impl Sealed for super::IsAddableToSteam {}
}

impl KnownState for DefaultState {}
impl KnownState for IsInstalled {}
impl KnownState for IsInstallable {}
impl KnownState for IsRunnable {}
impl KnownState for IsRunning {}
impl KnownState for IsAddableToSteam {}

pub type PLACEHOLDER = ActionErrorTEMPORARY;

// Data: any data your provider wants to keep track of internally
// State: one of the listed states above
pub struct Provider<Data: ?Sized, State: KnownState = DefaultState> {
    pub state: PhantomData<State>,
    pub data: Rc<Data>,
}

pub fn provider_from_trick<Data: KnownProviderData + ?Sized>(
    trick: &Trick,
) -> Result<Box<dyn ProviderChecks<Data>>, DynamicError>
where
    //TODO: figure out why this trait bound does not work
    FlatpakProvider: ProviderChecks<Data>,
    //DeckyInstallerProvider: ProviderChecks<Data>,
    //SimpleCommandProvider: ProviderChecks<Data>,
{
    match &trick.provider_config {
        // TODO: fix clone
        ProviderConfig::Flatpak(flatpak) => Ok(Box::new(new_flatpak_provider(flatpak.id.clone()))),
//        ProviderConfig::DeckyInstaller => Ok(Box::new(
//            new_decky_installer_provider(),
//        )),
        //        ProviderConfig::SimpleCommand => Box::new(Provider {
        //            data: Rc::new(SimpleCommandProviderData),
        //            state: PhantomData::<DefaultState>,
        //        }),
        _ => unimplemented!(),
    }
}

pub trait ProviderChecks<Data: ?Sized> {
    fn is_installable(&self) -> Result<Provider<Data, IsInstallable>, PLACEHOLDER>;
    fn is_installed(&self) -> Result<Provider<Data, IsInstalled>, PLACEHOLDER>;
    fn is_runnable(&self) -> Result<Provider<Data, IsRunnable>, PLACEHOLDER>;
    fn is_running(&self) -> Result<Provider<Data, IsRunning>, PLACEHOLDER>;
    fn is_addable_to_steam(&self) -> Result<Provider<Data, IsAddableToSteam>, PLACEHOLDER>;
}

pub trait Runnable {
    fn run(&self) -> Result<(), DynamicError>;
}

pub trait Running {
    fn kill(&self) -> Result<(), DynamicError>;
}

pub trait Installable {
    fn install(&self) -> Result<(), DynamicError>;
}

pub trait Installed {
    fn update(&self) -> Result<(), DynamicError>;
    fn remove(&self) -> Result<(), DynamicError>;
    fn force_reinstall(&self) -> Result<(), DynamicError>;
}

pub trait AddableToSteam {
    fn add_to_steam(&self) -> Result<(), DynamicError>;
    //TODO: someday
    //fn remove_from_steam(&self) -> Result<(), DynamicError>>;
}

pub trait ProviderActions: Runnable + Running + Installable + Installed + AddableToSteam {}
