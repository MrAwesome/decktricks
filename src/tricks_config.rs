use crate::prelude::*;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::btree_map::Iter;
use std::collections::BTreeMap;
use std::fs;

// TODO: unit test error messages for incorrect configs

// TODO: place this in the correct XDG dir and read from there, default to a compiled-in version
pub const DEFAULT_CONFIG_CONTENTS: &str = include_str!("../config.json");
//const DEFAULT_CONFIG_LOCATION: &str = "config.json";

#[derive(Debug, Deserialize)]
pub struct TricksConfig {
    pub known_categories: Vec<CategoryID>,
    pub tricks: Vec<Trick>,
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

#[derive(Debug, Clone)]
pub struct TricksLoader {
    tricks: BTreeMap<TrickID, Trick>,
    categories: Vec<CategoryID>,
}

impl TryFrom<&str> for TricksLoader {
    type Error = KnownError;

    fn try_from(text: &str) -> DeckResult<Self> {
        let mut config = TricksConfig::try_from(text)?;

        // Since we will almost always be sorting by display name
        // in the GUI, go ahead and sort here.
        config.tricks.sort_by_key(|t| t.display_name.clone());

        let mut tricks = BTreeMap::new();
        for trick in config.tricks {
            tricks.insert(trick.id.clone(), trick);
        }

        let categories = config.known_categories;

        Ok(Self { tricks, categories })
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
    pub fn get_btreemap(&self) -> &BTreeMap<TrickID, Trick> {
        &self.tricks
    }

    pub fn get_all_categories(&self) -> Vec<CategoryID> {
        self.categories.clone()
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
    pub icon: Option<String>,
    pub categories: Vec<String>,
    //download: Option<String>,
    //command_before: Option<String>,
    //command_after: Option<String>,
    //depends: Vec<TrickID>,
}

#[cfg(test)]
impl Trick {
    pub(crate) fn test() -> Self {
        Self {
            id: "trick_for_test".into(),
            provider_config: ProviderConfig::SimpleCommand(SimpleCommand {
                command: Default::default(),
                args: Default::default(),
                execution_dir: Default::default(),
            }),
            categories: Default::default(),
            display_name: Default::default(),
            description: Default::default(),
            always_present_on_steamdeck: Default::default(),
            icon: Default::default(),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "kebab-case")]
#[serde(tag = "type")]
pub enum ProviderConfig {
    Flatpak(Flatpak),
    DeckyInstaller(DeckyInstaller),
    EmuDeckInstaller(EmuDeckInstaller),
    SimpleCommand(SimpleCommand),
    SystemdRun(SystemdRun),
    //SystemPackage(SystemPackage)
}

impl std::fmt::Display for ProviderConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            ProviderConfig::Flatpak(_) => write!(f, "Flatpak"),
            ProviderConfig::DeckyInstaller(_) => write!(f, "DeckyInstaller"),
            ProviderConfig::EmuDeckInstaller(_) => write!(f, "EmuDeckInstaller"),
            ProviderConfig::SimpleCommand(_) => write!(f, "SimpleCommand"),
            ProviderConfig::SystemdRun(_) => write!(f, "SystemdRun"),
        }
    }
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Flatpak {
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SimpleCommand {
    pub command: String,
    pub args: Option<Vec<String>>,
    pub execution_dir: Option<String>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SystemdRun {
    pub unit_id: String,
    pub command: String,
    pub args: Option<Vec<String>>,
    pub execution_dir: Option<String>,
    pub is_system: Option<bool>,
}

impl SystemdRun {
    pub(crate) fn get_as_args(&self) -> Vec<String> {
        let mut args = vec!["--user", "--collect"]
            .iter()
            .map(ToString::to_string)
            .collect::<Vec<_>>();

        if let Some(ex) = &self.execution_dir {
            let dir_arg = format!("--working-directory={}", ex);
            args.push(dir_arg)
        };

        args.push(format!("--unit={}", self.unit_id));
        args.push(self.command.clone());

        if let Some(cmd_args) = &self.args {
            args.extend_from_slice(cmd_args);
        };

        args
    }
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
        categories: vec![],
        display_name: "ProtonUp-Qt".into(),
        always_present_on_steamdeck: None,
        icon: None,
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

#[test]
fn integration_check_categories() -> DeckResult<()> {
    let config = TricksConfig::try_from(DEFAULT_CONFIG_CONTENTS)?;
    for trick in config.tricks {
        assert!(!trick.categories.is_empty());
        for category in trick.categories {
            assert!(config.known_categories.contains(&category));
        }
    }
    Ok(())
}
