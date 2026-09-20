//! The gate-level scale suite: twelve original circuits, their generator, and
//! the reference models that say what each one computes.
//!
//! # What this file asserts, and against what
//!
//! * [`the_vendored_corpus_is_what_the_generator_emits`] — the files under
//!   `tests/verilog/scale/` are byte-for-byte what the generator produces.
//!   The generator is the source of truth and a hand edit is a failing test.
//! * [`the_structural_coverage_matrix_holds`] — the workload the suite is
//!   *for*: every gate primitive used somewhere, a gate of fan-in eight, a net
//!   driving a hundred gate inputs, a hundred-level combinational path, a
//!   circuit with two hundred scalar ports, buffer chains, module instances by
//!   the hundred. Each one is measured from the generated netlist and
//!   compared against a floor, so a circuit that quietly stops providing its
//!   trait fails here rather than silently.
//! * [`rspice_matches_the_reference_models`] — the correctness claim. Every
//!   vector of every circuit, through RSpice, against a model that implements
//!   the specification and has never seen a gate.
//! * [`the_nand_twin_agrees_with_its_primitive_twin`] — `sec32` and `sec32n`
//!   are the same function in two structures and must produce identical
//!   traces. A third oracle, and one that needs nothing installed.
//! * [`icarus_and_verilator_agree_on_the_scale_corpus`] and
//!   [`rspice_agrees_with_icarus_on_the_scale_corpus`] — the same external
//!   oracles the parent corpus uses, on the same terms: they skip with a
//!   diagnostic while the binaries are absent, and
//!   `RSPICE_VERILOG_ORACLES_REQUIRED=1` makes that absence a failure.
//!
//! # Regenerating
//!
//! `RSPICE_VERILOG_SCALE_WRITE=1 cargo test -p rspice-conformance --test
//! verilog_scale` rewrites the vendored files from the generator and then
//! passes. Run it after any generator change, and read the diff: a change to
//! one circuit that renumbers another's nets is a bug in the generator, not a
//! refresh.

use rspice_conformance::suites::verilog::{
    Corpus, VerilogEngine, corpus_dir,
    oracle::{self, oracles_required},
    run::{DEFAULT_TIMEOUT_MS, RunError, compare_engines, run_case},
    scale::{self, netlist::Gate, reference, vectors},
    testbench, trace,
};
use std::collections::BTreeSet;
use std::fs;
use std::time::Instant;

/// The environment variable that rewrites the vendored corpus.
const WRITE_ENV: &str = "RSPICE_VERILOG_SCALE_WRITE";

fn scale_corpus() -> Corpus {
    Corpus::load(&scale::scale_dir())
        .unwrap_or_else(|err| panic!("the scale corpus must load: {err}"))
}

// ===========================================================================
// The generator is the source of truth
// ===========================================================================

#[test]
fn the_vendored_corpus_is_what_the_generator_emits() {
    let root = scale::scale_dir();
    let files = scale::files();
    let write = std::env::var(WRITE_ENV).is_ok_and(|value| value == "1");

    if write {
        fs::create_dir_all(&root).expect("the corpus directory must exist");
        for (name, contents) in &files {
            fs::write(root.join(name), contents)
                .unwrap_or_else(|err| panic!("cannot write '{name}': {err}"));
        }
        println!("Rewrote {} file(s) under {}", files.len(), root.display());
        return;
    }

    let mut missing = Vec::new();
    let mut differing = Vec::new();
    for (name, expected) in &files {
        match fs::read_to_string(root.join(name)) {
            Err(_) => missing.push(name.clone()),
            Ok(found) if &found != expected => differing.push(name.clone()),
            Ok(_) => {}
        }
    }
    // Files on disk the generator does not produce would be run by the corpus
    // loader and maintained by nobody.
    let mut stray = Vec::new();
    if let Ok(entries) = fs::read_dir(&root) {
        for entry in entries.flatten() {
            let name = entry.file_name().to_string_lossy().into_owned();
            if entry.path().is_file() && !files.contains_key(&name) {
                stray.push(name);
            }
        }
    }
    stray.sort();

    assert!(
        missing.is_empty() && differing.is_empty() && stray.is_empty(),
        "the vendored scale corpus does not match the generator.\n  \
         missing: {missing:?}\n  differing: {differing:?}\n  unexpected: {stray:?}\n\
         Rerun with {WRITE_ENV}=1 to regenerate, then read the diff."
    );

    // Regenerating in this process must also be a no-op, which is what rules
    // out a generator whose output depends on when it ran.
    assert_eq!(files, scale::files(), "the generator is not deterministic");
}

