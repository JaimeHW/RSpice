//! The oracle-free conformance suites: corpora that carry no reference data.
//!
//! # What this instrument measures, and what it cannot
//!
//! The ngspice and Xyce suites answer "are RSpice's numbers right?" by
//! comparing against another simulator's published output. Two vendored
//! corpora cannot be asked that question, because neither upstream ships
//! reference results:
//!
//! - `tests/iscas85/` — the ISCAS85 benchmark circuits as SPICE netlists,
//!   validated upstream by eyeballing a plot. Up to 3512 gates and ~90k
//!   netlist lines, an order of magnitude past anything else vendored here.
//! - `tests/paranoia/` — the ngspice example decks assembled for upstream's
//!   Valgrind harness, validated upstream by not crashing under Valgrind.
//!
//! So this suite asks the question those corpora *can* answer: does every
//! deck load, build, and either complete its analyses or refuse them with a
//! diagnostic? That is a weaker claim than numeric conformance and it is
//! stated weakly on purpose — a green run here means RSpice survived the
//! deck, not that it was right about it.
//!
//! It is still the corpus that finds things the reference suites cannot.
//! ngspice's and Xyce's regression decks are, by construction, decks their
//! authors could already simulate; these are real circuits reaching for
//! whatever dialect corner they happened to need. Every failure is a named
//! gap in ingestion, device coverage, or solver robustness, and the manifest
//! is where those gaps are written down rather than tolerated.
//!
//! # Why one runner over two corpora
//!
//! [`suites`](crate::suites)'s module docs note that the shared shape of the
//! reference suites is not worth a trait, because what they do not share —
//! contract selection, reference format, result codec — is exactly what a
//! trait would have to abstract. These two corpora are the opposite case.
//! They differ in provenance, license, and what they are *for*, and in
//! nothing the runner can see: no reference format to select, no tolerance
//! to model, no comparison to perform. Giving each its own runner would
//! duplicate the whole file to vary a directory name.
//!
//! What stays per-corpus is what genuinely is per-corpus: the root, the
//! license and vendoring notes beside it, and its own manifest. Those live
//! in [`ExecutionCorpus`].

use rspice_core::abort_signal::AbortSignal;
use rspice_core::engine::{ConvergenceConfig, SimulationConfig};
use rspice_core::netlist::AnalysisCommand;
use rspice_core::{Engine, Netlist};
use std::collections::HashMap;
use std::path::{Path, PathBuf};
use std::time::{Duration, Instant};

mod contract;
mod corpus;
mod discovery;
mod manifest;
mod run;

pub use contract::ExecutionContract;
pub use corpus::ExecutionCorpus;

// ═══════════════════════════════════════════════════════════════════════════════
// Results
// ═══════════════════════════════════════════════════════════════════════════════

/// The outcome of running one deck.
#[derive(Debug, Clone)]
pub struct ExecutionResult {
    /// Corpus-relative deck path, which is also the manifest key.
    pub name: String,
    /// Whether the deck met its contract.
    pub passed: bool,
    /// The contract the deck was judged against.
    pub contract: ExecutionContract,
    /// How the deck actually behaved.
    pub outcome: ExecutionOutcome,
    /// Analyses the deck requested, in deck order.
    pub analyses: Vec<String>,
    /// Wall-clock duration in milliseconds.
    pub duration_ms: u128,
    /// Why the contract was not met, when it was not.
    pub failure: Option<ExecutionFailure>,
}

/// What happened to a deck, independent of whether that met its contract.
///
/// The distinction that matters is [`Refused`](Self::Refused) versus
/// [`Rejected`](Self::Rejected): a refusal is the engine declining a problem
/// it understood (no convergence, timestep collapse), a rejection is the deck
/// never becoming a circuit at all. Only the first is ever an acceptable
/// outcome for a deck whose contract says it should run.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ExecutionOutcome {
    /// Every requested analysis ran to completion with finite results.
    Completed,
    /// An analysis terminated with a clean numerical refusal.
    Refused { analysis: String, diagnostic: String },
    /// The deck did not parse, or did not build into a circuit.
    Rejected { diagnostic: String },
    /// The deck exceeded its time budget.
    TimedOut { analysis: String },
    /// The deck requested an analysis this suite does not execute.
    Unsupported { directive: String },
    /// The file parsed but requested no analysis at all.
    ///
    /// Kept distinct from [`Unsupported`](Self::Unsupported) because the two
    /// have opposite causes. Both corpora ship subcircuit libraries under
    /// deck extensions — `pll/vco_sub.cir` defines a VCO and simulates
    /// nothing — and those are correct files doing their job. Folding them in
    /// with "RSpice cannot run this directive" would file a working include
    /// as a coverage gap.
    NoAnalysis,
}

impl ExecutionOutcome {
    /// A one-line form used in reports and in stale-contract diagnostics.
    pub fn summary(&self) -> String {
        match self {
            Self::Completed => "completed".to_string(),
            Self::Refused {
                analysis,
                diagnostic,
            } => format!("refused during {analysis}: {diagnostic}"),
            Self::Rejected { diagnostic } => format!("rejected: {diagnostic}"),
            Self::TimedOut { analysis } => format!("timed out during {analysis}"),
            Self::Unsupported { directive } => format!("unsupported directive: {directive}"),
            Self::NoAnalysis => "parsed, requests no analysis".to_string(),
        }
    }
}

