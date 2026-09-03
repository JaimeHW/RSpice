//! The checked-in qualification baseline, regenerated and compared.
//!
//! `benchmarks/baselines/qualification/rspice-qualification-v1.json` is a
//! product artifact: it records what this build *is* — its toolchain, its
//! feature set, the analysis/result matrix each non-UI surface supports, the
//! oracle evidence checked in beside it — and what it *costs*, as counts. This
//! test regenerates the whole document from the code and the tree and compares
//! it to the checked-in copy.
//!
//! # Facts are compared exactly
//!
//! The `facts` section is derived from `rspice_core::execution::capability`,
//! from the vendored corpora, and from the structure of typed result documents
//! produced by fixed decks. None of it depends on floating-point results, so a
//! difference is a real change and the baseline must be regenerated
//! deliberately.
//!
//! # Gates are compared against a declared tolerance
//!
//! The `gates` section holds counts, never wall-clock. Wall-clock and resident
//! set size are not gated here at all: a shared CI runner cannot measure either
//! reproducibly, so gating on them would either be noise or be set so loose it
//! catches nothing. Every performance claim this file makes is therefore a
//! *count* — abort polls, retained points, result values, artifact bytes —
//! each with the tolerance it is judged against, because the counts downstream
//! of a Newton loop are exact on one host but not identical on every target.
//!
//! # Regenerating
//!
//! Set `RSPICE_UPDATE_QUALIFICATION_BASELINE=1` and run this test. Promotion
//! is still an engineering decision: a regenerated baseline is a claim that
//! the new numbers are correct, and the diff has to be read.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use rspice_core::Value;
use rspice_core::abort_signal::{CountingAbort, NoAbort};
use rspice_core::analysis::PssConfig;
use rspice_core::analysis::harmonic_balance::HbConfig;
use rspice_core::engine::{
    CompressionConfig, Engine, SimulationConfig, TransientCheckpointEncoding,
};
use rspice_core::execution::result_document::AnalysisResultDocument;
use rspice_core::execution::{
    ANALYSIS_CAPABILITY_MATRIX, ANALYSIS_RESULT_DOCUMENT_SCHEMA, ANALYSIS_RESULT_DOCUMENT_VERSION,
    AnalysisInstanceId, AnalysisKind, AnalysisRequest, DeckPlan, MappingStatus, NonUiSurface,
    SIGNAL_CAPABILITY_MATRIX, SurfaceCapability,
};
use rspice_core::netlist::Netlist;
use serde_json::{Value as Json, json};

/// Schema identifier written into the baseline.
const BASELINE_SCHEMA: &str = "rspice-qualification-baseline";
/// Version of that schema. Bump it when a field changes meaning.
const BASELINE_SCHEMA_VERSION: u32 = 1;
/// Environment variable that rewrites the checked-in file.
const UPDATE_ENV: &str = "RSPICE_UPDATE_QUALIFICATION_BASELINE";

fn workspace_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("rspice-core is a workspace crate under crates/")
        .to_path_buf()
}

fn baseline_path() -> PathBuf {
    workspace_root().join("benchmarks/baselines/qualification/rspice-qualification-v1.json")
}

//=============================================================================
// Fixtures
//=============================================================================

/// A small linear RC driven by a megahertz sine.
///
/// The same fixture `abort_iteration_bounds.rs` uses, and for the same reason:
/// every analysis family below converges on it in milliseconds, so the counts
/// are of the analysis rather than of a hard circuit.
fn fixture() -> (Engine, Netlist) {
    let netlist = Netlist::parse(
        "qualification baseline fixture\n\
         .param rval=1k\n\
         v1 in 0 sin(0 1 1meg) ac 1\n\
         r1 in out {rval}\n\
         c1 out 0 1n\n\
         .end\n",
    )
    .expect("the qualification fixture parses");
    (Engine::new(SimulationConfig::default()), netlist)
}

/// Run `body` with the compute pool pinned to one thread.
///
/// Several sweeps parallelize their points, and a work-stealing pool makes the
/// poll count a function of core count rather than of the analysis.
fn serialized<T: Send>(body: impl FnOnce() -> T + Send) -> T {
    rayon::ThreadPoolBuilder::new()
        .num_threads(1)
        .build()
        .expect("single-threaded compute pool")
        .install(body)
}

