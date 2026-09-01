//! Verilog-AMS semantics, one test per clause.
//!
//! Every test names the clause it checks and asserts an answer derived from
//! that clause's requirement in
//! [`ams_semantics::CASES`](rspice_conformance::suites::verilog::ams_semantics::CASES),
//! whose row for the test carries the derivation in prose beside it. The two
//! are kept together by construction: each test looks its own row up by name
//! and fails if it is missing, and [`the_table_and_the_suite_agree`] fails if a
//! row has no test.
//!
//! See the suite module's documentation for what the three verdicts mean and
//! why a refusal can be a conformance case.
#![cfg(feature = "verilog-digital")]

use rspice_conformance::suites::verilog::ams_semantics::{
    self as ams, CASES, ClauseCase, REQUIRED_CLAUSES, Verdict,
};

/// The delay `DELAYED_REACTION`'s process waits after a boundary change, in
/// nanosecond ticks.
const REACTION_DELAY_NS: f64 = 3.0;

/// The supply an auto-bridged boundary derives its thresholds from when the
/// deck declares none.
const DEFAULT_SUPPLY: f64 = 3.3;

fn expect_refusal(outcome: Result<impl Sized, String>, what: &str) -> String {
    match outcome {
        Ok(_) => panic!("{what} must be refused rather than answered"),
        Err(error) => error,
    }
}

fn assert_names(error: &str, case: &ClauseCase, fragments: &[&str]) {
    let lowered = error.to_lowercase();
    for fragment in fragments {
        assert!(
            lowered.contains(&fragment.to_lowercase()),
            "the {} refusal must name `{fragment}`, so the reader can act on it: {error}",
            case.clause
        );
    }
}

// ===========================================================================
// IEEE 1364-2005 section 5.4.1 — the assignment context sets the width
// ===========================================================================

#[test]
fn an_assignment_context_sets_the_expression_width() {
    let case = ams::case("an_assignment_context_sets_the_expression_width");
    assert_eq!(case.verdict, Verdict::Conforms);

    let report = ams::run_digital(
        ams::WIDTH_CONTEXT,
        &ams::vector_stimulus(
            "width_context",
            vec![ams::port("a", 4), ams::port("b", 4)],
            vec![ams::port("wide", 8), ams::port("narrow", 2)],
            vec![vec!["1111", "0001"], vec!["0011", "0010"]],
        ),
    )
    .expect("the width design runs");

    // 15 + 1 = 16. In an eight-bit context that is 8'b0001_0000; an addition
    // sized by its four-bit operands alone would carry out and give zero.
    assert_eq!(ams::observed(&report, 0, "wide"), "00010000");
    // The same sixteen, truncated to the two bits its target has.
    assert_eq!(ams::observed(&report, 0, "narrow"), "00");
    // 3 + 2 = 5, which fits four bits, so both contexts agree about the value
    // and disagree only about how much of it survives. Without this row the
    // case above could not tell a widened addition from a coincidence.
    assert_eq!(ams::observed(&report, 1, "wide"), "00000101");
    assert_eq!(ams::observed(&report, 1, "narrow"), "01");
}

// ===========================================================================
// IEEE 1364-2005 section 5.4.2 — signedness travels with the operand
// ===========================================================================

#[test]
fn signedness_is_a_property_of_the_operand_not_of_the_target() {
    let case = ams::case("signedness_is_a_property_of_the_operand_not_of_the_target");
    assert_eq!(case.verdict, Verdict::Conforms);

    let report = ams::run_digital(
        ams::SIGN_CONTEXT,
        &ams::vector_stimulus(
            "sign_context",
            vec![ams::port("a", 4)],
            vec![ams::port("unsigned_wide", 8), ams::port("signed_wide", 8)],
            vec![vec!["1000"], vec!["0011"]],
        ),
    )
    .expect("the signedness design runs");

    // 4'b1000 is eight unsigned and minus eight signed. Widened into eight
    // bits the first zero-extends and the second sign-extends, so they differ
    // in exactly the four bits the widening added.
    assert_eq!(ams::observed(&report, 0, "unsigned_wide"), "00001000");
    assert_eq!(ams::observed(&report, 0, "signed_wide"), "11111000");
    // A value whose top bit is clear is the same number either way, which is
    // what says the difference above is sign extension rather than an
    // unconditional inversion.
    assert_eq!(ams::observed(&report, 1, "unsigned_wide"), "00000011");
    assert_eq!(ams::observed(&report, 1, "signed_wide"), "00000011");
}

