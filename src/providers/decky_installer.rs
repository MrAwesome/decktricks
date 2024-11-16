use crate::prelude::*;
use crate::utils::run_remote_script;

// TODO: detect if on steam deck or not, and *do not mark as installable if not on steam deck*

//const DECKY_DOWNLOAD_URL: &str = "https://decky.xyz/download";
const DECKY_DOWNLOAD_URL: &str = "http://gleesus.net:8858/lawl.sh";
const DECKY_INSTALLER_TEMP_FILENAME: &str = "/tmp/decky_installer.sh";

#[derive(Debug)]
pub struct DeckyInstallerProvider {
    runner: RunnerRc,
    ctx: DeckySystemContext,
}

impl DeckyInstallerProvider {
    #[must_use]
    pub(super) fn new(runner: RunnerRc, ctx: DeckySystemContext) -> Self {
        Self { runner, ctx }
    }
}

#[derive(Debug, Clone)]
pub struct DeckySystemContext {
    pub is_installed: bool,
    pub is_running: bool,
}

impl DeckySystemContext {
    pub fn gather_with(runner: &RunnerRc) -> DeckResult<Self> {
        let (is_installed, is_running) = join_all!(
            || SysCommand::new("systemctl", vec!["is-enabled", "plugin_loader"]).run_with(runner),
            || SysCommand::new("systemctl", vec!["is-active", "plugin_loader"]).run_with(runner)
        );

        Ok(Self {
            is_installed: is_installed?.ran_successfully(),
            is_running: is_running?.ran_successfully(),
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
        not_implemented("Decky updates are not implemented yet!")
    }

    fn uninstall(&self) -> DeckResult<ActionSuccess> {
        // TODO: decky is removed by running the installer again. This may be a different command.
        not_implemented("Decky uninstall is not implemented yet!")
    }

    fn install(&self) -> DeckResult<ActionSuccess> {
        run_remote_script(&self.runner, DECKY_DOWNLOAD_URL, DECKY_INSTALLER_TEMP_FILENAME)?;
        success!("Decky installed successfully!")
    }

    fn run(&self) -> DeckResult<ActionSuccess> {
        not_possible("Decky is not runnable!")
    }

    fn kill(&self) -> DeckResult<ActionSuccess> {
        not_possible("Decky is not killable!")
    }

    fn add_to_steam(&self, _ctx: AddToSteamContext) -> DeckResult<ActionSuccess> {
        not_possible("Decky is automatically added to Steam.")
    }
}

#[derive(Debug)]
pub(crate) struct DeckyInstallerGeneralProvider;
impl GeneralProvider for DeckyInstallerGeneralProvider {
    fn update_all(&self) -> DeckResult<ActionSuccess> {
        // TODO: run the decky update command here
        not_implemented("Decky update is not implemented yet!")
    }
}
