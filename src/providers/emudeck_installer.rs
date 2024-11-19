use crate::utils::get_running_pids_exact;
use crate::prelude::*;
use crate::utils::kill_pids;
use crate::utils::{exists_and_executable, get_homedir, run_remote_script};

// TODO: determine differences between "running" (games being played) and "running the installer"
// TODO: "installed" is $HOME/Applications/EmuDeck.AppImage

const EMUDECK_DOWNLOAD_URL: &str =
    "https://raw.githubusercontent.com/dragoonDorise/EmuDeck/main/install.sh";
const EMUDECK_INSTALLER_TEMP_FILENAME: &str = "/tmp/emudeck_installer.sh";

const EMUDECK_BINARY_NAME: &str = "EmuDeck.AppImage";

fn get_emudeck_binary_path() -> String {
    format!("{}/Applications/{}", get_homedir(), EMUDECK_BINARY_NAME)
}

#[derive(Debug)]
pub struct EmuDeckInstallerProvider {
    runner: RunnerRc,
    ctx: EmuDeckSystemContext,
}

impl EmuDeckInstallerProvider {
    #[must_use]
    pub(super) fn new(
        runner: RunnerRc,
        ctx: EmuDeckSystemContext,
    ) -> Self {
        Self {
            runner,
            ctx,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct EmuDeckSystemContext {
    is_installed: bool,
    running_pids: Vec<String>
}

impl EmuDeckSystemContext {
    #[allow(clippy::unnecessary_wraps)]
    /// # Errors
    ///
    /// Returns errors relating to running pgrep and checking file existence/permissions.
    pub fn gather_with(runner: &RunnerRc) -> DeckResult<Self> {
        let (is_installed, running_main_pids, running_supplementary_pids) = 
            join_all!(
                || exists_and_executable(runner, &get_emudeck_binary_path()),
                || get_running_pids_exact(runner, EMUDECK_BINARY_NAME).unwrap_or_default(),
                || get_running_pids_exact(runner, "emudeck").unwrap_or_default()
                );

        let running_pids = [running_main_pids, running_supplementary_pids].concat();

        Ok(Self { is_installed, running_pids })
    }
}

impl TrickProvider for EmuDeckInstallerProvider {}

impl ProviderChecks for EmuDeckInstallerProvider {
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
        self.is_running()
    }

    fn is_updateable(&self) -> bool {
        // TODO: check
        self.is_installed()
    }

    fn is_runnable(&self) -> bool {
        self.is_installed()
    }

    fn is_running(&self) -> bool {
        !self.ctx.running_pids.is_empty()
    }
    fn is_addable_to_steam(&self) -> bool {
        true
    }
}

impl ProviderActions for EmuDeckInstallerProvider {
    fn update(&self) -> DeckResult<ActionSuccess> {
        // TODO: check
        not_implemented("EmuDeck updates are not implemented yet!")
    }

    fn uninstall(&self) -> DeckResult<ActionSuccess> {
        // TODO: check
        not_implemented("EmuDeck uninstall is not implemented yet!")
    }

    fn install(&self) -> DeckResult<ActionSuccess> {
        run_remote_script(
            &self.runner,
            EMUDECK_DOWNLOAD_URL,
            EMUDECK_INSTALLER_TEMP_FILENAME,
        )?;
        success!("EmuDeck installer installed successfully! Run now to fully install EmuDeck.")
    }

    fn run(&self) -> DeckResult<ActionSuccess> {
        SysCommand::new(get_emudeck_binary_path(), vec![])
            .run_with(&self.runner)?
            .as_success()
    }

    fn kill(&self) -> DeckResult<ActionSuccess> {
        kill_pids(&self.runner, &self.ctx.running_pids)
    }

    fn add_to_steam(&self, _ctx: AddToSteamContext) -> DeckResult<ActionSuccess> {
        // TODO: check
        not_possible("EmuDeck is automatically added to Steam.")
    }
}

#[derive(Debug)]
pub(crate) struct _EmuDeckInstallerGeneralProvider;
impl GeneralProvider for _EmuDeckInstallerGeneralProvider {
    fn update_all(&self) -> DeckResult<ActionSuccess> {
        // TODO: run the emudeck update command here
        // TODO: check
        not_implemented("EmuDeck update is not implemented yet!")
    }
}
