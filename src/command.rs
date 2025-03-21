use crate::steam::SteamSubcommand;
use clap::ValueEnum;
use serde::Serialize;
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

    // Will default to the current_log_level of the executor
    #[clap(short, long)]
    pub log_level: Option<LogType>,
}

impl DecktricksCommand {
    #[must_use]
    pub fn new(action: Action) -> Self {
        Self {
            action,
            gather_context_on_specific_actions: false,
            config: None,
            log_level: None,
        }
    }
}

#[derive(Debug, Clone, Copy, Serialize, ValueEnum, PartialEq, Eq, PartialOrd, Ord)]
pub enum LogType {
    Error = 0,
    Warn = 1,
    Log = 2,
    Info = 3,
    Debug = 4,
}

impl LogType {
    #[must_use]
    pub const fn get_prefix_for(self) -> &'static str {
        match self {
            LogType::Error => "[ERROR]",
            LogType::Warn => "[WARN]",
            LogType::Log => "[LOG]",
            LogType::Info => "[INFO]",
            LogType::Debug => "[DEBUG]",
        }
    }
}

impl From<u8> for LogType {
    fn from(log_level: u8) -> Self {
        match log_level {
            0 => Self::Error,
            1 => Self::Warn,
            2 => Self::Log,
            3 => Self::Info,
            4 => Self::Debug,
            _ => Self::Debug,
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
    Version {
        #[clap(long)]
        verbose: bool,
    },
    Steam {
        #[clap(subcommand)]
        _steam_subcommand: SteamSubcommand,
    },


    // Internal use:
    #[clap(hide = true)]
    GetActionDisplayNameMapping,
    #[clap(hide = true)]
    RunSystemCommand { command: String, args: Option<Vec<String>> },
    #[clap(hide = true)]
    DebugSteamShortcuts {
        filename: Option<String>,
    },
    #[clap(hide = true)]
    AddDecktricksToSteam,
}

impl Action {
    #[must_use]
    pub fn does_not_need_system_context(&self, gather_context_on_specific_actions: bool) -> bool {
        matches!(self, Self::Info { .. } | Self::GetConfig)
            || (gather_context_on_specific_actions
                && matches!(TypedAction::from(self), TypedAction::Specific(_)))
    }
}
