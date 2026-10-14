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
