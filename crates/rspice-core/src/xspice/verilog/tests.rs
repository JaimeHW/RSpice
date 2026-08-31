//! Executing digital Verilog, end to end.
//!
//! Every fixture here is compiled by the real front end and run by the real
//! host on the real event kernel. Nothing builds a plan by hand: a hand-built
//! plan would test the host against whatever shape the test author imagined,
//! and the shapes that matter are the ones the lowering emits.
//!
//! Each test names the clause of IEEE 1364-2005 it pins, and the expected
//! values are worked out from that clause rather than read off a run.

use super::*;
use crate::xspice::event_scheduler::{OscillationCause, SchedulerError, SchedulerLimits};
use rspice_veriloga::canonical_ir::digital_value::FourStateValue;

fn port(name: &str, width: u32) -> DigitalPort {
    DigitalPort {
        name: name.to_string(),
        width,
    }
}

fn vectors(rows: &[&[&str]]) -> Vec<Vec<String>> {
    rows.iter()
        .map(|row| row.iter().map(|value| (*value).to_string()).collect())
        .collect()
}

/// Every observation's values, flattened to `name=value` strings so an
/// expectation reads like a trace row.
fn rows(report: &DigitalRunReport) -> Vec<String> {
    report
        .observations
        .iter()
        .map(|observation| {
            observation
                .values
                .iter()
                .map(|(name, value)| format!("{name}={value}"))
                .collect::<Vec<_>>()
                .join(" ")
        })
        .collect()
}

// ===========================================================================
// Continuous assignment: section 6.1
// ===========================================================================

const COMBINATIONAL: &str = "\
module comb(a, b, y, n);
  input a, b;
  output y, n;
  wire a, b;
  wire y, n;
  assign y = a & b;
  assign n = ~a;
endmodule
";

/// A continuous assignment is a driver that is active from the start of the
/// simulation (section 6.1), so `y` is right at the first sample rather than
/// after the first input change.
#[test]
fn a_continuous_assignment_drives_from_time_zero() {
    let stimulus = DigitalStimulus {
        module: None,
        inputs: vec![port("a", 1), port("b", 1)],
        outputs: vec![port("y", 1), port("n", 1)],
        clock: None,
        step: 10,
        settle: 5,
        vectors: vectors(&[&["0", "0"], &["0", "1"], &["1", "0"], &["1", "1"]]),
    };
    let report = run_digital_verilog(COMBINATIONAL, &stimulus).expect("the design must run");
    assert_eq!(
        rows(&report),
        vec!["y=0 n=1", "y=0 n=1", "y=0 n=0", "y=1 n=0"]
    );
}

// ===========================================================================
// Four-state resolution: sections 4.1 and 7.9
// ===========================================================================

const TRISTATE: &str = "\
module tri_bus(hi, lo, a, bus, and_x, or_x);
  input hi, lo, a;
  output bus, and_x, or_x;
  wire hi, lo, a;
  wire bus;
  wire and_x, or_x;
  wire unknown;
  assign unknown = 1'bx;
  assign bus = hi ? 1'b1 : 1'bz;
  assign bus = lo ? 1'b0 : 1'bz;
  assign and_x = a & unknown;
  assign or_x = a | unknown;
endmodule
";

/// Table 4-1 through all four driver states of a two-driver net, and the
/// controlling-value rule of table 4-2 beside it: `0 & x` is 0 while `1 & x` is
/// x, so a front end that propagates X unconditionally fails the first and one
/// that ignores X fails the second.
#[test]
fn two_drivers_and_a_controlling_value_follow_the_four_state_tables() {
    let stimulus = DigitalStimulus {
        module: None,
        inputs: vec![port("hi", 1), port("lo", 1), port("a", 1)],
        outputs: vec![port("bus", 1), port("and_x", 1), port("or_x", 1)],
        clock: None,
        step: 10,
        settle: 5,
        vectors: vectors(&[
            &["0", "0", "0"],
            &["1", "0", "0"],
            &["0", "1", "0"],
            &["1", "1", "0"],
            &["0", "0", "1"],
        ]),
    };
    let report = run_digital_verilog(TRISTATE, &stimulus).expect("the design must run");
    assert_eq!(
        rows(&report),
        vec![
            // Neither driver on: the net is z, and 0 is controlling for AND
            // while 1 is not for OR.
            "bus=z and_x=0 or_x=x",
            "bus=1 and_x=0 or_x=x",
            "bus=0 and_x=0 or_x=x",
            // Both drivers on with opposite values: contention, which is x and
            // not either driver's value.
            "bus=x and_x=0 or_x=x",
            // a is 1, so AND is no longer controlled and OR is.
            "bus=z and_x=x or_x=1",
        ]
    );
}

