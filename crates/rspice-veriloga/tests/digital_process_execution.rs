//! Executing digital processes, from Verilog-AMS source.
//!
//! Every fixture here is compiled by the real front end and executed by the
//! real interpreter. Nothing builds IR by hand: a hand-built process would test
//! the interpreter against whatever shape the test author imagined, and the
//! shapes that matter are the ones the lowering actually emits. A test that
//! passes here pins the whole pipeline — parser, analyzer, digital lowering,
//! interpreter — against a clause of IEEE 1364-2005.
//!
//! Each test names the clause it pins. Where the standard and the lowering
//! could both plausibly be read two ways, the test says which reading is being
//! frozen and why.

use rspice_veriloga::canonical_ir::digital::{
    CanonicalDigitalPlan, CfgDigitalProcess, DigitalEdge, DigitalSchedulingRegion,
    DigitalSensitivityOrigin,
};
use rspice_veriloga::canonical_ir::digital_eval::{
    DigitalDeferredUpdate, DigitalEnvironment, DigitalEvalError, DigitalProcessOutcome,
    DigitalResumeState, DigitalSuspension, DigitalWaitRequest, any_term_is_satisfied,
    apply_deferred, classify_edge, resume, start,
};
use rspice_veriloga::canonical_ir::digital_value::FourStateValue;
use rspice_veriloga::canonical_ir::ids::DigitalSignalId;
use rspice_veriloga::four_state::FourStateBit;
use rspice_veriloga::{CompilerOptions, VerilogACompiler};

// ===========================================================================
// Harness
// ===========================================================================

fn digital_module(section: &str) -> String {
    format!(
        "module dut(p, n);\n\
         \x20   inout p, n;\n\
         \x20   electrical p, n;\n\
         \x20   real gain;\n\
         {section}\n\
         \x20   analog I(p, n) <+ gain * V(p, n);\n\
         endmodule\n"
    )
}

fn parse_value(spelling: &str) -> FourStateValue {
    let bits: Vec<FourStateBit> = spelling
        .chars()
        .map(|character| match character {
            '0' => FourStateBit::Zero,
            '1' => FourStateBit::One,
            'x' => FourStateBit::Unknown,
            'z' => FourStateBit::HighImpedance,
            other => panic!("not a four-state digit: {other}"),
        })
        .collect();
    FourStateValue::from_bits_msb_first(&bits)
}

/// A signal store and a nonblocking-update queue: the smallest thing that
/// satisfies [`DigitalEnvironment`], and a stand-in for the event kernel's own.
struct Store {
    values: Vec<FourStateValue>,
    deferred: Vec<DigitalDeferredUpdate>,
}

impl DigitalEnvironment for Store {
    fn read_signal(&self, signal: DigitalSignalId) -> Option<FourStateValue> {
        self.values.get(usize::from(signal)).cloned()
    }

    fn write_signal(&mut self, signal: DigitalSignalId, value: FourStateValue) {
        self.values[usize::from(signal)] = value;
    }

    fn defer_update(&mut self, update: DigitalDeferredUpdate) {
        self.deferred.push(update);
    }
}

struct Harness {
    plan: CanonicalDigitalPlan,
    store: Store,
}

impl Harness {
    fn new(section: &str) -> Self {
        let plan = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir(&digital_module(section))
            .expect("fixture must lower to canonical IR")
            .digital;
        // IEEE 1364-2005 section 4.2.2: a `reg` that nothing has written holds
        // `x`. Starting the store anywhere else would let a test pass because
        // of an initial value the language does not promise.
        let values = plan
            .signals
            .iter()
            .map(|signal| FourStateValue::splat(signal.width, FourStateBit::Unknown))
            .collect();
        Self {
            plan,
            store: Store {
                values,
                deferred: Vec::new(),
            },
        }
    }

    fn signal(&self, name: &str) -> DigitalSignalId {
        self.plan
            .signals
            .iter()
            .find(|signal| signal.name == name)
            .unwrap_or_else(|| panic!("no signal named {name}"))
            .id
    }

    fn set(&mut self, name: &str, spelling: &str) {
        let id = self.signal(name);
        let value = parse_value(spelling);
        assert_eq!(
            value.width(),
            self.plan.signal(id).expect("declared").width,
            "fixture drives `{name}` at the wrong width"
        );
        self.store.values[usize::from(id)] = value;
    }

