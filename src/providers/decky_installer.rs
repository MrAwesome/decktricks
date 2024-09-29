use crate::prelude::*;
use crate::run_system_command::system_command_ran_successfully;
use std::fs::File;
use std::io::copy;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

// TODO: detect if on steam deck or not, and *do not mark as installable if not on steam deck*

//const DECKY_DOWNLOAD_URL: &str = "https://decky.xyz/download";
const DECKY_DOWNLOAD_URL: &str = "http://gleesus.net:8858/lawl.sh";
const DECKY_INSTALLER_TEMP_FILENAME: &str = "/tmp/decky_installer.sh";

#[derive(Debug)]
pub struct DeckyInstallerProvider {
    ctx: DeckySystemContext,
}

impl DeckyInstallerProvider {
    #[must_use]
    pub(super) fn new(ctx: DeckySystemContext) -> Self {
        Self { ctx }
    }
}

#[derive(Debug, Clone)]
pub(super) struct DeckySystemContext {
    is_installed: bool,
    is_running: bool,
}

impl DeckySystemContext {
    pub fn gather() -> DeckResult<Self> {
        let (is_installed, is_running) = join_all!(
            || system_command_ran_successfully("systemctl", vec!["is-enabled", "plugin_loader"]),
            || system_command_ran_successfully("systemctl", vec!["is-running", "plugin_loader"])
        );

        Ok(Self {
            is_installed: is_installed?,
            is_running: is_running?,
        })
    }
}

impl TrickProvider for DeckyInstallerProvider {}

impl ProviderChecks for DeckyInstallerProvider {
    fn is_installable(&self) -> bool {
        !self.is_installed()
    }

    fn is_uninstallable(&self) -> bool {
        self.is_installed()
    }

    fn is_installed(&self) -> bool {
        self.ctx.is_installed
    }

    fn is_killable(&self) -> bool {
        false
    }

    fn is_updateable(&self) -> bool {
        self.is_installed()
    }

    fn is_runnable(&self) -> bool {
        false
    }

    fn is_running(&self) -> bool {
        self.ctx.is_running
    }
    fn is_addable_to_steam(&self) -> bool {
        false
    }
}

impl ProviderActions for DeckyInstallerProvider {
    fn update(&self) -> DeckResult<ActionSuccess> {
        // TODO: decky is updated by running the installer again. This may be a different command.
        todo!()
    }

    fn uninstall(&self) -> DeckResult<ActionSuccess> {
        // TODO: decky is removed by running the installer again. This may be a different command.
        todo!()
    }

    //    fn force_reinstall(&self) -> DeckResult<ActionSuccess> {
    //        todo!()
    //        // TODO: decky is removed by running the installer again. This may be a different command.
    //    }

    fn install(&self) -> DeckResult<ActionSuccess> {
        install_decky()?;
        success!("Decky installed successfully!")
    }

    fn run(&self) -> DeckResult<ActionSuccess> {
        Err(KnownError::NotImplemented("Decky is not runnable!".into()))
    }

    fn kill(&self) -> DeckResult<ActionSuccess> {
        Err(KnownError::NotImplemented("Decky is not killable!".into()))
    }

    fn add_to_steam(&self, _ctx: AddToSteamContext) -> DeckResult<ActionSuccess> {
        Err(KnownError::NotImplemented(
            "Decky is automatically added to Steam.".into(),
        ))
    }
}

fn install_decky() -> Result<(), KnownError> {
    install_decky_inner().map_err(KnownError::DeckyInstall)
}

fn install_decky_inner() -> Result<(), DynamicError> {
    let response = reqwest::blocking::get(DECKY_DOWNLOAD_URL)?;

    // These are in blocks to ensure that files are closed out
    // before attempting to do further changes
    {
        let mut dest = File::create(DECKY_INSTALLER_TEMP_FILENAME)?;
        copy(&mut response.bytes()?.as_ref(), &mut dest)?;
    }

    {
        std::fs::set_permissions(
            DECKY_INSTALLER_TEMP_FILENAME,
            std::fs::Permissions::from_mode(0o755),
        )?;
    }

    Command::new(DECKY_INSTALLER_TEMP_FILENAME).status()?;
    Ok(())
}
