//! Delays on continuous assignments, gates, primitives and nets (IEEE
//! 1364-2005 sections 6.1.3, 7.14 and 8.6), end to end.
//!
//! Expected text is what Icarus Verilog 13.0 printed for the same source
//! (`iverilog -g2005`, `vvp -n`) in an RSpice-authored probe, except where a
//! test says it is a closed form: Icarus does not run net delays ("sorry:
//! Net delays not supported").

use super::*;

/// Section 7.14: a change to 1 takes the rise delay, to 0 the fall delay, to
/// `z` the turn-off delay and to `x` the least of them; two delays turn off
/// in the lesser. A vector takes the longest delay any of its bits' changes
/// does, and a delayed output is `x` until its first value arrives.
#[test]
fn transitions_take_their_rise_fall_and_turn_off_delays() {
    let report = bench(
        r#"`timescale 1ns/1ns
module t3;
  reg a, en;
  reg [3:0] v;
  wire y1, y2, y3, y4, y5;
  wire [3:0] w;
  assign #(2,5) y1 = a;
  assign #(2,5,7) y2 = en ? a : 1'bz;
  bufif1 #(3,4,6) g1(y3, a, en);
  and #(1,3) g2(y4, a, en);
  not #(4) g3(y5, a);
  assign #(2,5,7) w = v;
  initial begin
    $monitor("%0t a=%b en=%b v=%b y1=%b y2=%b y3=%b y4=%b y5=%b w=%b", $time, a, en, v, y1, y2, y3, y4, y5, w);
    a = 0; en = 1; v = 4'b0000;
    #10 a = 1; v = 4'b0011;
    #10 a = 0; v = 4'b0010;
    #10 en = 0; v = 4'b0000;
    #10 a = 1'bx; en = 1; v = 4'bzzzz;
    #10 a = 1; v = 4'b10x1;
    #10 en = 1'bx; v = 4'b1z00;
    #10 a = 1'bz; en = 1; v = 4'bxxxx;
    #10 $finish;
  end
endmodule
"#,
        &[],
    );
    assert_eq!(
        stdout(&report),
        "0 a=0 en=1 v=0000 y1=x y2=x y3=x y4=x y5=x w=xxxx\n\
         3 a=0 en=1 v=0000 y1=x y2=x y3=x y4=0 y5=x w=xxxx\n\
         4 a=0 en=1 v=0000 y1=x y2=x y3=0 y4=0 y5=1 w=xxxx\n\
         5 a=0 en=1 v=0000 y1=0 y2=0 y3=0 y4=0 y5=1 w=0000\n\
         10 a=1 en=1 v=0011 y1=0 y2=0 y3=0 y4=0 y5=1 w=0000\n\
         11 a=1 en=1 v=0011 y1=0 y2=0 y3=0 y4=1 y5=1 w=0000\n\
         12 a=1 en=1 v=0011 y1=1 y2=1 y3=0 y4=1 y5=1 w=0011\n\
         13 a=1 en=1 v=0011 y1=1 y2=1 y3=1 y4=1 y5=1 w=0011\n\
         14 a=1 en=1 v=0011 y1=1 y2=1 y3=1 y4=1 y5=0 w=0011\n\
         20 a=0 en=1 v=0010 y1=1 y2=1 y3=1 y4=1 y5=0 w=0011\n\
         23 a=0 en=1 v=0010 y1=1 y2=1 y3=1 y4=0 y5=0 w=0011\n\
         24 a=0 en=1 v=0010 y1=1 y2=1 y3=0 y4=0 y5=1 w=0011\n\
         25 a=0 en=1 v=0010 y1=0 y2=0 y3=0 y4=0 y5=1 w=0010\n\
         30 a=0 en=0 v=0000 y1=0 y2=0 y3=0 y4=0 y5=1 w=0010\n\
         35 a=0 en=0 v=0000 y1=0 y2=0 y3=0 y4=0 y5=1 w=0000\n\
         36 a=0 en=0 v=0000 y1=0 y2=0 y3=z y4=0 y5=1 w=0000\n\
         37 a=0 en=0 v=0000 y1=0 y2=z y3=z y4=0 y5=1 w=0000\n\
         40 a=x en=1 v=zzzz y1=0 y2=z y3=z y4=0 y5=1 w=0000\n\
         41 a=x en=1 v=zzzz y1=0 y2=z y3=z y4=x y5=1 w=0000\n\
         42 a=x en=1 v=zzzz y1=x y2=x y3=z y4=x y5=1 w=0000\n\
         43 a=x en=1 v=zzzz y1=x y2=x y3=x y4=x y5=1 w=0000\n\
         44 a=x en=1 v=zzzz y1=x y2=x y3=x y4=x y5=x w=0000\n\
         47 a=x en=1 v=zzzz y1=x y2=x y3=x y4=x y5=x w=zzzz\n\
         50 a=1 en=1 v=10x1 y1=x y2=x y3=x y4=x y5=x w=zzzz\n\
         51 a=1 en=1 v=10x1 y1=x y2=x y3=x y4=1 y5=x w=zzzz\n\
         52 a=1 en=1 v=10x1 y1=1 y2=1 y3=x y4=1 y5=x w=zzzz\n\
         53 a=1 en=1 v=10x1 y1=1 y2=1 y3=1 y4=1 y5=x w=zzzz\n\
         54 a=1 en=1 v=10x1 y1=1 y2=1 y3=1 y4=1 y5=0 w=zzzz\n\
         55 a=1 en=1 v=10x1 y1=1 y2=1 y3=1 y4=1 y5=0 w=10x1\n\
         60 a=1 en=x v=1z00 y1=1 y2=1 y3=1 y4=1 y5=0 w=10x1\n\
         61 a=1 en=x v=1z00 y1=1 y2=1 y3=1 y4=x y5=0 w=10x1\n\
         62 a=1 en=x v=1z00 y1=1 y2=x y3=1 y4=x y5=0 w=10x1\n\
         63 a=1 en=x v=1z00 y1=1 y2=x y3=x y4=x y5=0 w=10x1\n\
         67 a=1 en=x v=1z00 y1=1 y2=x y3=x y4=x y5=0 w=1z00\n\
         70 a=z en=1 v=xxxx y1=1 y2=x y3=x y4=x y5=0 w=1z00\n\
         72 a=z en=1 v=xxxx y1=z y2=x y3=x y4=x y5=0 w=xxxx\n\
         74 a=z en=1 v=xxxx y1=z y2=x y3=x y4=x y5=x w=xxxx\n\
         77 a=z en=1 v=xxxx y1=z y2=z y3=x y4=x y5=x w=xxxx\n"
    );
}

