//! Module path delays of `specify` blocks (IEEE 1364-2005 section 14), end to
//! end.
//!
//! Expected text is what Icarus Verilog 13.0 printed for the same source
//! (`iverilog -gspecify`, `vvp -n`) in an RSpice-authored probe, and without
//! `-gspecify` for the run that leaves path delays out.

use super::*;

const DISTRIBUTED_AND_SEVERAL_PATHS: &str = r#"`timescale 1ns/1ns
module buf_p(output y, input a);
  assign y = a;
  specify (a => y) = (3, 5); endspecify
endmodule
module dist(output y, input a);
  assign #4 y = a;
  specify (a => y) = 2; endspecify
endmodule
module dist2(output y, input a);
  assign #2 y = a;
  specify (a => y) = 6; endspecify
endmodule
module two(output y, input a, input b);
  assign y = a ^ b;
  specify (a => y) = 2; (b => y) = 7; endspecify
endmodule
module t;
  reg a, b;
  wire y1, y2, y3, y4;
  buf_p u1(y1, a);
  dist u2(y2, a);
  dist2 u3(y3, a);
  two u4(y4, a, b);
  initial begin
    $monitor("%0t a=%b b=%b y1=%b y2=%b y3=%b y4=%b", $time, a, b, y1, y2, y3, y4);
    a = 0; b = 0;
    #10 a = 1;
    #10 a = 0;
    #10 b = 1;
    #10 a = 1; b = 0;
    #10 a = 0; #2 a = 1;
    #10 a = 0; #6 a = 1;
    #10 b = 1; #3 a = 0;
    #10 a = 1'bx;
    #10 a = 0;
    #20 $finish;
  end
endmodule
"#;

/// Section 14.4: with a distributed delay behind the destination, the larger
/// of it and the path delay is the one seen (`y2` is 4 behind `a`, `y3` 6).
/// A path is inertial (a pulse of 2 never reaches `y1`, and the pulse of 2
/// that `two` passes is exactly its delay). Of two paths to one output the
/// one from the input that changed last decides (`b` gives `y4` 7, `a` 2),
/// and a change to and from `x` takes the delays section 14.3.2 derives.
#[test]
fn a_path_delay_is_the_larger_and_follows_the_latest_source() {
    let report = bench(DISTRIBUTED_AND_SEVERAL_PATHS, &[]);
    assert_eq!(
        stdout(&report),
        "\
0 a=0 b=0 y1=x y2=x y3=x y4=x
2 a=0 b=0 y1=x y2=x y3=x y4=0
4 a=0 b=0 y1=x y2=0 y3=x y4=0
5 a=0 b=0 y1=0 y2=0 y3=x y4=0
6 a=0 b=0 y1=0 y2=0 y3=0 y4=0
10 a=1 b=0 y1=0 y2=0 y3=0 y4=0
12 a=1 b=0 y1=0 y2=0 y3=0 y4=1
13 a=1 b=0 y1=1 y2=0 y3=0 y4=1
14 a=1 b=0 y1=1 y2=1 y3=0 y4=1
16 a=1 b=0 y1=1 y2=1 y3=1 y4=1
20 a=0 b=0 y1=1 y2=1 y3=1 y4=1
22 a=0 b=0 y1=1 y2=1 y3=1 y4=0
24 a=0 b=0 y1=1 y2=0 y3=1 y4=0
25 a=0 b=0 y1=0 y2=0 y3=1 y4=0
26 a=0 b=0 y1=0 y2=0 y3=0 y4=0
30 a=0 b=1 y1=0 y2=0 y3=0 y4=0
37 a=0 b=1 y1=0 y2=0 y3=0 y4=1
40 a=1 b=0 y1=0 y2=0 y3=0 y4=1
43 a=1 b=0 y1=1 y2=0 y3=0 y4=1
44 a=1 b=0 y1=1 y2=1 y3=0 y4=1
46 a=1 b=0 y1=1 y2=1 y3=1 y4=1
50 a=0 b=0 y1=1 y2=1 y3=1 y4=1
52 a=1 b=0 y1=1 y2=1 y3=1 y4=0
54 a=1 b=0 y1=1 y2=1 y3=1 y4=1
62 a=0 b=0 y1=1 y2=1 y3=1 y4=1
64 a=0 b=0 y1=1 y2=1 y3=1 y4=0
66 a=0 b=0 y1=1 y2=0 y3=1 y4=0
67 a=0 b=0 y1=0 y2=0 y3=1 y4=0
68 a=1 b=0 y1=0 y2=0 y3=0 y4=0
70 a=1 b=0 y1=0 y2=0 y3=0 y4=1
71 a=1 b=0 y1=1 y2=0 y3=0 y4=1
72 a=1 b=0 y1=1 y2=1 y3=0 y4=1
74 a=1 b=0 y1=1 y2=1 y3=1 y4=1
78 a=1 b=1 y1=1 y2=1 y3=1 y4=1
81 a=0 b=1 y1=1 y2=1 y3=1 y4=1
85 a=0 b=1 y1=1 y2=0 y3=1 y4=1
86 a=0 b=1 y1=0 y2=0 y3=1 y4=1
87 a=0 b=1 y1=0 y2=0 y3=0 y4=1
91 a=x b=1 y1=0 y2=0 y3=0 y4=1
93 a=x b=1 y1=0 y2=0 y3=0 y4=x
94 a=x b=1 y1=x y2=0 y3=0 y4=x
95 a=x b=1 y1=x y2=x y3=0 y4=x
97 a=x b=1 y1=x y2=x y3=x y4=x
101 a=0 b=1 y1=x y2=x y3=x y4=x
103 a=0 b=1 y1=x y2=x y3=x y4=1
105 a=0 b=1 y1=x y2=0 y3=x y4=1
106 a=0 b=1 y1=0 y2=0 y3=x y4=1
107 a=0 b=1 y1=0 y2=0 y3=0 y4=1
"
    );
}