fn decade_sweep(points: usize) -> Vec<Value> {
    (0..points)
        .map(|index| 10.0_f64.powf(1.0 + 6.0 * index as Value / points as Value))
        .collect()
}

/// Total polls a completed run performs.
fn poll_count<T: Send, E: std::fmt::Debug + Send>(
    what: &str,
    run: impl FnOnce(&CountingAbort) -> Result<T, E> + Send,
) -> usize {
    let counter = CountingAbort::new(usize::MAX);
    let outcome = serialized(|| run(&counter));
    assert!(
        outcome.is_ok(),
        "{what} did not complete, so its poll count measures nothing: {:?}",
        outcome.err()
    );
    counter.count()
}

/// Polls performed after cancellation was first reported at an interior poll.
fn polls_after_cancel<T: Send, E: Send>(
    what: &str,
    total: usize,
    run: impl FnOnce(&CountingAbort) -> Result<T, E> + Send,
) -> usize {
    let threshold = total / 2;
    let abort = CountingAbort::new(threshold);
    let outcome = serialized(|| run(&abort));
    assert!(
        outcome.is_err(),
        "{what} completed despite being cancelled at poll {}",
        threshold + 1
    );
    abort.polls_after_abort()
}

/// Mint a planner analysis identity for a result document.
fn analysis_id(kind: AnalysisKind) -> AnalysisInstanceId {
    DeckPlan::new(Vec::new(), vec![AnalysisRequest::new(kind)])
        .expect("a plan with one analysis is valid")
        .analyses()
        .first()
        .expect("a plan names its analysis")
        .id()
}

//=============================================================================
// Capability rendering
//=============================================================================

fn status_json(status: MappingStatus) -> Json {
    match status {
        MappingStatus::Mapped => json!({ "status": "mapped" }),
        MappingStatus::Partial(note) => json!({ "status": "partial", "reason": note }),
        MappingStatus::Unsupported(note) => json!({ "status": "unsupported", "reason": note }),
    }
}

fn surface_json(capability: SurfaceCapability) -> Json {
    json!({
        "scalar": status_json(capability.scalar),
        "stepped": status_json(capability.stepped),
        "temperature": status_json(capability.temperature),
    })
}

/// The analysis/result matrix per non-UI surface, straight from the registry.
fn capability_json() -> Json {
    let analyses = ANALYSIS_CAPABILITY_MATRIX
        .iter()
        .map(|row| {
            let surfaces = NonUiSurface::ALL
                .iter()
                .map(|surface| {
                    (
                        surface.heading().to_owned(),
                        surface_json(row.surface(*surface)),
                    )
                })
                .collect::<serde_json::Map<_, _>>();
            json!({ "result": row.result.tag(), "surfaces": surfaces })
        })
        .collect::<Vec<_>>();
    let signals = SIGNAL_CAPABILITY_MATRIX
        .iter()
        .map(|row| {
            let surfaces = NonUiSurface::ALL
                .iter()
                .map(|surface| {
                    (
                        surface.heading().to_owned(),
                        status_json(row.surface(*surface)),
                    )
                })
                .collect::<serde_json::Map<_, _>>();
            json!({ "signal": format!("{:?}", row.signal), "surfaces": surfaces })
        })
        .collect::<Vec<_>>();
    json!({ "analyses": analyses, "signals": signals })
}

//=============================================================================
// Oracle evidence
//=============================================================================

fn count_files(root: &Path, extensions: &[&str]) -> usize {
    fn walk(dir: &Path, extensions: &[&str], count: &mut usize) {
        let Ok(entries) = std::fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            if path.is_dir() {
                walk(&path, extensions, count);
                continue;
            }
            let matches = path
                .extension()
                .and_then(|extension| extension.to_str())
                .is_some_and(|extension| {
                    extensions
                        .iter()
                        .any(|wanted| extension.eq_ignore_ascii_case(wanted))
                });
            if matches {
                *count += 1;
            }
        }
    }
    let mut count = 0;
    walk(root, extensions, &mut count);
    count
}