// ===========================================================================
// LRM 2.4 section 3.7 — an undriven real net is zero
// ===========================================================================

#[test]
fn an_undriven_real_net_is_zero() {
    let case = ams::case("an_undriven_real_net_is_zero");
    assert_eq!(case.verdict, Verdict::Conforms);

    let report = ams::run_digital(
        ams::UNDRIVEN_REAL,
        &ams::vector_stimulus(
            "undriven_real",
            vec![ams::port("sel", 1)],
            vec![ams::port("driven", 0), ams::port("undriven", 0)],
            vec![vec!["0"], vec!["1"], vec!["0"]],
        ),
    )
    .expect("the undriven-real design runs");

    for step in 0..3 {
        assert_eq!(
            ams::observed(&report, step, "undriven"),
            "0.0",
            "observation {step}: a real net nothing drives holds the section 3.7 zero"
        );
    }
    // And the driven net moves, so the zero above is a value rather than a
    // design that never ran.
    assert_eq!(ams::observed(&report, 0, "driven"), "2.5");
    assert_eq!(ams::observed(&report, 1, "driven"), "1.5");
    assert_eq!(ams::observed(&report, 2, "driven"), "2.5");
}

// ===========================================================================
// LRM 2.4 section 6.5.3 — one driver on a real net
// ===========================================================================

#[test]
fn a_real_net_with_two_drivers_is_refused() {
    let case = ams::case("a_real_net_with_two_drivers_is_refused");
    assert_eq!(case.verdict, Verdict::RefusesAsTheClauseAllows);

    let error = expect_refusal(
        ams::run_digital(
            ams::TWO_REAL_DRIVERS,
            &ams::vector_stimulus(
                "two_real_drivers",
                vec![ams::port("sel", 1)],
                vec![ams::port("out", 0)],
                vec![vec!["0"], vec!["1"]],
            ),
        ),
        "a `wreal` with two continuous drivers",
    );
    assert_names(
        &error,
        case,
        &[
            "out",
            "2 drivers",
            "6.5.3",
            "wrealsum",
            "wrealavg",
            "wrealmin",
            "wrealmax",
        ],
    );
}

#[test]
fn a_real_valued_module_port_is_not_a_discipline_boundary() {
    let case = ams::case("a_real_valued_module_port_is_not_a_discipline_boundary");
    assert_eq!(case.verdict, Verdict::BoundedByImplementation);

    let model = ams::ModelFile::new("real_port", ams::REAL_PORT_MODULE);
    let deck = format!(
        "* a real-valued module port on an X-card\n\
         vclk clk 0 pulse(0 3.3 5n 0.1n 0.1n 10n 20n)\n\
         x1 p 0 clk lvl real_port_module\n\
         rp p 0 1meg\n\
         rl lvl 0 10k\n\
         .va \"{}\" real_port_module\n\
         .tran 1n 40n\n\
         .end\n",
        model.deck_path()
    );
    let error = expect_refusal(
        ams::run_deck(&deck, 40.0e-9, 1.0e-9),
        "a `wreal` port bridged onto a circuit node",
    );
    assert_names(&error, case, &["x1", "level", "wreal"]);
}

// ===========================================================================
// LRM 2.4 section 7.3.1 — reading is allowed, contributing is not
// ===========================================================================

