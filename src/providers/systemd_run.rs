use crate::prelude::*;
use crate::system_command_runners::{SysCommandResultChecker, SysCommandRunner};
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;

#[derive(Debug)]
pub struct SystemdRunProvider {
    pub trick_id: TrickID,
    pub ctx: SpecificExecutionContext,
    pub is_unit_running: bool,
    pub systemd_run_data: SystemdRun,
}

// systemd-run --collect --working-directory=

impl SystemdRunProvider {
    pub(crate) fn new(
        trick_id: TrickID,
        ctx: SpecificExecutionContext,
        is_unit_running: bool,
        systemd_run_data: SystemdRun,
    ) -> Self {
        Self {
            trick_id,
            ctx,
            is_unit_running,
            systemd_run_data,
        }
    }
}

impl TrickProvider for SystemdRunProvider {}

impl ProviderChecks for SystemdRunProvider {
    fn get_execution_context(&self) -> &SpecificExecutionContext {
        &self.ctx
    }

    // These are meant to be simple system commands which are always known to be installed in
    // SteamOS. You can gather `which` data in FullSystemContext, if this becomes necessary.
    fn is_installable(&self) -> bool {
        false
    }
    fn is_uninstallable(&self) -> bool {
        false
    }
    fn is_installed(&self) -> bool {
        true
    }
    fn is_runnable(&self) -> bool {
        true
    }
    fn is_running(&self) -> bool {
        self.is_unit_running
    }
    fn is_killable(&self) -> bool {
        self.is_running()
    }
    fn is_updateable(&self) -> bool {
        false
    }
    fn is_addable_to_steam(&self) -> bool {
        self.is_installed()
    }
}

impl ProviderActions for SystemdRunProvider {
    // TODO: generalize these to be default implementations?
    fn uninstall(&self) -> DeckResult<ActionSuccess> {
        not_possible("Simple commands cannot be uninstalled!")
    }

    fn install(&self) -> DeckResult<ActionSuccess> {
        not_possible("Simple commands cannot be installed!")
    }

    fn run(&self) -> DeckResult<ActionSuccess> {
        let args = self.systemd_run_data.get_as_args();

        self.ctx
            .sys_command("/usr/bin/systemd-run", args)
            .enable_live_logging()
            //.env(PID_ENV_STRING, &self.trick_id)
            .run()?
            .as_success()
    }

    fn kill(&self) -> DeckResult<ActionSuccess> {
        self.ctx
            .sys_command(
                "/usr/bin/systemctl",
                ["stop", self.systemd_run_data.unit_id.as_ref()],
            )
            .enable_live_logging()
            .run()?
            .as_success()
    }

    fn update(&self) -> DeckResult<ActionSuccess> {
        not_possible("Simple commands cannot be updated!")
    }

    fn add_to_steam(&self) -> DeckResult<ActionSuccess> {
        add_to_steam(&AddToSteamTarget::Specific(
            TrickAddToSteamContext::try_from(&self.ctx.trick)?,
        ))
    }
}

impl GeneralProvider for SystemdRun {
    fn update_all(&self) -> DeckResult<ActionSuccess> {
        not_implemented("Simple commands cannot be updated!")
    }
}

#[derive(Debug, Default, Clone)]
pub struct SystemdRunUnitsContext {
    pub(crate) running_unit_ids: Vec<String>,
}

impl SystemdRunUnitsContext {
    pub fn gather_with(ctx: &impl ExecCtx, tricks_loader: &TricksLoader) -> DeckResult<Self> {
        // Find all of our detached commands
        let all_unit_ids: Vec<String> = tricks_loader
            .get_all_tricks()
            .filter_map(|t| match &t.1.provider_config {
                ProviderConfig::SystemdRun(systemd_run) => Some(systemd_run.unit_id.clone()),
                _ => None,
            })
            .collect();

        // Check with systemd if any of them are running
        let running_unit_ids = all_unit_ids
            .into_par_iter()
            .filter(|id| {
                ctx.sys_command("/usr/bin/systemctl", ["is-running", id.as_ref()])
                    .run()
                    .is_ok_and(|res| res.ran_successfully())
            })
            .map(|id| id)
            .collect();

        Ok(Self { running_unit_ids })
    }
}
