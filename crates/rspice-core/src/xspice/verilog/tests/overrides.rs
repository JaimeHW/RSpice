//! `force`/`release` and procedural `assign`/`deassign` end to end (IEEE
//! 1364-2005 section 9.3; design P2 section 4.2).
//!
//! Expected text is what Icarus Verilog 13.0 printed for the same source
//! (`iverilog -g2005`, `vvp -n`), an RSpice-authored probe.

use super::*;
use crate::xspice::event_scheduler::{SchedulerLimits, TimeResolution};
use host::checkpoint::HostCheckpointLimits;

/// A forced net shows the forced value and re-resolves when released; a
/// forced variable ignores procedural writes and keeps the forced value
/// after release; forced net bits compose with the bits the net's driver
/// still decides; an `assign` follows its operand and overrides procedural
/// writes, a `force` overrides the `assign`, and releasing the force uncovers
/// the `assign`'s current value; after `deassign` the variable keeps its value
/// until written; a concatenation and a real variable are forced as one.
#[test]
fn procedural_continuous_assignments_run_as_icarus_runs_them() {
    let report = bench(
        r#"`timescale 1ns/1ns
module top;
  reg r; wire w; assign w = r;
  reg [3:0] a; reg [3:0] v; wire [3:0] bus; assign bus = a;
  reg [3:0] t; reg q; real x, y;
  always @(bus) if ($time > 0) $display("%0t bus->%b", $time, bus);
  initial begin
    r = 0; #2 force w = 1; #1 $display("forced w=%b", w); release w; #0 $display("released w=%b", w);
    force r = 1; r = 0; #1 $display("forced r=%b", r); release r; $display("released r=%b", r); r = 0; $display("after assign r=%b", r);
    a = 4'b0101;
    #1 force bus[2:1] = 2'b11;
    #1 $display("bus=%b", bus);
    force bus[1] = 1'b0;
    #1 $display("bus=%b", bus);
    a = 4'b1010;
    #1 $display("bus=%b", bus);
    release bus[2];
    #1 $display("bus=%b", bus);
    release bus[1];
    #1 $display("bus=%b", bus);
    t = 4'd3; v = 4'd5; q = 0;
    assign v = t;
    #1 $display("v=%0d", v);
    t = 4'd7; v = 4'd0;
    #1 $display("v=%0d", v);
    force v = 4'd9;
    t = 4'd6;
    #1 $display("v=%0d", v);
    release v;
    #1 $display("v=%0d", v);
    deassign v;
    t = 4'd1;
    #1 $display("v=%0d", v);
    v = 4'd2;
    #1 $display("v=%0d", v);
    force {q, v} = 5'b11100;
    #1 $display("q=%b v=%b", q, v);
    release {q, v};
    q = 0;
    #1 $display("q=%b v=%b", q, v);
    y = 7.5; force x = y;
    #1 $display("x=%f", x);
    y = 2.0;
    #1 $display("x=%f", x);
    release x; x = 0.25;
    #1 $display("x=%f", x);
    $finish(0);
  end
endmodule
"#,
        &[],
    );
    assert_eq!(
        stdout(&report),
        "forced w=1\n\
         released w=0\n\
         forced r=1\n\
         released r=1\n\
         after assign r=0\n\
         4 bus->0101\n\
         5 bus->0111\n\
         bus=0111\n\
         6 bus->0101\n\
         bus=0101\n\
         7 bus->1100\n\
         bus=1100\n\
         8 bus->1000\n\
         bus=1000\n\
         9 bus->1010\n\
         bus=1010\n\
         v=3\n\
         v=7\n\
         v=9\n\
         v=6\n\
         v=6\n\
         v=2\n\
         q=1 v=1100\n\
         q=0 v=1100\n\
         x=7.500000\n\
         x=2.000000\n\
         x=0.250000\n"
    );
}

/// Section 9.3: the right-hand side of a procedural continuous assignment is
/// continuous, whatever expression it is. Icarus 13.0 evaluates an operator
/// or select right-hand side only once and says so ("sorry: ... will only be
/// evaluated once"), so this reading is the standard's, not the oracle's.
#[test]
fn an_expression_right_hand_side_follows_its_operands() {
    let report = bench(
        r#"`timescale 1ns/1ns
module top;
  reg [3:0] s, v; wire [3:0] n; assign n = 4'd0; real x; integer k;
  initial begin
    s = 4'd2; k = 3;
    force n = s + 4'd1; assign v = {s[1:0], s[3:2]}; force x = k * 1.5;
    #1 $display("n=%0d v=%b x=%f", n, v, x);
    s = 4'd6; k = 4;
    #1 $display("n=%0d v=%b x=%f", n, v, x);
    release n; deassign v; release x;
    s = 4'd1;
    #1 $display("n=%0d v=%b x=%f", n, v, x);
  end
endmodule
"#,
        &[],
    );
    assert_eq!(
        stdout(&report),
        "n=3 v=1000 x=4.500000\n\
         n=7 v=1001 x=6.000000\n\
         n=0 v=1001 x=6.000000\n"
    );
}

