//! `trireg` nets (IEEE 1364-2005 sections 4.6.3 and 7.14.2), end to end.
//!
//! Icarus Verilog 13.0 does not run `trireg` ("sorry: trireg nets not
//! supported"), so every expectation here is a closed form from the clauses
//! each test cites rather than a probe's output.

use super::*;

/// Sections 4.2.1, 4.4.1, 4.6.3 and 7.13.2: a `trireg` starts at `x` at its
/// charge strength, takes its driver's value at the driver's strength, and,
/// once the driver lets go, holds that value at its charge strength: `small`,
/// `medium` (the default) or `large`.
#[test]
fn a_trireg_holds_its_last_driven_value_at_its_charge_strength() {
    let report = bench(
        r#"`timescale 1ns/1ns
module top;
  reg d, en;
  trireg t;
  trireg (small) ts;
  trireg (large) tl;
  bufif1 (t, d, en);
  bufif1 (ts, d, en);
  bufif1 (tl, d, en);
  initial begin
    en = 0; d = 1;
    #1 $display("%v %v %v", t, ts, tl);
    en = 1;
    #1 $display("%v %v %v", t, ts, tl);
    en = 0;
    #1 $display("%v %v %v %b", t, ts, tl, t);
    d = 0; en = 1;
    #1 en = 0;
    #1 $display("%v %v %v %b", t, ts, tl, t);
  end
endmodule
"#,
        &[],
    );
    assert_eq!(
        stdout(&report),
        "MeX SmX LaX\n\
         St1 St1 St1\n\
         Me1 Sm1 La1 1\n\
         Me0 Sm0 La0 0\n"
    );
}

