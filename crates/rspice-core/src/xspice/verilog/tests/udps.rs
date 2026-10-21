//! User-defined primitives (IEEE 1364-2005 section 8), end to end.
//!
//! Expected text is what Icarus Verilog 13.0 printed for the same source
//! (`iverilog -g2005`, `vvp -n`) in an RSpice-authored probe.

use super::*;

/// A combinational table with `x` rows, `?` and a `z` input read as `x`, and
/// `x` for a combination no row gives; a sequential one with an `initial`,
/// the edge abbreviations, `(vw)` edges, `b`, `-`, a level row that dominates
/// the edge rows (section 8.8), and `x` for an edge no row gives; a latch of
/// level rows only; an unnamed instance; and a constant input, whose first
/// value is an edge from `x` at time zero.
#[test]
fn primitive_tables_follow_level_and_edge_rows() {
    let report = bench(
        r#"primitive mux (o, s, a, b);
  output o; input s, a, b;
  table
    0 0 ? : 0;
    0 1 ? : 1;
    1 ? 0 : 0;
    1 ? 1 : 1;
    x 0 0 : 0;
    x 1 1 : 1;
  endtable
endprimitive
primitive dffr (q, d, c, r);
  output q; reg q; input d, c, r;
  initial q = 1'b1;
  table
  // d c r : q : q+
     ? ? 1 : ? : 0;
     0 r 0 : ? : 0;
     1 r 0 : ? : 1;
     1 (x1) 0 : 1 : 1;
     0 (x1) 0 : 0 : 0;
     ? (?0) 0 : ? : -;
     ? (1x) 0 : ? : -;
     * b 0 : ? : -;
     ? ? (?0) : ? : -;
  endtable
endprimitive
primitive latch (q, e, d);
  output q; reg q; input e, d;
  table
     1 0 : ? : 0;
     1 1 : ? : 1;
     0 ? : ? : -;
  endtable
endprimitive
module top;
  reg s, a, b, d, c, r, e;
  wire m, q, l, k;
  mux (m, s, a, b);
  dffr ff (q, d, c, r);
  latch la (l, e, d), (k, 1'b1, d);
  initial begin
    $monitor("%0t m=%b q=%b l=%b k=%b", $time, m, q, l, k);
    s = 0; a = 1; b = 0; r = 0;
    #1 s = 1;
    #1 s = 1'bx; b = 1;
    #1 a = 1'bz;
    #1 s = 1'bz; a = 1;
    #1 d = 0; c = 0;
    #1 c = 1;
    #1 d = 1; e = 1;
    #1 c = 0;
    #1 c = 1'bx;
    #1 c = 1;
    #1 r = 1;
    #1 c = 0; e = 0;
    #1 c = 1; d = 0;
    #1 r = 0;
    #1 c = 0;
    #1 c = 1;
    #1 d = 1'bx;
  end
endmodule
"#,
        &[],
    );
    assert_eq!(
        stdout(&report),
        "0 m=1 q=1 l=x k=x\n\
         1 m=0 q=1 l=x k=x\n\
         2 m=1 q=1 l=x k=x\n\
         3 m=x q=1 l=x k=x\n\
         4 m=1 q=1 l=x k=x\n\
         5 m=1 q=x l=x k=0\n\
         6 m=1 q=0 l=x k=0\n\
         7 m=1 q=0 l=1 k=1\n\
         9 m=1 q=x l=1 k=1\n\
         11 m=1 q=0 l=1 k=1\n\
         13 m=1 q=0 l=1 k=0\n\
         17 m=1 q=0 l=1 k=x\n"
    );
}