#[test]
fn a_process_may_read_a_continuous_value_but_not_contribute_to_one() {
    let case = ams::case("a_process_may_read_a_continuous_value_but_not_contribute_to_one");
    assert_eq!(case.verdict, Verdict::Conforms);

    // The read half. A ramp from 0 V to 2 V over 100 ns crosses the process's
    // 1 V test at 50 ns; clock edges at 20 ns and 60 ns straddle it, so the
    // process reads 0.4 V at the first and 1.2 V at the second and its output
    // changes exactly once.
    let model = ams::ModelFile::new("reads_continuous", ams::READS_CONTINUOUS);
    let deck = format!(
        "* a process reading its module's own continuous terminal\n\
         vin p 0 pwl(0 0 100n 2.0)\n\
         vclk clk 0 pulse(0 3.3 20n 0.1n 0.1n 10n 40n)\n\
         x1 p 0 clk qs reads_continuous\n\
         rq qs 0 10k\n\
         .va \"{}\" reads_continuous\n\
         .tran 0.2n 100n\n\
         .end\n",
        model.deck_path()
    );
    let result = ams::run_deck(&deck, 100.0e-9, 0.2e-9).expect("the reading design runs");
    let transitions = ams::transition_times(&result, "qs");
    assert_eq!(
        transitions.len(),
        2,
        "the opening value and one change: {transitions:?}"
    );
    assert!(
        (transitions[1] - 60.0e-9).abs() < 1.0e-9,
        "the change must land at the clock edge whose sample crossed 1 V, saw {:e}",
        transitions[1]
    );

    // The contribution half. A process is a discrete context and a
    // contribution statement is not one of the statements it may contain, so
    // this is refused rather than run with the contribution dropped.
    let contributing = ams::ModelFile::new("contributes", ams::CONTRIBUTES_FROM_A_PROCESS);
    let deck = format!(
        "* a process containing a contribution statement\n\
         vclk clk 0 pulse(0 3.3 5n 0.1n 0.1n 10n 20n)\n\
         x1 p 0 clk qs contributes_from_a_process\n\
         rp p 0 1meg\n\
         rq qs 0 10k\n\
         .va \"{}\" contributes_from_a_process\n\
         .tran 1n 40n\n\
         .end\n",
        contributing.deck_path()
    );
    let error = expect_refusal(
        ams::run_deck(&deck, 40.0e-9, 1.0e-9),
        "a contribution statement inside a process",
    );
    // Refused by the front end, at the file it could not compile. The message
    // does not cite section 7.3.1 — a contribution statement is simply not a
    // procedural statement, so the parser reaches the end of what a process may
    // contain and stops, which is the right place for this to fail even though
    // the sentence a user reads is about grammar rather than about domains.
    assert_names(&error, case, &[&contributing.deck_path(), "compile"]);
    assert!(
        !error.to_lowercase().contains("dropped") && !error.to_lowercase().contains("ignored"),
        "the contribution must not be reported as something the run continued without: {error}"
    );
}

// ===========================================================================
// LRM 2.4 section 7.3.3 — probing a continuous net
// ===========================================================================

#[test]
fn a_process_probes_the_continuous_net_at_the_edge_that_woke_it() {
    let case = ams::case("a_process_probes_the_continuous_net_at_the_edge_that_woke_it");
    assert_eq!(case.verdict, Verdict::Conforms);

    // The same design as the read half above, sampled for *which* solution the
    // probe read rather than for whether it read one. The A/D bridge switches
    // at half the deck's 3.3 V supply, and the clock's 0.1 ns rise starting at
    // 60 ns crosses that half way up, so the edge is at 60.05 ns. A probe that
    // read the previous accepted timepoint's solution would still be below the
    // 1 V test there and would not change until the next clock edge, a whole
    // 40 ns period later.
    let model = ams::ModelFile::new("probes_terminal", ams::READS_CONTINUOUS);
    // The clock's high level is the deck's supply, written from the same
    // constant the derivation's threshold is half of, so the two cannot drift
    // apart. The crossing instant does not depend on the level — half way up a
    // linear ramp is half way up whatever it climbs to.
    let deck = format!(
        "* which solution a probe reads\n\
         vin p 0 pwl(0 0 100n 2.0)\n\
         vclk clk 0 pulse(0 {DEFAULT_SUPPLY} 20n 0.1n 0.1n 10n 40n)\n\
         x1 p 0 clk qs reads_continuous\n\
         rq qs 0 10k\n\
         .va \"{}\" reads_continuous\n\
         .tran 0.2n 100n\n\
         .end\n",
        model.deck_path()
    );
    let result = ams::run_deck(&deck, 100.0e-9, 0.2e-9).expect("the probing design runs");
    let transitions = ams::transition_times(&result, "qs");
    let change = *transitions.last().expect("the trace records the change");
    assert!(
        (change - 60.05e-9).abs() < 0.5e-9,
        "the probe must read the edge's own solution, putting the change at 60.05 ns; a \
         stale read would put it at 100 ns. Saw {change:e}"
    );
}