// ===========================================================================
// Nonblocking assignment: section 11
// ===========================================================================

const SWAP: &str = "\
module swap(clk, rst, x, y);
  input clk, rst;
  output x, y;
  wire clk, rst;
  reg [3:0] x, y;
  always @(posedge clk) begin
    if (rst) begin
      x <= 4'b0101;
      y <= 4'b1010;
    end else begin
      x <= y;
      y <= x;
    end
  end
endmodule
";

/// Both right-hand sides read the pre-edge values, so the pair alternates. A
/// scheduler that applied nonblocking updates eagerly would assign both the
/// same value on the first exchange and they would never diverge again.
#[test]
fn nonblocking_assignments_exchange_rather_than_collapse() {
    let stimulus = DigitalStimulus {
        module: None,
        inputs: vec![port("rst", 1)],
        outputs: vec![port("x", 4), port("y", 4)],
        clock: Some(DigitalClock {
            port: "clk".to_string(),
            half_period: 5,
        }),
        step: 10,
        settle: 8,
        vectors: vectors(&[&["1"], &["0"], &["0"], &["0"]]),
    };
    let report = run_digital_verilog(SWAP, &stimulus).expect("the design must run");
    assert_eq!(
        rows(&report),
        vec![
            // First edge seeds the pair.
            "x=0101 y=1010",
            "x=1010 y=0101",
            "x=0101 y=1010",
            "x=1010 y=0101",
        ]
    );
}

const PIPELINES: &str = "\
module pipelines(clk, rst, din, nb2, bl2);
  input clk, rst;
  input [3:0] din;
  output [3:0] nb2, bl2;
  wire clk, rst;
  wire [3:0] din;
  reg [3:0] nb0, nb1;
  reg [3:0] nb2;
  reg [3:0] bl0, bl1;
  reg [3:0] bl2;
  always @(posedge clk) begin
    if (rst) begin
      nb0 <= 4'b0;
      nb1 <= 4'b0;
      nb2 <= 4'b0;
    end else begin
      nb0 <= din;
      nb1 <= nb0;
      nb2 <= nb1;
    end
  end
  always @(posedge clk) begin
    if (rst) begin
      bl0 = 4'b0;
      bl1 = 4'b0;
      bl2 = 4'b0;
    end else begin
      bl0 = din;
      bl1 = bl0;
      bl2 = bl1;
    end
  end
endmodule
";

/// The whole contrast of section 11 in one design: three nonblocking
/// assignments in one block are a three-deep shift register because every
/// right-hand side reads the pre-edge state, while the same three written
/// blocking execute in source order inside the edge and carry `din` all the way
/// through in one clock.
#[test]
fn blocking_and_nonblocking_pipelines_have_different_depths() {
    let stimulus = DigitalStimulus {
        module: None,
        inputs: vec![port("rst", 1), port("din", 4)],
        outputs: vec![port("nb2", 4), port("bl2", 4)],
        clock: Some(DigitalClock {
            port: "clk".to_string(),
            half_period: 5,
        }),
        step: 10,
        settle: 8,
        vectors: vectors(&[
            &["1", "0000"],
            &["0", "0001"],
            &["0", "0010"],
            &["0", "0100"],
            &["0", "0100"],
            &["0", "0100"],
        ]),
    };
    let report = run_digital_verilog(PIPELINES, &stimulus).expect("the design must run");
    assert_eq!(
        rows(&report),
        vec![
            // Reset.
            "nb2=0000 bl2=0000",
            // `din` reaches bl2 on the very edge it is applied; the
            // nonblocking chain is still shifting zeros through.
            "nb2=0000 bl2=0001",
            "nb2=0000 bl2=0010",
            "nb2=0001 bl2=0100",
            "nb2=0010 bl2=0100",
            "nb2=0100 bl2=0100",
        ]
    );
}

