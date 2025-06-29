use crate::prelude::*;
use crate::utils::get_steam_appid;

const DECKTRICKS_CONTROLLER_LAYOUT_ID: &str = "3512322196";

pub fn open_controller_config(ctx: &impl ExecCtx) {
    let maybe_appid = get_steam_appid();
    if let Some(appid) = maybe_appid {
        let layout_steam_url = format!("steam://controllerconfig/{appid}/{DECKTRICKS_CONTROLLER_LAYOUT_ID}");
        ctx.sys_command("xdg-open", [&layout_steam_url]);
        // NOTE: you may need to sleep 100ms here and then try again, per
        // https://github.com/AAGaming00/Vesktop/blob/9ffa294aee7c42e05d2f6185b3f5d0ea0312c2f5/src/main/utils/steamOS.ts#L51
    }
}

