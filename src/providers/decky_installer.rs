use crate::prelude::*;
use crate::run_system_command::system_command_ran_successfully;
use std::fs::File;
use std::io::copy;
use std::marker::PhantomData;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;
use std::rc::Rc;

// TODO: detect if on steam deck or not, and *do not mark as installable if not on steam deck*

//const DECKY_DOWNLOAD_URL: &str = "https://decky.xyz/download";
const DECKY_DOWNLOAD_URL: &str = "http://gleesus.net:8858/lawl.sh";

#[derive(Debug, Copy, Clone)]
pub struct DeckyInstallerProviderData;

impl KnownProviderData for DeckyInstallerProviderData {}

pub type DeckyInstallerProvider = Provider<DeckyInstallerProviderData>;

pub fn new_decky_installer_provider() -> DeckyInstallerProvider {
    Provider {
        data: Rc::new(DeckyInstallerProviderData),
        state: PhantomData::<DefaultState>,
    }
}

#[allow(refining_impl_trait)]
impl<State: KnownState> ProviderChecks<DeckyInstallerProviderData>
    for Provider<DeckyInstallerProviderData, State>
where
    State: KnownState,
{
    fn is_installable(
        &self,
    ) -> Result<Provider<DeckyInstallerProviderData, IsInstallable>, ActionErrorTEMPORARY> {
        success!(self)
    }

    fn is_installed(
        &self,
    ) -> Result<Provider<DeckyInstallerProviderData, IsInstalled>, ActionErrorTEMPORARY> {
        if system_command_ran_successfully("systemctl", vec!["is-enabled", "plugin_loader"]) {
            success!(self)
        } else {
            Err(ActionErrorTEMPORARY {
                message: "Decky not installed!".into(),
            })
        }
    }
    fn is_runnable(&self) -> Result<Provider<DeckyInstallerProviderData, IsRunnable>, ActionErrorTEMPORARY> {
        Err(ActionErrorTEMPORARY {
            message: "Decky is not runnable.".into(),
        })
    }
    fn is_running(&self) -> Result<Provider<DeckyInstallerProviderData, IsRunning>, ActionErrorTEMPORARY> {
        if system_command_ran_successfully("systemctl", vec!["is-running", "plugin_loader"]) {
            success!(self)
        } else {
            Err(ActionErrorTEMPORARY {
                message: "Decky not running!".into(),
            })
        }
    }
    fn is_addable_to_steam(
        &self,
    ) -> Result<Provider<DeckyInstallerProviderData, IsAddableToSteam>, ActionErrorTEMPORARY> {
        Err(ActionErrorTEMPORARY {
            message: "Decky is automatically added to Steam.".into(),
        })
    }
}

impl Installed for Provider<DeckyInstallerProviderData, IsInstalled> {
    fn update(&self) -> Result<(), DynamicError> {
        // TODO: decky is updated by running the installer again. This may be a different command.
        self.is_installable()?.install()
    }

    fn remove(&self) -> Result<(), DynamicError> {
        // TODO: decky is removed by running the installer again. This may be a different command.
        self.is_installable()?.install()
    }

    fn force_reinstall(&self) -> Result<(), DynamicError> {
        self.is_installable()?.install()
    }
}

impl Installable for Provider<DeckyInstallerProviderData, IsInstallable> {
    fn install(&self) -> Result<(), DynamicError> {
        let temp_filename = "/tmp/decky_installer.sh";

        let response = reqwest::blocking::get(DECKY_DOWNLOAD_URL)?;

        {
            let mut dest = File::create(temp_filename)?;
            copy(&mut response.bytes()?.as_ref(), &mut dest)?;
        }

        {
            std::fs::set_permissions(temp_filename, std::fs::Permissions::from_mode(0o755))?;
        }

        Command::new(temp_filename).status()?;

        Ok(())
    }
}

impl Runnable for Provider<DeckyInstallerProviderData, IsRunnable> {
    fn run(&self) -> Result<(), DynamicError> {
        Err(Box::new(ActionErrorTEMPORARY { message: "Decky is not runnable!".into() }))
    }
}

impl Running for Provider<DeckyInstallerProviderData, IsRunning> {
    fn kill(&self) -> Result<(), DynamicError> {
        Err(Box::new(ActionErrorTEMPORARY { message: "Decky is not killable!".into() }))
    }
}

impl AddableToSteam for Provider<DeckyInstallerProviderData, IsAddableToSteam> {
    fn add_to_steam(&self) -> Result<(), DynamicError> {
        Err(Box::new(ActionErrorTEMPORARY { message: "Decky is automatically added to Steam.".into() }))
    }
}
