//! The digital Verilog corpus, its harness, RSpice, and the two independent
//! oracles.
//!
//! Most of these tests are structural and pass without any simulator
//! installed: the corpus is complete and self-consistent, every case generates
//! a well-formed testbench, the designs obey the no-self-reporting rule, and
//! oracle detection produces a typed answer either way.
//!
//! [`rspice_matches_its_independently_derived_expectations`] is the one that
//! says what RSpice actually does with this corpus, and it needs nothing
//! installed either — see its documentation for where each expectation came
//! from, which is the only thing that makes it worth anything.
//!
//! [`icarus_and_verilator_agree_on_the_two_state_corpus`] and
//! [`rspice_agrees_with_icarus_where_both_can_run_a_case`] need the binaries.
//! Without them they print what they looked for and how to install it, and
//! return. Set `RSPICE_VERILOG_ORACLES_REQUIRED=1` to make that absence a
//! failure; CI should set it as soon as the oracles are on the image, because
//! after that point "not installed" is a regression rather than a state of the
//! world.
//!
//! Run with `-- --nocapture` to see the skip diagnostics.

use rspice_conformance::suites::verilog::{
    Corpus, VerilogEngine, corpus_dir,
    oracle::{self, OracleAvailability, OracleTools, oracles_required},
    run::{DEFAULT_TIMEOUT_MS, RunError, compare_engines, run_case},
    testbench, trace,
};
use std::collections::BTreeSet;
use std::fs;

/// Cases the corpus must contain.
///
/// Pinned by name rather than counted. A count tells you the corpus changed
/// size; this tells you *which* language mechanism stopped being covered, which
/// is the question anyone reading the failure actually has. Removing a case is
/// allowed — editing this list is how you say you meant to.
const REQUIRED_CASES: [(&str, &str); 6] = [
    ("c17", "structural gate netlist (ISCAS85)"),
    (
        "gate_primitives",
        "all eight gate primitives against their operator forms",
    ),
    (
        "ripple_adder",
        "module instantiation and generate-unrolled hierarchy",
    ),
    ("dff_register", "posedge clocking and asynchronous reset"),
    ("nba_ordering", "blocking versus non-blocking assignment"),
    ("xz_propagation", "four-state X and Z propagation"),
];

fn corpus() -> Corpus {
    Corpus::load(&corpus_dir()).unwrap_or_else(|err| panic!("the Verilog corpus must load: {err}"))
}

#[test]
fn the_corpus_is_complete_and_self_consistent() {
    let corpus = corpus();

    // `Corpus::load` already enforces the hard invariants — manifest and disk
    // agree in both directions, every stimulus parses, every stimulus port
    // exists in the design it names. What is left to state here is what the
    // corpus is *for*.
    let present = corpus
        .cases
        .iter()
        .map(|case| case.name.as_str())
        .collect::<BTreeSet<_>>();
    for (name, covers) in REQUIRED_CASES {
        assert!(
            present.contains(name),
            "case '{name}' is gone, and with it the only coverage of {covers}"
        );
    }

    for case in &corpus.cases {
        assert!(
            !case.oracles.is_empty(),
            "case '{}' names no oracles, so nothing would ever check it",
            case.name
        );
        assert!(
            !case.stimulus.vectors.is_empty(),
            "case '{}' has no vectors",
            case.name
        );
        assert!(
            !case.note.trim().is_empty(),
            "case '{}' has no manifest note saying what it is for",
            case.name
        );
    }

    // The four-state case is the one entry Verilator cannot be held to. If it
    // ever silently gains Verilator, the suite would start reporting a
    // disagreement that is Verilator being two-state rather than anything
    // being wrong.
    let four_state = corpus
        .case("xz_propagation")
        .expect("checked by REQUIRED_CASES above");
    assert!(four_state.admits(VerilogEngine::Icarus));
    assert!(
        !four_state.admits(VerilogEngine::Verilator),
        "Verilator is a two-state simulator and cannot arbitrate X/Z"
    );

    // Everything else must be comparable across both, or the oracle-vs-oracle
    // check degenerates into one simulator talking to itself.
    let both = corpus
        .cases
        .iter()
        .filter(|case| case.admits(VerilogEngine::Icarus) && case.admits(VerilogEngine::Verilator))
        .count();
    assert!(
        both >= REQUIRED_CASES.len() - 1,
        "only {both} case(s) can be cross-checked between two oracles"
    );
}