/// Sections 6.1.3 and 7.14, the inertial rule: a pulse shorter than the
/// delay is swallowed; a re-evaluation to the value already pending keeps it
/// and its due time; any other value replaces the pending one, `x` replacing
/// a pending 0 at the least delay.
#[test]
fn a_delay_is_inertial() {
    let report = bench(
        r#"`timescale 1ns/1ns
module t6;
  reg a, b;
  wire y, z;
  assign #5 y = a | b;
  assign #(4,6) z = a;
  initial begin
    $monitor("%0t a=%b b=%b y=%b z=%b", $time, a, b, y, z);
    a = 0; b = 0;
    #10 a = 1; #2 a = 0;
    #10 a = 1; #2 b = 1;
    #10 a = 0; b = 0; #2 a = 1'bx;
    #10 a = 0; #3 a = 1; #1 a = 0;
    #10 a = 1; #4 a = 0; #4 a = 1;
    #10 a = 0; #1 a = 1; #1 a = 0;
    #20 $finish;
  end
endmodule
"#,
        &[],
    );
    assert_eq!(
        stdout(&report),
        "0 a=0 b=0 y=x z=x\n\
         5 a=0 b=0 y=0 z=x\n\
         6 a=0 b=0 y=0 z=0\n\
         10 a=1 b=0 y=0 z=0\n\
         12 a=0 b=0 y=0 z=0\n\
         22 a=1 b=0 y=0 z=0\n\
         24 a=1 b=1 y=0 z=0\n\
         26 a=1 b=1 y=0 z=1\n\
         27 a=1 b=1 y=1 z=1\n\
         34 a=0 b=0 y=1 z=1\n\
         36 a=x b=0 y=1 z=1\n\
         40 a=x b=0 y=1 z=x\n\
         41 a=x b=0 y=x z=x\n\
         46 a=0 b=0 y=x z=x\n\
         49 a=1 b=0 y=x z=x\n\
         50 a=0 b=0 y=x z=x\n\
         55 a=0 b=0 y=0 z=x\n\
         56 a=0 b=0 y=0 z=0\n\
         60 a=1 b=0 y=0 z=0\n\
         64 a=0 b=0 y=0 z=1\n\
         68 a=1 b=0 y=0 z=1\n\
         73 a=1 b=0 y=1 z=1\n\
         78 a=0 b=0 y=1 z=1\n\
         79 a=1 b=0 y=1 z=1\n\
         80 a=0 b=0 y=1 z=1\n\
         85 a=0 b=0 y=0 z=1\n\
         86 a=0 b=0 y=0 z=0\n"
    );
}