#[test]
fn the_scale_corpus_is_complete_and_names_every_circuit() {
    let corpus = scale_corpus();
    let present: BTreeSet<&str> = corpus.cases.iter().map(|case| case.name.as_str()).collect();
    for name in scale::NAMES {
        assert!(
            present.contains(name),
            "case '{name}' is gone from the corpus"
        );
    }
    assert_eq!(present.len(), scale::NAMES.len());

    for case in &corpus.cases {
        assert!(!case.note.trim().is_empty(), "'{}' has no note", case.name);
        assert!(
            case.admits(VerilogEngine::Icarus) && case.admits(VerilogEngine::Verilator),
            "'{}' must be comparable against both oracles; every scale case is \
             two-state clean",
            case.name
        );
        // Two-state clean is a property of the vectors, so check it rather
        // than assert it in prose.
        for vector in &case.stimulus.vectors {
            for value in vector {
                assert!(
                    value.chars().all(|ch| ch == '0' || ch == '1'),
                    "'{}' drives '{value}', and the scale suite is two-state",
                    case.name
                );
            }
        }
    }

    // The two corpora must stay disjoint: a name in both would make "which
    // one failed?" ambiguous in every report.
    let parent = Corpus::load(&corpus_dir()).expect("the parent corpus loads");
    for case in &parent.cases {
        assert!(
            !present.contains(case.name.as_str()),
            "'{}' is in both corpora",
            case.name
        );
    }
}

#[test]
fn no_scale_design_reports_its_own_results() {
    let corpus = scale_corpus();
    for case in &corpus.cases {
        let source = fs::read_to_string(&case.source).expect("readable");
        let code = source
            .lines()
            .map(|line| line.split("//").next().unwrap_or_default())
            .collect::<Vec<_>>()
            .join("\n");
        for forbidden in ["$display", "$monitor", "$write", "$dumpvars", "$finish"] {
            assert!(
                !code.contains(forbidden),
                "'{}' uses {forbidden}; designs are observed through their ports",
                case.name
            );
        }
        assert!(
            !code.contains("initial"),
            "'{}' has an initial block; the testbench owns all stimulus",
            case.name
        );
    }
}

#[test]
fn every_scale_case_generates_a_well_formed_testbench() {
    let corpus = scale_corpus();
    for case in &corpus.cases {
        let bench = testbench::render(&case.stimulus, &case.name);
        assert!(bench.contains(&format!("{} rspice_dut (", case.stimulus.module)));
        assert_eq!(
            bench.matches("rspice_sample;").count(),
            case.stimulus.vectors.len() + 1,
            "'{}': one sample per vector",
            case.name
        );
        assert_eq!(bench, testbench::render(&case.stimulus, &case.name));
    }
}

// ===========================================================================
// The structural workload
// ===========================================================================

