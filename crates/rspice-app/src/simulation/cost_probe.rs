//! What a frame asked the simulation model to derive, counted.
//!
//! A surface that resolves the same thing twice looks exactly like one that
//! resolves it once: both paint the same pixels, both pass every assertion
//! about what they say, and the only difference is how long the frame took.
//! The Analyses route expanded the declared space twice a frame for months —
//! once for the rail's prices, once for the editor's run-points control — and
//! nothing could have failed over it.
//!
//! So the three derivations a studio frame is allowed to pay for once are
//! counted here, and the routes that must not pay twice assert the count.
//! Test-only: the recording calls are `#[cfg(test)]`, so a shipped build has
//! no counter and no branch.
//!
//! The counts are per thread, because that is where a test's frame runs and
//! because two tests sharing one counter would report each other's work.

use std::cell::Cell;

/// One derivation a frame can pay for more than once.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Derivation {
    /// Expanding the declared run space into its points. The expensive one:
    /// its cost is the size of the space, not the size of the declaration.
    SpaceExpansion,
    /// Validating a run-set declaration. Cheap per call — the point count is a
    /// product rather than a walk — but it allocates its findings, and two
    /// calls a frame to print one number in two places is still two.
    RunSetValidation,
    /// Resolving the placed-source list, which walks the design's nets.
    PlacedSources,
}

thread_local! {
    static COUNTS: Cell<[usize; 3]> = const { Cell::new([0; 3]) };
}

const fn slot(derivation: Derivation) -> usize {
    match derivation {
        Derivation::SpaceExpansion => 0,
        Derivation::RunSetValidation => 1,
        Derivation::PlacedSources => 2,
    }
}

/// Note that `derivation` was performed.
pub(crate) fn record(derivation: Derivation) {
    COUNTS.with(|counts| {
        let mut current = counts.get();
        current[slot(derivation)] += 1;
        counts.set(current);
    });
}

/// How many times `derivation` has been performed on this thread since the
/// last [`reset`].
pub(crate) fn count(derivation: Derivation) -> usize {
    COUNTS.with(|counts| counts.get()[slot(derivation)])
}

/// Forget everything counted so far on this thread.
pub(crate) fn reset() {
    COUNTS.with(|counts| counts.set([0; 3]));
}

#[cfg(test)]
mod tests {
    use super::{Derivation, count, record, reset};

    /// The probe counts each derivation separately and starts from zero.
    ///
    /// A probe that always answered zero would make every cost pin pass
    /// forever, which is the failure mode a counting test has and an
    /// assertion about painted text does not.
    #[test]
    fn the_probe_counts_each_derivation_on_its_own() {
        reset();
        assert_eq!(count(Derivation::SpaceExpansion), 0);

        record(Derivation::SpaceExpansion);
        record(Derivation::SpaceExpansion);
        record(Derivation::PlacedSources);

        assert_eq!(count(Derivation::SpaceExpansion), 2);
        assert_eq!(count(Derivation::PlacedSources), 1);
        assert_eq!(count(Derivation::RunSetValidation), 0);

        reset();
        assert_eq!(count(Derivation::SpaceExpansion), 0);
        assert_eq!(count(Derivation::PlacedSources), 0);
    }
}