/// Counts of the oracle evidence checked in beside the suites.
///
/// These are counts of *evidence*, not of a live run: executing the ngspice
/// and Xyce corpora is a nightly job, and those jobs own the pass counts. What
/// this records is how much reference data the tree carries, which is what
/// moves when coverage is added or a corpus is re-vendored, and it is
/// measurable in a per-commit job.
fn oracle_evidence_json() -> Json {
    let tests = workspace_root().join("tests");
    json!({
        "note": "counts of checked-in oracle evidence; live pass counts belong to the nightly \
                 conformance jobs, which execute these corpora",
        "suites": {
            "ngspice": {
                "decks": count_files(&tests.join("ngspice"), &["cir"]),
                "reference_captures": count_files(&tests.join("ngspice"), &["out"]),
            },
            "xyce": {
                "decks": count_files(&tests.join("xyce").join("Netlists"), &["cir"]),
                "reference_captures": count_files(&tests.join("xyce"), &["prn"]),
            },
            // The foundry corpus spells its decks `.spice` and keeps its
            // per-case references as `.tsv` tables beside them.
            "gf180mcu": {
                "decks": count_files(&tests.join("gf180mcu"), &["spice"]),
                "reference_captures": count_files(&tests.join("gf180mcu"), &["tsv"]),
            },
            "execution_iscas85": {
                "decks": count_files(&tests.join("iscas85"), &["net"]),
                "reference_captures": 0,
            },
            "execution_paranoia": {
                "decks": count_files(&tests.join("paranoia"), &["cir", "sp", "deck"]),
                "reference_captures": count_files(&tests.join("paranoia"), &["out"]),
            },
            "analytical": {
                "decks": 0,
                "reference_captures": count_files(
                    &workspace_root().join("crates/rspice-core/tests/testdata"),
                    &["dat"],
                ),
            },
        }
    })
}

//=============================================================================
// Result-document structure digests
//=============================================================================

/// Digest the *structure* of a document: identity, schema, axis and signal
/// descriptors, availability, scalar identities, payload family and field
/// names. Sample values are excluded on purpose.
///
/// A digest over the values would be a claim that two targets produce
/// bit-identical IEEE results, which RSpice does not promise; run-to-run
/// determinism on one host is `determinism.rs`'s claim, not this file's. What
/// a structure digest does catch is exactly what a baseline is for: a channel
/// that stopped being retained, a unit that changed, a payload field that
/// appeared or vanished, an availability that flipped.
fn structure_digest(document: &AnalysisResultDocument) -> String {
    let json: Json = serde_json::from_str(&document.to_json().expect("a valid document encodes"))
        .expect("a document's own encoding parses");
    let mut hasher = blake3::Hasher::new();
    hash_structure(&json, &mut hasher);
    hasher.finalize().to_hex().to_string()
}

/// Walk a JSON value hashing keys, container shapes, and strings, but
/// replacing every number and boolean with a placeholder.
fn hash_structure(value: &Json, hasher: &mut blake3::Hasher) {
    match value {
        Json::Null => {
            hasher.update(b"null");
        }
        // Booleans and numbers are both *values*: a converged flag and a node
        // voltage are equally free to differ between targets, so both hash to
        // a placeholder and only their position in the document is recorded.
        Json::Bool(_) | Json::Number(_) => {
            hasher.update(b"#");
        }
        Json::String(text) => {
            hasher.update(b"s");
            hasher.update(&(text.len() as u64).to_le_bytes());
            hasher.update(text.as_bytes());
        }
        // A run of samples is a *quantity*, not a shape: how many points an
        // adaptive transient accepted is judged by a gate with a tolerance,
        // not by an exact digest. So an array whose every element is a value
        // hashes as one placeholder and its length is dropped. Every other
        // array — signals, scalars, axes, device states, payload records —
        // keeps its length, which is what makes a dropped channel a failure.
        Json::Array(items) => {
            let samples_only = items
                .iter()
                .all(|item| matches!(item, Json::Null | Json::Bool(_) | Json::Number(_)));
            if samples_only {
                hasher.update(b"[#]");
                return;
            }
            hasher.update(b"[");
            hasher.update(&(items.len() as u64).to_le_bytes());
            for item in items {
                hash_structure(item, hasher);
            }
            hasher.update(b"]");
        }
        Json::Object(entries) => {
            hasher.update(b"{");
            hasher.update(&(entries.len() as u64).to_le_bytes());
            for (key, item) in entries {
                hasher.update(&(key.len() as u64).to_le_bytes());
                hasher.update(key.as_bytes());
                hash_structure(item, hasher);
            }
            hasher.update(b"}");
        }
    }
}

