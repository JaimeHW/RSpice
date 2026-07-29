//! The vendored corpora this runner serves.

/// A corpus of decks with no reference data.
///
/// Each variant names a directory under the workspace `tests/` root that
/// carries its own upstream license text and `RSPICE-VENDORING.md`. The
/// variants exist so those provenance boundaries stay visible in code:
/// the two corpora are not interchangeable sources of decks, they are
/// separately licensed bodies of third-party material that happen to be
/// judged the same way.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExecutionCorpus {
    /// The ISCAS85 combinational benchmark circuits, as transistor-level
    /// SPICE over a 45nm predictive BSIM4 technology.
    ///
    /// Bought for scale. The annotated variants carry extracted parasitics
    /// and reach ~90k netlist lines and thousands of devices, which is where
    /// matrix ordering, device-evaluation throughput, and timestep control
    /// stop being theoretical.
    Iscas85,
    /// The ngspice example decks assembled for upstream's Valgrind harness.
    ///
    /// Bought for dialect breadth. These are the corners of ngspice that the
    /// regression suite does not reach — CIDER numerical devices, `.measure`
    /// forms, transient noise, XSPICE code models, Monte Carlo, memristors,
    /// operating-point-from-transient — written by people solving problems
    /// rather than by people writing tests.
    Paranoia,
}

impl ExecutionCorpus {
    /// Every corpus, for suites that sweep all of them.
    pub const ALL: [Self; 2] = [Self::Iscas85, Self::Paranoia];

    /// Directory name under the workspace `tests/` root.
    pub fn directory(self) -> &'static str {
        match self {
            Self::Iscas85 => "iscas85",
            Self::Paranoia => "paranoia",
        }
    }

    /// Human-readable name for reports.
    pub fn label(self) -> &'static str {
        match self {
            Self::Iscas85 => "ISCAS85",
            Self::Paranoia => "ngspice paranoia examples",
        }
    }

    /// Extensions that identify a deck rather than one of its dependencies.
    ///
    /// Both corpora keep model cards, stimulus files, and S-parameter data
    /// beside the decks that include them, so discovery has to select rather
    /// than sweep: a `.lib` full of subcircuits is not a deck, and running it
    /// as one would report a fabricated failure.
    pub fn deck_extensions(self) -> &'static [&'static str] {
        match self {
            Self::Iscas85 => &["net"],
            Self::Paranoia => &["cir", "sp", "deck"],
        }
    }
}
