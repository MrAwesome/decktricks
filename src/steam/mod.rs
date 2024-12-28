use clap::Subcommand;

#[derive(Clone, Debug, Subcommand)]
pub enum SteamSubcommand {
    Restart,
    RunGameID {
        full_game_id: String,
    },
    WaitForDecktricksShortcutRelaunch,
}