/// The floors the suite exists to clear, each measured from a generated
/// netlist.
///
/// These are what makes the suite a *replacement* for a gate-level benchmark
/// family rather than a set of circuits of similar size. Each row names the
/// workload it stands for; the numbers are floors, so a circuit that grows
/// past one still passes and a circuit that stops providing it does not.
#[test]
fn the_structural_coverage_matrix_holds() {
    let mut vocabulary = BTreeSet::new();
    let mut widest_gate = 0usize;
    let mut heaviest_net = 0usize;
    let mut deepest = 0usize;
    let mut most_instances = 0usize;
    let mut widest_ports = 0usize;
    let mut scalar_ported = Vec::new();
    let mut buffered = 0usize;

    for name in scale::NAMES {
        let Some(metrics) = scale::metrics(name) else {
            continue;
        };
        println!(
            "{name:<10} gates {:>5}  instances {:>4}  depth {:>4}  fan-in {:>2}  \
             fan-out {:>4}  ports {:>4}",
            metrics.gates,
            metrics.instances,
            metrics.depth,
            metrics.max_fan_in,
            metrics.max_fan_out,
            metrics.ports
        );
        vocabulary.extend(metrics.kinds.iter().copied());
        widest_gate = widest_gate.max(metrics.max_fan_in);
        heaviest_net = heaviest_net.max(metrics.max_fan_out);
        deepest = deepest.max(metrics.depth);
        most_instances = most_instances.max(metrics.instances);
        widest_ports = widest_ports.max(metrics.ports);
        if metrics.scalar_ported && metrics.gates >= 1_000 {
            scalar_ported.push(name);
        }
        if metrics.kinds.contains(&Gate::Buf) {
            buffered += 1;
        }
    }

    // 1. Every gate primitive the front end implements appears somewhere.
    let missing: Vec<&str> = Gate::ALL
        .iter()
        .filter(|kind| !vocabulary.contains(kind))
        .map(|kind| kind.keyword())
        .collect();
    assert!(
        missing.is_empty(),
        "no scale circuit uses these primitives: {missing:?}"
    );

    // 2. Large fan-in: not every gate is two-input.
    assert!(
        widest_gate >= 8,
        "the widest gate in the suite has {widest_gate} inputs"
    );

    // 3. High fanout: one net broadcasting across a datapath.
    assert!(
        heaviest_net >= 100,
        "the heaviest net in the suite drives {heaviest_net} gate inputs"
    );

    // 4. Extreme depth, in a circuit with narrow I/O.
    let multiplier = scale::metrics("mul16").expect("mul16 is structural");
    assert!(
        multiplier.depth >= 80,
        "mul16's longest path is {} gate levels",
        multiplier.depth
    );
    assert!(multiplier.ports <= 96, "mul16 is the depth-to-I/O extreme");
    assert!(deepest >= 80);

    // 5. Wide, shallow I/O.
    assert!(
        widest_ports >= 200,
        "the widest circuit has {widest_ports} ports"
    );
    let wide = scale::metrics("addcmp32").expect("addcmp32 is structural");
    assert!(wide.ports >= 200 && wide.depth <= 80, "{wide:?}");

    // 6. Scalar-port explosion: a large circuit whose whole interface is
    //    one-bit ports.
    assert!(
        !scalar_ported.is_empty(),
        "no large circuit exposes its interface as scalar ports"
    );

    // 8. Hierarchy at scale.
    assert!(
        most_instances >= 200,
        "the most hierarchical circuit has {most_instances} instances"
    );

    // 10. Buffer chains, from the fanout-limiting pass and the control
    //     decoders' explicit drive stages.
    assert!(buffered >= 8, "only {buffered} circuits carry buffers");

    // The NAND twin is exactly that: one primitive, and no other.
    let twin = scale::metrics("sec32n").expect("sec32n is structural");
    assert_eq!(
        twin.kinds,
        [Gate::Nand].into_iter().collect::<BTreeSet<_>>(),
        "sec32n must be NAND-only, and is not"
    );
    let primitive = scale::metrics("sec32").expect("sec32 is structural");
    assert!(primitive.kinds.contains(&Gate::Xor) && primitive.kinds.contains(&Gate::Xnor));
    assert_ne!(
        primitive.gates, twin.gates,
        "the pair must be structurally different, not the same netlist twice"
    );
}

/// Reconvergent fanout, stated as a measurement rather than as prose.
///
/// A Hamming syndrome bit and the position decoder it feeds are the textbook
/// reconvergence: each data bit reaches several syndrome trees and those trees
/// meet again at every decoder cell, so no single path explains a corrected
/// output. What is checked here is that the structure really is that shape —
/// a data bit fanning out to more than one syndrome tree — because a decoder
/// wired one-bit-per-syndrome would pass every functional test and provide
/// none of the workload.
#[test]
fn the_ecc_circuits_reconverge() {
    for (name, parity, width) in [("sec32", 6u32, 32usize), ("secded16", 5, 16)] {
        let positions =
            rspice_conformance::suites::verilog::scale::circuits::hamming_data_positions(
                parity, width,
            );
        let branching = positions
            .iter()
            .filter(|position| position.count_ones() >= 2)
            .count();
        assert!(
            branching * 2 >= width,
            "{name}: only {branching} of {width} data bits reach more than one \
             syndrome tree, so the decoder barely reconverges"
        );
        let deepest = positions
            .iter()
            .map(|position| position.count_ones())
            .max()
            .unwrap_or(0);
        assert!(
            deepest >= 3,
            "{name}: no data bit reaches three syndrome trees"
        );
    }
}

