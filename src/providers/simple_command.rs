use crate::prelude::*;

impl ProviderChecks for SimpleCommand {
    fn is_installable(
        &self,
    ) -> Result<bool, KnownError> {
        // These are meant to be simple system commands which are always known to be installed
        Ok(false)
    }

    fn is_installed(
        &self,
    ) -> Result<bool, KnownError> {
        Ok(false)
    }
    fn is_runnable(&self) -> Result<bool, KnownError> {
        Ok(true)
    }
    fn is_running(&self) -> Result<bool, KnownError> {
        // NOTE: for now, we aren't going to implement this until it's needed
        Ok(false)
    }
    fn is_addable_to_steam(
        &self,
    ) -> Result<bool, KnownError> {
        // For now, we'll assume these aren't commands people will want to run through Steam
        Ok(false)
    }
}

impl Installed for Provider<SimpleCommandProviderData, IsInstalled> {
    fn update(&self) -> Result<ActionSuccess, DynamicError> {
        unimplemented!()
    }

    fn remove(&self) -> Result<ActionSuccess, DynamicError> {
        unimplemented!()
    }

    fn force_reinstall(&self) -> Result<ActionSuccess, DynamicError> {
        unimplemented!()
    }
}

impl Installable for Provider<SimpleCommandProviderData, IsInstallable> {
    fn install(&self) -> Result<ActionSuccess, DynamicError> {
        unimplemented!()
    }
}

impl Runnable for Provider<SimpleCommandProviderData, IsRunnable> {
    fn run(&self) -> Result<ActionSuccess, DynamicError> {
        unimplemented!()
    }
}

impl Running for Provider<SimpleCommandProviderData, IsRunning> {
    fn kill(&self) -> Result<ActionSuccess, DynamicError> {
        unimplemented!()
    }
}

impl AddableToSteam for Provider<SimpleCommandProviderData, IsAddableToSteam> {
    fn add_to_steam(&self) -> Result<ActionSuccess, DynamicError> {
        unimplemented!()
    }
}
