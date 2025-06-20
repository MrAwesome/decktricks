use crate::utils::kill_pids;
use crate::utils::pgrep;
use std::sync::LazyLock;

use crate::prelude::*;
use crate::utils::exists_and_executable;
use crate::utils::fetch_and_prep_remote_executable;
use crate::utils::get_homedir;

const GEFORCE_INSTALLER_DOWNLOAD_URL: &str =
    "https://international.download.nvidia.com/GFNLinux/GeForceNOWSetup.bin";
const GEFORCE_INSTALLER_TEMP_FILENAME: &str = "/tmp/GeForceNOWSetup.bin";

pub(crate) static GEFORCE_LOCAL_EXECUTABLE: LazyLock<String> = LazyLock::new(|| {
    format!(
        "{}/{}",
        get_homedir(),
        ".local/share/applications/NVIDIA GeForce NOW"
    )
});

static GEFORCE_PGREP_STRING: LazyLock<String> = LazyLock::new(|| {
    format!("bash {}", GEFORCE_LOCAL_EXECUTABLE.as_str())
});

#[derive(Debug)]
pub struct GeForceInstallerProvider {
    ctx: SpecificExecutionContext,
    geforce_ctx: GeForceSystemContext,
}

impl GeForceInstallerProvider {
    #[must_use]
    pub(super) fn new(ctx: SpecificExecutionContext, geforce_ctx: GeForceSystemContext) -> Self {
        Self { ctx, geforce_ctx }
    }
}

#[derive(Debug, Clone, Default)]
pub struct GeForceSystemContext {
    pub is_installed: bool,
    pub running_pids: Vec<String>,
}

impl GeForceSystemContext {
    pub(crate) fn gather_with(ctx: &impl ExecCtx) -> Self {
        let (is_installed, running_pids) = join_all!(
            || exists_and_executable(ctx, GEFORCE_LOCAL_EXECUTABLE.as_str()),
            || pgrep(ctx, GEFORCE_PGREP_STRING.as_str()).unwrap_or_default()
        );

        Self {
            is_installed,
            running_pids,
        }
    }
}

impl TrickProvider for GeForceInstallerProvider {}

impl ProviderChecks for GeForceInstallerProvider {
    fn get_execution_context(&self) -> &SpecificExecutionContext {
        &self.ctx
    }

    fn is_installable(&self) -> bool {
        !self.is_installed()
    }

    fn is_uninstallable(&self) -> bool {
        self.is_installed()
    }

    fn is_installed(&self) -> bool {
        self.geforce_ctx.is_installed
    }

    fn is_killable(&self) -> bool {
        self.is_running()
    }

    fn is_updateable(&self) -> bool {
        self.is_installed()
    }

    fn is_runnable(&self) -> bool {
        self.is_installed()
    }

    fn is_running(&self) -> bool {
        !self.geforce_ctx.running_pids.is_empty()
    }
    fn is_addable_to_steam(&self) -> bool {
        self.is_installed()
    }
}

impl ProviderActions for GeForceInstallerProvider {
    fn update(&self) -> DeckResult<ActionSuccess> {
        not_possible("GeForce NOW updates itself automatically on run.")
    }

    fn uninstall(&self) -> DeckResult<ActionSuccess> {
        // TODO: geforce is removed by running the installer again. This may be a different command.
        self.install()
    }

    fn install(&self) -> DeckResult<ActionSuccess> {
        // Needed for geforce?
        //let _ = &self.ctx.sys_command("xhost", vec!["+"]).run();
        fetch_and_prep_remote_executable(
            &self.ctx,
            GEFORCE_INSTALLER_DOWNLOAD_URL,
            GEFORCE_INSTALLER_TEMP_FILENAME,
        )?
        .env(INSTALLING_ENV_STRING, self.ctx.trick.id.as_ref())
        .run()?;
        success!("GeForce NOW installed successfully!")
    }

    fn run(&self) -> DeckResult<ActionSuccess> {
        self.ctx
            .sys_command_no_args(GEFORCE_LOCAL_EXECUTABLE.as_str())
            .enable_live_logging()
            .run()?
            .as_success()
    }

    fn kill(&self) -> DeckResult<ActionSuccess> {
        kill_pids(&self.ctx, &self.geforce_ctx.running_pids)
    }

    fn add_to_steam(&self) -> DeckResult<ActionSuccess> {
        not_possible("GeForce NOW is automatically added to Steam.")
    }
}