    fn get(&self, name: &str) -> String {
        let id = self.signal(name);
        self.store.values[usize::from(id)].spelling()
    }

    fn process(&self, index: usize) -> &CfgDigitalProcess {
        &self.plan.processes[index]
    }

    fn start(&mut self, index: usize) -> DigitalProcessOutcome {
        start(&self.plan, &self.plan.processes[index], &mut self.store)
            .expect("the process must run")
    }

    fn resume(&mut self, index: usize, state: &DigitalResumeState) -> DigitalProcessOutcome {
        resume(
            &self.plan,
            &self.plan.processes[index],
            state,
            &mut self.store,
        )
        .expect("the process must resume")
    }

    /// Run the only process to its first stop.
    fn run(&mut self) -> DigitalProcessOutcome {
        assert_eq!(
            self.plan.processes.len(),
            1,
            "fixture must have exactly one process"
        );
        self.start(0)
    }

    /// Drain the nonblocking region, in the order the updates were scheduled.
    fn flush_nonblocking(&mut self) {
        let updates = std::mem::take(&mut self.store.deferred);
        for update in &updates {
            assert_eq!(update.region, DigitalSchedulingRegion::NonBlockingAssign);
            apply_deferred(&self.plan, &mut self.store, update).expect("update must apply");
        }
    }

    fn deferred_count(&self) -> usize {
        self.store.deferred.len()
    }
}

fn expect_finished(outcome: DigitalProcessOutcome) {
    assert!(
        matches!(outcome, DigitalProcessOutcome::Finished),
        "expected the process to finish, got {outcome:?}"
    );
}

fn expect_suspended(outcome: DigitalProcessOutcome) -> DigitalSuspension {
    match outcome {
        DigitalProcessOutcome::Suspended(suspension) => suspension,
        DigitalProcessOutcome::Finished => panic!("expected the process to suspend"),
    }
}

// ===========================================================================
// Nonblocking assignment (IEEE 1364-2005 sections 9.2.2, 11.4.1)
// ===========================================================================

/// The canonical statement of what a nonblocking assignment is: both right-hand
/// sides are evaluated where they are written, and neither update is visible
/// until the region drains, so the two values cross.
///
/// This is the test that would fail if the interpreter applied a `<=` where it
/// appeared, and no amount of ordinary logic would notice the difference.
#[test]
fn nonblocking_assignments_in_one_slot_swap() {
    let mut harness = Harness::new(
        "    reg a, b;\n\
     \x20   initial begin a <= b; b <= a; end",
    );
    harness.set("a", "0");
    harness.set("b", "1");

    expect_finished(harness.run());

    // Nothing has landed yet: the process is over and both signals still hold
    // what they held before it ran.
    assert_eq!(harness.get("a"), "0");
    assert_eq!(harness.get("b"), "1");
    assert_eq!(harness.deferred_count(), 2);

    harness.flush_nonblocking();
    assert_eq!(harness.get("a"), "1");
    assert_eq!(harness.get("b"), "0");
}

/// Section 9.2.1: a blocking assignment takes effect before the next statement,
/// so the second statement reads the value the first one wrote.
#[test]
fn blocking_assignments_are_visible_to_the_next_statement() {
    let mut harness = Harness::new(
        "    reg a, b, c;\n\
     \x20   initial begin a = b; c = a; end",
    );
    harness.set("a", "0");
    harness.set("b", "1");
    harness.set("c", "0");

    expect_finished(harness.run());

    assert_eq!(harness.get("a"), "1");
    assert_eq!(harness.get("c"), "1", "`c` read the new `a`");
    assert_eq!(harness.deferred_count(), 0, "no update was deferred");
}

/// The same two statements with `<=` produce a different, observable answer:
/// `c` reads the *old* `a`. This is the pair that makes the distinction a
/// semantic one rather than a spelling.
#[test]
fn the_nonblocking_form_of_the_same_pair_reads_the_old_value() {
    let mut harness = Harness::new(
        "    reg a, b, c;\n\
     \x20   initial begin a <= b; c <= a; end",
    );
    harness.set("a", "0");
    harness.set("b", "1");
    harness.set("c", "0");

    expect_finished(harness.run());
    harness.flush_nonblocking();

    assert_eq!(harness.get("a"), "1");
    assert_eq!(
        harness.get("c"),
        "0",
        "`c` read the `a` from before the slot"
    );
}

