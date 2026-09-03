//! Deterministic fault injection for artifact publication.
//!
//! "Either the old complete artifact or no artifact" is a claim about what
//! happens when publication *fails*, and the interesting failures — a full
//! disk, a refused allocation, a replace that loses a race — cannot be
//! produced on demand by a test that only calls the ordinary API. This module
//! is the qualification seam that produces them.
//!
//! A caller arms one [`ArtifactFault`] with [`ArmedFaults::arm`]; the returned
//! guard is scoped and thread-local, so only publications on the arming thread
//! are affected and dropping the guard restores whatever was armed before.
//! While it is armed, the next publication that reaches the named
//! [`ArtifactFaultPoint`] fails with a synthetic [`std::io::Error`] of the
//! chosen [`ArtifactFaultKind`]; every other point behaves normally.
//!
//! Nothing is armed unless a caller constructs a guard, so the seam costs the
//! production path one thread-local read per publication phase. It is exposed
//! rather than hidden behind `#[cfg(test)]` because the transactions under
//! test span three crates — the CLI's coordinate-set transaction, the engine's
//! checkpoint save, and this crate's own set commit — and a qualification gate
//! has to drive them through exactly the code the product ships.

use std::cell::{Cell, RefCell};
use std::io;
use std::rc::Rc;

use crate::FaultPoint;

/// A phase of artifact publication that can be made to fail.
///
/// The variants correspond one-to-one with the points the publication code
/// consults, so a test names a phase rather than guessing at a syscall.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ArtifactFaultPoint {
    /// Immediately after a staging file exists, before anything is written.
    AfterStagingCreated,
    /// Before the staged bytes are flushed.
    BeforeFlush,
    /// After the staged bytes are flushed and synchronized.
    AfterFlush,
    /// After the artifact is durable, before its destination is validated.
    BeforeCommit,
    /// The atomic replacement of the destination itself.
    Replace,
    /// The parent-directory synchronization that follows a replacement.
    SyncParent,
    /// Staging one member of an [`crate::AtomicArtifactSet`].
    StageMember,
    /// Snapshotting one member's predecessor, before any member is committed.
    CapturePredecessor,
    /// Committing the set's manifest member, which is always committed last.
    CommitManifest,
}

impl ArtifactFaultPoint {
    /// Stable machine-readable name, used in the injected error message.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::AfterStagingCreated => "after-staging-created",
            Self::BeforeFlush => "before-flush",
            Self::AfterFlush => "after-flush",
            Self::BeforeCommit => "before-commit",
            Self::Replace => "replace",
            Self::SyncParent => "sync-parent",
            Self::StageMember => "stage-member",
            Self::CapturePredecessor => "capture-predecessor",
            Self::CommitManifest => "commit-manifest",
        }
    }

    const fn internal(self) -> FaultPoint {
        match self {
            Self::AfterStagingCreated => FaultPoint::AfterPrepare,
            Self::BeforeFlush => FaultPoint::Flush,
            Self::AfterFlush => FaultPoint::AfterFlush,
            Self::BeforeCommit => FaultPoint::BeforeCommit,
            Self::Replace => FaultPoint::Replace,
            Self::SyncParent => FaultPoint::SyncParent,
            Self::StageMember => FaultPoint::SetStage,
            Self::CapturePredecessor => FaultPoint::SetPredecessor,
            Self::CommitManifest => FaultPoint::SetManifestCommit,
        }
    }
}

/// Which class of failure to synthesize.
///
/// The transaction contract is the same either way — the destination keeps its
/// previous bytes — but the two classes reach different error paths: an
/// allocation refusal is reported by the host as `OutOfMemory`, which a caller
/// must not confuse with a corrupt or missing file.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ArtifactFaultKind {
    /// A device or filesystem error.
    Io,
    /// A refused allocation, reported as [`io::ErrorKind::OutOfMemory`].
    Allocation,
}

impl ArtifactFaultKind {
    fn error(self, point: ArtifactFaultPoint) -> io::Error {
        match self {
            Self::Io => io::Error::other(format!("injected I/O failure at {}", point.as_str())),
            Self::Allocation => io::Error::new(
                io::ErrorKind::OutOfMemory,
                format!("injected allocation failure at {}", point.as_str()),
            ),
        }
    }
}

/// One armed failure: where, what kind, and which occurrence.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct ArtifactFault {
    point: ArtifactFaultPoint,
    kind: ArtifactFaultKind,
    member: Option<usize>,
    skip: usize,
}

impl ArtifactFault {
    /// Fail the first time publication reaches `point`.
    #[must_use]
    pub const fn io(point: ArtifactFaultPoint) -> Self {
        Self {
            point,
            kind: ArtifactFaultKind::Io,
            member: None,
            skip: 0,
        }
    }

    /// Refuse an allocation the first time publication reaches `point`.
    #[must_use]
    pub const fn allocation(point: ArtifactFaultPoint) -> Self {
        Self {
            point,
            kind: ArtifactFaultKind::Allocation,
            member: None,
            skip: 0,
        }
    }

    /// Restrict this fault to one set member, by its commit-order index.
    ///
    /// The manifest member, when a set has one, is the last index.
    #[must_use]
    pub const fn on_member(mut self, index: usize) -> Self {
        self.member = Some(index);
        self
    }

    /// Let the first `count` matching occurrences succeed before failing.
    #[must_use]
    pub const fn after_occurrences(mut self, count: usize) -> Self {
        self.skip = count;
        self
    }