#[test]
fn no_design_reports_its_own_results() {
    // The load-bearing rule of the harness. A design that prints is being
    // compared on its simulator's number formatting as much as on its
    // semantics, and an `initial` block inside the design can drive the very
    // ports the testbench drives.
    let corpus = corpus();
    for case in &corpus.cases {
        let source = fs::read_to_string(&case.source)
            .unwrap_or_else(|err| panic!("cannot read '{}': {err}", case.source.display()));
        let code = source
            .lines()
            .map(|line| line.split("//").next().unwrap_or_default())
            .collect::<Vec<_>>()
            .join("\n");

        for forbidden in ["$display", "$monitor", "$write", "$dumpvars", "$finish"] {
            assert!(
                !code.contains(forbidden),
                "case '{}' uses {forbidden}; designs are observed through their ports only",
                case.name
            );
        }
        assert!(
            !code.contains("initial"),
            "case '{}' has an initial block; the testbench owns all stimulus",
            case.name
        );
    }
}

#[test]
fn every_case_generates_a_well_formed_testbench() {
    let corpus = corpus();
    for case in &corpus.cases {
        let bench = testbench::render(&case.stimulus, &case.name);

        assert!(
            bench.contains(&format!("module {};", testbench::TOP_MODULE)),
            "case '{}': no top module",
            case.name
        );
        assert!(
            bench.contains(&format!("{} rspice_dut (", case.stimulus.module)),
            "case '{}': the design under test is not instantiated",
            case.name
        );
        assert!(
            bench.contains(testbench::TRACE_HEADER),
            "case '{}': no trace header",
            case.name
        );
        assert!(
            bench.contains("$finish;"),
            "case '{}': the run would never terminate",
            case.name
        );

        // One sample per vector, plus the task definition itself.
        let samples = bench.matches("rspice_sample;").count();
        assert_eq!(
            samples,
            case.stimulus.vectors.len() + 1,
            "case '{}': {samples} sample site(s) for {} vector(s)",
            case.name,
            case.stimulus.vectors.len()
        );

        // Every output is sampled; no input is.
        for port in case.stimulus.observed_outputs() {
            assert!(
                bench.contains(&format!("$write(\" {}=%b\", {});", port.name, port.name)),
                "case '{}': output '{}' never reaches the trace",
                case.name,
                port.name
            );
        }
        for port in case.stimulus.driven_inputs() {
            assert!(
                !bench.contains(&format!("$write(\" {}=%b\"", port.name)),
                "case '{}': input '{}' is in the trace, where it would only confirm \
                 what the harness itself wrote",
                case.name,
                port.name
            );
        }

        // Regenerating must be a no-op.
        assert_eq!(
            bench,
            testbench::render(&case.stimulus, &case.name),
            "case '{}': testbench generation is not deterministic",
            case.name
        );
    }
}