//=============================================================================
// Measurement
//=============================================================================

/// One gated metric: what was measured, and how far it may move.
struct Gate {
    metric: &'static str,
    unit: &'static str,
    value: f64,
    tolerance: f64,
    note: &'static str,
}

impl Gate {
    fn to_json(&self) -> Json {
        json!({
            "metric": self.metric,
            "unit": self.unit,
            "value": self.value,
            "tolerance": self.tolerance,
            "note": self.note,
        })
    }
}

/// Every gated count this build produces.
fn gates() -> Vec<Gate> {
    // Abort polls are control flow, so they do not move between build
    // profiles at all; the headroom is for a solver change that adds or
    // removes a poll site, not for measurement noise.
    const POLL_TOLERANCE: f64 = 0.15;
    // Accepted-point counts come out of an adaptive controller, so they can
    // differ slightly where the floating-point result does.
    const SHAPE_TOLERANCE: f64 = 0.10;
    // Serialized sizes track how many digits a value prints as.
    const BYTES_TOLERANCE: f64 = 0.10;

    let (engine, netlist) = fixture();
    let ac_frequencies = decade_sweep(200);
    let noise_frequencies = decade_sweep(120);
    let mut gates = Vec::new();

    // --- Operating point ----------------------------------------------------
    let op_polls = poll_count("operating point", |abort| {
        engine.run_dc_op_with_abort(&netlist, abort)
    });
    gates.push(Gate {
        metric: "op.abort_polls",
        unit: "polls",
        value: op_polls as f64,
        tolerance: POLL_TOLERANCE,
        note: "abort polls a completed operating point performs",
    });

    // --- DC sweep -----------------------------------------------------------
    let dc_polls = poll_count("DC sweep", |abort| {
        engine.run_dc_sweep_with_abort(&netlist, "v1", 0.0, 5.0, 0.05, abort)
    });
    gates.push(Gate {
        metric: "dc.abort_polls",
        unit: "polls",
        value: dc_polls as f64,
        tolerance: POLL_TOLERANCE,
        note: "abort polls a completed 101-point DC sweep performs",
    });
    gates.push(Gate {
        metric: "dc.abort_polls_after_cancel",
        unit: "polls",
        value: polls_after_cancel("DC sweep", dc_polls, |abort| {
            engine.run_dc_sweep_with_abort(&netlist, "v1", 0.0, 5.0, 0.05, abort)
        }) as f64,
        tolerance: 0.0,
        note: "a cancelled DC sweep must stop at the poll that cancelled it",
    });

    // --- AC -----------------------------------------------------------------
    let ac = serialized(|| engine.run_ac_with_abort(&netlist, &ac_frequencies, &NoAbort))
        .expect("the AC fixture solves");
    let ac_polls = poll_count("AC", |abort| {
        engine.run_ac_with_abort(&netlist, &ac_frequencies, abort)
    });
    gates.push(Gate {
        metric: "ac.points",
        unit: "points",
        value: ac.len() as f64,
        tolerance: 0.0,
        note: "an AC sweep returns exactly the frequencies it was asked for",
    });
    gates.push(Gate {
        metric: "ac.abort_polls",
        unit: "polls",
        value: ac_polls as f64,
        tolerance: POLL_TOLERANCE,
        note: "abort polls a completed 200-point AC sweep performs",
    });
    gates.push(Gate {
        metric: "ac.abort_polls_after_cancel",
        unit: "polls",
        value: polls_after_cancel("AC", ac_polls, |abort| {
            engine.run_ac_with_abort(&netlist, &ac_frequencies, abort)
        }) as f64,
        tolerance: 0.0,
        note: "a cancelled AC sweep must stop at the poll that cancelled it",
    });
    let ac_document = AnalysisResultDocument::from_ac(analysis_id(AnalysisKind::Ac), &ac)
        .expect("the AC result projects")
        .build()
        .expect("the AC document is valid");
    gates.push(Gate {
        metric: "ac.document_values",
        unit: "values",
        value: ac_document.total_value_count() as f64,
        tolerance: 0.0,
        note: "values the shared AC document retains",
    });
    gates.push(Gate {
        metric: "ac.document_bytes",
        unit: "bytes",
        value: ac_document
            .to_json()
            .expect("the AC document encodes")
            .len() as f64,
        tolerance: BYTES_TOLERANCE,
        note: "serialized size of the shared AC document",
    });

    // --- Transient ----------------------------------------------------------
    let transient = serialized(|| engine.run_tran_with_abort(&netlist, 2.0e-5, 2.0e-8, &NoAbort))
        .expect("the transient fixture solves");
    let tran_polls = poll_count("transient", |abort| {
        engine.run_tran_with_abort(&netlist, 2.0e-5, 2.0e-8, abort)
    });
    gates.push(Gate {
        metric: "tran.points",
        unit: "points",
        value: transient.time.len() as f64,
        tolerance: SHAPE_TOLERANCE,
        note: "accepted transient samples; adaptive, so judged against a tolerance",
    });
    gates.push(Gate {
        metric: "tran.abort_polls",
        unit: "polls",
        value: tran_polls as f64,
        tolerance: POLL_TOLERANCE,
        note: "abort polls a completed transient performs",
    });
    gates.push(Gate {
        metric: "tran.abort_polls_after_cancel",
        unit: "polls",
        value: polls_after_cancel("transient", tran_polls, |abort| {
            engine.run_tran_with_abort(&netlist, 2.0e-5, 2.0e-8, abort)
        }) as f64,
        tolerance: 0.0,
        note: "a cancelled transient must stop at the poll that cancelled it",
    });
    let tran_document = AnalysisResultDocument::from_transient(
        analysis_id(AnalysisKind::Tran),
        &transient,
        None,
        Vec::new(),
    )
    .expect("the transient result projects")
    .build()
    .expect("the transient document is valid");
    // Result-copy count. The engine's retained waveform columns are the
    // denominator; the shared document's total is the numerator. The baseline
    // ratio is above one because the document also carries the time axis and
    // the payload's step-size column, which the engine holds separately — but
    // it is a *fixed* number of extra columns, so the ratio only moves when
    // the publish path starts holding another copy of the sample set.
    let engine_samples: usize = transient
        .voltages
        .iter()
        .chain(&transient.branch_currents)
        .map(Vec::len)
        .sum::<usize>()
        + transient.time.len();
    gates.push(Gate {
        metric: "tran.document_values_per_engine_sample",
        unit: "ratio",
        value: tran_document.total_value_count() as f64 / engine_samples as f64,
        tolerance: 0.05,
        note: "shared-document values per engine sample; a step change means the publish path \
               copied the waveform set again",
    });
    gates.push(Gate {
        metric: "tran.document_bytes_per_point",
        unit: "bytes",
        value: tran_document
            .to_json()
            .expect("the transient document encodes")
            .len() as f64
            / transient.time.len() as f64,
        tolerance: BYTES_TOLERANCE,
        note: "serialized bytes of the shared transient document per accepted point",
    });

    // --- Noise --------------------------------------------------------------
    let noise = serialized(|| {
        engine.run_noise_with_abort(&netlist, 2, &noise_frequencies, 300.15, &NoAbort)
    })
    .expect("the noise fixture solves");
    let noise_polls = poll_count("noise", |abort| {
        engine.run_noise_with_abort(&netlist, 2, &noise_frequencies, 300.15, abort)
    });
    gates.push(Gate {
        metric: "noise.points",
        unit: "points",
        value: noise.len() as f64,
        tolerance: 0.0,
        note: "a noise sweep returns exactly the frequencies it was asked for",
    });
    gates.push(Gate {
        metric: "noise.abort_polls",
        unit: "polls",
        value: noise_polls as f64,
        tolerance: POLL_TOLERANCE,
        note: "abort polls a completed 120-point noise sweep performs",
    });

    // --- Harmonic balance and PSS -------------------------------------------
    let hb_polls = poll_count("harmonic balance", |abort| {
        engine.run_hb_with_abort(&netlist, HbConfig::new(1.0e6), abort)
    });
    gates.push(Gate {
        metric: "hb.abort_polls",
        unit: "polls",
        value: hb_polls as f64,
        tolerance: POLL_TOLERANCE,
        note: "abort polls a completed harmonic balance performs",
    });
    let hb = serialized(|| engine.run_hb_with_abort(&netlist, HbConfig::new(1.0e6), &NoAbort))
        .expect("the harmonic-balance fixture solves");
    gates.push(Gate {
        metric: "hb.newton_iterations",
        unit: "iterations",
        value: hb.result.iterations as f64,
        // Exact, because the fixture is linear: harmonic balance has nothing
        // to iterate on, and any nonzero count means it started a Newton loop
        // over a problem that has none.
        tolerance: 0.0,
        note: "Newton iterations harmonic balance reports on a linear circuit",
    });
    let pss_polls = poll_count("PSS", |abort| {
        engine.run_pss_with_abort(&netlist, PssConfig::new(1.0e6), abort)
    });
    gates.push(Gate {
        metric: "pss.abort_polls",
        unit: "polls",
        value: pss_polls as f64,
        tolerance: POLL_TOLERANCE,
        note: "abort polls a completed periodic steady-state solve performs",
    });
    let pss = serialized(|| engine.run_pss_with_abort(&netlist, PssConfig::new(1.0e6), &NoAbort))
        .expect("the PSS fixture solves");
    gates.push(Gate {
        metric: "pss.shooting_iterations",
        unit: "iterations",
        value: pss.iterations as f64,
        tolerance: POLL_TOLERANCE,
        note: "shooting Newton iterations the PSS solve reports",
    });

    // --- Compression --------------------------------------------------------
    let compressed = serialized(|| {
        engine.run_tran_compressed_with_abort(
            &netlist,
            2.0e-5,
            2.0e-8,
            // Loose enough that the decimator actually decimates: at 1e-9
            // absolute the RC waveform retains every accepted point, and a
            // compression gate whose ratio is exactly one measures nothing.
            CompressionConfig {
                enabled: true,
                abs_tol: 1.0e-4,
                rel_tol: 1.0e-3,
                maximum_retained_interval: 0.0,
            },
            &NoAbort,
        )
    })
    .expect("the compressed transient fixture solves");
    let report = &compressed.compression_report;
    gates.push(Gate {
        metric: "compression.retained_fraction",
        unit: "ratio",
        value: report.retained_points as f64 / report.input_points as f64,
        tolerance: 0.05,
        note: "retained points over input points at 1e-4 absolute / 1e-3 relative tolerance",
    });
    gates.push(Gate {
        metric: "compression.worst_tolerance_utilization",
        unit: "ratio",
        value: report
            .worst_observed
            .as_ref()
            .map_or(0.0, |observation| observation.tolerance_utilization),
        tolerance: 0.25,
        note: "worst reconstruction error as a fraction of the tolerance it was allowed",
    });

    // --- Checkpoint and output throughput -----------------------------------
    let (_, checkpoint) = serialized(|| engine.run_tran_checkpointed(&netlist, 1.0e-5, 2.0e-8))
        .expect("the checkpoint fixture solves its first segment");
    let unpacked = checkpoint
        .to_bytes(TransientCheckpointEncoding::Unpacked)
        .expect("the checkpoint encodes unpacked");
    let packed = checkpoint
        .to_bytes(TransientCheckpointEncoding::Packed)
        .expect("the checkpoint packs");
    gates.push(Gate {
        metric: "checkpoint.unpacked_bytes",
        unit: "bytes",
        value: unpacked.len() as f64,
        tolerance: BYTES_TOLERANCE,
        note: "canonical checkpoint text size for a fixed first segment",
    });
    gates.push(Gate {
        metric: "checkpoint.packed_over_unpacked",
        unit: "ratio",
        value: packed.len() as f64 / unpacked.len() as f64,
        tolerance: 0.20,
        note: "packed envelope size relative to the canonical text it authenticates",
    });

    gates
}

