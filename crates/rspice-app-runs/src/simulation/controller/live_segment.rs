//! The rows a running transient has written to its segment, opened again as the segment grows,
//! on a thread of their own so the frame never waits for the file.
//!
//! The live plot draws these rows as a finished run's are drawn: its columns read the segment,
//! and its tiles come from the store's summaries through the reader service. Nothing is copied
//! into the frame but the segment's last chunk, which each opening holds.

use std::path::PathBuf;
use std::thread::JoinHandle;
use std::time::Duration;

use rspice_simulation::live_transient::{LiveRows, open_live_rows};

use crate::time_compat::Instant;

/// How often the segment is opened again while the run writes it.
const REOPEN_EVERY: Duration = Duration::from_millis(250);

type Opening = JoinHandle<Result<Option<LiveRows>, String>>;

#[derive(Default)]
pub(super) struct LiveSegmentFollower {
    /// The opening in flight, if one is.
    opening: Option<Opening>,
    /// When the last opening began.
    opened_at: Option<Instant>,
    /// The newest rows an opening found.
    rows: Option<LiveRows>,
    /// Why the segment could not be opened, until it is said.
    failure: Option<String>,
    /// The segment could not be opened, and is not opened again for this run.
    stopped: bool,
}

impl LiveSegmentFollower {
    /// Take a finished opening and begin the next one when it is due; whether the rows grew.
    ///
    /// `path` is the segment the run is writing now, `None` once it stopped writing; the rows
    /// already found stay until [`Self::clear`].
    pub(super) fn poll(&mut self, path: Option<PathBuf>) -> bool {
        let mut grew = false;
        if self.opening.as_ref().is_some_and(JoinHandle::is_finished)
            && let Some(opening) = self.opening.take()
        {
            match opening.join() {
                Ok(Ok(Some(rows))) => {
                    if self
                        .rows
                        .as_ref()
                        .is_none_or(|shown| rows.rows > shown.rows)
                    {
                        self.rows = Some(rows);
                        grew = true;
                    }
                }
                Ok(Ok(None)) => {}
                Ok(Err(error)) => {
                    self.failure = Some(error);
                    self.stopped = true;
                }
                Err(_) => {
                    self.failure = Some("reading the running transient's rows failed".to_owned());
                    self.stopped = true;
                }
            }
        }
        if let Some(path) = path
            && self.opening.is_none()
            && !self.stopped
            && self.opened_at.is_none_or(|at| at.elapsed() >= REOPEN_EVERY)
        {
            self.opened_at = Some(Instant::now());
            self.opening = std::thread::Builder::new()
                .name("rspice-live-rows".to_owned())
                .spawn(move || open_live_rows(&path))
                .ok();
        }
        grew
    }

    /// The newest rows found, while the run's are being followed.
    pub(super) fn rows(&self) -> Option<&LiveRows> {
        self.rows.as_ref()
    }

    /// Why the segment could not be opened, once.
    pub(super) fn take_failure(&mut self) -> Option<String> {
        self.failure.take()
    }

    /// Stop following: an opening in flight finishes on its own and is dropped.
    pub(super) fn clear(&mut self) {
        *self = Self::default();
    }
}