// ===========================================================================
// Edge detection and asynchronous reset: sections 9.7.2 and table 5-2
// ===========================================================================

const REGISTER: &str = "\
module reg4(clk, rst, en, d, q, qd);
  input clk, rst, en;
  input [3:0] d;
  output [3:0] q, qd;
  wire clk, rst, en;
  wire [3:0] d;
  reg [3:0] q;
  reg [3:0] qd;
  always @(posedge clk or posedge rst) begin
    if (rst) begin
      q <= 4'b0;
      qd <= 4'b0;
    end else begin
      qd <= q;
      if (en) begin
        q <= d;
      end
    end
  end
endmodule
";

/// A sensitivity list with two edge terms runs on either of them and the body
/// decides which fired, and `qd` trails `q` by exactly one clock because both
/// are written nonblocking from the same edge.
#[test]
fn an_edge_triggered_register_loads_holds_and_resets() {
    let stimulus = DigitalStimulus {
        module: None,
        inputs: vec![port("rst", 1), port("en", 1), port("d", 4)],
        outputs: vec![port("q", 4), port("qd", 4)],
        clock: Some(DigitalClock {
            port: "clk".to_string(),
            half_period: 5,
        }),
        step: 10,
        settle: 8,
        vectors: vectors(&[
            &["1", "0", "0000"],
            &["0", "1", "0001"],
            &["0", "1", "0010"],
            &["0", "0", "0100"],
            &["0", "1", "1000"],
        ]),
    };
    let report = run_digital_verilog(REGISTER, &stimulus).expect("the design must run");
    assert_eq!(
        rows(&report),
        vec![
            // Reset is asynchronous and fires on its own rising edge, before
            // the first clock edge is even reached.
            "q=0000 qd=0000",
            "q=0001 qd=0000",
            "q=0010 qd=0001",
            // Enable low: `q` holds while `d` changes underneath, and `qd`
            // still samples the pre-edge `q`.
            "q=0010 qd=0010",
            "q=1000 qd=0010",
        ]
    );
}

// ===========================================================================
// Delay control: section 9.7.1
// ===========================================================================

const DELAYED: &str = "\
module delayed(go, out);
  input go;
  output out;
  wire go;
  reg out;
  always @(posedge go) begin
    out <= 1'b1;
    #7 out <= 1'b0;
  end
endmodule
";

/// A `#delay` suspends the process and resumes it that many time units later,
/// measured from the suspension. Sampled at 5, 15 and 25 time units, the
/// second observation is after the delay has expired and the third is not
/// disturbed by it.
#[test]
fn a_delay_resumes_the_process_that_many_time_units_later() {
    let stimulus = DigitalStimulus {
        module: None,
        inputs: vec![port("go", 1)],
        outputs: vec![port("out", 1)],
        clock: None,
        step: 10,
        settle: 5,
        vectors: vectors(&[&["0"], &["1"], &["1"]]),
    };
    let report = run_digital_verilog(DELAYED, &stimulus).expect("the design must run");
    assert_eq!(
        rows(&report),
        vec![
            // No edge yet.
            "out=x",
            // The edge at t=10 set `out`; the delay expires at t=17, after
            // this sample at t=15.
            "out=1", // t=25, past the delay.
            "out=0",
        ]
    );
}

// ===========================================================================
// Refusals
// ===========================================================================

#[test]
fn a_timescale_directive_is_refused_by_name() {
    let source = format!("`timescale 1ns/1ps\n{COMBINATIONAL}");
    let error = run_digital_verilog(
        &source,
        &DigitalStimulus {
            module: None,
            inputs: vec![port("a", 1), port("b", 1)],
            outputs: vec![port("y", 1)],
            clock: None,
            step: 10,
            settle: 5,
            vectors: vectors(&[&["0", "0"]]),
        },
    )
    .expect_err("the host cannot honour a timescale");
    assert!(
        matches!(error, DigitalRunError::TimescaleDirective { line: 1 }),
        "{error:?}"
    );
    assert!(error.to_string().contains("one tick is 1 ns"), "{error}");
}