/// Section 11.4.1 applies the update to the *left-hand side*, not to a snapshot
/// of the whole signal. Two nonblocking writes to different bits of one signal
/// in one slot therefore both survive; a deferred update that carried a whole
/// value would lose the first.
#[test]
fn two_nonblocking_bit_writes_to_one_signal_both_land() {
    let mut harness = Harness::new(
        "    reg [1:0] q;\n\
     \x20   initial begin q[0] <= 1'b1; q[1] <= 1'b1; end",
    );
    harness.set("q", "00");

    expect_finished(harness.run());
    assert_eq!(harness.get("q"), "00");

    harness.flush_nonblocking();
    assert_eq!(harness.get("q"), "11");
}

// ===========================================================================
// Assignment-context width (IEEE 1364-2005 section 5.2.1)
// ===========================================================================

/// Section 5.2.1: a narrow right-hand side is zero-extended at the assignment.
///
/// Zero-extended, specifically — the leading `1` of `10xz` does not propagate,
/// and neither would a leading `x`. That is the section 3.5.1 literal-padding
/// rule, which applies to source text and not to assignments, and confusing the
/// two is the whole reason this test names both.
#[test]
fn a_narrow_value_zero_fills_into_a_wider_register() {
    let mut harness = Harness::new(
        "    reg [3:0] narrow;\n\
     \x20   reg [7:0] wide;\n\
     \x20   initial wide = narrow;",
    );
    harness.set("narrow", "10xz");
    harness.set("wide", "11111111");

    expect_finished(harness.run());
    assert_eq!(harness.get("wide"), "000010xz");
}

/// Section 5.2.1: a wide right-hand side is truncated from the left, keeping
/// the least significant bits.
#[test]
fn a_wide_value_truncates_from_the_left_into_a_narrower_register() {
    let mut harness = Harness::new(
        "    reg [3:0] narrow;\n\
     \x20   reg [7:0] wide;\n\
     \x20   initial narrow = wide;",
    );
    harness.set("wide", "10xz0011");
    harness.set("narrow", "1111");

    expect_finished(harness.run());
    assert_eq!(harness.get("narrow"), "0011");
}

/// The resize is the *target's* width, not the signal's: assigning to one bit
/// of a register resizes the right-hand side to one bit and leaves every other
/// bit of the register alone.
#[test]
fn a_bit_select_target_resizes_to_one_bit_and_leaves_the_rest() {
    let mut harness = Harness::new(
        "    reg [3:0] q;\n\
     \x20   reg [3:0] source;\n\
     \x20   initial q[2] = source;",
    );
    harness.set("q", "0000");
    harness.set("source", "1011");

    expect_finished(harness.run());
    // The low bit of `source` is the one bit that fits, and it lands at 2.
    assert_eq!(harness.get("q"), "0100");
}

/// A part-select target takes the width the select names, and the same
/// zero-fill rule inside it.
#[test]
fn a_part_select_target_resizes_to_the_selected_width() {
    let mut harness = Harness::new(
        "    reg [7:0] q;\n\
     \x20   reg [1:0] source;\n\
     \x20   initial q[5:2] = source;",
    );
    harness.set("q", "00000000");
    harness.set("source", "11");

    expect_finished(harness.run());
    assert_eq!(harness.get("q"), "00001100");
}

// ===========================================================================
// Unknown propagation (IEEE 1364-2005 section 4.1)
// ===========================================================================