/// Section 6.1.3: a net declaration assignment's delay is its driver's; a
/// primitive instance's (section 8.6) is its output's, with a gate's rules;
/// a controlled bidirectional switch turns on after its first delay and off
/// after its second, inertially (section 7.14).
#[test]
fn declaration_primitive_and_switch_delays_are_their_drivers() {
    let report = bench(
        r#"`timescale 1ns/1ns
primitive inv (o, i);
  output o; input i;
  table 0 : 1; 1 : 0; endtable
endprimitive
primitive latch (q, d, en);
  output q; reg q; input d, en;
  table
    // d en : q : q+
       ?  0 : ? : - ;
       0  1 : ? : 0 ;
       1  1 : ? : 1 ;
  endtable
endprimitive
module t8;
  reg a, en;
  wire y, q;
  wire #3 m = a;
  inv #(2, 3) u1 (y, a);
  latch #4 u2 (q, a, en);
  wire p, n;
  tranif1 #(5, 2) s (p, n, en);
  assign p = a;
  initial begin
    $monitor("%0t a=%b en=%b m=%b y=%b q=%b n=%b", $time, a, en, m, y, q, n);
    a = 0; en = 1;
    #10 a = 1;
    #10 en = 0;
    #1 a = 0;
    #1 a = 1;
    #10 en = 1;
    #1 en = 0;
    #10 $finish;
  end
endmodule
"#,
        &[],
    );
    assert_eq!(
        stdout(&report),
        "0 a=0 en=1 m=x y=x q=x n=x\n\
         2 a=0 en=1 m=x y=1 q=x n=x\n\
         3 a=0 en=1 m=0 y=1 q=x n=x\n\
         4 a=0 en=1 m=0 y=1 q=0 n=x\n\
         5 a=0 en=1 m=0 y=1 q=0 n=0\n\
         10 a=1 en=1 m=0 y=1 q=0 n=1\n\
         13 a=1 en=1 m=1 y=0 q=0 n=1\n\
         14 a=1 en=1 m=1 y=0 q=1 n=1\n\
         20 a=1 en=0 m=1 y=0 q=1 n=1\n\
         21 a=0 en=0 m=1 y=0 q=1 n=0\n\
         22 a=1 en=0 m=1 y=0 q=1 n=z\n\
         32 a=1 en=1 m=1 y=0 q=1 n=z\n\
         33 a=1 en=0 m=1 y=0 q=1 n=z\n"
    );
}

/// Section 6.1.3, a closed form (Icarus does not run net delays): a net
/// delay is a second inertial stage on what the net's drivers resolve to, so
/// a change arrives after both delays, and a change that the drivers undo
/// within the net's delay never reaches the net.
#[test]
fn a_net_delay_is_a_second_inertial_stage() {
    let report = bench(
        r#"`timescale 1ns/1ns
module top;
  reg a, b;
  wire #3 n;
  wire #(2, 4) w;
  assign #2 n = a;
  assign w = a;
  assign w = b;
  initial begin
    $monitor("%0t a=%b b=%b n=%b w=%b", $time, a, b, n, w);
    a = 0; b = 0;
    #10 a = 1;
    #10 a = 0; #1 a = 1;
    #10 b = 1'bz; a = 0;
    #10 $finish;
  end
endmodule
"#,
        &[],
    );
    // n: the driver's 0 arrives at 2 and the net's at 5; a's rise at 10
    // reaches the driver at 12 and the net at 15; the one-unit pulse at 20
    // is swallowed by the driver; a's fall at 31 reaches the driver at 33 and
    // the net at 36. w: both drivers resolve to 0 at 0, arriving at the net's
    // fall delay, 4; a's rise against b's 0 resolves to x at 10, which the
    // net takes at its least delay, 2; the 0 the drivers resolve to from 20
    // to 21 is swallowed by the net; at 31 they resolve to 0 (b gone to z, a
    // to 0), arriving at the fall delay, 35.
    assert_eq!(
        stdout(&report),
        "0 a=0 b=0 n=x w=x\n\
         4 a=0 b=0 n=x w=0\n\
         5 a=0 b=0 n=0 w=0\n\
         10 a=1 b=0 n=0 w=0\n\
         12 a=1 b=0 n=0 w=x\n\
         15 a=1 b=0 n=1 w=x\n\
         20 a=0 b=0 n=1 w=x\n\
         21 a=1 b=0 n=1 w=x\n\
         31 a=0 b=z n=1 w=x\n\
         35 a=0 b=z n=1 w=0\n\
         36 a=0 b=z n=0 w=0\n"
    );
}

/// Section 6.1.3: a delay is fixed at elaboration, so one that reads a
/// variable is refused, and so is one on a driver of a real net.
#[test]
fn a_delay_that_is_not_a_constant_is_refused() {
    let sources = vec![(
        std::path::PathBuf::from("tb.v"),
        "module top;\n  reg [3:0] d; reg a;\n  wire y;\n  assign #(d) y = a;\nendmodule\n"
            .to_owned(),
    )];
    let error = run_digital_testbench(&sources, &HdlTestbenchOptions::default())
        .expect_err("a delay read from a variable is refused")
        .to_string();
    assert!(error.contains("not a constant expression"), "{error}");
}