/// Section 4.6.3, the driven state: once a driver of a `trireg` has the value
/// 0, 1 or `x`, the resolved value of its drivers propagates into the net at
/// their strength, and the stored charge takes no part, so a `weak` driver
/// gives `We1` on a `large` `trireg` and a `medium` one alike. The capacitive
/// state: when every driver is `z` again, each holds that 1 at its own charge
/// strength. A three-state driver under an unknown control drives an `L`
/// (`St0` or `z`), so `tx` is either driven, `St0`, or holding its `Me1`:
/// the `x` from `St0` to `Me1`, `62X` (section 7.10.1).
#[test]
fn a_driven_trireg_takes_its_drivers_value_at_their_strength() {
    let report = bench(
        r#"`timescale 1ns/1ns
module top;
  reg c, w, d, en;
  trireg (large) tl;
  trireg tm, tx;
  bufif1 (tl, 1'b0, c);
  bufif1 (tm, 1'b0, c);
  assign (weak1, weak0) tl = w;
  assign (weak1, weak0) tm = w;
  bufif1 (tx, d, en);
  initial begin
    c = 1; w = 1'bz; d = 1; en = 1;
    #1 c = 0; en = 0;
    #1 $display("%v %v %v", tl, tm, tx);
    w = 1;
    #1 $display("%v %v", tl, tm);
    w = 1'bz; d = 0; en = 1'bx;
    #1 $display("%v %v %v %b", tl, tm, tx, tx);
  end
endmodule
"#,
        &[],
    );
    assert_eq!(
        stdout(&report),
        "La0 Me0 Me1\n\
         We1 We1\n\
         La1 Me1 62X x\n"
    );
}

/// Sections 4.6.3 and 7.14.2: a `(weak0, weak1)` continuous assignment
/// driving a `large` `trireg` gives `We1`, the driven state. When the driver
/// goes to `z` at 2 the net holds its last driven value at its charge
/// strength, `La1`, and the charge decays to `x` at 2 + 5 = 7, the decay time
/// after the drivers let go, still `La1` at 6.
#[test]
fn a_weakly_driven_trireg_holds_at_its_charge_strength_then_decays() {
    let report = bench(
        r#"`timescale 1ns/1ns
module top;
  reg w;
  trireg (large) #(0, 0, 5) t;
  assign (weak0, weak1) t = w;
  always @(t) $display("%0t %v", $time, t);
  initial begin
    w = 1'bz;
    #1 w = 1;
    #1 $display("%0t %v", $time, t);
    w = 1'bz;
    #4 $display("%0t %v", $time, t);
    #4 $finish;
  end
endmodule
"#,
        &[],
    );
    assert_eq!(stdout(&report), "1 We1\n2 We1\n6 La1\n7 LaX\n");
}

/// Section 7.14.2: a held 1 or 0 decays to `x` its charge decay time after
/// the drivers let go, at exactly that tick; a driver that drives the net
/// again before then cancels the decay, and letting go again starts it over.
#[test]
fn a_trireg_decays_at_its_charge_decay_time_unless_driven_again() {
    let report = bench(
        r#"`timescale 1ns/1ns
module top;
  reg d, en;
  trireg (large) #(0, 0, 5) t;
  bufif1 (t, d, en);
  always @(t) $display("%0t %v", $time, t);
  initial begin
    en = 0; d = 1;
    #1 en = 1;
    #9 en = 0;
    #20 en = 1;
    #2 en = 0;
    #3 en = 1;
    #1 en = 0;
    #10 $finish;
  end
endmodule
"#,
        &[],
    );
    assert_eq!(stdout(&report), "1 St1\n15 LaX\n30 St1\n41 LaX\n");
}

/// Section 4.6.3.1: switches join `trireg`s into a capacitive network, where
/// the larger charge wins and equal charges of different values make `x`;
/// once parted, each holds the shared value at its own size. Through a
/// resistive switch a `large` charge passes as `medium` (table 7-14), which
/// still outweighs a `small` one.
#[test]
fn switches_share_charge_between_triregs() {
    let report = bench(
        r#"`timescale 1ns/1ns
module top;
  reg c, b, r;
  trireg (large) la, la2;
  trireg (small) sm, sm2;
  trireg m1, m2;
  bufif1 (la, 1'b1, c);
  bufif1 (sm, 1'b0, c);
  bufif1 (m1, 1'b1, c);
  bufif1 (m2, 1'b0, c);
  bufif1 (la2, 1'b1, c);
  bufif1 (sm2, 1'b0, c);
  tranif1 (la, sm, b);
  tranif1 (m1, m2, b);
  rtranif1 (la2, sm2, r);
  initial begin
    b = 0; r = 0; c = 1;
    #1 c = 0;
    #1 $display("%v %v %v %v %v %v", la, sm, m1, m2, la2, sm2);
    b = 1; r = 1;
    #1 $display("%v %v %v %v %v %v", la, sm, m1, m2, la2, sm2);
    b = 0; r = 0;
    #1 $display("%v %v %v %v %v %v", la, sm, m1, m2, la2, sm2);
  end
endmodule
"#,
        &[],
    );
    assert_eq!(
        stdout(&report),
        "La1 Sm0 Me1 Me0 La1 Sm0\n\
         La1 La1 MeX MeX La1 Me1\n\
         La1 Sm1 MeX MeX La1 Sm1\n"
    );
}

/// Sections 4.6.3 and 4.6.3.1 in a switch island. `ta`'s own drivers are all
/// `z`, so its `large` charge takes part in the island's resolution and
/// outweighs the `weak` driver of `na` across the switch: both are `La1`.
/// `tb` has a `weak` driver of its own, so it is in the driven state and its
/// charge takes no part: `tb` and `nb` are that driver's `We0`. When that
/// driver goes to `z`, `tb` holds the 0 at its charge strength and shares it.
#[test]
fn only_a_trireg_its_own_drivers_leave_at_z_shares_its_charge() {
    let report = bench(
        r#"`timescale 1ns/1ns
module top;
  reg c, b, w, v;
  trireg (large) ta, tb;
  wire na, nb;
  bufif1 (ta, 1'b1, c);
  bufif1 (tb, 1'b1, c);
  assign (weak0, weak1) na = w;
  assign (weak0, weak1) tb = v;
  tranif1 (ta, na, b);
  tranif1 (tb, nb, b);
  initial begin
    c = 1; b = 0; w = 0; v = 1'bz;
    #1 c = 0;
    #1 b = 1; v = 0;
    #1 $display("%v %v %v %v", ta, na, tb, nb);
    v = 1'bz;
    #1 $display("%v %v %v %v", ta, na, tb, nb);
  end
endmodule
"#,
        &[],
    );
    assert_eq!(stdout(&report), "La1 La1 We0 We0\nLa1 La1 La0 La0\n");
}

/// Sections 4.6.3.1 and 7.14.2: a charge shared through a switch is still a
/// charge, not a driver, so the joined `trireg`s keep decaying; when the
/// larger one decays, the shared value goes with it.
#[test]
fn shared_charge_decays_with_the_larger_trireg() {
    let report = bench(
        r#"`timescale 1ns/1ns
module top;
  reg c, b;
  trireg (large) #(0, 0, 10) da;
  trireg (small) #(0, 0, 100) ds;
  bufif1 (da, 1'b1, c);
  bufif1 (ds, 1'b0, c);
  tranif1 (da, ds, b);
  initial begin
    b = 0; c = 1;
    #1 c = 0;
    #1 b = 1;
    #8 $display("%0t %v %v", $time, da, ds);
    #2 $display("%0t %v %v", $time, da, ds);
  end
endmodule
"#,
        &[],
    );
    assert_eq!(stdout(&report), "10 La1 La1\n12 LaX LaX\n");
}

/// A `trireg` port whole on a wire makes them one `trireg` net (IEEE
/// 1364-2005 section 12.3.10), which holds charge in both modules. Joined to
/// a bit of a vector instead, it would resolve as part of that net without its
/// charge (section 4.6.3), so it is refused by name.
#[test]
fn a_trireg_port_joins_a_wire_or_is_refused() {
    let report = bench(
        "module child(p);\n  inout p;\n  trireg (large) p;\nendmodule\n\
         module top;\n  reg d, en;\n  wire w;\n  bufif1 (w, d, en);\n  child c(w);\n  \
         initial begin d = 1; en = 1; #1 en = 0; #1 $display(\"%v\", w); end\nendmodule\n",
        &[],
    );
    assert_eq!(stdout(&report), "La1\n");
    let sources = vec![(
        std::path::PathBuf::from("tb.v"),
        "module child(p);\n  inout p;\n  trireg p;\nendmodule\n\
         module top;\n  wire [1:0] v;\n  child c(v[1]);\n  initial #1 $display(\"%v\", v);\nendmodule\n"
            .to_owned(),
    )];
    let error = run_digital_testbench(&sources, &HdlTestbenchOptions::default())
        .expect_err("a trireg port on a vector bit is refused")
        .to_string();
    assert!(
        error.contains("is a `trireg` connected to another net through a port"),
        "{error}"
    );
}

/// Sections 4.6.3, 7.10.1 and 7.10.3: a three-state driver under an `x`
/// enable drives `L` or `H`, its value or nothing, so its `trireg` is either
/// driven, at the driver's strength with the charge taking no part, or
/// capacitive, at its charge. The net carries the range covering both.
///
/// | net | stored    | driven | on    | off   | covered        | `%v`  |
/// |-----|-----------|--------|-------|-------|----------------|-------|
/// | `a` | 0, medium | `StL`  | `St0` | `Me0` | `St0` to `Me0` | `620` |
/// | `b` | 1, medium | `StL`  | `St0` | `Me1` | `St0` to `Me1` | `62X` |
/// | `c` | 0, medium | `StH`  | `St1` | `Me0` | `Me0` to `St1` | `26X` |
/// | `e` | 1, medium | `StH`  | `St1` | `Me1` | `Me1` to `St1` | `621` |
/// | `w` | 1, large  | `WeL`  | `We0` | `La1` | `LaX`          | `LaX` |
///
/// The charge is the net's own value (section 4.6.3), and where the cases
/// disagree that value is `x`, so the held case is `x` at the charge
/// strength. For `a` to `e` the driver outweighs it and the range is the one
/// above; `w`'s weak driver does not, and `We0` against `LaX` is `LaX`. A
/// plain resolution of `WeL` against `La1` would give `La1`, which the
/// driver being on rules out. Once the enable is 0 each net holds the value
/// it had at its charge strength. Icarus Verilog 13.0 does not run `trireg`,
/// so every value is the closed form.
#[test]
fn a_trireg_under_an_unknown_enable_covers_driven_and_held() {
    let report = bench(
        r#"`timescale 1ns/1ns
module top;
  reg en, da, db, dc, de, dw;
  trireg a, b, c, e;
  trireg (large) w;
  bufif1 (a, da, en);
  bufif1 (b, db, en);
  bufif1 (c, dc, en);
  bufif1 (e, de, en);
  bufif1 (weak0, weak1) g (w, dw, en);
  initial begin
    en = 1; da = 0; db = 1; dc = 0; de = 1; dw = 1;
    #1 en = 0;
    #1 $display("%v %v %v %v %v", a, b, c, e, w);
    da = 0; db = 0; dc = 1; de = 1; dw = 0;
    en = 1'bx;
    #1 $display("%v %v %v %v %v %b%b%b%b%b", a, b, c, e, w, a, b, c, e, w);
    en = 0;
    #1 $display("%v %v %v %v %v", a, b, c, e, w);
  end
endmodule
"#,
        &[],
    );
    assert_eq!(
        stdout(&report),
        "Me0 Me1 Me0 Me1 La1\n\
         620 62X 26X 621 LaX 0xx1x\n\
         Me0 MeX MeX Me1 LaX\n"
    );
}

/// Section 7.14.2: a bit only an `x`-enabled driver drives may have been
/// let go, so its charge decay runs from the moment the enable became `x`.
/// Until the decay the net is `St0` or `Me0`; after it, the held case is
/// `x`, and the net covers `St0` to `MeX`. An enable of 1 drives it outright.
#[test]
fn a_trireg_an_unknown_enable_may_release_still_decays() {
    let report = bench(
        r#"`timescale 1ns/1ns
module top;
  reg d, en;
  trireg #(0, 0, 5) t;
  bufif1 (t, d, en);
  initial begin
    en = 0; d = 0;
    #1 en = 1;
    #1 $display("%0t %v", $time, t);
    en = 1'bx;
    #4 $display("%0t %v", $time, t);
    #2 $display("%0t %v", $time, t);
    en = 1;
    #1 $display("%0t %v", $time, t);
  end
endmodule
"#,
        &[],
    );
    assert_eq!(stdout(&report), "2 St0\n6 620\n8 62X\n9 St0\n");
}
