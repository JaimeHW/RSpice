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

    /// Observe analysis progress as a completed fraction in `[0, 1]`.
    ///
    /// The engine reports at the same low cadence it polls for aborts, so
    /// implementations may update UI state directly. The default ignores
    /// progress, keeping existing implementors unchanged.
    fn observe_progress(&self, _fraction: f64) {}
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