/// The facts this build declares about itself.
fn facts() -> Json {
    let (engine, netlist) = fixture();
    let operating_point = serialized(|| engine.run_dc_op_with_abort(&netlist, &NoAbort))
        .expect("the operating-point fixture solves");
    let op_document = AnalysisResultDocument::from_operating_point(
        analysis_id(AnalysisKind::Op),
        &operating_point,
        None,
    )
    .expect("the operating point projects")
    .build()
    .expect("the operating-point document is valid");
    let ac = serialized(|| engine.run_ac_with_abort(&netlist, &decade_sweep(16), &NoAbort))
        .expect("the AC fixture solves");
    let ac_document = AnalysisResultDocument::from_ac(analysis_id(AnalysisKind::Ac), &ac)
        .expect("the AC result projects")
        .build()
        .expect("the AC document is valid");
    let transient = serialized(|| engine.run_tran_with_abort(&netlist, 2.0e-6, 2.0e-8, &NoAbort))
        .expect("the transient fixture solves");
    let tran_document = AnalysisResultDocument::from_transient(
        analysis_id(AnalysisKind::Tran),
        &transient,
        None,
        Vec::new(),
    )
    .expect("the transient result projects")
    .build()
    .expect("the transient document is valid");
    let noise = serialized(|| {
        engine.run_noise_with_abort(&netlist, 2, &decade_sweep(16), 300.15, &NoAbort)
    })
    .expect("the noise fixture solves");
    let noise_document =
        AnalysisResultDocument::from_noise(analysis_id(AnalysisKind::Noise), &noise)
            .expect("the noise result projects")
            .build()
            .expect("the noise document is valid");

    json!({
        "toolchain": {
            "minimum_rust_version": env!("CARGO_PKG_RUST_VERSION"),
            "edition": "2024",
        },
        "required_features": {
            "note": "rspice-core optional capabilities that must be present for the claims below \
                     to mean what they say. Compared as a lower bound, not for equality: a \
                     workspace build unifies features across its members, so a run that enables \
                     more than this is still a valid build of the same contract, while one that \
                     enables less is not.",
            "veriloga": cfg!(feature = "veriloga"),
            "veriloga_native": cfg!(feature = "veriloga-native"),
            "veriloga_builtins_base": cfg!(feature = "veriloga-builtins-base"),
        },
        "result_document": {
            "schema": ANALYSIS_RESULT_DOCUMENT_SCHEMA,
            "schema_version": ANALYSIS_RESULT_DOCUMENT_VERSION,
        },
        "capability": capability_json(),
        "oracle_evidence": oracle_evidence_json(),
        "result_document_structure_blake3": {
            "note": "BLAKE3 over each document's identity, descriptors, availability, units and \
                     payload shape, with every numeric value replaced by a placeholder: IEEE \
                     results are not bit-identical across targets, and this baseline is compared \
                     on all of them",
            "op": structure_digest(&op_document),
            "ac": structure_digest(&ac_document),
            "tran": structure_digest(&tran_document),
            "noise": structure_digest(&noise_document),
        },
    })
}

