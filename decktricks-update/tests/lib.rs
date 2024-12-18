use std::ffi::OsStr;
use std::process::Command;

const COMMAND: &str = env!("CARGO_BIN_EXE_decktricks-update");
const TEST_DATA_DIR: &str = "test-data";

struct ExpectedResults {
    filename: &'static str,
    should_succeed: bool,
    //expected_error: Box<dyn std::error::Error>,
}

fn get_test_data() -> Vec<ExpectedResults> {
    vec![
        ExpectedResults {
            filename: "test-data/XXH64SUMS.correct",
            should_succeed: true,
        },
        ExpectedResults {
            filename: "test-data/XXH64SUMS.lawl_incorrect",
            should_succeed: false,
        },
        ExpectedResults {
            filename: "test-data/XXH64SUMS.lol_filename_not_present",
            should_succeed: true,
        },
        ExpectedResults {
            filename: "test-data/XXH64SUMS.lol_incorrect",
            should_succeed: false,
        },
        ExpectedResults {
            filename: "test-data/XXH64SUMS.nonexistent_file",
            should_succeed: false,
        },
        ExpectedResults {
            filename: "test-data/XXH64SUMS.tar_incorrect",
            should_succeed: false,
        },
    ]
}

#[test]
fn check_all_failures() {
    for expected_results in get_test_data() {
        let filename = expected_results.filename;

        let mut cmd = Command::new(COMMAND);
        cmd.args(["check-hashes", TEST_DATA_DIR, filename]);

        let succeeded = cmd.status().unwrap().success();

        if succeeded != expected_results.should_succeed {
            println!("Testing: \"{filename}\"");
            println!("Command:\n$ {} {}", COMMAND, cmd.get_args().map(OsStr::to_string_lossy).collect::<Vec<_>>().join(" "));
            println!("Expected: {}, Found: {}", expected_results.should_succeed, succeeded);
        }

        assert_eq!(expected_results.should_succeed, succeeded);
    }
}