/// Why a deck failed its contract.
///
/// Both variants are failures, and the second is the one worth having: a
/// deck that starts passing while its manifest still records it as a known
/// gap has to break the build, or the manifest silently becomes a list of
/// things that used to be broken. That is how an expected-failure list rots
/// into a permanent one.
#[derive(Debug, Clone)]
pub enum ExecutionFailure {
    /// The deck was expected to run and did not.
    Unmet { expected: &'static str },
    /// The deck outperformed its contract, which makes the contract stale.
    ContractStale { expected: &'static str },
}

/// Aggregate statistics for a corpus run.
#[derive(Debug, Clone, Default)]
pub struct ExecutionStatistics {
    pub total: usize,
    pub passed: usize,
    pub failed: usize,
    pub completed: usize,
    pub expected_unsupported: usize,
    pub expected_refusal: usize,
    pub library_fragments: usize,
    pub total_time_ms: u128,
}

impl ExecutionStatistics {
    /// Fold results into aggregate counts.
    pub fn collect(results: &[ExecutionResult]) -> Self {
        let mut stats = Self::default();
        for result in results {
            stats.total += 1;
            stats.total_time_ms += result.duration_ms;
            if result.passed {
                stats.passed += 1;
            } else {
                stats.failed += 1;
            }
            match result.contract {
                ExecutionContract::Executes => {
                    if matches!(result.outcome, ExecutionOutcome::Completed) {
                        stats.completed += 1;
                    }
                }
                ExecutionContract::ExpectedUnsupported => stats.expected_unsupported += 1,
                ExecutionContract::ExpectedRefusal => stats.expected_refusal += 1,
                ExecutionContract::LibraryFragment => stats.library_fragments += 1,
            }
        }
        stats
    }

    pub fn pass_rate(&self) -> f64 {
        if self.total == 0 {
            0.0
        } else {
            self.passed as f64 / self.total as f64 * 100.0
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Configuration
// ═══════════════════════════════════════════════════════════════════════════════

/// Runner configuration.
#[derive(Debug, Clone)]
pub struct ExecutionConfig {
    /// Time budget per deck, in milliseconds.
    ///
    /// A budget is part of the contract rather than a convenience: the
    /// annotated ISCAS85 decks are large enough that "slow" and "hung" are
    /// only distinguishable by declaring in advance which one we will accept.
    pub max_time_per_deck_ms: u128,
    /// Skip decks the manifest marks as costing more than a routine run.
    ///
    /// The full annotated ISCAS85 set is minutes per deck; gating it keeps
    /// the default run inside a per-commit budget while leaving the whole
    /// corpus one flag away.
    pub skip_extended: bool,
    /// Emit a line per deck as it runs.
    pub verbose: bool,
}

impl Default for ExecutionConfig {
    fn default() -> Self {
        Self {
            max_time_per_deck_ms: 120_000,
            skip_extended: true,
            verbose: false,
        }
    }
}

// ═══════════════════════════════════════════════════════════════════════════════
// Runner
// ═══════════════════════════════════════════════════════════════════════════════

/// Runs an oracle-free corpus against its manifest.
pub struct ExecutionRunner {
    corpus: ExecutionCorpus,
    config: ExecutionConfig,
    root: PathBuf,
    manifest: HashMap<String, ManifestEntry>,
}

#[derive(Debug, Clone, Copy)]
struct ManifestEntry {
    contract: ExecutionContract,
    extended: bool,
}

impl ExecutionRunner {
    /// Create a runner for `corpus`, rooted at the workspace `tests/` directory.
    pub fn new(corpus: ExecutionCorpus, tests_dir: &Path, config: ExecutionConfig) -> Self {
        let root = tests_dir.join(corpus.directory());
        let root = root.canonicalize().unwrap_or(root);
        let manifest = manifest::load(&root);
        Self {
            corpus,
            config,
            root,
            manifest,
        }
    }

    pub fn corpus(&self) -> ExecutionCorpus {
        self.corpus
    }

    pub fn config(&self) -> &ExecutionConfig {
        &self.config
    }

    /// The vendored corpus root.
    pub fn root(&self) -> &Path {
        &self.root
    }

    /// Manifest keys with no deck behind them.
    ///
    /// A manifest entry outliving its deck is how a corpus re-vendoring
    /// quietly drops coverage: the entry keeps asserting something about a
    /// file that is no longer there, and nothing notices.
    pub fn orphaned_manifest_entries(&self) -> Vec<String> {
        let mut orphans: Vec<String> = self
            .manifest
            .keys()
            .filter(|key| !self.root.join(key).is_file())
            .cloned()
            .collect();
        orphans.sort();
        orphans
    }

    /// Engine used for every deck in these corpora.
    ///
    /// Robust convergence rather than the production default, for the same
    /// reason the ngspice suite makes that choice: a deck that fails to
    /// converge under a deliberately weak solver has measured the solver, not
    /// the deck, and this corpus exists to measure the deck.
    fn engine(&self) -> Engine {
        let defaults = SimulationConfig::default();
        Engine::new(SimulationConfig {
            max_iterations: defaults.max_iterations.max(1200),
            convergence_config: ConvergenceConfig::robust(),
            spice_dialect: rspice_core::engine::SpiceDialect::Ngspice,
            ..defaults
        })
    }
}

/// Aborts a run once its wall-clock budget is spent.
#[derive(Debug, Clone, Copy)]
struct DeadlineAbort {
    start: Instant,
    budget: Duration,
}

impl DeadlineAbort {
    fn new(start: Instant, budget_ms: u128) -> Self {
        Self {
            start,
            budget: Duration::from_millis(budget_ms.min(u128::from(u64::MAX)) as u64),
        }
    }
}

impl AbortSignal for DeadlineAbort {
    #[inline]
    fn is_aborted(&self) -> bool {
        self.start.elapsed() >= self.budget
    }
}
