use std::{
    io::{BufRead, BufReader},
    process::{Child, Command, Stdio},
    sync::mpsc::{self, Receiver},
    thread,
};

#[derive(Debug)]
pub enum TailWatcher {
    Real(TailWatcherInner),
    Fake(TailWatcherFake),
}

impl TailWatcher {
    pub fn new(file: &str) -> Self {
        #[cfg(test)]
        {
            let _ = file;
            TailWatcherFake::new("")
        }

        #[cfg(not(test))]
        TailWatcherInner::new(file)
    }

    pub fn get_latest(&mut self) -> Option<String> {
        match self {
            Self::Real(x) => x.get_latest(),
            Self::Fake(x) => x.get_latest(),
        }
    }
}

#[derive(Debug)]
pub struct TailWatcherInner {
    rx: Receiver<String>,
    _child: Child, // keep the process alive for as long as the struct lives
}

impl TailWatcherInner {
    pub fn new(file: &str) -> TailWatcher {
        match Self::new_inner(file) {
            Ok(x) => TailWatcher::Real(x),
            Err(err) => {
                TailWatcherFake::new(&format!("Error initializing tail watcher! Error: {err:#?}"))
            }
        }
    }

    pub fn new_inner(file: &str) -> std::io::Result<Self> {
        let mut child = Command::new("tail")
            .arg("-F")
            .arg("-n")
            .arg("10000")
            .arg(file)
            .stdout(Stdio::piped())
            .spawn()?;

        let stdout = child.stdout.take().expect("failed to capture tail stdout");

        let reader = BufReader::new(stdout);
        let (tx, rx) = mpsc::channel::<String>();

        // Background thread to forward each line from tail to the channel
        thread::spawn(move || {
            for line in reader.lines() {
                match line {
                    Ok(l) => {
                        let res = tx.send(l + "\n");
                        if res.is_err() {
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }
        });

        Ok(Self { rx, _child: child })
    }

    /// Drain everything that has arrived since the last call.
    fn get_latest(&self) -> Option<String> {
        let mut out = String::new();
        for chunk in self.rx.try_iter() {
            out.push_str(&chunk);
        }
        if out.is_empty() {
            None
        } else {
            Some(out)
        }
    }
}

#[derive(Debug)]
pub struct TailWatcherFake {
    err_text: Option<String>,
}

impl TailWatcherFake {
    fn new(maybe_err_text: &str) -> TailWatcher {
        let err_text = if maybe_err_text.is_empty() {
            None
        } else {
            Some(maybe_err_text.into())
        };
        TailWatcher::Fake(Self { err_text: err_text.into() })
    }

    fn get_latest(&mut self) -> Option<String> {
        std::mem::take(&mut self.err_text)
    }
}
