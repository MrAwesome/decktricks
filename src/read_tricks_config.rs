use serde::{Deserialize, Serialize};
use serde_with::skip_serializing_none;
use std::collections::HashMap;
use std::fs::File;
use std::io::BufReader;

type ProviderID = String;

#[skip_serializing_none]
#[derive(Debug, Deserialize, Serialize)]
struct Trick {
    provider_config: ProviderConfig,
    display_name: String,
    always_present_on_steamdeck: Option<bool>,
    //download: Option<String>,
}

//#[derive(Debug, Deserialize, Serialize)]
//struct SystemPackageInfo {
//    apt: Option<String>,
//    pacman: Option<String>,
//}

#[derive(Debug, Deserialize, Serialize)]
struct Flatpak {
    id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "snake_case")]
#[serde(tag = "type")]
enum ProviderConfig {
    Flatpak(Flatpak),
}

#[derive(Debug, Deserialize)]
struct Config {
    tricks: HashMap<ProviderID, Trick>,
}

pub fn read_tricks_config() -> Result<(), Box<dyn std::error::Error>> {
    let file = File::open("sample.jsonc")?;
    let reader = BufReader::new(file);
    let config: Config = serde_hjson::from_reader(reader)?;

    println!("{:#?}", config);

    Ok(())
}


#[test]
fn reconvert_providerconfig() -> Result<(), Box<dyn std::error::Error>> {
    let trick = Trick {
        provider_config: ProviderConfig::Flatpak(Flatpak {
            id: "net.davidotek.pupgui2".into(),
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
