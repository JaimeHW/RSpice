//! Drive strengths and net types (IEEE 1364-2005 sections 4.6 and 7.9-7.10),
//! end to end.
//!
//! Expected text is what Icarus Verilog 13.0 printed for the same source
//! (`iverilog -g2005`, `vvp -n`) in an RSpice-authored probe.

use super::*;

/// Continuous assignments with drive strengths, a net declaration assignment
/// with one, `highz1`, `tri0`/`tri1`, `supply0`/`supply1`, `wand`/`wor`, and a
/// vector, printed with `%v` and `%b` through every driver state.
#[test]
fn drive_strengths_and_net_types_resolve_and_print_as_icarus_does() {
    let report = bench(
        r#"module top;
  reg a, b; reg [1:0] v;
  wire w1, w2, w3;
  wire [1:0] wv;
  tri0 t0; tri1 t1; supply0 s0; supply1 s1;
  wand wa; wor wo;
  wire (weak1, weak0) dw = a;
  assign (pull1, pull0) w1 = a;
  assign (strong1, strong0) w1 = b;
  assign (weak0, weak1) w2 = a;
  assign (weak0, weak1) w2 = b;
  assign (highz1, strong0) w3 = a;
  assign t0 = b;
  assign wa = a; assign wa = b;
  assign wo = a; assign wo = b;
  assign (pull0, pull1) wv = v;
  assign s0 = a;
  initial begin
    a = 0; b = 1'bz; v = 2'b1x;
    #1 $display("%v %v %v %v %v %v %v %v %v %v %v %b %b %b", w1, w2, w3, t0, t1, s0, s1, dw, wv, wa, wo, t0, wv, w3);
    a = 1; b = 0; v = 2'bz0;
    #1 $display("%v %v %v %v %v %v %v %v %v %v %v %b %b %b", w1, w2, w3, t0, t1, s0, s1, dw, wv, wa, wo, t0, wv, w3);
    a = 1'bx; b = 1;
    #1 $display("%v %v %v %v %v %v %v %v %v %v %v %b %b %b", w1, w2, w3, t0, t1, s0, s1, dw, wv, wa, wo, t0, wv, w3);
    a = 1'bz; b = 1'bz;
    #1 $display("%v %v %v %v %v %v %v %v %v %v %v %b %b %b", w1, w2, w3, t0, t1, s0, s1, dw, wv, wa, wo, t0, wv, w3);
  end
endmodule
"#,
        &[],
    );
    assert_eq!(
        stdout(&report),
        "Pu0 We0 St0 Pu0 Pu1 Su0 Su1 We0 Pu1_PuX St0 St0 0 1x 0\n\
         St0 WeX HiZ St0 Pu1 Su0 Su1 We1 HiZ_Pu0 St0 St1 0 z0 z\n\
         St1 WeX StL St1 Pu1 Su0 Su1 WeX HiZ_Pu0 StX St1 1 z0 x\n\
         HiZ HiZ HiZ Pu0 Pu1 Su0 Su1 HiZ HiZ_Pu0 HiZ HiZ 0 z0 z\n"
    );
}

/// IEEE 1364-2005 section 4.6.2: a `uwire` with a second driver is refused
/// (Icarus: "Unresolved wire 'u' cannot have multiple drivers").
#[test]
fn a_uwire_with_two_drivers_is_refused() {
    let sources = vec![(
        std::path::PathBuf::from("tb.v"),
        "module top;\n  reg a, b;\n  uwire u;\n  assign u = a;\n  assign u = b;\n  initial #1 $display(\"%b\", u);\nendmodule\n"
            .to_owned(),
    )];
    let error = run_digital_testbench(&sources, &HdlTestbenchOptions::default())
        .expect_err("a second driver of a uwire is refused")
        .to_string();
    assert!(
        error.contains("`u` is a `uwire` with more than one driver"),
        "{error}"
    );
    let one = bench(
        "module top;\n  reg a;\n  uwire u;\n  assign u = a;\n  initial begin a = 1; #1 $display(\"%b\", u); end\nendmodule\n",
        &[],
    );
    assert_eq!(stdout(&one), "1\n");
}

/// The rows a loop over data and control in `0 1 x z` prints.
const SWEEP: &str = "  integer i, j;\n  reg [3:0] vals;\n";

