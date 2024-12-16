use crate::NUM_RETRIES;
use std::error::Error;


//   curl -f -L -O --progress-bar --retry 7 --connect-timeout 60 \
//     --output-dir "$TMP_UPDATE" \
//     'https://github.com/MrAwesome/decktricks/releases/download/stable/decktricks.tar.xz'
pub fn live_update() -> Result<String, Box<dyn Error>> {
    Ok("placeholder".into())
//    return dt_err(format!(
//            "!!! Failed to update to new Decktricks version! Last error was:\n{err:#?}"
//    ));
//    println!("Decktricks updated successfully! Restart Decktricks to use the new version.");
//    }
}
