use std::time::Duration;

use crate::prelude::*;
use crate::utils::get_steam_appid;

const DECKTRICKS_CONTROLLER_LAYOUT_ID: &str = "3512322196";

pub fn open_controller_config(ctx: &impl ExecCtx) {
    let maybe_appid = get_steam_appid();
    if let Some(appid) = maybe_appid {
        let layout_steam_url =
            format!("steam://controllerconfig/{appid}/{DECKTRICKS_CONTROLLER_LAYOUT_ID}");
        // This is all based on:
        // https://github.com/AAGaming00/Vesktop/blob/9ffa294aee7c42e05d2f6185b3f5d0ea0312c2f5/src/main/utils/steamOS.ts#L51
        ctx.sys_command("steam", ["-ifrunning", &layout_steam_url]);
        std::thread::sleep(Duration::from_millis(100));
        ctx.sys_command("steam", ["-ifrunning", &layout_steam_url]);
    }
}
