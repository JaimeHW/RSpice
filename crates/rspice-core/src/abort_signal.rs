//! Abort Signal for Long-Running Simulations
//!
//! This module provides a cooperative cancellation mechanism for simulations.
//! Commercial SPICE simulators like Cadence Spectre support simulation abort
//! to allow users to stop long-running analyses without killing the process.
//!
//! # Architecture
//!
//! The [`AbortSignal`] trait provides a clean abstraction that:
//! - Keeps the engine independent of UI threading primitives
//! - Enables easy testing with mock abort signals
//! - Allows different abort implementations (atomic, channel-based, etc.)
//!
//! # Usage
//!
//! ```rust
//! use rspice_core::abort_signal::{AbortSignal, NoAbort};
//!
//! // For non-cancellable runs or testing
//! let signal = NoAbort;
//! assert!(!signal.is_aborted());
//!
//! // The engine polls on a fixed stride during long loops
//! // if signal.is_aborted() { return Err(...); }
//! ```
//!
//! # Bounded-polling policy
//!
//! Cancellation latency is a contract, not a best effort: a caller that
//! requests a stop must observe it within a bounded amount of solver work,
//! whatever the deck does. Every long-running loop therefore polls on a fixed
//! stride rather than at whatever boundary happened to be convenient:
//!
//! - Natural per-point boundaries — each frequency, sweep point, Monte Carlo
//!   trial, `.STEP`/`.TEMP` coordinate, transform bin, or serialized record —
//!   are polled every iteration or on a small fixed stride.
//! - Inner Newton-Raphson iterations *are* polled, every
//!   `ABORT_CHECK_INTERVAL` (16) iterations, and so is the transient
//!   step-attempt loop. A stiff deck can spend minutes inside one timepoint's
//!   Newton solve; leaving that loop unpolled made the observable latency
//!   unbounded, which is why the older "never poll inside Newton" rule was
//!   removed.
//! - Nothing polls per residual evaluation or per matrix entry. The stride is
//!   chosen so the check is far cheaper than the work between two checks
//!   while still bounding latency by a countable number of iterations.
//!
//! [`CountingAbort`] exists to hold this to an exact number: a test sets a
//! poll threshold, runs the analysis, and asserts both that the analysis
//! stopped and at which poll it did so.

use std::io::{self, Read};
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

//=============================================================================
// TransientSample - the observation payload
//=============================================================================

/// One accepted transient sample, borrowed from the analysis result.
///
/// A transient result is stored column-major: `time` gains one entry per
/// accepted sample and every retained waveform column grows with it, so the
/// sample being reported is the last entry of each column. A column that
/// output projection deliberately did not retain stays empty.
///
/// This is a view, not a copy. The engine names it instead of handing over its
/// own result type both because the abort layer sits below the analyses and
/// because borrowing is what keeps the hook inert: a non-interactive run pays
/// no allocation to report a sample nobody reads.
#[derive(Debug, Clone, Copy)]
pub struct TransientSample<'a> {
    /// Accepted time points. The reported sample is the last entry.
    pub time: &'a [crate::Value],
    /// Node names, positionally aligned with `node_voltages`.
    pub node_names: &'a [String],
    /// Retained node-voltage columns, indexed `[node][sample]`.
    pub node_voltages: &'a [Vec<crate::Value>],
    /// Branch names, positionally aligned with `branch_currents`.
    pub branch_names: &'a [String],
    /// Retained branch-current columns, indexed `[branch][sample]`.
    pub branch_currents: &'a [Vec<crate::Value>],
}

//=============================================================================
// AbortSignal Trait
//=============================================================================

/// Trait for checking abort signals during long-running simulation operations.
///
/// This abstraction allows the simulation engine to be:
/// - Testable without threading dependencies
/// - Flexible in abort mechanism (atomic, channel, mock, etc.)
/// - Independent of the UI layer
///
/// # Implementors
///
/// - [`NoAbort`]: Never aborts (for tests and non-cancellable runs)
/// - [`AtomicAbort`]: Atomic bool-based abort (for thread-safe UI integration)
/// - [`ImmediateAbort`]: Always aborts (for testing abort paths)
pub trait AbortSignal: Send + Sync {
    /// Check if an abort has been requested.
    ///
    /// Returns `true` if the simulation should stop immediately.
    /// Implementations must be thread-safe and lock-free for performance.
    fn is_aborted(&self) -> bool;

    /// Observe analysis progress as a completed fraction in `[0, 1]`.
    ///
    /// The engine reports at the same low cadence it polls for aborts, so
    /// implementations may update UI state directly. The default ignores
    /// progress, keeping existing implementors unchanged.
    fn observe_progress(&self, _fraction: f64) {}

