use crate::actions::ActionErrorTEMPORARY;
use crate::prelude::*;
use crate::run_system_command::system_command_ran_successfully;
use std::fs::File;
use std::io::copy;
use std::marker::PhantomData;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;
use std::rc::Rc;

//const DECKY_DOWNLOAD_URL: &str = "https://decky.xyz/download";
const DECKY_DOWNLOAD_URL: &str = "http://gleesus.net:8858/lawl.sh";

#[derive(Debug, Copy, Clone)]
pub struct DeckyInstallerProviderData;

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
    ) -> Result<Provider<DeckyInstallerProviderData, IsInstallable>, PLACEHOLDER> {
        success!(self)
    }

    fn is_installed(
        &self,
    ) -> Result<Provider<DeckyInstallerProviderData, IsInstalled>, PLACEHOLDER> {
        if system_command_ran_successfully("systemctl", vec!["is-enabled", "plugin_loader"]) {
            success!(self)
        } else {
            Err(ActionErrorTEMPORARY {
                message: format!("Decky not installed!"),
            })
        }
    }
    fn is_runnable(&self) -> Result<Provider<DeckyInstallerProviderData, IsRunnable>, PLACEHOLDER> {
        Err(ActionErrorTEMPORARY {
            message: format!("Decky is not runnable."),
        })
    }
    fn is_running(&self) -> Result<Provider<DeckyInstallerProviderData, IsRunning>, PLACEHOLDER> {
        if system_command_ran_successfully("systemctl", vec!["is-running", "plugin_loader"]) {
            success!(self)
        } else {
            Err(ActionErrorTEMPORARY {
                message: format!("Decky not running!"),
            })
        }
    }
    fn is_addable_to_steam(
        &self,
    ) -> Result<Provider<DeckyInstallerProviderData, IsAddableToSteam>, PLACEHOLDER> {
        Err(ActionErrorTEMPORARY {
            message: format!("Decky is automatically added to Steam."),
        })
    }
}

impl Installed for Provider<DeckyInstallerProviderData, IsInstalled> {
    fn update(&self) -> Result<(), DynamicError> {
        unimplemented!()
    }

    fn remove(&self) -> Result<(), DynamicError> {
        unimplemented!()
    }

    fn force_reinstall(&self) -> Result<(), DynamicError> {
        unimplemented!()
    }
}

impl Installable for Provider<DeckyInstallerProviderData, IsInstallable> {
    fn install(&self) -> Result<(), DynamicError> {
        let output = "/tmp/decky_installer.sh";

        let response = reqwest::blocking::get(DECKY_DOWNLOAD_URL)?;
        let mut dest = File::create(output)?;
        copy(&mut response.bytes()?.as_ref(), &mut dest)?;

        let mut perms = dest.metadata()?.permissions();
        perms.set_mode(0o755);
        dest.set_permissions(perms)?;

        Command::new(format!("./{}", output))
            .spawn()
            .unwrap()
            .wait()?;
        Ok(())
    }
}

impl Runnable for Provider<DeckyInstallerProviderData, IsRunnable> {
    fn run(&self) -> Result<(), DynamicError> {
        unimplemented!()
    }
}

impl Running for Provider<DeckyInstallerProviderData, IsRunning> {
    fn kill(&self) -> Result<(), DynamicError> {
        unimplemented!()
    }
}

impl AddableToSteam for Provider<DeckyInstallerProviderData, IsAddableToSteam> {
    fn add_to_steam(&self) -> Result<(), DynamicError> {
        unimplemented!()
    }
}
