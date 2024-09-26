use crate::prelude::*;
use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::hash_map::Iter;
use std::collections::HashMap;

// TODO: place this in the correct XDG dir and read from there, default to a compiled-in version
const DEFAULT_CONFIG_CONTENTS: &str = include_str!("../config.json");
//const DEFAULT_CONFIG_LOCATION: &str = "config.json";
pub type TrickID = String;

#[derive(Debug, Deserialize)]
pub struct TricksConfig {
    tricks: HashMap<TrickID, Trick>,
}

impl From<serde_json::Error> for KnownError {
    fn from(e: serde_json::Error) -> Self {
        KnownError::ConfigParsing(e)
    }
}

impl TryFrom<&str> for TricksConfig {
    type Error = KnownError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(serde_json::from_str(value).map_err(KnownError::from)?)
    }
}

impl TricksConfig {
    pub fn from_default_config() -> Result<TricksConfig, KnownError> {
        Ok(Self::try_from(DEFAULT_CONFIG_CONTENTS)?)
    }

    pub fn get_trick(&self, maybe_id: &str) -> Result<&Trick, KnownError> {
        let maybe_trick = self.tricks.get(maybe_id);
        match maybe_trick {
            Some(trick) => Ok(trick),
            None => Err(KnownError::UnknownTrickID(Box::new(DeckTricksError {
                message: format!("Trick not known: {maybe_id}"),
            }))),
        }
    }

    pub fn get_all_tricks(&self) -> Iter<TrickID, Trick> {
        self.tricks.iter()
    }
}

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct Trick {
    pub provider_config: ProviderConfig,
    pub display_name: String,
    pub always_present_on_steamdeck: Option<bool>,
    //download: Option<String>,
    //command_before: Option<String>,
    //command_after: Option<String>,
    //depends: Vec<TrickID>,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
pub enum ProviderConfig {
    Flatpak(Flatpak),
    DeckyInstaller,
    SimpleCommand(SimpleCommand),
    //SystemPackage(SystemPackage)
    Custom,
}

// custtom can be something like:
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
    pub args: Vec<String>,
}

//#[derive(Debug, Deserialize, Serialize)]
//struct SystemPackage {
//    apt: Option<String>,
//    pacman: Option<String>,
//}

// Tests a write/read cycle of config objects to the config file format
#[test]
fn reconvert_providerconfig() -> Result<(), KnownError> {
    let id = "net.davidotek.pupgui2";
    let trick = Trick {
        provider_config: ProviderConfig::Flatpak(Flatpak { id: id.into() }),
        display_name: "ProtonUp-Qt".into(),
        always_present_on_steamdeck: None,
    };

    let after_first_serialization = serde_json::to_string_pretty(&trick).map_err(KnownError::from)?;
    let expected_text = after_first_serialization.clone();
    let trick = serde_json::from_str::<Trick>(&after_first_serialization).map_err(KnownError::from)?;
    let after_second_serialization = serde_json::to_string_pretty(&trick).map_err(KnownError::from)?;

    assert_eq!(expected_text, after_second_serialization);

    Ok(())
}

// Integration test of the actual config
#[test]
fn integration_check_default_config() -> Result<(), KnownError> {
    let config = TricksConfig::from_default_config()?;
    let prov = &config.get_trick("lutris")?.provider_config;

    match prov {
        ProviderConfig::Flatpak(flatpak) => assert_eq!("net.lutris.Lutris", flatpak.id),
        other => panic!("Unexpected data received for lutris config: {:#?}", other),
    }

    Ok(())
}
