use super::flatpak_helpers::{
    get_installed_flatpak_applications, get_running_flatpak_applications,
};
use crate::prelude::*;
use crate::run_system_command::system_command_output as sys_output;

type FlatpakID = String;

#[derive(Debug)]
pub(crate) struct FlatpakProvider {
    id: FlatpakID,
    ctx: FlatpakSystemContext,
}

impl FlatpakProvider {
    pub(crate) fn new(flatpak: &Flatpak, ctx: FlatpakSystemContext) -> Self {
        let id = flatpak.id.clone();
        Self { id, ctx }
    }
}

#[derive(Debug, Clone)]
pub(crate) struct FlatpakSystemContext {
    running: Vec<FlatpakID>,
    installed: Vec<FlatpakID>,
}

impl FlatpakSystemContext {
    // TODO: parallelize this
    pub(crate) fn gather() -> DeckResult<Self> {
        let (running, installed) = join_all!(
            get_running_flatpak_applications,
            get_installed_flatpak_applications
        );

        Ok(Self {
            running: running?,
            installed: installed?,
        })
    }
}

impl TrickProvider for FlatpakProvider {}

impl FlatpakProvider {
    fn is_pkg_installed(&self) -> bool {
        self.ctx.installed.contains(&self.id)
    }

    fn is_pkg_running(&self) -> bool {
        self.ctx.running.contains(&self.id)
    }
}

#[cfg(not(test))]
impl FlatpakProvider {
    // NOTE: Can handle/track child pid status here, but
    // `flatpak ps` gives us that easily and authoritatively.
    fn flatpak_run(&self) -> DeckResult<ActionSuccess> {
        sys_output("flatpak", vec!["run", &self.id])
    }

    fn flatpak_install(&self) -> DeckResult<ActionSuccess> {
        sys_output("flatpak", vec!["install", "-y", &self.id])
    }

    fn flatpak_uninstall(&self) -> DeckResult<ActionSuccess> {
        sys_output("flatpak", vec!["uninstall", "-y", &self.id])
    }

    fn flatpak_kill(&self) -> DeckResult<ActionSuccess> {
        sys_output("flatpak", vec!["kill", &self.id])
    }

    fn flatpak_update(&self) -> DeckResult<ActionSuccess> {
        sys_output("flatpak", vec!["update", &self.id])
    }
}

// TODO: remove all test blocks for checks

impl ProviderChecks for FlatpakProvider {
    fn is_installable(&self) -> bool {
        !self.is_installed()
    }

    fn is_uninstallable(&self) -> bool {
        self.is_installed()
    }

    fn is_installed(&self) -> bool {
        self.is_pkg_installed()
    }

    fn is_runnable(&self) -> bool {
        self.is_installed()
    }

    fn is_running(&self) -> bool {
        self.is_pkg_running()
    }

    fn is_killable(&self) -> bool {
        self.is_running()
    }

    fn is_updateable(&self) -> bool {
        self.is_installed()
    }

    fn is_addable_to_steam(&self) -> bool {
        self.is_installed()
    }
}

impl ProviderActions for FlatpakProvider {
    fn uninstall(&self) -> DeckResult<ActionSuccess> {
        self.flatpak_uninstall()?;
        success!("\"{}\" uninstalled successfully.", self.id)
    }

    fn install(&self) -> DeckResult<ActionSuccess> {
        self.flatpak_install()?;
        success!("\"{}\" installed successfully.", self.id)
    }

    fn run(&self) -> DeckResult<ActionSuccess> {
        // TODO: check return status and return Err if appropriate
        self.flatpak_run()
    }

    fn kill(&self) -> DeckResult<ActionSuccess> {
        self.flatpak_kill()?;
        success!()
    }

    fn update(&self) -> DeckResult<ActionSuccess> {
        self.flatpak_update()?;
        success!()
    }

    fn add_to_steam(&self, _ctx: AddToSteamContext) -> DeckResult<ActionSuccess> {
        not_implemented("Add to steam is not yet implemented for flatpak!")
    }
}

#[derive(Debug)]
pub(crate) struct FlatpakGeneralProvider;
impl GeneralProvider for FlatpakGeneralProvider {
    fn update_all(&self) -> DeckResult<ActionSuccess> {
        // IMPORTANT: for flatpak update -y, you MUST run it twice to remove unused runtimes.
        sys_output("flatpak", vec!["update", "-y"])?;
        sys_output("flatpak", vec!["update", "-y"])?;

        success!("Flatpak update run successfully!")
    }
}

#[cfg(test)]
impl FlatpakProvider {
    #[allow(clippy::unused_self)]
    fn flatpak_run(&self) -> DeckResult<ActionSuccess> {
        success!("flatpak run success in test")
    }

    #[allow(clippy::unused_self)]
    fn flatpak_kill(&self) -> DeckResult<ActionSuccess> {
        success!("flatpak kill success in test")
    }

    #[allow(clippy::unused_self)]
    fn flatpak_update(&self) -> DeckResult<ActionSuccess> {
        success!("flatpak update success in test")
    }

    #[allow(clippy::unused_self)]
    fn flatpak_install(&self) -> DeckResult<ActionSuccess> {
        success!("flatpak install success in test")
    }

    #[allow(clippy::unused_self)]
    fn flatpak_uninstall(&self) -> DeckResult<ActionSuccess> {
        success!("flatpak uninstall success in test")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    impl Flatpak {
        pub(crate) fn new<S: Into<String>>(id: S) -> Self {
            Flatpak { id: id.into() }
        }
    }

    fn get_system_context() -> FlatpakSystemContext {
        FlatpakSystemContext {
            installed: vec!["installed_package".into(), "installed_package2".into()],
            running: vec!["running_package".into(), "running_package2".into()],
        }
    }

    fn fprov(id: &str) -> FlatpakProvider {
        let ctx = get_system_context();
        FlatpakProvider::new(&Flatpak::new(id), ctx)
    }

    #[allow(clippy::unnecessary_wraps)]
    #[test]
    fn test_new_flatpak_provider() -> DeckResult<()> {
        let provider = fprov("test_pkg");
        assert_eq!(provider.id, "test_pkg");
        Ok(())
    }

    #[test]
    fn test_is_pkg_installed_true() {
        let provider = fprov("installed_package");
        assert!(provider.is_installed());
        let provider = fprov("package_not_installed");
        assert!(!provider.is_installed());
    }

    #[test]
    fn test_installable() {
        let provider = fprov("RANDOM_NAME_FROM_NOWHERE");
        assert!(provider.is_installable());
    }

    #[test]
    fn test_updateable() {
        let provider = fprov("installed_package");
        assert!(provider.is_updateable());
        let provider = fprov("test_pkg_not_installed");
        assert!(!provider.is_updateable());
    }

    #[test]
    fn test_is_pkg_running() {
        let provider = fprov("running_package");
        assert!(provider.is_running());
        let provider = fprov("not_running_package");
        assert!(!provider.is_running());
    }
}
