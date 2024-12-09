use crate::prelude::*;
use crate::utils::run_remote_script;

// TODO: detect if on steam deck or not, and *do not mark as installable if not on steam deck*

const DECKY_DOWNLOAD_URL: &str = "https://github.com/SteamDeckHomebrew/decky-installer/releases/latest/download/user_install_script.sh";
const DECKY_INSTALLER_TEMP_FILENAME: &str = "/tmp/decky_installer.sh";

#[derive(Debug)]
pub struct DeckyInstallerProvider {
    ctx: SpecificExecutionContext,
    decky_ctx: DeckySystemContext,
}

impl DeckyInstallerProvider {
    #[must_use]
    pub(super) fn new(
        ctx: SpecificExecutionContext,
        decky_ctx: DeckySystemContext,
    ) -> Self {
        Self {
            ctx,
            decky_ctx,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct DeckySystemContext {
    pub is_installed: bool,
    pub is_running: bool,
}

impl DeckySystemContext {
    pub(crate) fn gather_with(ctx: &impl ExecCtx) -> DeckResult<Self> {
        let (is_installed, is_running) = join_all!(
            || SysCommand::new(ctx, "/usr/bin/systemctl", ["is-enabled", "plugin_loader"])
                .run(),
            || SysCommand::new(ctx, "/usr/bin/systemctl", ["is-active", "plugin_loader"])
                .run()
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
        self.decky_ctx.is_installed
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
        self.decky_ctx.is_running
    }
    fn is_addable_to_steam(&self) -> bool {
        false
    }
}

impl ProviderActions for DeckyInstallerProvider {
    fn update(&self) -> DeckResult<ActionSuccess> {
        // TODO: decky is updated by running the installer again. This may be a different command.
        self.install()
    }

    fn uninstall(&self) -> DeckResult<ActionSuccess> {
        // TODO: decky is removed by running the installer again. This may be a different command.
        self.install()
    }

    fn install(&self) -> DeckResult<ActionSuccess> {
        let _ = SysCommand::new(&self.ctx, "xhost", vec!["+"]).run();
        run_remote_script(
            &self.ctx,
            DECKY_DOWNLOAD_URL,
            DECKY_INSTALLER_TEMP_FILENAME,
        )?;
        success!("Decky installed successfully!")
    }

    fn run(&self) -> DeckResult<ActionSuccess> {
        not_possible("Decky is not runnable!")
    }

    fn kill(&self) -> DeckResult<ActionSuccess> {
        not_possible("Decky is not killable!")
    }

    fn add_to_steam(&self) -> DeckResult<ActionSuccess> {
        not_possible("Decky is automatically added to Steam.")
    }
}

#[derive(Debug)]
pub(crate) struct DeckyInstallerGeneralProvider;
impl GeneralProvider for DeckyInstallerGeneralProvider {
    fn update_all(&self) -> DeckResult<ActionSuccess> {
        // TODO: run the decky update command here (not the installer directly)
        success!()
    }
}