/// `bufif0`, `bufif1`, `notif0` and `notif1` (section 7.5) over every data and
/// control value, one at a drive strength.
///
/// Icarus prints `StX` for a strong `notif` under an unknown control. Section
/// 7.5's tables give `H` and `L` there (a 0 data bit inverts to a 1 that the
/// gate may or may not drive), which is what Icarus itself prints for a `notif`
/// with unequal strengths; those four cells are the standard's.
#[test]
fn three_state_gates_drive_l_and_h_under_an_unknown_control() {
    let report = bench(
        &format!(
            "module top;\n  reg d, c;\n  wire b0, b1, n0, n1;\n\
             \x20 bufif0 g0 (b0, d, c);\n  bufif1 g1 (b1, d, c);\n  notif0 g2 (n0, d, c);\n\
             \x20 notif1 (weak1, pull0) g3 (n1, d, c);\n{SWEEP}\
             \x20 initial begin\n    vals = 4'bzx10;\n\
             \x20   for (i = 0; i < 4; i = i + 1)\n      for (j = 0; j < 4; j = j + 1) begin\n\
             \x20       d = vals[i]; c = vals[j];\n\
             \x20       #1 $display(\"%b %b: %v %v %v %v\", d, c, b0, b1, n0, n1);\n\
             \x20     end\n  end\nendmodule\n"
        ),
        &[],
    );
    assert_eq!(
        stdout(&report),
        "0 0: St0 HiZ St1 HiZ\n0 1: HiZ St0 HiZ We1\n0 x: StL StL StH WeH\n0 z: StL StL StH WeH\n\
         1 0: St1 HiZ St0 HiZ\n1 1: HiZ St1 HiZ Pu0\n1 x: StH StH StL PuL\n1 z: StH StH StL PuL\n\
         x 0: StX HiZ StX HiZ\nx 1: HiZ StX HiZ 53X\nx x: StX StX StX 53X\nx z: StX StX StX 53X\n\
         z 0: StX HiZ StX HiZ\nz 1: HiZ StX HiZ 53X\nz x: StX StX StX 53X\nz z: StX StX StX 53X\n"
    );
}

/// `nmos`, `pmos` and `cmos` (sections 7.6 and 7.7) pass the strength their
/// data arrives with, supply reduced to strong (section 7.12); `pullup` and
/// `pulldown` drive at pull or the strength given (section 7.8). A switch
/// follows a strength change of the net it passes that leaves the value alone.
#[test]
fn mos_switches_pass_their_input_strength_and_pull_gates_drive_pull() {
    let report = bench(
        &format!(
            "module top;\n  reg d, c, cp, s;\n  wire pu, m1, m2, m5, sp, mp, pw, pd;\n\
             \x20 supply1 vdd;\n  pullup (pu);\n  assign (strong1, highz0) pu = s;\n\
             \x20 nmos (m1, d, c);\n  pmos (m2, d, c);\n  cmos (m5, d, c, cp);\n\
             \x20 nmos (sp, vdd, c);\n  nmos (mp, pu, c);\n\
             \x20 pullup (weak1) (pw);\n  pulldown (strong0) (pd);\n{SWEEP}\
             \x20 initial begin\n    vals = 4'bzx10;\n    s = 0; cp = 1;\n\
             \x20   for (i = 0; i < 4; i = i + 1)\n      for (j = 0; j < 4; j = j + 1) begin\n\
             \x20       d = vals[i]; c = vals[j];\n\
             \x20       #1 $display(\"%b %b: %v %v %v %v %v %v %v\", d, c, m1, m2, m5, sp, mp, pw, pd);\n\
             \x20     end\n\
             \x20   c = 1; cp = 0;\n    #1 $display(\"%v %v %v\", mp, pu, m5);\n\
             \x20   s = 1;\n    #1 $display(\"%v %v %v\", mp, pu, m5);\n\
             \x20   s = 0;\n    #1 $display(\"%v %v %v\", mp, pu, m5);\n\
             \x20 end\nendmodule\n"
        ),
        &[],
    );
    assert_eq!(
        stdout(&report),
        "0 0: HiZ St0 HiZ HiZ HiZ We1 St0\n0 1: St0 HiZ St0 St1 Pu1 We1 St0\n\
         0 x: StL StL StL StH PuH We1 St0\n0 z: StL StL StL StH PuH We1 St0\n\
         1 0: HiZ St1 HiZ HiZ HiZ We1 St0\n1 1: St1 HiZ St1 St1 Pu1 We1 St0\n\
         1 x: StH StH StH StH PuH We1 St0\n1 z: StH StH StH StH PuH We1 St0\n\
         x 0: HiZ StX HiZ HiZ HiZ We1 St0\nx 1: StX HiZ StX St1 Pu1 We1 St0\n\
         x x: StX StX StX StH PuH We1 St0\nx z: StX StX StX StH PuH We1 St0\n\
         z 0: HiZ HiZ HiZ HiZ HiZ We1 St0\nz 1: HiZ HiZ HiZ St1 Pu1 We1 St0\n\
         z x: HiZ HiZ HiZ StH PuH We1 St0\nz z: HiZ HiZ HiZ StH PuH We1 St0\n\
         Pu1 Pu1 HiZ\nSt1 St1 HiZ\nPu1 Pu1 HiZ\n"
    );
}

