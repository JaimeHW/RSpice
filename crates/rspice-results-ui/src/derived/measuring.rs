//! Stored series' extents measured away from the frame.
//!
//! A stored series' extent is read from a tile of its store, tens of
//! microseconds each. A pane of a hundred thousand such curves asked for
//! them all on the frame that plotted them, and that frame took seconds: the
//! frame measures a share of them itself and hands the rest to a thread,
//! which wakes the window when they are measured.

use rspice_results::waveform::{SharedWaveformValues, ValueExtent};
use std::collections::{HashMap, HashSet};

/// The bits of an X window, `None` for a series' whole record.
pub(super) type WindowBits = Option<(u64, u64)>;

/// A series' extent over a window, as measured.
pub(super) type Measured = (u64, WindowBits, Option<ValueExtent>);

/// The bits of `window`.
pub(super) fn bits(window: Option<(f64, f64)>) -> WindowBits {
    window.map(|(x0, x1)| (x0.to_bits(), x1.to_bits()))
}

/// One series' extent to measure: `y` against `x` over `window`.
struct Ask {
    window: Option<(f64, f64)>,
    x: SharedWaveformValues,
    y: SharedWaveformValues,
}

/// What is being measured, and where it arrives.
struct Sent {
    asks: HashSet<(u64, WindowBits)>,
    measuring: std::thread::JoinHandle<Vec<Measured>>,
}

/// The extents asked for away from the frame.
#[derive(Default)]
pub(super) struct Measuring {
    /// Asked for and not yet sent, by series key and whether it is the
    /// whole record's: the window last asked of each.
    asked: HashMap<(u64, bool), Ask>,
    sent: Option<Sent>,
    /// The window to wake when a measurement arrives.
    waker: Option<egui::Context>,
}

impl Clone for Measuring {
    /// A copy measures afresh what it is asked; it wakes the same window.
    fn clone(&self) -> Self {
        Self {
            waker: self.waker.clone(),
            ..Self::default()
        }
    }
}

impl std::fmt::Debug for Measuring {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Measuring")
            .field("asked", &self.asked.len())
            .field("sent", &self.sent.as_ref().map(|sent| sent.asks.len()))
            .finish()
    }
}

impl Measuring {
    /// Whether the extent under `key` over `window` has been asked for and
    /// not yet kept.
    pub(super) fn asked_for(&self, key: u64, window: Option<(f64, f64)>) -> bool {
        let bits = bits(window);
        self.sent
            .as_ref()
            .is_some_and(|sent| sent.asks.contains(&(key, bits)))
            || self
                .asked
                .get(&(key, window.is_none()))
                .is_some_and(|ask| self::bits(ask.window) == bits)
    }

    /// Ask for `y`'s extent against `x` over `window` under `key`, in place
    /// of the one last asked of it over a window, or over its whole record.
    pub(super) fn ask(
        &mut self,
        key: u64,
        window: Option<(f64, f64)>,
        (x, y): (&SharedWaveformValues, &SharedWaveformValues),
    ) {
        self.asked.insert(
            (key, window.is_none()),
            Ask {
                window,
                x: x.clone(),
                y: y.clone(),
            },
        );
    }

    /// Wake `ctx` when a measurement arrives.
    pub(super) fn wake(&mut self, ctx: &egui::Context) {
        if self.waker.is_none() {
            self.waker = Some(ctx.clone());
        }
    }

    /// Forget what was asked and what is being measured, as of data no
    /// longer shown.
    pub(super) fn forget(&mut self) {
        self.asked.clear();
        self.sent = None;
    }

    /// The extents measured since the last call, once all that was sent has
    /// been.
    pub(super) fn arrived(&mut self) -> Vec<Measured> {
        if !self
            .sent
            .as_ref()
            .is_some_and(|sent| sent.measuring.is_finished())
        {
            return Vec::new();
        }
        let Some(sent) = self.sent.take() else {
            return Vec::new();
        };
        match sent.measuring.join() {
            Ok(measured) => measured,
            // A measurement that failed fails the frame, as it did when the
            // frame measured.
            Err(panic) => std::panic::resume_unwind(panic),
        }
    }

    /// Send what was asked to be measured, unless something is being.
    pub(super) fn send(&mut self) {
        if self.sent.is_some() || self.asked.is_empty() {
            return;
        }
        let asks: Vec<(u64, Ask)> = self
            .asked
            .drain()
            .map(|((key, _), ask)| (key, ask))
            .collect();
        let sent = asks
            .iter()
            .map(|(key, ask)| (*key, bits(ask.window)))
            .collect();
        let waker = self.waker.clone();
        // A thread that cannot start drops what was asked; the frames that
        // still want it ask again.
        let spawned = std::thread::Builder::new()
            .name("plot extents".to_owned())
            .spawn(move || {
                let measured = measure(&asks);
                if let Some(ctx) = waker {
                    ctx.request_repaint();
                }
                measured
            });
        if let Ok(measuring) = spawned {
            self.sent = Some(Sent {
                asks: sent,
                measuring,
            });
        }
    }
}

/// Measure `asks`, shared among the machine's threads.
fn measure(asks: &[(u64, Ask)]) -> Vec<Measured> {
    let one = |(key, ask): &(u64, Ask)| {
        (
            *key,
            bits(ask.window),
            ask.y.extent_over(&ask.x, ask.window),
        )
    };
    let threads = std::thread::available_parallelism().map_or(1, std::num::NonZero::get);
    let share = asks.len().div_ceil(threads).max(1);
    std::thread::scope(|scope| {
        let shares: Vec<_> = asks
            .chunks(share)
            .map(|part| {
                let spawned = std::thread::Builder::new()
                    .name("plot extents".to_owned())
                    .spawn_scoped(scope, move || part.iter().map(one).collect::<Vec<_>>());
                // A share no thread could take is measured here.
                spawned.map_err(|_| part.iter().map(one).collect::<Vec<_>>())
            })
            .collect();
        shares
            .into_iter()
            .flat_map(|share| match share {
                Ok(thread) => thread
                    .join()
                    .unwrap_or_else(|panic| std::panic::resume_unwind(panic)),
                Err(measured) => measured,
            })
            .collect()
    })
}
