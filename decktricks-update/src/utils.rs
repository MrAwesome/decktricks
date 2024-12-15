pub(crate) fn get_homedir() -> String {
    std::env::var("HOME").unwrap_or_else(|_| "/home/deck".into())
}

pub(crate) fn get_decktricks_dir() -> String {
    format!("{}/{}", get_homedir(), ".local/share/decktricks")
}

pub(crate) fn get_update_tmpdir() -> String {
    format!("{}/{}", get_decktricks_dir(), "tmp_update")
}