#[test]
fn a_probe_of_a_net_that_is_not_a_terminal_is_refused_by_name() {
    let case = ams::case("a_probe_of_a_net_that_is_not_a_terminal_is_refused_by_name");
    assert_eq!(case.verdict, Verdict::BoundedByImplementation);

    let model = ams::ModelFile::new("internal_probe", ams::PROBES_AN_INTERNAL_NET);
    let deck = format!(
        "* a process probing an internal analog net\n\
         vin p 0 pwl(0 0 40n 2)\n\
         vclk clk 0 pulse(0 3.3 5n 0.1n 0.1n 10n 20n)\n\
         x1 p 0 clk qs probes_an_internal_net\n\
         rq qs 0 10k\n\
         .va \"{}\" probes_an_internal_net\n\
         .tran 1n 40n\n\
         .end\n",
        model.deck_path()
    );
    let error = expect_refusal(
        ams::run_deck(&deck, 40.0e-9, 1.0e-9),
        "a probe of an internal analog net",
    );
    // The net, the module, and the clause the refusal falls short of. A bound
    // stated is one a user can work around.
    assert_names(
        &error,
        case,
        &["mid", "probes_an_internal_net", "7.3.3", "not a terminal"],
    );
}

// ===========================================================================
// LRM 2.4 section 7.3.6.1 — flooring an analog timepoint onto the grid
// ===========================================================================

#[test]
fn an_analog_timepoint_is_floored_onto_the_tick_grid() {
    let case = ams::case("an_analog_timepoint_is_floored_onto_the_tick_grid");
    assert_eq!(case.verdict, Verdict::Conforms);

    // A ramp from 0 V to 3.3 V over 100 ns crosses the boundary's half-supply
    // threshold at 50 ns. The stepper is held to a 0.7 ns ceiling so the
    // accepted timepoint that detects the crossing is off the nanosecond grid,
    // which is the case flooring exists for.
    let model = ams::ModelFile::new("floored_grid", ams::DELAYED_REACTION);
    let deck = format!(
        "* an off-grid crossing floored onto the tick grid\n\
         vin c 0 pwl(0 0 100n 3.3)\n\
         x1 p 0 c y delayed_reaction\n\
         rp p 0 1meg\n\
         ry y 0 10k\n\
         .va \"{}\" delayed_reaction\n\
         .tran 0.7n 100n\n\
         .end\n",
        model.deck_path()
    );
    let result = ams::run_deck(&deck, 100.0e-9, 0.7e-9).expect("the floored-grid design runs");

    // `y` changes when the process resumes, three ticks after the tick the
    // crossing was published into. So the change time minus 3 ns is that
    // tick's seconds, which must be a whole number of nanoseconds at or before
    // the crossing's own accepted timepoint and within one nanosecond of it.
    let transitions = ams::transition_times(&result, "y");
    let change = *transitions
        .last()
        .expect("the reaction reaches the boundary");
    let published = change - REACTION_DELAY_NS * 1.0e-9;
    let ticks = (published / 1.0e-9).round();
    assert!(
        (published - ticks * 1.0e-9).abs() < 1.0e-15,
        "the publication instant must be a whole tick, saw {published:e} s"
    );

    // The crossing itself: the first accepted timepoint at or after 50 ns.
    let crossing = result
        .time
        .iter()
        .copied()
        .find(|time| *time >= 50.0e-9)
        .expect("the run reaches the crossing");
    assert!(
        published <= crossing,
        "flooring must never publish past the timepoint that caused it: {published:e} s \
         against {crossing:e} s"
    );
    assert!(
        crossing - published < 1.0e-9,
        "and must be within one tick of it: {published:e} s against {crossing:e} s"
    );
    assert!(
        crossing > 50.0e-9,
        "the deck must produce an off-grid crossing, or flooring has nothing to do; saw \
         {crossing:e} s"
    );
}

// ===========================================================================
// LRM 2.4 section 7.3.6.2 — an activation is an exact analog timepoint
// ===========================================================================