// ===========================================================================
// Correctness
// ===========================================================================

/// One trace row, written the way the trace grammar writes it.
fn row_text(row: &trace::TraceRow) -> String {
    row.values
        .iter()
        .map(|(name, value)| format!("{name}={value}"))
        .collect::<Vec<_>>()
        .join(" ")
}

/// The rows the reference model says a circuit must produce.
fn expected_rows(name: &str) -> Vec<String> {
    let spec = vectors::spec(name).expect("every circuit has a stimulus");
    let outputs = spec.observed();
    let answers: Vec<reference::Values> = if spec.clock.is_some() {
        reference::evaluate_sequence(name, &spec.vectors)
    } else {
        spec.vectors
            .iter()
            .map(|inputs| reference::evaluate(name, inputs))
            .collect()
    };
    answers
        .iter()
        .map(|answer| {
            outputs
                .iter()
                .map(|port| {
                    let value = reference::word(answer, &port.name, port.width);
                    format!("{}={}", port.name, vectors::binary(value, port.width))
                })
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect()
}

/// Every circuit, every vector, against a model that has never seen a gate.
///
/// The wall time of each case is printed because it is the number that decides
/// which CI tier this suite belongs in, and a number nobody measures is a
/// number that drifts.
#[test]
fn rspice_matches_the_reference_models() {
    let corpus = scale_corpus();
    let workspace = std::env::temp_dir().join("rspice-verilog-scale-expectations");
    let mut total = 0u128;

    for name in scale::NAMES {
        let case = corpus
            .case(name)
            .unwrap_or_else(|| panic!("'{name}' is in the corpus"));
        let started = Instant::now();
        let outcome = run_case(
            case,
            VerilogEngine::Rspice,
            None,
            &workspace,
            DEFAULT_TIMEOUT_MS,
        )
        .unwrap_or_else(|err| panic!("'{name}' must run on RSpice: {err}"));
        let elapsed = started.elapsed().as_millis();
        total += elapsed;

        let expected = expected_rows(name);
        assert_eq!(
            expected.len(),
            case.stimulus.vectors.len(),
            "'{name}': one expectation per vector"
        );
        let actual: Vec<String> = outcome.trace.rows.iter().map(row_text).collect();
        if actual != expected {
            let first = actual
                .iter()
                .zip(&expected)
                .position(|(left, right)| left != right)
                .unwrap_or(0);
            panic!(
                "'{name}' disagrees with its reference model at vector {first}\n  \
                 stimulus: {:?}\n  RSpice:    {}\n  reference: {}",
                case.stimulus.vectors.get(first),
                actual.get(first).map(String::as_str).unwrap_or("<missing>"),
                expected
                    .get(first)
                    .map(String::as_str)
                    .unwrap_or("<missing>"),
            );
        }
        println!(
            "{name:<10} {:>4} vectors  {elapsed:>6} ms",
            case.stimulus.vectors.len()
        );
    }
    println!("RSpice ran the whole scale suite in {total} ms.");
    assert!(!workspace.exists(), "the RSpice arm must leave no debris");
}

/// The same function through two structures, compared to each other.
#[test]
fn the_nand_twin_agrees_with_its_primitive_twin() {
    let corpus = scale_corpus();
    let workspace = std::env::temp_dir().join("rspice-verilog-scale-twins");
    let divergences = compare_engines(
        corpus.case("sec32").expect("in the corpus"),
        (VerilogEngine::Rspice, None),
        (VerilogEngine::Rspice, None),
        &workspace,
        DEFAULT_TIMEOUT_MS,
    )
    .expect("sec32 runs");
    assert!(divergences.is_empty(), "sec32 disagrees with itself");

    let primitive = run_case(
        corpus.case("sec32").expect("in the corpus"),
        VerilogEngine::Rspice,
        None,
        &workspace,
        DEFAULT_TIMEOUT_MS,
    )
    .expect("sec32 runs");
    let nand_only = run_case(
        corpus.case("sec32n").expect("in the corpus"),
        VerilogEngine::Rspice,
        None,
        &workspace,
        DEFAULT_TIMEOUT_MS,
    )
    .expect("sec32n runs");

    let divergences = trace::compare_traces(&primitive.trace, &nand_only.trace);
    assert!(
        divergences.is_empty(),
        "the structural pair disagrees: {}",
        trace::describe("sec32", "sec32n", &divergences)
    );
    assert!(!primitive.trace.rows.is_empty());
}

/// The error-injection coverage the ECC circuits are here for.
///
/// Checked on the reference model, because what is being asserted is that the
/// *vector set* reaches these cases at all: a suite whose ECC vectors never
/// flip a bit would pass every other test in this file.
#[test]
fn the_ecc_vectors_inject_errors_and_the_models_answer() {
    let sec = vectors::spec("sec32").expect("present");
    let mut clean = 0usize;
    let mut corrected = 0usize;
    for inputs in &sec.vectors {
        let out = reference::evaluate("sec32", inputs);
        if out["err"] == 0 {
            clean += 1;
        } else if out["syn"] != 0 {
            corrected += 1;
        }
    }
    assert!(clean >= 8, "only {clean} clean codewords");
    assert!(corrected >= 32, "only {corrected} vectors raise a syndrome");

    let secded = vectors::spec("secded16").expect("present");
    let mut single = 0usize;
    let mut double = 0usize;
    for inputs in &secded.vectors {
        let out = reference::evaluate("secded16", inputs);
        single += usize::from(out["sec"] == 1);
        double += usize::from(out["ded"] == 1);
    }
    assert!(single >= 16, "only {single} single-error corrections");
    assert!(double >= 10, "only {double} double-error detections");
}

// ===========================================================================
// External oracles
// ===========================================================================

#[test]
fn icarus_and_verilator_agree_on_the_scale_corpus() {
    let icarus = oracle::detect(VerilogEngine::Icarus);
    let verilator = oracle::detect(VerilogEngine::Verilator);
    let (Some(icarus_tools), Some(verilator_tools)) = (icarus.tools(), verilator.tools()) else {
        let reason = format!(
            "Skipping oracle-vs-oracle on the scale corpus.\n{}\n{}",
            icarus.diagnostic(),
            verilator.diagnostic()
        );
        assert!(!oracles_required(), "{reason}");
        println!("{reason}");
        return;
    };

    let corpus = scale_corpus();
    let workspace = tempfile::Builder::new()
        .prefix("rspice-verilog-scale-agreement-")
        .tempdir()
        .expect("scratch directory");
    for case in &corpus.cases {
        let divergences = compare_engines(
            case,
            (VerilogEngine::Icarus, Some(icarus_tools)),
            (VerilogEngine::Verilator, Some(verilator_tools)),
            workspace.path(),
            DEFAULT_TIMEOUT_MS,
        )
        .unwrap_or_else(|err| panic!("case '{}': {err}", case.name));
        assert!(
            divergences.is_empty(),
            "case '{}': {}",
            case.name,
            trace::describe("Icarus", "Verilator", &divergences)
        );
    }
    println!(
        "Icarus and Verilator agree on {} case(s).",
        corpus.cases.len()
    );
}

#[test]
fn rspice_agrees_with_icarus_on_the_scale_corpus() {
    let icarus = oracle::detect(VerilogEngine::Icarus);
    let Some(tools) = icarus.tools() else {
        let reason = format!(
            "Skipping RSpice-vs-Icarus on the scale corpus.\n{}",
            icarus.diagnostic()
        );
        assert!(!oracles_required(), "{reason}");
        println!("{reason}");
        return;
    };

    let corpus = scale_corpus();
    let workspace = tempfile::Builder::new()
        .prefix("rspice-verilog-scale-vs-icarus-")
        .tempdir()
        .expect("scratch directory");
    for case in &corpus.cases {
        match compare_engines(
            case,
            (VerilogEngine::Rspice, None),
            (VerilogEngine::Icarus, Some(tools)),
            workspace.path(),
            DEFAULT_TIMEOUT_MS,
        ) {
            Ok(divergences) => assert!(
                divergences.is_empty(),
                "case '{}': {}",
                case.name,
                trace::describe("RSpice", "Icarus", &divergences)
            ),
            // Unlike the parent corpus, nothing here is allowed to be refused:
            // every scale circuit is built from constructs the front end
            // already implements, and a refusal is a regression.
            Err(RunError::RspiceRefused { case, detail }) => {
                panic!("RSpice refused scale case '{case}': {detail}")
            }
            Err(err) => panic!("case '{}': {err}", case.name),
        }
    }
    println!("RSpice and Icarus agree on {} case(s).", corpus.cases.len());
}
