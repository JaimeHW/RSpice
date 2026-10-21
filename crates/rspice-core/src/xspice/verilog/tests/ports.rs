//! Port forms end to end.
//!
//! Expected text is what Icarus Verilog 13.0 printed for the same source
//! (`iverilog -g2005`, `vvp -n`) in an RSpice-authored probe.

use super::*;

/// `output time` in both port styles (IEEE 1364-2005 section 12.3.3's
/// output-variable type): an unsigned 64-bit variable that keeps a value past
/// 32 bits, takes `$time`, accepts a declaration initializer and sizes into a
/// narrower net by assignment, beside the older `output t; time t;` pair.
#[test]
fn time_ports_run_as_icarus_runs_them() {
    let report = bench(
        r#"module child(output time t, output time u);
  initial begin t = 64'd5000000000; u = $time; #7 u = $time; end
endmodule
module child2(t); output t; time t; initial t = 3;
endmodule
module child3(t); output time t; initial t = 64'hFFFF_FFFF_FFFF_FFFF; endmodule
module child4(output time t = 9, output time u); initial u = t + 1; endmodule
module top;
  wire [63:0] a, b, c, d, e, f;
  wire [31:0] narrow;
  child c1(a, b); child2 c2(c); child3 c3(d); child4 c4(e, f);
  child3 c5(narrow);
  initial begin #1 $display("%0d %0d %0d %0d %0d %0d %0d", a, b, c, d, e, f, narrow); #10 $display("%0d", b); end
endmodule
"#,
        &[],
    );
    assert_eq!(
        stdout(&report),
        "5000000000 0 3 18446744073709551615 9 10 4294967295\n7\n"
    );
}

/// A net of a different width (IEEE 1364-2005 section 12.3.9): an input
/// reads as `assign port = net;`, extended by the net's signedness or
/// truncated, an output as `assign net = port;` with a wider net's extra bits
/// driven 0, and an inout joins the bits the two share from the least
/// significant, leaving the rest to their own drivers.
#[test]
fn ports_of_a_different_width_truncate_and_extend_as_icarus_does() {
    let report = bench(
        r#"module nin(input [3:0] a, output [3:0] y); assign y = a; endmodule
module nout(input [1:0] a, output [5:0] y); assign y = {a, a, a}; endmodule
module sin(input signed [3:0] a, output [3:0] y); assign y = a; endmodule
module io(inout [3:0] p); wire [3:0] seen = p; endmodule
module top;
  reg [1:0] r2; reg signed [1:0] s2; reg [5:0] r6;
  wire [3:0] y1, y2, y3, y4; wire [2:0] n3; wire [7:0] w8; wire [1:0] y5;
  wire signed [1:0] sw2 = s2;
  wire [1:0] iow; wire [5:0] io6;
  nin u1(.a(r2), .y(y1));
  nin u2(.a(r6), .y(y2));
  nin u3(.a(sw2), .y(y3));
  sin u4(.a(r2), .y(y4));
  nout u5(.a(r2), .y(n3));
  nout u6(.a(r2), .y(w8));
  nin u7(.a(r2), .y(y5));
  io u8(.p(iow));
  io u9(.p(io6));
  assign iow = 2'b10;
  assign io6 = 6'b110011;
  initial begin
    r2 = 2'b11; s2 = -1; r6 = 6'b101101;
    #1 $display("%b %b %b %b %b %b %b", y1, y2, y3, y4, n3, w8, y5);
    $display("%b %b %b %b", u8.p, u9.p, iow, io6);
    r2 = 2'bz1; #1 $display("%b %b", y1, w8);
  end
endmodule
"#,
        &[],
    );
    assert_eq!(
        stdout(&report),
        "0011 1101 1111 0011 111 00111111 11\n\
         zz10 0011 10 110011\n\
         00z1 00z1z1z1\n"
    );
}

/// An output port driving a concatenation and a part-select of another
/// width, and an inout joined to a narrower part-select.
#[test]
fn an_output_drives_a_concatenation_and_a_select_of_another_width() {
    let report = bench(
        r#"module src(input [3:0] a, output [3:0] y, output [1:0] z); assign y = a; assign z = a[1:0]; endmodule
module io(inout [3:0] p); wire [3:0] seen = p; endmodule
module top;
  reg [3:0] r; wire c; wire [1:0] s; wire e, f, g; wire [7:0] bus;
  src u1(.a(r), .y({c, s}), .z({e, f, g}));
  src u2(.a(r), .y(bus[6:5]), .z(bus[3:0]));
  io u3(.p(bus[7:6]));
  initial begin
    r = 4'b1011;
    #1 $display("%b %b %b %b %b %b %b", c, s, e, f, g, bus, u3.p);
    r = 4'b0110;
    #1 $display("%b %b %b %b %b %b %b", c, s, e, f, g, bus, u3.p);
  end
endmodule
"#,
        &[],
    );
    assert_eq!(
        stdout(&report),
        "0 11 0 1 1 z11z0011 zzz1\n\
         1 10 0 1 0 z10z0010 zzz1\n"
    );
}

/// Arrays of instances (IEEE 1364-2005 section 12.1.2): one instance per
/// element, named after its index, each taking its share of a connection as
/// wide as all the ports together, the leftmost element the leftmost bits
/// (section 12.1.3), and the whole of a connection as wide as one port.
#[test]
fn an_array_of_instances_shares_out_its_connections_as_icarus_does() {
    let report = bench(
        r#"module inv2(input [1:0] a, input en, output [1:0] y);
  parameter D = 3;
  assign y = en ? ~a : a;
  initial #D if (a == 2'b01) $display("%0t %m holds 01", $time);
endmodule
module top;
  parameter N = 3;
  reg [5:0] a; reg en; wire [5:0] y; wire [0:5] z; wire [3:0] w;
  inv2 u[N-1:0] (.a(a), .en(en), .y(y));
  inv2 #(.D(4)) v[0:2] (a, en, z);
  inv2 #(5) b[1:0] (.a(a[5:2]), .en(en), .y(w));
  initial begin
    a = 6'b110100; en = 0;
    #1 $display("%b %b %b", y, z, w);
    en = 1;
    #1 $display("%b %b %b", y, z, w);
  end
endmodule
"#,
        &[],
    );
    assert_eq!(
        stdout(&report),
        "110100 110100 1101\n\
         001011 001011 0010\n\
         3 top.u[1] holds 01\n\
         4 top.v[1] holds 01\n\
         5 top.b[0] holds 01\n"
    );
}
