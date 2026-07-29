//! What a deck in an oracle-free corpus is required to do.

/// The contract a deck is judged against.
///
/// The vocabulary is deliberately narrower than the ngspice suite's. Half of
/// that suite's contracts describe how to *compare* against reference data —
/// locked grids, extracted measures, references the producing simulator could
/// not generate — and none of them mean anything where there is no reference
/// to compare against. What is left is the part that survives without an
/// oracle: did the deck run, and if not, was the failure one we have already
/// adjudicated and named?
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionContract {
    /// The deck must parse, build, and complete every analysis it requests
    /// with finite results.
    ///
    /// This is the default for any deck the manifest does not mention, which
    /// is the direction that fails safe: a newly vendored deck is required to
    /// work until someone adjudicates otherwise, rather than being silently
    /// exempt until someone notices.
    Executes,
    /// The deck reaches for a feature RSpice does not implement, and must
    /// fail while doing so.
    ///
    /// Recorded with the specific gap in the manifest's note column. A deck
    /// under this contract that starts passing fails the suite, because the
    /// gap it stood for has closed and the entry is now a lie about the
    /// simulator's coverage.
    ExpectedUnsupported,
    /// The deck is adjudicated genuinely un-simulatable, and some analysis
    /// must terminate with a clean numerical refusal.
    ///
    /// "Clean" is the whole content of the contract: the engine must decline
    /// the problem with a diagnostic, not hang, panic, or return values that
    /// are quietly non-finite. A deck that completes under this contract is
    /// also a failure — the adjudication needs revisiting, not honouring.
    ExpectedRefusal,
    /// The file is an include fragment, not a deck: it must parse and build
    /// but requests no analysis of its own.
    ///
    /// Both corpora keep subcircuit libraries under deck extensions —
    /// `pll/vco_sub.cir`, `xspice/pll/loop-filter.cir` — so discovery finds
    /// them and cannot tell them apart from a deck by name. Marking them
    /// here rather than filtering them out in discovery keeps the claim
    /// checkable: the file still has to parse, and a fragment that starts
    /// requesting an analysis is flagged rather than quietly reclassified.
    LibraryFragment,
}

impl ExecutionContract {
    pub(super) fn parse(token: &str) -> Option<Self> {
        match token.trim() {
            "executes" => Some(Self::Executes),
            "expected_unsupported" => Some(Self::ExpectedUnsupported),
            "expected_refusal" => Some(Self::ExpectedRefusal),
            "library_fragment" => Some(Self::LibraryFragment),
            _ => None,
        }
    }

    /// The manifest spelling, used in diagnostics so a failure names the row
    /// the reader has to go edit.
    pub fn token(self) -> &'static str {
        match self {
            Self::Executes => "executes",
            Self::ExpectedUnsupported => "expected_unsupported",
            Self::ExpectedRefusal => "expected_refusal",
            Self::LibraryFragment => "library_fragment",
        }
    }
}
