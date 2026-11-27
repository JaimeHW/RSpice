//! In-memory axes classified away from the frame.
//!
//! A curve's shape is one pass over its X axis, about ten nanoseconds a sample unoptimized. A run
//! held in memory with an axis of a million samples to each of its signals spent the frame that
//! plotted it classifying them: a build classifies the axes it meets itself until it has passed
//! over [`FRAME_AXIS_SAMPLES`], and hands the rest to a thread, which wakes the window when each
//! is classified. Their curves stand empty until a build after that, as the curves on a stored
//! axis do while the reader service reads it.

use rspice_results::sampling::SweepShape;
use rspice_results::waveform::{SharedWaveformValues, SharingKey};
use std::collections::HashMap;
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::{Arc, OnceLock};

/// How many samples of in-memory axes one build of the strips classifies itself: about a
/// hundredth of a second of them unoptimized. The page has one thread, and classifies them all.
pub const FRAME_AXIS_SAMPLES: usize = if cfg!(target_arch = "wasm32") {
    usize::MAX
} else {
    1 << 20
};

/// How many axes are being classified away from the frame.
static UNDER_WAY: AtomicUsize = AtomicUsize::new(0);

/// Whether an axis is being classified away from the frame.
pub fn classifying_off_frame() -> bool {
    UNDER_WAY.load(Ordering::SeqCst) > 0
}

/// One axis classified away from the frame: its shape once it has arrived.
#[derive(Debug, Clone)]
pub struct Classification(Arc<OnceLock<Arc<SweepShape>>>);

impl Classification {
    /// Whether the axis's shape has arrived.
    pub fn arrived(&self) -> bool {
        self.0.get().is_some()
    }
}

/// The in-memory axes handed away from the frame, each with the axis, held so that its key names
/// it while it is classified.
#[derive(Clone, Default)]
pub(super) struct Classifying {
    axes: HashMap<SharingKey, (SharedWaveformValues, Classification)>,
    /// The window to wake when an axis's shape arrives, shared with the threads classifying: a
    /// build asks before the frame that draws its strips names the window.
    waker: Arc<OnceLock<egui::Context>>,
}

impl std::fmt::Debug for Classifying {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Classifying")
            .field("axes", &self.axes.len())
            .finish()
    }
}

impl Classifying {
    /// Wake `ctx` when an axis's shape arrives.
    pub(super) fn wake(&mut self, ctx: &egui::Context) {
        let _ = self.waker.get_or_init(|| ctx.clone());
    }

    /// The shape of the in-memory axis `x` when it has been classified away from the frame.
    pub(super) fn arrived(&self, x: &SharedWaveformValues) -> Option<Arc<SweepShape>> {
        let (_, classification) = self.axes.get(&x.sharing_key())?;
        classification.0.get().cloned()
    }

    /// Hand `x` to a thread to classify, unless it already is: `false` when no thread could take
    /// it, for the frame to classify it itself.
    pub(super) fn ask(&mut self, x: &SharedWaveformValues) -> bool {
        let key = x.sharing_key();
        if self.axes.contains_key(&key) {
            return true;
        }
        let held = Arc::new(OnceLock::new());
        let (axis, arrived, waker) = (x.clone(), Arc::clone(&held), Arc::clone(&self.waker));
        UNDER_WAY.fetch_add(1, Ordering::SeqCst);
        let spawned = std::thread::Builder::new()
            .name("classify axis".to_owned())
            .spawn(move || {
                let shape = SweepShape::of(axis.in_memory().unwrap_or_default());
                let _ = arrived.set(Arc::new(shape));
                UNDER_WAY.fetch_sub(1, Ordering::SeqCst);
                if let Some(ctx) = waker.get() {
                    ctx.request_repaint();
                }
            });
        if spawned.is_err() {
            UNDER_WAY.fetch_sub(1, Ordering::SeqCst);
            return false;
        }
        self.axes.insert(key, (x.clone(), Classification(held)));
        true
    }

    /// The axes still being classified.
    pub(super) fn under_way(&self) -> Vec<Classification> {
        self.axes
            .values()
            .map(|(_, classification)| classification)
            .filter(|classification| !classification.arrived())
            .cloned()
            .collect()
    }

    /// Let go of the axes already classified, as of data that changed: an axis still shown is
    /// classified again, and one still being classified is kept for the build after it arrives.
    pub(super) fn keep_under_way(&mut self) {
        self.axes
            .retain(|_, (_, classification)| !classification.arrived());
    }
}