/// A commented-out directive is not one. Without this the refusal would fire on
/// a design that merely mentions it.
#[test]
fn a_commented_timescale_is_not_a_directive() {
    let source = format!("// `timescale 1ns/1ns\n{COMBINATIONAL}");
    run_digital_verilog(
        &source,
        &DigitalStimulus {
            module: None,
            inputs: vec![port("a", 1), port("b", 1)],
            outputs: vec![port("y", 1)],
            clock: None,
            step: 10,
            settle: 5,
            vectors: vectors(&[&["1", "1"]]),
        },
    )
    .expect("a comment is not a directive");
}

#[test]
fn a_mixed_signal_module_is_refused_rather_than_run_digital_only() {
    const MIXED: &str = "\
module mixed(p, n, a, y);
  inout p, n;
  electrical p, n;
  input a;
  output y;
  wire a, y;
  assign y = ~a;
  analog I(p, n) <+ V(p, n);
endmodule
";
    let error = run_digital_verilog(
        MIXED,
        &DigitalStimulus {
            module: None,
            inputs: vec![port("a", 1)],
            outputs: vec![port("y", 1)],
            clock: None,
            step: 10,
            settle: 5,
            vectors: vectors(&[&["0"]]),
        },
    )
    .expect_err("mixed-signal interleave is not implemented");
    assert!(
        matches!(error, DigitalRunError::MixedSignalModule { .. }),
        "{error:?}"
    );
}

#[test]
fn a_stimulus_driving_a_net_the_design_drives_is_refused() {
    let error = run_digital_verilog(
        COMBINATIONAL,
        &DigitalStimulus {
            module: None,
            // `y` is an output the design drives; driving it from outside would
            // need resolution against a driver the stimulus does not have.
            inputs: vec![port("y", 1)],
            outputs: vec![port("n", 1)],
            clock: None,
            step: 10,
            settle: 5,
            vectors: vectors(&[&["1"]]),
        },
    )
    .expect_err("the design drives `y`");
    assert!(
        matches!(
            error,
            DigitalRunError::StimulusOnDrivenNet { drivers: 1, .. }
        ),
        "{error:?}"
    );
}

#[test]
fn a_stimulus_naming_a_port_the_design_lacks_is_refused() {
    let error = run_digital_verilog(
        COMBINATIONAL,
        &DigitalStimulus {
            module: None,
            inputs: vec![port("nope", 1)],
            outputs: vec![port("y", 1)],
            clock: None,
            step: 10,
            settle: 5,
            vectors: vectors(&[&["1"]]),
        },
    )
    .expect_err("there is no port called `nope`");
    assert!(
        matches!(error, DigitalRunError::UnknownSignal { .. }),
        "{error:?}"
    );
}

#[test]
fn a_vector_column_of_the_wrong_width_is_refused() {
    let error = run_digital_verilog(
        COMBINATIONAL,
        &DigitalStimulus {
            module: None,
            inputs: vec![port("a", 1), port("b", 1)],
            outputs: vec![port("y", 1)],
            clock: None,
            step: 10,
            settle: 5,
            vectors: vectors(&[&["01", "0"]]),
        },
    )
    .expect_err("`a` is one bit wide");
    assert!(
        matches!(error, DigitalRunError::VectorWidth { .. }),
        "{error:?}"
    );
}

// ===========================================================================
// Oscillation
// ===========================================================================

/// A combinational loop with no delay in it, which IEEE 1364-2005 leaves as a
/// hang. `~x` is `x`, so an inverter feeding itself settles in four-state
/// logic; a `case` that maps every ambiguous value to a defined one does not,
/// and is the smallest thing that really oscillates.
const OSCILLATOR: &str = "\
module osc(seed, y);
  input seed;
  output y;
  wire seed;
  reg y;
  always @(y or seed) begin
    case (y)
      1'b1: y = 1'b0;
      default: y = 1'b1;
    endcase
  end
endmodule
";

