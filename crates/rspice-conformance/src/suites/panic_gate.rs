//! The corpus-wide panic gate.
//!
//! Every other suite here answers "does RSpice produce the right numbers?".
//! This one answers a narrower question that has to be true before that one is
//! worth asking: **no vendored deck makes the simulator panic.**
//!
//! Rule 2 of the engineering contract says authored input never panics — an
//! unsupported construct, a malformed card, a nonexistent node, a coordinate
//! that overflows a limit are all typed refusals. A panic is a different kind
//! of failure from a wrong number: it takes the process down, so an embedding
//! application loses its own state with it, and it is invisible to a
//! conformance suite that only compares waveforms against a reference.
//!
//! # What it runs
//!
//! Every deck in every vendored corpus goes through the ingestion and planning
//! path in process, inside [`std::panic::catch_unwind`]: include expansion,
//! parsing, deck planning, coordinate materialization, one materialized run,
//! and authored-output resolution. That is where an authored-input panic
//! lives, and it is cheap enough to sweep thousands of decks inside a normal
//! CI job.
//!
//! Solver panics are *not* in scope here and do not need to be: the regression
//! suites already run each deck's solve in its own child process precisely so
//! a crash there is attributable and non-fatal. This gate covers the half of
//! the pipeline those subprocess runners reach only for the decks they are
//! configured to execute.
//!
//! # What it reports
//!
//! [`PanicGateReport`] carries the deck count, how many loaded, how many were
//! refused (which is fine — a refusal is the contract), and every deck that
//! panicked with the stage and message. The count is printed by the
//! integration test so a CI log shows the corpus size the gate actually swept
//! rather than a bare "ok".

use std::panic::{AssertUnwindSafe, catch_unwind};
use std::path::Path;
use std::sync::Mutex;

use rspice_core::ResourceLimits;
use rspice_core::abort_signal::CountingAbort;
use rspice_core::engine::{Engine, SimulationConfig};
use rspice_core::execution::{DeckPlan, SignalProjection};
use rspice_core::netlist::{Netlist, validate_output_symbols};

/// One vendored corpus the gate sweeps.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PanicGateCorpus {
    /// Human-readable name used in reports.
    pub label: &'static str,
    /// Directory under the workspace `tests/` root.
    pub directory: &'static str,
    /// Extensions that identify a deck rather than one of its dependencies.
    pub deck_extensions: &'static [&'static str],
}

/// Every corpus with SPICE decks in it.
///
/// `tests/verilog/` is deliberately absent: it holds Verilog sources for the
/// digital oracle harness, not decks this pipeline can load.
pub const PANIC_GATE_CORPORA: &[PanicGateCorpus] = &[
    PanicGateCorpus {
        label: "ngspice",
        directory: "ngspice",
        deck_extensions: &["cir", "sp", "net"],
    },
    PanicGateCorpus {
        label: "Xyce",
        directory: "xyce",
        deck_extensions: &["cir"],
    },
    PanicGateCorpus {
        label: "GF180MCU",
        directory: "gf180mcu",
        // The foundry corpus spells its decks `.spice`; `.cir`/`.sp` would
        // find nothing and the corpus would be present in the list and absent
        // from the sweep.
        deck_extensions: &["spice"],
    },
    PanicGateCorpus {
        label: "ISCAS85",
        directory: "iscas85",
        deck_extensions: &["net"],
    },
    PanicGateCorpus {
        label: "ngspice paranoia examples",
        directory: "paranoia",
        deck_extensions: &["cir", "sp", "deck"],
    },
];

/// Expanded source above which the materialization stage is skipped.
///
/// Elaborating a deck builds its concrete netlist, and the ISCAS85-scale decks
/// are ~90k lines over a BSIM4 card: a few of them would dominate a sweep of
/// several thousand. Those decks are elaborated and solved by
/// [`super::execution`], which budgets minutes per deck for exactly that, so
/// skipping the stage here loses no coverage — it moves it to the suite that
/// already pays for it. Every other stage still runs on every deck.
pub const MAX_MATERIALIZED_SOURCE_BYTES: usize = 1024 * 1024;