    /// Observe one fully accepted transient sample.
    ///
    /// This hook runs only after the sample has been committed to the
    /// analysis result. Rejected Newton or timestep attempts never cross this
    /// boundary. The default is intentionally inert so non-interactive
    /// engine users pay no allocation or transport cost.
    fn observe_transient_sample(&self, _sample: TransientSample<'_>) {}
}

//=============================================================================
// NoAbort - Default No-Op Implementation
//=============================================================================

/// A no-op abort signal that never aborts.
///
/// Use this for:
/// - Unit tests that don't need abort functionality
/// - Non-interactive batch simulations
/// - Backward compatibility with existing engine APIs
#[derive(Debug, Clone, Copy, Default)]
pub struct NoAbort;

impl AbortSignal for NoAbort {
    #[inline(always)]
    fn is_aborted(&self) -> bool {
        false
    }
}

//=============================================================================
// ImmediateAbort - Test Helper
//=============================================================================

/// An abort signal that always returns true.
///
/// Useful for testing that abort paths are correctly implemented.
#[derive(Debug, Clone, Copy, Default)]
pub struct ImmediateAbort;

impl AbortSignal for ImmediateAbort {
    #[inline(always)]
    fn is_aborted(&self) -> bool {
        true
    }
}

//=============================================================================
// AtomicAbort - Thread-Safe Implementation
//=============================================================================

/// Thread-safe abort signal using atomic operations.
///
/// This is the primary implementation for UI integration where the abort
/// flag is set from a UI thread and checked from a simulation thread.
///
/// # Example
///
/// ```rust
/// use rspice_core::abort_signal::{AtomicAbort, AbortSignal};
/// use std::sync::Arc;
///
/// let abort = Arc::new(AtomicAbort::new());
/// let abort_clone = Arc::clone(&abort);
///
/// // UI thread sets abort
/// abort_clone.set();
///
/// // Simulation thread checks abort
/// assert!(abort.is_aborted());
/// ```
#[derive(Debug, Default)]
pub struct AtomicAbort {
    aborted: AtomicBool,
}

impl AtomicAbort {
    /// Create a new abort signal in the non-aborted state.
    #[inline]
    pub fn new() -> Self {
        Self {
            aborted: AtomicBool::new(false),
        }
    }

    /// Request abort of the simulation.
    ///
    /// This is typically called from the UI thread when the user clicks stop.
    /// Uses `SeqCst` ordering for maximum visibility across threads.
    #[inline]
    pub fn set(&self) {
        self.aborted.store(true, Ordering::SeqCst);
    }

    /// Clear the abort flag.
    ///
    /// Call this before starting a new simulation to reset state.
    #[inline]
    pub fn clear(&self) {
        self.aborted.store(false, Ordering::SeqCst);
    }
}

impl AbortSignal for AtomicAbort {
    #[inline(always)]
    fn is_aborted(&self) -> bool {
        self.aborted.load(Ordering::SeqCst)
    }
}

/// Share one flag across threads without borrowing it.
impl AbortSignal for Arc<AtomicAbort> {
    #[inline(always)]
    fn is_aborted(&self) -> bool {
        self.aborted.load(Ordering::SeqCst)
    }
}

/// Drive aborts from a plain `std` atomic a frontend already owns.
impl AbortSignal for Arc<AtomicBool> {
    #[inline(always)]
    fn is_aborted(&self) -> bool {
        self.load(Ordering::SeqCst)
    }
}

/// Forward through a borrow so `&dyn AbortSignal` satisfies the trait too.
impl<T: AbortSignal + ?Sized> AbortSignal for &T {
    #[inline(always)]
    fn is_aborted(&self) -> bool {
        (*self).is_aborted()
    }

    fn observe_progress(&self, fraction: f64) {
        (*self).observe_progress(fraction);
    }

    fn observe_transient_sample(&self, sample: TransientSample<'_>) {
        (*self).observe_transient_sample(sample);
    }
}

/// Accept an owned, type-erased signal.
impl AbortSignal for Box<dyn AbortSignal> {
    #[inline(always)]
    fn is_aborted(&self) -> bool {
        self.as_ref().is_aborted()
    }

    fn observe_progress(&self, fraction: f64) {
        self.as_ref().observe_progress(fraction);
    }

    fn observe_transient_sample(&self, sample: TransientSample<'_>) {
        self.as_ref().observe_transient_sample(sample);
    }
}

/// Accept a shared, type-erased signal.
impl AbortSignal for Arc<dyn AbortSignal> {
    #[inline(always)]
    fn is_aborted(&self) -> bool {
        self.as_ref().is_aborted()
    }