/// The loop must reach the kernel's ceiling and report which process kept
/// firing, rather than running forever.
///
/// Driven through [`DigitalHost`] directly so the ceiling can be small: at the
/// production ceiling of ten thousand the test would still pass and would take
/// ten thousand times as long to say so.
#[test]
fn a_combinational_loop_reports_the_kernels_oscillation_diagnostic() {
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    let artifact = compiler
        .compile_canonical_ir_module(OSCILLATOR, None)
        .expect("the oscillator compiles");
    let plan = &artifact.digital;

    let limits = SchedulerLimits {
        max_delta_cycles_per_tick: 64,
        ..SchedulerLimits::default()
    };
    let mut host = DigitalHost::new(
        plan,
        TimeResolution::new(TIME_UNIT_EXPONENT).expect("1 ns is a legal resolution"),
        limits,
    );
    host.start().expect("nothing has woken the loop yet");

    let seed = host.signal("seed").expect("`seed` is declared");
    let error = host
        .force(seed, FourStateValue::splat(1, FourStateBit::Zero), 0)
        .expect_err("the loop cannot settle");

    let DigitalRunError::Scheduler(SchedulerError::Oscillation(diagnostic)) = &error else {
        panic!("expected the kernel's typed oscillation diagnostic, got {error:?}");
    };
    assert_eq!(diagnostic.cause, OscillationCause::DeltaCycleLimit);
    assert_eq!(diagnostic.delta_cycle_limit, 64);
    assert_eq!(diagnostic.tick, 0);
    // The report names the process that kept firing, which is the evidence
    // needed to find the loop.
    let (busiest, count) = diagnostic
        .entities
        .first()
        .expect("the diagnostic must name a driver");
    assert!(busiest.instance.starts_with("always#"), "{busiest:?}");
    assert!(*count > 1, "the busiest driver fired {count} time(s)");
    assert!(error.to_string().contains("did not settle"), "{error}");
}

/// A design that does settle must not trip the same ceiling, or the guard would
/// be reporting the size of the design rather than the depth of its settling.
#[test]
fn a_settling_design_stays_far_below_the_delta_ceiling() {
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    let artifact = compiler
        .compile_canonical_ir_module(COMBINATIONAL, None)
        .expect("the design compiles");
    let plan = &artifact.digital;

    let limits = SchedulerLimits {
        max_delta_cycles_per_tick: 8,
        ..SchedulerLimits::default()
    };
    let mut host = DigitalHost::new(
        plan,
        TimeResolution::new(TIME_UNIT_EXPONENT).expect("1 ns is a legal resolution"),
        limits,
    );
    host.start().expect("startup settles");
    let a = host.signal("a").expect("`a` is declared");
    let b = host.signal("b").expect("`b` is declared");
    host.force(a, FourStateValue::splat(1, FourStateBit::One), 0)
        .expect("settles");
    host.force(b, FourStateValue::splat(1, FourStateBit::One), 0)
        .expect("settles");
    let y = host.signal("y").expect("`y` is declared");
    assert_eq!(host.read(y).expect("declared").spelling(), "1");
}

// ===========================================================================
// Real nets: Verilog-AMS LRM 2.4 section 3.7
// ===========================================================================

/// A real-valued port. Width zero is how the front end spells "no bits", and
/// the stimulus says the same so the two are checked against each other.
fn real_port(name: &str) -> DigitalPort {
    port(name, 0)
}

const RNM_GAIN: &str = "\
module rnm_gain(vin, vout, over);
  input wreal vin;
  output wreal vout;
  output over;
  reg over;
  assign vout = vin * 0.5 + 0.25;
  always @(vout) if (vout > 1.0) over = 1'b1; else over = 1'b0;
endmodule
";