/// The resistive switches reduce what they pass by table 7-14 (section 7.12):
/// supply and strong to pull, pull to weak, weak to medium. Computed from the
/// table, because Icarus 13 passes these unreduced.
#[test]
fn resistive_switches_reduce_strength_by_table_7_14() {
    let report = bench(
        "module top;\n  reg d, c;\n  wire pu, r1, r2, r3, r4, r5;\n  supply1 vdd;\n  pullup (pu);\n\
         \x20 rnmos (r1, d, c);\n  rpmos (r2, d, c);\n  rnmos (r3, vdd, c);\n\
         \x20 rnmos (r4, pu, c);\n  rcmos (r5, r4, c, 1'b1);\n\
         \x20 initial begin\n\
         \x20   d = 0; c = 1;\n    #1 $display(\"%v %v %v %v %v\", r1, r2, r3, r4, r5);\n\
         \x20   d = 1; c = 1'bx;\n    #1 $display(\"%v %v %v %v %v\", r1, r2, r3, r4, r5);\n\
         \x20   c = 0;\n    #1 $display(\"%v %v %v %v %v\", r1, r2, r3, r4, r5);\n\
         \x20 end\nendmodule\n",
        &[],
    );
    assert_eq!(
        stdout(&report),
        "Pu0 HiZ Pu1 We1 Me1\nPuH PuH PuH WeH MeH\nHiZ Pu1 HiZ HiZ HiZ\n"
    );
}

/// Bidirectional switches (sections 7.6 and 7.12) resolve the island they
/// join to a fixed point: supply passes `tran` as strong and each `rtran`
/// reduces by table 7-14, a chain joins and splits under its controls, an
/// unknown control passes an ambiguous strength, and a driver released from
/// the island leaves nothing behind.
#[test]
fn bidirectional_switches_resolve_their_islands() {
    let report = bench(
        r#"module top;
  reg a, c, d;
  supply0 s0;
  wire w1, w2, w3, w4, w5, r1, r2, x1, x2, k1, k2;
  tran (s0, w1);
  rtran (w1, r1);
  rtran (r1, r2);
  assign (weak1, weak0) w2 = a;
  tranif1 (w2, w3, c);
  tranif0 (w3, w4, d);
  assign w4 = d ? 1'bz : 1'b1;
  assign w5 = c ? a : 1'bz;
  tran (w5, x1);
  rtranif1 (x1, x2, d);
  pullup (k1);
  tran (k1, k2);
  initial begin
    a = 0; c = 0; d = 0;
    #1 $display("%v %v %v | %v %v %v | %v %v %v | %v %v", w1, r1, r2, w2, w3, w4, w5, x1, x2, k1, k2);
    c = 1;
    #1 $display("%v %v %v | %v %v %v | %v %v %v | %v %v", w1, r1, r2, w2, w3, w4, w5, x1, x2, k1, k2);
    d = 1;
    #1 $display("%v %v %v | %v %v %v | %v %v %v | %v %v", w1, r1, r2, w2, w3, w4, w5, x1, x2, k1, k2);
    c = 1'bx; a = 1;
    #1 $display("%v %v %v | %v %v %v | %v %v %v | %v %v", w1, r1, r2, w2, w3, w4, w5, x1, x2, k1, k2);
    c = 0; d = 1'bx;
    #1 $display("%v %v %v | %v %v %v | %v %v %v | %v %v", w1, r1, r2, w2, w3, w4, w5, x1, x2, k1, k2);
    c = 1; d = 1'bz; a = 1'bx;
    #1 $display("%v %v %v | %v %v %v | %v %v %v | %v %v", w1, r1, r2, w2, w3, w4, w5, x1, x2, k1, k2);
  end
endmodule
"#,
        &[],
    );
    assert_eq!(
        stdout(&report),
        "St0 Pu0 We0 | We0 St1 St1 | HiZ HiZ HiZ | Pu1 Pu1\n\
         St0 Pu0 We0 | St1 St1 St1 | St0 St0 HiZ | Pu1 Pu1\n\
         St0 Pu0 We0 | We0 We0 HiZ | St0 St0 Pu0 | Pu1 Pu1\n\
         St0 Pu0 We0 | We1 WeH HiZ | StX StX PuX | Pu1 Pu1\n\
         St0 Pu0 We0 | We1 StX StX | HiZ HiZ HiZ | Pu1 Pu1\n\
         St0 Pu0 We0 | StX StX StX | StX StX PuX | Pu1 Pu1\n"
    );
}

