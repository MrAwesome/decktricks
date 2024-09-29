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


#[macro_export(local_inner_macros)]
macro_rules! join {
    ($e1:expr, $e2:expr) => {
        rayon::join($e1, $e2)
    }
}

#[macro_export(local_inner_macros)]
macro_rules! join_all {
    ($e1:expr) => {{
        $e1()
    }};
    ($e1:expr, $e2:expr) => {{
        join!($e1, $e2)
    }};
    ($e1:expr, $e2:expr, $e3:expr) => {{
        let ((a, b), c) = join!(|| join_all!($e1, $e2), $e3);
        (a, b, c)
    }};
    ($e1:expr, $e2:expr, $e3:expr, $e4:expr) => {{
        let ((a, b), (c, d)) = join!(|| join_all!($e1, $e2), || join_all!($e3, $e4));
        (a, b, c, d)
    }};
    ($e1:expr, $e2:expr, $e3:expr, $e4:expr, $e5:expr) => {{
        let ((a, b, c, d), e) = join!(|| join_all!($e1, $e2, $e3, $e4), $e5);
        (a, b, c, d, e)
    }};
    ($e1:expr, $e2:expr, $e3:expr, $e4:expr, $e5:expr, $e6:expr) => {{
        let ((a, b, c, d), (e, f)) = join!(|| join_all!($e1, $e2, $e3, $e4), || join_all!($e5, $e6));
        (a, b, c, d, e, f)
    }};
    ($e1:expr, $e2:expr, $e3:expr, $e4:expr, $e5:expr, $e6:expr, $e7:expr) => {{
        let ((a, b, c, d, e, f), g) = join!(|| join_all!($e1, $e2, $e3, $e4, $e5, $e6), $e7);
        (a, b, c, d, e, f, g)
    }};
    ($e1:expr, $e2:expr, $e3:expr, $e4:expr, $e5:expr, $e6:expr, $e7:expr, $e8:expr) => {{
        let ((a, b, c, d), (e, f, g, h)) = join!(|| join_all!($e1, $e2, $e3, $e4), || join_all!($e5, $e6, $e7, $e8));
        (a, b, c, d, e, f, g, h)
    }};
}