    fn observe_progress(&self, fraction: f64) {
        self.as_ref().observe_progress(fraction);
    }

    fn observe_transient_sample(&self, sample: TransientSample<'_>) {
        self.as_ref().observe_transient_sample(sample);
    }
}

//=============================================================================
// CountingAbort - Test Helper for Verification
//=============================================================================

/// An abort signal that counts check invocations before aborting.
///
/// The first `threshold` polls report "keep going"; poll number
/// `threshold + 1` is the first that reports an abort. That makes the
/// iteration bound of an analysis exactly measurable: run it under a
/// `CountingAbort`, then assert both that it stopped and that
/// [`count`](Self::count) equals the poll at which it should have noticed.
/// A run that keeps working past its cancellation shows up as a poll count
/// larger than `threshold + 1`; one that never polls at all shows up as
/// `threshold`-or-fewer polls with a successful result.
#[derive(Debug)]
pub struct CountingAbort {
    /// Number of checks answered "not aborted" before the first abort.
    threshold: usize,
    /// Number of checks performed so far.
    count: std::sync::atomic::AtomicUsize,
}

impl CountingAbort {
    /// Create a counting abort whose `threshold + 1`-th poll is the first to
    /// report an abort.
    pub fn new(threshold: usize) -> Self {
        Self {
            threshold,
            count: std::sync::atomic::AtomicUsize::new(0),
        }
    }

    /// Total number of polls performed so far.
    pub fn count(&self) -> usize {
        self.count.load(Ordering::SeqCst)
    }

    /// The exact one-based poll index at which cancellation was first
    /// reported, or `None` when the signal never reached its threshold.
    ///
    /// With a threshold of `n`, an analysis that stops the instant it is told
    /// to leaves `count() == n + 1`, so `observed_at() == Some(n + 1)` and
    /// [`polls_after_abort`](Self::polls_after_abort) is zero.
    pub fn observed_at(&self) -> Option<usize> {
        (self.count() > self.threshold).then_some(self.threshold + 1)
    }

    /// Number of polls performed after cancellation was first reported.
    ///
    /// Zero means the analysis returned without polling again, which is the
    /// contract every bounded loop is held to.
    pub fn polls_after_abort(&self) -> usize {
        self.count().saturating_sub(self.threshold + 1)
    }
}

impl AbortSignal for CountingAbort {
    fn is_aborted(&self) -> bool {
        let previous = self.count.fetch_add(1, Ordering::SeqCst);
        previous >= self.threshold
    }
}

//=============================================================================
// AbortReader - cancellable, optionally capped stream reads
//=============================================================================

/// Wraps a [`Read`] so a cancellation request interrupts a long read, with an
/// optional hard cap on how many bytes may be produced.
///
/// Reading a model library, a Verilog-A source tree, or a cache record is
/// unbounded work driven by authored input, so it has to observe cancellation
/// like any solver loop. It cannot use the solver's stride: one `read` call
/// can be arbitrarily slow, so this polls once per call, which is the finest
/// boundary a reader has.
///
/// A cancelled read fails with [`io::ErrorKind::Interrupted`] carrying
/// `message`; a read that would exceed `byte_cap` fails with
/// [`io::ErrorKind::Other`]. Both are indistinguishable from an ordinary I/O
/// failure once a decoder such as `serde_json` has wrapped them, so callers
/// that need to tell the two apart — and tell either apart from malformed
/// content — inspect [`was_cancelled`](Self::was_cancelled) and
/// [`exceeded_cap`](Self::exceeded_cap) afterwards.
pub struct AbortReader<'a, R> {
    inner: R,
    abort: &'a dyn AbortSignal,
    message: &'static str,
    byte_cap: Option<usize>,
    bytes_read: usize,
    cancelled: bool,
    exceeded: bool,
}

impl<'a, R> AbortReader<'a, R> {
    /// Poll `abort` before each read, with no byte cap of this reader's own.
    ///
    /// Use this when an outer helper already enforces the size limit.
    pub fn new(inner: R, abort: &'a dyn AbortSignal, message: &'static str) -> Self {
        Self {
            inner,
            abort,
            message,
            byte_cap: None,
            bytes_read: 0,
            cancelled: false,
            exceeded: false,
        }
    }

    /// Poll `abort` before each read and refuse to produce more than
    /// `byte_cap` bytes.
    ///
    /// A stream that ends exactly at the cap succeeds; one byte more fails.
    pub fn with_byte_cap(
        inner: R,
        abort: &'a dyn AbortSignal,
        message: &'static str,
        byte_cap: usize,
    ) -> Self {
        Self {
            byte_cap: Some(byte_cap),
            ..Self::new(inner, abort, message)
        }
    }