/// A keeper whose switches the pin itself controls holds the pin's last value
/// at weak strength once its driver lets go; a `tran` array shares vectors out
/// bit by bit; and a switch inside a module joins the nets its inout ports
/// collapse with.
#[test]
fn switch_keepers_arrays_and_ports_join_islands() {
    let report = bench(
        r#"module sw(inout p, inout q, input en);
  rtranif0 (p, q, en);
endmodule
module top;
  reg value, enable, en;
  wire pin;
  pullup (weak1) (keep1);
  pulldown (weak0) (keep0);
  tranif1 (pin, keep1, pin);
  tranif0 (pin, keep0, pin);
  bufif1 (pin, value, enable);
  reg [1:0] v;
  wire [1:0] a, b;
  assign a = v;
  tran t[1:0] (a, b);
  wire p, q;
  assign (pull1, pull0) p = value;
  sw s (p, q, en);
  initial begin
    value = 0; enable = 1; v = 2'b1z; en = 0;
    #1 $display("%v %v %v %v %v %v", pin, b[1], b[0], p, q, keep0);
    enable = 0; v = 2'bz0; en = 1;
    #1 $display("%v %v %v %v %v %v", pin, b[1], b[0], p, q, keep0);
    value = 1; enable = 1; en = 1'bx;
    #1 $display("%v %v %v %v %v %v", pin, b[1], b[0], p, q, keep0);
    enable = 0;
    #1 $display("%v %v %v %v %v %v", pin, b[1], b[0], p, q, keep0);
  end
endmodule
"#,
        &[],
    );
    assert_eq!(
        stdout(&report),
        "St0 St1 HiZ Pu0 We0 St0\n\
         We0 HiZ St0 Pu0 HiZ We0\n\
         St1 HiZ St0 Pu1 WeH We0\n\
         We1 HiZ St0 Pu1 WeH We0\n"
    );
}

/// A forced net bit (section 9.3.2) drives its island at strong whatever the
/// island passes it, and rejoins the island when released; a `tran` array
/// joins part selects bit by bit.
#[test]
fn a_forced_bit_drives_its_island() {
    let report = bench(
        r#"module top;
  reg a, b;
  wire x, y, z;
  wire [3:0] c;
  assign (weak1, weak0) x = a;
  assign y = b;
  tran (x, y);
  rtran (y, z);
  tran t[1:0] (c[1:0], c[3:2]);
  assign c[0] = a;
  assign (pull1, pull0) c[3] = b;
  initial begin
    a = 0; b = 1'bz;
    #1 $display("%v %v %v %b", x, y, z, c);
    force x = 1'b1;
    #1 $display("%v %v %v %b", x, y, z, c);
    b = 0;
    #1 $display("%v %v %v %b", x, y, z, c);
    release x;
    #1 $display("%v %v %v %b", x, y, z, c);
    force z = 1'bx; b = 1'bz;
    #1 $display("%v %v %v %b", x, y, z, c);
    release z;
    #1 $display("%v %v %v %b", x, y, z, c);
  end
endmodule
"#,
        &[],
    );
    assert_eq!(
        stdout(&report),
        "We0 We0 Me0 z0z0\nSt1 St1 Pu1 z0z0\nSt1 StX PuX 0000\n\
         St0 St0 Pu0 0000\nPuX PuX StX z0z0\nWe0 We0 Me0 z0z0\n"
    );
}

