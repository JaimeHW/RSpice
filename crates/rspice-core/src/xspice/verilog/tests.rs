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
use crate::xspice::event_scheduler::{Instant, OscillationCause, SchedulerError, SchedulerLimits};
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

#[test]
fn digital_clock_queries_use_each_hierarchical_module_unit() {
    let source = "`timescale 10ns/1ps
        module child(q); output q; reg q; initial begin q=0; #0.15
            q=($realtime==0.15 && $time==0 && $stime==0 && $abstime>1.49999999999e-9 && $abstime<1.50000000001e-9);
        end endmodule
        `timescale 1ns/1ps
        module top(q,c); output q,c; reg q; wire c; child u(c);
        initial begin q=0; #1.5
            q=($realtime==1.5 && $time==2 && $stime==2 && $abstime>1.49999999999e-9 && $abstime<1.50000000001e-9);
        end endmodule";
    let design = CompiledDigitalDesign::compile(source, Some("top")).unwrap();
    let report = design
        .run(&DigitalStimulus {
            module: None,
            inputs: vec![],
            outputs: vec![port("q", 1), port("c", 1)],
            clock: None,
            step: 1500,
            settle: 0,
            vectors: vec![vec![], vec![]],
        })
        .unwrap();
    assert_eq!(rows(&report), ["q=0 c=0", "q=1 c=1"]);
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

#[test]
fn portless_generated_processes_resume_on_the_event_kernel() {
    let source = "module self_contained;
        reg [3:0] q; wire [3:0] w; genvar i;
        generate for (i=0; i<4; i=i+1) begin : gen
            initial begin q[i]=i%2; #2 q[i]=~q[i]; end
        end endgenerate
        assign w=q;
        endmodule";
    let design = CompiledDigitalDesign::compile(source, None).expect("portless design compiles");
    let stimulus = DigitalStimulus {
        module: None,
        inputs: vec![],
        outputs: vec![port("w", 4)],
        clock: None,
        step: 2,
        settle: 1,
        vectors: vec![vec![], vec![], vec![]],
    };
    for _ in 0..2 {
        let report = design
            .run(&stimulus)
            .expect("generated processes execute and resume");
        assert_eq!(rows(&report), ["w=1010", "w=0101", "w=0101"]);
    }
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

#[test]
fn delayed_nonblocking_updates_preserve_capture_order_and_nba_regions() {
    let source = "module timed(q,held,seen,inactive,done);
        output [7:0] q,held,seen,inactive; output done;
        reg [7:0] q,held,seen,inactive,data,duration; reg done;
        initial begin
          q=0; held=0; seen=0; inactive=0; done=0; data=1; duration=5;
          q <= #duration data;
          q <= #2 2;
          held[3:0] <= #3 data[3:0]; held[7:4] <= #3 4'ha;
          data=9; duration=9; done=1;
          #5 seen=q;
          q <= #0 3;
          #0 inactive=q;
        end endmodule";
    let report = run_digital_verilog(
        source,
        &DigitalStimulus {
            module: None,
            inputs: vec![],
            outputs: vec![
                port("q", 8),
                port("held", 8),
                port("seen", 8),
                port("inactive", 8),
                port("done", 1),
            ],
            clock: None,
            step: 1,
            settle: 0,
            vectors: vec![vec![]; 7],
        },
    )
    .unwrap();
    let values: Vec<Vec<String>> = report
        .observations
        .iter()
        .map(|row| row.values.iter().map(|(_, value)| value.clone()).collect())
        .collect();
    assert_eq!(values.len(), 7);
    for (tick, row) in values.iter().enumerate() {
        let q = if tick < 2 {
            0
        } else if tick < 5 {
            2
        } else {
            3
        };
        let held = if tick < 3 { 0 } else { 0xa1 };
        let before_nba = if tick < 5 { 0 } else { 2 };
        assert_eq!(
            *row,
            [
                format!("{q:08b}"),
                format!("{held:08b}"),
                format!("{before_nba:08b}"),
                format!("{before_nba:08b}"),
                "1".to_string()
            ],
            "tick {tick}"
        );
    }
    // A completed process's only future work can be an NBA wakeup. Delay
    // deadlines outside the host's exact physical range must fail explicitly.
    let error = run_digital_verilog(
        "module overflow(q); output q; reg q; initial q<=#2251799813685248 1; endmodule",
        &DigitalStimulus {
            module: None,
            inputs: vec![],
            outputs: vec![port("q", 1)],
            clock: None,
            step: 1,
            settle: 0,
            vectors: vec![vec![]],
        },
    )
    .unwrap_err();
    assert!(matches!(error, DigitalRunError::TickOverflow), "{error}");
}

#[test]
fn event_nonblocking_captures_observe_registration_order_and_deliver_once() {
    let source = "module events(q,late,captured,done,seen,real_ok,ordered);
        output [7:0] q,late,captured,seen,ordered; output done,real_ok;
        reg [7:0] q,late,captured,seen,data,ordered; reg done,real_ok,clk,a,b;
        real changed,held;
        initial begin
          q=0; late=0; captured=0; done=0; seen=0; real_ok=0; ordered=0;
          clk=0; a=0; b=0; changed=0.0; held=0.0; data=8'h41;
          captured <= @(negedge clk) data;
          q <= @(posedge clk) 8'h11; q <= @(posedge clk) 8'h22;
          held <= @(changed) 1.25;
          ordered <= @(posedge a) 1; ordered <= @(posedge b) 2;
          b=1; a=1;
          data=8'h99; clk=1; changed=2.0;
          late <= @(posedge clk) 8'h33;
          #0 seen=q;
          #1 real_ok=(held==1.25); clk=0;
          #1 clk=1;
          @(posedge clk) begin done=1; q=0; late=0; end
        end
        initial begin #3 clk=0; #1 clk=1; #1 clk=0; #1 clk=1; end
        endmodule";
    let report = run_digital_verilog(
        source,
        &DigitalStimulus {
            module: None,
            inputs: vec![],
            outputs: vec![
                port("q", 8),
                port("late", 8),
                port("captured", 8),
                port("done", 1),
                port("seen", 8),
                port("real_ok", 1),
                port("ordered", 8),
            ],
            clock: None,
            step: 1,
            settle: 0,
            vectors: vec![vec![]; 7],
        },
    )
    .unwrap();
    for (tick, row) in report.observations.iter().enumerate() {
        let q = if tick < 4 { 0x22 } else { 0 };
        let late = if (2..4).contains(&tick) { 0x33 } else { 0 };
        let captured = if tick == 0 { 0 } else { 0x41 };
        let actual: Vec<_> = row.values.iter().map(|(_, value)| value.clone()).collect();
        assert_eq!(
            actual,
            [
                format!("{q:08b}"),
                format!("{late:08b}"),
                format!("{captured:08b}"),
                u8::from(tick >= 4).to_string(),
                "00000000".to_string(),
                u8::from(tick >= 1).to_string(),
                "00000010".to_string()
            ],
            "tick {tick}"
        );
    }
}

// ===========================================================================
// Refusals
// ===========================================================================

#[test]
fn an_active_timescale_directive_executes() {
    let source = format!("`timescale 1ns/1ps\n{COMBINATIONAL}");
    let compiled = CompiledDigitalDesign::compile(&source, None).unwrap();
    assert_eq!(compiled.time_resolution().seconds_per_tick(), 1e-12);
    let result = compiled
        .run(&DigitalStimulus {
            module: None,
            inputs: vec![port("a", 1), port("b", 1)],
            outputs: vec![port("y", 1)],
            clock: None,
            step: 10,
            settle: 5,
            vectors: vectors(&[&["0", "0"]]),
        })
        .expect("declared timing executes");
    assert_eq!(result.observations.len(), 1);
}

