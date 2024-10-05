use crate::gui::GuiType;
pub use clap::{Parser, Subcommand};

#[derive(Parser)]
#[clap(name = "decktricks")]
pub struct DeckTricksCommand {
    // The actual command to be run
    #[clap(subcommand)]
    pub action: Action,

    // Anything below this line is global context for Actions we run
    // ---------------------------------------------------------- //

    // Filename of an override config
    #[clap(short, long)]
    pub config: Option<String>,
    // Currently unused since pretty_env_logger uses env variables.
    // #[clap(short, long)]
    // pub debug: bool,
}

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
}
