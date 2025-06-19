use crate::prelude::geforce_now::GEFORCE_LOCAL_EXECUTABLE;
use crate::prelude::*;
use crate::providers::emudeck_installer::get_emudeck_binary_path;
use crate::utils::{get_homedir, which};
use std::collections::HashMap;
use std::fmt::Display;
use std::sync::LazyLock;
use std::time::{SystemTime, UNIX_EPOCH};
use steam_shortcuts_util::parse_shortcuts;
use steam_shortcuts_util::shortcut::{Shortcut, ShortcutOwned};
use steam_shortcuts_util::shortcuts_to_bytes;

const DECKTRICKS_FULL_APPID_FILENAME: &str = "/tmp/decktricks_newest_full_steam_appid";
const DECKTRICKS_NAME_STRING: &str = "decktricks";

static STEAM_USERDATA_PATH: LazyLock<String> = LazyLock::new(|| {
    let homedir = get_homedir();
    format!("{homedir}/.local/share/Steam/userdata")
});

use std::ops::{Deref, DerefMut};

#[derive(Debug, Clone)]
pub struct SteamShortcut(ShortcutOwned);

impl From<ShortcutOwned> for SteamShortcut {
    fn from(s: ShortcutOwned) -> Self {
        Self(s)
    }
}

impl Deref for SteamShortcut {
    type Target = ShortcutOwned;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for SteamShortcut {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl SteamShortcut {
    fn get_all_from_system() -> DeckResult<Vec<Self>> {
        Ok(get_steam_shortcuts_inner(None, false)?
            .values()
            .flat_map(|v| v.clone())
            .collect())
    }

    fn is_decktricks_shortcut(&self) -> bool {
        self.tags.iter().any(|s| s == DECKTRICKS_NAME_STRING)
    }

    fn is_existing_trick_shortcut(&self, trick_id: &TrickID) -> bool {
        let trick_tag = format!("decktricks-{}", trick_id);
        self.tags.contains(&trick_tag)
    }
}

#[derive(Debug, Default, Clone)]
pub struct AllKnownSteamShortcutsContext {
    shortcuts: Vec<SteamShortcut>,
}

impl AllKnownSteamShortcutsContext {
    pub(crate) fn trick_has_existing_shortcut(&self, trick_id: &TrickID) -> bool {
        self.shortcuts
            .iter()
            .any(|s| s.is_existing_trick_shortcut(trick_id))
    }

    // NOTE: the unused ctx here is a good clue that we're not using our test-safe abstractions
    pub(crate) fn gather_with(_ctx: &impl ExecCtx) -> DeckResult<Self> {
        let shortcuts = SteamShortcut::get_all_from_system()?;

        Ok(Self { shortcuts })
    }
}

// TODO: call this from wherever you gather full context
pub(crate) fn get_steam_shortcuts_inner(
    mut specific_filename: Option<String>,
    fail_if_not_found: bool,
) -> DeckResult<HashMap<String, Vec<SteamShortcut>>> {
    if let Ok(fname) = std::env::var("DECKTRICKS_OVERRIDE_STEAM_SHORTCUTS_FILE") {
        specific_filename = Some(fname);
    }

    if let Some(filename) = specific_filename {
        let shortcuts = get_current_shortcuts_from_file(&filename, fail_if_not_found)?;
        Ok(HashMap::from([(filename, shortcuts)]))
    } else {
        let userdata_path = get_userdata_path();
        let mut map = HashMap::new();
        for ref steam_userid in get_steam_userids(userdata_path, fail_if_not_found)? {
            let filename = format!("{userdata_path}/{steam_userid}/config/shortcuts.vdf");
            let shortcuts = get_current_shortcuts_from_file(&filename, fail_if_not_found)?;
            map.insert(filename, shortcuts);
        }
        Ok(map)
    }
}

fn get_full_appid(appid_short: u32) -> u64 {
    let appid_long: u64 = (u64::from(appid_short) << 32) | 0x02_000_000;
    appid_long
}

#[derive(Debug)]
pub enum AddToSteamTarget {
    Decktricks,
    Specific(TrickAddToSteamContext),
}

impl Display for AddToSteamTarget {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Decktricks => "Decktricks",
                Self::Specific(ctx) => ctx.app_name.as_ref(),
            }
        )
    }
}