#[test]
fn oracle_detection_produces_a_typed_answer_for_every_oracle() {
    // Green on any machine: what is asserted is that detection *decides*, not
    // which way it decides. The failure this guards against is detection
    // panicking, hanging, or returning something that cannot be acted on.
    let mut required_but_absent = Vec::new();

    for engine in VerilogEngine::ORACLES {
        let availability = oracle::detect(engine);
        let diagnostic = availability.diagnostic();
        println!("{diagnostic}\n");

        match &availability {
            OracleAvailability::Available(tools) => {
                assert_eq!(tools.engine, engine);
                assert!(
                    !tools.programs.is_empty(),
                    "{engine} reported available with no programs"
                );
                for program in &tools.programs {
                    assert!(
                        !program.version.trim().is_empty(),
                        "{engine} program '{}' reported an empty version",
                        program.program
                    );
                }
            }
            OracleAvailability::Missing { missing, .. } => {
                assert!(
                    !missing.is_empty(),
                    "{engine} is missing nothing in particular"
                );
                // A skip has to be actionable or it trains people to ignore it.
                assert!(diagnostic.contains("To install:"), "{diagnostic}");
                assert!(
                    diagnostic.contains(oracle::ORACLES_REQUIRED_ENV),
                    "{diagnostic}"
                );
                required_but_absent.push(engine);
            }
            OracleAvailability::Unusable { reason, .. } => {
                panic!("{engine} is installed but unusable: {reason}");
            }
            OracleAvailability::NotAnOracle(_) => {
                panic!("{engine} must be treated as an oracle");
            }
        }
    }

    // Detection is meaningless for the system under test, and says so rather
    // than guessing.
    assert!(matches!(
        oracle::detect(VerilogEngine::Rspice),
        OracleAvailability::NotAnOracle(VerilogEngine::Rspice)
    ));

    if oracles_required() && !required_but_absent.is_empty() {
        panic!(
            "{}=1 but these oracles are absent: {}",
            oracle::ORACLES_REQUIRED_ENV,
            required_but_absent
                .iter()
                .map(|engine| engine.to_string())
                .collect::<Vec<_>>()
                .join(", ")
        );
    }
}

#[test]
fn icarus_and_verilator_agree_on_the_two_state_corpus() {
    // The check the whole harness exists to make. Two simulators written by
    // different people, sharing no code, asked the same question about the
    // same source, must give the same answer. Until that holds, a
    // disagreement between RSpice and either one would be uninterpretable.
    let icarus = oracle::detect(VerilogEngine::Icarus);
    let verilator = oracle::detect(VerilogEngine::Verilator);

    let (Some(icarus_tools), Some(verilator_tools)) = (icarus.tools(), verilator.tools()) else {
        let reason = format!(
            "Skipping oracle-vs-oracle agreement.\n{}\n{}",
            icarus.diagnostic(),
            verilator.diagnostic()
        );
        assert!(
            !oracles_required(),
            "{}=1 but the oracles are not both available.\n{reason}",
            oracle::ORACLES_REQUIRED_ENV
        );
        println!("{reason}");
        return;
    };

    let corpus = corpus();
    let workspace = tempfile::Builder::new()
        .prefix("rspice-verilog-agreement-")
        .tempdir()
        .expect("scratch directory");

    let mut compared = 0usize;
    for case in &corpus.cases {
        if !(case.admits(VerilogEngine::Icarus) && case.admits(VerilogEngine::Verilator)) {
            continue;
        }
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
        compared += 1;
    }

    assert!(
        compared > 0,
        "no case was cross-checked; an empty agreement is not agreement"
    );
    println!("Icarus and Verilator agree on {compared} case(s).");
    println!("  Icarus:    {}", identity(icarus_tools));
    println!("  Verilator: {}", identity(verilator_tools));
}

#[test]
fn each_available_oracle_produces_one_observation_per_vector() {
    // Agreement between two wrong runs is still agreement. This pins the shape
    // of each trace against the stimulus that produced it, independently of
    // the other oracle, so a case that silently stopped early cannot pass by
    // stopping early in both.
    let corpus = corpus();
    let workspace = tempfile::Builder::new()
        .prefix("rspice-verilog-shape-")
        .tempdir()
        .expect("scratch directory");

    let mut checked = 0usize;
    for engine in VerilogEngine::ORACLES {
        let availability = oracle::detect(engine);
        let Some(tools) = availability.tools() else {
            println!("{}", availability.diagnostic());
            continue;
        };
        for case in corpus.cases_for(engine) {
            let outcome = rspice_conformance::suites::verilog::run::run_case(
                case,
                engine,
                Some(tools),
                workspace.path(),
                DEFAULT_TIMEOUT_MS,
            )
            .unwrap_or_else(|err| panic!("case '{}' on {engine}: {err}", case.name));

            assert_eq!(
                outcome.trace.rows.len(),
                case.stimulus.vectors.len(),
                "case '{}' on {engine}: {} observation(s) for {} vector(s)",
                case.name,
                outcome.trace.rows.len(),
                case.stimulus.vectors.len()
            );
            let expected_ports = case
                .stimulus
                .observed_outputs()
                .iter()
                .map(|port| port.name.clone())
                .collect::<BTreeSet<_>>();
            for row in &outcome.trace.rows {
                let observed = row
                    .values
                    .iter()
                    .map(|(name, _)| name.clone())
                    .collect::<BTreeSet<_>>();
                assert_eq!(
                    observed, expected_ports,
                    "case '{}' on {engine}, step {}: trace ports do not match the stimulus",
                    case.name, row.step
                );
            }
            checked += 1;
        }
    }

    if checked == 0 {
        assert!(
            !oracles_required(),
            "{}=1 but no oracle was available",
            oracle::ORACLES_REQUIRED_ENV
        );
        println!("No oracle available; trace-shape checking was skipped.");
    }
}

