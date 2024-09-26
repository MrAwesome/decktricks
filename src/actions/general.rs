use crate::prelude::*;
use crate::tricks_config::TricksConfig;

#[derive(Debug)]
pub(crate) enum GeneralAction {
    List { installed: bool },
}

impl GeneralAction {
    pub(crate) fn run(&self, config: &TricksConfig) -> Result<ActionSuccess, KnownError> {
        match self {
            Self::List { installed } => {
                let tricks = config.get_all_tricks();

                let tricks_names: Vec<&str> = match installed {
                    false => tricks
                        .map(|name_and_trick| name_and_trick.0.as_str())
                        .collect(),
                    true => tricks
                        .filter(|name_and_trick| {
                            let trick = name_and_trick.1;
                            let provider = DynProvider::try_from(trick);
                            provider.is_ok_and(|t| t.is_installed().is_ok())
                        })
                        .map(|name_and_trick| name_and_trick.0.as_str())
                        .collect(),
                };

                let tricks_newline_delineated = tricks_names.join("\n");
                success!(tricks_newline_delineated)
            }
        }
    }
}
