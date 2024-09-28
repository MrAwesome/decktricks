#[macro_export(local_inner_macros)]
macro_rules! success {
    ($msg:expr) => {
        Ok(ActionSuccess::success(Some($msg)))
    };
    ($fmt:literal, $($arg:expr)*) => {
        Ok(ActionSuccess::success(Some(::std::format!($fmt, $($arg)*))))
    };
    () => {
        Ok(ActionSuccess::success(None as Option<String>))
    };
}

#[macro_export(local_inner_macros)]
macro_rules! err {
    ($msg:expr) => {
        Box::new(DeckTricksError::new(::std::format!($msg)))
    };
    ($fmt:literal, $($arg:expr)*) => {
        Box::new(DeckTricksError::new(::std::format!($fmt, $($arg)*)))
    };
}

#[macro_export(local_inner_macros)]
macro_rules! info {
    ($msg:expr) => {
        eprintln!("{}", $msg)
    };
    ($fmt:literal, $($arg:expr)*) => {
        eprintln!($fmt, $($arg)*)
    };
}

#[macro_export(local_inner_macros)]
macro_rules! enum_with_all_variants {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $($variant:ident),* $(,)?
        }
    ) => {
        $(#[$meta])*
        $vis enum $name {
            $($variant),*
        }

        impl $name {
            $vis fn all_variants() -> Vec<$name> {
                std::vec![$($name::$variant),*]
            }
        }
    };
}