fn identity(tools: &OracleTools) -> String {
    tools.identity()
}

// ===========================================================================
// RSpice
// ===========================================================================

/// What RSpice must do with one corpus case.
enum Expected {
    /// One row per vector, each written the way the trace grammar writes it:
    /// the observed outputs in stimulus order, `name=value`, space separated.
    Trace(&'static [&'static str]),
    /// The case must be refused, with a diagnostic containing this fragment.
    ///
    /// A refusal is a legitimate answer for a construct the front end does not
    /// implement, and the fragment is what makes it an *actionable* one: the
    /// test fails if the gap moves, rather than silently accepting whatever new
    /// reason the engine invents for not running the case.
    Refused(&'static str),
}

/// Expectations for every case in the corpus, derived without running RSpice.
///
/// This is the load-bearing claim of the whole file, so it is worth being exact
/// about where each number came from. **None of these were produced by running
/// RSpice and pasting its output back.** Neither Icarus nor Verilator is
/// installed on the machine this was written on, so there was nothing to copy
/// from either.
///
/// * **c17** — evaluated by hand from the six NAND equations in `c17.v`
///   (`N10 = ~(N1 & N3)`, `N11 = ~(N3 & N6)`, `N16 = ~(N2 & N11)`,
///   `N19 = ~(N11 & N7)`, `N22 = ~(N10 & N16)`, `N23 = ~(N16 & N19)`) over the
///   32 input combinations of `c17.stim`, in the ascending binary order the
///   stimulus lists them. Two-state throughout: every input is driven 0 or 1
///   and no net is left undriven, so no four-state rule is involved.
///
/// * **dff_register** — worked out from the register's own text against the
///   stimulus timing. One rising clock edge per vector at `k*10 + 5`, sampled
///   at `k*10 + 8`; `q` follows `d` on an edge when `en` is high and holds
///   otherwise; `q_delayed` is written non-blocking from the same edge and
///   therefore samples the *pre-edge* `q`, trailing it by exactly one clock.
///   The asynchronous reset fires on its own rising edge — including the
///   `x`-to-1 transition at `t = 0`, which IEEE 1364-2005 table 5-2 classifies
///   as a `posedge` — so the first two observations are the cleared register.
///
/// * **nba_ordering** — the three blocks were traced separately, which is what
///   the case is for. The non-blocking chain shifts one stage per edge, so
///   `din` needs three clocks to reach `nb2`; the blocking chain executes in
///   source order inside one edge, so `din` reaches `bl2` on the first; and the
///   swap pair alternates because both right-hand sides read the pre-edge
///   values. `bl2` leading `nb2` by two observations is the assertion.
///
/// * **gate_primitives**, **ripple_adder**, **xz_propagation** — refused. Each
///   names one construct the front end does not implement, and the fragment
///   pins which one, so a case moving from refused to running is a visible
///   change rather than a silent one.
const EXPECTED: [(&str, Expected); 6] = [
    (
        "c17",
        Expected::Trace(&[
            "N22=0 N23=0",
            "N22=0 N23=1",
            "N22=0 N23=0",
            "N22=0 N23=1",
            "N22=0 N23=0",
            "N22=0 N23=1",
            "N22=0 N23=0",
            "N22=0 N23=0",
            "N22=1 N23=1",
            "N22=1 N23=1",
            "N22=1 N23=1",
            "N22=1 N23=1",
            "N22=1 N23=1",
            "N22=1 N23=1",
            "N22=0 N23=0",
            "N22=0 N23=0",
            "N22=0 N23=0",
            "N22=0 N23=1",
            "N22=0 N23=0",
            "N22=0 N23=1",
            "N22=1 N23=0",
            "N22=1 N23=1",
            "N22=1 N23=0",
            "N22=1 N23=0",
            "N22=1 N23=1",
            "N22=1 N23=1",
            "N22=1 N23=1",
            "N22=1 N23=1",
            "N22=1 N23=1",
            "N22=1 N23=1",
            "N22=1 N23=0",
            "N22=1 N23=0",
        ]),
    ),
    (
        "dff_register",
        Expected::Trace(&[
            // Reset asserted, twice.
            "q=0000 q_delayed=0000",
            "q=0000 q_delayed=0000",
            // Reset released with the enable high: `q` follows `d`, and
            // `q_delayed` trails it by one clock.
            "q=0001 q_delayed=0000",
            "q=0010 q_delayed=0001",
            "q=0100 q_delayed=0010",
            "q=1000 q_delayed=0100",
            // Enable low: `q` holds 1000 while `d` changes underneath, and
            // `q_delayed` catches up to it.
            "q=1000 q_delayed=1000",
            "q=1000 q_delayed=1000",
            // Enable high again from the held state.
            "q=1010 q_delayed=1000",
            "q=0101 q_delayed=1010",
            // Asynchronous reset from a non-zero state.
            "q=0000 q_delayed=0000",
            "q=1100 q_delayed=0000",
        ]),
    ),
    (
        "nba_ordering",
        Expected::Trace(&[
            // Synchronous reset, held two clocks. The swap pair is seeded with
            // two different values and alternates from there.
            "nb2=0000 bl2=0000 swap_x=0101 swap_y=1010",
            "nb2=0000 bl2=0000 swap_x=0101 swap_y=1010",
            // Reset released. `bl2` shows the current `din` on the very next
            // observation; `nb2` is still shifting zeros.
            "nb2=0000 bl2=0001 swap_x=1010 swap_y=0101",
            "nb2=0000 bl2=0010 swap_x=0101 swap_y=1010",
            // Two observations later the first value reaches `nb2`.
            "nb2=0001 bl2=0100 swap_x=1010 swap_y=0101",
            "nb2=0010 bl2=1000 swap_x=0101 swap_y=1010",
            "nb2=0100 bl2=1111 swap_x=1010 swap_y=0101",
            "nb2=1000 bl2=1111 swap_x=0101 swap_y=1010",
            // `din` held, so both pipelines drain to the same steady value.
            "nb2=1111 bl2=1111 swap_x=1010 swap_y=0101",
            "nb2=1111 bl2=1111 swap_x=0101 swap_y=1010",
        ]),
    ),
    // `a ~^ b`, the binary XNOR operator of section 4.1.9, and the `^{...}`
    // reduction of section 4.1.10.
    ("gate_primitives", Expected::Refused("Expected RParen")),
    // Generate regions, section 12.4.
    ("ripple_adder", Expected::Refused("genvar")),
    // `===` and `!==`, the case-equality operators of section 4.1.8.
    ("xz_propagation", Expected::Refused("Invalid expression")),
];

