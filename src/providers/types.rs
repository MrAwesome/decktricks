//use crate::prelude::*;

use std::marker::PhantomData;
use std::rc::Rc;

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

#[derive(Debug)]
pub struct PLACEHOLDER {}

// Data: any data your provider wants to keep track of internally
// State: one of the listed states above
pub struct Provider<Data, State: KnownState> {
    pub data: Rc<Data>,
    pub state: PhantomData<State>,
}

pub trait ProviderChecks<'a, Data> {
    fn is_installable(&self) -> Result<Provider<Data, IsInstallable>, PLACEHOLDER>;
    fn is_installed(&self) -> Result<Provider<Data, IsInstalled>, PLACEHOLDER>;
    fn is_runnable(&self) -> Result<Provider<Data, IsRunnable>, PLACEHOLDER>;
    fn is_running(&self) -> Result<Provider<Data, IsRunning>, PLACEHOLDER>;
    fn is_addable_to_steam(&self) -> Result<Provider<Data, IsAddableToSteam>, PLACEHOLDER>;
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
