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
    CanonicalDigitalPlan, CfgDigitalProcess, DigitalDriverId, DigitalEdge, DigitalProcessKind,
    DigitalSchedulingRegion, DigitalSensitivityOrigin,
};
use rspice_veriloga::canonical_ir::digital_eval::{
    DigitalDeferredUpdate, DigitalDrive, DigitalEnvironment, DigitalEvalError,
    DigitalProcessOutcome, DigitalRealDrive, DigitalResumeState, DigitalSuspension, DigitalUpdate,
    DigitalWaitRequest, any_term_is_satisfied, apply_deferred, classify_edge, resume, start,
};
use rspice_veriloga::canonical_ir::digital_value::FourStateValue;
use rspice_veriloga::canonical_ir::ids::DigitalSignalId;
use rspice_veriloga::four_state::FourStateBit;
use rspice_veriloga::{CompilerOptions, VerilogACompiler};
use std::collections::BTreeMap;

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

/// A signal store, a nonblocking-update queue, and one slot per driver: the
/// smallest thing that satisfies [`DigitalEnvironment`], and a stand-in for the
/// event kernel's own.
struct Store {
    values: Vec<FourStateValue>,
    /// The value of every real net, in the same signal space. A four-state
    /// signal's slot is never read; keeping one table per signal id rather than
    /// a map means a signal has exactly one place its value can be.
    reals: Vec<f64>,
    deferred: Vec<DigitalDeferredUpdate>,
    /// The latest contribution of each driver, which is what a resolver
    /// combines. Kept per driver rather than written into the net, because a
    /// net with two drivers has two contributions and storing one over the
    /// other is the bug the driver identity exists to prevent.
    driven: BTreeMap<DigitalDriverId, DigitalDrive>,
    /// The same, for a real net's drivers.
    driven_reals: BTreeMap<DigitalDriverId, DigitalRealDrive>,
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

    fn drive_signal(&mut self, drive: DigitalDrive) {
        self.driven.insert(drive.driver, drive);
    }

    fn write_real_signal(&mut self, signal: DigitalSignalId, value: f64) {
        self.reals[usize::from(signal)] = value;
    }

    fn read_real_signal(&self, signal: DigitalSignalId) -> Option<f64> {
        self.reals.get(usize::from(signal)).copied()
    }

