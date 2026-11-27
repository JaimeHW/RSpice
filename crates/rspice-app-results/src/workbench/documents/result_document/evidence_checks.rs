//! Validations of retained evidence too large for a frame, taken away from it.
//!
//! The validator reads every sample of every signal an analysis holds in memory, as a run the
//! results store refused holds them. An analysis of a few million samples is validated on a
//! thread of its own and reads as still being read until the verdict is in, as a stored one does
//! while the reader service reads it; a smaller one, and any in the browser, which has one
//! thread, is validated where it is asked. A gesture that reads what is shown, an export or a
//! copy, waits for the verdicts it needs ([`EvidenceChecks::wait_here`]).

use std::cell::RefCell;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};

use super::{AnalysisPresentationKey, frame_work};
use crate::state::{AnalysisResult, RunHistoryRevision, SimulationState};

/// How many samples held in memory a frame validates itself: about twenty milliseconds of an
/// unoptimized build's frame. A larger analysis is validated away from the frame.
const FRAME_VALIDATION: usize = if cfg!(target_arch = "wasm32") {
    usize::MAX
} else {
    2_000_000
};

type Verdict = Result<(), Arc<str>>;

/// The validations under way away from the frame, each of an analysis as the retained runs
/// stood when it started.
#[derive(Default)]
pub(crate) struct EvidenceChecks {
    running: RefCell<Vec<Check>>,
    /// The window to wake when a validation ends.
    waker: RefCell<Option<egui::Context>>,
    /// How many gestures under way wait for their verdicts ([`Self::wait_here`]).
    waits: Arc<AtomicUsize>,
}

/// While held, verdicts are waited for where they are asked.
pub(crate) struct WaitHere(Arc<AtomicUsize>);

impl Drop for WaitHere {
    fn drop(&mut self) {
        self.0.fetch_sub(1, Ordering::Relaxed);
    }
}

struct Check {
    analysis: AnalysisPresentationKey,
    source: RunHistoryRevision,
    verdict: std::thread::JoinHandle<Verdict>,
}

impl EvidenceChecks {
    /// The validator's verdict on `analysis`, the retained analysis `key` names: taken here when
    /// a frame can afford it, else away from the frame, the verdict reading as still being read
    /// ([`rspice_results::waveform::is_pending_read`]) until it is in.
    pub(super) fn verdict(
        &self,
        simulation: &SimulationState,
        key: AnalysisPresentationKey,
        analysis: &AnalysisResult,
    ) -> Verdict {
        if held_in_memory(analysis) <= FRAME_VALIDATION {
            return validated_here(analysis);
        }
        let waits = self.waits.load(Ordering::Relaxed) > 0;
        let source = simulation.retained.runs.revision();
        let mut running = self.running.borrow_mut();
        // A check of the runs as they stood before is let go; its thread ends on its own.
        running.retain(|check| check.source == source);
        if let Some(place) = running.iter().position(|check| check.analysis == key) {
            if !waits && !running[place].verdict.is_finished() {
                return Err(rspice_results::waveform::BEING_READ.into());
            }
            return running
                .swap_remove(place)
                .verdict
                .join()
                .unwrap_or_else(|_| validated_here(analysis));
        }
        if waits {
            return validated_here(analysis);
        }
        // The copy shares the samples: each series is one shared array.
        let mut copy = analysis.data.without_waveforms();
        copy.waveforms = analysis
            .data
            .waveforms
            .iter()
            .map(|trace| trace.data.clone())
            .collect();
        let waker = self.waker.borrow().clone();
        let spawned = std::thread::Builder::new()
            .name("validate evidence".to_owned())
            .spawn(move || {
                let verdict = copy.validate_retained_evidence().map_err(Arc::<str>::from);
                if let Some(ctx) = waker {
                    ctx.request_repaint();
                }
                verdict
            });
        match spawned {
            Ok(verdict) => {
                running.push(Check {
                    analysis: key,
                    source,
                    verdict,
                });
                Err(rspice_results::waveform::BEING_READ.into())
            }
            // A thread that cannot start validates nothing: the frame does.
            Err(_) => validated_here(analysis),
        }
    }

    /// Wait for the verdicts asked for while the guard is held: a gesture that reads what is
    /// shown takes them then, waiting for a validation under way, rather than finding an
    /// analysis still being read.
    pub(crate) fn wait_here(&self) -> WaitHere {
        self.waits.fetch_add(1, Ordering::Relaxed);
        WaitHere(Arc::clone(&self.waits))
    }

    /// Whether a validation is still under way away from the frame.
    pub(crate) fn checking(&self) -> bool {
        self.running
            .borrow()
            .iter()
            .any(|check| !check.verdict.is_finished())
    }

    /// Wake the window through `ctx` when a validation ends.
    pub(crate) fn wake_through(&self, ctx: &egui::Context) {
        let mut waker = self.waker.borrow_mut();
        if waker.is_none() {
            *waker = Some(ctx.clone());
        }
    }
}

impl Clone for EvidenceChecks {
    /// A copy validates what it needs itself.
    fn clone(&self) -> Self {
        Self::default()
    }
}

impl std::fmt::Debug for EvidenceChecks {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("EvidenceChecks")
            .field("running", &self.running.borrow().len())
            .finish()
    }
}

fn validated_here(analysis: &AnalysisResult) -> Verdict {
    frame_work::note(frame_work::DatasetWalk::EvidenceValidation);
    analysis
        .data
        .validate_retained_evidence()
        .map_err(Into::into)
}

/// The samples `analysis` holds in memory, which the validator reads on the thread that asks.
fn held_in_memory(analysis: &AnalysisResult) -> usize {
    let held = |values: &rspice_results::waveform::WaveformValues| {
        values.in_memory().map_or(0, <[f64]>::len)
    };
    analysis
        .data
        .waveforms
        .iter()
        .map(|trace| {
            let complex = trace
                .data
                .complex
                .iter()
                .map(|complex| held(&complex.real) + held(&complex.imag))
                .sum::<usize>();
            held(&trace.data.x) + held(&trace.data.y) + complex
        })
        .sum()
}
