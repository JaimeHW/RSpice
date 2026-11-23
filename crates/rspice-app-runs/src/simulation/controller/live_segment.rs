//! The rows a running transient has written to its segment, opened again as the segment grows,
//! on a thread of their own so the frame never waits for the file.
//!
//! The live plot draws these rows as a finished run's are drawn: its columns read the segment,
//! and its tiles come from the store's summaries through the reader service. Nothing is copied
//! into the frame but the segment's last chunk, which each opening holds.
//!
//! A run can write a million columns. Whatever grows with them is built on the opening's thread
//! and let go of on a thread of its own, so the frame only moves it.

use std::any::Any;
use std::path::PathBuf;
use std::sync::mpsc::Sender;
use std::thread::JoinHandle;
use std::time::Duration;

use rspice_simulation::execution::DeckOutputs;
use rspice_simulation::live_transient::{LiveRows, open_live_rows};

use crate::state::{RetainedTrace, ShownWaveforms};
use crate::time_compat::Instant;

/// How often the segment is opened again while the run writes it.
const REOPEN_EVERY: Duration = Duration::from_millis(250);

type Opening = JoinHandle<Result<Option<(LiveRows, Option<LivePreview>)>, String>>;

/// A manual deck's live preview as one opening built it: every column as a trace, in the order
/// the finished result lists them, by name, so its panes and colours stay where they are when
/// the run ends; and the view's copy of them. It plots what the deck names, as the finished
/// result does; the rest wait in the data browser.
pub(super) struct LivePreview {
    pub(super) traces: Vec<RetainedTrace>,
    pub(super) shown: ShownWaveforms,
}

impl LivePreview {
    fn of(rows: &LiveRows, outputs: &DeckOutputs) -> Self {
        // A trace stores no colour, as a finished run's do: the palette colours it by its
        // slot, so the live plot's colours are the ones the finished result keeps.
        let mut traces = rows
            .waveforms
            .iter()
            .cloned()
            .map(|data| RetainedTrace {
                visible: outputs.plots(&data.name),
                data,
                color: String::new(),
            })
            .collect::<Vec<_>>();
        traces.sort_by(|left, right| left.name.cmp(&right.name));
        let shown = ShownWaveforms::new(traces.clone());
        Self { traces, shown }
    }
}

/// A thread that drops what it is sent: what the frame lets go of, where its size costs no
/// frame.
#[derive(Default)]
pub(super) struct Release {
    sender: Option<Sender<Box<dyn Any + Send>>>,
}

impl Release {
    pub(super) fn send(&mut self, value: impl Any + Send) {
        let mut value: Box<dyn Any + Send> = Box::new(value);
        if let Some(sender) = &self.sender {
            match sender.send(value) {
                Ok(()) => return,
                Err(refused) => value = refused.0,
            }
        }
        let (sender, received) = std::sync::mpsc::channel::<Box<dyn Any + Send>>();
        let started = std::thread::Builder::new()
            .name("rspice-release".to_owned())
            .spawn(move || received.into_iter().for_each(drop));
        // With no thread to drop it on, it is dropped here.
        if started.is_ok() && sender.send(value).is_ok() {
            self.sender = Some(sender);
        }
    }
}

#[derive(Default)]
pub(super) struct LiveSegmentFollower {
    /// The opening in flight, if one is.
    opening: Option<Opening>,
    /// When the last opening began.
    opened_at: Option<Instant>,
    /// The newest rows an opening found.
    rows: Option<LiveRows>,
    /// The preview the newest opening built, until it is taken.
    preview: Option<LivePreview>,
    /// Why the segment could not be opened, until it is said.
    failure: Option<String>,
    /// The segment could not be opened, and is not opened again for this run.
    stopped: bool,
    release: Release,
}

impl LiveSegmentFollower {
    /// Take a finished opening and begin the next one when it is due; whether the rows grew.
    ///
    /// `path` is the segment the run is writing now, `None` once it stopped writing; the rows
    /// already found stay until [`Self::clear`]. Given the deck's `outputs`, each opening also
    /// builds the live preview of every column, for [`Self::take_preview`].
    pub(super) fn poll(&mut self, path: Option<PathBuf>, outputs: Option<&DeckOutputs>) -> bool {
        let mut grew = false;
        if self.opening.as_ref().is_some_and(JoinHandle::is_finished)
            && let Some(opening) = self.opening.take()
        {
            match opening.join() {
                Ok(Ok(Some((rows, preview)))) => {
                    if self
                        .rows
                        .as_ref()
                        .is_none_or(|shown| rows.rows > shown.rows)
                    {
                        let superseded = (
                            self.rows.replace(rows),
                            std::mem::replace(&mut self.preview, preview),
                        );
                        self.release.send(superseded);
                        grew = true;
                    } else {
                        self.release.send((rows, preview));
                    }
                }
                Ok(Ok(None)) => {}
                // The run names its segment before it writes the segment's header, so until
                // rows were found the segment is opened again when due. A segment that was
                // read before and cannot be read now has failed.
                Ok(Err(_)) if self.rows.is_none() => {}
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
            let outputs = outputs.cloned();
            self.opening = std::thread::Builder::new()
                .name("rspice-live-rows".to_owned())
                .spawn(move || {
                    Ok(open_live_rows(&path)?.map(|rows| {
                        let preview = outputs.map(|outputs| LivePreview::of(&rows, &outputs));
                        (rows, preview)
                    }))
                })
                .ok();
        }
        grew
    }

    /// The newest rows found, while the run's are being followed.
    pub(super) fn rows(&self) -> Option<&LiveRows> {
        self.rows.as_ref()
    }

    /// The preview the newest opening built, once.
    pub(super) fn take_preview(&mut self) -> Option<LivePreview> {
        self.preview.take()
    }

    /// Let go of `value` where its size costs no frame.
    pub(super) fn release(&mut self, value: impl Any + Send) {
        self.release.send(value);
    }

    /// Why the segment could not be opened, once.
    pub(super) fn take_failure(&mut self) -> Option<String> {
        self.failure.take()
    }

    /// Stop following: an opening in flight finishes on its own and is dropped, on its thread.
    pub(super) fn clear(&mut self) {
        let release = std::mem::take(&mut self.release);
        let followed = std::mem::replace(
            self,
            Self {
                release,
                ..Self::default()
            },
        );
        self.release.send((followed.rows, followed.preview));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A segment the run has named but not begun writing holds no header yet: opening it is
    /// not a failure, and it is opened again when due.
    #[test]
    fn a_segment_not_yet_begun_is_opened_again_rather_than_given_up() {
        let path = std::env::temp_dir().join(format!(
            "rspice-live-rows-not-begun-{}.seg",
            std::process::id()
        ));
        std::fs::write(&path, b"").expect("an empty segment file is written");
        let mut follower = LiveSegmentFollower::default();
        follower.poll(Some(path.clone()), None);
        while follower
            .opening
            .as_ref()
            .is_some_and(|opening| !opening.is_finished())
        {
            std::thread::sleep(Duration::from_millis(5));
        }
        follower.poll(Some(path.clone()), None);
        let failure = follower.take_failure();
        let stopped = follower.stopped;
        std::thread::sleep(REOPEN_EVERY);
        follower.poll(Some(path.clone()), None);
        let opened_again = follower.opening.is_some();
        follower.clear();
        let _ = std::fs::remove_file(&path);

        assert_eq!(failure, None);
        assert!(!stopped);
        assert!(opened_again, "the segment is opened again when due");
    }
}