fn measure() -> Json {
    json!({
        "schema": BASELINE_SCHEMA,
        "schema_version": BASELINE_SCHEMA_VERSION,
        "description": "Qualification baseline: what this build supports, and what it costs in \
                        counts. Wall-clock and resident set size are deliberately absent — a \
                        shared runner cannot measure either reproducibly — so every performance \
                        claim here is a count judged against a declared tolerance.",
        "facts": facts(),
        "gates": gates().iter().map(Gate::to_json).collect::<Vec<_>>(),
    })
}

//=============================================================================
// Comparison
//=============================================================================

fn gate_map(document: &Json) -> BTreeMap<String, (f64, f64)> {
    document["gates"]
        .as_array()
        .expect("a baseline carries a gate array")
        .iter()
        .map(|gate| {
            (
                gate["metric"]
                    .as_str()
                    .expect("a gate names its metric")
                    .to_owned(),
                (
                    gate["value"].as_f64().expect("a gate carries a value"),
                    gate["tolerance"]
                        .as_f64()
                        .expect("a gate declares a tolerance"),
                ),
            )
        })
        .collect()
}

#[test]
fn the_checked_in_qualification_baseline_is_current() {
    let measured = measure();
    let path = baseline_path();

    if std::env::var_os(UPDATE_ENV).is_some() {
        std::fs::create_dir_all(path.parent().expect("the baseline has a directory"))
            .expect("create the baseline directory");
        let mut rendered =
            serde_json::to_string_pretty(&measured).expect("the baseline serializes");
        rendered.push('\n');
        std::fs::write(&path, rendered).expect("write the regenerated baseline");
        println!("regenerated {}", path.display());
        return;
    }

    let checked_in: Json = serde_json::from_slice(&std::fs::read(&path).unwrap_or_else(|error| {
        panic!(
            "the qualification baseline is missing from {}: {error}. Regenerate it with \
                 {UPDATE_ENV}=1",
            path.display()
        )
    }))
    .expect("the checked-in qualification baseline is JSON");

    assert_eq!(
        checked_in["schema"], measured["schema"],
        "the checked-in baseline is a different document"
    );
    assert_eq!(
        checked_in["schema_version"], measured["schema_version"],
        "the baseline schema version moved; regenerate with {UPDATE_ENV}=1 and read the diff"
    );

    // Facts are exact: none of them depends on a floating-point result.
    for section in [
        "toolchain",
        "result_document",
        "capability",
        "oracle_evidence",
        "result_document_structure_blake3",
    ] {
        assert_eq!(
            checked_in["facts"][section], measured["facts"][section],
            "the '{section}' facts drifted from the checked-in baseline; regenerate with \
             {UPDATE_ENV}=1 once the change is intended"
        );
    }

    // Features are a lower bound rather than an equality: `cargo test
    // --workspace` unifies features across members, so a nightly run enables
    // everything any member asks for. A build that enables *more* than the
    // baseline declares still satisfies its claims; one that enables less does
    // not, because the capability matrix and the document digests were
    // measured with those capabilities present.
    let required = checked_in["facts"]["required_features"]
        .as_object()
        .expect("the baseline declares its required features");
    for (feature, declared) in required {
        if declared.as_bool() != Some(true) {
            continue;
        }
        assert_eq!(
            measured["facts"]["required_features"][feature].as_bool(),
            Some(true),
            "the baseline was captured with '{feature}' enabled and this build does not have it"
        );
    }

    // Gates are judged against their own declared tolerance.
    let expected = gate_map(&checked_in);
    let actual = gate_map(&measured);
    let missing = expected
        .keys()
        .filter(|metric| !actual.contains_key(*metric))
        .collect::<Vec<_>>();
    let added = actual
        .keys()
        .filter(|metric| !expected.contains_key(*metric))
        .collect::<Vec<_>>();
    assert!(
        missing.is_empty() && added.is_empty(),
        "the gate set changed: no longer measured {missing:?}, newly measured {added:?}; \
         regenerate with {UPDATE_ENV}=1"
    );

    let mut regressions = Vec::new();
    for (metric, (baseline_value, tolerance)) in expected {
        let (measured_value, _) = actual[&metric];
        let allowed = baseline_value.abs() * tolerance;
        let drift = (measured_value - baseline_value).abs();
        if drift > allowed {
            regressions.push(format!(
                "{metric}: baseline {baseline_value}, measured {measured_value} \
                 (drift {drift:.3} > allowed {allowed:.3} at tolerance {tolerance})"
            ));
        }
    }
    assert!(
        regressions.is_empty(),
        "{} qualification metric(s) moved beyond their declared tolerance:\n{}",
        regressions.len(),
        regressions.join("\n")
    );
}