/// `hdl_path_delays=no`: every path is left out, as Icarus Verilog runs
/// the same source without `-gspecify`; the distributed delays remain.
#[test]
fn a_run_without_path_delays_keeps_only_the_distributed_ones() {
    let report = testbench(
        &[("tb.v", DISTRIBUTED_AND_SEVERAL_PATHS)],
        &HdlTestbenchOptions {
            path_delays: false,
            ..HdlTestbenchOptions::default()
        },
    );
    assert_eq!(
        stdout(&report),
        "\
0 a=0 b=0 y1=0 y2=x y3=x y4=0
2 a=0 b=0 y1=0 y2=x y3=0 y4=0
4 a=0 b=0 y1=0 y2=0 y3=0 y4=0
10 a=1 b=0 y1=1 y2=0 y3=0 y4=1
12 a=1 b=0 y1=1 y2=0 y3=1 y4=1
14 a=1 b=0 y1=1 y2=1 y3=1 y4=1
20 a=0 b=0 y1=0 y2=1 y3=1 y4=0
22 a=0 b=0 y1=0 y2=1 y3=0 y4=0
24 a=0 b=0 y1=0 y2=0 y3=0 y4=0
30 a=0 b=1 y1=0 y2=0 y3=0 y4=1
40 a=1 b=0 y1=1 y2=0 y3=0 y4=1
42 a=1 b=0 y1=1 y2=0 y3=1 y4=1
44 a=1 b=0 y1=1 y2=1 y3=1 y4=1
50 a=0 b=0 y1=0 y2=1 y3=1 y4=0
52 a=1 b=0 y1=1 y2=1 y3=0 y4=1
54 a=1 b=0 y1=1 y2=1 y3=1 y4=1
62 a=0 b=0 y1=0 y2=1 y3=1 y4=0
64 a=0 b=0 y1=0 y2=1 y3=0 y4=0
66 a=0 b=0 y1=0 y2=0 y3=0 y4=0
68 a=1 b=0 y1=1 y2=0 y3=0 y4=1
70 a=1 b=0 y1=1 y2=0 y3=1 y4=1
72 a=1 b=0 y1=1 y2=1 y3=1 y4=1
78 a=1 b=1 y1=1 y2=1 y3=1 y4=0
81 a=0 b=1 y1=0 y2=1 y3=1 y4=1
83 a=0 b=1 y1=0 y2=1 y3=0 y4=1
85 a=0 b=1 y1=0 y2=0 y3=0 y4=1
91 a=x b=1 y1=x y2=0 y3=0 y4=x
93 a=x b=1 y1=x y2=0 y3=x y4=x
95 a=x b=1 y1=x y2=x y3=x y4=x
101 a=0 b=1 y1=0 y2=x y3=x y4=1
103 a=0 b=1 y1=0 y2=x y3=0 y4=1
105 a=0 b=1 y1=0 y2=0 y3=0 y4=1
"
    );
}