/// A real in, an algebraic mapping, a real out — and a four-state flag driven
/// from a comparison on the way. The whole route in one run: a `wreal` port, a
/// continuous assignment in the real domain, and a process woken by section
/// 3.7's value-change event.
#[test]
fn a_real_valued_block_maps_its_input_and_flags_it() {
    let stimulus = DigitalStimulus {
        module: None,
        inputs: vec![real_port("vin")],
        outputs: vec![real_port("vout"), port("over", 1)],
        clock: None,
        step: 10,
        settle: 5,
        vectors: vectors(&[&["0.0"], &["1.0"], &["2.0"], &["-3.0"], &["4.0"]]),
    };
    let report = run_digital_verilog(RNM_GAIN, &stimulus).expect("the design must run");
    // vin/2 + 1/4, exactly representable at every one of these points, so the
    // expectation is the arithmetic and not a rounding of it.
    assert_eq!(
        rows(&report),
        vec![
            "vout=0.25 over=0",
            "vout=0.75 over=0",
            "vout=1.25 over=1",
            "vout=-1.25 over=0",
            "vout=2.25 over=1",
        ]
    );
}

const RNM_UNDRIVEN: &str = "\
module rnm_undriven(seen, level);
  input seen;
  output wreal level;
  wire seen;
endmodule
";

/// "If no driver is connected to a wreal net, its value shall be zero (0.0).
/// Unlike other digital nets which have an initial value of `z`, wreal nets
/// shall have an initial value of zero." — section 3.7, end to end.
#[test]
fn an_undriven_real_net_reads_zero_rather_than_high_impedance() {
    let stimulus = DigitalStimulus {
        module: None,
        inputs: vec![port("seen", 1)],
        outputs: vec![real_port("level")],
        clock: None,
        step: 10,
        settle: 5,
        vectors: vectors(&[&["0"], &["1"]]),
    };
    let report = run_digital_verilog(RNM_UNDRIVEN, &stimulus).expect("the design must run");
    assert_eq!(rows(&report), vec!["level=0.0", "level=0.0"]);
}

const RNM_DAC: &str = "\
module rnm_dac(code, out);
  input [3:0] code;
  output wreal out;
  wire [3:0] code;
  wreal step3, step2, step1, step0;
  assign step3 = code[3] ? 8.0 : 0.0;
  assign step2 = code[2] ? 4.0 : 0.0;
  assign step1 = code[1] ? 2.0 : 0.0;
  assign step0 = code[0] ? 1.0 : 0.0;
  assign out = (step3 + step2) + (step1 + step0);
endmodule
";

/// A four-state bus in, a real out: the ladder weights each bit and sums them.
///
/// The bits never become a real. Each rung is a four-state *condition* choosing
/// between two real constants, which is the only bridge section 3.7 leaves open
/// without `$bitstoreal` — and is how a real-number DAC is actually written.
#[test]
fn a_real_valued_dac_ladder_weights_each_bit() {
    let stimulus = DigitalStimulus {
        module: None,
        inputs: vec![port("code", 4)],
        outputs: vec![real_port("out")],
        clock: None,
        step: 10,
        settle: 5,
        vectors: vectors(&[&["0000"], &["0001"], &["0101"], &["1010"], &["1111"]]),
    };
    let report = run_digital_verilog(RNM_DAC, &stimulus).expect("the design must run");
    assert_eq!(
        rows(&report),
        vec!["out=0.0", "out=1.0", "out=5.0", "out=10.0", "out=15.0"]
    );
}

/// One source for the four resolved spellings, each pinned against the
/// arithmetic its keyword names. Two drivers, on the three combinations of
/// 2.0/3.0 and -3.0/-1.0 the selector reaches.
#[test]
fn each_wreal_resolution_combines_its_drivers_end_to_end() {
    for (keyword, expected) in [
        ("wrealsum", ["1.0", "2.0", "0.0"]),
        ("wrealavg", ["0.5", "1.0", "0.0"]),
        ("wrealmin", ["-1.0", "-1.0", "-3.0"]),
        ("wrealmax", ["2.0", "3.0", "3.0"]),
    ] {
        let source = format!(
            "module bus_case(sel, out);\n\
             \x20 input [1:0] sel;\n\
             \x20 output {keyword} out;\n\
             \x20 wire [1:0] sel;\n\
             \x20 assign out = sel[1] ? 3.0 : 2.0;\n\
             \x20 assign out = sel[0] ? -1.0 : -3.0;\n\
             endmodule\n"
        );
        let stimulus = DigitalStimulus {
            module: None,
            inputs: vec![port("sel", 2)],
            outputs: vec![real_port("out")],
            clock: None,
            step: 10,
            settle: 5,
            // 01: 2.0 and -1.0. 11: 3.0 and -1.0. 10: 3.0 and -3.0.
            vectors: vectors(&[&["01"], &["11"], &["10"]]),
        };
        let report = run_digital_verilog(&source, &stimulus)
            .unwrap_or_else(|error| panic!("`{keyword}` must run: {error}"));
        assert_eq!(
            rows(&report),
            expected
                .iter()
                .map(|value| format!("out={value}"))
                .collect::<Vec<_>>(),
            "{keyword}"
        );
    }
}

