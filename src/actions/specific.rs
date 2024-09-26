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

        match self {
            Self::Install { .. } => provider.install(),
            Self::Run { .. } => provider.run(),
            Self::Uninstall { .. } => provider.uninstall(),
            Self::AddToSteam { 
                //name, 
                .. } => provider.add_to_steam(),
            Self::Kill { .. } => provider.kill(),

            // TODO: this is provider-agnostic, just run code here and return ActionSuccess with the
            // string
            Self::Info { .. } => {
                success!("{:?}", provider)
            },
        }
    }
}
