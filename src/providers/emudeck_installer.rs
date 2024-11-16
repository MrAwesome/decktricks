use crate::prelude::*;
use crate::utils::{exists_and_executable, get_homedir, run_remote_script};

// TODO: determine differences between "running" (games being played) and "running the installer"
// TODO: "installed" is $HOME/Applications/EmuDeck.AppImage

const EMUDECK_DOWNLOAD_URL: &str =
    "https://raw.githubusercontent.com/dragoonDorise/EmuDeck/main/install.sh";
const EMUDECK_INSTALLER_TEMP_FILENAME: &str = "/tmp/emudeck_installer.sh";

const EMUDECK_BINARY_NAME: &str = "EmuDeck.AppImage";

#[derive(Debug)]
pub struct EmuDeckInstallerProvider {
    ctx: EmuDeckSystemContext,
    //runner: RunnerRc,
}

impl EmuDeckInstallerProvider {
    #[must_use]
    pub(super) fn new(ctx: EmuDeckSystemContext) -> Self {
        Self { ctx }
    }
}

#[derive(Debug, Clone)]
pub struct EmuDeckSystemContext {
    is_installed: bool,
    is_running: bool,
}

impl EmuDeckSystemContext {
    #[allow(clippy::unnecessary_wraps)]
    /// # Errors
    ///
    /// Returns errors relating to running pgrep and checking file existence/permissions.
    pub fn gather_with(runner: &RunnerRc) -> DeckResult<Self> {
        let (is_installed, is_running) = join_all!(
            || exists_and_executable(runner, &format!("{}/{}", get_homedir(), EMUDECK_BINARY_NAME)),
            || SysCommand::new("pgrep", vec!["-f", EMUDECK_BINARY_NAME])
                .run_with(runner)
                .map(|x| x.ran_successfully())
                .unwrap_or(false)
        );

        Ok(Self {
            is_installed,
            is_running,
        })
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
        // TODO: check
        false
    }

    fn is_running(&self) -> bool {
        self.ctx.is_running
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
        run_remote_script(EMUDECK_DOWNLOAD_URL, EMUDECK_INSTALLER_TEMP_FILENAME)
            .map_err(KnownError::EmuDeckInstall)?;
        success!("EmuDeck installer installed successfully! Run now to fully install EmuDeck.")
    }

    fn run(&self) -> DeckResult<ActionSuccess> {
        // TODO: check
        not_possible("EmuDeck is not runnable!")
    }

    fn kill(&self) -> DeckResult<ActionSuccess> {
        // TODO: check
        not_possible("EmuDeck is not killable!")
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