/// Sections 14.2.4 and 14.2.5: a state-dependent path counts while its
/// condition is 1 or `x` (section 14.2.4.1); with no path counting the
/// change is not delayed; `ifnone` stands in when no conditional path to its
/// output counts; an edge-sensitive path counts only its edge, and on a
/// variable output port it delays the net the port drives; a polarity
/// changes no delay. A specparam names a path's delay. Expected values are
/// Icarus Verilog's (`-gspecify`) except while `s` is `x` (41 to 61), which
/// are the LRM's closed form: Icarus reads an `x` condition as false, and
/// gives `y1` and `y2` 1 at 51 and `y3` 1 at 56 (its `ifnone` delay of 5);
/// here both of `cond`'s paths count (the least, 2, gives 53), `nopath`'s
/// counts (55), and `ifn`'s conditional path stands `ifnone` aside (53).
#[test]
fn conditional_ifnone_edge_sensitive_and_polar_paths() {
    let report = bench(
        r#"`timescale 1ns/1ns
module cond(output y, input a, input s);
  assign y = a;
  specify
    specparam fast = 2, slow = 6;
    if (s) (a => y) = fast;
    if (!s) (a => y) = slow;
  endspecify
endmodule
module nopath(output y, input a, input s);
  assign y = a;
  specify
    if (s) (a => y) = 4;
  endspecify
endmodule
module ifn(output y, input a, input s);
  assign y = a;
  specify
    if (s) (a => y) = 2;
    ifnone (a => y) = 5;
  endspecify
endmodule
module dff(output reg q, input c, input d);
  always @(posedge c) q <= d;
  specify
    (posedge c => (q +: d)) = (3, 2);
  endspecify
endmodule
module pol(output y, input a);
  assign y = ~a;
  specify (a -=> y) = (2, 4); endspecify
endmodule
module t;
  reg a, s, c, d;
  wire y1, y2, y3, q, y6;
  cond u1(y1, a, s);
  nopath u2(y2, a, s);
  ifn u3(y3, a, s);
  dff u4(q, c, d);
  pol u6(y6, a);
  initial begin
    $monitor("%0t a=%b s=%b c=%b d=%b y1=%b y2=%b y3=%b q=%b y6=%b", $time, a, s, c, d, y1, y2, y3, q, y6);
    s = 0; c = 0; d = 1;
    #1 a = 0;
    #10 a = 1;
    #10 s = 1;
    #10 a = 0;
    #10 s = 1'bx;
    #10 a = 1;
    #10 s = 0; c = 1;
    #10 c = 0; d = 0;
    #10 c = 1;
    #10 d = 1;
    #3 c = 0;
    #3 c = 1;
    #20 $finish;
  end
endmodule
"#,
        &[],
    );
    assert_eq!(
        stdout(&report),
        "\
0 a=x s=0 c=0 d=1 y1=x y2=x y3=x q=x y6=x
1 a=0 s=0 c=0 d=1 y1=x y2=0 y3=x q=x y6=x
3 a=0 s=0 c=0 d=1 y1=x y2=0 y3=x q=x y6=1
6 a=0 s=0 c=0 d=1 y1=x y2=0 y3=0 q=x y6=1
7 a=0 s=0 c=0 d=1 y1=0 y2=0 y3=0 q=x y6=1
11 a=1 s=0 c=0 d=1 y1=0 y2=1 y3=0 q=x y6=1
15 a=1 s=0 c=0 d=1 y1=0 y2=1 y3=0 q=x y6=0
16 a=1 s=0 c=0 d=1 y1=0 y2=1 y3=1 q=x y6=0
17 a=1 s=0 c=0 d=1 y1=1 y2=1 y3=1 q=x y6=0
21 a=1 s=1 c=0 d=1 y1=1 y2=1 y3=1 q=x y6=0
31 a=0 s=1 c=0 d=1 y1=1 y2=1 y3=1 q=x y6=0
33 a=0 s=1 c=0 d=1 y1=0 y2=1 y3=0 q=x y6=1
35 a=0 s=1 c=0 d=1 y1=0 y2=0 y3=0 q=x y6=1
41 a=0 s=x c=0 d=1 y1=0 y2=0 y3=0 q=x y6=1
51 a=1 s=x c=0 d=1 y1=0 y2=0 y3=0 q=x y6=1
53 a=1 s=x c=0 d=1 y1=1 y2=0 y3=1 q=x y6=1
55 a=1 s=x c=0 d=1 y1=1 y2=1 y3=1 q=x y6=0
61 a=1 s=0 c=1 d=1 y1=1 y2=1 y3=1 q=x y6=0
64 a=1 s=0 c=1 d=1 y1=1 y2=1 y3=1 q=1 y6=0
71 a=1 s=0 c=0 d=0 y1=1 y2=1 y3=1 q=1 y6=0
81 a=1 s=0 c=1 d=0 y1=1 y2=1 y3=1 q=1 y6=0
83 a=1 s=0 c=1 d=0 y1=1 y2=1 y3=1 q=0 y6=0
91 a=1 s=0 c=1 d=1 y1=1 y2=1 y3=1 q=0 y6=0
94 a=1 s=0 c=0 d=1 y1=1 y2=1 y3=1 q=0 y6=0
97 a=1 s=0 c=1 d=1 y1=1 y2=1 y3=1 q=0 y6=0
100 a=1 s=0 c=1 d=1 y1=1 y2=1 y3=1 q=1 y6=0
"
    );
}