#[derive(Debug, Default, Clone)]
pub struct TrickAddToSteamContext {
    pub trick_id: String,
    pub app_name: String,
    pub exe: String,
    pub start_dir: String,
    pub icon: String,
    pub shortcut_path: String,
    pub launch_options: String,
}

impl TryFrom<&Trick> for TrickAddToSteamContext {
    type Error = KnownError;
    fn try_from(trick: &Trick) -> Result<Self, Self::Error> {
        let trick_id = trick.id.clone();
        // NOTE: `exe` can be in double quotes if you are worried about commands with spaces etc
        let app_name = trick.display_name.clone();
        let prov = trick.provider_config.clone();
        let icon = trick.icon.clone().unwrap_or_default();
        // TODO: determine if this does anything, and if so what and if it's worth it
        let shortcut_path = String::default();
        //let exe = "".into(); // FIXME
        //let launch_options = "".into(); // FIXME
        //let start_dir = "".into(); // FIXME

        let ctx = match prov {
            ProviderConfig::Flatpak(flatpak) => {
                let exe = "\"/usr/bin/flatpak\"".into();

                // NOTE: in the future you may need to add --command=blah here, but for most things this will work
                let launch_options = format!("\"run\" \"--branch=stable\" \"--arch=x86_64\" \"{}\"", flatpak.id);

                // This is what Steam uses, even if it's a silly place to cd to
                let start_dir = "/usr/bin".into();

                TrickAddToSteamContext {
                    trick_id,
                    app_name,
                    exe,
                    start_dir,
                    icon,
                    shortcut_path,
                    launch_options,
                }
            },

            ProviderConfig::DeckyInstaller(_decky) => {
                Err(KnownError::AddToSteamError("Decky is automatically added to Steam! You should never see this error, please report it.".to_string()))?
            },

            ProviderConfig::EmuDeckInstaller(_emudeck) => {
                let exe = format!("\"{}\"", get_emudeck_binary_path());
                let start_dir = "/usr/bin".into();
                let launch_options = String::default();
                TrickAddToSteamContext {
                    trick_id,
                    app_name,
                    exe,
                    start_dir,
                    icon,
                    shortcut_path,
                    launch_options,
                }
            },

            ProviderConfig::GeForceInstaller(_emudeck) => {
                let exe = format!("\"{}\"", GEFORCE_LOCAL_EXECUTABLE.as_str());
                let start_dir = "/usr/bin".into();
                let launch_options = String::default();
                TrickAddToSteamContext {
                    trick_id,
                    app_name,
                    exe,
                    start_dir,
                    icon,
                    shortcut_path,
                    launch_options,
                }
            },

            ProviderConfig::SimpleCommand(cmd) => {
                let exe_unwrapped = which(&cmd.command)?;

                let exe = format!("\"{exe_unwrapped}\"");

                let start_dir = cmd.execution_dir.unwrap_or_else(|| "/usr/bin".into());

                let launch_options = match cmd.args {
                    Some(args) => args.join(" "),
                    None => String::default(),
                };

                TrickAddToSteamContext {
                    trick_id,
                    app_name,
                    exe,
                    start_dir,
                    icon,
                    shortcut_path,
                    launch_options,
                }
            },
            ProviderConfig::SystemdRun(d) => {
                let exe_unwrapped = which(&d.command)?;

                let exe = format!("\"{exe_unwrapped}\"");

                let start_dir = d.execution_dir.unwrap_or_else(|| "/usr/bin".into());

                let launch_options = match d.args {
                    Some(args) => args.join(" "),
                    None => String::default(),
                };

                TrickAddToSteamContext {
                    trick_id,
                    app_name,
                    exe,
                    start_dir,
                    icon,
                    shortcut_path,
                    launch_options,
                }
            },
        };
        Ok(ctx)
    }
}

