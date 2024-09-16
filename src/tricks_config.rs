use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use std::collections::hash_map::Iter;
use std::fs::File;
use std::io::BufReader;

const DEFAULT_CONFIG_LOCATION: &str = "config.json";
type ProviderID = String;

#[derive(Debug, Deserialize)]
pub struct TricksConfig {
    tricks: HashMap<ProviderID, Trick>,
}

impl TricksConfig {
    pub fn from_default_config() -> Result<TricksConfig, Box<dyn std::error::Error>> {
        let file = File::open(DEFAULT_CONFIG_LOCATION)?;
        let reader = BufReader::new(file);
        Ok(serde_json::from_reader(reader)?)
    }

    pub fn get_trick(&self, maybe_id: &str) -> Option<&Trick> {
        self.tricks.get(maybe_id)
    }

    pub fn get_all_tricks(&self) -> Iter<ProviderID, Trick> {
        self.tricks.iter()
    }
}


#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
pub struct Trick {
    provider_config: ProviderConfig,
    display_name: String,
    always_present_on_steamdeck: Option<bool>,
    //download: Option<String>,
    //command_before: Option<String>,
    //command_after: Option<String>,
    //depends: Vec<ProviderID>,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
enum ProviderConfig {
    Flatpak(Flatpak),
    //SystemPackage(SystemPackage)
    Custom,
}

// custtom can be something like:
// match provider_id {
//    "decky" => { blah },
//}

#[derive(Debug, Deserialize, Serialize)]
struct Flatpak {
    id: String,
}

//#[derive(Debug, Deserialize, Serialize)]
//struct SystemPackage {
//    apt: Option<String>,
//    pacman: Option<String>,
//}

// Tests a write/read cycle of config objects to the config file format
#[test]
fn reconvert_providerconfig() -> Result<(), Box<dyn std::error::Error>> {
    let id = "net.davidotek.pupgui2";
    let trick = Trick {
        provider_config: ProviderConfig::Flatpak(Flatpak {
            id: id.into(),
        }),
        display_name: "ProtonUp-Qt".into(),
        always_present_on_steamdeck: None,
    };

    let output1 = serde_json::to_string_pretty(&trick);
    let res: Result<Trick, serde_json::Error> = serde_json::from_str(&output1.as_ref().unwrap());
    let output2 = serde_json::to_string_pretty(&res.unwrap());

    assert_eq!(output1.unwrap(), output2.unwrap());

    Ok(())
}

// Integration test of the actual config
#[test]
fn integration_check_default_config() -> Result<(), Box<dyn std::error::Error>> {
    let config = TricksConfig::from_default_config()?;
    let prov = &config.get_trick("lutris").unwrap().provider_config;

    match prov {
        ProviderConfig::Flatpak(flatpak) => assert_eq!("net.lutris.Lutris", flatpak.id),
        other => panic!("Unexpected data received for lutris config: {:#?}", other),
    }

    Ok(())
}