#[test]
fn every_gate_declares_a_unit_a_tolerance_and_a_reason() {
    let path = baseline_path();
    let checked_in: Json = serde_json::from_slice(
        &std::fs::read(&path).expect("the checked-in qualification baseline is readable"),
    )
    .expect("the checked-in qualification baseline is JSON");

    let gates = checked_in["gates"]
        .as_array()
        .expect("a baseline carries a gate array");
    assert!(!gates.is_empty(), "a baseline with no gates gates nothing");
    for gate in gates {
        let metric = gate["metric"].as_str().expect("a gate names its metric");
        assert!(
            gate["unit"].as_str().is_some_and(|unit| !unit.is_empty()),
            "{metric} declares no unit"
        );
        assert!(
            gate["note"].as_str().is_some_and(|note| !note.is_empty()),
            "{metric} declares no reason, so a reader cannot tell what a regression means"
        );
        let tolerance = gate["tolerance"]
            .as_f64()
            .expect("a gate declares a tolerance");
        assert!(
            (0.0..=0.5).contains(&tolerance),
            "{metric} declares a tolerance of {tolerance}; above 0.5 a gate catches nothing"
        );
        assert!(
            !gate["metric"]
                .as_str()
                .unwrap_or_default()
                .contains("seconds"),
            "{metric} looks like a wall-clock metric; this baseline gates counts only"
        );
    }
}