/// Stack the sweep runs on.
///
/// A test-harness thread gets 2 MiB, which is smaller than the stack a shipped
/// binary runs on. Ingestion recurses over hierarchy and expression depth and
/// is bounded by `ResourceLimits`, not by the stack, so a 2 MiB thread would
/// report a limit the product does not have — and a stack overflow is an abort
/// that no `catch_unwind` can turn into a finding.
pub const SWEEP_STACK_BYTES: usize = 32 * 1024 * 1024;

/// The pipeline stage a deck reached.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PanicGateStage {
    /// Reading the deck from disk.
    Read,
    /// Expanding `.INCLUDE`/`.LIB` relative to the deck.
    ExpandIncludes,
    /// Parsing the expanded source.
    Parse,
    /// Deriving the run axes and analysis identities.
    Plan,
    /// Enumerating the Cartesian coordinate set.
    Coordinates,
    /// Materializing the first coordinate's concrete netlist.
    Materialize,
    /// Resolving the authored `.PRINT`/`.SAVE` output contract.
    Projection,
}

impl PanicGateStage {
    /// Stable machine-readable name used in reports.
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Read => "read",
            Self::ExpandIncludes => "expand-includes",
            Self::Parse => "parse",
            Self::Plan => "plan",
            Self::Coordinates => "coordinates",
            Self::Materialize => "materialize",
            Self::Projection => "projection",
        }
    }
}

/// One deck that panicked, and where.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct DeckPanic {
    /// `<corpus>/<corpus-relative path>`, the stable identifier a failure is
    /// filed under.
    pub deck: String,
    /// Stage the panic escaped from.
    pub stage: PanicGateStage,
    /// Panic message and source location, as the hook captured them.
    pub message: String,
}

impl std::fmt::Display for DeckPanic {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            formatter,
            "{} panicked during {}: {}",
            self.deck,
            self.stage.as_str(),
            self.message
        )
    }
}

/// What the sweep found.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct PanicGateReport {
    /// Decks discovered and swept.
    pub decks: usize,
    /// Decks that completed the whole pipeline.
    pub loaded: usize,
    /// Decks refused with a typed error somewhere in the pipeline. A refusal
    /// is the contract working, not a failure.
    pub refused: usize,
    /// Decks whose expanded source exceeded [`MAX_MATERIALIZED_SOURCE_BYTES`],
    /// so they were parsed and planned but not elaborated here.
    pub materialization_skipped: usize,
    /// Every deck that panicked, in discovery order.
    pub panics: Vec<DeckPanic>,
}

impl PanicGateReport {
    /// One line for a CI log.
    pub fn summary(&self) -> String {
        format!(
            "panic-gate: decks={} loaded={} refused={} materialization-skipped={} panicked={}",
            self.decks,
            self.loaded,
            self.refused,
            self.materialization_skipped,
            self.panics.len()
        )
    }

    fn merge(&mut self, other: Self) {
        self.decks += other.decks;
        self.loaded += other.loaded;
        self.refused += other.refused;
        self.materialization_skipped += other.materialization_skipped;
        self.panics.extend(other.panics);
    }
}

/// Sweep every corpus under `tests_dir`, on a stack the product actually has.
pub fn run_panic_gate(tests_dir: &Path) -> PanicGateReport {
    let tests_dir = tests_dir.to_path_buf();
    std::thread::Builder::new()
        .stack_size(SWEEP_STACK_BYTES)
        .spawn(move || {
            let recorder = PanicRecorder::install();
            let mut report = PanicGateReport::default();
            for corpus in PANIC_GATE_CORPORA {
                report.merge(run_corpus(&tests_dir, *corpus, &recorder));
            }
            report
        })
        .expect("spawn the panic-gate sweep thread")
        .join()
        .expect("the sweep thread itself must not die")
}