    /// The phase this fault fires at.
    #[must_use]
    pub const fn point(self) -> ArtifactFaultPoint {
        self.point
    }

    /// The failure class this fault synthesizes.
    #[must_use]
    pub const fn kind(self) -> ArtifactFaultKind {
        self.kind
    }
}

/// Per-thread armed state. `fired` is shared with the guard so a caller can
/// assert that the fault it armed was actually reached.
struct ArmedState {
    fault: ArtifactFault,
    current_member: usize,
    matched: usize,
    fired: Rc<Cell<usize>>,
}

impl ArmedState {
    fn check(&mut self, point: FaultPoint) -> io::Result<()> {
        if point != self.fault.point.internal() {
            return Ok(());
        }
        if self
            .fault
            .member
            .is_some_and(|member| member != self.current_member)
        {
            return Ok(());
        }
        self.matched += 1;
        if self.matched <= self.fault.skip {
            return Ok(());
        }
        self.fired.set(self.fired.get() + 1);
        Err(self.fault.kind.error(self.fault.point))
    }
}

thread_local! {
    static ARMED: RefCell<Option<ArmedState>> = const { RefCell::new(None) };
}

/// Scoped guard for an armed fault. Dropping it disarms.
pub struct ArmedFaults {
    previous: Option<ArmedState>,
    fired: Rc<Cell<usize>>,
}

impl ArmedFaults {
    /// Arm `fault` for publications on this thread until the guard is dropped.
    ///
    /// Arming replaces any fault already armed on this thread and restores it
    /// when the guard drops, so a nested scope is well defined rather than a
    /// silent overwrite.
    #[must_use]
    pub fn arm(fault: ArtifactFault) -> Self {
        let fired = Rc::new(Cell::new(0));
        let state = ArmedState {
            fault,
            current_member: 0,
            matched: 0,
            fired: Rc::clone(&fired),
        };
        let previous = ARMED.with(|slot| slot.borrow_mut().replace(state));
        Self { previous, fired }
    }

    /// How many times the armed fault has fired so far.
    #[must_use]
    pub fn fired(&self) -> usize {
        self.fired.get()
    }
}

impl Drop for ArmedFaults {
    fn drop(&mut self) {
        let previous = self.previous.take();
        // `try_with` rather than `with`: a guard dropped while this thread's
        // locals are already being destroyed must disarm quietly instead of
        // panicking inside a `Drop`.
        let _ = ARMED.try_with(|slot| *slot.borrow_mut() = previous);
    }
}

impl std::fmt::Debug for ArmedFaults {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        formatter
            .debug_struct("ArmedFaults")
            .field("fired", &self.fired.get())
            .finish()
    }
}

/// The production path's read of this thread's armed fault.
///
/// `try_with` rather than `with`: publication can happen inside a `Drop` while
/// a thread's locals are being destroyed, and an artifact write must not start
/// panicking because a test-only slot is already gone. An unavailable slot is
/// exactly the same answer as an empty one — nothing is armed.
pub(crate) fn check_armed(point: FaultPoint) -> io::Result<()> {
    ARMED
        .try_with(|slot| match slot.borrow_mut().as_mut() {
            Some(state) => state.check(point),
            None => Ok(()),
        })
        .unwrap_or(Ok(()))
}

pub(crate) fn enter_armed_member(index: usize) {
    let _ = ARMED.try_with(|slot| {
        if let Some(state) = slot.borrow_mut().as_mut() {
            state.current_member = index;
        }
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nothing_is_injected_until_a_fault_is_armed() {
        assert!(check_armed(FaultPoint::Replace).is_ok());
    }

    #[test]
    fn an_armed_fault_fires_once_at_its_own_point() {
        let armed = ArmedFaults::arm(ArtifactFault::io(ArtifactFaultPoint::Replace));
        assert!(check_armed(FaultPoint::AfterPrepare).is_ok());
        assert!(check_armed(FaultPoint::Replace).is_err());
        assert_eq!(armed.fired(), 1);
    }

    #[test]
    fn an_allocation_fault_is_reported_as_out_of_memory() {
        let _armed = ArmedFaults::arm(ArtifactFault::allocation(
            ArtifactFaultPoint::AfterStagingCreated,
        ));
        let error = check_armed(FaultPoint::AfterPrepare).expect_err("the fault fires");
        assert_eq!(error.kind(), io::ErrorKind::OutOfMemory);
    }

    #[test]
    fn occurrences_can_be_skipped_and_members_selected() {
        let armed = ArmedFaults::arm(
            ArtifactFault::io(ArtifactFaultPoint::CapturePredecessor)
                .on_member(1)
                .after_occurrences(1),
        );
        enter_armed_member(0);
        assert!(check_armed(FaultPoint::SetPredecessor).is_ok());
        enter_armed_member(1);
        assert!(check_armed(FaultPoint::SetPredecessor).is_ok());
        assert!(check_armed(FaultPoint::SetPredecessor).is_err());
        assert_eq!(armed.fired(), 1);
    }

    #[test]
    fn dropping_a_guard_restores_the_previously_armed_fault() {
        let outer = ArmedFaults::arm(ArtifactFault::io(ArtifactFaultPoint::Replace));
        {
            let _inner = ArmedFaults::arm(ArtifactFault::io(ArtifactFaultPoint::SyncParent));
            assert!(check_armed(FaultPoint::Replace).is_ok());
        }
        assert!(check_armed(FaultPoint::Replace).is_err());
        assert_eq!(outer.fired(), 1);
    }
}
