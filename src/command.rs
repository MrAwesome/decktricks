use crate::gui::GuiType;
use crate::prelude::TypedAction;
pub use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "decktricks")]
pub struct DecktricksCommand {
    // The actual command to be executed, such as "run" or "install".
    // To seed an Executor for repeated use, use "actions".
    #[clap(subcommand)]
    pub action: Action,

    // Whether or not to gather system context before running an Action.
    // Most useful from a GUI, where we should be relatively
    // sure that a SpecificAction is valid/possible.
    #[clap(short, long)]
    pub gather_context_on_specific_actions: bool,

    // Anything below this line is global context for Actions: //
    // ------------------------------------------------------- //

    // Filename of an override config
    #[clap(short, long)]
    pub config: Option<String>,
    // Currently unused since pretty_env_logger uses env variables.
    // #[clap(short, long)]
    // pub debug: bool,
}

impl DecktricksCommand {
    #[must_use]
    pub fn new(action: Action) -> Self {
        Self {
            action,
            gather_context_on_specific_actions: false,
            config: None,
        }
    }
}

// * "Run"
// * "Install"
// * "Uninstall"
// * "Update"
// * "Add To Steam"
//
// * "More Actions"
// * "Kill"
// * "Force Reinstall"
// * "Info"
// * "Configure"/"Settings"

// This is the canonical list of actions we can take on tricks.
// It is directly parsed by clap for the commandline, and should be
// the entry point for any GUI or downstream lib.
#[derive(Debug, Clone, Subcommand)]
pub enum Action {
    Run {
        id: String,
    },
    Install {
        id: String,
    },
    Kill {
        id: String,
    },
    #[clap(alias = "remove")]
    Uninstall {
        id: String,
    },
    AddToSteam {
        #[clap(long)]
        name: Option<String>,
        id: String,
    },
    Info {
        id: String,
    },

    // Note that "update" can work both globally or for a specific id.
    Update {
        id: Option<String>,
    },

    // Items below do not take trick ids, and function differently.
    List {
        #[clap(long)]
        installed: bool,
    },
    Actions {
        id: Option<String>,
        #[clap(long)]
        json: bool,
    },
    Gui {
        #[clap(subcommand)]
        gui: GuiType,
    },
    GetConfig,


    // Internal use:
    #[clap(hide = true)]
    GetActionDisplayNameMapping,
    #[clap(hide = true)]
    GatherContext,
    #[clap(hide = true)]
    RunSystemCommand { command: String, args: Option<Vec<String>> },
}

impl Action {
    #[must_use]
    pub fn does_not_need_system_context(&self, gather_context_on_specific_actions: bool) -> bool {
        matches!(self, Self::Info { .. } | Self::GetConfig)
            || (gather_context_on_specific_actions
                && matches!(TypedAction::from(self), TypedAction::Specific(_)))
    }
}