/// Sweep one corpus.
pub fn run_corpus(
    tests_dir: &Path,
    corpus: PanicGateCorpus,
    recorder: &PanicRecorder,
) -> PanicGateReport {
    let root = tests_dir.join(corpus.directory);
    let mut decks = Vec::new();
    collect(&root, &root, corpus.deck_extensions, &mut decks);
    decks.sort();

    let mut report = PanicGateReport {
        decks: decks.len(),
        ..PanicGateReport::default()
    };
    for key in decks {
        let deck = format!("{}/{key}", corpus.directory);
        match sweep_deck(&root.join(&key), recorder) {
            DeckOutcome::Loaded => report.loaded += 1,
            DeckOutcome::LoadedWithoutMaterialization => {
                report.loaded += 1;
                report.materialization_skipped += 1;
            }
            DeckOutcome::Refused => report.refused += 1,
            DeckOutcome::Panicked { stage, message } => report.panics.push(DeckPanic {
                deck,
                stage,
                message,
            }),
        }
    }
    report
}

enum DeckOutcome {
    Loaded,
    LoadedWithoutMaterialization,
    Refused,
    Panicked {
        stage: PanicGateStage,
        message: String,
    },
}

/// The engine the gate plans with.
///
/// Production defaults, deliberately: the gate asks whether a deck a user
/// could hand the shipped simulator makes it panic, so a more permissive
/// resource policy would be measuring a configuration nobody runs.
fn gate_engine() -> Engine {
    Engine::new(SimulationConfig::default())
}

/// Run one deck through ingestion and planning, catching a panic at whichever
/// stage it escapes from.
fn sweep_deck(path: &Path, recorder: &PanicRecorder) -> DeckOutcome {
    macro_rules! stage {
        ($stage:expr, $body:expr) => {
            match run_stage($stage, recorder, || $body) {
                Ok(Ok(value)) => value,
                Ok(Err(())) => return DeckOutcome::Refused,
                Err(outcome) => return outcome,
            }
        };
    }

    let source = stage!(
        PanicGateStage::Read,
        std::fs::read_to_string(path).map_err(drop)
    );
    let expanded = stage!(
        PanicGateStage::ExpandIncludes,
        Netlist::preprocess_includes(&source, path).map_err(drop)
    );
    let netlist = stage!(
        PanicGateStage::Parse,
        Netlist::parse_with_path(&expanded, path).map_err(drop)
    );
    let limits = ResourceLimits::default();
    let plan = stage!(
        PanicGateStage::Plan,
        DeckPlan::from_netlist(&netlist, &limits).map_err(drop)
    );
    let coordinates = stage!(
        PanicGateStage::Coordinates,
        plan.coordinates_with_abort(&limits, &rspice_core::abort_signal::NoAbort)
            .map_err(drop)
    );
    if coordinates.is_empty() {
        return DeckOutcome::Refused;
    }
    // Materializing one coordinate builds the concrete netlist a run would
    // solve, which is where a `.STEP` binding or an `.ALTER` variant would
    // trip an index. Cancellation is bounded so a pathological deck cannot
    // turn the sweep into a solve.
    let materialized = expanded.len() <= MAX_MATERIALIZED_SOURCE_BYTES;
    if materialized {
        let engine = gate_engine();
        let abort = CountingAbort::new(50_000);
        stage!(PanicGateStage::Materialize, {
            engine
                .prepare_deck_plan_materializer_with_abort(&netlist, &plan, &abort)
                .and_then(|materializer| materializer.materialize_run_with_abort(0, &abort))
                .map(drop)
                .map_err(drop)
        });
    }
    stage!(PanicGateStage::Projection, {
        let _ = validate_output_symbols(&netlist);
        SignalProjection::from_netlist(&netlist).map_err(drop)
    });
    if materialized {
        DeckOutcome::Loaded
    } else {
        DeckOutcome::LoadedWithoutMaterialization
    }
}

/// Run one fallible stage under `catch_unwind`, mapping a panic to an outcome.
fn run_stage<T>(
    stage: PanicGateStage,
    recorder: &PanicRecorder,
    body: impl FnOnce() -> Result<T, ()>,
) -> Result<Result<T, ()>, DeckOutcome> {
    catch_unwind(AssertUnwindSafe(body)).map_err(|_| DeckOutcome::Panicked {
        stage,
        message: recorder.take(),
    })
}