/// A user function in a right-hand side subscribes it to the module signals
/// the function body reads, through a nested call, and not to the function's
/// own argument and result storage. Neither oracle runs this continuously:
/// Icarus 13.0 evaluates it once ("sorry"), and Verilator 5.048 evaluates a
/// function-call right-hand side once while it follows the bodies written
/// inline (`a + g`, `a + g + g`). The expected text is that inline form's,
/// which Verilator printed for an RSpice-authored probe.
#[test]
fn a_function_right_hand_side_follows_the_signals_the_function_reads() {
    let report = bench(
        r#"`timescale 1ns/1ns
module top;
  reg [3:0] g, a, v; wire [3:0] w; assign w = 4'd0;
  function [3:0] plus_g;
    input [3:0] x;
    plus_g = x + g;
  endfunction
  function [3:0] twice;
    input [3:0] x;
    twice = plus_g(x) + g;
  endfunction
  initial begin
    g = 4'd1; a = 4'd2;
    force v = plus_g(a); force w = twice(a);
    #1 $display("v=%0d w=%0d", v, w);
    g = 4'd3;
    #1 $display("v=%0d w=%0d", v, w);
    a = 4'd4;
    #1 $display("v=%0d w=%0d", v, w);
  end
endmodule
"#,
        &[],
    );
    assert_eq!(stdout(&report), "v=3 w=4\nv=5 w=8\nv=7 w=10\n");
}

/// A right-hand side that reads what it overrides never settles; the run
/// ends naming the statement rather than hanging.
#[test]
fn a_force_that_feeds_itself_is_refused_by_name() {
    let error = run_digital_testbench(
        &[(
            "tb.v".into(),
            "module top; reg a; initial begin a = 0; force a = ~a; #1 $finish; end endmodule\n"
                .into(),
        )],
        &HdlTestbenchOptions::default(),
    )
    .err()
    .map(|error| error.to_string())
    .unwrap_or_default();
    assert!(
        error.contains("procedural continuous assignment") && error.contains("did not settle"),
        "{error}"
    );
}

fn host(source: &str) -> DigitalHost {
    let plan = rspice_veriloga::VerilogACompiler::default()
        .compile_canonical_ir(source)
        .unwrap()
        .digital;
    DigitalHost::new(
        &plan,
        TimeResolution::new(-9).unwrap(),
        SchedulerLimits::default(),
    )
}

/// Design P2 section 5.4: an active `force` and `assign` survive a host image
/// round trip in its `forces` section, including the value a net resolves to
/// beneath the force, and the restored host continues exactly as the original.
#[test]
fn a_host_image_carries_the_overrides_in_effect() {
    const SOURCE: &str = r#"`timescale 1ns/1ns
module top;
  reg [3:0] a, t, v; wire [3:0] w; assign w = a;
  initial begin
    a = 4'd1; t = 4'd2;
    force w[1:0] = t[1:0];
    assign v = t;
    #5 a = 4'd12; t = 4'd3;
    #5 release w; deassign v;
    #5 v = 4'd0;
  end
endmodule
"#;
    let mut original = host(SOURCE);
    original.start().unwrap();
    original.advance_to(3).unwrap();
    let w = original.signal("w").unwrap();
    let v = original.signal("v").unwrap();
    assert_eq!(original.read(w).unwrap().to_u64(), Some(0b0010));
    let saved = original
        .checkpoint(HostCheckpointLimits::default())
        .unwrap();
    let text = serde_json::to_string(&saved).unwrap();
    assert!(text.contains("\"forces\""), "{text}");
    let transported: host::checkpoint::HostCheckpoint = serde_json::from_str(&text).unwrap();
    let mut restored = host(SOURCE);
    restored
        .restore_checkpoint(&transported, HostCheckpointLimits::default())
        .unwrap();
    assert_eq!(
        restored
            .checkpoint(HostCheckpointLimits::default())
            .unwrap(),
        saved
    );
    for tick in [7, 12, 20] {
        original.advance_to(tick).unwrap();
        restored.advance_to(tick).unwrap();
        for signal in [w, v] {
            assert_eq!(original.read(signal), restored.read(signal), "tick {tick}");
        }
    }
    // Forced bits 1:0 follow t (3); bits 3:2 follow a (12) beneath the force.
    let mut probe = host(SOURCE);
    probe.start().unwrap();
    probe.advance_to(7).unwrap();
    assert_eq!(probe.read(w).unwrap().to_u64(), Some(0b1111));
    assert_eq!(probe.read(v).unwrap().to_u64(), Some(3));
    assert_eq!(original.read(w).unwrap().to_u64(), Some(12));
    assert_eq!(original.read(v).unwrap().to_u64(), Some(0));

    // A tampered owner is refused before anything is installed.
    let mut value = serde_json::to_value(&saved).unwrap();
    value["forces"]["image"]["signals"][0]["owners"][0] = serde_json::json!(99);
    let tampered: host::checkpoint::HostCheckpoint = serde_json::from_value(value).unwrap();
    assert!(
        host(SOURCE)
            .restore_checkpoint(&tampered, HostCheckpointLimits::default())
            .is_err()
    );
}
