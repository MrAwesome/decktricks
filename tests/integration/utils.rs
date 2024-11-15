use decktricks::{errors::DecktricksError, prelude::DynamicError};

use std::process::Command;

pub const BINARY_NAME: &str = env!("CARGO_BIN_EXE_decktricks");
type CliResult = Result<String, DynamicError>;
pub(crate) fn run_cli_with_args(args: Vec<&str>) -> CliResult {
    let result = Command::new(BINARY_NAME).args(args.clone()).output()?;

    if result.status.success() {
        Ok(String::from_utf8_lossy(&result.stdout).into())
    } else {
        Err(Box::new(DecktricksError::new(format!(
            "Command failed!\nCommand: {} {}\nResult: {:#?}",
            BINARY_NAME,
            args.join(" "),
            result
        ))))
    }
}

#[macro_export(local_inner_macros)]
macro_rules! decktricks_cli {
    ($($arg:tt),*) => {
        $crate::integration::utils::run_cli_with_args(std::vec![$($arg,)*])
    }
}
