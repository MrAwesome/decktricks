use crate::error::dt_err;
use crate::NUM_RETRIES;
use std::convert::Infallible;
use std::io::Read;
use std::time::Duration;

pub(crate) fn download_url(
    url: &str,
) -> Result<Box<dyn Read + Send + Sync>, Box<dyn std::error::Error>> {
    let mut exponential_backoff_wait = 1;
    let mut retries_remaining = NUM_RETRIES;
    loop {
        let last_err: Result<Infallible, ureq::Error> = match ureq::get(url).call() {
            Err(err) => {
                if retries_remaining == 0 {
                    return Err(dt_err(format!(
                        "!!! Failed to update to new Decktricks version! Last error was:\n{err:#?}"
                    )));
                }
                Err(err)
            }
            Ok(response) => return Ok(response.into_reader()),
        };

        eprintln!("Failed to fetch {url}, will retry. Error was: {last_err:#?}");
        eprintln!("Will sleep {exponential_backoff_wait} seconds before trying again...");
        std::thread::sleep(Duration::from_secs(exponential_backoff_wait));
        exponential_backoff_wait *= 2;
        retries_remaining -= 1;
    }
}
