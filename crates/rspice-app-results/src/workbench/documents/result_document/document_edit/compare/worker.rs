//! Where a comparison runs. It reads every sample of the signals it compares,
//! so it runs on a thread of its own while the form shows how far it has got,
//! and it keeps nothing once it answers: the samples it read go with it. On
//! the web, which has no threads, it runs where it is asked.
//!
//! A [`Worker`] is the whole of the arrangement: the comparison goes in when
//! it starts, progress and the answer come back through one channel, and
//! dropping it stops the comparison.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::mpsc::{self, Receiver, TryRecvError};
use std::sync::{Arc, Mutex, MutexGuard};

use rspice_results::analysis_result::AnalysisResult;
use rspice_results::waveform::RetainedWaveform;
use rspice_results::waveform_comparison::{
    ComparisonAlignmentDraft, ComparisonOptions, ComparisonOutcome, ComparisonSide,
    execute_comparison,
};

use crate::product::DatasetBinding;

/// One run's side of a comparison: the waveforms it compares, sharing their
/// samples with the run.
pub(crate) struct Side {
    pub binding: DatasetBinding,
    pub analysis: AnalysisResult<RetainedWaveform>,
}

/// A comparison a worker runs, owned whole.
pub(crate) struct Job {
    pub candidate: Side,
    pub baseline: Side,
    pub signals: Vec<String>,
    pub alignment: ComparisonAlignmentDraft,
    pub alignment_signal: String,
    pub threshold: f64,
    pub maximum_lag_samples: u32,
    pub absolute_tolerance: f64,
    pub relative_tolerance: f64,
}

impl Side {
    fn compared(&self) -> ComparisonSide<'_, RetainedWaveform> {
        ComparisonSide {
            binding: self.binding,
            analysis: &self.analysis,
        }
    }
}

impl Job {
    fn run(
        &self,
        progress: &mut dyn FnMut(usize, usize) -> bool,
    ) -> Result<ComparisonOutcome, String> {
        execute_comparison(
            self.candidate.compared(),
            self.baseline.compared(),
            &self.signals,
            ComparisonOptions {
                alignment: self.alignment,
                alignment_signal: &self.alignment_signal,
                threshold: self.threshold,
                maximum_lag_samples: self.maximum_lag_samples,
                absolute_tolerance: self.absolute_tolerance,
                relative_tolerance: self.relative_tolerance,
            },
            progress,
        )
    }
}

/// What a comparison answers: its outcome, boxed so the progress messages
/// that precede it stay small, or why it has none.
pub(crate) type Answer = Result<Box<ComparisonOutcome>, String>;

enum Message {
    Progress(usize, usize),
    Done(Answer),
}

/// Where a running comparison has got.
pub(crate) enum Poll {
    /// Signals compared, of how many.
    Running(usize, usize),
    Done(Answer),
}

/// A comparison running, or answered and not yet collected.
pub(crate) struct Worker {
    messages: Mutex<Receiver<Message>>,
    progress: Mutex<(usize, usize)>,
    stop: Arc<AtomicBool>,
}

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex
        .lock()
        .unwrap_or_else(|poisoned| poisoned.into_inner())
}

impl Worker {
    /// Start `job`.
    pub(crate) fn start(job: Job) -> Self {
        let (sender, messages) = mpsc::channel();
        let stop = Arc::new(AtomicBool::new(false));
        let total = job.signals.len();
        let work = {
            let sender = sender.clone();
            let stop = Arc::clone(&stop);
            move || {
                let mut progress = |done, total| {
                    // A form that stopped listening has no use for progress.
                    let _ = sender.send(Message::Progress(done, total));
                    !stop.load(Ordering::Relaxed)
                };
                let answer = job.run(&mut progress).map(Box::new);
                let _ = sender.send(Message::Done(answer));
            }
        };
        #[cfg(not(target_arch = "wasm32"))]
        {
            let started = std::thread::Builder::new()
                .name("results comparison".to_owned())
                .spawn(work);
            if let Err(error) = started {
                let _ = sender.send(Message::Done(Err(format!(
                    "The comparison could not start: {error}."
                ))));
            }
        }
        #[cfg(target_arch = "wasm32")]
        work();
        drop(sender);
        Self {
            messages: Mutex::new(messages),
            progress: Mutex::new((0, total)),
            stop,
        }
    }

    /// Where the comparison has got, or its answer once it has one.
    pub(crate) fn poll(&self) -> Poll {
        let messages = lock(&self.messages);
        let mut progress = lock(&self.progress);
        loop {
            match messages.try_recv() {
                Ok(Message::Progress(done, total)) => *progress = (done, total),
                Ok(Message::Done(answer)) => return Poll::Done(answer),
                Err(TryRecvError::Empty) => return Poll::Running(progress.0, progress.1),
                Err(TryRecvError::Disconnected) => {
                    return Poll::Done(Err("The comparison ended without an answer.".to_owned()));
                }
            }
        }
    }

    /// Wait for the answer.
    #[cfg(any(test, feature = "test-support"))]
    pub(crate) fn wait(&self) -> Answer {
        let messages = lock(&self.messages);
        loop {
            match messages.recv() {
                Ok(Message::Progress(done, total)) => *lock(&self.progress) = (done, total),
                Ok(Message::Done(answer)) => return answer,
                Err(_) => return Err("The comparison ended without an answer.".to_owned()),
            }
        }
    }

    /// Ask the comparison to stop after the signal it is comparing.
    pub(crate) fn stop(&self) {
        self.stop.store(true, Ordering::Relaxed);
    }
}

impl Drop for Worker {
    fn drop(&mut self) {
        self.stop();
    }
}

impl std::fmt::Debug for Worker {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (done, total) = *lock(&self.progress);
        formatter
            .debug_struct("Worker")
            .field("done", &done)
            .field("total", &total)
            .finish_non_exhaustive()
    }
}
