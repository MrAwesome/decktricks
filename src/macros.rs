#[macro_export(local_inner_macros)]
macro_rules! success {
    ($s:expr, $($arg:tt)*) => {
        Ok(ActionSuccess::new(Some(::std::format!($s, $($arg)*))))
    };
    ($s:expr) => {
        Ok(ActionSuccess::new(Some($s)))
    };
    () => {
        Ok(ActionSuccess::new::<&str>(None))
    };
}