#[test]
fn a_digital_activation_is_an_exact_analog_timepoint() {
    let case = ams::case("a_digital_activation_is_an_exact_analog_timepoint");
    assert_eq!(case.verdict, Verdict::Conforms);

    const TSTOP_NS: u32 = 1000;
    const ACTIVATION_NS: u32 = 5;

    let model = ams::ModelFile::new("self_clocked", ams::SELF_CLOCKED_DIVIDER);
    let deck = format!(
        "* a module scheduling its own activations for a microsecond\n\
         x1 p 0 qdiv self_clocked_divider\n\
         rp p 0 1meg\n\
         r1 qdiv out 1k\n\
         c1 out 0 10p\n\
         .va \"{}\" self_clocked_divider\n\
         .tran 1n {TSTOP_NS}n\n\
         .end\n",
        model.deck_path()
    );
    let result = ams::run_deck(&deck, f64::from(TSTOP_NS) * 1.0e-9, 1.0e-9)
        .expect("the self-clocked design runs");

    let activations = TSTOP_NS / ACTIVATION_NS;
    assert!(activations >= 200, "the run must cover many activations");
    let mut missing = Vec::new();
    for index in 1..=activations {
        // The tick's seconds, formed the way the conversion forms them: a tick
        // count times the seconds per tick.
        let expected = f64::from(index * ACTIVATION_NS) * 1.0e-9;
        if !result
            .time
            .iter()
            .any(|time| time.to_bits() == expected.to_bits())
        {
            missing.push(expected);
        }
    }
    assert!(
        missing.is_empty(),
        "{} of {activations} activations were not accepted timepoints with their own bits; \
         first few: {:?}",
        missing.len(),
        &missing[..missing.len().min(5)]
    );
}

// ===========================================================================
// LRM 2.4 section 7.7.3 — connect rules reach a mixed module's boundary
// ===========================================================================

/// Where the boundary switches, given the supply its connect rule supplies.
///
/// The ramp is 0 V to 5 V over 100 ns and the boundary switches at half the
/// supply, so the instant is `100 ns * (supply / 2) / 5 V`.
fn switching_instant(supply: f64) -> f64 {
    100.0e-9 * (supply / 2.0) / 5.0
}

fn run_ramp_toggle(rules: Option<&str>) -> f64 {
    let model = ams::ModelFile::new("ramp_toggle", ams::RAMP_TOGGLE);
    let library = rules.map(ams::connect_library);
    let include = library
        .as_ref()
        .map(|library| format!(".veriloga \"{}\"\n", library.deck_path()))
        .unwrap_or_default();
    let deck = format!(
        "* a ramp across a boundary a connect rule parameterizes\n\
         {include}\
         vclk clk 0 pwl(0 0 100n 5)\n\
         x1 p 0 clk qs ramp_toggle\n\
         rp p 0 1meg\n\
         rq qs 0 10k\n\
         .va \"{}\" ramp_toggle\n\
         .tran 1n 100n\n\
         .end\n",
        model.deck_path()
    );
    let result = ams::run_deck(&deck, 100.0e-9, 1.0e-9).expect("the ramp design runs");
    let transitions = ams::transition_times(&result, "qs");
    assert_eq!(
        transitions.len(),
        2,
        "the opening value and one crossing: {transitions:?}"
    );
    transitions[1]
}

#[test]
fn connect_rules_parameterize_a_mixed_module_boundary() {
    let case = ams::case("connect_rules_parameterize_a_mixed_module_boundary");
    assert_eq!(case.verdict, Verdict::Conforms);

    /// The stepper's ceiling, which is how late the accepted timepoint that
    /// records a crossing may be.
    const MAX_STEP: f64 = 1.0e-9;

    let check = |label: &str, observed: f64, supply: f64| {
        let expected = switching_instant(supply);
        assert!(
            observed >= expected && observed - expected <= MAX_STEP,
            "{label}: a boundary converting against {supply} V switches at {expected:e} s and \
             is recorded at the first accepted timepoint at or after it, so within one \
             {MAX_STEP:e} s step. Saw {observed:e} s"
        );
    };

    // No connect rules at all: the boundary derives its threshold from the
    // deck's supply, which defaults to 3.3 V.
    check("no connect rules", run_ramp_toggle(None), DEFAULT_SUPPLY);
    // A rule that names the boundary's connect module but overrides nothing
    // must land in the same place, or the selection itself is changing the
    // answer.
    check(
        "a rule with no overrides",
        run_ramp_toggle(Some(
            "connectrules deck;\n    connect a2d;\n    connect d2a;\nendconnectrules\n",
        )),
        DEFAULT_SUPPLY,
    );
    // And a rule supplying one volt moves it to a tenth of the ramp rather
    // than a third of it, which is the whole point of section 7.7.3.
    check(
        "a rule supplying one volt",
        run_ramp_toggle(Some(
            "connectrules deck;\n    connect a2d #(.vsup(1.0));\n    connect d2a #(.vsup(1.0));\nendconnectrules\n",
        )),
        1.0,
    );
}

// ===========================================================================
// The table and the suite
// ===========================================================================