#[allow(clippy::cast_possible_truncation)]
fn get_unix_time() -> u32 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_secs() as u32
}

fn get_userdata_path() -> &'static str {
    STEAM_USERDATA_PATH.as_str()
}

fn get_steam_userids(userdata_path: &str, fail_if_not_found: bool) -> DeckResult<Vec<String>> {
    let mut steam_userids = vec![];

    if let Ok(entries) = std::fs::read_dir(userdata_path) {
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                if let Some(dir_name) = path.file_name().and_then(|n| n.to_str()) {
                    if dir_name.chars().all(|c| c.is_ascii_digit()) && dir_name != "0" {
                        steam_userids.push(dir_name.into());
                    }
                }
            }
        }
    }
    if steam_userids.is_empty() && fail_if_not_found {
        Err(KnownError::AddToSteamError(
            "No Steam user directory found!".into(),
        ))
    } else {
        Ok(steam_userids)
    }
}

fn get_current_shortcuts_from_file(
    filename: &str,
    fail_if_not_found: bool,
) -> DeckResult<Vec<SteamShortcut>> {
    let content = match std::fs::read(filename) {
        Ok(content) => content,
        Err(err) => {
            if fail_if_not_found {
                Err(err).map_err(|e| {
                    KnownError::AddToSteamError(format!(
                        "Failed to read shortcuts from {filename}: {e:#?}"
                    ))
                })?
            } else {
                return Ok(Vec::<SteamShortcut>::default());
            }
        }
    };
    if content.is_empty() {
        Ok(Vec::<SteamShortcut>::default())
    } else {
        Ok(parse_shortcuts(content.as_slice())
            .map_err(|e| {
                KnownError::AddToSteamError(format!(
                    "Error parsing shortcuts from {filename}: {e:#?}"
                ))
            })?
            .iter()
            .map(Shortcut::to_owned)
            .map(From::from)
            .collect())
    }
}

fn create_decktricks_shortcut(desired_order_num: &str) -> SteamShortcut {
    let app_name = "Decktricks";
    let homedir = get_homedir();
    // TODO: wrap in double quotes always? often in Steam this is in double quotes
    let bindir_path = format!("{homedir}/.local/share/decktricks/bin");
    let exe = format!("{bindir_path}/decktricks-gui.sh");
    let start_dir = bindir_path;
    let icon = "";
    // Is there any advantage to setting this?
    let shortcut_path = "";
    let launch_options = "";

    let mut decktricks_shortcut = Shortcut::new(
        desired_order_num,
        app_name,
        exe.as_ref(),
        start_dir.as_ref(),
        icon,
        shortcut_path,
        launch_options,
    );

    // Set the last play time to now, so that Decktricks (or any program?) shows up
    // first/early in the launcher
    // NOTE: This does not seem to work, hence the complicated steam restarts
    //       in the installer
    decktricks_shortcut.last_play_time = get_unix_time();

    decktricks_shortcut.tags = vec!["decktricks"];

    SteamShortcut::from(decktricks_shortcut.to_owned())
}

fn create_specific_shortcut(
    desired_order_num: &str,
    sctx: &TrickAddToSteamContext,
) -> SteamShortcut {
    let app_name = &sctx.app_name;
    let exe = &sctx.exe;
    let start_dir = &sctx.start_dir;
    let icon = &sctx.icon;
    // Is there any advantage to setting this?
    let shortcut_path = &sctx.shortcut_path;
    let launch_options = &sctx.launch_options;

    let mut specific_shortcut = Shortcut::new(
        desired_order_num,
        app_name,
        exe,
        start_dir,
        icon,
        shortcut_path,
        launch_options,
    );

    // Set the last play time to now, so that Decktricks (or any program?) shows up
    // first/early in the launcher
    // NOTE: This does not seem to work, hence the complicated steam restarts
    //       in the installer
    specific_shortcut.last_play_time = get_unix_time();

    let tag = format!("decktricks-{}", sctx.trick_id);
    specific_shortcut.tags = vec![tag.as_ref()];

    SteamShortcut::from(specific_shortcut.to_owned())
}