/// Every lowered operator class, against the section 4.1 tables, on one pair of
/// operands chosen so that each of `0`, `1`, `x` and `z` meets each of the
/// others.
///
/// The expected spellings are written out rather than computed from the shared
/// tables. Computing them would check that the interpreter calls the table it
/// calls, which is not in doubt; writing them out checks that it calls the
/// right one, and that the value reached the write at the right width.
#[test]
fn unknown_bits_propagate_through_every_wide_operator() {
    // l = 1100, r = 1x0z: the column pairs are 1&1, 1&x, 0&0, 0&z.
    let cases = [
        // Section 4.1.9, bitwise.
        ("l & r", "1x00"),
        // `1|x` is 1 because 1 controls OR; `0|z` is x because 0 does not.
        ("l | r", "110x"),
        ("l ^ r", "0x0x"),
        ("~r", "0x1x"),
        // Section 4.1.5: one unknown bit makes the whole result unknown, at the
        // operand width rather than one bit.
        ("l + r", "xxxx"),
        ("l - r", "xxxx"),
        ("l * r", "xxxx"),
        // Section 4.1.12: an unknown shift *count* poisons everything, while an
        // unknown bit in the shifted value merely moves.
        ("l << r", "xxxx"),
        ("r << 4'b0001", "x0z0"),
        ("r >> 4'b0001", "01x0"),
        // Section 4.1.14 and 4.2.1.
        ("{l[3:2], r[1:0]}", "110z"),
    ];
    for (expression, expected) in cases {
        let mut harness = Harness::new(&format!(
            "    reg [3:0] l, r, y;\n\
         \x20   initial y = {expression};"
        ));
        harness.set("l", "1100");
        harness.set("r", "1x0z");
        harness.set("y", "0000");
        expect_finished(harness.run());
        assert_eq!(harness.get("y"), expected, "`{expression}`");
    }
}

/// The operator classes whose result is one bit, section 4.1.6 through 4.1.8.
#[test]
fn unknown_bits_propagate_through_every_scalar_operator() {
    let cases = [
        // Section 4.1.7: `==` is unknown if *either* operand has an unknown
        // bit, even though the known bits already disagree. This is the rule
        // that makes `==` useless for detecting `x`.
        ("l == r", "x"),
        ("l != r", "x"),
        // Section 4.1.6, relational.
        ("l < r", "x"),
        ("l >= r", "x"),
        // Section 4.1.8: a logical operator works on truth values, and both `l`
        // and `r` have a `1` in them, so both are true whatever else they hold.
        // An unknown bit does not make a value ambiguous when a `1` settles it.
        ("l && r", "1"),
        ("l || r", "1"),
        ("!l", "0"),
        // `u4` is the ambiguous one: unknown bits and no `1` to settle them, so
        // its truth value is `x`. The controlling values still dominate — `x &&
        // 0` is 0 because 0 controls AND — but `x || 0` stays unknown.
        ("z4 && u4", "0"),
        ("z4 || u4", "x"),
        ("u4 && l", "x"),
        ("!u4", "x"),
        ("!z4", "1"),
    ];
    for (expression, expected) in cases {
        let mut harness = Harness::new(&format!(
            "    reg [3:0] l, r, z4, u4;\n\
         \x20   reg y;\n\
         \x20   initial y = {expression};"
        ));
        harness.set("l", "1100");
        harness.set("r", "1x0z");
        harness.set("z4", "0000");
        harness.set("u4", "00x0");
        harness.set("y", "0");
        expect_finished(harness.run());
        assert_eq!(harness.get("y"), expected, "`{expression}`");
    }
}

/// Section 4.1.13: a conditional *expression* with an ambiguous condition
/// evaluates both arms and merges them, keeping the bits they agree on.
///
/// Distinct from the conditional *statement*, which takes the else branch — the
/// standard treats the two differently and the next test pins the other half.
#[test]
fn a_conditional_expression_merges_its_arms_when_the_condition_is_unknown() {
    let mut harness = Harness::new(
        "    reg c;\n\
     \x20   reg [3:0] a, b, y;\n\
     \x20   initial y = c ? a : b;",
    );
    harness.set("a", "1100");
    harness.set("b", "1010");
    harness.set("y", "0000");

    harness.set("c", "1");
    expect_finished(harness.run());
    assert_eq!(harness.get("y"), "1100");

    harness.set("c", "0");
    expect_finished(harness.run());
    assert_eq!(harness.get("y"), "1010");

    // Neither arm is chosen: bit 3 is 1 in both and bit 0 is 0 in both, so
    // those survive; the two middle bits disagree and become `x`.
    harness.set("c", "x");
    expect_finished(harness.run());
    assert_eq!(harness.get("y"), "1xx0");
}