/// Every row has a test, every required clause has a row, and no row states a
/// derivation it does not have.
///
/// The first half cannot be checked by reflection, so it is checked by
/// construction instead: every test above opens by looking its own row up with
/// [`ams::case`], which panics on a name the table does not carry. This asserts
/// the other direction — that the table has not grown a row nothing runs — by
/// counting, which is the one place a count is the right instrument, because
/// what it is protecting against is a row added without a test rather than a
/// particular row going missing.
#[test]
fn the_table_and_the_suite_agree() {
    const TESTS: usize = 11;
    assert_eq!(
        CASES.len(),
        TESTS,
        "a clause row was added or removed without its test; every row above is exercised by \
         the test of the same name"
    );

    for clause in REQUIRED_CLAUSES {
        assert!(
            CASES.iter().any(|case| case.clause == *clause),
            "clause {clause} is required and has no case"
        );
    }
    for case in CASES {
        assert!(
            REQUIRED_CLAUSES.contains(&case.clause),
            "`{}` cites clause {} which is not in the required list; add it there so the \
             coverage claim stays complete",
            case.name,
            case.clause
        );
        assert!(
            case.requirement.len() > 60 && case.derivation.len() > 120,
            "`{}` must state what the clause requires and how its answer follows; a row \
             without a derivation is a golden with a clause number on it",
            case.name
        );
    }
}

/// The connect-module signatures this suite declares still match the shipped
/// library's.
///
/// The clause-7 case has to put `connectmodule` declarations in the file its
/// deck includes, because section 7.7.1 refuses a `connect` statement naming an
/// undeclared module. Reaching the shipped ones would mean this crate depending
/// on the Verilog-AMS front end, and that edge changes `Cargo.lock` — which is
/// a digest input of the generated-built-ins bundle, so adding it would make
/// every build demand a regeneration of a ten-gigabyte corpus to carry two
/// module signatures.
///
/// So the signatures are declared here and checked against there. What is
/// checked is what the case depends on: that both modules still exist under
/// these names, and that `vsup` is still the parameter a `connect` statement
/// overrides to move the boundary's thresholds.
#[test]
fn connect_module_signatures_still_match_the_shipped_library() {
    let path = ams::workspace_root().join(ams::SHIPPED_LIBRARY_SOURCE);
    let source = std::fs::read_to_string(&path).unwrap_or_else(|error| {
        panic!(
            "the shipped connect-module library this suite copies from is missing from {}: \
             {error}",
            path.display()
        )
    });
    for declaration in ["connectmodule a2d(a, d);", "connectmodule d2a(d, a);"] {
        assert!(
            source.contains(declaration),
            "`{}` no longer declares `{declaration}`; this suite's own copy is now describing \
             something the engine does not ship",
            path.display()
        );
        assert!(
            ams::CONNECT_MODULE_SIGNATURES.contains(declaration),
            "this suite stopped declaring `{declaration}`"
        );
    }
    assert!(
        source.contains("parameter real vsup = 3.3;"),
        "`{}` no longer gives its connect modules a `vsup` parameter with the deck-supply \
         default; the clause-7 case's whole derivation is that overriding it moves the \
         boundary's threshold to half of whatever was supplied",
        path.display()
    );
}

/// The suite does not restate clause 7's discipline resolution.
///
/// The LRM's Figure 7-3 hierarchy, section 7.4.4.1's basic algorithm and Annex
/// F.2.1's table are machine-checked against the figure's own net list in
/// `rspice-veriloga`'s `connect::tests`. This asserts that those tests are
/// still where this suite says they are, so the reference does not quietly
/// become a gap: a suite that cites another suite has to fail when that one
/// moves, or the citation is a comment.
#[test]
fn discipline_resolution_is_covered_where_this_suite_says_it_is() {
    let path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("the conformance crate sits under crates/")
        .join("rspice-veriloga")
        .join("src")
        .join("connect")
        .join("tests.rs");
    let source = std::fs::read_to_string(&path).unwrap_or_else(|error| {
        panic!(
            "the discipline-resolution suite this one defers to is missing from {}: {error}",
            path.display()
        )
    });
    for marker in ["Figure 7-3", "7.4.4.1", "F.2.1"] {
        assert!(
            source.contains(marker),
            "`{}` no longer covers {marker}; either it moved, in which case update this \
             citation, or the coverage went away, in which case this suite has a gap it is \
             claiming not to have",
            path.display()
        );
    }
}