fn collect(root: &Path, dir: &Path, extensions: &[&str], out: &mut Vec<String>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            collect(root, &path, extensions, out);
            continue;
        }
        let is_deck = path
            .extension()
            .and_then(|extension| extension.to_str())
            .is_some_and(|extension| {
                extensions
                    .iter()
                    .any(|wanted| extension.eq_ignore_ascii_case(wanted))
            });
        if is_deck && let Ok(relative) = path.strip_prefix(root) {
            out.push(relative.to_string_lossy().replace('\\', "/"));
        }
    }
}

/// Captures panic messages instead of printing thousands of backtraces.
///
/// The default hook writes every panic to standard error, which for a sweep
/// of several thousand decks would bury the one line that matters. Installing
/// a recorder keeps the message and its location and prints nothing; the
/// report carries them to the failure message.
pub struct PanicRecorder {
    last: &'static Mutex<Option<String>>,
}

static LAST_PANIC: Mutex<Option<String>> = Mutex::new(None);

impl PanicRecorder {
    /// Replace the process panic hook for the rest of this process.
    ///
    /// The gate is one test in its own binary, so the hook is not restored:
    /// restoring it would race with the panics the sweep is still catching.
    pub fn install() -> Self {
        std::panic::set_hook(Box::new(|info| {
            let location = info
                .location()
                .map_or_else(|| "unknown location".to_owned(), ToString::to_string);
            let message = info.payload_as_str().unwrap_or("<non-string payload>");
            if let Ok(mut last) = LAST_PANIC.lock() {
                *last = Some(format!("{message} (at {location})"));
            }
        }));
        Self { last: &LAST_PANIC }
    }

    /// The most recent panic message, consumed.
    pub fn take(&self) -> String {
        self.last
            .lock()
            .ok()
            .and_then(|mut last| last.take())
            .unwrap_or_else(|| "<panic message unavailable>".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::*;

    fn temp_dir(tag: &str) -> PathBuf {
        let unique = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("system time after the Unix epoch")
            .as_nanos();
        std::env::temp_dir().join(format!("rspice_panic_gate_{tag}_{unique}"))
    }

    #[test]
    fn discovery_selects_deck_extensions_and_recurses() {
        let root = temp_dir("discovery");
        std::fs::create_dir_all(root.join("nested")).expect("create corpus fixture");
        for (name, body) in [
            ("top.cir", "top\n.end\n"),
            ("nested/inner.sp", "inner\n.end\n"),
            ("models.lib", ".subckt m a\n.ends\n"),
        ] {
            std::fs::write(root.join(name), body).expect("write corpus fixture");
        }
        let mut decks = Vec::new();
        collect(&root, &root, &["cir", "sp"], &mut decks);
        decks.sort();
        assert_eq!(
            decks,
            vec!["nested/inner.sp".to_string(), "top.cir".to_string()]
        );
        std::fs::remove_dir_all(root).expect("remove corpus fixture");
    }

    #[test]
    fn a_refused_deck_is_counted_as_a_refusal_not_a_panic() {
        let tests_dir = temp_dir("refusal");
        let root = tests_dir.join("paranoia");
        std::fs::create_dir_all(&root).expect("create corpus fixture");
        // A device card naming an undeclared model is a typed refusal.
        std::fs::write(
            root.join("refused.cir"),
            "refused deck\nQ1 c b e nonexistent_model\n.op\n.end\n",
        )
        .expect("write corpus fixture");
        std::fs::write(
            root.join("good.cir"),
            "good deck\nV1 a 0 1\nR1 a 0 1k\n.op\n.end\n",
        )
        .expect("write corpus fixture");

        let recorder = PanicRecorder::install();
        let report = run_corpus(
            &tests_dir,
            PanicGateCorpus {
                label: "fixture",
                directory: "paranoia",
                deck_extensions: &["cir"],
            },
            &recorder,
        );
        assert_eq!(report.decks, 2);
        assert!(report.panics.is_empty(), "{:?}", report.panics);
        assert_eq!(report.loaded + report.refused, 2);
        assert!(report.summary().contains("panicked=0"));

        std::fs::remove_dir_all(tests_dir).expect("remove corpus fixture");
    }
}
