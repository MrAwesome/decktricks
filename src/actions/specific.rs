use crate::prelude::*;

#[derive(Debug)]
pub(crate) enum SpecificAction {
    Run {
        id: String,
    },
    Install {
        id: String,
    },
    Kill {
        id: String,
    },
    Uninstall {
        id: String,
    },
    AddToSteam {
        name: Option<String>,
        id: String,
    },
    Info {
        id: String,
    },
}

impl SpecificAction {
    pub(crate) fn id(&self) -> &str {
        match self {
            Self::Run { id }
            | Self::Kill { id }
            | Self::Info { id }
            | Self::Install { id }
            | Self::AddToSteam { id, .. }
            | Self::Uninstall { id } => id,
        }
    }

    pub(crate) fn run(
        &self, 
        config: &TricksConfig,
    ) -> Result<ActionSuccess, KnownError> {
        let trick_id = self.id();
        let trick = config.get_trick(trick_id.as_ref())?;
        let provider = DynProvider::try_from(trick)?;

        // TODO: implement
        //let possible = provider.possible();

        //if !possible.contains(&action.id()) {
            // XXX
            //return Err(format!("Action \"{:?}\", supported actions: ", action.try_into()));
            //unimplemented!()
        //}
        if provider.can(self)? {
            match self {
                Self::Install { .. } => provider.install(),
                Self::Run { .. } => provider.run(),
                Self::Uninstall { .. } => provider.uninstall(),
                Self::AddToSteam { 
                    name,
                    .. } => provider.add_to_steam(AddToSteamContext { _name: name.clone() }),
                Self::Kill { .. } => provider.kill(),

                Self::Info { .. } => {
                    success!("{:?}", provider)
                },
            }
        } else {
            todo!("make this error handling more specific by having each action do its own check, or...?")
        }
    }
}