// ===========================================================================
// Conditional statements (IEEE 1364-2005 section 9.4)
// ===========================================================================

/// Section 9.4: the first statement runs only if the condition is true, where
/// true means a nonzero *known* value. A condition that is `x` or `z` is not
/// true, so the `else` runs — the same path a plain zero takes.
///
/// This is the reading being frozen. The lowering emits a plain
/// `CfgTerminator::Branch` and says nothing about ambiguous conditions, so the
/// choice is the interpreter's to make and this is where it is written down.
#[test]
fn an_ambiguous_condition_takes_the_else_branch() {
    for (condition, expected) in [("1", "1"), ("0", "0"), ("x", "0"), ("z", "0")] {
        let mut harness = Harness::new(
            "    reg c, y;\n\
         \x20   initial if (c) y = 1'b1; else y = 1'b0;",
        );
        harness.set("c", condition);
        harness.set("y", "x");
        expect_finished(harness.run());
        assert_eq!(harness.get("y"), expected, "condition `{condition}`");
    }
}

/// A wide condition is reduced by its truth value, section 4.1.8: any `1` makes
/// it true, all `0` makes it false, and an unknown bit with no `1` to settle it
/// leaves it ambiguous — which section 9.4 then sends to the `else`.
#[test]
fn a_wide_condition_is_reduced_by_its_truth_value() {
    for (condition, expected) in [
        ("0010", "1"),
        ("0000", "0"),
        ("00x0", "0"),
        ("00z0", "0"),
        // A `1` settles the question even beside an unknown bit.
        ("10x0", "1"),
    ] {
        let mut harness = Harness::new(
            "    reg [3:0] c;\n\
         \x20   reg y;\n\
         \x20   initial if (c) y = 1'b1; else y = 1'b0;",
        );
        harness.set("c", condition);
        harness.set("y", "x");
        expect_finished(harness.run());
        assert_eq!(harness.get("y"), expected, "condition `{condition}`");
    }
}

/// A `case` selects by equality, so section 4.1.7's rule reaches it: a selector
/// with an unknown bit matches no item and falls to the default, because every
/// comparison it makes is `x` and section 9.4 sends every one of those to the
/// else.
///
/// The chain is what the lowering emits — `case` becomes a branch chain — so
/// this also pins that the chain terminates where the author's `default` is.
#[test]
fn a_case_selector_with_an_unknown_bit_falls_to_the_default() {
    for (selector, expected) in [("00", "1"), ("01", "0"), ("10", "x"), ("0x", "x")] {
        let mut harness = Harness::new(
            "    reg [1:0] sel;\n\
         \x20   reg q;\n\
         \x20   always @* case (sel) 2'b00: q = 1'b1; 2'b01: q = 1'b0; \
             default: q = 1'bx; endcase",
        );
        harness.set("sel", selector);
        harness.set("q", "0");

        // The process opens with `@*`, so it suspends before its body; the body
        // runs on the resumption the sensitivity would have caused.
        let suspension = expect_suspended(harness.run());
        let state = suspension.resume_state().clone();
        expect_suspended(harness.resume(0, &state));

        assert_eq!(harness.get("q"), expected, "selector `{selector}`");
    }
}

/// `casez` and `casex` were refused by the lowering and stay refused. The
/// interpreter has no node for a wildcard comparison, so a fixture that started
/// compiling would reach it as an exact `case` and quietly match on `x`.
#[test]
fn wildcard_case_statements_are_still_refused() {
    for keyword in ["casez", "casex"] {
        let source = digital_module(&format!(
            "    reg [1:0] sel;\n\
         \x20   reg q;\n\
         \x20   always @* {keyword} (sel) 2'b1?: q = 1'b1; default: q = 1'b0; endcase"
        ));
        let result =
            VerilogACompiler::new(CompilerOptions::default()).compile_canonical_ir(&source);
        assert!(result.is_err(), "`{keyword}` must still refuse to lower");
    }
}

// ===========================================================================
// Suspension and resumption
// ===========================================================================