    /// Whether a read failed because cancellation was requested.
    pub const fn was_cancelled(&self) -> bool {
        self.cancelled
    }

    /// Whether a read failed because the byte cap would have been exceeded.
    pub const fn exceeded_cap(&self) -> bool {
        self.exceeded
    }

    /// Bytes handed to the caller so far.
    pub const fn bytes_read(&self) -> usize {
        self.bytes_read
    }
}

impl<R: Read> Read for AbortReader<'_, R> {
    fn read(&mut self, buffer: &mut [u8]) -> io::Result<usize> {
        if self.abort.is_aborted() {
            self.cancelled = true;
            return Err(io::Error::new(io::ErrorKind::Interrupted, self.message));
        }
        let Some(cap) = self.byte_cap else {
            return self.inner.read(buffer);
        };

        let remaining = cap.saturating_sub(self.bytes_read);
        if remaining == 0 {
            // Distinguish "the stream ended exactly at the cap" from "the
            // stream is longer than the cap" by asking for one more byte.
            let mut probe = [0_u8; 1];
            if self.inner.read(&mut probe)? == 0 {
                return Ok(0);
            }
            self.exceeded = true;
            return Err(io::Error::other(format!(
                "{} exceeds the {cap} byte read limit",
                self.message
            )));
        }

        let readable = remaining.min(buffer.len());
        let read = self.inner.read(&mut buffer[..readable])?;
        self.bytes_read = self.bytes_read.saturating_add(read);
        Ok(read)
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn counting_abort_reports_the_exact_poll_that_first_cancelled() {
        let abort = CountingAbort::new(3);
        assert_eq!(abort.observed_at(), None);
        for expected in 1..=3 {
            assert!(!abort.is_aborted());
            assert_eq!(abort.count(), expected);
            assert_eq!(abort.observed_at(), None);
        }
        assert!(abort.is_aborted());
        assert_eq!(abort.observed_at(), Some(4));
        assert_eq!(abort.polls_after_abort(), 0);

        assert!(abort.is_aborted());
        assert_eq!(abort.polls_after_abort(), 1);
    }

    #[test]
    fn counting_abort_with_zero_threshold_cancels_on_its_first_poll() {
        let abort = CountingAbort::new(0);
        assert!(abort.is_aborted());
        assert_eq!(abort.observed_at(), Some(1));
        assert_eq!(abort.polls_after_abort(), 0);
    }

    #[test]
    fn abort_reader_passes_bytes_through_until_cancellation() {
        let abort = CountingAbort::new(1);
        let mut reader = AbortReader::new(&b"abcdef"[..], &abort, "test stream");
        let mut first = [0_u8; 3];
        assert_eq!(reader.read(&mut first).expect("first read"), 3);
        assert_eq!(&first, b"abc");

        let error = reader
            .read(&mut first)
            .expect_err("the second read is cancelled");
        assert_eq!(error.kind(), io::ErrorKind::Interrupted);
        assert!(reader.was_cancelled());
        assert!(!reader.exceeded_cap());
    }

    #[test]
    fn abort_reader_byte_cap_accepts_the_boundary_and_refuses_one_more() {
        let exact = CountingAbort::new(usize::MAX);
        let mut reader = AbortReader::with_byte_cap(&b"abcd"[..], &exact, "test stream", 4);
        let mut sink = Vec::new();
        reader
            .read_to_end(&mut sink)
            .expect("stream ends at the cap");
        assert_eq!(sink, b"abcd");
        assert_eq!(reader.bytes_read(), 4);
        assert!(!reader.exceeded_cap());

        let over = CountingAbort::new(usize::MAX);
        let mut reader = AbortReader::with_byte_cap(&b"abcde"[..], &over, "test stream", 4);
        let mut sink = Vec::new();
        let error = reader
            .read_to_end(&mut sink)
            .expect_err("one byte past the cap fails");
        assert_eq!(error.kind(), io::ErrorKind::Other);
        assert!(reader.exceeded_cap());
        assert!(!reader.was_cancelled());
    }

    #[test]
    fn abort_reader_reports_cancellation_before_the_cap() {
        let abort = ImmediateAbort;
        let mut reader = AbortReader::with_byte_cap(&b"abcd"[..], &abort, "test stream", 1);
        let mut sink = Vec::new();
        let error = reader
            .read_to_end(&mut sink)
            .expect_err("cancellation precedes the cap");
        assert_eq!(error.kind(), io::ErrorKind::Interrupted);
        assert!(reader.was_cancelled());
        assert!(!reader.exceeded_cap());
        assert_eq!(reader.bytes_read(), 0);
    }
}
