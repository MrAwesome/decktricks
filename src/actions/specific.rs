use crate::{enum_with_all_variants, prelude::*};
use serde::Serialize;

#[derive(Debug)]
pub(crate) enum SpecificAction {
    Run { id: String },
    Install { id: String },
    Kill { id: String },
    Uninstall { id: String },
    AddToSteam { name: Option<String>, id: String },
    Info { id: String },
    Update { id: String },
}

// TODO: Ensure these names are the same as elsewhere
// NOTE: These are serialized in kebab-case to match clap's commandline arg style
enum_with_all_variants!(
    #[derive(Debug, Clone, Serialize)]
    #[serde(rename_all = "kebab-case")]
    pub(crate) enum SpecificActionID {
        Run,
        Install,
        Kill,
        Uninstall,
        AddToSteam,
        Info,
        Update,
    }
);

impl From<SpecificAction> for SpecificActionID {
    fn from(action: SpecificAction) -> Self {
        match action {
            SpecificAction::Run { .. } => Self::Run,
            SpecificAction::Install { .. } => Self::Install,
            SpecificAction::Kill { .. } => Self::Kill,
            SpecificAction::Uninstall { .. } => Self::Uninstall,
            SpecificAction::AddToSteam { .. } => Self::AddToSteam,
            SpecificAction::Info { .. } => Self::Info,
            SpecificAction::Update { .. } => Self::Update,
        }
    }
}

impl SpecificAction {
    pub(crate) fn id(&self) -> &str {
        match self {
            Self::Run { id }
            | Self::Kill { id }
            | Self::Info { id }
            | Self::Install { id }
            | Self::AddToSteam { id, .. }
            | Self::Update { id }
            | Self::Uninstall { id } => id,
        }
    }

    pub(crate) fn run(&self, loader: &TricksLoader) -> Result<ActionSuccess, KnownError> {
        let trick_id = self.id();
        let trick = loader.get_trick(trick_id.as_ref())?;
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
                Self::AddToSteam { name, .. } => provider.add_to_steam(AddToSteamContext {
                    _name: name.clone(),
                }),
                Self::Kill { .. } => provider.kill(),
                Self::Update { .. } => provider.update(),

                Self::Info { .. } => {
                    success!(
                        "{}",
                        serde_json::to_string_pretty(trick)
                            .map_err(|e| KnownError::ConfigParsing(e))?
                    )
                }
            }
        } else {
            todo!("make this error handling more specific by having each action do its own check, or...?")
        }
    }
}