/// A commented-out directive must not alter the compiled timing.
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
fn the_digital_only_convenience_api_routes_mixed_modules_to_the_transient_host() {
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
    .expect_err("the vector-only API has no analog Newton contract");
    assert!(
        matches!(error, DigitalRunError::MixedSignalModule { .. }),
        "{error:?}"
    );
    assert!(error.to_string().contains("MixedSignalHost"), "{error}");
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

/// An NBA feedback loop with no delay, which IEEE 1364-2005 leaves as a
/// hang. The update occurs after the process re-arms its event control. `~x` is `x`, so an inverter feeding itself settles in four-state
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
      1'b1: y <= 1'b0;
      default: y <= 1'b1;
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
    // The kernel reports the instant; the host names the tick of it on the
    // design's own grid, which is what the sentence prints.
    assert_eq!(diagnostic.at, Instant::ZERO);
    assert_eq!(diagnostic.tick, Some(0));
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

// ===========================================================================
// Compile once, run many: what a compiled design carries between runs
// ===========================================================================

/// Two modules with identical port lists that differ only in what `qd` loads.
///
/// Identical ports on purpose: every name a stimulus for one uses resolves in
/// the other, so nothing about *resolution* can catch a run against the wrong
/// design. That is what makes the module check worth having and what makes the
/// interleaving pin below mean something.
const TWO_REGISTERS: &str = "\
module reg_direct(clk, rst, en, d, q, qd);
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

module reg_inverted(clk, rst, en, d, q, qd);
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
      qd <= ~q;
      if (en) begin
        q <= d;
      end
    end
  end
endmodule
";

/// The stimulus [`an_edge_triggered_register_loads_holds_and_resets`] uses,
/// as a function so the compile-once pins can drive several designs with it.
///
/// A design with state is what these pins need: a run that left anything
/// behind would show as a `q` that started where the last one stopped.
fn register_stimulus(module: Option<&str>) -> DigitalStimulus {
    DigitalStimulus {
        module: module.map(str::to_string),
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
    }
}

/// The trace [`an_edge_triggered_register_loads_holds_and_resets`] derives from
/// sections 9.7.2 and 5.2, restated here as the value a compiled design has to
/// keep producing.
const REGISTER_TRACE: [&str; 5] = [
    "q=0000 qd=0000",
    "q=0001 qd=0000",
    "q=0010 qd=0001",
    "q=0010 qd=0010",
    "q=1000 qd=0010",
];

/// The one-call route is the composition of the two halves and nothing else.
#[test]
fn the_one_call_route_is_the_compiled_route_composed() {
    let stimulus = register_stimulus(None);
    let one_call = run_digital_verilog(REGISTER, &stimulus).expect("the design must run");
    let compiled = CompiledDigitalDesign::compile(REGISTER, None).expect("the design must compile");
    let two_calls = compiled.run(&stimulus).expect("the design must run");
    assert_eq!(one_call, two_calls);
    assert_eq!(rows(&one_call), REGISTER_TRACE);
    assert!(format!("{compiled:?}").contains("reg4"), "{compiled:?}");
}

/// Two runs of one compiled design are identical, field for field.
///
/// The property the split rests on. Every kind of state this host keeps is
/// loaded by the first run — a `reg`, the nonblocking queue that carries
/// `qd <= q`, a process suspended on an edge, the clock level the harness
/// drives — and if any of it survived into the second run, the second's `q`
/// would start at `1000` rather than at the reset the first vector asserts.
#[test]
fn two_runs_of_one_compiled_design_produce_identical_reports() {
    let compiled = CompiledDigitalDesign::compile(REGISTER, None).expect("the design must compile");
    let stimulus = register_stimulus(None);
    let first = compiled.run(&stimulus).expect("the first run must run");
    for _ in 0..3 {
        assert_eq!(
            compiled.run(&stimulus).expect("a later run must run"),
            first
        );
    }
    assert_eq!(rows(&first), REGISTER_TRACE);
}

/// Runs of two compiled designs, interleaved, do not reach each other.
///
/// Both designs are stateful, carry the same port names, and are compiled from
/// the same source, so a plan shared where it should not be or a host reused
/// across designs would be visible in the trace rather than only in the timing.
#[test]
fn interleaved_runs_of_two_compiled_designs_do_not_reach_each_other() {
    let direct = CompiledDigitalDesign::compile(TWO_REGISTERS, Some("reg_direct"))
        .expect("`reg_direct` must compile");
    let inverted = CompiledDigitalDesign::compile(TWO_REGISTERS, Some("reg_inverted"))
        .expect("`reg_inverted` must compile");
    let direct_stimulus = register_stimulus(Some("reg_direct"));
    let inverted_stimulus = register_stimulus(Some("reg_inverted"));

    let direct_alone = direct.run(&direct_stimulus).expect("`reg_direct` must run");
    let inverted_alone = inverted
        .run(&inverted_stimulus)
        .expect("`reg_inverted` must run");

    // The two do differ, and where they must: the first clock edge after the
    // reset loads `qd` with `q` in one design and with `~q` in the other, and
    // `q` is `0000` there because the first vector asserted the asynchronous
    // reset.
    assert_eq!(rows(&direct_alone), REGISTER_TRACE);
    assert_eq!(rows(&inverted_alone)[1], "q=0001 qd=1111");

    for _ in 0..3 {
        assert_eq!(
            direct.run(&direct_stimulus).expect("`reg_direct` must run"),
            direct_alone
        );
        assert_eq!(
            inverted
                .run(&inverted_stimulus)
                .expect("`reg_inverted` must run"),
            inverted_alone
        );
    }
}

/// A stimulus naming another module is refused rather than run against this
/// one.
#[test]
fn a_stimulus_naming_another_module_is_refused_by_a_compiled_design() {
    let direct = CompiledDigitalDesign::compile(TWO_REGISTERS, Some("reg_direct"))
        .expect("`reg_direct` must compile");
    match direct.run(&register_stimulus(Some("reg_inverted"))) {
        Err(DigitalRunError::StimulusModule {
            compiled,
            requested,
        }) => {
            assert_eq!(compiled, "reg_direct");
            assert_eq!(requested, "reg_inverted");
        }
        other => panic!("expected a module refusal, got {other:?}"),
    }
    // A stimulus that names nothing is a stimulus that names this design.
    direct
        .run(&register_stimulus(None))
        .expect("an unnamed stimulus runs against whatever was compiled");
}

/// Every refusal the one-call route makes before running, the compile half
/// makes at compile time — which is the point of having a compile half.
#[test]
fn the_compile_half_makes_every_pre_run_refusal() {
    const TIMESCALED: &str = "`timescale 1ns/1us\nmodule t(a, y);\n  input a;\n  output y;\n  \
                              wire a;\n  assign y = ~a;\nendmodule\n";
    match CompiledDigitalDesign::compile(TIMESCALED, None) {
        Err(DigitalRunError::Compile { detail }) => {
            assert!(detail.contains("precision"), "{detail}")
        }
        other => panic!("expected a timescale refusal, got {other:?}"),
    }

    const MIXED: &str = "\
module mixed_pre(p, n, a, y);
  inout p, n;
  electrical p, n;
  input a;
  output y;
  wire a, y;
  assign y = ~a;
  analog I(p, n) <+ V(p, n);
endmodule
";
    match CompiledDigitalDesign::compile(MIXED, None) {
        Err(DigitalRunError::MixedSignalModule { module, .. }) => assert_eq!(module, "mixed_pre"),
        other => panic!("expected a mixed-module refusal, got {other:?}"),
    }

    match CompiledDigitalDesign::compile("module m(a); input a; endmodule\n", None) {
        Err(DigitalRunError::Compile { .. } | DigitalRunError::NoDigitalContent { .. }) => {}
        other => panic!("expected a refusal for a module with nothing to run, got {other:?}"),
    }
}

#[test]
fn computed_event_expressions_preserve_pulses_captures_and_current_clocks() {
    let source = "module computed(q,late,selected,guarded,real_ok,done);
      output [7:0] q; output late,selected,guarded,real_ok,done;
      reg [7:0] q; reg late,selected,guarded,real_ok,done,clk,enable;
      reg [3:0] bus; reg [1:0] index; real a,b,held;
      initial begin
        q=0; late=0; selected=0; guarded=0; real_ok=0; done=0;
        clk=0; enable=0; bus=0; index=0; a=0.0; b=0.0; held=0.0;
        q <= @(posedge (clk & enable)) 7; clk=1;
        q <= @(posedge (clk & enable)) 9; enable=1; enable=0;
        late <= @(posedge (clk & enable)) 1;
        selected <= @(bus[index]) 1; bus=4'b1000; index=1;
        guarded <= @(posedge (bus==4'b1010)) 1; bus=4'b1010;
        held <= @(a+b) 1.25; a=1.0; b=-1.0;
        #1 real_ok=(held==1.25); clk=0; enable=1;
        #1 clk=1; @(posedge (clk & enable)) done=1;
      end
      initial begin #3 clk=0; #1 clk=1; end
      endmodule";
    let report = run_digital_verilog(
        source,
        &DigitalStimulus {
            module: None,
            inputs: vec![],
            outputs: vec![
                port("q", 8),
                port("late", 1),
                port("selected", 1),
                port("guarded", 1),
                port("real_ok", 1),
                port("done", 1),
            ],
            clock: None,
            step: 1,
            settle: 0,
            vectors: vec![vec![]; 5],
        },
    )
    .unwrap();
    for (tick, row) in report.observations.iter().enumerate() {
        let actual: Vec<_> = row.values.iter().map(|(_, value)| value.as_str()).collect();
        assert_eq!(
            actual,
            [
                "00001001",
                if tick >= 2 { "1" } else { "0" },
                "1",
                "1",
                if tick >= 1 { "1" } else { "0" },
                if tick >= 4 { "1" } else { "0" }
            ],
            "tick {tick}"
        );
    }
    let report = run_digital_verilog(
        "module timed(drive,q); input drive; output q; wire drive; reg q; initial q=0; always @(posedge (drive & ($time>=2))) q=1; endmodule",
        &DigitalStimulus { module:None, inputs:vec![port("drive",1)], outputs:vec![port("q",1)], clock:None, step:1, settle:0, vectors:vectors(&[&["1"],&["0"],&["1"]]) }
    ).unwrap();
    assert_eq!(
        report
            .observations
            .iter()
            .map(|row| row.values[0].1.as_str())
            .collect::<Vec<_>>(),
        ["0", "0", "1"]
    );
}

#[test]
fn repeat_controls_count_each_transition_and_retain_all_expression_baselines() {
    let source = "module repeated(q,blocked,done,computed,real_ok);
      output [7:0] q,blocked; output done,computed,real_ok;
      reg [7:0] q,blocked,data,n; reg done,computed,real_ok,a,b,arm; real r,held;
      initial begin
        q=0; blocked=0; done=0; computed=0; real_ok=0;
        a=0; b=0; arm=0; data=8'h42; n=3; r=0.0; held=0.0;
        q <= repeat (n) @(posedge a or posedge b) data;
        computed <= repeat (3) @(posedge (a & 1'b1) or negedge (a & 1'b1)) 1;
        held <= repeat (2.5) @(r+r) 1.25;
        #1 arm=1; #1 n=1; data=8'h99;
        a=1; b=1; a=0; r=1.0; r=2.0;
        #1 a=1; r=3.0;
        #1 real_ok=(held==1.25);
      end
      initial begin @(posedge arm) blocked = repeat (n) @(posedge a or posedge b) data; done=1; end
      endmodule";
    let report = run_digital_verilog(
        source,
        &DigitalStimulus {
            module: None,
            inputs: vec![],
            outputs: vec![
                port("q", 8),
                port("blocked", 8),
                port("done", 1),
                port("computed", 1),
                port("real_ok", 1),
            ],
            clock: None,
            step: 1,
            settle: 0,
            vectors: vec![vec![]; 5],
        },
    )
    .unwrap();
    for (tick, row) in report.observations.iter().enumerate() {
        let actual: Vec<_> = row.values.iter().map(|(_, value)| value.as_str()).collect();
        let delivered = if tick >= 3 { "01000010" } else { "00000000" };
        assert_eq!(
            actual,
            [
                delivered,
                delivered,
                if tick >= 3 { "1" } else { "0" },
                if tick >= 3 { "1" } else { "0" },
                if tick >= 4 { "1" } else { "0" }
            ],
            "tick {tick}"
        );
    }
}

// ===========================================================================
// Self-checking testbenches: system tasks end to end (design P2 section 2.3)
//
// Expected text is what Icarus Verilog 13.0 printed for the same source
// (`iverilog -g2005`, `vvp -n`), less its own `$finish called at` line, which
// RSpice reports as a finish record instead.
// ===========================================================================

use crate::config::{HdlSeverity, HdlStream};

mod threads;

fn testbench(files: &[(&str, &str)], options: &HdlTestbenchOptions) -> HdlTestbenchReport {
    let sources: Vec<_> = files
        .iter()
        .map(|(path, text)| (std::path::PathBuf::from(path), (*text).to_owned()))
        .collect();
    run_digital_testbench(&sources, options).expect("the testbench runs")
}

fn bench(source: &str, plusargs: &[&str]) -> HdlTestbenchReport {
    testbench(
        &[("tb.v", source)],
        &HdlTestbenchOptions {
            plusargs: plusargs
                .iter()
                .map(|argument| (*argument).to_owned())
                .collect(),
            ..HdlTestbenchOptions::default()
        },
    )
}

fn stdout(report: &HdlTestbenchReport) -> String {
    report
        .records
        .iter()
        .filter(|record| record.stream == HdlStream::Stdout)
        .map(|record| record.text.as_str())
        .collect()
}

/// `$display`/`$write`/`$strobe`/`$monitor`, `%t` under the design precision
/// and under `$timeformat`, `$random(seed)`, both plusarg functions and `%m`.
#[test]
fn a_testbench_prints_what_icarus_prints() {
    let report = bench(
        "`timescale 1ns/100ps\n\
         module tb;\n\
         \x20 reg [7:0] a;\n\
         \x20 integer seed, r, n;\n\
         \x20 initial begin\n\
         \x20   a = 8'd5; seed = 7;\n\
         \x20   $display(\"start a=%d %h %b %o\", a, a, a, a);\n\
         \x20   $write(\"w%0d;\", a);\n\
         \x20   $display;\n\
         \x20   $strobe(\"strobe a=%0d at %t\", a, $time);\n\
         \x20   a = 8'd6;\n\
         \x20   #3 a = 8'd7;\n\
         \x20   $monitor(\"mon a=%0d t=%0t\", a, $time);\n\
         \x20   #2 a = 8'd8;\n\
         \x20   #1 a = 8'd8;\n\
         \x20   $timeformat(-9, 2, \" ns\", 10);\n\
         \x20   #1 $display(\"tf %t|\", $realtime);\n\
         \x20   r = $random(seed);\n\
         \x20   $display(\"r=%0d seed=%0d\", r, seed);\n\
         \x20   if ($test$plusargs(\"VERBOSE\")) $display(\"verbose\");\n\
         \x20   if ($value$plusargs(\"N=%d\", n)) $display(\"n=%0d\", n);\n\
         \x20   $display(\"%m %0s|%s|\", \"ab\", \"cd\");\n\
         \x20   $finish(0);\n\
         \x20 end\n\
         endmodule\n",
        &["+VERBOSE", "+N=42"],
    );
    assert_eq!(
        stdout(&report),
        "start a=  5 05 00000101 005\n\
         w5;\n\
         strobe a=6 at                    0\n\
         mon a=7 t=30\n\
         mon a=8 t=50\n\
         tf    7.00 ns|\n\
         r=-2146999808 seed=483484\n\
         verbose\n\
         n=42\n\
         tb ab|cd|\n"
    );
    assert_eq!(report.end, HdlTestbenchEnd::Finished);
    assert_eq!(report.finish_level, Some(0));
    assert_eq!(report.end_tick, 70);
    assert!((report.end_seconds - 7e-9).abs() < 1e-21);
    let finish = report.records.last().expect("a finish record");
    assert_eq!(
        finish.stream,
        HdlStream::Finish {
            stop: false,
            level: 0
        }
    );
    assert_eq!(
        (finish.origin.file.as_str(), finish.origin.line),
        ("tb.v", 23)
    );
    assert_eq!(finish.origin.scope, "tb");
}

/// Without plusargs neither function matches, and `$value$plusargs` leaves
/// its variable alone.
#[test]
fn unmatched_plusargs_leave_the_variable_unchanged() {
    let report = bench(
        "module tb; integer n; initial begin n = 9;\n\
         if (!$value$plusargs(\"N=%d\", n)) $display(\"no N, n=%0d\", n);\n\
         if (!$test$plusargs(\"V\")) $display(\"no V\"); end endmodule\n",
        &[],
    );
    assert_eq!(stdout(&report), "no N, n=9\nno V\n");
    assert_eq!(report.end, HdlTestbenchEnd::Quiescent);
}

/// IEEE 1364-2005 sections 17.1.1 and 17.9, against Icarus: field widths and
/// flags, unknown digits, the empty argument, the distribution functions and
/// the global `$random` stream; then the severity tasks.
#[test]
fn formats_distributions_and_severities_match_icarus() {
    let report = bench(
        "module sev;\n\
         \x20 integer seed, i;\n\
         \x20 initial begin\n\
         \x20   seed = 1;\n\
         \x20   $display(\"%0d %0d %0d\", $dist_uniform(seed, 0, 99), $dist_uniform(seed, 0, 99), seed);\n\
         \x20   seed = 3;\n\
         \x20   $display(\"%0d %0d\", $dist_normal(seed, 10, 4), $dist_poisson(seed, 5));\n\
         \x20   i = $random;\n\
         \x20   $display(\"%0d\", i);\n\
         \x20   $display(\"[%5d] [%-5d] [%05d] [%h] [%0h] [%e] [%0.3f] [%g] [%c] [%s]\", -3, 7, 42, 12'habc, 12'habc, 1.5, 3.14159, 0.0001, 65, \"hi\");\n\
         \x20   $display(\"[%t] [%0t] [%b] [%d] [%o]\", 1, 2, 4'bx01z, 8'bxxxxxxxx, 6'o7z);\n\
         \x20   $display(1.5, , 3);\n\
         \x20   $warning(\"careful %0d\", 1);\n\
         \x20   $error(\"bad %0d\", 2);\n\
         \x20   $fatal(0, \"boom\");\n\
         \x20   $display(\"never\");\n\
         \x20 end\n\
         endmodule\n",
        &[],
    );
    assert_eq!(
        stdout(&report),
        "0 11 475628535\n\
         9 8\n\
         303379748\n\
         [   -3] [7    ] [00042] [abc] [abc] [1.500000e+00] [3.142] [0.0001] [A] [hi]\n\
         [                   1] [2] [x01z] [  x] [7z]\n\
         1.50000           3\n"
    );
    let diagnostics: Vec<_> = report
        .records
        .iter()
        .filter_map(|record| match record.stream {
            HdlStream::Diagnostic { severity } => {
                Some((severity, record.text.as_str(), record.origin.line))
            }
            _ => None,
        })
        .collect();
    assert_eq!(
        diagnostics,
        [
            (HdlSeverity::Warning, "careful 1", 13),
            (HdlSeverity::Error, "bad 2", 14),
            (HdlSeverity::Fatal, "boom", 15),
        ]
    );
    assert_eq!(report.end, HdlTestbenchEnd::Fatal);
    assert_eq!(report.finish_level, Some(0));
    assert_eq!((report.warnings, report.errors, report.fatals), (1, 1, 1));
}

/// Design P2 section 4.3: `$finish` ends the thread at once, the rest of its
/// tick still runs — other same-tick processes, the nonblocking update it
/// scheduled, the event that update wakes, and the strobe region — and no
/// later tick runs. Icarus prints the same four lines.
#[test]
fn finish_completes_its_tick_and_runs_no_later_one() {
    let report = bench(
        "`timescale 1ns/1ns\n\
         module fin;\n\
         \x20 reg a;\n\
         \x20 initial begin a = 0; #5 a <= 1; $display(\"finish at %0t\", $time); $finish(1); $display(\"never\"); end\n\
         \x20 always @(posedge a) $display(\"posedge a at %0t\", $time);\n\
         \x20 initial #5 $display(\"same tick\");\n\
         \x20 initial #5 $strobe(\"strobe at %0t\", $time);\n\
         \x20 initial #6 $display(\"next tick never\");\n\
         endmodule\n",
        &[],
    );
    assert_eq!(
        stdout(&report),
        "finish at 5\nsame tick\nposedge a at 5\nstrobe at 5\n"
    );
    assert_eq!(report.end, HdlTestbenchEnd::Finished);
    assert_eq!(report.finish_level, Some(1));
    assert_eq!(report.end_tick, 5);
}

/// `$stop` ends a batch run the way `$finish` does, and says it stopped.
#[test]
fn stop_ends_a_batch_run() {
    let report = bench(
        "module s; initial begin #2 $stop; $display(\"never\"); end endmodule\n",
        &[],
    );
    assert_eq!(report.end, HdlTestbenchEnd::Stopped);
    assert_eq!(report.finish_level, Some(1));
    assert!(matches!(
        report.records.last().map(|record| &record.stream),
        Some(HdlStream::Finish {
            stop: true,
            level: 1
        })
    ));
    assert_eq!(stdout(&report), "");
}

/// Design P2 section 2.3: every source is one compilation unit, every module
/// nothing instantiates is a root, and the roots are linked into one design;
/// `%m` of a root is its module name and each record keeps its own file line.
#[test]
fn every_uninstantiated_module_is_a_linked_root() {
    let report = testbench(
        &[
            (
                "zeta.v",
                "`timescale 1ns/1ns\nmodule zeta;\n  initial #1 $display(\"%m z\");\nendmodule\n",
            ),
            (
                "alpha.v",
                "module alpha;\n  initial $display(\"%m a\");\n  initial #2 $finish;\nendmodule\n",
            ),
        ],
        &HdlTestbenchOptions::default(),
    );
    assert_eq!(stdout(&report), "alpha a\nzeta z\n");
    let origins: Vec<_> = report
        .records
        .iter()
        .filter(|record| record.stream == HdlStream::Stdout)
        .map(|record| (record.origin.file.as_str(), record.origin.line))
        .collect();
    assert_eq!(origins, [("alpha.v", 2), ("zeta.v", 3)]);
    assert_eq!(report.end, HdlTestbenchEnd::Finished);
    assert_eq!(report.end_tick, 2, "the timescale carried into alpha.v");
}

/// Design P2 section 2.4: the reverse schedule starts same-tick activations
/// in the reverse of source order, which is what exposes a racy test.
#[test]
fn the_reverse_schedule_reverses_same_tick_starts() {
    let source = "module r; initial $display(\"one\"); initial $display(\"two\"); endmodule\n";
    assert_eq!(stdout(&bench(source, &[])), "one\ntwo\n");
    let reversed = testbench(
        &[("r.v", source)],
        &HdlTestbenchOptions {
            schedule: HdlSchedule::ReverseActive,
            ..HdlTestbenchOptions::default()
        },
    );
    assert_eq!(stdout(&reversed), "two\none\n");
}

/// A free-running clock ends at the tick ceiling rather than running on.
#[test]
fn a_free_running_testbench_ends_at_the_tick_ceiling() {
    let report = testbench(
        &[("c.v", "module c; reg k = 0; always #1 k = ~k; endmodule\n")],
        &HdlTestbenchOptions {
            tick_ceiling: Some(10),
            ..HdlTestbenchOptions::default()
        },
    );
    assert_eq!(report.end, HdlTestbenchEnd::TickCeiling);
    assert_eq!(report.end_tick, 10);
}

/// The file and waveform tasks compile and are refused at run time as not
/// supported yet; so are the options whose behaviour has not landed. The text
/// names the feature, never a plan lane.
#[test]
fn unlanded_tasks_and_options_are_refused_as_not_supported_yet() {
    let error = run_digital_testbench(
        &[(
            "f.v".into(),
            "module f; integer fd; initial fd = $fopen(\"x.txt\"); endmodule\n".into(),
        )],
        &HdlTestbenchOptions::default(),
    )
    .expect_err("file I/O is not supported yet");
    let DigitalRunError::Unsupported(refusal) = &error else {
        panic!("expected a typed capability refusal, got {error:?}");
    };
    assert_eq!(refusal.capability, "hdl.system_task.file_io");
    let text = error.to_string();
    assert!(
        text.contains("HDL file and waveform I/O (`$fopen`) is not supported yet"),
        "{text}"
    );
    assert!(!text.contains("lane"), "{text}");
    let error = run_digital_testbench(
        &[(
            "f.v".into(),
            "module f; initial $display; endmodule\n".into(),
        )],
        &HdlTestbenchOptions {
            delay_select: crate::config::HdlDelaySelect::Max,
            ..HdlTestbenchOptions::default()
        },
    )
    .expect_err("delay selection is not supported yet");
    let DigitalRunError::Unsupported(refusal) = &error else {
        panic!("expected a typed capability refusal, got {error:?}");
    };
    assert_eq!(refusal.capability, "hdl.option.delay_select");
    let text = error.to_string();
    assert!(
        text.contains("min:typ:max delay selection is not supported yet"),
        "{text}"
    );
    assert!(!text.contains("lane"), "{text}");
}