/// Contract item 2: a signal is not an SSA value. Two reads of one signal on
/// either side of a suspension are two nodes, and if the world moved while the
/// process was suspended they return different values.
///
/// The test moves the world by hand, which is exactly what the kernel will do.
#[test]
fn two_reads_across_a_suspension_see_different_values() {
    let mut harness = Harness::new(
        "    reg d, a, b;\n\
     \x20   initial begin a = d; #5 b = d; end",
    );
    harness.set("d", "0");
    harness.set("a", "x");
    harness.set("b", "x");

    let suspension = expect_suspended(harness.run());
    assert_eq!(
        harness.get("a"),
        "0",
        "the first read happened before the wait"
    );
    assert_eq!(harness.get("b"), "x", "the second statement has not run");

    harness.set("d", "1");
    let state = suspension.resume_state().clone();
    expect_finished(harness.resume(0, &state));

    assert_eq!(harness.get("a"), "0");
    assert_eq!(harness.get("b"), "1", "the second read saw the new value");
}

/// A `#delay` reports the number of time units it wants and nothing else; the
/// interpreter has no clock and does not pretend to.
///
/// The resumption continues *mid-sequence*: the statements after the delay run
/// and the ones before it do not run again.
#[test]
fn a_delay_reports_its_operand_and_resumes_mid_sequence() {
    let mut harness = Harness::new(
        "    reg first, second, third;\n\
     \x20   initial begin first = 1'b1; #5 second = 1'b1; third = 1'b1; end",
    );
    harness.set("first", "0");
    harness.set("second", "0");
    harness.set("third", "0");

    let suspension = expect_suspended(harness.run());
    assert_eq!(*suspension.wait(), DigitalWaitRequest::Delay(5));
    assert_eq!(harness.get("first"), "1");
    assert_eq!(harness.get("second"), "0");
    assert_eq!(harness.get("third"), "0");

    let state = suspension.resume_state().clone();
    expect_finished(harness.resume(0, &state));

    assert_eq!(harness.get("first"), "1", "the prefix did not run twice");
    assert_eq!(harness.get("second"), "1");
    assert_eq!(harness.get("third"), "1");
}

/// Section 9.9.1: an `initial` process runs once. Its function returns, so the
/// interpreter reports [`DigitalProcessOutcome::Finished`] and there is nothing
/// to resume.
#[test]
fn an_initial_process_finishes() {
    let mut harness = Harness::new(
        "    reg q;\n\
     \x20   initial q = 1'b0;",
    );
    harness.set("q", "x");
    expect_finished(harness.run());
    assert_eq!(harness.get("q"), "0");
}

/// Section 9.9.2: an `always` process restarts. The lowering spells the restart
/// as a back edge, so the interpreter never sees a `Return` — every pass ends
/// at the same `Wait`, forever, and the loop is bounded only by this driver.
#[test]
fn an_always_process_suspends_forever() {
    let mut harness = Harness::new(
        "    reg clk, q, d;\n\
     \x20   always @(posedge clk) q <= d;",
    );
    harness.set("clk", "0");
    harness.set("d", "1");
    harness.set("q", "0");

    let suspension = expect_suspended(harness.run());
    let mut state = suspension.resume_state().clone();
    let first_block = state.block();

    for pass in 0..8 {
        let suspension = expect_suspended(harness.resume(0, &state));
        assert_eq!(
            suspension.resume_state().block(),
            first_block,
            "pass {pass} came back to the same suspension point"
        );
        state = suspension.resume_state().clone();
        harness.flush_nonblocking();
        assert_eq!(harness.get("q"), "1");
    }
}

