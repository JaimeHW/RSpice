//! What a design sees at time zero.
//!
//! IEEE 1364-2005 leaves the time-zero start order open (section 11.4). The
//! expected text is what Icarus Verilog 13.0 printed for the same sources
//! (`iverilog`, `vvp`), whose order the ivtest gold files record. Each case
//! runs under both start orders: the result must not depend on either.

use super::*;

fn source_order(source: &str) -> String {
    stdout(&testbench(
        &[("tb.v", source)],
        &HdlTestbenchOptions::default(),
    ))
}

#[test]
fn an_initial_block_reads_nets_the_continuous_assignments_already_drive() {
    // Icarus: `b = [zzzz 0000] a=0101 e=0011 f=0111 g=0110`.
    let printed = both_orders(
        "module test;
           wire [3:0] z = 4'd0;
           wire signed [3:0] b[1:0];
           assign b[0] = $signed(z);
           wire [3:0] a = 4'd5;
           wire [3:0] e;
           assign e = 4'd3;
           reg [3:0] r = 4'd7;
           wire [3:0] f = r;
           wire [3:0] g = a + 1;
           initial $display(\"b = [%b %b] a=%b e=%b f=%b g=%b\", b[1], b[0], a, e, f, g);
         endmodule\n",
    );
    assert_eq!(printed, "b = [zzzz 0000] a=0101 e=0011 f=0111 g=0110\n");
}

#[test]
fn an_always_block_waiting_at_time_zero_sees_the_first_driven_values() {
    // Icarus: `0101 0110 0111 0101`. Every `always` reaches its event control
    // before the continuous assignments first drive, constant port
    // connections included, so each computes from them.
    let printed = both_orders(
        "module leaf(input [3:0] a, output reg [3:0] s);
           always @(*) s = a;
         endmodule
         module top;
           wire [3:0] c = 4'd5;
           wire [3:0] d;
           assign d = 4'd6;
           reg [3:0] s, s2;
           wire [3:0] s3, s4;
           always @* s = c;
           always @(d) s2 = d;
           leaf l1(4'd7, s3);
           leaf l2(c, s4);
           initial #1 $display(\"%b %b %b %b\", s, s2, s3, s4);
         endmodule\n",
    );
    assert_eq!(printed, "0101 0110 0111 0101\n");
}

#[test]
fn only_a_level_sensitive_always_is_waiting_when_time_zero_writes_land() {
    // Icarus: `c=0 l=1 m=0 s=1 w=0`. A combinational `always` is parked
    // before the `initial` block writes; an edge-triggered one starts after
    // the `initial` declared before it, so its time-zero edge is gone; and a
    // net an unknown variable drives starts unknown, so its first `x` is no
    // change. The reverse order starts the edge-triggered processes ahead of
    // the `initial` block, so only the source order is pinned.
    let printed = source_order(
        "module top;
           reg c, l, m, u;
           integer nc = 0, nl = 0, nm = 0, ns = 0, nw = 0;
           wire w = u;
           initial begin c = 1; l = 1; m = 1; end
           always @(posedge c) nc = nc + 1;
           always @(l) nl = nl + 1;
           always @(posedge m or l) nm = nm + 1;
           always @* ns = ns + m;
           always @(w) nw = nw + 1;
           initial #1 $display(\"c=%0d l=%0d m=%0d s=%0d w=%0d\", nc, nl, nm, ns, nw);
         endmodule\n",
    );
    assert_eq!(printed, "c=0 l=1 m=0 s=1 w=0\n");
}

#[test]
fn a_net_only_a_constant_z_drives_stays_z_and_wakes_nothing() {
    // Icarus: `v=z w=x u=0 k=x m=xzzz nv=0 nw=0 nu=1 nk=0 nm=1`. A driven net
    // starts unknown, so a first `x` is no change; `assign v = 1'bz` drives
    // nothing (Icarus elaborates it away), so `v` stays `z` and its
    // `always` does not wake either. A first value that differs from `x`
    // anywhere, `0` or `xzzz`, wakes its `always`.
    let printed = both_orders(
        "module top;
           wire v, u, k;
           wire [3:0] m;
           reg r;
           wire w = r;
           integer nv = 0, nw = 0, nu = 0, nk = 0, nm = 0;
           assign v = 1'bz;
           assign u = 1'b0;
           assign k = 1'bx;
           assign m = 4'bxzzz;
           always @(v) nv = nv + 1;
           always @(w) nw = nw + 1;
           always @(u) nu = nu + 1;
           always @(k) nk = nk + 1;
           always @(m) nm = nm + 1;
           initial #1 $display(\"v=%b w=%b u=%b k=%b m=%b nv=%0d nw=%0d nu=%0d nk=%0d nm=%0d\",
                               v, w, u, k, m, nv, nw, nu, nk, nm);
         endmodule\n",
    );
    assert_eq!(printed, "v=z w=x u=0 k=x m=xzzz nv=0 nw=0 nu=1 nk=0 nm=1\n");
}
