#[macro_export(local_inner_macros)]
macro_rules! success {
    ($msg:expr, $($arg:tt)*) => {
        Ok(ActionSuccess::success(Some(::std::format!($msg, $($arg)*))))
    };
    ($msg:expr) => {
        Ok(ActionSuccess::success(Some($msg)))
    };
    () => {
        Ok(ActionSuccess::success(None as Option<String>))
    };
}
