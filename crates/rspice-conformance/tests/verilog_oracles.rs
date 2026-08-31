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
const REQUIRED_CASES: [(&str, &str); 13] = [
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
    ("case_forms", "case, casez and casex, and an implicit sensitivity list"),
    ("loop_forms", "the for, while and repeat loops"),
    (
        "delay_forms",
        "intra-assignment and statement delays inside a process",
    ),
    (
        "lvalue_forms",
        "part-select, concatenation and bit-select assignment targets",
    ),
    (
        "gate_forms",
        "unnamed, multi-instance, n-input and multi-output gate spellings",
    ),
    ("bus_forms", "two continuous drivers resolving on one net"),
    ("edge_forms", "negedge sensitivity and a register before its first write"),
];

/// Cases whose answer depends on x or z, and which Verilator therefore cannot
/// arbitrate.
///
/// Named rather than counted, and checked in both directions below: a case
/// that silently gained Verilator would make the suite report a disagreement
/// that is Verilator being two-state, and a two-state case that silently lost
/// it would drop out of the oracle-versus-oracle check without saying so.
const FOUR_STATE_CASES: [&str; 3] = ["xz_propagation", "bus_forms", "edge_forms"];

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

    // Every case runs on Icarus; only the four-state ones are held back from
    // Verilator, and exactly those. Stated as a per-case equality rather than
    // a count, so a case moving into or out of the four-state set has to be
    // said out loud in `FOUR_STATE_CASES`.
    for case in &corpus.cases {
        assert!(
            case.admits(VerilogEngine::Icarus),
            "case '{}' names no four-state oracle",
            case.name
        );
        let four_state = FOUR_STATE_CASES.contains(&case.name.as_str());
        assert_eq!(
            case.admits(VerilogEngine::Verilator),
            !four_state,
            "case '{}': Verilator is a two-state simulator, so it may arbitrate \
             exactly the cases whose answers never contain x or z",
            case.name
        );
    }

    // And the oracle-vs-oracle check must have something left to compare, or
    // it degenerates into one simulator talking to itself.
    let both = corpus.cases.len() - FOUR_STATE_CASES.len();
    assert!(
        both >= 8,
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

/// What RSpice must do with one corpus case: one trace row per vector, each
/// written the way the trace grammar writes it — the observed outputs in
/// stimulus order, `name=value`, space separated.
///
/// Every case in the corpus has one. There is deliberately no way to spell
/// "this case is refused" here any more: the corpus exists to say what this
/// engine does with the language, and a corpus entry it cannot execute is a
/// gap, not an expectation. The refusal *path* is still exercised — see
/// `suites::verilog::run`'s own tests, which build a design out of a construct
/// the front end does not implement and check that it refuses by name rather
/// than producing an empty trace that would agree with anything.
type Expected = &'static [&'static str];

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
/// * **gate_primitives** — the eight-row truth table of `(a, b, c)`, evaluated
///   by hand from the module's own eighteen expressions. The gate-instantiated
///   and operator-spelled halves are *not* two derivations: each row asserts
///   that `g_and` and `o_and` are the same bit, so a front end with two paths
///   fails the row rather than agreeing with itself. `wide` is
///   `{a, b, c, 1'b1} ^ 4'b1010`, worked bit by bit — `(a^1)(b^0)(c^1)(1^0)`,
///   most significant first. `mux` is `c ? a : b`. `reduce` is `^{a, b, c}`,
///   the parity of the three inputs, which IEEE 1364-2005 section 4.1.10
///   defines as the XOR table folded across the concatenation's bits.
///
/// * **ripple_adder** — eight-bit `a + b + cin`, computed in decimal per
///   vector and written back in binary, with `cout` the ninth bit. Each row's
///   comment names the carry behaviour the stimulus chose it for. Nothing about
///   the *structure* is assumed: the design builds the sum out of eight
///   `full_adder` instances through a generate loop, and the assertion is that
///   the structure computes the arithmetic — which is why the expectation is
///   arithmetic rather than a stage-by-stage trace.
///
/// * **xz_propagation** — read off the four-state rules the module's own header
///   names, one at a time. `and_x` is `a & 1'bx`, which section 4.1.9's AND
///   table makes `0` when `a` is `0` (the controlling value) and `x` when `a`
///   is `1`; `or_x` is the dual. `bus` has two conditional drivers, and its
///   value is section 7.9's resolution of their contributions: `z` when neither
///   drives, the driven value when exactly one does, and `x` on contention.
///   `eq_case` is `1` in every row and `eq_log` is `x` in every row, because
///   section 4.1.8 makes `===` total over all four states and `==` unknown as
///   soon as either operand has one — the standing check that the two operators
///   have not been unified. `sel` walks the `casez` arms: section 9.5.1 makes a
///   `?` in a case item a don't-care, so `4'b1???` matches any selector whose
///   top bit is `1`, `4'b01??` the next, `4'b001?` the next, and `4'b0001` and
///   `4'b0000` reach the default.
///
/// * **case_forms** — the three case statements evaluated by hand over the
///   eight selectors of `case_forms.stim`, with `mixed = a ^ b` worked out
///   first. Each arm's label was matched against the selector's binary
///   spelling: `case` exactly, `casez` with `?` matching anything, and `casex`
///   likewise (the two agree here because no selector bit is x or z, which is
///   what makes the case two-state clean). `hit` is set by one arm only.
///
/// * **loop_forms** — three closed forms, one per loop. `ones` is the
///   population count of `d`; `shifted` is `(d << 3) & 0xff`, the `while`
///   loop's three left shifts; `doubled` is `(d * 4) & 0xff`, the two
///   self-additions of `repeat (2)`. The reset vectors are zero throughout.
///
/// * **delay_forms** — from the timing, not from the text. Vector `k` applies
///   its inputs at `10k`, the edge is at `10k + 5`, and the sample at
///   `10k + 8`. `prompt` is written at the edge, so it shows `d[k]`. `lagged`
///   captures `d[k]` at the edge and updates at `10k + 11`, which is after
///   vector `k`'s sample and before vector `k + 1`'s, so it shows `d[k - 1]`.
///   `held` suspends its process to `10k + 9`, reads `d` there — still
///   `d[k]`, since the next vector is not applied until `10(k + 1)` — and
///   updates immediately, again landing between the two samples. Both
///   therefore trail `prompt` by one observation, and both are zero for the
///   two reset vectors and for the first vector after them, whose predecessor
///   was a reset.
///
/// * **lvalue_forms** — `hi` and `lo` are the two halves of `d`, `swapped` is
///   those halves exchanged, and `bit0` is `d[0]`. Worked out from the
///   assignment targets: the two part selects write `swapped` from the
///   opposite half of `d` each, and the concatenation target splits `d` most
///   significant part first.
///
/// * **gate_forms** — the four truth tables over all sixteen input
///   combinations: `wide` is `a & b & c & d`, `pair0` is `~(a & b)`, `pair1`
///   is `~(c & d)`, both `fanned` outputs are `a` and both `inverted` outputs
///   are `~b`.
///
/// * **bus_forms** — IEEE 1364-2005 table 5-1, applied per bit. One enable
///   gives that driver's operand; neither gives z; both give the operand where
///   the two agree and x where they differ, which is why the fifth vector
///   reads `1xx0`.
///
/// * **edge_forms** — the rising edge at `10k + 5` writes `rise` from `d[k]`,
///   and the falling edge at `10k` writes `fall` from whatever `rise` held
///   then, which is `d[k - 1]`. Before any falling edge has run with a written
///   `rise`, `fall` is the x that section 4.2 gives an unwritten `reg`.
const EXPECTED: [(&str, Expected); 13] = [
    (
        "c17",
        &[
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
        ],
    ),
    (
        "dff_register",
        &[
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
        ],
    ),
    (
        "nba_ordering",
        &[
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
        ],
    ),
    (
        "gate_primitives",
        &[
            "g_and=0 g_or=0 g_nand=1 g_nor=1 g_xor=0 g_xnor=1 g_buf=0 g_not=1 o_and=0 o_or=0 \
             o_nand=1 o_nor=1 o_xor=0 o_xnor=1 o_buf=0 o_not=1 wide=1011 mux=0 reduce=0",
            "g_and=0 g_or=0 g_nand=1 g_nor=1 g_xor=0 g_xnor=1 g_buf=0 g_not=1 o_and=0 o_or=0 \
             o_nand=1 o_nor=1 o_xor=0 o_xnor=1 o_buf=0 o_not=1 wide=1001 mux=0 reduce=1",
            "g_and=0 g_or=1 g_nand=1 g_nor=0 g_xor=1 g_xnor=0 g_buf=0 g_not=1 o_and=0 o_or=1 \
             o_nand=1 o_nor=0 o_xor=1 o_xnor=0 o_buf=0 o_not=1 wide=1111 mux=1 reduce=1",
            "g_and=0 g_or=1 g_nand=1 g_nor=0 g_xor=1 g_xnor=0 g_buf=0 g_not=1 o_and=0 o_or=1 \
             o_nand=1 o_nor=0 o_xor=1 o_xnor=0 o_buf=0 o_not=1 wide=1101 mux=0 reduce=0",
            "g_and=0 g_or=1 g_nand=1 g_nor=0 g_xor=1 g_xnor=0 g_buf=1 g_not=0 o_and=0 o_or=1 \
             o_nand=1 o_nor=0 o_xor=1 o_xnor=0 o_buf=1 o_not=0 wide=0011 mux=0 reduce=1",
            "g_and=0 g_or=1 g_nand=1 g_nor=0 g_xor=1 g_xnor=0 g_buf=1 g_not=0 o_and=0 o_or=1 \
             o_nand=1 o_nor=0 o_xor=1 o_xnor=0 o_buf=1 o_not=0 wide=0001 mux=1 reduce=0",
            "g_and=1 g_or=1 g_nand=0 g_nor=0 g_xor=0 g_xnor=1 g_buf=1 g_not=0 o_and=1 o_or=1 \
             o_nand=0 o_nor=0 o_xor=0 o_xnor=1 o_buf=1 o_not=0 wide=0111 mux=1 reduce=0",
            "g_and=1 g_or=1 g_nand=0 g_nor=0 g_xor=0 g_xnor=1 g_buf=1 g_not=0 o_and=1 o_or=1 \
             o_nand=0 o_nor=0 o_xor=0 o_xnor=1 o_buf=1 o_not=0 wide=0101 mux=1 reduce=1",
        ],
    ),
    (
        "ripple_adder",
        &[
            // 0 + 0 + 0
            "sum=00000000 cout=0",
            // 255 + 1 = 256: the carry ripples all eight stages and falls out.
            "sum=00000000 cout=1",
            // 255 + 255 + 1 = 511, which is 0x1FF.
            "sum=11111111 cout=1",
            // 0x55 + 0xAA = 0xFF, with no stage generating a carry.
            "sum=11111111 cout=0",
            // 0xAA + 0x55 + 1 = 0x100.
            "sum=00000000 cout=1",
            // 0x0F + 1 = 0x10: the carry ripples four stages and stops.
            "sum=00010000 cout=0",
            // 0x80 + 0x80 = 0x100, generated only in the top stage.
            "sum=00000000 cout=1",
            // 0x7F + 1 = 0x80, the signed-overflow boundary.
            "sum=10000000 cout=0",
            // 0 + 0 + 1: `cin` alone, and no stage carries.
            "sum=00000001 cout=0",
            // 255 + 0 + 1 = 0x100: `cin` injected under a full row of ones.
            "sum=00000000 cout=1",
        ],
    ),
    (
        "xz_propagation",
        &[
            "and_x=0 or_x=x bus=z eq_case=1 eq_log=x sel=00",
            "and_x=x or_x=1 bus=z eq_case=1 eq_log=x sel=00",
            "and_x=0 or_x=x bus=z eq_case=1 eq_log=x sel=11",
            "and_x=0 or_x=x bus=1 eq_case=1 eq_log=x sel=10",
            "and_x=0 or_x=x bus=0 eq_case=1 eq_log=x sel=01",
            "and_x=0 or_x=x bus=x eq_case=1 eq_log=x sel=00",
            "and_x=x or_x=1 bus=z eq_case=1 eq_log=x sel=11",
            "and_x=x or_x=1 bus=z eq_case=1 eq_log=x sel=10",
            "and_x=x or_x=1 bus=z eq_case=1 eq_log=x sel=01",
            "and_x=x or_x=1 bus=z eq_case=1 eq_log=x sel=00",
            "and_x=x or_x=1 bus=z eq_case=1 eq_log=x sel=00",
        ],
    ),
    (
        "case_forms",
        &[
            "exact=10101010 wildz=11111111 wildx=11001100 hit=0",
            "exact=10101010 wildz=11111111 wildx=11001100 hit=0",
            "exact=11001100 wildz=01100110 wildx=01011010 hit=0",
            "exact=01100110 wildz=01100110 wildx=10101010 hit=1",
            "exact=00000000 wildz=00001111 wildx=00001111 hit=0",
            "exact=00000000 wildz=11110000 wildx=01011010 hit=0",
            "exact=00000000 wildz=00000000 wildx=00000000 hit=0",
            "exact=00000000 wildz=11111111 wildx=01011010 hit=0",
        ],
    ),
    (
        "loop_forms",
        &[
            "ones=0000 shifted=00000000 doubled=00000000",
            "ones=0101 shifted=10011000 doubled=11001100",
            "ones=0001 shifted=00001000 doubled=00000100",
            "ones=1000 shifted=11111000 doubled=11111100",
            "ones=0000 shifted=00000000 doubled=00000000",
            "ones=0100 shifted=10101000 doubled=01010100",
            "ones=0001 shifted=00000000 doubled=00000000",
            "ones=0100 shifted=01111000 doubled=00111100",
        ],
    ),
    (
        "delay_forms",
        &[
            "prompt=0000 lagged=0000 held=0000",
            "prompt=0000 lagged=0000 held=0000",
            "prompt=0011 lagged=0000 held=0000",
            "prompt=0101 lagged=0011 held=0011",
            "prompt=1001 lagged=0101 held=0101",
            "prompt=1110 lagged=1001 held=1001",
            "prompt=0110 lagged=1110 held=1110",
        ],
    ),
    (
        "lvalue_forms",
        &[
            "swapped=00000000 hi=0000 lo=0000 bit0=0",
            "swapped=01011010 hi=1010 lo=0101 bit0=1",
            "swapped=00001111 hi=1111 lo=0000 bit0=0",
            "swapped=00000000 hi=0000 lo=0000 bit0=0",
            "swapped=01110001 hi=0001 lo=0111 bit0=1",
            "swapped=10111100 hi=1100 lo=1011 bit0=1",
        ],
    ),
    (
        "gate_forms",
        &[
            "wide=0 pair0=1 pair1=1 fanned0=0 fanned1=0 inverted0=1 inverted1=1",
            "wide=0 pair0=1 pair1=1 fanned0=0 fanned1=0 inverted0=1 inverted1=1",
            "wide=0 pair0=1 pair1=1 fanned0=0 fanned1=0 inverted0=1 inverted1=1",
            "wide=0 pair0=1 pair1=0 fanned0=0 fanned1=0 inverted0=1 inverted1=1",
            "wide=0 pair0=1 pair1=1 fanned0=0 fanned1=0 inverted0=0 inverted1=0",
            "wide=0 pair0=1 pair1=1 fanned0=0 fanned1=0 inverted0=0 inverted1=0",
            "wide=0 pair0=1 pair1=1 fanned0=0 fanned1=0 inverted0=0 inverted1=0",
            "wide=0 pair0=1 pair1=0 fanned0=0 fanned1=0 inverted0=0 inverted1=0",
            "wide=0 pair0=1 pair1=1 fanned0=1 fanned1=1 inverted0=1 inverted1=1",
            "wide=0 pair0=1 pair1=1 fanned0=1 fanned1=1 inverted0=1 inverted1=1",
            "wide=0 pair0=1 pair1=1 fanned0=1 fanned1=1 inverted0=1 inverted1=1",
            "wide=0 pair0=1 pair1=0 fanned0=1 fanned1=1 inverted0=1 inverted1=1",
            "wide=0 pair0=0 pair1=1 fanned0=1 fanned1=1 inverted0=0 inverted1=0",
            "wide=0 pair0=0 pair1=1 fanned0=1 fanned1=1 inverted0=0 inverted1=0",
            "wide=0 pair0=0 pair1=1 fanned0=1 fanned1=1 inverted0=0 inverted1=0",
            "wide=1 pair0=0 pair1=0 fanned0=1 fanned1=1 inverted0=0 inverted1=0",
        ],
    ),
    (
        "bus_forms",
        &[
            "bus=1010 seen=1010",
            "bus=0101 seen=0101",
            "bus=zzzz seen=zzzz",
            "bus=xxxx seen=xxxx",
            "bus=1xx0 seen=1xx0",
            "bus=1111 seen=1111",
            "bus=0000 seen=0000",
        ],
    ),
    (
        "edge_forms",
        &[
            "rise=0011 fall=xxxx",
            "rise=0101 fall=0011",
            "rise=1001 fall=0101",
            "rise=1110 fall=1001",
            "rise=0110 fall=1110",
        ],
    ),
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
        )
        .unwrap_or_else(|err| panic!("case '{name}' must run on RSpice: {err}"));

        assert_eq!(
            outcome.trace.rows.len(),
            case.stimulus.vectors.len(),
            "case '{name}': one observation per vector"
        );
        let actual: Vec<String> = outcome.trace.rows.iter().map(row_text).collect();
        assert_eq!(
            actual,
            expected
                .iter()
                .map(|row| (*row).to_string())
                .collect::<Vec<_>>(),
            "case '{name}' disagrees with the expectation derived from its source"
        );
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
