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
    DigitalProcessOutcome, DigitalRealDrive, DigitalResumeState, DigitalScalar, DigitalSuspension,
    DigitalUpdate, DigitalWaitRequest, any_term_is_satisfied, apply_deferred, classify_edge,
    resume, start,
};
use rspice_veriloga::canonical_ir::digital_value::FourStateValue;
use rspice_veriloga::canonical_ir::ids::{DigitalAnalogProbeId, DigitalSignalId};
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

#[test]
fn digital_clock_queries_preserve_integer_bits_rounding_and_resume_time() {
    use rspice_veriloga::canonical_ir::{CfgValueKind, CfgValueType, DigitalClock};
    let source = "`timescale 10ns/1ps\nmodule clocked;
        reg [63:0] t; reg [31:0] st; reg [127:0] wide; real rt, at;
        initial begin
            t=$time; st=$stime; wide=$time+1; rt=$realtime; at=$abstime;
            #0;
            t=$time; st=$stime; wide=$time+1; rt=$realtime; at=$abstime;
        end endmodule";
    let mut harness = Harness::from_source(source);
    assert!(matches!(
        start(
            &harness.plan,
            &harness.plan.processes[0],
            &mut harness.store
        ),
        Err(DigitalEvalError::ClockUnavailable)
    ));
    harness.store.clock = Some(DigitalClock {
        tick: 14_999,
        absolute_seconds: 14.999e-9,
    });
    let suspension = expect_suspended(harness.start(0));
    assert_eq!(harness.get("t"), format!("{:064b}", 1));
    assert_eq!(harness.get_real("rt"), 1.4999);
    assert_eq!(harness.get_real("at"), 14.999e-9);
    harness.store.clock = Some(DigitalClock {
        tick: 15_000,
        absolute_seconds: 15e-9,
    });
    assert!(matches!(
        harness.resume(0, suspension.resume_state()),
        DigitalProcessOutcome::Finished
    ));
    assert_eq!(harness.get("t"), format!("{:064b}", 2));
    assert_eq!(harness.get("st"), format!("{:032b}", 2));
    assert_eq!(harness.get("wide"), format!("{:0128b}", 3));
    assert_eq!(harness.get_real("rt"), 1.5);
    assert_eq!(harness.get_real("at"), 15e-9);

    // Above the exact f64 integer range, and the specified low-32-bit $stime
    // wraparound. A widened enclosing expression must not widen $stime itself.
    for (tick, low) in [
        (4_294_967_301_u64, 5_u64),
        (9_007_199_254_740_993, 1),
        (u64::MAX, u32::MAX as u64),
    ] {
        let mut wide = Harness::from_source(
            "`timescale 1fs/1fs\nmodule wide; reg [127:0] t, st; initial begin t=$time+128'd1; st=$stime+128'd1; end endmodule",
        );
        wide.store.clock = Some(DigitalClock {
            tick,
            absolute_seconds: tick as f64 * 1e-15,
        });
        assert!(matches!(wide.start(0), DigitalProcessOutcome::Finished));
        assert_eq!(wide.get("t"), format!("{:0128b}", u128::from(tick) + 1));
        assert_eq!(wide.get("st"), format!("{:0128b}", u128::from(low) + 1));
    }
    let mut altered = harness.plan.clone();
    let value = altered.processes[0]
        .function
        .values
        .iter_mut()
        .find(|value| matches!(value.kind, CfgValueKind::DigitalTime { .. }))
        .unwrap();
    value.value_type = CfgValueType::FourState { width: 7 };
    assert!(altered.validate().is_err());
    for function in ["$time", "$stime", "$realtime", "$abstime"] {
        let source = format!("module bad; reg [63:0] q; initial q={function}(1); endmodule");
        assert!(
            VerilogACompiler::default()
                .compile_canonical_ir_module(&source, None)
                .is_err(),
            "{function}"
        );
    }
}

/// A signal store, a nonblocking-update queue, and one slot per driver: the
/// smallest thing that satisfies [`DigitalEnvironment`], and a stand-in for the
/// event kernel's own.
struct Store {
    clock: Option<rspice_veriloga::canonical_ir::DigitalClock>,
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
    /// The continuous-domain potential each of the plan's probes reads
    /// (Verilog-AMS LRM 2.4 section 7.3.3), in probe id order.
    ///
    /// `None` until a test samples one in, so that a fixture which forgets to
    /// supply an analog solution is refused by the interpreter rather than
    /// passing against a fabricated zero — the same discipline the real store
    /// applies.
    analog: Vec<Option<f64>>,
}

#[test]
fn digital_clock_queries_compose_with_wide_signed_arithmetic_and_shifts() {
    let mut harness = Harness::from_source(
        "`timescale 1fs/1fs\nmodule wide;
        reg signed [128:0] a,b,d,m,product,fill;
        reg [128:0] u,square,shifted,empty,invalid,unsized_value,decimal_value;
        reg lt,gt,bad; reg [256:0] count;
        initial begin
            a=-129'd7; b=129'd2; d=a/b; m=a%b; product=a*b;
            u=$time+(129'd1<<64); square=u*u;
            count=257'd1; shifted=u<<count;
            count=257'd1<<200; empty=u>>count; fill=a>>>count;
            invalid=u/129'd0; lt=(a<b); gt=(square>u); bad=(129'bx<u);
            unsized_value='h100000000000000000000000000000001;
            decimal_value=129'd340282366920938463463374607431768211457;
        end endmodule",
    );
    harness.store.clock = Some(rspice_veriloga::canonical_ir::DigitalClock {
        tick: 1,
        absolute_seconds: 1e-15,
    });
    assert!(matches!(harness.start(0), DigitalProcessOutcome::Finished));
    assert_eq!(harness.get("u"), format!("{:0129b}", (1u128 << 64) + 1));
    assert_eq!(
        harness.get("square"),
        format!("1{:0128b}", (1u128 << 65) + 1)
    );
    assert_eq!(
        harness.get("shifted"),
        format!("{:0129b}", (1u128 << 65) + 2)
    );
    assert_eq!(harness.get("d"), format!("{}01", "1".repeat(127)));
    assert_eq!(harness.get("m"), "1".repeat(129));
    assert_eq!(harness.get("product"), format!("{}0010", "1".repeat(125)));
    assert_eq!(harness.get("empty"), "0".repeat(129));
    assert_eq!(harness.get("fill"), "1".repeat(129));
    assert_eq!(harness.get("invalid"), "x".repeat(129));
    assert_eq!(harness.get("lt"), "1");
    assert_eq!(harness.get("gt"), "1");
    assert_eq!(harness.get("bad"), "x");
    assert_eq!(
        harness.get("unsized_value"),
        format!("1{}1", "0".repeat(127))
    );
    assert_eq!(harness.get("decimal_value"), harness.get("unsized_value"));
}

impl DigitalEnvironment for Store {
    fn read_clock(&self) -> Option<rspice_veriloga::canonical_ir::DigitalClock> {
        self.clock
    }
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

    fn read_analog_variable(&self, probe: DigitalAnalogProbeId) -> Option<f64> {
        self.read_analog_potential(probe)
    }

    fn read_analog_flow(&self, probe: DigitalAnalogProbeId) -> Option<f64> {
        self.read_analog_potential(probe)
    }

    fn read_analog_potential(&self, probe: DigitalAnalogProbeId) -> Option<f64> {
        self.analog.get(usize::from(probe)).copied().flatten()
    }
}

struct Harness {
    plan: CanonicalDigitalPlan,
    store: Store,
}

impl Harness {
    fn new(section: &str) -> Self {
        Self::from_source(&digital_module(section))
    }

    /// A fixture written as a whole module, with no analog block in it.
    ///
    /// The ownership rule for a module-level `real` (see the `digital_lower`
    /// module documentation) moves one into the discrete domain only in a
    /// module that declares no analog block, so a fixture about real *state*
    /// cannot be built with [`digital_module`], which declares one.
    fn rnm(source: &str) -> Self {
        Self::from_source(source)
    }

    fn from_source(source: &str) -> Self {
        Self::from_module(source, None)
    }

    fn from_module(source: &str, module: Option<&str>) -> Self {
        let plan = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir_module(source, module)
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
        let analog = vec![None; plan.analog_probes.len()];
        Self {
            plan,
            store: Store {
                clock: None,
                values,
                reals,
                deferred: Vec::new(),
                driven: BTreeMap::new(),
                driven_reals: BTreeMap::new(),
                analog,
            },
        }
    }

    /// The id of the probe a spelling names, as
    /// [`DigitalAnalogProbe::spelling`] renders it.
    fn probe(&self, spelling: &str) -> DigitalAnalogProbeId {
        self.plan
            .analog_probes
            .iter()
            .find(|probe| probe.spelling() == spelling)
            .unwrap_or_else(|| panic!("no analog probe spelled {spelling}"))
            .id
    }