    fn drive_real_signal(&mut self, drive: DigitalRealDrive) {
        self.driven_reals.insert(drive.driver, drive);
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
        // Verilog-AMS LRM 2.4 section 3.7: a `wreal` starts at zero, not at
        // `z`, and that is stated as the *net's* initial value rather than as
        // an absence of drivers.
        let reals = vec![0.0; plan.signals.len()];
        Self {
            plan,
            store: Store {
                values,
                reals,
                deferred: Vec::new(),
                driven: BTreeMap::new(),
                driven_reals: BTreeMap::new(),
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

    /// Settle the nets every driver contributed to.
    ///
    /// Resolution proper is the kernel's — combining two drivers of one net is
    /// a table over the whole net — so this stand-in refuses to guess: a net
    /// with one driver resolves to that driver's contribution, and a net with
    /// two is a fixture this harness will not pretend to run.
    fn resolve_drivers(&mut self) {
        let drives: Vec<DigitalDrive> = self.store.driven.values().cloned().collect();
        for drive in drives {
            let count = self.plan.drivers_of(drive.driver.signal).count();
            assert_eq!(
                count, 1,
                "multi-driver resolution belongs to the kernel; signal {:?} has {count} drivers",
                drive.driver.signal
            );
            apply_deferred(
                &self.plan,
                &mut self.store,
                &DigitalDeferredUpdate {
                    target: drive.target.clone(),
                    value: DigitalUpdate::FourState(drive.value.clone()),
                    region: DigitalSchedulingRegion::Active,
                },
            )
            .expect("a drive must apply");
        }
    }

    fn drive_count(&self) -> usize {
        self.store.driven.len()
    }

    /// Force a real net from outside, as a stimulus generator would.
    fn set_real(&mut self, name: &str, value: f64) {
        let id = self.signal(name);
        assert!(
            self.plan.signal(id).expect("declared").kind.is_real(),
            "`{name}` is not a real net"
        );
        self.store.reals[usize::from(id)] = value;
    }

    fn get_real(&self, name: &str) -> f64 {
        self.store.reals[usize::from(self.signal(name))]
    }

    /// Settle the single-driver real nets every driver contributed to.
    ///
    /// The same stand-in `resolve_drivers` is, and refusing for the same
    /// reason: Verilog-AMS LRM 2.4 section 6.5.3 gives a `wreal` one driver,
    /// and combining several is the kernel's fold rather than this harness's.
    fn resolve_real_drivers(&mut self) {
        let drives: Vec<DigitalRealDrive> = self.store.driven_reals.values().cloned().collect();
        for drive in drives {
            let count = self.plan.drivers_of(drive.driver.signal).count();
            assert_eq!(
                count, 1,
                "multi-driver resolution belongs to the kernel; signal {:?} has {count} drivers",
                drive.driver.signal
            );
            self.store.reals[usize::from(drive.driver.signal)] = drive.value;
        }
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

/// The assignment context of a concatenation target is the *whole* left-hand
/// side, so a narrow right-hand side zero-extends across it and the top
/// element gets a 0 rather than the `x` that slicing an unresized value yields.
///
/// One bit into `{carry, sum}` is the smallest case that tells the two
/// readings apart, and the reading being frozen is section 5.2.1's: the width
/// of the assignment is the sum of the target's parts.
#[test]
fn a_narrow_value_zero_fills_across_a_concatenation_target() {
    let mut harness = Harness::new(
        "    reg carry, sum, src;\n\
     \x20   initial {carry, sum} = src;",
    );
    harness.set("src", "1");
    harness.set("carry", "x");
    harness.set("sum", "x");

    expect_finished(harness.run());
    assert_eq!(harness.get("carry"), "0", "the extension is zero, not `x`");
    assert_eq!(harness.get("sum"), "1");
}

/// The same target with an unsized literal, which section 3.5.1 gives 32 bits.
/// It reaches the right answer through truncation rather than extension, which
/// is why it cannot stand in for the test above — a lowering that never
/// extends still passes this one.
#[test]
fn an_unsized_literal_into_a_concatenation_target_truncates_to_it() {
    let mut harness = Harness::new(
        "    reg carry, sum;\n\
     \x20   initial {carry, sum} = 1;",
    );
    harness.set("carry", "x");
    harness.set("sum", "x");

    expect_finished(harness.run());
    assert_eq!(harness.get("carry"), "0");
    assert_eq!(harness.get("sum"), "1");
}

/// And the other direction: a wide right-hand side is truncated from the left
/// across the concatenation, keeping the least significant bits, which are then
/// distributed over the elements from the most significant end down.
#[test]
fn a_wide_value_truncates_from_the_left_into_a_concatenation_target() {
    let mut harness = Harness::new(
        "    reg [1:0] hi, lo;\n\
     \x20   reg [7:0] src;\n\
     \x20   initial {hi, lo} = src;",
    );
    harness.set("src", "1011xz01");
    harness.set("hi", "11");
    harness.set("lo", "11");

    expect_finished(harness.run());
    // The low four bits are `xz01`; `hi` takes the upper pair of those.
    assert_eq!(harness.get("hi"), "xz");
    assert_eq!(harness.get("lo"), "01");
}

/// Extension across a concatenation is the same in the nonblocking form, which
/// resizes when the statement runs rather than when the update lands.
#[test]
fn a_concatenation_target_extends_the_same_way_nonblocking() {
    let mut harness = Harness::new(
        "    reg [1:0] hi;\n\
     \x20   reg lo, src;\n\
     \x20   initial {hi, lo} <= src;",
    );
    harness.set("src", "1");
    harness.set("hi", "11");
    harness.set("lo", "0");

    expect_finished(harness.run());
    assert_eq!(harness.get("hi"), "11", "nothing has landed yet");

    harness.flush_nonblocking();
    assert_eq!(harness.get("hi"), "00");
    assert_eq!(harness.get("lo"), "1");
}

// ===========================================================================
// Context-determined expression width (IEEE 1364-2005 section 5.4.1)
// ===========================================================================
//
// Section 5.2.1's resize, above, is the *last* step. Section 5.4.1 is the one
// before it, and the two are easy to confuse into a wrong answer: the
// assignment's left-hand side is part of the expression's context, so the
// operands of a context-determined operator are extended to the width of the
// largest expression *including the target* and the operation is performed at
// that width. Computing at the operand width and widening afterwards loses the
// bits the wider target was written to hold — which is a wrong number rather
// than a refused one, and is what every test in this section pins.

/// The headline case. `4'b1111 * 4'b1111` is 225, which needs eight bits; the
/// target has eight, so section 5.4.1 makes the multiplication eight bits wide
/// and the answer is `8'b11100001`.
///
/// Multiplying at the operand width first gives `225 mod 16 = 1`, then widens
/// that to `8'b00000001`. The two readings differ in every bit above the
/// bottom nibble, which is what makes this the clearest statement of the rule.
#[test]
fn a_product_is_computed_at_the_width_of_its_assignment_target() {
    let mut harness = Harness::new(
        "    reg [3:0] a, b;\n\
     \x20   reg [7:0] p;\n\
     \x20   initial p = a * b;",
    );
    harness.set("a", "1111");
    harness.set("b", "1111");

    expect_finished(harness.run());
    // 15 * 15 = 225 = 0xE1.
    assert_eq!(harness.get("p"), "11100001");
}

/// One bit of target beyond the operands is enough: `15 + 15 = 30` needs five,
/// and a five-bit target gives the addition five bits, so the carry survives.
#[test]
fn an_addition_keeps_the_carry_the_wider_target_has_room_for() {
    let mut harness = Harness::new(
        "    reg [3:0] a, b;\n\
     \x20   reg [4:0] p;\n\
     \x20   initial p = a + b;",
    );
    harness.set("a", "1111");
    harness.set("b", "1111");

    expect_finished(harness.run());
    // 15 + 15 = 30 = 5'b11110. Adding at four bits gives 14, then zero-extends
    // to `5'b01110` — the dropped carry.
    assert_eq!(harness.get("p"), "11110");
}

/// Table 5-22 gives `<<` a result the size of its *left* operand, and the left
/// operand is context-determined. So an eight-bit target makes `a` eight bits
/// before the shift, and a bit shifted past bit 3 is still there.
///
/// The right operand is self-determined and takes no part in this; the shift
/// count is a number of positions, not a value being combined.
#[test]
fn a_left_shift_happens_at_the_target_width_not_the_operand_width() {
    let mut harness = Harness::new(
        "    reg [3:0] a;\n\
     \x20   reg [7:0] p;\n\
     \x20   initial p = a << 5;",
    );
    harness.set("a", "0001");

    expect_finished(harness.run());
    // Bit 0 moves to bit 5. Shifting a four-bit `a` first shifts every bit out
    // and leaves `4'b0000`, which widens to zero.
    assert_eq!(harness.get("p"), "00100000");
}

/// The classic pin on the other side of the rule: section 5.4.1 makes *every*
/// operand of a concatenation self-determined, so the outer context does not
/// reach into one. `b + c` inside `{a, b + c}` is four bits wide whatever the
/// target is, and the sum wraps.
///
/// This is the test that fails if "context" is implemented as a width pushed
/// down the whole tree rather than one stopped at the operators that stop it.
#[test]
fn a_concatenation_operand_is_self_determined_and_wraps() {
    let mut harness = Harness::new(
        "    reg [3:0] a, b, c;\n\
     \x20   reg [15:0] p;\n\
     \x20   initial p = {a, b + c};",
    );
    harness.set("a", "1010");
    harness.set("b", "1111");
    harness.set("c", "0001");

    expect_finished(harness.run());
    // `b + c` is `16 mod 16 = 0` at four bits. The concatenation is eight bits
    // wide, and section 5.2.1 zero-extends it into the sixteen-bit target.
    assert_eq!(harness.get("p"), "0000000010100000");
}

/// A comparison's operands form their own context: they are sized to each
/// other and to nothing else, and the one-bit result is self-determined.
///
/// `a == b` with a four-bit `a` and an eight-bit `b` compares them at eight
/// bits — so a `b` whose high nibble is set is *not* equal to any `a` — and a
/// sixteen-bit target changes neither the comparison nor the width of its
/// answer.
#[test]
fn a_comparison_sizes_its_operands_to_each_other_only() {
    let mut harness = Harness::new(
        "    reg [3:0] a;\n\
     \x20   reg [7:0] b;\n\
     \x20   reg [15:0] p;\n\
     \x20   initial p = a == b;",
    );
    harness.set("a", "1111");
    harness.set("b", "00001111");

    expect_finished(harness.run());
    assert_eq!(harness.get("p"), "0000000000000001", "equal at eight bits");

    harness.set("b", "10001111");
    expect_finished(harness.run());
    assert_eq!(
        harness.get("p"),
        "0000000000000000",
        "the high nibble of `b` is part of the comparison"
    );
}

/// Section 5.4.1: an unsized literal is at least 32 bits, and takes the context
/// size when the context is larger.
///
/// `1 << 35` is the smallest expression that tells the two readings apart. The
/// literal is the shift's left operand, which is context-determined, so a
/// 40-bit target makes it forty bits and bit 0 survives its journey to bit 35.
/// A literal frozen at thirty-two bits shifts every bit out and leaves zero.
#[test]
fn an_unsized_literal_takes_a_context_wider_than_thirty_two_bits() {
    let mut harness = Harness::new(
        "    reg [39:0] p;\n\
     \x20   initial p = 1 << 35;",
    );

    expect_finished(harness.run());
    let mut expected = String::from("0000");
    expected.push('1');
    expected.push_str(&"0".repeat(35));
    assert_eq!(expected.len(), 40);
    assert_eq!(harness.get("p"), expected, "bit 35 is set and nothing else");
}

/// The floor is still thirty-two: a concatenation's operands are
/// self-determined, so the same literal inside one is exactly thirty-two bits
/// wide however wide the target is.
///
/// Together with the test above this pins both halves of section 5.4.1's
/// unsized-literal rule — the 32-bit minimum, and the context taking over
/// above it — and neither reading satisfies both.
#[test]
fn an_unsized_literal_in_a_concatenation_is_exactly_thirty_two_bits() {
    let mut harness = Harness::new(
        "    reg [3:0] a;\n\
     \x20   reg [35:0] p;\n\
     \x20   initial p = {a, 1};",
    );
    harness.set("a", "1010");

    expect_finished(harness.run());
    let mut expected = String::from("1010");
    expected.push_str(&"0".repeat(31));
    expected.push('1');
    assert_eq!(expected.len(), 36);
    assert_eq!(harness.get("p"), expected, "`a` sits above a 32-bit one");
}

/// Table 5-22 gives `~i` the size of `i`, and makes `i` context-determined.
/// So `~(a == b)` in an eight-bit target is a comparison producing one bit,
/// zero-extended to eight, and *then* inverted — `8'b11111110`, not the
/// `8'b00000000` that inverting one bit and widening afterwards gives.
///
/// The distinction is invisible for a wide operand and total for a narrow one,
/// which is why the operand here is a comparison rather than a register.
#[test]
fn a_bitwise_not_inverts_at_the_context_width() {
    let mut harness = Harness::new(
        "    reg [3:0] a, b;\n\
     \x20   reg [7:0] p;\n\
     \x20   initial p = ~(a == b);",
    );
    harness.set("a", "0110");
    harness.set("b", "0110");

    expect_finished(harness.run());
    assert_eq!(harness.get("p"), "11111110");
}

/// Both arms of `?:` are context-determined and the condition is not. The
/// selected arm is therefore computed at the target's width, which is the same
/// rule as the bare operator — a conditional does not become a place where the
/// context is dropped.
#[test]
fn both_arms_of_a_conditional_take_the_context_width() {
    let mut harness = Harness::new(
        "    reg [3:0] a, b;\n\
     \x20   reg sel;\n\
     \x20   reg [7:0] p;\n\
     \x20   initial p = sel ? a * b : a + b;",
    );
    harness.set("a", "1111");
    harness.set("b", "1111");
    harness.set("sel", "1");

    expect_finished(harness.run());
    assert_eq!(harness.get("p"), "11100001", "225 at eight bits");

    harness.set("sel", "0");
    expect_finished(harness.run());
    assert_eq!(harness.get("p"), "00011110", "30 at eight bits");
}

/// A continuous assignment carries the same context as a procedural one: the
/// driver's target is its left-hand side, and section 5.4.1 does not
/// distinguish the two forms.
#[test]
fn a_continuous_assignment_sizes_its_expression_to_the_driven_net() {
    let mut harness = Harness::new(
        "    reg [3:0] a, b;\n\
     \x20   wire [7:0] p;\n\
     \x20   assign p = a * b;",
    );
    harness.set("a", "1111");
    harness.set("b", "1111");

    // The driver evaluates before it waits, so the contribution is already
    // published when the process suspends on its operands.
    expect_suspended(harness.start(0));
    harness.resolve_drivers();
    assert_eq!(harness.get("p"), "11100001");
}

/// A narrower target still truncates, which is section 5.2.1's step surviving
/// underneath section 5.4.1's. The expression is sized to the *largest* of the
/// operands and the target, so a two-bit target does not shrink a four-bit
/// addition — it takes the low two bits of it.
#[test]
fn a_narrower_target_truncates_a_wider_expression() {
    let mut harness = Harness::new(
        "    reg [3:0] a, b;\n\
     \x20   reg [1:0] p;\n\
     \x20   initial p = a + b;",
    );
    harness.set("a", "1111");
    harness.set("b", "0011");

    expect_finished(harness.run());
    // 15 + 3 = 18, which is `4'b0010` at four bits; the low two bits are `10`.
    assert_eq!(harness.get("p"), "10");
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

/// Section 9.5: a `case` item is matched bit by bit *including* `x` and `z`,
/// which is an identity comparison and not `==`.
///
/// The distinction is invisible until a label carries an unknown digit: `==`
/// makes every comparison against an `x` unknown, so `2'bx0` would match
/// nothing and fall to the default. The standard matches it against a selector
/// of `x0` and no other.
#[test]
fn a_case_item_matches_unknown_digits_by_identity() {
    for (selector, expected) in [("x0", "1"), ("00", "0"), ("10", "0"), ("xx", "0")] {
        let mut harness = Harness::new(
            "    reg [1:0] sel;\n\
         \x20   reg q;\n\
         \x20   initial case (sel) 2'bx0: q = 1'b1; default: q = 1'b0; endcase",
        );
        harness.set("sel", selector);
        harness.set("q", "x");
        expect_finished(harness.run());
        assert_eq!(harness.get("q"), expected, "selector `{selector}`");
    }
}

/// Section 9.5.1: `casez` ignores the positions where either operand holds `z`
/// — `?` in a literal is `z` — and compares the rest by identity.
///
/// "Either operand" is the part that is easy to get wrong: a `z` in the
/// *selector* is a don't-care too, not a value that fails to match. Both
/// directions are covered here.
#[test]
fn casez_ignores_high_impedance_positions_in_either_operand() {
    for (selector, expected) in [
        // The label is `1?`, so bit 0 is ignored and bit 1 must be 1.
        ("10", "1"),
        ("11", "1"),
        ("1x", "1"),
        ("1z", "1"),
        ("00", "0"),
        // An `x` in the selector is *not* a don't-care for `casez`.
        ("x0", "0"),
        // A `z` in the selector is, so this matches the `1?` arm at bit 1.
        ("z1", "1"),
    ] {
        let mut harness = Harness::new(
            "    reg [1:0] sel;\n\
         \x20   reg q;\n\
         \x20   initial casez (sel) 2'b1?: q = 1'b1; default: q = 1'b0; endcase",
        );
        harness.set("sel", selector);
        harness.set("q", "x");
        expect_finished(harness.run());
        assert_eq!(harness.get("q"), expected, "casez selector `{selector}`");
    }
}

/// Section 9.5.1: `casex` ignores `x` as well, which is the whole difference
/// between the two forms. The same selectors that `casez` rejects for holding
/// an `x` are matched here.
#[test]
fn casex_ignores_unknown_positions_as_well() {
    for (selector, expected) in [("10", "1"), ("x0", "1"), ("z0", "1"), ("00", "0")] {
        let mut harness = Harness::new(
            "    reg [1:0] sel;\n\
         \x20   reg q;\n\
         \x20   initial casex (sel) 2'b1?: q = 1'b1; default: q = 1'b0; endcase",
        );
        harness.set("sel", selector);
        harness.set("q", "x");
        expect_finished(harness.run());
        assert_eq!(harness.get("q"), expected, "casex selector `{selector}`");
    }
}

/// A match test yields a bit, never an unknown one, so an arm is taken or it is
/// not. This is what makes `casez` usable for decoding a bus that holds `x`
/// where `==` is not.
#[test]
fn a_wildcard_arm_is_taken_even_when_the_selector_is_all_unknown() {
    let mut harness = Harness::new(
        "    reg [1:0] sel;\n\
     \x20   reg q;\n\
     \x20   initial casex (sel) 2'b??: q = 1'b1; default: q = 1'b0; endcase",
    );
    harness.set("sel", "xx");
    harness.set("q", "0");
    expect_finished(harness.run());
    assert_eq!(harness.get("q"), "1");
}

// ===========================================================================
// Process-local variables and loops (IEEE 1364-2005 sections 9.6, 9.8.1)
// ===========================================================================

/// A variable declared inside the process is the process's own: it merges
/// through block parameters rather than through the signal store, and nothing
/// outside the process can see it.
#[test]
fn a_process_local_carries_a_value_between_statements() {
    let mut harness = Harness::new(
        "    reg [3:0] q;\n\
     \x20   initial begin : work\n\
     \x20       integer i;\n\
     \x20       i = 4'b0011;\n\
     \x20       i = i + 1;\n\
     \x20       q = i;\n\
     \x20   end",
    );
    harness.set("q", "0000");
    expect_finished(harness.run());
    assert_eq!(harness.get("q"), "0100");
}

/// A declaration initializer runs where it is written, and a local with none
/// starts at `x` (IEEE 1364-2005 section 4.2.2).
#[test]
fn a_process_local_starts_at_its_initializer_or_at_unknown() {
    let mut harness = Harness::new(
        "    reg [3:0] initialized, bare;\n\
     \x20   initial begin : work\n\
     \x20       reg [3:0] a = 4'b1010;\n\
     \x20       reg [3:0] b;\n\
     \x20       initialized = a;\n\
     \x20       bare = b;\n\
     \x20   end",
    );
    harness.set("initialized", "0000");
    harness.set("bare", "0000");
    expect_finished(harness.run());
    assert_eq!(harness.get("initialized"), "1010");
    assert_eq!(harness.get("bare"), "xxxx");
}

/// Section 9.8.1: a name declared in a block shadows a module signal of the
/// same name for the extent of the block. The signal keeps its value, which is
/// how the test tells the two apart.
#[test]
fn a_process_local_shadows_a_module_signal_of_the_same_name() {
    let mut harness = Harness::new(
        "    reg [3:0] shared, observed;\n\
     \x20   initial begin : work\n\
     \x20       reg [3:0] shared;\n\
     \x20       shared = 4'b1111;\n\
     \x20       observed = shared;\n\
     \x20   end",
    );
    harness.set("shared", "0000");
    harness.set("observed", "0000");
    expect_finished(harness.run());
    assert_eq!(harness.get("observed"), "1111", "the local was read");
    assert_eq!(
        harness.get("shared"),
        "0000",
        "the module signal was not written"
    );
}

/// Section 9.6.2: `for` runs its initialization once, tests before each pass,
/// and updates at the end of one. The counter is an ordinary process-local, so
/// the loop needs no mechanism of its own.
///
/// The counter is used as a shift count rather than as a bit index, because a
/// select whose bounds are not constant is still refused — a write target is a
/// compile-time `DigitalWriteSelect`, and a runtime one is a different node.
#[test]
fn a_for_loop_runs_its_body_once_per_pass() {
    let mut harness = Harness::new(
        "    reg [3:0] q;\n\
     \x20   initial begin : work\n\
     \x20       integer i;\n\
     \x20       q = 4'b0000;\n\
     \x20       for (i = 0; i < 4; i = i + 1) q = q | (4'b0001 << i);\n\
     \x20   end",
    );
    harness.set("q", "0000");
    expect_finished(harness.run());
    assert_eq!(harness.get("q"), "1111");
}

/// A select whose index is a process-local is refused, by name, rather than
/// silently folded to the counter's initial value.
#[test]
fn a_select_indexed_by_a_process_local_is_refused() {
    let error = VerilogACompiler::new(CompilerOptions::default())
        .compile_canonical_ir(&digital_module(
            "    reg [3:0] q;\n\
         \x20   initial begin : work\n\
         \x20       integer i;\n\
         \x20       for (i = 0; i < 4; i = i + 1) q[i] = 1'b1;\n\
         \x20   end",
        ))
        .expect_err("a runtime select bound must be refused");
    assert!(
        error.to_string().contains("must have constant bounds"),
        "{error}"
    );
}

/// A `for` whose condition is false at the start runs its body no times.
#[test]
fn a_for_loop_with_a_false_condition_never_enters_its_body() {
    let mut harness = Harness::new(
        "    reg [3:0] q;\n\
     \x20   initial begin : work\n\
     \x20       integer i;\n\
     \x20       q = 4'b0000;\n\
     \x20       for (i = 4; i < 4; i = i + 1) q = 4'b1111;\n\
     \x20   end",
    );
    harness.set("q", "0000");
    expect_finished(harness.run());
    assert_eq!(harness.get("q"), "0000");
}

/// `while` tests before each pass, over a counter the body moves.
#[test]
fn a_while_loop_tests_before_each_pass() {
    let mut harness = Harness::new(
        "    reg [3:0] q;\n\
     \x20   initial begin : work\n\
     \x20       integer i;\n\
     \x20       q = 4'b0000;\n\
     \x20       i = 0;\n\
     \x20       while (i < 3) begin q = q + 4'b0001; i = i + 1; end\n\
     \x20   end",
    );
    harness.set("q", "0000");
    expect_finished(harness.run());
    assert_eq!(harness.get("q"), "0011");
}

/// Section 9.6.2: `repeat` evaluates its count once and runs the body that many
/// times. The count is read before the loop, so a body that changes the signal
/// it came from does not change the number of passes.
#[test]
fn a_repeat_loop_evaluates_its_count_once() {
    let mut harness = Harness::new(
        "    reg [3:0] count, q;\n\
     \x20   initial begin\n\
     \x20       q = 4'b0000;\n\
     \x20       repeat (count) begin q = q + 4'b0001; count = 4'b0000; end\n\
     \x20   end",
    );
    harness.set("count", "0011");
    harness.set("q", "0000");
    expect_finished(harness.run());
    assert_eq!(harness.get("q"), "0011", "three passes, not one");
}

/// A count with an unknown bit is no number of passes at all, so the body runs
/// zero times — the truth-value reduction of the counter says so without a rule
/// of its own.
#[test]
fn a_repeat_loop_with_an_unknown_count_runs_no_passes() {
    for (count, expected) in [("0010", "0010"), ("0000", "0000"), ("00x0", "0000")] {
        let mut harness = Harness::new(
            "    reg [3:0] count, q;\n\
         \x20   initial begin\n\
         \x20       q = 4'b0000;\n\
         \x20       repeat (count) q = q + 4'b0001;\n\
         \x20   end",
        );
        harness.set("count", count);
        harness.set("q", "0000");
        expect_finished(harness.run());
        assert_eq!(harness.get("q"), expected, "count `{count}`");
    }
}

// ===========================================================================
// State across a suspension
// ===========================================================================

/// The resume-argument round trip, on the shape that needs it: a loop counter
/// live across a `#delay` inside the loop body.
///
/// The interpreter starts every resumption with an empty value table, so the
/// counter can only survive as a resume argument bound to a block parameter. A
/// lowering that left it in the value table produces a process that reads a
/// value nothing defines on its second pass, which is why the assertion is on
/// what lands in `q` on each of the four passes rather than only at the end.
#[test]
fn a_loop_counter_survives_a_suspension_inside_the_loop() {
    let mut harness = Harness::new(
        "    reg [3:0] q;\n\
     \x20   initial begin : work\n\
     \x20       integer i;\n\
     \x20       for (i = 0; i < 4; i = i + 1) begin #1 q <= i; end\n\
     \x20   end",
    );
    harness.set("q", "0000");

    let mut outcome = harness.run();
    for pass in 0..4 {
        let suspension = expect_suspended(outcome);
        assert_eq!(*suspension.wait(), DigitalWaitRequest::Delay(1));
        assert!(
            !suspension.resume_state().arguments().is_empty(),
            "pass {pass} must carry the counter across the suspension"
        );
        let state = suspension.resume_state().clone();
        outcome = harness.resume(0, &state);
        harness.flush_nonblocking();
        assert_eq!(
            harness.get("q"),
            format!("{:04b}", pass),
            "pass {pass} wrote its own counter value"
        );
    }
    expect_finished(outcome);
}

/// IEEE 1364-2005 section 9.2.2: an intra-assignment timing control evaluates
/// the right-hand side *before* suspending and writes it after.
///
/// So the value has to cross the suspension too. It is not a variable and has
/// no name, and the interpreter's value table does not survive — it travels as
/// a resume argument like everything else that lives across a `Wait`.
#[test]
fn an_intra_assignment_delay_writes_the_value_read_before_it() {
    let mut harness = Harness::new(
        "    reg d, q;\n\
     \x20   initial q <= #5 d;",
    );
    harness.set("d", "1");
    harness.set("q", "0");

    let suspension = expect_suspended(harness.run());
    assert_eq!(*suspension.wait(), DigitalWaitRequest::Delay(5));
    assert_eq!(harness.get("q"), "0", "nothing is written before the delay");

    // The world moves while the process sleeps. The assignment must still
    // write the `d` it read when it ran.
    harness.set("d", "0");
    let state = suspension.resume_state().clone();
    expect_finished(harness.resume(0, &state));
    harness.flush_nonblocking();
    assert_eq!(harness.get("q"), "1", "the value read before the delay");
}

/// A process-local declared outside a suspension and read after it keeps what
/// it held, which is the same mechanism seen from the other side.
#[test]
fn a_process_local_survives_a_suspension() {
    let mut harness = Harness::new(
        "    reg [3:0] q, source;\n\
     \x20   initial begin : work\n\
     \x20       reg [3:0] saved;\n\
     \x20       saved = source;\n\
     \x20       #5 q = saved;\n\
     \x20   end",
    );
    harness.set("source", "1010");
    harness.set("q", "0000");

    let suspension = expect_suspended(harness.run());
    harness.set("source", "0101");
    let state = suspension.resume_state().clone();
    expect_finished(harness.resume(0, &state));
    assert_eq!(
        harness.get("q"),
        "1010",
        "the local held the value from before the delay"
    );
}

// ===========================================================================
// Continuous assignments (IEEE 1364-2005 section 6.1)
// ===========================================================================

/// A continuous assignment is a driver, so it evaluates *before* it waits: it
/// is active from the start of the simulation rather than from the first change
/// of an operand.
///
/// It also suspends afterwards rather than finishing, because it has to
/// re-evaluate when an operand moves.
#[test]
fn a_continuous_assignment_drives_before_it_waits() {
    let mut harness = Harness::new(
        "    wire a, b;\n\
     \x20   wire y;\n\
     \x20   assign y = a & b;",
    );
    harness.set("a", "1");
    harness.set("b", "1");
    harness.set("y", "x");

    let suspension = expect_suspended(harness.run());
    assert_eq!(harness.drive_count(), 1, "the driver ran before suspending");
    harness.resolve_drivers();
    assert_eq!(harness.get("y"), "1");

    // The operands move; the driver wakes and publishes the new value.
    harness.set("b", "0");
    let state = suspension.resume_state().clone();
    expect_suspended(harness.resume(0, &state));
    harness.resolve_drivers();
    assert_eq!(harness.get("y"), "0");
}

/// The sensitivity is derived from the right-hand side's read set, the rule
/// section 9.7.5 gives `@*`. The driven net is not in it — a driver that woke
/// itself would never settle.
#[test]
fn a_continuous_assignment_waits_on_its_operands() {
    let mut harness = Harness::new(
        "    wire a, b, outside;\n\
     \x20   wire y;\n\
     \x20   assign y = a | b;",
    );
    assert_eq!(
        harness.process(0).kind,
        DigitalProcessKind::ContinuousAssign
    );
    assert_eq!(
        harness
            .process(0)
            .static_sensitivity
            .as_ref()
            .expect("a driver has a static list")
            .origin,
        DigitalSensitivityOrigin::Implicit
    );

    harness.set("a", "0");
    harness.set("b", "0");
    harness.set("outside", "0");
    harness.set("y", "0");
    let suspension = expect_suspended(harness.run());
    let DigitalWaitRequest::Event(terms) = suspension.wait() else {
        panic!("a driver waits on an event");
    };
    let zero = parse_value("0");
    let one = parse_value("1");
    for name in ["a", "b"] {
        let signal = harness.signal(name);
        assert!(any_term_is_satisfied(terms, signal, &zero, &one), "{name}");
    }
    for name in ["y", "outside"] {
        let signal = harness.signal(name);
        assert!(!any_term_is_satisfied(terms, signal, &zero, &one), "{name}");
    }
}

/// A driver with no operands cannot change, so it evaluates once and returns
/// rather than waiting for an event that can never arrive.
#[test]
fn a_constant_driver_finishes_after_driving_once() {
    let mut harness = Harness::new(
        "    wire y;\n\
     \x20   assign y = 1'b1;",
    );
    harness.set("y", "x");
    expect_finished(harness.run());
    assert!(harness.process(0).static_sensitivity.is_none());
    harness.resolve_drivers();
    assert_eq!(harness.get("y"), "1");
}

/// Each element of a concatenation target is its own driver: two nets, each
/// driven by one expression. The right-hand side is resized to the total width
/// first, exactly as a procedural concatenation target is.
///
/// It is also *sized* to that total width, which is what makes this the
/// carry-out idiom rather than a spelling of it. IEEE 1364-2005 section 5.4.1
/// puts the whole left-hand side in the addition's context, so `{cout, sum}`
/// makes `a + b` three bits and the carry out of the two-bit sum lands in
/// `cout`. Adding at the operand width and zero-extending afterwards gives a
/// `cout` that is always 0 — a carry-out net that never carries, which is the
/// defect this fixture is now the closest unit statement of.
#[test]
fn a_concatenation_target_becomes_one_driver_per_element() {
    let mut harness = Harness::new(
        "    wire [1:0] a, b;\n\
     \x20   wire cout;\n\
     \x20   wire [1:0] sum;\n\
     \x20   assign {cout, sum} = a + b;",
    );
    let cout = harness.signal("cout");
    let sum = harness.signal("sum");
    assert_eq!(harness.plan.drivers.len(), 2);
    assert_eq!(harness.plan.drivers_of(cout).count(), 1);
    assert_eq!(harness.plan.drivers_of(sum).count(), 1);
    // Each net's driver is index 0 of that net: the numbering is per net, not
    // per module.
    for driver in &harness.plan.drivers {
        assert_eq!(driver.id.index, 0);
    }

    harness.set("a", "11");
    harness.set("b", "01");
    harness.set("cout", "x");
    harness.set("sum", "xx");
    expect_suspended(harness.run());
    harness.resolve_drivers();
    // `3 + 1 = 4`, computed at the three bits the target asks for: `3'b100`.
    // The concatenation then distributes it most significant part first.
    assert_eq!(harness.get("sum"), "00");
    assert_eq!(harness.get("cout"), "1");
}

/// Two drivers on one net produce two driver identities, each with its own
/// index, and the plan reports both before anything runs — which is what a
/// resolver needs in order to know it has a net to resolve at all.
#[test]
fn two_drivers_on_one_net_get_distinct_identities() {
    let plan = VerilogACompiler::new(CompilerOptions::default())
        .compile_canonical_ir(&digital_module(
            "    wire a, b;\n\
         \x20   wire y;\n\
         \x20   assign y = a;\n\
         \x20   assign y = b;",
        ))
        .expect("two drivers on a net compile")
        .digital;
    let y = plan
        .signals
        .iter()
        .find(|signal| signal.name == "y")
        .expect("declared")
        .id;
    let indices: Vec<u32> = plan.drivers_of(y).map(|driver| driver.id.index).collect();
    assert_eq!(indices, vec![0, 1], "declaration order, per net");
    // And each has its own process, so the two are separately schedulable.
    let processes: Vec<_> = plan.drivers_of(y).map(|driver| driver.process).collect();
    assert_ne!(processes[0], processes[1]);
    assert_eq!(plan.processes.len(), 2);
}

/// IEEE 1364-2005 section 6.1.2: a net declaration assignment *is* a continuous
/// assignment. It used to be dropped at the declaration, which left the net
/// with no driver at all and said nothing about it.
#[test]
fn a_net_declaration_assignment_is_a_driver() {
    let mut harness = Harness::new(
        "    wire a, b;\n\
     \x20   wire y = a ^ b;",
    );
    assert_eq!(harness.plan.drivers.len(), 1);
    harness.set("a", "1");
    harness.set("b", "0");
    harness.set("y", "x");
    expect_suspended(harness.run());
    harness.resolve_drivers();
    assert_eq!(harness.get("y"), "1");
}

// ===========================================================================
// Bitwise XNOR (IEEE 1364-2005 section 4.1.9)
// ===========================================================================

/// `~^` runs the section 4.1.9 XNOR table elementwise, and both spellings of
/// the operator are the same operator.
///
/// The expected value is read off the table: `1~^1` is `1`, `0~^0` is `1`,
/// and a `x` or `z` on either side makes the position `x` — XNOR has no
/// controlling value, so nothing settles a position with an unknown in it.
#[test]
fn xnor_runs_the_table_elementwise_in_both_spellings() {
    for spelling in ["~^", "^~"] {
        let mut harness = Harness::new(&format!(
            "    wire [3:0] a, b;\n\
         \x20   wire [3:0] y;\n\
         \x20   assign y = a {spelling} b;"
        ));
        harness.set("a", "1100");
        harness.set("b", "1x0z");
        harness.set("y", "xxxx");
        expect_suspended(harness.run());
        harness.resolve_drivers();
        assert_eq!(harness.get("y"), "1x1x", "spelled `{spelling}`");
    }
}

/// XNOR is the complement of XOR at every position, which is what makes
/// `a ~^ b` and `~(a ^ b)` the same value — the reading the lexer's maximal
/// munch has to be safe under.
#[test]
fn xnor_agrees_with_the_negation_of_xor() {
    let mut harness = Harness::new(
        "    wire [3:0] a, b;\n\
     \x20   wire [3:0] direct, composed;\n\
     \x20   assign direct = a ~^ b;\n\
     \x20   assign composed = ~(a ^ b);",
    );
    harness.set("a", "10xz");
    harness.set("b", "1x0z");
    harness.set("direct", "xxxx");
    harness.set("composed", "xxxx");
    expect_suspended(harness.start(0));
    expect_suspended(harness.start(1));
    harness.resolve_drivers();
    assert_eq!(harness.get("direct"), harness.get("composed"));
    assert_eq!(harness.get("direct"), "1xxx");
}

/// `~^` sits on XOR's tier of table 4-2, which puts it *below* `&` and *above*
/// `|`. So `a & b ~^ c` groups as `(a & b) ~^ c`, and `a ~^ b | c` as
/// `(a ~^ b) | c`.
///
/// Both operand sets are chosen so the two candidate groupings disagree, which
/// is the only thing that makes a precedence test worth running:
///
/// * `0 & 0 ~^ 0` — correct `(0 & 0) ~^ 0` is `0 ~^ 0` = 1; the misgrouping
///   `0 & (0 ~^ 0)` is `0 & 1` = 0.
/// * `0 ~^ 0 | 0` — correct `(0 ~^ 0) | 0` is `1 | 0` = 1; the misgrouping
///   `0 ~^ (0 | 0)` is `0 ~^ 0` = 1 as well, so that one is separated with
///   `c = 1`: correct is `1 | 1` = 1 and the misgrouping is `0 ~^ 1` = 0.
#[test]
fn xnor_binds_below_bitwise_and_and_above_bitwise_or() {
    let mut harness = Harness::new(
        "    wire a, b, c;\n\
     \x20   wire tighter, looser;\n\
     \x20   assign tighter = a & b ~^ c;\n\
     \x20   assign looser  = a ~^ b | c;",
    );
    harness.set("a", "0");
    harness.set("b", "0");
    harness.set("c", "1");
    harness.set("tighter", "x");
    harness.set("looser", "x");
    expect_suspended(harness.start(0));
    expect_suspended(harness.start(1));
    harness.resolve_drivers();
    // `(0 & 0) ~^ 1` = `0 ~^ 1` = 0; misgrouped `0 & (0 ~^ 1)` = `0 & 0` = 0.
    // Equal here, so `c` is flipped below for the `&` half.
    assert_eq!(harness.get("looser"), "1", "(0 ~^ 0) | 1");

    harness.set("c", "0");
    harness.set("tighter", "x");
    expect_suspended(harness.start(0));
    harness.resolve_drivers();
    assert_eq!(harness.get("tighter"), "1", "(0 & 0) ~^ 0");
}

// ===========================================================================
// Reduction operators (IEEE 1364-2005 section 4.1.10)
// ===========================================================================

/// Every reduction operator against the value section 4.1.10 gives it.
///
/// Each expectation is the section's own definition applied by hand: the
/// bitwise operator of section 4.1.9 folded across the operand's bits, with the
/// `nand`/`nor`/`xnor` forms inverting the single-bit result at the end.
///
/// The `x` rows are the ones worth reading. A reduction is *not* poisoned by an
/// unknown bit in general: `&2'b0x` is `0` because `0` is AND's controlling
/// value and `|2'b1x` is `1` because `1` is OR's, while `^2'b0x` is `x` because
/// XOR has no controlling value at all. An implementation that poisoned the
/// result whenever any operand bit was unknown would get the first two wrong,
/// and one that ignored unknown bits would get the third wrong.
#[test]
fn reduction_operators_fold_the_bitwise_tables() {
    let cases = [
        ("&", "1111", "1"),
        ("&", "1101", "0"),
        ("&", "0x", "0"),
        ("&", "1x", "x"),
        ("&", "1z", "x"),
        ("~&", "1111", "0"),
        ("~&", "1101", "1"),
        ("~&", "0x", "1"),
        ("~&", "1x", "x"),
        ("|", "0000", "0"),
        ("|", "0010", "1"),
        ("|", "1x", "1"),
        ("|", "0x", "x"),
        ("~|", "0000", "1"),
        ("~|", "0010", "0"),
        ("~|", "1x", "0"),
        ("~|", "0x", "x"),
        // Parity: an even number of ones is 0, an odd number is 1.
        ("^", "1010", "0"),
        ("^", "1110", "1"),
        ("^", "0x", "x"),
        ("^", "1x", "x"),
        ("~^", "1010", "1"),
        ("~^", "1110", "0"),
        ("~^", "0x", "x"),
        // `^~` is the same operator as `~^`.
        ("^~", "1010", "1"),
    ];
    for (operator, operand, expected) in cases {
        let width = operand.len();
        let mut harness = Harness::new(&format!(
            "    wire [{}:0] a;\n\
         \x20   wire y;\n\
         \x20   assign y = {operator}a;",
            width - 1
        ));
        harness.set("a", operand);
        harness.set("y", "x");
        expect_suspended(harness.run());
        harness.resolve_drivers();
        assert_eq!(harness.get("y"), expected, "{operator}{width}'b{operand}");
    }
}

/// A reduction over a concatenation, which is the form that cannot be
/// desugared before the operand's width is known: `{a, b, c}` names no signal
/// to bit-select out of, so the fold has to happen where the concatenation is
/// already a value.
#[test]
fn a_reduction_folds_a_concatenation() {
    let mut harness = Harness::new(
        "    wire a, b, c;\n\
     \x20   wire y;\n\
     \x20   assign y = ^{a, b, c};",
    );
    for (a, b, c, expected) in [
        ("0", "0", "0", "0"),
        ("0", "0", "1", "1"),
        ("0", "1", "1", "0"),
        ("1", "1", "1", "1"),
        ("1", "1", "0", "0"),
    ] {
        harness.set("a", a);
        harness.set("b", b);
        harness.set("c", c);
        harness.set("y", "x");
        expect_suspended(harness.start(0));
        harness.resolve_drivers();
        assert_eq!(harness.get("y"), expected, "^{{{a},{b},{c}}}");
    }
}

/// A one-bit operand reduces to itself: a fold with nothing to fold against.
#[test]
fn a_one_bit_reduction_is_the_bit_itself() {
    for (operator, input, expected) in [
        ("&", "1", "1"),
        ("|", "0", "0"),
        ("^", "x", "x"),
        ("~&", "1", "0"),
        ("~^", "0", "1"),
    ] {
        let mut harness = Harness::new(&format!(
            "    wire a;\n\
         \x20   wire y;\n\
         \x20   assign y = {operator}a;"
        ));
        harness.set("a", input);
        harness.set("y", "x");
        expect_suspended(harness.run());
        harness.resolve_drivers();
        assert_eq!(harness.get("y"), expected, "{operator}{input}");
    }
}

/// A reduction's operand is in the driver's sensitivity list.
///
/// The read set of a discrete-domain expression form is collected through one
/// generic walk, and the walk's catch-all is silent: a form it did not descend
/// into would contribute no reads, the driver would get an empty sensitivity
/// list, and it would evaluate once at time zero and then never again. That is
/// a stuck output rather than a refusal, so it is pinned here.
#[test]
fn a_reduction_operand_reaches_the_sensitivity_list() {
    let mut harness = Harness::new(
        "    wire [1:0] a;\n\
     \x20   wire y;\n\
     \x20   assign y = |a;",
    );
    harness.set("a", "00");
    harness.set("y", "x");
    let suspension = expect_suspended(harness.run());
    let DigitalWaitRequest::Event(terms) = suspension.wait() else {
        panic!("a driver waits on an event");
    };
    let signal = harness.signal("a");
    assert!(any_term_is_satisfied(
        terms,
        signal,
        &parse_value("00"),
        &parse_value("01")
    ));
}

// ===========================================================================
// Case equality (IEEE 1364-2005 section 4.1.8)
// ===========================================================================

/// `===` compares `x` and `z` as ordinary values and always answers with a
/// definite bit; `==` answers `x` as soon as either operand has one. Confusing
/// the two is how an unknown leaks into control flow, so the divergence is
/// pinned on the same operands.
#[test]
fn case_equality_is_defined_where_logical_equality_is_not() {
    let mut harness = Harness::new(
        "    wire [3:0] a, b;\n\
     \x20   wire strict, loose, differs;\n\
     \x20   assign strict  = (a === b);\n\
     \x20   assign loose   = (a ==  b);\n\
     \x20   assign differs = (a !== b);",
    );

    // Identical, unknown bits included: `===` says 1, `==` says x.
    harness.set("a", "10xz");
    harness.set("b", "10xz");
    for index in 0..3 {
        expect_suspended(harness.start(index));
    }
    harness.resolve_drivers();
    assert_eq!(harness.get("strict"), "1");
    assert_eq!(harness.get("loose"), "x");
    assert_eq!(harness.get("differs"), "0");

    // Differing only in a state `==` cannot see: `x` against `z`.
    harness.set("b", "10zz");
    for index in 0..3 {
        expect_suspended(harness.start(index));
    }
    harness.resolve_drivers();
    assert_eq!(
        harness.get("strict"),
        "0",
        "`x` and `z` are not the same bit"
    );
    assert_eq!(harness.get("loose"), "x");
    assert_eq!(harness.get("differs"), "1");

    // Two-state operands: the two operators agree.
    harness.set("a", "1010");
    harness.set("b", "1010");
    for index in 0..3 {
        expect_suspended(harness.start(index));
    }
    harness.resolve_drivers();
    assert_eq!(harness.get("strict"), "1");
    assert_eq!(harness.get("loose"), "1");
    assert_eq!(harness.get("differs"), "0");
}

/// Unequal widths are compared with the shorter one zero-filled, per section
/// 4.1.8 — the same extension section 9.5 gives a case item.
#[test]
fn case_equality_zero_fills_the_narrower_operand() {
    let mut harness = Harness::new(
        "    wire [3:0] wide;\n\
     \x20   wire [1:0] narrow;\n\
     \x20   wire y;\n\
     \x20   assign y = (wide === narrow);",
    );
    harness.set("wide", "0011");
    harness.set("narrow", "11");
    harness.set("y", "x");
    expect_suspended(harness.run());
    harness.resolve_drivers();
    assert_eq!(harness.get("y"), "1");

    harness.set("wide", "1011");
    expect_suspended(harness.start(0));
    harness.resolve_drivers();
    assert_eq!(harness.get("y"), "0", "the filled bits are compared too");
}

/// `!==` is the complement of `===`, and can be one safely because `===` never
/// yields `x` for the negation to invert into something the standard does not
/// define.
#[test]
fn case_inequality_is_the_exact_complement() {
    let mut harness = Harness::new(
        "    wire [1:0] a, b;\n\
     \x20   wire same, different;\n\
     \x20   assign same      = (a === b);\n\
     \x20   assign different = (a !== b);",
    );
    for (left, right) in [("0x", "0x"), ("0x", "00"), ("zz", "xx"), ("10", "10")] {
        harness.set("a", left);
        harness.set("b", right);
        harness.set("same", "x");
        harness.set("different", "x");
        expect_suspended(harness.start(0));
        expect_suspended(harness.start(1));
        harness.resolve_drivers();
        let same = harness.get("same");
        let different = harness.get("different");
        assert!(same == "0" || same == "1", "`===` never yields `{same}`");
        assert_ne!(same, different, "`{left}` vs `{right}`");
    }
}

// ===========================================================================
// Real nets (Verilog-AMS LRM 2.4 section 3.7)
// ===========================================================================

/// A continuous assignment drives a real net with a real expression, and the
/// value arrives as a real rather than as bits.
///
/// Section 3.7's own example is `assign wrstim = stim;`, a real driven onto a
/// `wreal` by an ordinary continuous assignment.
#[test]
fn a_continuous_assignment_drives_a_real_net() {
    let mut harness = Harness::new(
        "    wreal vin, vout;\n\
     \x20   assign vout = vin * 0.5 + 1.0;",
    );
    harness.set_real("vin", 3.0);
    expect_suspended(harness.run());
    harness.resolve_real_drivers();
    assert_eq!(harness.get_real("vout"), 2.5);

    harness.set_real("vin", -1.0);
    expect_suspended(harness.run());
    harness.resolve_real_drivers();
    assert_eq!(harness.get_real("vout"), 0.5);
}

/// A process reads a `wreal` into a process-local `real`, compares it, and
/// drives four-state bits from the answer — the shape of section 6.5.3's own
/// `a2d` example, without a bit of conversion between the domains anywhere.
#[test]
fn a_process_reads_a_real_net_into_a_real_local() {
    let mut harness = Harness::new(
        "    wreal vin;\n\
     \x20   reg [1:0] code;\n\
     \x20   always @(vin) begin : convert\n\
     \x20       real residue;\n\
     \x20       residue = vin;\n\
     \x20       if (residue > 0.5) code = 2'b11;\n\
     \x20       else if (residue > 0.0) code = 2'b01;\n\
     \x20       else code = 2'b00;\n\
     \x20   end",
    );
    // An `always @(vin)` suspends at its top before running anything, so the
    // first pass only reaches the wait; every reading after that is a
    // resumption, which is what a value change on `vin` would cause.
    let mut state = expect_suspended(harness.start(0)).into_parts().1;
    for (input, expected) in [(0.75, "11"), (0.25, "01"), (-1.0, "00"), (0.5, "01")] {
        harness.set_real("vin", input);
        state = expect_suspended(harness.resume(0, &state)).into_parts().1;
        assert_eq!(harness.get("code"), expected, "for {input}");
    }
}

/// A bare real is a branch condition, IEEE 1364-2005 section 9.4's "nonzero
/// known value" — and the test is exact, so `-0.0` is false and `1e-300` is
/// true.
#[test]
fn a_real_is_a_condition_by_being_nonzero() {
    let mut harness = Harness::new(
        "    wreal level;\n\
     \x20   reg live;\n\
     \x20   always @(level) if (level) live = 1'b1; else live = 1'b0;",
    );
    let mut state = expect_suspended(harness.start(0)).into_parts().1;
    for (input, expected) in [(0.0, "0"), (-0.0, "0"), (1e-300, "1"), (-2.5, "1")] {
        harness.set_real("level", input);
        state = expect_suspended(harness.resume(0, &state)).into_parts().1;
        assert_eq!(harness.get("live"), expected, "for {input}");
    }
}

/// A `wreal` is a net, so IEEE 1364-2005 section 6.2's rule applies to it
/// unchanged: only a continuous driver writes one.
#[test]
fn a_procedural_assignment_to_a_real_net_is_refused() {
    let error = VerilogACompiler::new(CompilerOptions::default())
        .compile_canonical_ir(&digital_module(
            "    wreal vout;\n\
         \x20   initial vout = 1.0;",
        ))
        .expect_err("a net is not procedurally assignable");
    let rendered = error.to_string();
    assert!(
        rendered.contains("`wreal`") && rendered.contains("section 6.2"),
        "expected the section 6.2 refusal, got: {rendered}"
    );
}

/// Section 6.5.3 permits one driver of a real-valued net, and the LRM defines
/// no resolution for two — so a second is refused, and the refusal says which
/// spellings do combine.
#[test]
fn a_second_driver_on_a_plain_wreal_is_refused() {
    let error = VerilogACompiler::new(CompilerOptions::default())
        .compile_canonical_ir(&digital_module(
            "    wreal a, b, bus;\n\
         \x20   assign bus = a;\n\
         \x20   assign bus = b;",
        ))
        .expect_err("section 6.5.3 permits one driver");
    let rendered = error.to_string();
    assert!(
        rendered.contains("section 6.5.3") && rendered.contains("wrealsum"),
        "expected the arity refusal, got: {rendered}"
    );
}

/// The resolved spellings admit what `wreal` refuses, and the plan records
/// which resolution was named — the fold itself is the kernel's.
#[test]
fn a_resolved_real_net_admits_several_drivers() {
    for keyword in ["wrealsum", "wrealavg", "wrealmin", "wrealmax"] {
        let plan = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir(&digital_module(&format!(
                "    wreal a, b;\n\
             \x20   {keyword} bus;\n\
             \x20   assign bus = a;\n\
             \x20   assign bus = b;"
            )))
            .unwrap_or_else(|error| panic!("`{keyword}` must lower: {error}"))
            .digital;
        let bus = plan
            .signals
            .iter()
            .find(|signal| signal.name == "bus")
            .expect("declared");
        assert_eq!(bus.width, 0, "a real net has no bits");
        assert_eq!(
            bus.kind
                .resolution()
                .expect("a real net names a resolution")
                .keyword(),
            keyword
        );
        assert_eq!(plan.drivers_of(bus.id).count(), 2);
    }
}

/// `posedge` on a real has no transition to classify, and a range on one
/// declares an array of nets nothing downstream has. Both refuse by name.
#[test]
fn the_real_net_refusals_name_themselves() {
    let cases = [
        (
            "    wreal level;\n\
         \x20   reg q;\n\
         \x20   always @(posedge level) q = 1'b1;",
            "section 9.7.2",
        ),
        (
            "    wreal [3:0] bus;\n\
         \x20   reg q;\n\
         \x20   initial q = 1'b0;",
            "bus of real nets",
        ),
        (
            "    wreal level;\n\
         \x20   reg q;\n\
         \x20   initial q = level[0];",
            "no bits to select",
        ),
        // Verilog-AMS LRM 2.4 table 4-2 makes `%` legal on real operands and
        // this wave does not implement it; the refusal says which of the two
        // kinds of missing it is.
        (
            "    wreal a, b, out;\n\
         \x20   assign out = a % b;",
            "table 4-2 but is not implemented yet",
        ),
        // `?:` *is* implemented, and its arms are still one domain each.
        (
            "    wreal a, out;\n\
         \x20   reg sel;\n\
         \x20   assign out = sel ? a : 2;",
            "a four-state operand in a real expression has no conversion",
        ),
    ];
    for (section, expected) in cases {
        let error = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir(&digital_module(section))
            .expect_err("the construct must be refused");
        let rendered = error.to_string();
        assert!(
            rendered.contains(expected),
            "expected `{expected}` to be named in: {rendered}"
        );
    }
}

// ===========================================================================
// What still refuses
// ===========================================================================

/// The refusals that remain inside a process, each naming what is missing.
#[test]
fn the_remaining_process_refusals_name_themselves() {
    let cases = [
        (
            "    reg [3:0] q;\n\
             \x20   integer i;\n\
             \x20   initial for (i = 0; i < 4; i = i + 1) q[i] = 1'b0;",
            "module-level",
        ),
        // A process-local `real` lowers now — Verilog-AMS LRM 2.4 section
        // 6.5.3's own example reads a `wreal` into one. A `string` still does
        // not, and stands in its place.
        (
            "    reg q;\n\
             \x20   initial begin : work string s; q = 1'b0; end",
            "process-local `string`",
        ),
        // What a real *cannot* do is meet a four-state value inside one
        // operator. Section 3.7 converts between the two with an explicit
        // `$realtobits`/`$bitstoreal`, and there is no implicit conversion to
        // invent for an `x`.
        (
            "    reg [3:0] q;\n\
             \x20   initial begin : work real r; r = 1.0; q = r; end",
            "a real value has no four-state form here",
        ),
        (
            "    reg q;\n\
             \x20   initial begin : work real r; r = 1.0 + q; q = 1'b0; end",
            "a four-state operand in a real expression has no conversion",
        ),
        (
            "    reg q;\n\
             \x20   initial begin : work integer i; i <= 1; q = 1'b0; end",
            "nonblocking assignment to the process-local `i`",
        ),
        (
            "    reg q;\n\
             \x20   initial begin : work reg [3:0] t; t[0] = 1'b1; q = t[0]; end",
            "select on the process-local `t`",
        ),
    ];
    for (section, expected) in cases {
        let error = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir(&digital_module(section))
            .expect_err("the construct must be refused");
        let rendered = error.to_string();
        assert!(
            rendered.contains(expected),
            "expected `{expected}` to be named in: {rendered}"
        );
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

// ===========================================================================
// Signed expressions (IEEE 1364-2005 section 5.4.2)
// ===========================================================================
//
// The IR tests pin which operand got which extension. These pin the numbers
// that come out, because the two can fail apart: an operand correctly
// sign-extended into an operator that then divides unsigned produces a wrong
// answer with a right-looking graph.
//
// Every fixture here is the same source twice, once with `signed` and once
// without, so what is being read is the *difference* the qualifier makes rather
// than an absolute value that might be right for the wrong reason.

/// Run one `initial` process and read one output.
fn signed_case(section: &str, output: &str) -> String {
    let mut harness = Harness::new(section);
    harness.run();
    harness.get(output)
}

/// Section 5.4.1's extension under section 5.4.2's rule, at the assignment.
///
/// `reg signed [3:0] a` holding `1111` is -1, and -1 in eight bits is
/// `11111111`. The same four bits in an unsigned `reg` are 15, and 15 in eight
/// bits is `00001111`. One declaration keyword, two values, and nothing else
/// in the fixture differs.
#[test]
fn a_signed_value_sign_extends_to_a_wider_target() {
    assert_eq!(
        signed_case(
            "    reg signed [3:0] a;\n\
             \x20   reg [7:0] p;\n\
             \x20   initial begin a = 4'b1111; p = a; end",
            "p",
        ),
        "11111111",
    );
    assert_eq!(
        signed_case(
            "    reg [3:0] a;\n\
             \x20   reg [7:0] p;\n\
             \x20   initial begin a = 4'b1111; p = a; end",
            "p",
        ),
        "00001111",
    );
}

/// Section 4.3.2: an `x` or `z` in the sign position extends with itself, so a
/// value whose sign is not known does not acquire a known one.
#[test]
fn an_unknown_sign_bit_extends_as_itself() {
    assert_eq!(
        signed_case(
            "    reg signed [3:0] a;\n\
             \x20   reg [7:0] p;\n\
             \x20   initial begin a = 4'bx111; p = a; end",
            "p",
        ),
        "xxxxx111",
    );
    // Unsigned, the same bits gain four known zeros: section 5.2.1's fill does
    // not consult the top bit at all.
    assert_eq!(
        signed_case(
            "    reg [3:0] a;\n\
             \x20   reg [7:0] p;\n\
             \x20   initial begin a = 4'bx111; p = a; end",
            "p",
        ),
        "0000x111",
    );
}

/// Section 4.1.6 with 5.4.2: `-1 < 0` holds between signed operands and does
/// not the moment either side is unsigned, where the same bits mean 15.
#[test]
fn a_relational_comparison_reads_its_operands_as_the_declaration_says() {
    let compare = |declarations: &str, expression: &str| {
        signed_case(
            &format!(
                "{declarations}\n\
                 \x20   reg y;\n\
                 \x20   initial begin a = 4'b1111; b = 4'b0000; y = {expression}; end",
            ),
            "y",
        )
    };
    assert_eq!(
        compare("    reg signed [3:0] a, b;", "a < b"),
        "1",
        "-1 < 0"
    );
    assert_eq!(compare("    reg [3:0] a, b;", "a < b"), "0", "15 < 0");
    // One unsigned operand is enough to make the whole comparison unsigned,
    // even though `a` is still declared signed.
    assert_eq!(
        compare("    reg signed [3:0] a;\n\x20   reg [3:0] b;", "a < b"),
        "0",
        "a signed operand compared against an unsigned one is read unsigned",
    );
}

/// Rule (b) reaching a comparison: `-1` is a signed 32-bit literal, so
/// `a == -1` sign-extends `a` to meet it and holds. The based spelling of the
/// same bit pattern is unsigned and does not.
#[test]
fn a_plain_decimal_literal_is_signed_and_a_based_one_is_not() {
    let compare = |expression: &str| {
        signed_case(
            &format!(
                "    reg signed [3:0] a;\n\
                 \x20   reg y;\n\
                 \x20   initial begin a = 4'b1111; y = {expression}; end",
            ),
            "y",
        )
    };
    assert_eq!(compare("a == -1"), "1");
    // `4'd15` is unsigned, so the comparison is unsigned and `a` is read as 15.
    assert_eq!(compare("a == 4'd15"), "1");
    // ...and against the signed spelling of the same four bits, both sides are
    // signed and both are -1.
    assert_eq!(compare("a == 4'sd15"), "1");
    // The discriminating pair, and it is the *literal's* marker that decides.
    // Against `8'sd15` both sides are signed, so `a` sign-extends to -1 and
    // meets 15: not equal. Against `8'd15` the literal is unsigned, which makes
    // the whole comparison unsigned, so `a` zero-extends to 15 and meets 15.
    // Same four bits on the left; two answers, from the marker alone.
    assert_eq!(compare("a == 8'sd15"), "0", "-1 is not 15");
    assert_eq!(
        compare("a == 8'd15"),
        "1",
        "an unsigned literal unsigns `a`"
    );
}

/// Section 4.1.5: signed and unsigned `+ - *` produce the same bits at a common
/// width — the whole difference is the extension that got them there — and `/`
/// and `%` do not.
#[test]
fn only_division_and_modulus_differ_between_signed_and_unsigned() {
    let compute = |signedness: &str, expression: &str| {
        signed_case(
            &format!(
                "    reg {signedness}[3:0] a, b;\n\
                 \x20   reg {signedness}[3:0] p;\n\
                 \x20   initial begin a = 4'b1001; b = 4'b0010; p = {expression}; end",
            ),
            "p",
        )
    };
    // Nine and two, or minus seven and two, at four bits: the same bits out.
    for expression in ["a + b", "a - b", "a * b"] {
        assert_eq!(
            compute("signed ", expression),
            compute("", expression),
            "`{expression}` at a common width is the same operation"
        );
    }
    // 9 / 2 = 4; -7 / 2 truncates toward zero to -3, which is `1101`.
    assert_eq!(compute("", "a / b"), "0100");
    assert_eq!(compute("signed ", "a / b"), "1101");
    // 9 % 2 = 1; -7 % 2 takes the sign of the first operand and is -1.
    assert_eq!(compute("", "a % b"), "0001");
    assert_eq!(compute("signed ", "a % b"), "1111");
}

/// Section 4.1.12: `>>>` shifts in the sign bit of a signed expression and
/// zeros otherwise, and `>>` shifts in zeros whatever the declaration says.
#[test]
fn arithmetic_right_shift_fills_with_the_sign_only_when_signed() {
    let shift = |signedness: &str, spelling: &str| {
        signed_case(
            &format!(
                "    reg {signedness}[7:0] a;\n\
                 \x20   reg [7:0] p;\n\
                 \x20   initial begin a = 8'b10000000; p = a {spelling} 2; end",
            ),
            "p",
        )
    };
    assert_eq!(shift("signed ", ">>>"), "11100000");
    assert_eq!(shift("", ">>>"), "00100000", "unsigned `>>>` is `>>`");
    assert_eq!(shift("signed ", ">>"), "00100000", "`>>` never sign-fills");
    // `<<<` is `<<`, which the standard states outright.
    assert_eq!(shift("signed ", "<<<"), shift("signed ", "<<"));
    assert_eq!(shift("signed ", "<<<"), "00000000");
}

/// Rules (d), (e) and (f) at run time: a part-select of a whole `reg signed` is
/// unsigned, so the expression containing it is, so the signed sibling reaches
/// it zero-extended. The same expression without the select sign-extends.
#[test]
fn a_select_makes_its_expression_unsigned() {
    let assign = |right: &str| {
        signed_case(
            &format!(
                "    reg signed [3:0] a;\n\
                 \x20   reg signed [7:0] p;\n\
                 \x20   initial begin a = 4'b1111; p = {right}; end",
            ),
            "p",
        )
    };
    // -1 + 0 at eight bits, both operands signed.
    assert_eq!(assign("a + 4'sd0"), "11111111");
    // `a[3:0]` is the same four bits and unsigned, so the sum is unsigned and
    // `a` is zero-extended: 15 + 15 = 30.
    assert_eq!(assign("a + a[3:0]"), "00011110");
    // A concatenation of one element, likewise.
    assert_eq!(assign("a + {a}"), "00011110");
}

/// The signedness travels *down* as well as up. In `(a + b) + c` with `a` and
/// `b` signed and `c` a plain `reg`, the unsigned `c` makes the whole
/// expression unsigned and the inner sum is computed unsigned too — which is
/// section 5.5's "determined from the whole context before evaluation".
#[test]
fn an_unsigned_operand_poisons_the_whole_expression() {
    let assign = |declarations: &str| {
        signed_case(
            &format!(
                "{declarations}\n\
                 \x20   reg [7:0] p;\n\
                 \x20   initial begin a = 4'b1111; b = 4'b0000; c = 4'b0000;\n\
                 \x20       p = (a + b) + c; end",
            ),
            "p",
        )
    };
    // All signed: -1 + 0 + 0 at eight bits.
    assert_eq!(assign("    reg signed [3:0] a, b, c;"), "11111111");
    // `c` unsigned: 15 + 0 + 0 at eight bits, and `a` was zero-extended even
    // though `a + b` on its own would have been signed.
    assert_eq!(
        assign("    reg signed [3:0] a, b;\n\x20   reg [3:0] c;"),
        "00001111",
    );
}

// ===========================================================================
// Elaborated hierarchy (IEEE 1364-2005 sections 12.1.2 and 12.3)
// ===========================================================================
//
// The interpreter is unchanged by hierarchy, and that is the claim these
// tests make: a design of several modules is elaborated into one plan, and the
// same `start`/`resume`/`apply_deferred` that runs a single module runs it.
// Nothing below asks the plan which instance anything came from.

/// A whole elaborated design, run by the same interpreter one module is.
struct Design {
    plan: CanonicalDigitalPlan,
    store: Store,
    /// What each process is waiting for, once it has suspended.
    waits: Vec<Option<(DigitalWaitRequest, DigitalResumeState)>>,
}

impl Design {
    fn new(source: &str, module: &str) -> Self {
        let plan = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir_module(source, Some(module))
            .expect("the hierarchy must elaborate and lower")
            .digital;
        let values = plan
            .signals
            .iter()
            .map(|signal| FourStateValue::splat(signal.width, FourStateBit::Unknown))
            .collect();
        let waits = vec![None; plan.processes.len()];
        let reals = vec![0.0; plan.signals.len()];
        Self {
            plan,
            store: Store {
                values,
                reals,
                deferred: Vec::new(),
                driven: BTreeMap::new(),
                driven_reals: BTreeMap::new(),
            },
            waits,
        }
    }

    fn signal(&self, name: &str) -> DigitalSignalId {
        self.plan
            .signals
            .iter()
            .find(|signal| signal.name == name)
            .unwrap_or_else(|| panic!("no elaborated signal named {name}"))
            .id
    }

    fn set(&mut self, name: &str, spelling: &str) {
        let id = self.signal(name);
        self.store.values[usize::from(id)] = parse_value(spelling);
    }

    fn get(&self, name: &str) -> String {
        self.store.values[usize::from(self.signal(name))].spelling()
    }

    /// Run every process from its entry, which is what a kernel does once at
    /// the start of a simulation.
    fn start_all(&mut self) {
        for index in 0..self.plan.processes.len() {
            let outcome = start(&self.plan, &self.plan.processes[index], &mut self.store)
                .expect("the process must run");
            self.record(index, outcome);
        }
    }

    fn record(&mut self, index: usize, outcome: DigitalProcessOutcome) {
        self.waits[index] = match outcome {
            DigitalProcessOutcome::Suspended(suspension) => Some(suspension.into_parts()),
            DigitalProcessOutcome::Finished => None,
        };
    }

    /// Apply every driver's latest contribution to the net it drives.
    ///
    /// A stand-in for the kernel's resolver, and no more than that: a net with
    /// two drivers has two contributions and a table between them, which is
    /// not the compiler's, so a fixture that reaches one is refused here
    /// rather than guessed at.
    fn resolve_drivers(&mut self) {
        let drives: Vec<DigitalDrive> = self.store.driven.values().cloned().collect();
        for drive in drives {
            let count = self.plan.drivers_of(drive.driver.signal).count();
            assert_eq!(
                count, 1,
                "multi-driver resolution belongs to the kernel; signal {:?} has {count} drivers",
                drive.driver.signal
            );
            apply_deferred(
                &self.plan,
                &mut self.store,
                &DigitalDeferredUpdate {
                    target: drive.target.clone(),
                    value: DigitalUpdate::FourState(drive.value.clone()),
                    region: DigitalSchedulingRegion::Active,
                },
            )
            .expect("a drive must apply");
        }
    }

    /// Relax the continuous drivers to their fixed point.
    ///
    /// Every driver re-evaluates and re-drives on each pass, so a combinational
    /// network settles in as many passes as it has levels. Convergence is
    /// asserted rather than assumed: a network that never settles is a
    /// lowering defect, not a test that needs more passes.
    fn settle(&mut self) {
        for _ in 0..16 {
            let before = self.store.values.clone();
            for index in 0..self.plan.processes.len() {
                if self.plan.processes[index].kind != DigitalProcessKind::ContinuousAssign {
                    continue;
                }
                let Some((_, state)) = self.waits[index].clone() else {
                    continue;
                };
                let outcome = resume(
                    &self.plan,
                    &self.plan.processes[index],
                    &state,
                    &mut self.store,
                )
                .expect("a driver must resume");
                self.record(index, outcome);
            }
            self.resolve_drivers();
            if self.store.values == before {
                return;
            }
        }
        panic!("the driver network did not settle");
    }

    /// Move one signal and resume whatever that transition satisfies.
    ///
    /// The sensitivity test is the interpreter's own
    /// [`any_term_is_satisfied`], so a process wakes here for exactly the
    /// reason a kernel would wake it.
    fn transition(&mut self, name: &str, spelling: &str) {
        let id = self.signal(name);
        let before = self.store.values[usize::from(id)].clone();
        let after = parse_value(spelling);
        self.store.values[usize::from(id)] = after.clone();

        let woken: Vec<usize> = (0..self.plan.processes.len())
            .filter(|index| match &self.waits[*index] {
                Some((DigitalWaitRequest::Event(terms), _)) => {
                    any_term_is_satisfied(terms, id, &before, &after)
                }
                _ => false,
            })
            .collect();
        for index in woken {
            let state = self.waits[index]
                .as_ref()
                .map(|(_, state)| state.clone())
                .expect("the process is suspended");
            let outcome = resume(
                &self.plan,
                &self.plan.processes[index],
                &state,
                &mut self.store,
            )
            .expect("the woken process must resume");
            self.record(index, outcome);
        }

        // Section 11: the nonblocking updates of the slot land after every
        // process in it has run, and the drivers settle on what they wrote.
        let updates = std::mem::take(&mut self.store.deferred);
        for update in &updates {
            apply_deferred(&self.plan, &mut self.store, update).expect("an update must apply");
        }
        self.settle();
    }
}

/// A gate library and a two-level structural design over it.
const NAND2: &str = "module nand2(y, a, b);\n\
                     \x20   output y;\n\
                     \x20   input a, b;\n\
                     \x20   wire y, a, b;\n\
                     \x20   assign y = ~(a & b);\n\
                     endmodule\n";

fn structural(child: &str, top_section: &str) -> String {
    format!(
        "{child}\n\
         module top(p, n);\n\
         \x20   inout p, n;\n\
         \x20   electrical p, n;\n\
         {top_section}\n\
         \x20   analog I(p, n) <+ V(p, n);\n\
         endmodule\n"
    )
}

/// A half adder built from two gate instances computes what a half adder
/// computes. The point is not the arithmetic: it is that two instances of two
/// different modules, connected through the parent's nets, evaluate together.
#[test]
fn a_structural_half_adder_computes_its_truth_table() {
    let library = "module xor2(y, a, b);\n\
                   \x20   output y;\n\
                   \x20   input a, b;\n\
                   \x20   wire y, a, b;\n\
                   \x20   assign y = a ^ b;\n\
                   endmodule\n\
                   module and2(y, a, b);\n\
                   \x20   output y;\n\
                   \x20   input a, b;\n\
                   \x20   wire y, a, b;\n\
                   \x20   assign y = a & b;\n\
                   endmodule\n";
    let source = structural(
        library,
        "    wire a, b, sum, carry;\n\
     \x20   xor2 x1(sum, a, b);\n\
     \x20   and2 a1(carry, a, b);",
    );

    for (a, b) in [("0", "0"), ("0", "1"), ("1", "0"), ("1", "1")] {
        let mut design = Design::new(&source, "top");
        design.set("a", a);
        design.set("b", b);
        design.start_all();
        design.resolve_drivers();
        design.settle();

        let expected_sum = u8::from((a == "1") ^ (b == "1"));
        let expected_carry = u8::from((a == "1") && (b == "1"));
        assert_eq!(design.get("sum"), expected_sum.to_string(), "a={a} b={b}");
        assert_eq!(
            design.get("carry"),
            expected_carry.to_string(),
            "a={a} b={b}"
        );
    }
}

/// Two levels of hierarchy: the top instantiates an AND that is itself two
/// NAND instances. Nothing in the plan says so — there is one flat network of
/// four nets and two drivers — and it computes `a & b`.
#[test]
fn a_two_level_hierarchy_computes_through_both_levels() {
    let library = format!(
        "{NAND2}\
         module and2(y, a, b);\n\
         \x20   output y;\n\
         \x20   input a, b;\n\
         \x20   wire y, a, b, n1;\n\
         \x20   nand2 g1(n1, a, b);\n\
         \x20   nand2 g2(y, n1, n1);\n\
         endmodule\n"
    );
    let source = structural(&library, "    wire a, b, y;\n     and2 u1(y, a, b);");

    for (a, b) in [("0", "0"), ("0", "1"), ("1", "0"), ("1", "1")] {
        let mut design = Design::new(&source, "top");
        // The elaborated design is flat: the inner net is the only one that
        // needed a hierarchical name.
        assert_eq!(design.plan.signals.len(), 4);
        assert_eq!(design.plan.processes.len(), 2);
        design.set("a", a);
        design.set("b", b);
        design.start_all();
        design.resolve_drivers();
        design.settle();
        let expected = u8::from((a == "1") && (b == "1"));
        assert_eq!(design.get("y"), expected.to_string(), "a={a} b={b}");
    }
}

/// The ISCAS-85 c17 benchmark, six instances of one gate module, over every
/// one of its thirty-two input vectors.
///
/// Six instances of one module is the case a flattening that lost identity
/// would get wrong in a way no smaller fixture would show: the gates share a
/// source module, three nets fan out to two gates each, and the answer depends
/// on every instance having evaluated its own inputs.
#[test]
fn the_c17_benchmark_computes_over_its_whole_input_space() {
    let source = structural(
        NAND2,
        "    wire n1, n2, n3, n6, n7;\n\
     \x20   wire n10, n11, n16, n19, n22, n23;\n\
     \x20   nand2 g10(n10, n1, n3);\n\
     \x20   nand2 g11(n11, n3, n6);\n\
     \x20   nand2 g16(n16, n2, n11);\n\
     \x20   nand2 g19(n19, n11, n7);\n\
     \x20   nand2 g22(n22, n10, n16);\n\
     \x20   nand2 g23(n23, n16, n19);",
    );

    for vector in 0u8..32 {
        let bit = |position: u8| u8::from(vector & (1 << position) != 0);
        let (n1, n2, n3, n6, n7) = (bit(0), bit(1), bit(2), bit(3), bit(4));
        // An independent evaluation of the same netlist, which is what makes
        // this a check rather than a restatement of the compiler's answer.
        let nand = |x: u8, y: u8| 1 - (x & y);
        let n10 = nand(n1, n3);
        let n11 = nand(n3, n6);
        let n16 = nand(n2, n11);
        let n19 = nand(n11, n7);
        let n22 = nand(n10, n16);
        let n23 = nand(n16, n19);

        let mut design = Design::new(&source, "top");
        for (name, value) in [("n1", n1), ("n2", n2), ("n3", n3), ("n6", n6), ("n7", n7)] {
            design.set(name, &value.to_string());
        }
        design.start_all();
        design.resolve_drivers();
        design.settle();
        assert_eq!(design.get("n22"), n22.to_string(), "vector {vector:05b}");
        assert_eq!(design.get("n23"), n23.to_string(), "vector {vector:05b}");
    }
}

/// A two-stage shift register: two instances of one flip-flop module, clocked
/// together.
///
/// The sequential case, and the one that needs per-instance process identity —
/// each edge resumes two processes, each writing its own instance's variable —
/// and the implicit driver of IEEE 1364-2005 section 12.3.9.2, which is what
/// carries each instance's `reg` out onto the net the next stage reads.
#[test]
fn a_two_stage_shift_register_shifts_one_stage_per_edge() {
    let source = structural(
        "module dff(q, clk, d);\n\
         \x20   output q;\n\
         \x20   input clk, d;\n\
         \x20   reg q;\n\
         \x20   wire clk, d;\n\
         \x20   always @(posedge clk) q <= d;\n\
         endmodule\n",
        "    wire clk, d, q1, q2;\n\
     \x20   dff u1(.q(q1), .clk(clk), .d(d));\n\
     \x20   dff u2(.q(q2), .clk(clk), .d(q1));",
    );
    let mut design = Design::new(&source, "top");
    design.set("clk", "0");
    design.set("d", "1");
    design.set("u1.q", "0");
    design.set("u2.q", "0");
    design.start_all();
    design.resolve_drivers();
    design.settle();
    assert_eq!(design.get("q1"), "0");
    assert_eq!(design.get("q2"), "0");

    // First edge: the `1` on `d` reaches the first stage only. Both flops
    // sampled at the same instant, so the second saw the *old* `q1`, which is
    // the whole reason a nonblocking assignment exists.
    design.transition("clk", "1");
    assert_eq!(design.get("q1"), "1");
    assert_eq!(design.get("q2"), "0");

    design.transition("clk", "0");
    assert_eq!(design.get("q1"), "1", "nothing happens on the falling edge");
    assert_eq!(design.get("q2"), "0");

    design.transition("clk", "1");
    assert_eq!(design.get("q1"), "1");
    assert_eq!(design.get("q2"), "1", "the second edge shifts it on");
}

/// Two instances driving one net are two contributions, kept apart.
///
/// The execution half of the multi-driver pin: the store holds one value per
/// driver identity, so nothing has overwritten anything. A collapse that
/// merged the two output ports into one driver of the net would have destroyed
/// one of these before a resolver could see it, and no assertion about the
/// net's *value* would have noticed.
#[test]
fn two_instances_driving_one_net_contribute_separately() {
    let source = structural(
        "module drv(y, a);\n\
         \x20   output y;\n\
         \x20   input a;\n\
         \x20   wire y, a;\n\
         \x20   assign y = a;\n\
         endmodule\n",
        "    wire a, b, bus;\n\
     \x20   drv d1(bus, a);\n\
     \x20   drv d2(bus, b);",
    );
    let mut design = Design::new(&source, "top");
    design.set("a", "1");
    design.set("b", "0");
    design.start_all();

    let bus = design.signal("bus");
    let drivers: Vec<DigitalDriverId> = design.plan.drivers_of(bus).map(|d| d.id).collect();
    assert_eq!(drivers.len(), 2);
    assert_eq!(design.store.driven.len(), 2, "one contribution per driver");
    let contributions: Vec<String> = drivers
        .iter()
        .map(|driver| design.store.driven[driver].value.spelling())
        .collect();
    assert_eq!(
        contributions,
        vec!["1".to_string(), "0".to_string()],
        "each instance published what it computed, and neither overwrote the other"
    );
}
