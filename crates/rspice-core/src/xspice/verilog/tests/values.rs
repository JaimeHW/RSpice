//! Literal values, selects and the directives that shape a net, end to end.
//!
//! Expected text is what Icarus Verilog 13.0 printed for the same source
//! (`iverilog -g2005`, `vvp -n`) in an RSpice-authored probe.

use super::*;

/// Indexed part-selects (IEEE 1364-2005 section 5.2.1): `+:` and `-:` on a
/// descending and an ascending vector, read, written and driven by a
/// variable base, including a select that runs past the vector's bits.
#[test]
fn indexed_part_selects_read_write_and_drive_as_icarus_does() {
    let report = bench(
        r#"module top;
  reg [15:0] r; reg [0:15] b; integer i; wire [7:0] w; wire [3:0] lo;
  assign w = r[i +: 8];
  assign lo = b[i -: 4];
  initial begin
    r = 16'hA5C3; b = 16'h1234; i = 4;
    #1 $display("%h %h %h %h %h", r[4 +: 8], r[11 -: 8], b[0 +: 4], b[7 -: 4], w);
    $display("%h", lo);
    r[i +: 4] = 4'hF; b[8 +: 8] = 8'hEE;
    #1 $display("%h %h %h", r, b, w);
    i = 14;
    #1 $display("%h %b", w, r[i +: 4]);
  end
endmodule
"#,
        &[],
    );
    assert_eq!(stdout(&report), "5c 5c 1 2 5c\n2\na5f3 12ee 5f\nxX xx10\n");
}

/// String literals (IEEE 1364-2005 section 3.6) as values: assigned to
/// vectors wider and narrower than the string, in parameters, a continuous
/// assignment, arithmetic, comparisons, a `case`, concatenations and octal
/// and tab escapes.
#[test]
fn string_literals_are_the_vectors_icarus_makes_of_them() {
    let report = bench(
        r#"module top;
  parameter P = "hi";
  localparam [8*3:1] Q = "abc";
  reg [8*5:1] s; reg [15:0] t; reg [7:0] c; reg [31:0] x; reg [31:0] y [0:1];
  wire [31:0] z = "\000a\000b";
  wire [23:0] q3 = "a\tb";
  integer i;
  initial begin
    s = "hello"; t = "hello"; c = "A" + 1; x = "\101\102";
    y[0] = "ab"; y[1] = {"a", "b"};
    #1;
    $display("%s|%s|%s|%h|%h|%0d", s, t, c, x, z, "AB" == 16'h4142);
    $display("%h %h %s %s", y[0], y[1], P, Q);
    $display("%h %b", q3, "\n");
    if (s == "hello") $display("eq");
    case (t) "lo": $display("case lo"); default: $display("case other"); endcase
    for (i = 0; i < 3; i = i + 1) $write("%s", Q[8*(3-i) -: 8]);
    $display("");
    $display("%0d", {"x", "yz"} === 24'h78797a);
  end
endmodule
"#,
        &[],
    );
    assert_eq!(
        stdout(&report),
        "hello|lo|B|00004142|00610062|1\n\
         00006162 00006162 hi abc\n\
         610962 00001010\n\
         eq\n\
         case lo\n\
         abc\n\
         1\n"
    );
}

/// `vectored` and `scalared` (IEEE 1364-2005 section 4.3.2) change nothing a
/// net holds, and `` `begin_keywords `` selects the language this compiler
/// reads already.
#[test]
fn vectored_and_scalared_nets_hold_what_plain_nets_hold() {
    let report = bench(
        r#"`begin_keywords "1364-2005"
module top;
  wire vectored [3:0] v = 4'b1010;
  tri scalared [1:0] s;
  wire scalared [0:2] n = 3'b011;
  assign s = v[1:0];
  initial #1 $display("%b %b %b %b %b", v, s, v[2], n, n[0]);
endmodule
`end_keywords
"#,
        &[],
    );
    assert_eq!(stdout(&report), "1010 10 0 011 0\n");
}

/// `` `unconnected_drive `` (IEEE 1364-2005 section 19.9) pulls the
/// unconnected input ports of the modules that follow it, until
/// `` `nounconnected_drive `` or `` `resetall ``; a connected port keeps its
/// connection.
#[test]
fn unconnected_drive_pulls_the_inputs_nothing_connects() {
    let report = bench(
        r#"module top;
  wire [1:0] y0, y1, yz; wire c;
  pl0 a(.y(y0));
  pl1 b(.y(y1), .in());
  plz z(.y(yz));
  pl1 d(.in(2'b01), .y(c));
  initial #1 $display("%b %b %b %b", y0, y1, yz, c);
endmodule
`unconnected_drive pull0
module pl0(in, y); input [1:0] in; output [1:0] y; assign y = in; endmodule
`nounconnected_drive
`unconnected_drive pull1
module pl1(in, y); input [1:0] in; output [1:0] y; assign y = in; endmodule
`resetall
module plz(in, y); input [1:0] in; output [1:0] y; assign y = in; endmodule
"#,
        &[],
    );
    assert_eq!(stdout(&report), "00 11 zz 1\n");
}