/// Section 14.2.4.1: a condition that is `x` or `z` counts as true, and a
/// condition that is a vector decides by its least significant bit alone.
/// Expected values are the LRM's closed form. Icarus Verilog (`-gspecify`)
/// agrees on `y1`, whose condition read as the OR of its bits would give 0
/// at 33 (`s` = 10), but reads `z` and `x` as false, giving `y2` 0 at 38 and
/// 1 at 58 through its `ifnone`.
#[test]
fn an_x_or_z_condition_counts_and_a_vector_condition_reads_its_lsb() {
    let report = bench(
        r#"`timescale 1ns/1ns
module vc(output y, input a, input [1:0] s);
  assign y = a;
  specify
    if (s) (a => y) = 2;
    ifnone (a => y) = 5;
  endspecify
endmodule
module zc(output y, input a, input s);
  assign y = a;
  specify
    if (s) (a => y) = 3;
    ifnone (a => y) = 7;
  endspecify
endmodule
module t;
  reg a, z;
  reg [1:0] s;
  wire y1, y2;
  vc u1(y1, a, s);
  zc u2(y2, a, z);
  initial begin
    $monitor("%0t a=%b s=%b z=%b y1=%b y2=%b", $time, a, s, z, y1, y2);
    s = 2'b01; z = 1;
    #1 a = 0;
    #10 a = 1;
    #10 s = 2'b10; z = 1'bz;
    #10 a = 0;
    #10 s = 2'b11; z = 1'bx;
    #10 a = 1;
    #10 s = 2'b00; z = 0;
    #10 a = 0;
    #10 $finish;
  end
endmodule
"#,
        &[],
    );
    assert_eq!(
        stdout(&report),
        "\
0 a=x s=01 z=1 y1=x y2=x
1 a=0 s=01 z=1 y1=x y2=x
3 a=0 s=01 z=1 y1=0 y2=x
4 a=0 s=01 z=1 y1=0 y2=0
11 a=1 s=01 z=1 y1=0 y2=0
13 a=1 s=01 z=1 y1=1 y2=0
14 a=1 s=01 z=1 y1=1 y2=1
21 a=1 s=10 z=z y1=1 y2=1
31 a=0 s=10 z=z y1=1 y2=1
34 a=0 s=10 z=z y1=1 y2=0
36 a=0 s=10 z=z y1=0 y2=0
41 a=0 s=11 z=x y1=0 y2=0
51 a=1 s=11 z=x y1=0 y2=0
53 a=1 s=11 z=x y1=1 y2=0
54 a=1 s=11 z=x y1=1 y2=1
61 a=1 s=00 z=0 y1=1 y2=1
71 a=0 s=00 z=0 y1=1 y2=1
76 a=0 s=00 z=0 y1=0 y2=1
78 a=0 s=00 z=0 y1=0 y2=0
"
    );
}

