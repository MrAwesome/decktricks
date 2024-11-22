use crate::{enum_with_all_variants, prelude::*};
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub(crate) enum SpecificAction {
    Run { id: String },
    Install { id: String },
    Kill { id: String },
    Uninstall { id: String },
    AddToSteam { name: Option<String>, id: String },
    // TODO: move Info to General, since it doesn't actually require a provider. Or should it?
    Info { id: String },
    Update { id: String },
}

// TODO: Ensure these names are the same as elsewhere
// NOTE: These are serialized in kebab-case to match clap's commandline arg style
//
// This proc macro gives us access to SpecificActionID.all_variants()
enum_with_all_variants!(
    #[derive(Debug, Clone, Serialize, Eq, PartialEq, Hash)]
    #[serde(rename_all = "kebab-case")]
    pub enum SpecificActionID {
        // XXX IMPORTANT! XXX
        //  The order here determines the order for how these actions
        //  are displayed in the GUI!
        // XXX IMPORTANT! XXX
        Run,
        Install,
        AddToSteam,
        Update,
        Uninstall,
        Kill,

        // Info should always be last:
        Info,
        // Nothing here, please!
    }
);

impl SpecificActionID {
    pub fn get_display_name(&self) -> &'static str {
        match self {
            Self::Run => "Run",
            Self::AddToSteam => "Add To Steam",
            Self::Install => "Install",
            Self::Uninstall => "Uninstall",
            Self::Update => "Update",
            Self::Kill => "Kill",
            Self::Info => "Info",
        }
    }

    pub fn get_display_name_mapping() -> HashMap<String, &'static str> {
        let all_vars = SpecificActionID::all_variants();
        all_vars
            .into_iter()
            .map(|v| {
                let dname = v.get_display_name();
                (v.to_string(), dname)
            })
            .collect()
    }
}

impl Display for SpecificActionID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            // TODO: this may be too much trouble
            String::try_from(self).unwrap_or("ERROR_SERIALIZING_SPECIFIC_ACTION_ID".into())
        )
    }
}

impl TryFrom<&SpecificActionID> for String {
    type Error = KnownError;
    fn try_from(id: &SpecificActionID) -> Result<Self, Self::Error> {
        Ok(serde_json::to_string(id)
            .map_err(KnownError::from)?
            .trim_matches(|c| c == '"')
            .into())
    }
}

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

    pub(crate) fn do_with(&self, executor: &Executor) -> DeckResult<ActionSuccess> {
        let (loader, full_ctx, runner) = executor.get_pieces();

        let trick_id = self.id();
        let trick = loader.get_trick(trick_id.as_ref())?;
        let provider = DynProvider::try_from((trick, full_ctx, runner))?;

        if provider.can(self) {
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
                        serde_json::to_string_pretty(trick).map_err(KnownError::from)?
                    )
                }
            }
        } else {
            // TODO: Make these more specific, maybe the checks can return a reason after all
            //       on the other hand, this is only for the CLI which doesn't really matter.
            //       Actions within the GUI will only show up if they pass this function.
            Err(KnownError::ActionGated(format!(
                "Action '{}' is not possible on trick '{}' right now. Is it installed/running? HINT: (Try 'actions')",
                String::try_from(&SpecificActionID::from(self.clone()))?,
                trick.id
            )))
        }
    }
}

#[test]
fn test_specific_id_display_map() {
    let m = SpecificActionID::get_display_name_mapping();
    assert_eq!(*m.get("info").unwrap(), "Info");
}
