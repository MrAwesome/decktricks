use crate::{enum_with_all_variants, prelude::*};
use serde::Serialize;
use std::collections::HashMap;
use std::fmt::Display;

#[derive(Debug, Clone)]
pub enum SpecificAction {
    Run { id: String },
    Install { id: String },
    Kill { id: String },
    Uninstall { id: String },
    AddToSteam { id: String },
    Update { id: String },
    // NOTE: Info does not actually require a provider or anything else,
    // it just reads from the config.
    Info { id: String },
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
        Install,
        Run,
        AddToSteam,
        Update,
        Uninstall,
        Kill,

        // Info should always be last:
        Info,
        // Nothing here, please!
    }
);

// Needed for hot reloading for ActionButton in the Godot GUI
impl Default for SpecificActionID {
    fn default() -> Self {
        Self::Run
    }
}

impl SpecificActionID {
    #[must_use]
    pub fn get_display_name(&self, is_ongoing: bool, is_completed: bool) -> &'static str {
        match self {
            Self::Run => if is_ongoing { "Running" } else { "Run" },
            Self::AddToSteam => if is_completed { "Added" } else { "Add To Steam" },
            Self::Install => if is_ongoing { "Installing" } else { "Install" },
            Self::Uninstall => if is_ongoing { "Uninstalling" } else { "Uninstall" },
            Self::Update => if is_ongoing { "Updating" } else { "Update" },
            Self::Kill => "Kill",
            Self::Info => "Info",
        }
    }

    #[must_use]
    pub fn get_display_name_mapping() -> HashMap<String, &'static str> {
        let all_vars = SpecificActionID::all_variants();
        all_vars
            .into_iter()
            .map(|v| {
                let dname = v.get_display_name(false, false);
                (v.to_string(), dname)
            })
            .collect()
    }

    pub fn as_action(&self, trick_id: String) -> SpecificAction {
        let id = trick_id;
        match self {
            Self::Run => SpecificAction::Run { id },
            Self::Install => SpecificAction::Install { id },
            Self::Kill => SpecificAction::Kill { id },
            Self::Uninstall => SpecificAction::Uninstall { id },
            Self::AddToSteam => SpecificAction::AddToSteam { id },
            Self::Info => SpecificAction::Info { id },
            Self::Update => SpecificAction::Update { id },
        }
    }
}

impl Display for SpecificActionID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            // TODO: this may be too much trouble
            String::try_from(self).unwrap_or_else(|e| format!("{e}"))
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

impl From<&SpecificAction> for SpecificActionID {
    fn from(action: &SpecificAction) -> Self {
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
            | Self::AddToSteam { id }
            | Self::Update { id }
            | Self::Uninstall { id } => id,
        }
    }

    pub(crate) fn as_info(id: &impl ToString) -> Self {
        SpecificAction::Info { id: id.to_string() }
    }

    pub(crate) fn do_with(
        self,
        executor: &Executor,
        current_log_level: LogType,
    ) -> (Option<SpecificExecutionContext>, DeckResult<ActionSuccess>) {
        let config = executor.get_loaded_config();
        let trick_id = self.id();
        let maybe_trick = config.get_trick(trick_id.as_ref());

        match maybe_trick {
            Ok(trick) => {
                let ctx = executor.get_new_specific_execution_context(
                    current_log_level,
                    trick.clone(),
                    self.clone(),
                    // In the context of actually taking an action, we don't care if we're installing
                    // or added to Steam, since at the moment these are purely for cosmetic purposes
                    // TODO: code smell
                    false,
                    false,
                );

                let res = self.do_with_inner(&ctx, executor, trick);
                (Some(ctx), res)
            }
            Err(err) => (None, Err(err)),
        }

    }

    pub fn do_with_inner(
        self,
        ctx: &SpecificExecutionContext,
        executor: &Executor,
        trick: &Trick,
    ) -> DeckResult<ActionSuccess> {
        let (_loader, full_ctx, _runner) = executor.get_pieces();
        let provider = DynTrickProvider::new(&ctx, full_ctx);

        if provider.can(&self) {
            match self {
                Self::Install { .. } => provider.install(),
                Self::Run { .. } => provider.run(),
                Self::Uninstall { .. } => provider.uninstall(),
                Self::AddToSteam { .. } => provider.add_to_steam(),
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
                String::try_from(&SpecificActionID::from(&self))?,
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
