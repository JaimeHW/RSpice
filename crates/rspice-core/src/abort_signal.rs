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
//! // The engine checks periodically during long loops
//! // if signal.is_aborted() { return Err(...); }
//! ```
//!
//! # Performance Considerations
//!
//! To minimize overhead, abort should be checked:
//! - Every N iterations (e.g., 1000) in tight loops
//! - At natural boundaries (each frequency point, sweep point, etc.)
//! - Never inside inner Newton-Raphson iterations (too frequent)

use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};

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

/// Implement AbortSignal for Arc<AtomicAbort> for convenience
impl AbortSignal for Arc<AtomicAbort> {
    #[inline(always)]
    fn is_aborted(&self) -> bool {
        self.aborted.load(Ordering::SeqCst)
    }
}

/// Implement AbortSignal for Arc<AtomicBool> for direct use with std atomics
impl AbortSignal for Arc<AtomicBool> {
    #[inline(always)]
    fn is_aborted(&self) -> bool {
        self.load(Ordering::SeqCst)
    }
}

/// Implement AbortSignal for references to trait objects
impl<T: AbortSignal + ?Sized> AbortSignal for &T {
    #[inline(always)]
    fn is_aborted(&self) -> bool {
        (*self).is_aborted()
    }
}

/// Implement AbortSignal for Box<dyn AbortSignal>
impl AbortSignal for Box<dyn AbortSignal> {
    #[inline(always)]
    fn is_aborted(&self) -> bool {
        self.as_ref().is_aborted()
    }
}

/// Implement AbortSignal for Arc<dyn AbortSignal>
impl AbortSignal for Arc<dyn AbortSignal> {
    #[inline(always)]
    fn is_aborted(&self) -> bool {
        self.as_ref().is_aborted()
    }
}

//=============================================================================
// CountingAbort - Test Helper for Verification
//=============================================================================

/// An abort signal that counts check invocations before aborting.
///
/// Useful for testing that:
/// - Abort is checked at appropriate intervals
/// - Simulation terminates promptly after abort
#[derive(Debug)]
pub struct CountingAbort {
    /// Number of checks before returning true
    threshold: usize,
    /// Current check count
    count: std::sync::atomic::AtomicUsize,
}

impl CountingAbort {
    /// Create a counting abort that returns true after `threshold` checks.
    pub fn new(threshold: usize) -> Self {
        Self {
            threshold,
            count: std::sync::atomic::AtomicUsize::new(0),
        }
    }

    /// Get the current check count
    pub fn count(&self) -> usize {
        self.count.load(Ordering::SeqCst)
    }
}

impl AbortSignal for CountingAbort {
    fn is_aborted(&self) -> bool {
        let count = self.count.fetch_add(1, Ordering::SeqCst);
        count >= self.threshold
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    //=========================================================================
    // NoAbort Tests
    //=========================================================================

    #[test]
    fn test_no_abort_always_returns_false() {
        let signal = NoAbort;
        for _ in 0..1000 {
            assert!(!signal.is_aborted());
        }
    }

    #[test]
    fn test_no_abort_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<NoAbort>();
    }

    #[test]
    fn test_no_abort_default() {
        let signal = NoAbort::default();
        assert!(!signal.is_aborted());
    }

    #[test]
    fn test_no_abort_clone_copy() {
        let signal = NoAbort;
        let cloned = signal;
        assert!(!cloned.is_aborted());
    }

    //=========================================================================
    // ImmediateAbort Tests
    //=========================================================================

    #[test]
    fn test_immediate_abort_always_returns_true() {
        let signal = ImmediateAbort;
        for _ in 0..1000 {
            assert!(signal.is_aborted());
        }
    }

