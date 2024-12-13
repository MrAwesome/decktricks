// IMPORTANT!
//
// Be *very* careful making changes to this code. Any mistakes here can lock
// every Decktricks user onto an old/broken version,
// with no way to automatically update to a new version.
//
// TODO: always continue if hashes don't match, but only after multiple retries and a warning
// TODO: untar
// TODO: check/generate desired hash of checksum file
// TODO: check/generate desired hash of tarball
// TODO: check presence/hashes of all files on disk (let checksums file be the source of truth on what to grab)
// TODO: extensive unit and integration tests
// TODO: "generate-hashes <tar>" (generates hashes for everything in the tar and the tar itself) 
// TODO: "live-update" (download and go)

// We handle all args ourselves just to keep the number of dependencies for this very critical code
// as low as possible.
use decktricks_update::HashCheckOutcome;
use decktricks_update::check_hashes_for_update_needed;
use decktricks_update::live_update;
use decktricks_update::generate_hashfile_for_tarball;

fn print_usage_and_exit() -> ! {
    eprintln!("Usage: decktricks-update <cmd>

Commands:
    generate-hashes <tar-filename>
    check-hashes <target-dir> <hash-filename>
    live-update
");
    std::process::exit(1);
}

fn main() {
    let mut args = std::env::args();
    args.next(); // Ignore $0
    let output = match args.next() {
        Some(arg) => match arg.as_str() {
            "generate-hashes" => match args.next() {
                Some(hashes_arg) => generate_hashfile_for_tarball(hashes_arg.as_str()),
                None => print_usage_and_exit()
            }
            "live-update" => live_update(),
            "check-hashes" => match (args.next(), args.next()) {
                (Some(target_dir), Some(hash_filename)) => 
                    match check_hashes_for_update_needed(target_dir.as_str(), hash_filename.as_str()) {
                        Ok(needed) => match needed {
                            HashCheckOutcome::NoActionNeeded => std::process::exit(0),
                            HashCheckOutcome::UpdateNeeded => std::process::exit(1),
                        },
                        Err(err) => Err(err),
                    }
                _ => print_usage_and_exit()
            }
            _ => print_usage_and_exit(),
        }
        None => print_usage_and_exit()
    };
    // TODO: handle errors and report back to gui
    println!("{}", output.unwrap());
}
