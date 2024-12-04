use crate::prelude::*;
use crate::providers::emudeck_installer::get_emudeck_binary_path;
use crate::utils::{get_homedir, which};
use std::collections::HashMap;
use std::fmt::Display;
use std::time::{SystemTime, UNIX_EPOCH};
use steam_shortcuts_util::parse_shortcuts;
use steam_shortcuts_util::shortcut::{Shortcut, ShortcutOwned};
use steam_shortcuts_util::shortcuts_to_bytes;

// TODO: gate in tests

#[derive(Debug)]
pub enum AddToSteamTarget {
    Decktricks,
    Specific(AddToSteamContext),
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

#[derive(Debug, Default)]
pub struct AddToSteamContext {
    pub app_name: String,
    pub exe: String,
    pub start_dir: String,
    pub icon: String,
    pub shortcut_path: String,
    pub launch_options: String,
}

impl TryFrom<&Trick> for AddToSteamContext {
    type Error = KnownError;
    fn try_from(trick: &Trick) -> Result<Self, Self::Error> {
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

                AddToSteamContext {
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
                AddToSteamContext {
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

                AddToSteamContext {
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

fn get_userdata_path() -> String {
    let homedir = get_homedir();
    format!("{homedir}/.local/share/Steam/userdata")
}

fn get_steam_userids(userdata_path: &str) -> DeckResult<Vec<String>> {
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
    if steam_userids.is_empty() {
        Err(KnownError::AddToSteamError(
            "No Steam user directory found!".into(),
        ))
    } else {
        Ok(steam_userids)
    }
}

fn get_current_shortcuts(filename: &str, fail_if_not_found: bool) -> DeckResult<Vec<ShortcutOwned>> {
    let content = match std::fs::read(filename) {
        Ok(content) => content,
        Err(err) => {
            if fail_if_not_found {
                Err(err)
                    .map_err(|e| KnownError::AddToSteamError(format!("Failed to read shortcuts from {filename}: {e:#?}")))?
            } else {
                return Ok(Vec::<ShortcutOwned>::default())
            }
        },
    };
    if content.is_empty() {
        Ok(Vec::<ShortcutOwned>::default())
    } else {
        Ok(parse_shortcuts(content.as_slice())
            .map_err(|e| KnownError::AddToSteamError(format!("Error parsing shortcuts from {filename}: {e:#?}")))?
            .iter()
            .map(Shortcut::to_owned)
            .collect())
    }
}

fn create_decktricks_shortcut(desired_order_num: &str) -> ShortcutOwned {
    let app_name = "Decktricks";
    let homedir = get_homedir();
    // TODO: wrap in double quotes always?
    let dtdir_path = format!("{homedir}/.local/share/decktricks");
    let exe = format!("{dtdir_path}/decktricks-gui.sh");
    let start_dir = dtdir_path;
    let icon = "";
    // Is there any advantage to setting this?
    let shortcut_path = "";
    let launch_options = "";

    let mut new_shortcut = Shortcut::new(
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
    new_shortcut.last_play_time = get_unix_time();

    new_shortcut.to_owned()
}

fn create_specific_shortcut(desired_order_num: &str, sctx: &AddToSteamContext) -> ShortcutOwned {
    let app_name = &sctx.app_name;
    let exe = &sctx.exe;
    let start_dir = &sctx.start_dir;
    let icon = &sctx.icon;
    // Is there any advantage to setting this?
    let shortcut_path = &sctx.shortcut_path;
    let launch_options = &sctx.launch_options;

    let mut new_shortcut = Shortcut::new(
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
    new_shortcut.last_play_time = get_unix_time();

    new_shortcut.to_owned()
}

fn create_shortcut(target: &AddToSteamTarget, desired_order_num: &str) -> ShortcutOwned {
    match target {
        AddToSteamTarget::Decktricks => create_decktricks_shortcut(desired_order_num),
        AddToSteamTarget::Specific(sctx) => create_specific_shortcut(desired_order_num, sctx),
    }
}

// Return the highest order num in the file, plus one
fn get_desired_order_num(shortcuts: &[ShortcutOwned]) -> String {
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

fn write_shortcuts_to_disk(path: &str, shortcuts: &[ShortcutOwned]) -> DeckResult<()> {
    let shortcut_bytes_vec = shortcuts_to_bytes(&shortcuts.iter().map(|s| s.borrow()).collect());
    std::fs::write(path, &shortcut_bytes_vec)
        .map_err(|e| KnownError::AddToSteamError(format!("Failed to write shortcuts to disk: {e:#?}")))?;
    Ok(())
}

pub(crate) fn get_shortcuts(
    mut filename: Option<String>,
    fail_if_not_found: bool,
) -> DeckResult<HashMap<String, Vec<ShortcutOwned>>> {
    let override_filename = std::env::var("DECKTRICKS_OVERRIDE_STEAM_SHORTCUTS_FILE");
    if let Ok(fname) = override_filename {
        filename = Some(fname);
    }

    if let Some(filename) = filename {
        let shortcuts = get_current_shortcuts(&filename, fail_if_not_found)?;
        Ok(HashMap::from([(filename, shortcuts)]))
    } else {
        let userdata_path = &get_userdata_path();
        let mut map = HashMap::new();
        for ref steam_userid in get_steam_userids(userdata_path)? {
            let filename = format!("{userdata_path}/{steam_userid}/config/shortcuts.vdf");
            let shortcuts = get_current_shortcuts(&filename, fail_if_not_found)?;
            map.insert(filename, shortcuts);
        }
        Ok(map)
    }
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
    for (filename, mut shortcuts) in get_shortcuts(None, false)? {
        let desired_order_num = get_desired_order_num(&shortcuts);
        let new_shortcut = create_shortcut(target, desired_order_num.as_ref());
        shortcuts.push(new_shortcut);
        write_shortcuts_to_disk(&filename, &shortcuts)?;
    }
    success!("Successfully added \"{}\" to Steam.", target)
}

pub(crate) fn debug_steam_shortcuts(filename: Option<String>) -> DeckResult<ActionSuccess> {
    success!(format!("{:#?}", get_shortcuts(filename, true)?))
}
