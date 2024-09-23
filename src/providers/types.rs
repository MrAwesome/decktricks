////use crate::providers::decky_installer::DeckyInstallerProviderData;
////use crate::providers::flatpak::FlatpakProviderData;
////use crate::providers::flatpak::FlatpakProviderData;
////use crate::providers::decky_installer::DeckyInstallerProviderData;
////use crate::providers::flatpak::new_flatpak_provider;
////use crate::providers::flatpak::FlatpakProvider;
////use crate::providers::decky_installer::new_decky_installer_provider;
////use crate::providers::decky_installer::DeckyInstallerProvider;
//use crate::prelude::*;
//
////use crate::actions::ActionErrorTEMPORARY;
////use crate::tricks_config::ProviderConfig;
////use crate::tricks_config::Trick;
////use std::marker::PhantomData;
////use std::rc::Rc;
//
////use super::decky_installer::DeckyInstallerProvider;
////use super::flatpak::FlatpakProvider;
////use super::flatpak::new_flatpak_provider;
////use super::decky_installer::new_decky_installer_provider;
//
//struct ProviderInfo {
//    id: String,
//    display_name: String,
//}
//
//// TODO: Sort in GUI
//#[derive(Debug, PartialEq, Clone)]
//enum Action {
//    Install = 0,
//    Run = 1,
//    Uninstall = 2,
//    AddToSteam = 3,
//    // Numbers above 50 are hidden behind a menu click
//    Kill = 80,
//    Info = 99,
//}
//
//const DEFAULT_ACTIONS: [Action; 1] = [Action::Info];
//
//trait Provider: ProviderChecks + ProviderActions {
//    fn execute(&self, action: &Action) -> Result<(), String> {
//        let actions = self.actions();
//        match action {
//            Action::Install if actions.contains(&Action::Install) => self.install(),
//            Action::Run if actions.contains(&Action::Run) => self.run(),
//            Action::Uninstall if actions.contains(&Action::Uninstall) => self.install(),
//            Action::AddToSteam if actions.contains(&Action::AddToSteam) => self.run(),
//            Action::Kill if actions.contains(&Action::Kill) => self.run(),
//            Action::Info if actions.contains(&Action::Info) => unimplemented!(),
//            _ => Err(format!("Action {:?} not supported for <name/id>, supported actions: <actions>", action)), 
//            
//        }
//    }
//    fn get_provider_info(&self) -> ProviderInfo;
//    fn specific_actions(&self) -> Vec<Action>;
//    fn default_actions(&self) -> Vec<Action> {
//        DEFAULT_ACTIONS
//    }
//
//    fn actions(&self) -> Vec<Action> {
//        [self.specific_actions(), self.default_actions()].concat()
//    }
//}
//
//pub trait ProviderChecks {
//    fn is_installable(&self) -> Result<(), DynamicError>;
//    fn is_installed(&self) -> Result<(), DynamicError>;
//    fn is_runnable(&self) -> Result<(), DynamicError>;
//    fn is_running(&self) -> Result<(), DynamicError>;
//    fn is_addable_to_steam(&self) -> Result<(), DynamicError>;
//}
//
//pub trait ProviderActions {
//    fn run(&self) -> Result<(), DynamicError>;
//    fn kill(&self) -> Result<(), DynamicError>;
//    fn install(&self) -> Result<(), DynamicError>;
//    fn update(&self) -> Result<(), DynamicError>;
//    fn remove(&self) -> Result<(), DynamicError>;
//    fn force_reinstall(&self) -> Result<(), DynamicError>;
//    fn add_to_steam(&self) -> Result<(), DynamicError>;
//    //TODO: someday
//    //fn remove_from_steam(&self) -> Result<(), DynamicError>>;
//}
//
////pub trait KnownProviderData {}
////
////// These are the states that any given trick can be in.
////// They can be thought of as tags - any trick can be in multiple states
////// at any given time, but we only know once we run the predicate check
////// for a state in that moment.
////pub struct DefaultState;
////pub struct IsInstalled;
////pub struct IsInstallable;
////pub struct IsRunnable;
////pub struct IsRunning;
////pub struct IsAddableToSteam;
////
////pub trait KnownState: sealed::Sealed {}
////
////mod sealed {
////    pub trait Sealed {}
////
////    impl Sealed for super::DefaultState {}
////    impl Sealed for super::IsInstalled {}
////    impl Sealed for super::IsInstallable {}
////    impl Sealed for super::IsRunnable {}
////    impl Sealed for super::IsRunning {}
////    impl Sealed for super::IsAddableToSteam {}
////}
////
////impl KnownState for DefaultState {}
////impl KnownState for IsInstalled {}
////impl KnownState for IsInstallable {}
////impl KnownState for IsRunnable {}
////impl KnownState for IsRunning {}
////impl KnownState for IsAddableToSteam {}
////
////pub type PLACEHOLDER = ActionErrorTEMPORARY;
////
////// Data: any data your provider wants to keep track of internally
////// State: one of the listed states above
////pub struct Provider<Data: ?Sized, State: KnownState = DefaultState> {
////    pub state: PhantomData<State>,
////    pub data: Rc<Data>,
////}
////
//////pub enum ProviderTypes {
//////    Flatpak(Provider<FlatpakProviderData>),
//////    DeckyInstaller(Provider<DeckyInstallerProviderData>),
//////}
////
////// TODO: fix clone
////pub fn provider_from_trick<Data: KnownProviderData + ?Sized>(
////    trick: &Trick,
////) -> Result<Box<dyn ProviderChecks<Data>>, DynamicError>
////where
////    FlatpakProvider: ProviderChecks<Data>,
////    //DeckyInstallerProvider: ProviderChecks<Data>,
////    //where
////    //    Box<dyn ProviderChecks<Data>>: From<FlatpakProvider>,
////    //    Box<dyn ProviderChecks<Data>>: From<DeckyInstallerProvider>
////{
////    match &trick.provider_config {
////        ProviderConfig::Flatpak(flatpak) => Ok(Box::new(new_flatpak_provider(flatpak.id.clone()))),
////        //        ProviderConfig::DeckyInstaller => Ok(Box::new(
////        //            new_decky_installer_provider(),
////        //        )),
////        _ => unimplemented!(),
////    }
////}
////
////pub trait ProviderChecks<Data: ?Sized> {
////    fn is_installable(&self) -> Result<Provider<Data, IsInstallable>, PLACEHOLDER>;
////    fn is_installed(&self) -> Result<Provider<Data, IsInstalled>, PLACEHOLDER>;
////    fn is_runnable(&self) -> Result<Provider<Data, IsRunnable>, PLACEHOLDER>;
////    fn is_running(&self) -> Result<Provider<Data, IsRunning>, PLACEHOLDER>;
////    fn is_addable_to_steam(&self) -> Result<Provider<Data, IsAddableToSteam>, PLACEHOLDER>;
////}
////
////pub trait Runnable {
////    fn run(&self) -> Result<(), DynamicError>;
////}
////
////pub trait Running {
////    fn kill(&self) -> Result<(), DynamicError>;
////}
////
////pub trait Installable {
////    fn install(&self) -> Result<(), DynamicError>;
////}
////
////pub trait Installed {
////    fn update(&self) -> Result<(), DynamicError>;
////    fn remove(&self) -> Result<(), DynamicError>;
////    fn force_reinstall(&self) -> Result<(), DynamicError>;
////}
////
////pub trait AddableToSteam {
////    fn add_to_steam(&self) -> Result<(), DynamicError>;
////    //TODO: someday
////    //fn remove_from_steam(&self) -> Result<(), DynamicError>>;
////}
////
////pub trait ProviderActions: Runnable + Running + Installable + Installed + AddableToSteam {}