/// One trace row, written the way [`EXPECTED`] writes it.
fn row_text(row: &trace::TraceRow) -> String {
    row.values
        .iter()
        .map(|(name, value)| format!("{name}={value}"))
        .collect::<Vec<_>>()
        .join(" ")
}

/// RSpice against expectations nothing in RSpice produced.
///
/// A conformance suite whose expectations came out of the thing it is checking
/// proves only that the thing is deterministic. See [`EXPECTED`] for how each
/// one of these was actually derived.
#[test]
fn rspice_matches_its_independently_derived_expectations() {
    let corpus = corpus();
    let workspace = std::env::temp_dir().join("rspice-verilog-expectations");

    assert_eq!(
        EXPECTED.len(),
        corpus.cases.len(),
        "every corpus case needs an expectation, or one could be added and never checked"
    );

    for (name, expected) in EXPECTED {
        let case = corpus
            .case(name)
            .unwrap_or_else(|| panic!("case '{name}' is in the corpus"));
        let outcome = run_case(
            case,
            VerilogEngine::Rspice,
            None,
            &workspace,
            DEFAULT_TIMEOUT_MS,
        );

        match expected {
            Expected::Trace(rows) => {
                let outcome =
                    outcome.unwrap_or_else(|err| panic!("case '{name}' must run on RSpice: {err}"));
                assert_eq!(
                    outcome.trace.rows.len(),
                    case.stimulus.vectors.len(),
                    "case '{name}': one observation per vector"
                );
                let actual: Vec<String> = outcome.trace.rows.iter().map(row_text).collect();
                assert_eq!(
                    actual,
                    rows.iter()
                        .map(|row| (*row).to_string())
                        .collect::<Vec<_>>(),
                    "case '{name}' disagrees with the expectation derived from its source"
                );
            }
            Expected::Refused(fragment) => {
                let error = outcome
                    .err()
                    .unwrap_or_else(|| panic!("case '{name}' must be refused, not run"));
                let RunError::RspiceRefused { detail, .. } = &error else {
                    panic!("case '{name}' must be refused by name, got {error:?}");
                };
                assert!(
                    detail.contains(fragment),
                    "case '{name}' must still be refused for {fragment:?}, got {detail:?}"
                );
            }
        }
    }

    // Nothing on disk: the RSpice arm compiles and runs in this process.
    assert!(!workspace.exists(), "the RSpice arm must leave no debris");
}