/// Arrays of gate instances (sections 7.1.5 and 7.1.6): a vector terminal
/// shared out bit by bit, a concatenation, and two instances driving one
/// scalar.
#[test]
fn gate_instance_arrays_share_their_terminals_out_by_width() {
    let report = bench(
        "module top;\n  reg [3:0] dv; reg en, d;\n  wire [3:0] bv; wire [1:0] nv; wire w;\n\
         \x20 bufif1 ba[3:0] (bv, dv, en);\n  nmos na[1:0] (nv, {1'b0, d}, en);\n\
         \x20 bufif0 (pull1, pull0) bw[1:0] (w, dv[1:0], en);\n\
         \x20 initial begin\n\
         \x20   dv = 4'b10xz; en = 1; d = 1;\n    #1 $display(\"%b %b %b %v\", bv, nv, w, w);\n\
         \x20   en = 0; dv = 4'b0110;\n    #1 $display(\"%b %b %b %v\", bv, nv, w, w);\n\
         \x20   dv = 4'b0011;\n    #1 $display(\"%b %b %b %v\", bv, nv, w, w);\n\
         \x20 end\nendmodule\n",
        &[],
    );
    assert_eq!(
        stdout(&report),
        "10xx 01 z HiZ\nzzzz zz x PuX\nzzzz zz 1 Pu1\n"
    );
}

/// IEEE 1364-2005 section 6.5: an identifier first named in a gate's
/// terminal list is an implicit scalar net of the default net type, and an
/// array of gate instances (section 7.1.5) is a gate instance like any other.
/// Section 7.1.6 then gives that one-bit terminal to every instance.
///
/// In the first design `a` and `b` are named nowhere else and nothing drives
/// them, so each is `z`, which a `not` reads as `x` (section 7.2, table 7-3):
/// `xxxx` and `x`. In the second a one-element `buf` array drives `a` from
/// `d`, so `y` is `~d` in every bit: `xxxx` while `d` is `x`, then 1111 and
/// 0000. `c` stays `z`, so `v = a & c` is `x` but for `a = 0`, where it is 0
/// (table 7-3). Under `default_nettype none` the name is refused.
#[test]
fn a_gate_terminal_names_an_implicit_net() {
    let report = bench(
        "module top;\n  wire [3:0] y;\n  wire w;\n  not n[3:0] (y, a);\n  not (w, b);\n\
         \x20 initial #1 $display(\"%b %b\", y, w);\nendmodule\n",
        &[],
    );
    assert_eq!(stdout(&report), "xxxx x\n");
    let report = bench(
        "module top;\n  wire [3:0] y;\n  reg d;\n  not n[3:0] (y, a);\n  buf b[0:0] (a, d);\n\
         \x20 and (v, a, c);\n\
         \x20 initial begin\n\
         \x20   #1 $display(\"%b %b\", y, v);\n    d = 0;\n    #1 $display(\"%b %b\", y, v);\n\
         \x20   d = 1;\n    #1 $display(\"%b %b\", y, v);\n\
         \x20 end\nendmodule\n",
        &[],
    );
    assert_eq!(stdout(&report), "xxxx x\n1111 0\n0000 x\n");
    let sources = vec![(
        std::path::PathBuf::from("tb.v"),
        "`default_nettype none\nmodule top;\n  wire [3:0] y;\n  not n[3:0] (y, a);\nendmodule\n"
            .to_owned(),
    )];
    let error = run_digital_testbench(&sources, &HdlTestbenchOptions::default())
        .expect_err("an undeclared terminal under `default_nettype none` is refused")
        .to_string();
    assert!(
        error.contains("`a` requires a net declaration under `default_nettype none`"),
        "{error}"
    );
}