const RNM_WAKE: &str = "\
module rnm_wake(vin, ticks);
  input wreal vin;
  output [3:0] ticks;
  reg [3:0] ticks;
  initial ticks = 4'b0000;
  always @(vin) ticks = ticks + 4'b0001;
endmodule
";

/// `@(wreal)` wakes on a value change and on nothing else. The count rises once
/// per distinct value and stays put when the stimulus repeats one — including
/// across a change of one unit in the last place, because section 3.7's event
/// is a change of value and this host applies no tolerance to it.
#[test]
fn a_value_change_on_a_real_net_wakes_a_waiting_process() {
    let stimulus = DigitalStimulus {
        module: None,
        inputs: vec![real_port("vin")],
        outputs: vec![port("ticks", 4)],
        clock: None,
        step: 10,
        settle: 5,
        vectors: vectors(&[
            &["1.0"],
            &["1.0"],
            &["2.0"],
            &["2.0"],
            &["2.0000000000000004"],
            &["2.0000000000000004"],
        ]),
    };
    let report = run_digital_verilog(RNM_WAKE, &stimulus).expect("the design must run");
    // Three distinct values, so three wakeups: the repeats move nothing and
    // wake nothing, and the last pair differs from `2.0` by one unit in the
    // last place and does.
    assert_eq!(
        rows(&report),
        vec![
            "ticks=0001",
            "ticks=0001",
            "ticks=0010",
            "ticks=0010",
            "ticks=0011",
            "ticks=0011",
        ]
    );
}

/// A stimulus and a design that disagree about a port's domain are refused
/// rather than converted for. The design is the authority either way.
#[test]
fn a_stimulus_in_the_wrong_value_domain_is_refused() {
    let bits_for_a_real = DigitalStimulus {
        module: None,
        inputs: vec![port("vin", 1)],
        outputs: vec![real_port("vout"), port("over", 1)],
        clock: None,
        step: 10,
        settle: 5,
        vectors: vectors(&[&["1"]]),
    };
    match run_digital_verilog(RNM_GAIN, &bits_for_a_real) {
        Err(DigitalRunError::StimulusValueDomain { name, port_is_real }) => {
            assert_eq!(name, "vin");
            assert!(port_is_real);
        }
        other => panic!("expected a domain refusal, got {other:?}"),
    }

    let a_real_for_bits = DigitalStimulus {
        module: None,
        inputs: vec![real_port("vin")],
        outputs: vec![real_port("vout"), real_port("over")],
        clock: None,
        step: 10,
        settle: 5,
        vectors: vectors(&[&["1.0"]]),
    };
    match run_digital_verilog(RNM_GAIN, &a_real_for_bits) {
        Err(DigitalRunError::StimulusValueDomain { name, port_is_real }) => {
            assert_eq!(name, "over");
            assert!(!port_is_real);
        }
        other => panic!("expected a domain refusal, got {other:?}"),
    }
}

/// A column that is not a number, for a port that carries one.
#[test]
fn a_real_column_that_is_not_a_number_is_refused() {
    let stimulus = DigitalStimulus {
        module: None,
        inputs: vec![real_port("vin")],
        outputs: vec![real_port("vout"), port("over", 1)],
        clock: None,
        step: 10,
        settle: 5,
        vectors: vectors(&[&["1z0"]]),
    };
    match run_digital_verilog(RNM_GAIN, &stimulus) {
        Err(DigitalRunError::RealSpelling { port, spelling }) => {
            assert_eq!(port, "vin");
            assert_eq!(spelling, "1z0");
        }
        other => panic!("expected a spelling refusal, got {other:?}"),
    }
}