/// RSpice against a simulator written by somebody else.
///
/// This is what [`rspice_matches_its_independently_derived_expectations`]
/// cannot be: a check against an implementation that shares no code and no
/// author with RSpice. It is armed and skips while Icarus is absent, exactly as
/// the oracle-versus-oracle check does, so installing the binary turns it on
/// with no further change here.
#[test]
fn rspice_agrees_with_icarus_where_both_can_run_a_case() {
    let icarus = oracle::detect(VerilogEngine::Icarus);
    let Some(tools) = icarus.tools() else {
        let reason = format!("Skipping RSpice-vs-Icarus.\n{}", icarus.diagnostic());
        assert!(
            !oracles_required(),
            "{}=1 but Icarus is not available.\n{reason}",
            oracle::ORACLES_REQUIRED_ENV
        );
        println!("{reason}");
        return;
    };

    let corpus = corpus();
    let workspace = tempfile::Builder::new()
        .prefix("rspice-verilog-rspice-vs-icarus-")
        .tempdir()
        .expect("scratch directory");

    let mut compared = 0usize;
    let mut refused = Vec::new();
    for case in &corpus.cases {
        if !case.admits(VerilogEngine::Icarus) {
            continue;
        }
        match compare_engines(
            case,
            (VerilogEngine::Rspice, None),
            (VerilogEngine::Icarus, Some(tools)),
            workspace.path(),
            DEFAULT_TIMEOUT_MS,
        ) {
            Ok(divergences) => {
                assert!(
                    divergences.is_empty(),
                    "case '{}': {}",
                    case.name,
                    trace::describe("RSpice", "Icarus", &divergences)
                );
                compared += 1;
            }
            // A construct RSpice does not implement is a known gap, not a
            // disagreement. It is reported rather than swallowed, and the pins
            // in `EXPECTED` are what keep the list from growing unnoticed.
            Err(RunError::RspiceRefused { case, detail }) => refused.push((case, detail)),
            Err(err) => panic!("case '{}': {err}", case.name),
        }
    }

    println!("RSpice and Icarus agree on {compared} case(s).");
    for (case, detail) in &refused {
        println!("  RSpice refused '{case}': {detail}");
    }
    assert!(
        compared > 0,
        "no case was cross-checked; an empty agreement is not agreement"
    );
    println!("  Icarus: {}", identity(tools));
}