    #[test]
    fn test_immediate_abort_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<ImmediateAbort>();
    }

    //=========================================================================
    // AtomicAbort Tests
    //=========================================================================

    #[test]
    fn test_atomic_abort_new_is_not_aborted() {
        let signal = AtomicAbort::new();
        assert!(!signal.is_aborted());
    }

    #[test]
    fn test_atomic_abort_set_makes_aborted() {
        let signal = AtomicAbort::new();
        assert!(!signal.is_aborted());
        signal.set();
        assert!(signal.is_aborted());
    }

    #[test]
    fn test_atomic_abort_clear_resets() {
        let signal = AtomicAbort::new();
        signal.set();
        assert!(signal.is_aborted());
        signal.clear();
        assert!(!signal.is_aborted());
    }

    #[test]
    fn test_atomic_abort_multiple_sets_idempotent() {
        let signal = AtomicAbort::new();
        signal.set();
        signal.set();
        signal.set();
        assert!(signal.is_aborted());
    }

    #[test]
    fn test_atomic_abort_cross_thread() {
        let signal = Arc::new(AtomicAbort::new());
        let signal_clone = Arc::clone(&signal);

        // Spawn thread that will set abort after delay
        let handle = thread::spawn(move || {
            thread::sleep(Duration::from_millis(10));
            signal_clone.set();
        });

        // Main thread should see the abort
        while !signal.is_aborted() {
            thread::sleep(Duration::from_millis(1));
        }

        handle.join().unwrap();
        assert!(signal.is_aborted());
    }

    #[test]
    fn test_atomic_abort_is_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<AtomicAbort>();
    }

    #[test]
    fn test_atomic_abort_default() {
        let signal = AtomicAbort::default();
        assert!(!signal.is_aborted());
    }

    //=========================================================================
    // Arc<AtomicAbort> Tests
    //=========================================================================

    #[test]
    fn test_arc_atomic_abort_trait_impl() {
        let signal = Arc::new(AtomicAbort::new());
        assert!(!signal.is_aborted());
        signal.set();
        assert!(signal.is_aborted());
    }

    //=========================================================================
    // Arc<AtomicBool> Tests
    //=========================================================================

    #[test]
    fn test_arc_atomic_bool_trait_impl() {
        let signal = Arc::new(AtomicBool::new(false));
        assert!(!signal.is_aborted());
        signal.store(true, Ordering::SeqCst);
        assert!(signal.is_aborted());
    }

    //=========================================================================
    // CountingAbort Tests
    //=========================================================================

    #[test]
    fn test_counting_abort_aborts_at_threshold() {
        let signal = CountingAbort::new(5);

        // First 5 checks return false
        for i in 0..5 {
            assert!(!signal.is_aborted(), "Check {} should not abort", i);
        }

        // 6th check returns true
        assert!(signal.is_aborted());
    }

    #[test]
    fn test_counting_abort_counts_correctly() {
        let signal = CountingAbort::new(100);

        for i in 0..10 {
            let _ = signal.is_aborted();
            assert_eq!(signal.count(), i + 1);
        }
    }

    #[test]
    fn test_counting_abort_zero_threshold() {
        let signal = CountingAbort::new(0);
        // Should abort immediately
        assert!(signal.is_aborted());
    }

    #[test]
    fn test_counting_abort_one_threshold() {
        let signal = CountingAbort::new(1);
        // First check returns false
        assert!(!signal.is_aborted());
        // Second check returns true
        assert!(signal.is_aborted());
    }

    //=========================================================================
    // Trait Object Tests
    //=========================================================================

    #[test]
    fn test_trait_object_no_abort() {
        let signal: &dyn AbortSignal = &NoAbort;
        assert!(!signal.is_aborted());
    }

    #[test]
    fn test_trait_object_immediate_abort() {
        let signal: &dyn AbortSignal = &ImmediateAbort;
        assert!(signal.is_aborted());
    }

    #[test]
    fn test_trait_object_atomic_abort() {
        let atomic = AtomicAbort::new();
        let signal: &dyn AbortSignal = &atomic;
        assert!(!signal.is_aborted());
        atomic.set();
        assert!(signal.is_aborted());
    }

    #[test]
    fn test_boxed_trait_object() {
        let signal: Box<dyn AbortSignal> = Box::new(NoAbort);
        assert!(!signal.is_aborted());

        let signal: Box<dyn AbortSignal> = Box::new(ImmediateAbort);
        assert!(signal.is_aborted());
    }

    #[test]
    fn test_arc_trait_object() {
        let signal: Arc<dyn AbortSignal> = Arc::new(NoAbort);
        assert!(!signal.is_aborted());

        let signal: Arc<dyn AbortSignal> = Arc::new(ImmediateAbort);
        assert!(signal.is_aborted());
    }

    //=========================================================================
    // Reference Implementation Tests
    //=========================================================================

    #[test]
    fn test_reference_impl() {
        let no_abort = NoAbort;
        let signal: &dyn AbortSignal = &no_abort;
        assert!(!signal.is_aborted());
    }

    //=========================================================================
    // Performance Tests
    //=========================================================================

    #[test]
    fn test_abort_check_is_fast() {
        // Verify abort checking doesn't introduce significant overhead
        let signal = NoAbort;
        let start = std::time::Instant::now();
        for _ in 0..1_000_000 {
            let _ = signal.is_aborted();
        }
        let elapsed = start.elapsed();

        // Should complete in well under 100ms on any modern hardware
        assert!(
            elapsed.as_millis() < 100,
            "Abort check too slow: {:?}",
            elapsed
        );
    }

    #[test]
    fn test_atomic_abort_check_is_fast() {
        let signal = AtomicAbort::new();
        let start = std::time::Instant::now();
        for _ in 0..1_000_000 {
            let _ = signal.is_aborted();
        }
        let elapsed = start.elapsed();

        // Atomic loads should be fast
        assert!(
            elapsed.as_millis() < 200,
            "Atomic abort check too slow: {:?}",
            elapsed
        );
    }

    //=========================================================================
    // Thread Safety Stress Test
    //=========================================================================

    #[test]
    fn test_atomic_abort_concurrent_access() {
        let signal = Arc::new(AtomicAbort::new());
        let mut handles = vec![];

        // Spawn readers
        for _ in 0..4 {
            let signal = Arc::clone(&signal);
            handles.push(thread::spawn(move || {
                for _ in 0..10000 {
                    let _ = signal.is_aborted();
                }
            }));
        }

        // Spawn a writer that will eventually set abort
        let signal_writer = Arc::clone(&signal);
        handles.push(thread::spawn(move || {
            thread::sleep(Duration::from_millis(5));
            signal_writer.set();
        }));

        // All threads should complete without panic
        for handle in handles {
            handle.join().unwrap();
        }

        // Signal should be set
        assert!(signal.is_aborted());
    }
}