    /// Publish one analog solution's value for a probe.
    fn set_analog(&mut self, spelling: &str, value: f64) {
        let id = self.probe(spelling);
        self.store.analog[usize::from(id)] = Some(value);
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
        let (updates, delayed): (Vec<_>, Vec<_>) = std::mem::take(&mut self.store.deferred)
            .into_iter()
            .partition(|update| matches!(update.wait, None | Some(DigitalWaitRequest::Delay(0))));
        self.store.deferred = delayed;
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
                    wait: None,
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
     \x20   initial q = #5 d;",
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
    assert_eq!(harness.get("q"), "1", "the value read before the delay");
}

#[test]
fn delayed_nonblocking_writes_capture_values_and_continue_without_suspending() {
    let mut h = Harness::from_source(
        "`timescale 1ns/100ps\nmodule timed;
         reg [7:0] q, data, duration, stage; real r;
         initial begin
           q=0; data=8'h42; duration=3; stage=0; r=0.0;
           q <= #duration data;
           r <= #0.15 1.25;
           {q[3:0],q[7:4]} <= #duration 8'hab;
           q <= #0 8'h11;
           q <= #(1'bx) 8'h22;
           data=8'h77; duration=9; stage=1;
           #1 stage=2;
         end endmodule",
    );
    let suspension = expect_suspended(h.run());
    assert_eq!(
        *suspension.wait(),
        DigitalWaitRequest::Delay(10),
        "only the explicit statement delay suspends"
    );
    assert_eq!(h.get("stage"), "00000001");
    assert_eq!(h.get("q"), "00000000");
    assert_eq!(h.deferred_count(), 6);
    h.flush_nonblocking();
    assert_eq!(
        h.get("q"),
        "00100010",
        "zero and X delays stay in this NBA region"
    );
    assert_eq!(h.get_real("r"), 0.0);
    assert_eq!(h.deferred_count(), 4);
    let captures = &h.store.deferred;
    assert_eq!(
        captures
            .iter()
            .map(|u| match u.wait {
                Some(DigitalWaitRequest::Delay(ticks)) => ticks,
                _ => panic!("expected captured delay"),
            })
            .collect::<Vec<_>>(),
        [30, 2, 30, 30]
    );
    assert_eq!(
        captures[0].value,
        DigitalUpdate::FourState(FourStateValue::from_u64(8, 0x42))
    );
    assert_eq!(captures[1].value, DigitalUpdate::Real(1.25));
    assert_eq!(
        captures[2].value,
        DigitalUpdate::FourState(FourStateValue::from_u64(4, 0xa))
    );
    assert_eq!(
        captures[3].value,
        DigitalUpdate::FourState(FourStateValue::from_u64(4, 0xb))
    );
    expect_finished(h.resume(0, suspension.resume_state()));
    assert_eq!(h.get("stage"), "00000010");
    assert_eq!(h.deferred_count(), 4, "completion cannot cancel captures");
}

#[test]
fn event_nonblocking_writes_capture_values_sensitivities_and_continue() {
    let mut h = Harness::from_source(
        "module events; reg a,b,done; reg [7:0] data,q,implicit_q; wreal changed; real held;
         initial begin
           done=0; data=8'h42; held=0.0;
           q <= @(posedge a or negedge b) data;
           held <= @(changed) 1.25;
           implicit_q <= @* data;
           data=8'h99; done=1;
         end endmodule",
    );
    expect_finished(h.run());
    assert_eq!(h.get("done"), "1");
    assert_eq!(h.get("q"), "xxxxxxxx");
    assert_eq!(h.get_real("held"), 0.0);
    assert_eq!(h.deferred_count(), 3);
    h.flush_nonblocking();
    assert_eq!(
        h.deferred_count(),
        3,
        "unsatisfied events do not enter NBA delivery"
    );
    let captures = &h.store.deferred;
    let Some(DigitalWaitRequest::Event(terms)) = &captures[0].wait else {
        panic!("event capture")
    };
    assert_eq!(
        terms.iter().map(|t| (t.signal, t.edge)).collect::<Vec<_>>(),
        [
            (h.signal("a"), Some(DigitalEdge::Posedge)),
            (h.signal("b"), Some(DigitalEdge::Negedge))
        ]
    );
    assert_eq!(
        captures[0].value,
        DigitalUpdate::FourState(FourStateValue::from_u64(8, 0x42))
    );
    assert_eq!(captures[1].value, DigitalUpdate::Real(1.25));
    let Some(DigitalWaitRequest::Event(terms)) = &captures[2].wait else {
        panic!("implicit event capture")
    };
    assert_eq!(terms.len(), 1);
    assert_eq!(terms[0].signal, h.signal("data"));
    assert_eq!(
        captures[2].value,
        DigitalUpdate::FourState(FourStateValue::from_u64(8, 0x42))
    );
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
        assert!(
            !rendered.contains("Internal error"),
            "a construct the author wrote is not an internal error: {rendered}"
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
        // A module-level `integer` that only a process writes is that
        // process's own signal now, so the refusal that remains in the
        // module-level family is the *initial value*: an initializer is
        // scheduled by the domain that owns the variable, and a variable the
        // discrete domain has taken over has no continuous-domain schedule to
        // be initialized from.
        (
            "    real bias = 1.0;\n\
             \x20   reg q;\n\
             \x20   initial begin q = 1'b0; bias = 2.0; end",
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
        assert!(
            !rendered.contains("Internal error"),
            "a construct the author wrote is not an internal error: {rendered}"
        );
    }
}

/// Every construct the discrete-domain lowering refuses reaches its author the
/// way a refused construct should: a semantic error naming the construct at its
/// offset. None of them arrives as `Internal error`.
///
/// That distinction is the whole point of the row set. `Internal error` says
/// the compiler is broken and leaves the author nothing to do; a program that
/// declares a `string` inside a process is not a broken compiler, it is a
/// construct this lowering does not build, and the author can only know that
/// if the compiler says so. Spectre and Virtuoso reserve the internal-error
/// phrasing for invariant violations, and `canonical_ir/digital_lower.rs`
/// classifies each of its diagnostic sites accordingly.
///
/// One row per construct that was found to reach a refusal. Two of them are
/// answered by the analyzer before the lowering sees them — an unpacked array
/// and a run-time select bound — and stay here anyway, because the subject is
/// what the author is told rather than which pass says it. Every row is a
/// program the front end already refused, so the set is also the pin that no
/// refused program quietly started compiling.
#[test]
fn no_digital_lowering_refusal_reaches_the_author_as_an_internal_error() {
    let cases = [
        // Verilog-AMS LRM 2.4 section 6.5.3: one driver of a `wreal`.
        (
            "    wreal a, b, bus;\n\
             \x20   assign bus = a;\n\
             \x20   assign bus = b;",
            "section 6.5.3",
        ),
        // A process-local declaration this lowering has no storage for.
        (
            "    reg q;\n\
             \x20   initial begin : work string s; q = 1'b0; end",
            "process-local `string`",
        ),
        // An unpacked array inside a process: the analyzer owns this one and
        // the lowering's own array arms are unreachable behind it. The row
        // stays because the suite's subject is what an author is told, not
        // which pass says it.
        (
            "    reg q;\n\
             \x20   initial begin : work reg [1:0] m [0:3]; q = 1'b0; end",
            "unpacked array dimensions on the process-local `m`",
        ),
        // A process-local is an SSA value, so neither a deferred update nor a
        // partial write has anywhere to land.
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
        // A module-level variable the discrete domain has taken over, whose
        // initializer the continuous domain no longer schedules.
        (
            "    real bias = 1.0;\n\
             \x20   reg q;\n\
             \x20   initial begin q = 1'b0; bias = 2.0; end",
            "module-level",
        ),
        // A select whose bound is only known at run time. The analyzer owns
        // this one now; the lowering's constant fold stands behind it.
        (
            "    reg [3:0] q;\n\
             \x20   initial begin : work integer i; i = 0; q[i] = 1'b1; end",
            "must have constant bounds",
        ),
        // `@*` over a statement that reads nothing would never resume.
        (
            "    reg q;\n\
             \x20   always @* q = 1'b1;",
            "names no signal",
        ),
        // No row for `sized`'s "a real value has no four-state form here": an
        // assignment and a `repeat` count both reach a real through VAMS-2023
        // 4.2.1's numeric conversion and lower, so no program was found that
        // reaches that arm. It stays classified as a refusal — if a position
        // ever does reach it, the author is told what they wrote, not that the
        // compiler broke.
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
        assert!(
            !rendered.contains("Internal error"),
            "a construct the author wrote is not an internal error: {rendered}"
        );
        assert!(
            rendered.contains("Semantic error at offset"),
            "a refused construct is reported at its offset: {rendered}"
        );
    }
}

/// A run-time select position is refused by the analyzer, not by an invariant.
///
/// `q[i] = 1'b0` is legal Verilog — IEEE 1364-2005 section 4.2.1 admits a
/// variable bit select on the left-hand side — and this lowering has no
/// read-modify-write node for one. That is a limitation of this compiler, so
/// it is reported the way a user construct is reported: a semantic error that
/// names what was written and carries the offset it was written at. It used to
/// reach `digital_lower`'s constant fold instead and come back as "Internal
/// error: canonical IR validation failed", which tells an author that the
/// compiler is broken rather than that their program is not supported yet.
///
/// The last case is the boundary the refusal must not cross: a bit select
/// being *read* lowers to a node that takes its position as a value, so a
/// run-time position there is a program rather than a refusal.
#[test]
fn a_run_time_select_bound_is_refused_by_the_analyzer() {
    for (section, expected) in [
        (
            "    reg [3:0] q;\n\
         \x20   integer i;\n\
         \x20   initial for (i = 0; i < 4; i = i + 1) q[i] = 1'b0;",
            "a bit select on the left-hand side must have constant bounds",
        ),
        (
            "    reg [3:0] q;\n\
         \x20   integer i;\n\
         \x20   initial begin i = 1; q[i:0] = 2'b01; end",
            "a part select of `q` must have constant bounds",
        ),
        (
            "    reg [3:0] q;\n\
         \x20   reg [1:0] seen;\n\
         \x20   integer i;\n\
         \x20   initial begin i = 1; seen = q[i:0]; end",
            "a part select of `q` must have constant bounds",
        ),
    ] {
        let error = VerilogACompiler::new(CompilerOptions::default())
            .compile_canonical_ir(&digital_module(section))
            .expect_err("a run-time select position must be refused");
        let rendered = error.to_string();
        assert!(
            rendered.starts_with("Semantic error"),
            "a user construct must be refused by the analyzer: {rendered}"
        );
        assert!(
            rendered.contains(expected),
            "expected `{expected}` to be named in: {rendered}"
        );
    }

    VerilogACompiler::new(CompilerOptions::default())
        .compile_canonical_ir(&digital_module(
            "    reg [3:0] q;\n\
         \x20   reg seen;\n\
         \x20   integer i;\n\
         \x20   initial begin i = 1; seen = q[i]; end",
        ))
        .expect("a bit select read at a run-time position still lowers");
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
// Real state (IEEE 1364-2005 section 3.9)
// ===========================================================================
//
// What a real-number model is built out of, and what could not be written
// before: a real value that is still there on the next clock edge. Every test
// here runs the state through at least one suspension, because a real that
// survives no suspension is a real expression rather than real state.

/// A process-local `real` accumulator, across a suspension.
///
/// The critical path, and the one nothing else covers: a process-local is an
/// SSA variable of the process function, so it does not live in the signal
/// store and nothing outside can hold it while the process is asleep. It
/// crosses the `Wait` as a resume argument or it does not cross at all — and
/// the interpreter starts a resumption with an empty value table, so a
/// lowering that assumed a register kept it would read nothing here.
///
/// The accumulator is read *and* written on each pass, which is what makes the
/// carry load-bearing: a lowering that re-initialised the local on resumption
/// would leave `acc` at section 3.9's `0.0` every time and the sum would be the
/// last addend rather than the total.
///
/// This case suspends inside a `forever`; the re-entry cases below also require
/// the static lifetime when control leaves and re-enters the declaring block.
#[test]
fn a_process_local_real_survives_a_suspension() {
    let mut harness = Harness::rnm(
        "module dut(clk, step, total);\n\
         \x20   input clk;\n\
         \x20   input wreal step;\n\
         \x20   output wreal total;\n\
         \x20   always begin : accumulate\n\
         \x20     real acc;\n\
         \x20     forever begin\n\
         \x20       @(posedge clk);\n\
         \x20       acc = acc + step;\n\
         \x20     end\n\
         \x20   end\n\
         endmodule\n",
    );

    const ADDENDS: [f64; 4] = [1.5, 0.25, -0.75, 4.0];
    harness.set("clk", "0");
    let mut state = expect_suspended(harness.start(0)).resume_state().clone();
    for (index, addend) in ADDENDS.iter().enumerate() {
        harness.set_real("step", *addend);
        // One resumption per rising edge. Falling edges do not satisfy a
        // `posedge` term, so a kernel would not resume there and neither does
        // this — resuming anyway would run the body twice per cycle.
        harness.set("clk", "1");
        state = expect_suspended(harness.resume(0, &state))
            .resume_state()
            .clone();
        harness.set("clk", "0");
        let expected: f64 = ADDENDS[..=index].iter().sum();
        // The resume state is the only place the accumulator can be, so
        // reading it out of the state is reading exactly the claim.
        assert!(
            state
                .arguments()
                .iter()
                .any(|value| matches!(value, DigitalScalar::Real(held) if *held == expected)),
            "after {} edge(s) the carried accumulator should be {expected}, and the state \
             carries {:?}",
            index + 1,
            state.arguments()
        );
    }
}

#[test]
fn named_block_locals_retain_values_when_control_reenters_the_block() {
    for process in [
        "always #1 begin : acc real x; x = x + 1.0; q = x; end",
        "always begin : acc real x; #1; x = x + 1.0; q = x; end",
        "initial forever begin : acc real x; #1; x = x + 1.0; q = x; end",
        "initial repeat (3) begin : acc real x; #1; x = x + 1.0; q = x; end",
    ] {
        let mut harness =
            Harness::from_source(&format!("module dut(output real q); {process} endmodule"));
        let mut state = expect_suspended(harness.start(0)).resume_state().clone();
        for expected in [1.0, 2.0, 3.0] {
            let outcome = harness.resume(0, &state);
            assert_eq!(harness.get_real("q"), expected, "{process}");
            if expected < 3.0 {
                state = expect_suspended(outcome).resume_state().clone();
            }
        }
    }
}

#[test]
fn static_local_initializers_run_once_per_process_start() {
    let mut harness = Harness::from_source(
        r#"
module dut(output reg [7:0] q);
always #1 begin : acc
    reg [7:0] count = 8'd4;
    count = count + 1;
    q = count;
end
endmodule
"#,
    );
    for _ in 0..2 {
        let mut state = expect_suspended(harness.start(0)).resume_state().clone();
        for expected in ["00000101", "00000110", "00000111"] {
            state = expect_suspended(harness.resume(0, &state))
                .resume_state()
                .clone();
            assert_eq!(harness.get("q"), expected);
        }
    }
}

#[test]
fn static_local_state_survives_waits_outside_its_lexical_scope() {
    let mut harness = Harness::from_source(
        r#"
module dut(output real q, output real r);
initial forever begin
    #1;
    begin : left real x; x = x + 1.0; q = x; end
    #1;
    begin : right real x; x = x + 10.0; r = x; end
end
endmodule
"#,
    );
    let mut state = expect_suspended(harness.start(0)).resume_state().clone();
    for expected in [1.0, 2.0, 3.0] {
        state = expect_suspended(harness.resume(0, &state))
            .resume_state()
            .clone();
        assert_eq!(harness.get_real("q"), expected);
        state = expect_suspended(harness.resume(0, &state))
            .resume_state()
            .clone();
        assert_eq!(harness.get_real("r"), expected * 10.0);
    }
}

#[test]
fn static_locals_are_independent_between_instances_and_resume_snapshots() {
    let mut harness = Harness::from_module(
        r#"
module counter(output real q);
always #1 begin : acc real x; x = x + 1.0; q = x; end
endmodule
module dut(input clk);
counter a();
counter b();
endmodule
"#,
        Some("dut"),
    );
    let a = expect_suspended(harness.start(0)).resume_state().clone();
    let b = expect_suspended(harness.start(1)).resume_state().clone();
    let a_next = expect_suspended(harness.resume(0, &a))
        .resume_state()
        .clone();
    harness.resume(0, &a_next);
    assert_eq!(harness.get_real("a.q"), 2.0);
    // Replaying the saved frame restores its static values, not the values
    // left by a later evaluation or by another elaborated instance.
    harness.resume(0, &a);
    assert_eq!(harness.get_real("a.q"), 1.0);
    harness.resume(1, &b);
    assert_eq!(harness.get_real("b.q"), 1.0);
}

#[test]
fn resume_refuses_a_recompiled_process_before_writing_any_signal() {
    let mut old = Harness::new("reg q; initial #1 q = 1'b1;");
    let state = expect_suspended(old.run()).resume_state().clone();
    let mut changed = Harness::new("reg q; initial #1 q = 1'b0;");
    changed.set("q", "1");
    resume(
        &changed.plan,
        &changed.plan.processes[0],
        &state,
        &mut changed.store,
    )
    .expect_err("a resume frame must not enter a different compiled process");
    assert_eq!(changed.get("q"), "1", "refusal must precede all writes");
}

#[test]
fn a_portless_digital_module_executes_its_processes() {
    let mut harness = Harness::from_source("module dut; reg q; initial q = 1'b1; endmodule");
    expect_finished(harness.run());
    assert_eq!(harness.get("q"), "1");
}

#[test]
fn identical_recompilation_and_serialization_preserve_resume_identity() {
    let source = "reg q; initial #1 q = 1'b1;";
    let mut original = Harness::new(source);
    let state = expect_suspended(original.run()).resume_state().clone();
    let mut rebuilt = Harness::new(source);
    rebuilt.plan = serde_json::from_str(&serde_json::to_string(&rebuilt.plan).unwrap()).unwrap();
    rebuilt
        .plan
        .validate()
        .expect("round-trip preserves integrity");
    assert_eq!(
        original.plan.content_identity,
        rebuilt.plan.content_identity
    );
    expect_finished(rebuilt.resume(0, &state));
    assert_eq!(rebuilt.get("q"), "1");
}

#[test]
fn a_process_cannot_be_paired_with_another_containing_plan() {
    let mut original = Harness::new("reg q; initial #1 q = 1'b1;");
    let state = expect_suspended(original.run()).resume_state().clone();
    let mut changed = Harness::new("reg q; initial #1 q = 1'b0;");
    changed.set("q", "1");
    assert!(matches!(
        resume(
            &original.plan,
            &changed.plan.processes[0],
            &state,
            &mut changed.store
        ),
        Err(DigitalEvalError::ProcessNotInPlan(_))
    ));
    assert_eq!(changed.get("q"), "1");
}

/// A process-local `real` starts at zero; a four-state local starts at `x`.
///
/// Two clauses, and they disagree on purpose. IEEE 1364-2005 section 3.9 gives
/// a `real` variable an initial value of zero; section 4.2.2 gives an
/// unwritten `reg` or `integer` all-`x`. A `real` has no `x` to start at, so
/// the second rule cannot be stretched to cover it and the lowering needs a
/// separate answer — which is what this pins, by declaring one of each in one
/// declarative region and reading both out of the state the suspension carries
/// before anything writes either.
#[test]
fn an_unwritten_real_local_is_zero_and_a_four_state_one_is_unknown() {
    let mut harness = Harness::rnm(
        "module dut(clk);\n\
         \x20   input clk;\n\
         \x20   always begin : sample\n\
         \x20     real seen;\n\
         \x20     integer count;\n\
         \x20     @(posedge clk);\n\
         \x20     seen = seen + 1.0;\n\
         \x20     count = count + 1;\n\
         \x20   end\n\
         endmodule\n",
    );
    harness.set("clk", "0");
    let state = expect_suspended(harness.start(0)).resume_state().clone();
    let carried = state.arguments();
    assert_eq!(
        carried.len(),
        2,
        "both locals cross the suspension, got {carried:?}"
    );
    assert_eq!(
        carried[0],
        DigitalScalar::Real(0.0),
        "section 3.9: an unwritten `real` is zero"
    );
    let DigitalScalar::FourState(count) = &carried[1] else {
        panic!("the `integer` local must cross as a four-state value, got {carried:?}");
    };
    assert_eq!(
        count.spelling(),
        "x".repeat(32),
        "section 4.2.2: an unwritten `integer` is all-`x`"
    );
}

/// A module-level `real` written by a process, across a clock edge.
///
/// The canonical real-number-model idiom: a one-pole low-pass whose state is
/// the whole model. Written with `<=`, so the update is deferred to the
/// nonblocking region and the state a pass reads is the one the previous pass
/// left — which is what makes the recurrence the recurrence.
///
/// The expected values are the same recurrence evaluated in Rust, in the same
/// order and the same `f64` arithmetic, so agreement is exact rather than
/// within a tolerance.
#[test]
fn a_module_level_real_holds_state_across_a_clock_edge() {
    const K: f64 = 0.25;
    let mut harness = Harness::rnm(
        "module dut(clk, vin, vout);\n\
         \x20   parameter real K = 0.25;\n\
         \x20   input clk;\n\
         \x20   input wreal vin;\n\
         \x20   output wreal vout;\n\
         \x20   real state;\n\
         \x20   always @(posedge clk) state <= state + (vin - state) * K;\n\
         \x20   assign vout = state;\n\
         endmodule\n",
    );

    harness.set("clk", "0");
    // Process 0 is the `always`; process 1 is the continuous assignment, which
    // this test does not drive — `state` is read out of the store directly, so
    // what is being checked is the variable and not the net it feeds.
    let mut state = expect_suspended(harness.start(0)).resume_state().clone();
    let mut reference = 0.0f64;
    for step in 0..8 {
        let input = if step < 4 { 1.0 } else { 0.0 };
        harness.set_real("vin", input);
        harness.set("clk", "1");
        state = expect_suspended(harness.resume(0, &state))
            .resume_state()
            .clone();
        assert_eq!(
            harness.get_real("state"),
            reference,
            "the nonblocking update has not landed yet, so the state is still the old one"
        );
        harness.flush_nonblocking();
        reference += (input - reference) * K;
        assert_eq!(
            harness.get_real("state"),
            reference,
            "step {step} of the recurrence"
        );
        harness.set("clk", "0");
        state = expect_suspended(harness.resume(0, &state))
            .resume_state()
            .clone();
    }
    assert!(
        reference > 0.0,
        "the fixture must actually accumulate, or it proves nothing"
    );
}

/// A blocking write to a real variable takes effect where it is written.
///
/// The other half of section 6.2's pair, and it goes straight into the store
/// rather than through a driver: a variable has no drivers, so the contribution
/// table stays empty and nothing is resolved.
#[test]
fn a_blocking_write_to_a_real_variable_lands_immediately() {
    let mut harness = Harness::rnm(
        "module dut(clk, vin, vout);\n\
         \x20   input clk;\n\
         \x20   input wreal vin;\n\
         \x20   output real vout;\n\
         \x20   always @(posedge clk) vout = vin * 2.0;\n\
         endmodule\n",
    );
    harness.set("clk", "0");
    let state = expect_suspended(harness.run()).resume_state().clone();
    harness.set_real("vin", 1.25);
    harness.set("clk", "1");
    expect_suspended(harness.resume(0, &state));
    assert_eq!(harness.get_real("vout"), 2.5);
    assert_eq!(
        harness.deferred_count(),
        0,
        "a blocking assignment defers nothing"
    );
    assert_eq!(
        harness.drive_count(),
        0,
        "a procedural write is not a driver contribution"
    );
}

/// `$realtobits` and `$bitstoreal` round-trip a real exactly.
///
/// Verilog-AMS LRM 2.4 section 3.7's own bridge. Exactness is the claim: the
/// conversion is the value's IEEE 754 storage rather than a rounding, so a
/// value with a long mantissa comes back bit for bit.
#[test]
fn a_real_round_trips_through_its_bit_pattern() {
    let mut harness = Harness::rnm(
        "module dut(clk, vin, vout);\n\
         \x20   input clk;\n\
         \x20   input wreal vin;\n\
         \x20   output real vout;\n\
         \x20   reg [63:0] pattern;\n\
         \x20   always @(posedge clk) begin\n\
         \x20     pattern = $realtobits(vin);\n\
         \x20     vout = $bitstoreal(pattern);\n\
         \x20   end\n\
         endmodule\n",
    );
    harness.set("clk", "0");
    let mut state = expect_suspended(harness.run()).resume_state().clone();
    for value in [0.1f64, -1.0 / 3.0, 1.7976931348623157e308, 0.0] {
        harness.set_real("vin", value);
        harness.set("clk", "1");
        state = expect_suspended(harness.resume(0, &state))
            .resume_state()
            .clone();
        assert_eq!(harness.get_real("vout"), value, "round trip of {value}");
        assert_eq!(
            harness.get("pattern"),
            format!("{:064b}", value.to_bits()),
            "the pattern is the IEEE 754 storage of {value}"
        );
        harness.set("clk", "0");
        state = expect_suspended(harness.resume(0, &state))
            .resume_state()
            .clone();
    }
}

/// `$bitstoreal` of a pattern with an unknown bit refuses at runtime.
///
/// Neither standard rules on this, and the ruling is stated on
/// [`DigitalEvalError::UnknownBitsToReal`]: the conversion is defined over an
/// IEEE 754 bit pattern and an `x` is the absence of a bit, so there is no real
/// to produce and none is invented. A `reg` nothing has written holds `x`
/// (section 4.2.2), which is how the case arises without contriving it.
#[test]
fn bitstoreal_refuses_an_unknown_bit() {
    let plan = VerilogACompiler::new(CompilerOptions::default())
        .compile_canonical_ir(
            "module dut(clk, vout);\n\
             \x20   input clk;\n\
             \x20   output real vout;\n\
             \x20   reg [63:0] pattern;\n\
             \x20   always @(posedge clk) vout = $bitstoreal(pattern);\n\
             endmodule\n",
        )
        .expect("fixture must lower to canonical IR")
        .digital;
    let mut store = Store {
        clock: None,
        values: plan
            .signals
            .iter()
            .map(|signal| FourStateValue::splat(signal.width, FourStateBit::Unknown))
            .collect(),
        reals: vec![0.0; plan.signals.len()],
        deferred: Vec::new(),
        driven: BTreeMap::new(),
        driven_reals: BTreeMap::new(),
        analog: vec![None; plan.analog_probes.len()],
    };
    let clk = plan
        .signals
        .iter()
        .find(|signal| signal.name == "clk")
        .expect("declared")
        .id;
    store.values[usize::from(clk)] = parse_value("0");
    let outcome = start(&plan, &plan.processes[0], &mut store).expect("the process must suspend");
    let state = expect_suspended(outcome).resume_state().clone();
    store.values[usize::from(clk)] = parse_value("1");
    let error = resume(&plan, &plan.processes[0], &state, &mut store)
        .expect_err("an unknown pattern has no real");
    assert!(
        matches!(error, DigitalEvalError::UnknownBitsToReal(_)),
        "expected the unknown-bits refusal, got {error:?}"
    );
    assert!(
        error.to_string().contains("IEEE 754 bit pattern"),
        "the diagnostic must say why there is no answer, got {error}"
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
        let Harness { plan, store } = Harness::from_module(source, Some(module));
        let waits = vec![None; plan.processes.len()];
        Self { plan, store, waits }
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

    fn set_real(&mut self, name: &str, value: f64) {
        let id = self.signal(name);
        self.store.reals[usize::from(id)] = value;
    }

    fn get_real(&self, name: &str) -> f64 {
        self.store.reals[usize::from(self.signal(name))]
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
                    wait: None,
                },
            )
            .expect("a drive must apply");
        }
        // The real half of the same stand-in, held to the same one-driver rule
        // for the same reason (Verilog-AMS LRM 2.4 section 6.5.3).
        let real_drives: Vec<DigitalRealDrive> =
            self.store.driven_reals.values().cloned().collect();
        for drive in real_drives {
            let count = self.plan.drivers_of(drive.driver.signal).count();
            assert_eq!(
                count, 1,
                "multi-driver resolution belongs to the kernel; signal {:?} has {count} drivers",
                drive.driver.signal
            );
            self.store.reals[usize::from(drive.driver.signal)] = drive.value;
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
            let before = (self.store.values.clone(), self.store.reals.clone());
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
            if (self.store.values.clone(), self.store.reals.clone()) == before {
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

/// A child's `output real` port drives a parent `wreal` net.
///
/// The `output reg` idiom of IEEE 1364-2005 section 12.3.9.2, with section
/// 3.9's `real` as the variable type: the port cannot be *joined* with the net
/// it connects to, because a variable and a net are different things, so it
/// keeps its own signal and the connection becomes an ordinary driver on the
/// outer net. That is the same elaboration a four-state variable port gets —
/// nothing is special-cased for reals — and this is what says the
/// generalisation holds end to end: the child's process writes its own port,
/// the synthesized driver publishes it, and the parent's net carries it.
#[test]
fn a_child_output_real_port_drives_a_parent_real_net() {
    let source = "module scaler(clk, vin, vout);\n\
                  \x20   input clk;\n\
                  \x20   input wreal vin;\n\
                  \x20   output real vout;\n\
                  \x20   always @(posedge clk) vout = vin * 3.0;\n\
                  endmodule\n\
                  module top(clk, src, sink);\n\
                  \x20   input clk;\n\
                  \x20   input wreal src;\n\
                  \x20   output wreal sink;\n\
                  \x20   scaler u1(clk, src, sink);\n\
                  endmodule\n";
    let mut design = Design::new(source, "top");

    let port = design.signal("u1.vout");
    let sink = design.signal("sink");
    assert_ne!(port, sink, "a variable port cannot be joined with a net");
    assert!(
        design.plan.signal(port).expect("declared").kind.is_real(),
        "the child's port carries a real"
    );
    assert!(
        design
            .plan
            .signal(port)
            .expect("declared")
            .procedurally_assignable,
        "`output real` is a variable, so a process may write it"
    );
    assert_eq!(
        design.plan.drivers_of(sink).count(),
        1,
        "the connection is exactly one driver of the outer net"
    );

    design.set("clk", "0");
    design.set_real("src", 1.5);
    design.start_all();
    design.transition("clk", "1");
    assert_eq!(
        design.get_real("u1.vout"),
        4.5,
        "the port holds what it wrote"
    );
    assert_eq!(
        design.get_real("sink"),
        4.5,
        "and the driver carried it to the parent's net"
    );
}

// ===========================================================================
// Cross-domain reads: Verilog-AMS LRM 2.4 section 7.3
// ===========================================================================

/// Section 7.3.3, and its own worked example: "All continuous nets can be
/// probed from a discrete context using access functions." The clause's
/// `always @(posedge clk) out = V(in);` is the shape below, and the point of
/// running it is that the probe is *executed* — it reaches the environment,
/// which is the only thing that knows what the analog solver has settled.
///
/// Section 7.3's opening paragraph is what makes the direction one-way: "Read
/// operations of nets and variables in both domains are allowed from both
/// contexts. Write operations of nets and variables are only allowed from the
/// context of their domain." So there is a probe and no contribution, here or
/// anywhere in a process.
#[test]
fn a_process_samples_a_continuous_net_when_it_wakes() {
    let mut harness = Harness::new(
        "    wire clk;\n\
         \x20   reg hi;\n\
         \x20   always @(posedge clk) hi <= (V(p, n) > 0.5);",
    );

    assert_eq!(
        harness
            .plan
            .analog_probes
            .iter()
            .map(|probe| probe.spelling())
            .collect::<Vec<_>>(),
        vec!["V(p, n)".to_string()],
        "the probe is registered on the plan, by the author's own net names"
    );

    harness.set("clk", "0");
    harness.set_analog("V(p, n)", 0.25);
    let DigitalProcessOutcome::Suspended(suspension) = harness.start(0) else {
        panic!("the process must suspend on its event control");
    };
    let state = suspension.resume_state().clone();

    // Below the threshold at the first edge.
    harness.set("clk", "1");
    let outcome = harness.resume(0, &state);
    harness.flush_nonblocking();
    assert_eq!(harness.get("hi"), "0", "0.25 V is not above 0.5 V");
    let DigitalProcessOutcome::Suspended(suspension) = outcome else {
        panic!("the process must suspend again");
    };
    let state = suspension.resume_state().clone();

    // The probe is a leaf pinned to its block, so the second edge reads the
    // solution that exists *then* rather than the one the first edge read.
    // That is the whole reason it is not common-subexpressioned.
    harness.set("clk", "0");
    harness.set_analog("V(p, n)", 0.75);
    harness.set("clk", "1");
    harness.resume(0, &state);
    harness.flush_nonblocking();
    assert_eq!(
        harness.get("hi"),
        "1",
        "the second sample is the second solution, not the first"
    );
}

/// A probe the environment has no analog solution for is refused, not read as
/// zero volts.
///
/// The distinction matters because an unbound net and a grounded one are
/// different circuits: a process that computed against a fabricated 0 V would
/// produce a plausible waveform for a design nobody described, which is the
/// same failure `$bitstoreal` on an `x` refuses rather than guessing at.
#[test]
fn a_probe_with_no_analog_solution_is_refused_by_name() {
    let mut harness = Harness::new(
        "    wire clk;\n\
         \x20   reg hi;\n\
         \x20   always @(posedge clk) hi <= (V(p, n) > 0.5);",
    );
    harness.set("clk", "0");
    let DigitalProcessOutcome::Suspended(suspension) = harness.start(0) else {
        panic!("the process must suspend on its event control");
    };
    let state = suspension.resume_state().clone();
    harness.set("clk", "1");

    let error = resume(
        &harness.plan,
        &harness.plan.processes[0],
        &state,
        &mut harness.store,
    )
    .expect_err("an unsampled probe must refuse");
    assert!(
        matches!(error, DigitalEvalError::AnalogProbeUnavailable(_)),
        "unexpected error: {error}"
    );
}

// ===========================================================================
// A declared index is a name (IEEE 1364-2005 sections 3.3.1 and 4.2.1)
// ===========================================================================

/// Section 3.3.1: the *left* bound is the most significant bit, whatever the
/// two bounds are, so a declared index names a bit rather than counting from
/// the least significant end.
///
/// `reg [7:4] q` is four bits called 7, 6, 5 and 4. `q[7]` is its most
/// significant and `q[4]` its least — not bit 7 and bit 4 of a four-bit value,
/// which are respectively off the end and off the end. Every one of the four
/// reads came back `x` before the position rule existed.
#[test]
fn a_bit_select_names_the_bit_the_declaration_names() {
    let mut harness = Harness::new(
        "    reg [7:4] q;\n\
     \x20   reg [3:0] seen;\n\
     \x20   initial seen = {q[7], q[6], q[5], q[4]};",
    );
    harness.set("q", "10xz");

    expect_finished(harness.run());
    assert_eq!(harness.get("seen"), "10xz");
}

/// The same declaration written the other way round. Section 3.3.1 permits an
/// `lsb` greater than the `msb`, and the left bound is still the most
/// significant bit — so `[4:7]` names its top bit 4, and reading it in
/// declaration order gives back the value unchanged.
#[test]
fn an_ascending_declaration_still_names_its_leading_index_the_most_significant_bit() {
    let mut harness = Harness::new(
        "    reg [4:7] q;\n\
     \x20   reg [3:0] seen;\n\
     \x20   initial seen = {q[4], q[5], q[6], q[7]};",
    );
    harness.set("q", "10xz");

    expect_finished(harness.run());
    assert_eq!(harness.get("seen"), "10xz");
}

/// The ascending case that was already reachable before any of this, and was
/// already wrong: `[0:7]` names its most significant bit 0 and its least 7.
///
/// A kernel that reads a declared index as a position answers this test with
/// the value reversed, which is why it is written out bit by bit.
#[test]
fn a_zero_anchored_ascending_declaration_reads_msb_first() {
    let mut harness = Harness::new(
        "    reg [0:7] q;\n\
     \x20   reg [7:0] seen;\n\
     \x20   initial seen = {q[0], q[1], q[2], q[3], q[4], q[5], q[6], q[7]};",
    );
    harness.set("q", "10xz0011");

    expect_finished(harness.run());
    assert_eq!(harness.get("seen"), "10xz0011");
}

/// The declaration every design already used, unchanged: on `[7:0]` a declared
/// index and a position are the same number, which is why the defect stayed
/// invisible for as long as it did.
#[test]
fn a_descending_zero_anchored_declaration_is_the_identity() {
    let mut harness = Harness::new(
        "    reg [7:0] q;\n\
     \x20   reg [7:0] seen;\n\
     \x20   initial seen = {q[7], q[6], q[5], q[4], q[3], q[2], q[1], q[0]};",
    );
    harness.set("q", "10xz0011");

    expect_finished(harness.run());
    assert_eq!(harness.get("seen"), "10xz0011");
}

/// A one-bit vector is not a scalar: `reg [3:3] s` has one bit, and that bit is
/// called 3.
#[test]
fn a_single_bit_vector_names_its_one_bit_by_its_bound() {
    let mut harness = Harness::new(
        "    reg [3:3] s;\n\
     \x20   reg [1:0] seen;\n\
     \x20   initial seen = {s[3], s[3]};",
    );
    harness.set("s", "1");

    expect_finished(harness.run());
    assert_eq!(harness.get("seen"), "11");
}

#[test]
fn a_local_shadowing_a_parameter_is_not_folded_into_a_select() {
    let section = "parameter integer index = 0; reg [1:0] q;
         initial begin : work integer index; index = 1; q[index] = 1'b1; end";
    VerilogACompiler::default()
        .compile_canonical_ir(&digital_module(section))
        .expect_err("a runtime local must not be replaced by the shadowed parameter");
}

#[test]
fn digital_index_boundaries_do_not_panic_or_clamp() {
    for (section, diagnostic) in [
        ("reg q; initial q[-64'sh8000000000000000] = 1'b1;", "select"),
        (
            "parameter real idx = 1e30; reg q; initial q[idx] = 1'b1;",
            "select",
        ),
        (
            "parameter real idx = -1e30; reg q, seen; initial seen = q[idx];",
            "real value",
        ),
    ] {
        let error = VerilogACompiler::default()
            .compile_canonical_ir(&digital_module(section))
            .expect_err("unsupported index forms must fail explicitly");
        assert!(error.to_string().contains(diagnostic), "{section}: {error}");
    }
    // Negating the minimum signed 64-bit value wraps at its declared width.
    // Read indices now execute as digital expressions: the wrapped value
    // selects the minimum bound exactly, or yields X on a different range.
    for (bounds, expected) in [
        ("0:0", "x"),
        ("64'sh8000000000000000:64'sh8000000000000000", "1"),
    ] {
        let mut harness = Harness::new(&format!(
            "reg [{bounds}] q; reg seen; initial seen=q[-64'sh8000000000000000];"
        ));
        harness.set("q", "1");
        expect_finished(harness.run());
        assert_eq!(harness.get("seen"), expected);
    }
}

#[test]
fn digital_index_and_delay_boundary_values_execute_exactly() {
    let mut harness = Harness::new(
        "parameter real DELAY = 2147483647;
         reg [64'sh8000000000000000:64'sh8000000000000000] q;
         reg seen;
         initial begin
             q[64'sh8000000000000000] = 1'b1;
             seen = q[64'sh8000000000000000];
             #DELAY seen = 1'b0;
         end",
    );
    let DigitalProcessOutcome::Suspended(suspension) = harness.run() else {
        panic!("the boundary delay must suspend");
    };
    assert_eq!(harness.get("q"), "1");
    assert_eq!(harness.get("seen"), "1");
    assert_eq!(*suspension.wait(), DigitalWaitRequest::Delay(2147483647));
}

#[test]
fn parameter_delays_outside_the_executable_range_are_never_clamped() {
    for delay in ["9223372036854775808.0", "1e30", "-2147483649", "-1"] {
        let section = format!("parameter real DELAY = {delay}; reg q; initial #DELAY q = 1'b1;");
        let mut harness = Harness::new(&section);
        let error = start(
            &harness.plan,
            &harness.plan.processes[0],
            &mut harness.store,
        )
        .expect_err("an unrepresentable delay must fail before scheduling");
        assert!(
            matches!(error, DigitalEvalError::InvalidDelay { .. }),
            "{delay}: {error}"
        );
    }
}

#[test]
fn runtime_delays_sample_locals_signals_and_time_at_each_encounter() {
    // A delay expression is a read for @* even when its body writes a constant.
    for statement in ["#d q=1;", "q = #d 1;"] {
        let mut h = Harness::new(&format!("reg [7:0] d; reg q; always @* {statement}"));
        h.set("d", "00000001");
        let trigger = expect_suspended(h.run());
        let DigitalWaitRequest::Event(terms) = trigger.wait() else {
            panic!("implicit event wait")
        };
        assert_eq!(terms.len(), 1);
        assert_eq!(terms[0].signal, h.signal("d"));
        let delay = expect_suspended(h.resume(0, trigger.resume_state()));
        assert_eq!(*delay.wait(), DigitalWaitRequest::Delay(1));
    }
    use rspice_veriloga::canonical_ir::DigitalClock;
    let mut h = Harness::from_source(
        "`timescale 1ns/100ps\nmodule timed;
         parameter integer count=0; reg [7:0] bits; wreal fraction; reg [3:0] stage;
         initial begin : work
           integer count; count=1; stage=0;
           #count stage=1;
           #bits stage=2;
           #fraction stage=3;
           #(10.0-$realtime) stage=4;
           #(8'd255+8'd1) stage=5;
         end endmodule",
    );
    h.set("bits", "00000010");
    h.set_real("fraction", 0.15);
    h.store.clock = Some(DigitalClock {
        tick: 0,
        absolute_seconds: 0.0,
    });
    let first = expect_suspended(h.run());
    assert_eq!(
        *first.wait(),
        DigitalWaitRequest::Delay(10),
        "local shadows parameter"
    );
    h.set("bits", "00000011");
    h.store.clock = Some(DigitalClock {
        tick: 10,
        absolute_seconds: 1e-9,
    });
    let second = expect_suspended(h.resume(0, first.resume_state()));
    assert_eq!(*second.wait(), DigitalWaitRequest::Delay(30));
    h.set("bits", "00000111");
    h.set_real("fraction", 0.25);
    assert_eq!(
        *second.wait(),
        DigitalWaitRequest::Delay(30),
        "armed delay is captured"
    );
    h.store.clock = Some(DigitalClock {
        tick: 40,
        absolute_seconds: 4e-9,
    });
    let third = expect_suspended(h.resume(0, second.resume_state()));
    assert_eq!(
        *third.wait(),
        DigitalWaitRequest::Delay(3),
        "round at module precision"
    );
    h.store.clock = Some(DigitalClock {
        tick: 43,
        absolute_seconds: 4.3e-9,
    });
    let fourth = expect_suspended(h.resume(0, third.resume_state()));
    assert_eq!(*fourth.wait(), DigitalWaitRequest::Delay(57));
    h.store.clock = Some(DigitalClock {
        tick: 100,
        absolute_seconds: 10e-9,
    });
    let fifth = expect_suspended(h.resume(0, fourth.resume_state()));
    assert_eq!(
        *fifth.wait(),
        DigitalWaitRequest::Delay(0),
        "eight-bit sum wraps before delay conversion"
    );
    assert_eq!(h.get("stage"), "0100", "zero delay still suspends");
    expect_finished(h.resume(0, fifth.resume_state()));
    assert_eq!(h.get("stage"), "0101");
}

#[test]
fn runtime_delays_convert_unknown_signed_and_wide_values_without_clamping() {
    for (expression, ticks) in [
        ("8'bx0000001", 0),
        ("8'bz0000001", 0),
        ("8'hff", 255),
        ("129'd1", 1),
        ("64'd9007199254740992+1", 9007199254740993),
        ("-(129'sd18446744073709551616)", 0),
    ] {
        let mut h = Harness::new(&format!("reg q; initial #({expression}) q=1;"));
        assert_eq!(
            *expect_suspended(h.run()).wait(),
            DigitalWaitRequest::Delay(ticks),
            "{expression}"
        );
    }
    for expression in [
        "8'shff",
        "-1",
        "-1.0",
        "129'd18446744073709551617",
        "64'h8000000000000000",
    ] {
        let mut h = Harness::new(&format!("reg q; initial #({expression}) q=1;"));
        let error = start(&h.plan, &h.plan.processes[0], &mut h.store).unwrap_err();
        assert!(
            matches!(error, DigitalEvalError::InvalidDelay { .. }),
            "{expression}: {error}"
        );
        assert_eq!(h.get("q"), "x", "failed conversion must not resume");
    }
    for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
        let mut h = Harness::new("wreal d; reg q; initial #d q=1;");
        h.set_real("d", value);
        assert!(matches!(
            start(&h.plan, &h.plan.processes[0], &mut h.store),
            Err(DigitalEvalError::InvalidDelay { .. })
        ));
    }
}

#[test]
fn large_integer_index_expressions_select_the_exact_declared_bit() {
    let mut harness = Harness::new(
        "reg [9007199254740992+1:9007199254740992+1] q;
         reg seen;
         initial begin q[9007199254740992+1] = 1'b1; seen = q[9007199254740992+1]; end",
    );
    expect_finished(harness.run());
    assert_eq!(harness.get("q"), "1");
    assert_eq!(harness.get("seen"), "1");
}

/// Section 4.2.1: a part select runs in the declaration's direction, and takes
/// the bits it names.
#[test]
fn a_part_select_takes_the_bits_the_declaration_names() {
    let mut harness = Harness::new(
        "    reg [7:4] q;\n\
     \x20   reg [1:0] middle;\n\
     \x20   initial middle = q[6:5];",
    );
    harness.set("q", "10xz");

    expect_finished(harness.run());
    assert_eq!(harness.get("middle"), "0x");

    let mut harness = Harness::new(
        "    reg [4:7] q;\n\
     \x20   reg [1:0] middle;\n\
     \x20   initial middle = q[5:6];",
    );
    harness.set("q", "10xz");

    expect_finished(harness.run());
    assert_eq!(harness.get("middle"), "0x");

    // The whole of a non-zero-anchored vector, named by its own bounds.
    let mut harness = Harness::new(
        "    reg [7:4] q;\n\
     \x20   reg [3:0] all;\n\
     \x20   initial all = q[7:4];",
    );
    harness.set("q", "10xz");

    expect_finished(harness.run());
    assert_eq!(harness.get("all"), "10xz");
}

/// The brief's own example: a concatenation of a part select of a
/// non-zero-anchored vector and one of an ordinary vector.
#[test]
fn a_concatenation_of_two_declarations_takes_each_ones_named_bits() {
    let mut harness = Harness::new(
        "    reg [7:4] x;\n\
     \x20   reg [3:0] y;\n\
     \x20   reg [7:0] both;\n\
     \x20   initial both = {x[7:4], y[3:0]};",
    );
    harness.set("x", "1010");
    harness.set("y", "0011");

    expect_finished(harness.run());
    assert_eq!(harness.get("both"), "10100011");
}

/// A write names bits the same way a read does, and leaves the rest alone.
#[test]
fn a_partial_write_names_its_bits_the_way_the_declaration_does() {
    let mut harness = Harness::new(
        "    reg [7:4] q;\n\
     \x20   initial begin\n\
     \x20      q[7] = 1'b1;\n\
     \x20      q[6:5] = 2'b01;\n\
     \x20   end",
    );
    harness.set("q", "0000");

    expect_finished(harness.run());
    assert_eq!(harness.get("q"), "1010");

    let mut harness = Harness::new(
        "    reg [4:7] q;\n\
     \x20   initial begin\n\
     \x20      q[4] = 1'b1;\n\
     \x20      q[5:6] = 2'b01;\n\
     \x20   end",
    );
    harness.set("q", "0000");

    expect_finished(harness.run());
    assert_eq!(harness.get("q"), "1010");
}

/// An `integer`'s bits are numbered [31:0] by section 3.9, so its top bit is
/// selectable by name and is the sign bit rather than a read off the end.
#[test]
fn an_integer_numbers_its_bits_thirty_one_down_to_zero() {
    let mut harness = Harness::new(
        "    reg [1:0] seen;\n\
     \x20   initial begin: body\n\
     \x20      integer i;\n\
     \x20      i = 1;\n\
     \x20      seen = {i[31], i[0]};\n\
     \x20   end",
    );

    expect_finished(harness.run());
    assert_eq!(harness.get("seen"), "01");
}

/// A process-local `reg` carries its own declaration, so a select on one is
/// resolved against the bounds it was written with rather than against a bare
/// width.
#[test]
fn a_process_local_reg_names_its_bits_by_its_own_declaration() {
    let mut harness = Harness::new(
        "    reg [3:0] seen;\n\
     \x20   initial begin: body\n\
     \x20      reg [7:4] t;\n\
     \x20      t = 4'b1001;\n\
     \x20      seen = {t[7], t[6], t[5], t[4]};\n\
     \x20   end",
    );

    expect_finished(harness.run());
    assert_eq!(harness.get("seen"), "1001");
}

#[test]
fn computed_event_expressions_track_result_edges_and_dynamic_indices() {
    let mut h = Harness::from_source(
        "module computed; reg a,b,q; reg [2:0] bus; reg [1:0] index;
         initial begin a=0; b=0; q=0; bus=0; index=0;
           @(posedge (a & b) or bus[index]) q=1;
         end endmodule",
    );
    let transported: CanonicalDigitalPlan =
        serde_json::from_str(&serde_json::to_string(&h.plan).unwrap()).unwrap();
    transported.validate().unwrap();
    assert_eq!(transported, h.plan);
    let DigitalProcessOutcome::Suspended(suspension) = h.run() else {
        panic!("must wait");
    };
    let (DigitalWaitRequest::Expressions(mut wait), resume) = suspension.into_parts() else {
        panic!("computed wait");
    };
    let mut scratch = rspice_veriloga::canonical_ir::digital_eval::DigitalEvalScratch::new();
    for (name, bits, expected) in [
        ("b", "1", false),
        ("bus", "010", false),
        ("index", "01", true),
    ] {
        h.set(name, bits);
        let signal = h.signal(name);
        assert_eq!(
            wait.observe(&h.plan, signal, &mut h.store, &mut scratch)
                .unwrap(),
            expected,
            "{name}"
        );
    }
    expect_finished(h.resume(0, &resume));
    assert_eq!(h.get("q"), "1");

    // Real results have value-change events; four-state comparisons of real
    // operands have ordinary bit edges. Repeated references to one input are legal.
    for event in ["a+a", "posedge (a > 0.5)", "posedge b or negedge b", "1'b0"] {
        let source = format!(
            "module events; real a; reg b,q; initial begin a=0.0; q=0; @({event}) q=1; end endmodule"
        );
        VerilogACompiler::default()
            .compile_canonical_ir_module(&source, None)
            .unwrap();
    }
    let source = "module local_event; initial begin : scope real saved; saved=0.0; @($realtobits(saved)); end endmodule";
    let error = VerilogACompiler::default()
        .compile_canonical_ir_module(source, None)
        .unwrap_err();
    assert!(error.to_string().contains("process-local"), "{error}");
}

#[test]
fn computed_event_dependencies_use_exact_runtime_bit_indices() {
    let mut h = Harness::from_source(
        "module bits; reg [-2:1] bus; reg signed [95:0] index; reg q; initial q=bus[index]; endmodule",
    );
    h.set("bus", "1001");
    for (index, expected) in [
        (format!("{}10", "1".repeat(94)), "1"), // -2, above 64 bits
        ("1".repeat(96), "0"),                  // -1
        (format!("{}1", "0".repeat(95)), "1"),
        ("0".repeat(96), "0"),
        (format!("{}10", "0".repeat(94)), "x"), // beyond upper declared index
        (format!("01{}", "0".repeat(94)), "x"), // large known positive index
        ("x".repeat(96), "x"),
        ("z".repeat(96), "x"),
    ] {
        h.set("index", &index);
        expect_finished(h.run());
        assert_eq!(h.get("q"), expected, "index {index}");
    }
}

#[test]
fn repeat_controls_normalize_counts_and_bypass_zero_event_evaluation() {
    let mut invalid = Harness::from_source(
        "module invalid_count; wreal count; reg q; initial q <= repeat(count) @(1'b0) 1; endmodule",
    );
    for count in [f64::NAN, f64::INFINITY, 3e10] {
        invalid.set_real("count", count);
        assert!(matches!(
            start(
                &invalid.plan,
                &invalid.plan.processes[0],
                &mut invalid.store
            ),
            Err(DigitalEvalError::InvalidRepeatCount { .. })
        ));
        assert_eq!(invalid.deferred_count(), 0);
    }

    for (count, expected) in [
        ("-1", 0),
        ("-2'd1", 3),
        ("2'b1x", 0),
        ("2'b1z", 0),
        ("0", 0),
        ("2.5", 3),
        ("-0.5", 0),
        ("3", 3),
    ] {
        let mut h = Harness::from_source(&format!(
            "module counts; reg [7:0] n; initial begin n=0; repeat ({count}) n=n+1; end endmodule"
        ));
        expect_finished(h.run());
        assert_eq!(h.get("n"), format!("{expected:08b}"), "count {count}");
        if expected == 0 {
            let mut h = Harness::from_source(&format!(
                "module skip; reg q,n,seen; initial begin q=0; n=0;
                 q = repeat ({count}) @($bitstoreal(64'bx)) 1;
                 n <= repeat ({count}) @($bitstoreal(64'bx)) 1;
                 seen=q & ~n; end endmodule"
            ));
            expect_finished(h.run());
            assert_eq!(h.get("seen"), "1", "blocking immediate; NBA pending");
            h.flush_nonblocking();
            assert_eq!(h.get("n"), "1");
        }
    }
}

#[test]
fn repeat_controls_capture_wide_counts_data_and_implicit_dependencies_once() {
    let mut generated = Harness::from_source(
        "module generated; reg clk; reg [1:0] q; genvar i;
         generate for(i=0;i<2;i=i+1) begin: g
           initial q[i] <= repeat(i+1) @(posedge clk) 1'b1;
         end endgenerate endmodule",
    );
    for index in 0..2 {
        expect_finished(generated.start(index));
    }
    assert_eq!(
        generated
            .store
            .deferred
            .iter()
            .map(|update| match &update.wait {
                Some(DigitalWaitRequest::Repeated { count, .. }) =>
                    count.remaining().to_u64().unwrap(),
                _ => panic!("generated repeat count"),
            })
            .collect::<Vec<_>>(),
        [1, 2]
    );

    let mut h = Harness::from_source(
        "module capture; reg [95:0] n; reg [7:0] data,q,implicit_q; reg clk,done;
         initial begin data=8'h42; done=0;
           q <= repeat (n) @(posedge clk) data;
           implicit_q <= repeat (2) @* data;
           n=0; data=8'h99; done=1;
         end endmodule",
    );
    h.set("n", &format!("{}1{}", "0".repeat(31), "0".repeat(64)));
    let transported: CanonicalDigitalPlan =
        serde_json::from_str(&serde_json::to_string(&h.plan).unwrap()).unwrap();
    transported.validate().unwrap();
    assert_eq!(transported, h.plan);
    expect_finished(h.run());
    assert_eq!(h.get("done"), "1");
    assert_eq!(h.deferred_count(), 2);
    let Some(DigitalWaitRequest::Repeated { count, event }) = &h.store.deferred[0].wait else {
        panic!("repeated event");
    };
    assert_eq!(count.remaining().width(), 96);
    let mut count = count.clone();
    assert!(!count.consume());
    assert_eq!(
        count.remaining().spelling(),
        format!("{}{}", "0".repeat(32), "1".repeat(64))
    );
    assert!(matches!(event.as_ref(), DigitalWaitRequest::Event(_)));
    assert_eq!(
        h.store.deferred[0].value,
        DigitalUpdate::FourState(FourStateValue::from_u64(8, 0x42))
    );
    let Some(DigitalWaitRequest::Repeated { count, event }) = &h.store.deferred[1].wait else {
        panic!("repeated implicit event");
    };
    assert_eq!(count.remaining().to_u64(), Some(2));
    let DigitalWaitRequest::Event(terms) = event.as_ref() else {
        panic!("implicit dependencies");
    };
    assert_eq!(
        terms.iter().map(|term| term.signal).collect::<Vec<_>>(),
        [h.signal("data")]
    );
}

/// VAMS-2023 4.2.1.1–3: convert numbers, preserving integral subexpressions.
#[test]
fn mixed_numeric_conversions_preserve_sign_width_rounding_and_clock_values() {
    use rspice_veriloga::canonical_ir::DigitalClock;
    let mut harness = Harness::from_source(
        r#"
module numeric;
    reg signed [7:0] s;
    reg [7:0] u, up, down, wrapped;
    reg [95:0] wide, wide_round;
    reg signed [95:0] wide_signed;
    reg [3:0] nibble;
    reg [63:0] pattern;
    reg ok;
    real a,b,c,d,signed_mix,unsigned_mix,wide_real,negative_wide,own_width,local_real,selected,rem,power;
    initial begin : work
        integer i;
        real local_value;
        i=-3; s=-2; u=254; wide=96'h000100000000000000000001;
        wide_signed=-3; negative_wide=wide_signed; nibble=15; own_width=(nibble+4'd1)+0.0;
        a=3+5.0; b=1/2; c=8.0+(1/2); d=1/2.0;
        signed_mix=s+0.5; unsigned_mix=u+0.5; wide_real=wide;
        local_value=i; local_real=local_value;
        selected=1 ? 2 : 1.25;
        rem=-10.5%3; power=2.0**3;
        up=35.5; down=-1.5; wrapped=255.5;
        wide_round=1208925819614629174706176.0;
        pattern=$realtobits(1);
        ok=($realtime>0 && 0<$realtime && s<0.0 && u>0.0 && !0.0 && (1.0 || 1'bx) && !(0.0 && 1'bx));
    end
endmodule
"#,
    );
    // Serialization carries both conversions and their width/sign contracts.
    harness.plan = serde_json::from_str(&serde_json::to_string(&harness.plan).unwrap()).unwrap();
    harness.plan.validate().unwrap();
    harness.store.clock = Some(DigitalClock {
        tick: 1,
        absolute_seconds: 1e-9,
    });
    assert!(matches!(harness.start(0), DigitalProcessOutcome::Finished));
    for (name, expected) in [
        ("a", 8.0),
        ("b", 0.0),
        ("c", 8.0),
        ("d", 0.5),
        ("signed_mix", -1.5),
        ("unsigned_mix", 254.5),
        ("wide_real", 2.0_f64.powi(80)),
        ("negative_wide", -3.0),
        ("own_width", 0.0),
        ("local_real", -3.0),
        ("selected", 2.0),
        ("rem", -1.5),
        ("power", 8.0),
    ] {
        assert_eq!(harness.get_real(name), expected, "{name}");
    }
    assert_eq!(harness.get("up"), "00100100");
    assert_eq!(harness.get("down"), "11111110");
    assert_eq!(harness.get("wrapped"), "00000000");
    assert_eq!(harness.get("wide_round"), format!("{:096b}", 1_u128 << 80));
    assert_eq!(
        harness.get("pattern"),
        format!("{:064b}", 1.0_f64.to_bits())
    );
    assert_eq!(harness.get("ok"), "1");
}

#[test]
fn numeric_conversion_reports_unknown_bits_and_nonfinite_integer_inputs() {
    for digit in ["x", "z"] {
        let mut harness = Harness::from_source(
            "module unknown; reg [95:0] data; real r; initial r=data+1.0; endmodule",
        );
        harness.set("data", &format!("{digit}{}", "0".repeat(95)));
        let error = start(
            &harness.plan,
            &harness.plan.processes[0],
            &mut harness.store,
        )
        .unwrap_err();
        assert!(
            matches!(error, DigitalEvalError::InvalidNumericConversion { .. }),
            "{error}"
        );
        assert!(error.to_string().contains("X/Z"));
    }
    let mut harness = Harness::from_source(
        "module invalid; reg [7:0] q; real r; initial begin r=1.0/0.0; q=r; end endmodule",
    );
    let error = start(
        &harness.plan,
        &harness.plan.processes[0],
        &mut harness.store,
    )
    .unwrap_err();
    assert!(
        matches!(error, DigitalEvalError::InvalidNumericConversion { .. }),
        "{error}"
    );
    assert!(error.to_string().contains("finite"));
}

#[test]
fn numeric_conversions_validate_their_value_domains_and_target_widths() {
    use rspice_veriloga::canonical_ir::{CfgValueKind, CfgValueType};
    let harness = Harness::from_source(
        "module shape; real r; reg [7:0] q; initial begin r=3; q=r; end endmodule",
    );
    for real_to_int in [false, true] {
        let mut plan = harness.plan.clone();
        let value = plan.processes[0]
            .function
            .values
            .iter_mut()
            .find(|v| match v.kind {
                CfgValueKind::DigitalRealToInteger { .. } => real_to_int,
                CfgValueKind::DigitalIntegerToReal { .. } => !real_to_int,
                _ => false,
            })
            .unwrap();
        value.value_type = if real_to_int {
            CfgValueType::Real
        } else {
            CfgValueType::FourState { width: 8 }
        };
        let error = format!("{:?}", plan.validate().unwrap_err());
        let expected = if real_to_int {
            "real-to-integer conversion"
        } else {
            "integer-to-real conversion"
        };
        assert!(
            error.contains(expected),
            "the structural check must precede identity validation: {error}"
        );
    }
    let mut plan = harness.plan.clone();
    for value in &mut plan.processes[0].function.values {
        if let CfgValueKind::DigitalRealToInteger { width, .. } = &mut value.kind {
            *width = 9;
        }
    }
    let error = format!("{:?}", plan.validate().unwrap_err());
    assert!(error.contains("real-to-integer conversion"), "{error}");
}

#[test]
fn real_conditionals_skip_inactive_reads_and_preserve_statement_flow() {
    let mut h = Harness::from_source(
        r#"
      module lazy(p); inout p; electrical p;
        reg select; reg [95:0] poison; real result, captured, deferred, after;
        initial begin : scope
          real local;
          local=0.0;
          result=(select ? 5.0 : V(p)) + (select ? 1.0 : $bitstoreal(64'bx))
                 + (select ? 2.0 : poison);
          while (select ? (local < 3.0 ? 1.0 : 0.0) : poison)
            local=select ? local+1.0 : poison;
          deferred <= select ? local : poison;
          captured = #2 (select ? local : poison);
          after = select ? V(p) : poison;
        end
      endmodule
    "#,
    );
    h.set("select", "1");
    let suspended = expect_suspended(h.start(0));
    assert_eq!(h.get_real("result"), 8.0);
    h.set("select", "0");
    h.set("poison", &format!("{:096b}", 9));
    h.flush_nonblocking();
    assert_eq!(h.get_real("deferred"), 3.0);
    expect_finished(h.resume(0, suspended.resume_state()));
    assert_eq!(h.get_real("captured"), 3.0);
    assert_eq!(h.get_real("after"), 9.0);

    // A continuous driver must re-enter the condition's entry, not reuse the
    // selected arm or cached values from the preceding activation.
    let mut driver = Harness::from_source(
        "module mux; reg select; reg [95:0] poison; wreal y; assign y=select ? 2.5 : poison; endmodule",
    );
    driver.set("select", "1");
    let wait = expect_suspended(driver.start(0));
    assert_eq!(
        driver.store.driven_reals.values().next().unwrap().value,
        2.5
    );
    driver.set("select", "0");
    driver.set("poison", &format!("{:096b}", 7));
    expect_suspended(driver.resume(0, wait.resume_state()));
    assert_eq!(
        driver.store.driven_reals.values().next().unwrap().value,
        7.0
    );
}

#[test]
fn real_conditionals_apply_ambiguous_rule_and_report_selected_errors() {
    for condition in ["x", "z"] {
        let mut h = Harness::from_source(
            "module ambiguous; reg c; real different,same; initial begin different=c ? 2.5 : 9.0; same=c ? 2.5 : 2.5; end endmodule",
        );
        h.set("c", condition);
        expect_finished(h.start(0));
        assert_eq!(h.get_real("different"), 0.0);
        assert_eq!(h.get_real("same"), 0.0);
    }
    for (expression, condition) in [
        ("c ? 2.5 : poison", "0"),
        ("c ? poison : 2.5", "1"),
        ("c ? poison : 2.5", "x"),
        ("c ? 2.5 : poison", "z"),
    ] {
        let mut h = Harness::from_source(&format!(
            "module selected; reg c; reg [95:0] poison; real r; initial r={expression}; endmodule"
        ));
        h.set("c", condition);
        assert!(
            matches!(
                start(&h.plan, &h.plan.processes[0], &mut h.store),
                Err(DigitalEvalError::InvalidNumericConversion { .. })
            ),
            "{expression}, {condition}"
        );
    }
}

#[test]
fn computed_event_real_conditionals_observe_only_selected_values() {
    let mut h = Harness::from_source(
        "module events; reg select,q; reg [95:0] data; real a;
         initial begin a=2.5; q=0; @(select ? a : data) q=1; end endmodule",
    );
    h.set("select", "1");
    let transported: CanonicalDigitalPlan =
        serde_json::from_str(&serde_json::to_string(&h.plan).unwrap()).unwrap();
    transported.validate().unwrap();
    h.plan = transported;
    let suspension = expect_suspended(h.start(0));
    let (DigitalWaitRequest::Expressions(mut wait), resume) = suspension.into_parts() else {
        panic!("computed wait")
    };
    let mut scratch = rspice_veriloga::canonical_ir::digital_eval::DigitalEvalScratch::new();
    // Changing the unused operand, including X/Z, leaves the event baseline.
    h.set("data", &"z".repeat(96));
    let data = h.signal("data");
    assert!(
        !wait
            .observe(&h.plan, data, &mut h.store, &mut scratch)
            .unwrap()
    );
    h.set("data", &format!("{:096b}", 7));
    assert!(
        !wait
            .observe(&h.plan, data, &mut h.store, &mut scratch)
            .unwrap()
    );
    h.set("select", "0");
    let select = h.signal("select");
    assert!(
        wait.observe(&h.plan, select, &mut h.store, &mut scratch)
            .unwrap()
    );
    h.set_real("a", 8.5);
    let a = h.signal("a");
    assert!(
        !wait
            .observe(&h.plan, a, &mut h.store, &mut scratch)
            .unwrap()
    );
    h.set("select", "1");
    assert!(
        wait.observe(&h.plan, select, &mut h.store, &mut scratch)
            .unwrap()
    );
    expect_finished(h.resume(0, &resume));
    assert_eq!(h.get("q"), "1");
}

#[test]
fn computed_event_programs_reject_invalid_nested_artifacts() {
    use rspice_veriloga::canonical_ir::{CfgTerminator, CfgValueKind, CfgValueType};
    let h = Harness::from_source(
        "module validation; reg c; reg [7:0] data; real a; initial begin a=2.5; @(c ? a : data); end endmodule",
    );
    for (case, expected) in [
        (0, "absent result"),
        (1, "undeclared signal"),
        (2, "must be pure"),
        (3, "must be acyclic"),
        (4, "every return path"),
        (5, "four-state condition"),
        (6, "must be pure"),
    ] {
        let mut plan = h.plan.clone();
        let node = plan.processes[0]
            .function
            .values
            .iter_mut()
            .find(|v| matches!(v.kind, CfgValueKind::DigitalExpression { .. }))
            .unwrap();
        let nested = node.kind.clone();
        let CfgValueKind::DigitalExpression { function, result } = &mut node.kind else {
            unreachable!()
        };
        match case {
            0 => *result = 999999usize.into(),
            1 => {
                let value = function
                    .values
                    .iter_mut()
                    .find(|v| matches!(v.kind, CfgValueKind::DigitalRealSignalRead { .. }))
                    .unwrap();
                value.kind = CfgValueKind::DigitalRealSignalRead {
                    signal: 999999usize.into(),
                };
            }
            2 => {
                let value = function
                    .values
                    .iter_mut()
                    .find(|v| matches!(v.kind, CfgValueKind::DigitalRealSignalRead { .. }))
                    .unwrap();
                value.kind = nested;
            }
            3 => {
                function.blocks[usize::from(function.entry)].terminator = CfgTerminator::Jump {
                    target: function.entry,
                    args: Vec::new(),
                }
            }
            4 => {
                *result = function
                    .values
                    .iter()
                    .find(|v| matches!(v.kind, CfgValueKind::DigitalIntegerToReal { .. }))
                    .unwrap()
                    .id
            }
            5 => {
                let real = function
                    .values
                    .iter()
                    .find(|v| matches!(v.kind, CfgValueKind::RealConstant(_)))
                    .unwrap()
                    .id;
                let CfgTerminator::Branch { condition, .. } =
                    &mut function.blocks[usize::from(function.entry)].terminator
                else {
                    panic!("branch")
                };
                *condition = real;
            }
            6 => {
                let value = function
                    .values
                    .iter_mut()
                    .find(|v| matches!(v.kind, CfgValueKind::DigitalRealSignalRead { .. }))
                    .unwrap();
                value.value_type = CfgValueType::Effect;
            }
            _ => unreachable!(),
        }
        let error = format!("{:?}", plan.validate().unwrap_err());
        assert!(error.contains(expected), "case {case}: {error}");
    }
}

#[test]
fn four_state_conditionals_skip_inactive_real_reads_and_capture_before_wait() {
    let mut h = Harness::from_source(
        r#"
      module lazy_bits(p); inout p; electrical p;
        reg select; reg [95:0] poison;
        reg [63:0] q,converted,held,after;
        initial begin
          q=select ? 64'h55 : $realtobits(V(p));
          converted=select ? $realtobits(2.5) : $realtobits(poison);
          held=#1 (select ? 64'd7 : $realtobits(poison));
          after=select ? $realtobits(poison) : $realtobits(V(p));
        end
      endmodule
    "#,
    );
    h.set("select", "1");
    let wait = expect_suspended(h.start(0));
    assert_eq!(h.get("q"), format!("{:064b}", 0x55));
    assert_eq!(h.get("converted"), format!("{:064b}", 2.5f64.to_bits()));
    h.set("select", "0");
    h.set_analog("V(p)", 3.5);
    expect_finished(h.resume(0, wait.resume_state()));
    assert_eq!(h.get("held"), format!("{:064b}", 7));
    assert_eq!(h.get("after"), format!("{:064b}", 3.5f64.to_bits()));
    // The same unavailable input must still fail when selected, and both
    // arms are evaluated when the condition is ambiguous.
    for condition in ["1", "x", "z"] {
        let mut selected = Harness::from_source(
            "module selected(p); inout p; electrical p; reg c; reg [63:0] q; initial q=c ? $realtobits(V(p)) : 64'h55; endmodule",
        );
        selected.set("c", condition);
        assert!(
            matches!(
                start(
                    &selected.plan,
                    &selected.plan.processes[0],
                    &mut selected.store
                ),
                Err(DigitalEvalError::AnalogProbeUnavailable(_))
            ),
            "{condition}"
        );
    }
}

#[test]
fn four_state_conditionals_preserve_wide_signed_merges_and_nested_control_flow() {
    let mut h = Harness::from_source(
        "module widths; reg c; reg signed [3:0] a; reg signed [7:0] b; reg signed [95:0] q;
       initial q=c ? a : (1 ? b : a); endmodule",
    );
    h.set("a", "1000");
    h.set("b", "11111110");
    for (condition, expected) in [
        ("1", format!("{}1000", "1".repeat(92))),
        ("0", format!("{}1110", "1".repeat(92))),
        ("x", format!("{}1xx0", "1".repeat(92))),
        ("z", format!("{}1xx0", "1".repeat(92))),
    ] {
        h.set("c", condition);
        expect_finished(h.start(0));
        assert_eq!(h.get("q"), expected, "{condition}");
    }
}

#[test]
fn digital_flow_probes_sample_named_solver_currents_and_validate_quantity() {
    use rspice_veriloga::canonical_ir::{CfgValueKind, CfgValueType};
    let mut harness = Harness::from_source(
        r#"
module samples(p,n); inout p,n; electrical p,n; branch(p,n) supply;
real amperes, volts, total;
analog V(supply)<+2;
initial begin amperes=I(supply); volts=V(supply); total=I(<p>); #1 amperes=I(<supply>); end
endmodule
"#,
    );
    let missing = start(
        &harness.plan,
        &harness.plan.processes[0],
        &mut harness.store,
    );
    assert!(matches!(
        missing,
        Err(DigitalEvalError::AnalogProbeUnavailable(_))
    ));
    harness.set_analog("I(<supply>)", -0.002);
    harness.set_analog("V(<supply>)", 2.0);
    let wait = expect_suspended(harness.start(0));
    assert_eq!(harness.get_real("amperes"), -0.002);
    assert_eq!(harness.get_real("volts"), 2.0);
    assert_eq!(harness.get_real("total"), -0.002);
    harness.set_analog("I(<supply>)", -0.004);
    assert!(matches!(
        harness.resume(0, wait.resume_state()),
        DigitalProcessOutcome::Finished
    ));
    assert_eq!(harness.get_real("amperes"), -0.004);
    for wrong_type in [false, true] {
        let mut malformed = harness.plan.clone();
        let read = malformed.processes[0]
            .function
            .values
            .iter_mut()
            .find(|v| matches!(v.kind, CfgValueKind::DigitalAnalogFlow { .. }))
            .unwrap();
        if wrong_type {
            read.value_type = CfgValueType::FourState { width: 1 };
        } else if let CfgValueKind::DigitalAnalogFlow { probe } = read.kind {
            read.kind = CfgValueKind::DigitalAnalogPotential { probe };
        }
        assert!(malformed.validate().is_err());
    }
}

#[test]
fn digital_flow_probes_resolve_custom_nature_roles() {
    use rspice_veriloga::canonical_ir::digital::DigitalAnalogQuantity as AccessKind;
    let mut harness = Harness::from_source(
        r#"
nature TestPotential units="V"; access=TestU; abstol=1e-6; endnature
nature TestFlow units="A"; access=TestQ; abstol=1e-12; endnature
discipline custom potential TestPotential; flow TestFlow; enddiscipline
module samples(p,n); inout p,n; custom p,n; branch(p,n) source;
real amperes, volts;
analog TestU(source)<+2;
initial begin amperes=TestQ(source); volts=TestU(p,n); end
endmodule
"#,
    );
    assert_eq!(harness.plan.analog_probes[0].quantity, AccessKind::Flow);
    assert_eq!(
        harness.plan.analog_probes[1].quantity,
        AccessKind::Potential
    );
    harness.set_analog("TestQ(<source>)", -0.003);
    harness.set_analog("TestU(p, n)", 2.0);
    assert!(matches!(harness.start(0), DigitalProcessOutcome::Finished));
    assert_eq!(harness.get_real("amperes"), -0.003);
    assert_eq!(harness.get_real("volts"), 2.0);
}

#[test]
fn digital_flow_probes_join_simultaneous_equations_and_refuse_unsupported_events() {
    use rspice_veriloga::canonical_ir::CfgValueKind;
    let source = r#"
module samples(p,n); inout p,n; electrical p,n; branch(p,n) a,b;
real x,y,total;
analog begin I(a)<+V(p,n)*0.001+ddt(1e-12*V(p,n)); I(b)<+0.002; end
initial begin #1; x=I(a); y=I(b); total=I(<p>); end
endmodule
"#;
    let compiler = VerilogACompiler::default();
    let artifact = compiler.compile_canonical_ir_module(source, None).unwrap();
    assert_eq!(
        artifact
            .hir
            .internal_nodes
            .iter()
            .filter(|n| n.is_state)
            .count(),
        2
    );
    assert!(
        artifact.digital.processes[0]
            .function
            .values
            .iter()
            .all(|v| !matches!(v.kind, CfgValueKind::DigitalAnalogFlow { .. })),
        "current-source reads must use the simultaneous mathematical states"
    );
    for source in [
        "module bad(p,n); inout p,n; electrical p,n; branch(p,n) b; analog V(b)<+1; initial begin real b; b=I(b); end endmodule",
        "module bad(p,n); inout p,n; electrical p,n; reg q; analog V(p,n)<+1; initial @(I(p,n)) q=1; endmodule",
        "module bad(p,n); inout p,n; electrical p,n; reg q; analog I(p,n)<+1; initial @(I(p,n)) q=1; endmodule",
    ] {
        assert!(
            compiler.compile_canonical_ir_module(source, None).is_err(),
            "{source}"
        );
    }
}

#[test]
fn analog_variable_reads_retain_real_values_signed_width_and_resampling() {
    let mut harness = Harness::from_source(
        r#"
module variable_reader(p); inout p; electrical p;
real measured, captured; integer count; reg [63:0] wide; reg negative;
analog begin measured=2*V(p); count=-3; I(p)<+V(p)/1000; end
initial begin captured=measured; wide=count; negative=(count<0); #1 captured=measured; end
endmodule
"#,
    );
    let unavailable = expect_suspended(harness.start(0));
    assert_eq!(
        unavailable.wait(),
        &DigitalWaitRequest::AnalogSample(harness.probe("measured"))
    );
    harness.set_analog("measured", 2.25);
    harness.set_analog("count", -3.0);
    let wait = expect_suspended(harness.resume(0, unavailable.resume_state()));
    assert_eq!(harness.get_real("captured"), 2.25);
    assert_eq!(harness.get("wide"), format!("{:064b}", u64::MAX - 2));
    assert_eq!(harness.get("negative"), "1");
    harness.set_analog("measured", 4.5);
    expect_finished(harness.resume(0, wait.resume_state()));
    assert_eq!(harness.get_real("captured"), 4.5);
    harness.set_analog("count", -3.5);
    assert!(matches!(
        start(
            &harness.plan,
            &harness.plan.processes[0],
            &mut harness.store
        ),
        Err(DigitalEvalError::InvalidNumericConversion { .. })
    ));
}

#[test]
fn analog_sample_barriers_preserve_expression_operands_and_prefix_effects() {
    let mut harness = Harness::from_source(
        r#"
module barrier(p); inout p; electrical p;
real a,b,captured,bias; reg choose; reg [7:0] prefix,deferred;
analog begin a=V(p); b=2*V(p); I(p)<+V(p)/1000; end
initial begin : work
  integer local;
  prefix=prefix+1; bias=2; local=7; deferred<=19;
  captured=bias+(choose ? a+b : 9.0)+local;
  #1; captured=captured+a;
end
endmodule
"#,
    );
    harness.set("prefix", "00000000");
    harness.set("choose", "1");
    harness.set_real("bias", 2.0);
    let first = expect_suspended(harness.start(0));
    assert_eq!(
        first.wait(),
        &DigitalWaitRequest::AnalogSample(harness.probe("a"))
    );
    assert!(first.resume_state().analog_instruction().is_some());
    assert_eq!(harness.get("prefix"), "00000001");
    assert_eq!(harness.deferred_count(), 1);

    // Other work at the barrier may change a signal. Already-read operands
    // and the chosen branch still belong to the suspended expression.
    harness.set("prefix", "01100100");
    harness.set("choose", "0");
    harness.set_real("bias", 100.0);
    harness.set_analog("a", 3.0);
    let second = expect_suspended(harness.resume(0, first.resume_state()));
    assert_eq!(
        second.wait(),
        &DigitalWaitRequest::AnalogSample(harness.probe("b"))
    );
    let still_waiting = expect_suspended(harness.resume(0, second.resume_state()));
    assert_eq!(second, still_waiting);
    harness.set_analog("a", 100.0);
    harness.set_analog("b", 4.0);
    let delay = expect_suspended(harness.resume(0, still_waiting.resume_state()));
    assert!(matches!(delay.wait(), DigitalWaitRequest::Delay(_)));
    assert_eq!(harness.get_real("captured"), 16.0);
    assert_eq!(harness.get("prefix"), "01100100");
    assert_eq!(harness.deferred_count(), 1);
    harness.flush_nonblocking();
    assert_eq!(harness.get("deferred"), "00010011");
    harness.set_analog("a", 5.0);
    expect_finished(harness.resume(0, delay.resume_state()));
    assert_eq!(harness.get_real("captured"), 21.0);
}

#[test]
fn analog_sample_barriers_preserve_loop_locals_and_skip_untaken_reads() {
    let mut harness = Harness::from_source(
        r#"
module loops(p); inout p; electrical p;
real sample,total; reg choose;
analog begin sample=V(p); I(p)<+V(p)/1000; end
initial begin : work
  integer i; total=0;
  for (i=0; i<3; i=i+1) begin
    total=total+(choose ? sample : 10.0)+i;
    #0;
  end
end
endmodule
"#,
    );
    harness.set("choose", "0");
    let first_delay = expect_suspended(harness.start(0));
    assert_eq!(first_delay.wait(), &DigitalWaitRequest::Delay(0));
    assert_eq!(harness.get_real("total"), 10.0);
    harness.set("choose", "1");
    let read = expect_suspended(harness.resume(0, first_delay.resume_state()));
    assert_eq!(
        read.wait(),
        &DigitalWaitRequest::AnalogSample(harness.probe("sample"))
    );
    harness.set_analog("sample", 2.0);
    let second_delay = expect_suspended(harness.resume(0, read.resume_state()));
    assert_eq!(harness.get_real("total"), 13.0);
    let probe = usize::from(harness.probe("sample"));
    harness.store.analog[probe] = None;
    let next_read = expect_suspended(harness.resume(0, second_delay.resume_state()));
    harness.set_analog("sample", 4.0);
    let final_delay = expect_suspended(harness.resume(0, next_read.resume_state()));
    assert_eq!(harness.get_real("total"), 19.0);
    expect_finished(harness.resume(0, final_delay.resume_state()));
}

#[test]
fn module_integer_ownership_executes_signed_state_nba_and_partial_writes() {
    let mut harness = Harness::from_source(
        r#"
module counter;
integer state; reg signed [63:0] wide; reg msb,was_unknown;
initial begin
  was_unknown=(state===32'bx);
  state=32'sh7fffffff;
  state<=state+1;
  #1;
  wide=state; msb=state[31]; state[7:0]=8'hff;
  #1;
end
endmodule
"#,
    );
    let state = expect_suspended(harness.start(0)).resume_state().clone();
    assert_eq!(harness.get("was_unknown"), "1");
    assert_eq!(harness.get("state"), format!("{:032b}", i32::MAX));
    harness.flush_nonblocking();
    assert_eq!(harness.get("state"), format!("{:032b}", 0x80000000_u32));
    expect_suspended(harness.resume(0, &state));
    assert_eq!(
        harness.get("wide"),
        format!("{:064b}", (i32::MIN as i64) as u64)
    );
    assert_eq!(harness.get("msb"), "1");
    assert_eq!(harness.get("state"), format!("{:032b}", 0x800000ff_u32));
}
