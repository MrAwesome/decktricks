use crate::prelude::*;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::hash_map::Iter;
use std::collections::HashMap;
use std::fs;

// TODO: unit test error messages for incorrect configs

// TODO: place this in the correct XDG dir and read from there, default to a compiled-in version
pub const DEFAULT_CONFIG_CONTENTS: &str = include_str!("../config.json");
//const DEFAULT_CONFIG_LOCATION: &str = "config.json";
pub type TrickID = String;

#[derive(Debug, Deserialize)]
pub struct TricksConfig {
    tricks: Vec<Trick>,
}

impl TryFrom<&str> for TricksConfig {
    type Error = KnownError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        serde_json::from_str(value).map_err(KnownError::from)
    }
}

impl From<serde_json::Error> for KnownError {
    fn from(e: serde_json::Error) -> Self {
        KnownError::ConfigParsing(e)
    }
}

pub struct TricksLoader {
    tricks: HashMap<TrickID, Trick>,
}

impl TryFrom<&str> for TricksLoader {
    type Error = KnownError;

    fn try_from(text: &str) -> DeckResult<Self> {
        let config = TricksConfig::try_from(text)?;
        let mut tricks = HashMap::new();
        for trick in config.tricks {
            tricks.insert(trick.id.clone(), trick);
        }

        Ok(Self { tricks })
    }
}

impl TricksLoader {
    // NOTE: Currently, this does *not* read from the config file at runtime!
    //       The config is read at compile time, so you need to cargo build/run
    //       to see changes to the config.
    //
    /// # Errors
    ///
    /// Returns errors relating to file loads or config parsing.
    pub fn from_default_config() -> DeckResult<Self> {
        Self::try_from(DEFAULT_CONFIG_CONTENTS)
    }

    // NOTE: Currently, this does *not* read from the config file at runtime!
    //       The config is read at compile time, so you need to cargo build/run
    //       to see changes to the config.
    //
    /// # Errors
    ///
    /// Returns errors relating to file loads or config loading/parsing.
    pub fn from_config(path: &str) -> DeckResult<Self> {
        let contents = read_config(path)?;
        Self::try_from(contents.as_ref())
    }

    /// # Errors
    ///
    /// Returns errors relating to config parsing.
    pub fn get_trick(&self, maybe_id: &str) -> DeckResult<&Trick> {
        let maybe_trick = self.tricks.get(maybe_id);
        match maybe_trick {
            Some(trick) => Ok(trick),
            None => Err(KnownError::UnknownTrickID(maybe_id.into())),
        }
    }

    #[must_use]
    pub fn get_all_tricks(&self) -> Iter<TrickID, Trick> {
        self.tricks.iter()
    }

    #[must_use]
    pub fn get_hashmap(&self) -> &HashMap<TrickID, Trick> {
        &self.tricks
    }
}

fn read_config(path: &str) -> DeckResult<String> {
    fs::read(path)
        .map(|contents| String::from_utf8_lossy(&contents).into())
        .map_err(KnownError::ConfigRead)
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Trick {
    pub id: String,
    pub provider_config: ProviderConfig,
    pub display_name: String,
    pub description: String,
    pub always_present_on_steamdeck: Option<bool>,
    //download: Option<String>,
    //command_before: Option<String>,
    //command_after: Option<String>,
    //depends: Vec<TrickID>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "type")]
pub enum ProviderConfig {
    Flatpak(Flatpak),
    DeckyInstaller(DeckyInstaller),
    EmuDeckInstaller(EmuDeckInstaller),
    SimpleCommand(SimpleCommand),
    //SystemPackage(SystemPackage)
    Custom,
}

impl std::fmt::Display for ProviderConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ProviderConfig::Flatpak(_) => write!(f, "Flatpak"),
            ProviderConfig::DeckyInstaller(_) => write!(f, "DeckyInstaller"),
            ProviderConfig::EmuDeckInstaller(_) => write!(f, "EmuDeckInstaller"),
            ProviderConfig::SimpleCommand(_) => write!(f, "SimpleCommand"),
            ProviderConfig::Custom => write!(f, "Custom"),
        }
    }
}

// custom can be something like:
// match provider_id {
//    "decky" => { blah },
//}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Flatpak {
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SimpleCommand {
    pub command: String,
    pub args: Option<Vec<String>>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct DeckyInstaller;

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct EmuDeckInstaller;

//#[derive(Debug, Deserialize, Serialize)]
//struct SystemPackage {
//    apt: Option<String>,
//    pacman: Option<String>,
//}

// Tests a write/read cycle of config objects to the config file format
#[test]
fn reconvert_providerconfig() -> DeckResult<()> {
    let id = "net.davidotek.pupgui2";
    let trick = Trick {
        id: id.into(),
        provider_config: ProviderConfig::Flatpak(Flatpak { id: id.into() }),
        description: "lol".into(),
        display_name: "ProtonUp-Qt".into(),
        always_present_on_steamdeck: None,
    };

    let after_first_serialization =
        serde_json::to_string_pretty(&trick).map_err(KnownError::from)?;
    let expected_text = after_first_serialization.clone();
    let trick =
        serde_json::from_str::<Trick>(&after_first_serialization).map_err(KnownError::from)?;
    let after_second_serialization =
        serde_json::to_string_pretty(&trick).map_err(KnownError::from)?;

    assert_eq!(expected_text, after_second_serialization);

    Ok(())
}

// Integration test of the actual config
#[test]
fn integration_check_default_config() -> DeckResult<()> {
    let loader = TricksLoader::from_default_config()?;
    let prov = &loader.get_trick("lutris")?.provider_config;

    match prov {
        ProviderConfig::Flatpak(flatpak) => Ok(assert_eq!("net.lutris.Lutris", flatpak.id)),
        other => Err(KnownError::TestError(format!(
            "Unexpected data received for lutris config: {other:#?}"
        ))),
    }
}