/// Section 14.3 with six delays: a vector destination takes the transition
/// of its least significant bit for every bit, as Icarus Verilog does ("using
/// the LSB delay for all bits"), and an unchanged least significant bit reads
/// as Icarus reads it (`0` to `0` a rise, `1` to `1` a fall, `x` to `x` as to
/// `0`, `z` to `z` as to `0`).
#[test]
fn a_vector_path_takes_its_least_significant_bits_transition() {
    let report = bench(
        r#"`timescale 1ns/1ns
module vec(output [1:0] y, input [1:0] a);
  assign y = a;
  specify (a *> y) = (2, 7, 4, 3, 9, 5); endspecify
endmodule
module t;
  reg [1:0] v;
  wire [1:0] y;
  vec u(y, v);
  initial begin
    $monitor("%0t v=%b y=%b", $time, v, y);
    v = 2'b00;
    #20 v = 2'b10;
    #20 v = 2'b00;
    #20 v = 2'b01;
    #20 v = 2'b11;
    #20 v = 2'b01;
    #20 v = 2'b0z;
    #20 v = 2'b01;
    #20 v = 2'b0z;
    #20 v = 2'b00;
    #20 v = 2'b0x;
    #20 v = 2'b01;
    #20 v = 2'b0x;
    #20 v = 2'b0z;
    #20 v = 2'b0x;
    #20 v = 2'b1x;
    #20 v = 2'b0z;
    #20 v = 2'b1z;
    #20 v = 2'b00;
    #20 $finish;
  end
endmodule
"#,
        &[],
    );
    assert_eq!(
        stdout(&report),
        "\
0 v=00 y=xx
7 v=00 y=00
20 v=10 y=00
22 v=10 y=10
40 v=00 y=10
42 v=00 y=00
60 v=01 y=00
62 v=01 y=01
80 v=11 y=01
87 v=11 y=11
100 v=01 y=11
107 v=01 y=01
120 v=0z y=01
129 v=0z y=0z
140 v=01 y=0z
143 v=01 y=01
160 v=0z y=01
169 v=0z y=0z
180 v=00 y=0z
185 v=00 y=00
200 v=0x y=00
202 v=0x y=0x
220 v=01 y=0x
223 v=01 y=01
240 v=0x y=01
247 v=0x y=0x
260 v=0z y=0x
269 v=0z y=0z
280 v=0x y=0z
283 v=0x y=0x
300 v=1x y=0x
307 v=1x y=1x
320 v=0z y=1x
329 v=0z y=0z
340 v=1z y=0z
345 v=1z y=1z
360 v=00 y=1z
365 v=00 y=00
"
    );
}