/// The wait a process reports carries the terms the author wrote, edges and
/// all, so the kernel can match a change against them without re-reading the
/// IR.
#[test]
fn an_event_wait_reports_the_terms_it_was_compiled_with() {
    let mut harness = Harness::new(
        "    reg clk, rst, q;\n\
     \x20   always @(posedge clk or negedge rst) q <= 1'b1;",
    );
    let clk = harness.signal("clk");
    let rst = harness.signal("rst");

    let suspension = expect_suspended(harness.run());
    let DigitalWaitRequest::Event(terms) = suspension.wait() else {
        panic!("an event control must report an event wait");
    };
    assert_eq!(terms.len(), 2);
    assert_eq!(terms[0].signal, clk);
    assert_eq!(terms[0].edge, Some(DigitalEdge::Posedge));
    assert_eq!(terms[1].signal, rst);
    assert_eq!(terms[1].edge, Some(DigitalEdge::Negedge));

    // And the classification the kernel will apply to them. A rising clock
    // resumes the process; a rising reset does not, because the term asked for
    // the other direction.
    let zero = parse_value("0");
    let one = parse_value("1");
    let unknown = parse_value("x");
    assert!(any_term_is_satisfied(terms, clk, &zero, &one));
    assert!(!any_term_is_satisfied(terms, rst, &zero, &one));
    assert!(any_term_is_satisfied(terms, rst, &one, &zero));
    // Section 5 table 5-2: a transition into `x` from either rail is an edge.
    assert!(any_term_is_satisfied(terms, clk, &zero, &unknown));
    assert!(any_term_is_satisfied(terms, rst, &one, &unknown));
    assert_eq!(
        classify_edge(FourStateBit::Zero, FourStateBit::Unknown),
        Some(DigitalEdge::Posedge)
    );
    assert_eq!(
        classify_edge(FourStateBit::One, FourStateBit::Unknown),
        Some(DigitalEdge::Negedge)
    );
}

/// Section 9.7.5: `@*` derives its terms from the read set of the statement it
/// guards. A change to a signal in that set resumes the process; a change to a
/// signal outside it does not, however close by it is declared.
#[test]
fn an_implicit_sensitivity_list_resumes_only_on_its_derived_set() {
    let mut harness = Harness::new(
        "    reg a, b, c, outside;\n\
     \x20   reg y;\n\
     \x20   always @* y = (a & b) | c;",
    );
    assert_eq!(
        harness
            .process(0)
            .static_sensitivity
            .as_ref()
            .expect("an opening `@*` has a static list")
            .origin,
        DigitalSensitivityOrigin::Implicit
    );

    harness.set("a", "1");
    harness.set("b", "1");
    harness.set("c", "0");
    harness.set("outside", "0");
    harness.set("y", "0");

    let suspension = expect_suspended(harness.run());
    let DigitalWaitRequest::Event(terms) = suspension.wait() else {
        panic!("`@*` must report an event wait");
    };

    let zero = parse_value("0");
    let one = parse_value("1");
    for name in ["a", "b", "c"] {
        let signal = harness.signal(name);
        assert!(
            any_term_is_satisfied(terms, signal, &zero, &one),
            "`{name}` is in the derived read set"
        );
    }
    // Neither the signal the process writes nor an unrelated declaration is a
    // trigger: `y` is an lvalue and `outside` appears nowhere in the statement.
    for name in ["y", "outside"] {
        let signal = harness.signal(name);
        assert!(
            !any_term_is_satisfied(terms, signal, &zero, &one),
            "`{name}` is not in the derived read set"
        );
    }

    // The terms are level-sensitive, so a rewrite of the same value is not an
    // event even though a write happened.
    let signal = harness.signal("a");
    assert!(!any_term_is_satisfied(terms, signal, &one, &one));

    // And resuming actually runs the body.
    let state = suspension.resume_state().clone();
    expect_suspended(harness.resume(0, &state));
    assert_eq!(harness.get("y"), "1");
}

// ===========================================================================
// Resume-state validation
// ===========================================================================

/// A resume state is checked against the function it is handed to. Resuming one
/// process with another's state is refused rather than performed on whatever
/// block happens to share the number.
#[test]
fn a_resume_state_from_another_process_is_refused() {
    let mut harness = Harness::new(
        "    reg clk, rst, q;\n\
     \x20   always @(posedge clk) q <= 1'b1;\n\
     \x20   always @(negedge rst) q <= 1'b0;",
    );
    harness.set("clk", "0");
    harness.set("rst", "1");
    harness.set("q", "0");

    let suspension = expect_suspended(harness.start(0));
    let state = suspension.resume_state().clone();
    assert_eq!(state.process(), harness.process(0).id);

    let refusal = resume(
        &harness.plan,
        &harness.plan.processes[1],
        &state,
        &mut harness.store,
    )
    .expect_err("a state from process 0 must not resume process 1");
    assert!(
        matches!(refusal, DigitalEvalError::ResumeProcessMismatch { .. }),
        "got {refusal:?}"
    );
    assert_eq!(
        harness.get("q"),
        "0",
        "the refused resumption changed nothing"
    );
}