fn create_shortcut(target: &AddToSteamTarget, desired_order_num: &str) -> SteamShortcut {
    match target {
        AddToSteamTarget::Decktricks => create_decktricks_shortcut(desired_order_num),
        AddToSteamTarget::Specific(sctx) => create_specific_shortcut(desired_order_num, sctx),
    }
}

// Return the highest order num in the file, plus one, to ensure we always add our new entry
// as a unique ID that isn't already present in the file
fn get_desired_order_num(shortcuts: &[SteamShortcut]) -> String {
    match shortcuts
        .iter()
        .map(|s| {
            s.order.parse::<i64>().unwrap_or_else(|e| {
                eprintln!("Error parsing order number, defaulting to 1000. Error: {e}");
                999
            })
        })
        .max()
    {
        Some(max) => max + 1,
        None => 0,
    }
    .to_string()
}

fn write_shortcuts_to_disk(path: &str, shortcuts: &[SteamShortcut]) -> DeckResult<()> {
    let shortcut_bytes_vec = shortcuts_to_bytes(&shortcuts.iter().map(|s| s.borrow()).collect());
    std::fs::write(path, &shortcut_bytes_vec).map_err(|e| {
        KnownError::AddToSteamError(format!("Failed to write shortcuts to disk: {e:#?}"))
    })?;
    Ok(())
}

/// # Errors
///
/// Errors from finding, reading, and parsing shortcuts.vdf files
#[cfg(not(test))]
pub fn add_to_steam(target: &AddToSteamTarget) -> DeckResult<ActionSuccess> {
    add_to_steam_real(target)
}

#[cfg(test)]
pub fn add_to_steam(_target: &AddToSteamTarget) -> DeckResult<ActionSuccess> {
    success!("Ran in test...")
}

pub fn add_to_steam_real(target: &AddToSteamTarget) -> DeckResult<ActionSuccess> {
    let mut newest_shortcut = None;
    // In all likelihood, this will only ever run once. But since we don't know the
    // steam userid of the current user, we hedge our bets and add the shortcut to
    // all steam users present on the system.
    for (filename, mut shortcuts) in get_steam_shortcuts_inner(None, false)? {
        // Trim out existing Decktricks shortcuts
        match target {
            AddToSteamTarget::Decktricks => {
                shortcuts.retain(|s| !s.is_decktricks_shortcut());
            }
            AddToSteamTarget::Specific(sctx) => {
                shortcuts.retain(|s| !s.is_existing_trick_shortcut(&sctx.trick_id));
            }
        }

        let desired_order_num = get_desired_order_num(&shortcuts);
        let new_shortcut = create_shortcut(target, desired_order_num.as_ref());

        // Used for getting the full app_id below
        newest_shortcut = Some(new_shortcut.clone());

        shortcuts.push(new_shortcut);
        write_shortcuts_to_disk(&filename, &shortcuts)?;
    }

    if let AddToSteamTarget::Decktricks = target {
        match newest_shortcut {
            Some(shortcut) => {
                std::fs::write(
                    DECKTRICKS_FULL_APPID_FILENAME,
                    format!("{}", get_full_appid(shortcut.app_id)),
                )
                .map_err(|e| {
                    KnownError::AddToSteamError(format!(
                        "Failed to write full appid to disk: {e:#?}"
                    ))
                })?;
            }
            None => Err(KnownError::AddToSteamError(
                "Did not add Decktricks to Steam, so cannot write to file!".into(),
            ))?,
        };
    };

    success!("Successfully added \"{}\" to Steam.", target)
}

pub(crate) fn debug_steam_shortcuts(filename: Option<String>) -> DeckResult<ActionSuccess> {
    let mut outputs = Vec::<String>::default();
    for (filename, shortcuts) in get_steam_shortcuts_inner(filename, true)? {
        outputs.push(format!("{filename}: "));
        for shortcut in shortcuts {
            outputs.push(format!("{shortcut:#?}"));
            outputs.push(format!(
                "full_appid: {}\n\n",
                get_full_appid(shortcut.app_id)
            ));
        }
    }
    success!(outputs.join("\n"))
}
