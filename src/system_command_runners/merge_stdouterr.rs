use std::io::Write;
use crate::prelude::*;

use std::io::{BufRead, BufReader};
use std::process::Child;
use std::sync::mpsc::{self, Sender};
use std::thread;

pub(super) enum StreamLine {
    Stdout(String),
    Stderr(String),
}

fn forward_lines<R: std::io::Read + Send + 'static>(
    reader: R,
    tx: Sender<StreamLine>,
    which: fn(String) -> StreamLine,
) {
    thread::spawn(move || {
        let buf = BufReader::new(reader);
        for line in buf.lines() {
            match line {
                Ok(line) => {
                    // send, ignore if receiver hung up
                    let _ = tx.send(which(line));
                }
                Err(err) => {
                    eprintln!("error reading stream: {}", err);
                    break;
                }
            }
        }
    });
}

pub(super) fn live_log_child_and_wait_with_output(
    ctx: &impl ExecCtx,
    cmdline_for_logging: String,
    child: &mut Child,
) -> std::io::Result<()> {
    let (stdout, stderr) = match child.stdout.take().zip(child.stderr.take()) {
        Some((out, err)) => (out, err),
        None => {
            warn!(
                ctx,
                "Failed to get stdout/err for child process! Cmdline: {}", cmdline_for_logging
            );
            return Ok(());
        }
    };

    let (tx, rx) = mpsc::channel();

    forward_lines(stdout, tx.clone(), StreamLine::Stdout);
    forward_lines(stderr, tx, StreamLine::Stderr);

    for msg in rx {
        match msg {
            StreamLine::Stdout(line) => {
                stdout_println!(ctx.as_ctx(), line)
            }
            StreamLine::Stderr(line) => {
                stdout_eprintln!(ctx.as_ctx(), line)
            }
        }
        std::io::stdout().flush().unwrap();
        std::io::stderr().flush().unwrap();
    }

    // We don't need to wait here, as we'll do wait_with_output later outside
    Ok(())
}
