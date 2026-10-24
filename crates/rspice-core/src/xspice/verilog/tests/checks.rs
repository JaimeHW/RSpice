//! Timing checks and notifiers of `specify` blocks (IEEE 1364-2005 section
//! 15), end to end.
//!
//! Icarus Verilog does not evaluate timing checks, so every expected record
//! here is the LRM's closed form, worked out in each test's comment from the
//! stimulus times. The windows are those of OVI Verilog LRM 1.0 B.9, which
//! IEEE 1364-2005 section 15 carries forward. Two are inferred, pending the
//! IEEE text: `$removal`'s closed end, by symmetry with `$recovery`, and
//! `$nochange`'s window, open at both ends.

use super::*;

/// The warning records, in order.
fn warnings(report: &HdlTestbenchReport) -> Vec<String> {
    report
        .records
        .iter()
        .filter(|record| {
            record.stream
                == HdlStream::Diagnostic {
                    severity: HdlSeverity::Warning,
                }
        })
        .map(|record| record.text.clone())
        .collect()
}

fn violations(check: &str, times: &[u32]) -> Vec<String> {
    times
        .iter()
        .map(|time| format!("u: {check} violated at {time} ns"))
        .collect()
}

const WINDOWS: &str = r#"`timescale 1ns/1ns
module w(input d, input c);
  specify
    $setup(d, posedge c, 3);
    $hold(posedge c, d, 2);
    $removal(posedge c, d, 3);
    $recovery(posedge c, d, 2);
    $setuphold(posedge c, d, 3, 2);
    $recrem(posedge c, d, 1, 4);
    $hold(posedge c, d, 0);
  endspecify
endmodule
module t;
  reg d, c;
  w u(d, c);
  initial begin
    d = 0; c = 0;
    #17 d = 1;
    #3 c = 1;
    #5 c = 0;
    #13 d = 0;
    #2 c = 1;
    #5 c = 0;
    #15 c = 1; d = 1;
    #5 c = 0;
    #15 c = 1;
    #1 d = 0;
    #4 c = 0;
    #15 c = 1;
    #2 d = 1;
    #3 c = 0;
    #15 c = 1;
    #0 d = 0;
    #5 c = 0;
    #15 d = 1;
    #0 c = 1;
    #10 $finish;
  end
endmodule
"#;

/// The stability windows, `rel` the data change's time less the rising
/// edge's: `$setup` -3 < rel < 0, `$hold` 0 <= rel < 2, `$removal`
/// -3 < rel <= 0, `$recovery` 0 < rel < 2, `$setuphold(3, 2)` -3 < rel < 2,
/// `$recrem(1, 4)` -4 < rel < 1, and a limit of 0 never violates.
///
/// - rel -3 (17, 20): only `$recrem`.
/// - rel -2 (38, 40): `$setup`, `$removal`, `$setuphold` and `$recrem`, at
///   the edge.
/// - rel 0 (60 in one statement, 120 and 140 each a `#0` apart): `$hold`,
///   `$removal`, `$setuphold` and `$recrem`; not `$setup` or `$recovery`.
/// - rel 1 (80, 81): `$hold`, `$recovery` and `$setuphold`.
/// - rel 2 (100, 102): none, every window ending before 2.
#[test]
fn stability_windows_follow_the_lrm_at_their_ends() {
    let report = bench(WINDOWS, &[]);
    let hold = "$hold(posedge c, d, 2)";
    let removal = "$removal(posedge c, d, 3)";
    let setuphold = "$setuphold(posedge c, d, 3, 2)";
    let recrem = "$recrem(posedge c, d, 1, 4)";
    let mut expected = violations(recrem, &[20]);
    expected.extend(violations("$setup(d, posedge c, 3)", &[40]));
    for check in [removal, setuphold, recrem] {
        expected.extend(violations(check, &[40]));
    }
    for time in [60] {
        for check in [hold, removal, setuphold, recrem] {
            expected.extend(violations(check, &[time]));
        }
    }
    for check in [hold, "$recovery(posedge c, d, 2)", setuphold] {
        expected.extend(violations(check, &[81]));
    }
    for time in [120, 140] {
        for check in [hold, removal, setuphold, recrem] {
            expected.extend(violations(check, &[time]));
        }
    }
    assert_eq!(warnings(&report), expected);
}

const CLOCKS: &str = r#"`timescale 1ns/1ns
module k(input c, input d, input s);
  specify
    $period(posedge c, 10);
    $width(posedge c, 4, 1);
    $skew(posedge c, s, 3);
    $nochange(posedge c, d, 2, 1);
  endspecify
endmodule
module t;
  reg c, d, s;
  k u(c, d, s);
  initial begin
    c = 0; d = 0; s = 0;
    #10 c = 1;
    #2 s = 1;
    #3 c = 0;
    #3 c = 1;
    #3 c = 0;
    #3 s = 0;
    #6 c = 1;
    #1 c = 0;
    #8 d = 1;
    #1 c = 1;
    #1 d = 0;
    #4 c = 0; d = 1;
    #1 d = 0;
    #12 d = 1;
    #2 c = 1;
    #5 c = 0;
    #5 $finish;
  end
endmodule
"#;

/// `$period`: rising edges at 10, 18, 30, 40 and 60, so only the gap of 8
/// at 18 is under 10 (10 at 40 is not). `$width`: high phases of 5, 3, 1,
/// 5 and 5, so only the 3 ending at 21 lies strictly between the threshold
/// of 1 and the limit of 4; the 1 ending at 31 is at the threshold. `$skew`:
/// `s` changes 2 after the edge at 10 and 6 after the one at 18 (24).
/// `$nochange`: the window runs from 2 before each rising edge to 1 after
/// the falling one. `d` at 39 is inside the window of the edge at 40,
/// violating at the edge. 41 is in its high phase, and 45 is at its falling
/// edge, before 46. 46 is that end and 58 the start of the next window, so
/// neither violates.
#[test]
fn clock_checks_follow_the_lrm() {
    let report = bench(CLOCKS, &[]);
    let mut expected = violations("$period(posedge c, 10)", &[18]);
    expected.extend(violations("$width(posedge c, 4, 1)", &[21]));
    expected.extend(violations("$skew(posedge c, s, 3)", &[24]));
    expected.extend(violations("$nochange(posedge c, d, 2, 1)", &[40, 41, 45]));
    assert_eq!(warnings(&report), expected);
}

const NOTIFIERS: &str = r#"`timescale 1ns/1ns
module n(input d, input c);
  reg ntf, nz;
  specify
    $setup(d, posedge c, 5, ntf);
    $hold(posedge c, d, 5, nz);
  endspecify
  initial nz = 1'bz;
  always @(ntf) $display("%0t ntf=%b", $time, ntf);
  initial #60 $display("end ntf=%b nz=%b", ntf, nz);
endmodule
module t;
  reg d, c;
  n u(d, c);
  initial begin
    d = 0; c = 0;
    #8 d = 1;
    #2 c = 1;
    #2 c = 0;
    #6 d = 0;
    #2 c = 1;
    #2 c = 0;
    #6 d = 1;
    #2 c = 1;
    #2 c = 0;
    #8 c = 1;
    #2 d = 0;
    #3 c = 0;
    #20 $finish;
  end
endmodule
"#;

/// Section 15.5: each violation toggles its notifier, `x` to 0, 0 to 1,
/// 1 to 0, and `z` stays `z`. `$setup` fails at the edges of 10, 20 and 30
/// (`d` 2 before each), and `$hold` at 42 (`d` 2 after the edge of 40).
#[test]
fn a_violation_toggles_its_notifier() {
    let report = bench(NOTIFIERS, &[]);
    assert_eq!(
        stdout(&report),
        "10 ntf=0\n20 ntf=1\n30 ntf=0\nend ntf=0 nz=z\n"
    );
    let mut expected = violations("$setup(d, posedge c, 5)", &[10, 20, 30]);
    expected.extend(violations("$hold(posedge c, d, 5)", &[42]));
    assert_eq!(warnings(&report), expected);
}

const SPECPARAM_LIMITS: &str = r#"`timescale 1ns/1ns
module n(input d, input c);
  specify
    specparam tsu = 3.0, th = 2;
    $setup(d, posedge c, tsu);
    $hold(posedge c, d, th);
  endspecify
endmodule
module t;
  reg d, c;
  n u(d, c);
  initial begin
    d = 0; c = 0;
    #8 d = 1;
    #2 c = 1;
    #5 c = 0;
    #2 d = 0;
    #3 c = 1;
    #5 c = 0;
    #5 c = 1;
    #1 d = 1;
    #4 c = 0;
    #5 c = 1;
    #2 d = 0;
    #5 $finish;
  end
endmodule
"#;

/// Section 15.2: a limit is a constant expression, and a specparam read only
/// by a timing check is one. With `$setup` -3 < rel < 0 and `$hold`
/// 0 <= rel < 2: `d` at 8 is 2 before the edge at 10 and fails `$setup`;
/// `d` at 17 is 3 before the edge at 20 and passes; `d` at 31 is 1 after the
/// edge at 30 and fails `$hold`; `d` at 42 is 2 after the edge at 40 and
/// passes.
#[test]
fn a_specparam_names_a_timing_check_limit() {
    let report = bench(SPECPARAM_LIMITS, &[]);
    let mut expected = violations("$setup(d, posedge c, 3)", &[10]);
    expected.extend(violations("$hold(posedge c, d, 2)", &[31]));
    assert_eq!(warnings(&report), expected);
}

const CONDITIONS: &str = r#"`timescale 1ns/1ns
module cnd(input d, input c, input e);
  specify
    $hold(posedge c &&& e, d, 5);
    $hold(posedge c &&& ~e, d, 5);
    $hold(posedge c &&& e === 1'b1, d, 5);
    $hold(posedge c &&& e !== 1'b1, d, 5);
    $hold(posedge c &&& e == 1'b1, d, 5);
    $hold(posedge c &&& e != 1'b1, d, 5);
  endspecify
endmodule
module t;
  reg d, c, e;
  cnd u(d, c, e);
  initial begin
    d = 0; c = 0; e = 1'bx;
    #10 c = 1;
    #2 d = 1;
    #3 c = 0; e = 1;
    #5 c = 1;
    #2 d = 0;
    #3 c = 0; e = 0;
    #5 c = 1;
    #2 d = 1;
    #3 c = 0; e = 1'bz;
    #5 c = 1;
    #2 d = 0;
    #10 $finish;
  end
endmodule
"#;

/// Section 15.6 and OVI Verilog LRM 1.0 B.9.10: `x` or `z` on the
/// conditioning signal enables a check only through the nondeterministic
/// `==` and `!=`, never through a bare signal, `~`, `===` or `!==`. With
/// `e` at `x` (edge at 10, `d` at 12) and at `z` (40, 42) only the last two
/// forms fail. With `e` at 1 (20, 22) the forms that hold for 1 fail, and
/// with `e` at 0 (30, 32) the others.
#[test]
fn an_unknown_condition_enables_only_a_nondeterministic_check() {
    let report = bench(CONDITIONS, &[]);
    let form = |condition: &str| format!("$hold(posedge c &&& {condition}, d, 5)");
    let mut expected = Vec::new();
    for (time, conditions) in [
        (12, &["e == 1'b1", "e != 1'b1"][..]),
        (22, &["e", "e === 1'b1", "e == 1'b1"][..]),
        (32, &["~e", "e !== 1'b1", "e != 1'b1"][..]),
        (42, &["e == 1'b1", "e != 1'b1"][..]),
    ] {
        for condition in conditions {
            expected.extend(violations(&form(condition), &[time]));
        }
    }
    assert_eq!(warnings(&report), expected);
}

const WIDE_CONDITIONS: &str = r#"`timescale 1ns/1ns
module wc(input d, input c, input [1:0] v, input e);
  specify
    $hold(posedge c &&& v, d, 5);
    $hold(posedge c &&& (v[1] & e), d, 5);
    $hold(posedge c &&& v + e == 1'b1, d, 5);
  endspecify
endmodule
module t;
  reg d, c, e;
  reg [1:0] v;
  wc u(d, c, v, e);
  initial begin
    d = 0; c = 0; v = 2'b01; e = 0;
    #10 c = 1;
    #2 d = 1;
    #3 c = 0; v = 2'b10; e = 1;
    #5 c = 1;
    #2 d = 0;
    #3 c = 0; v = 2'b0x; e = 0;
    #5 c = 1;
    #2 d = 1;
    #3 c = 0; v = 2'b11; e = 1;
    #5 c = 1;
    #2 d = 0;
    #5 $finish;
  end
endmodule
"#;

/// Section 15.6: a condition reads the least significant bit of its
/// expression. `v` reads `v[0]`; `(v[1] & e)` is deterministic and
/// `v + e == 1'b1` compares the sum's low bit, nondeterministically. At the
/// edges at 10, 20, 30 and 40, `v` and `e` are 01 and 0, 10 and 1, 0x and 0,
/// and 11 and 1, so the low bit of `v + e` is 1, 1, `x` and 0: `d` 2 after
/// each edge violates the first check at 12 and 42, the second at 22 and
/// 42, and the third at 12, 22 and 32.
#[test]
fn a_condition_reads_the_low_bit_of_any_expression() {
    let report = bench(WIDE_CONDITIONS, &[]);
    let vector = "$hold(posedge c &&& v, d, 5)";
    let compound = "$hold(posedge c &&& (v[1] & e), d, 5)";
    let sum = "$hold(posedge c &&& v + e == 1'b1, d, 5)";
    let mut expected = Vec::new();
    for (time, checks) in [
        (12, &[vector, sum][..]),
        (22, &[compound, sum][..]),
        (32, &[sum][..]),
        (42, &[vector, compound][..]),
    ] {
        for check in checks {
            expected.extend(violations(check, &[time]));
        }
    }
    assert_eq!(warnings(&report), expected);
}

const DELAYED: &str = r#"`timescale 1ns/1ns
module ff(output reg q, input d, input clk);
  wire dclk, dd;
  reg ntf;
  specify
    $setuphold(posedge clk, d, -2, 5, ntf, , , dclk, dd);
  endspecify
  always @(posedge dclk) q <= dd;
endmodule
module t;
  reg d, clk;
  wire q;
  ff u(q, d, clk);
  initial begin
    $monitor("%0t clk=%b d=%b q=%b", $time, clk, d, q);
    d = 0; clk = 0;
    #10 clk = 1;
    #1 d = 1;
    #4 clk = 0;
    #5 clk = 1;
    #3 d = 0;
    #2 clk = 0;
    #10 $finish;
  end
endmodule
"#;

/// Section 15.8: a setup limit of -2 delays the flop's clock copy `dclk` by
/// 2 and leaves `dd` undelayed. The flop therefore samples `d` 2 after each
/// edge of `clk`: it takes the 1 that arrives at 11 at 12. The window
/// shrinks to [2, 5) after the edge: `d` at 11 (1 after the edge at 10) is
/// legal, and `d` at 23 (3 after the edge at 20) violates.
///
/// With timing checks left out, `dclk` and `dd` are plain copies. The flop
/// then takes `d` at each edge of `clk` itself, as Icarus Verilog runs the
/// same source, and nothing is reported.
#[test]
fn a_negative_limit_delays_the_signals_a_model_reads() {
    let report = bench(DELAYED, &[]);
    assert_eq!(
        stdout(&report),
        "\
0 clk=0 d=0 q=x
10 clk=1 d=0 q=x
11 clk=1 d=1 q=x
12 clk=1 d=1 q=1
15 clk=0 d=1 q=1
20 clk=1 d=1 q=1
23 clk=1 d=0 q=1
25 clk=0 d=0 q=1
"
    );
    assert_eq!(
        warnings(&report),
        violations("$setuphold(posedge clk, d, -2, 5)", &[23])
    );
    let unchecked = testbench(
        &[("tb.v", DELAYED)],
        &HdlTestbenchOptions {
            timing_checks: false,
            ..HdlTestbenchOptions::default()
        },
    );
    assert_eq!(
        stdout(&unchecked),
        "\
0 clk=0 d=0 q=x
10 clk=1 d=0 q=0
11 clk=1 d=1 q=0
15 clk=0 d=1 q=0
20 clk=1 d=1 q=1
23 clk=1 d=0 q=1
25 clk=0 d=0 q=1
"
    );
    assert!(
        warnings(&unchecked).is_empty(),
        "{:?}",
        warnings(&unchecked)
    );
}

const SHARED: &str = r#"`timescale 1ns/1ns
module ff(output reg q, input d, input e, input clk);
  wire dclk, dd, de;
  specify
    $setuphold(posedge clk, d, -2, 20, , , , dclk, dd);
    $setuphold(posedge clk, e, -10, 20, , , , dclk, de);
  endspecify
  always @(posedge dclk) q <= dd;
endmodule
module t;
  reg d, e, clk;
  wire q;
  ff u(q, d, e, clk);
  initial begin
    $monitor("%0t q=%b", $time, q);
    d = 0; e = 0; clk = 0;
    #10 clk = 1;
    #1 d = 1;
    #4 d = 0;
    #3 e = 1;
    #2 e = 0;
    #20 $finish;
  end
endmodule
"#;

/// Section 15.5.1: both checks name `dclk`, so its delay satisfies both. The
/// least delays with `r - d` at least 2 (the first check's setup of -2) and
/// at least 10 (the second's) delay `dclk` by 10 and neither data copy: the
/// second check's limit sets the delay. The flop therefore samples `d` at 20,
/// 10 after the edge at 10, and takes 0, not the 1 it held at 12.
///
/// The first check's window stays (2, 20) after the edge, but its delayed
/// signals meet at 10, inside it, so it is open at 2: `d` at 11 (rel 1) is
/// legal and `d` at 15 (rel 5) violates. The second's meet at 10, its closed
/// end: `e` at 18 (rel 8) is legal and `e` at 20 (rel 10) violates.
#[test]
fn a_shared_delayed_signal_takes_the_delay_every_check_needs() {
    let report = bench(SHARED, &[]);
    assert_eq!(stdout(&report), "0 q=x\n20 q=0\n");
    let mut expected = violations("$setuphold(posedge clk, d, -2, 20)", &[15]);
    expected.extend(violations("$setuphold(posedge clk, e, -10, 20)", &[20]));
    assert_eq!(warnings(&report), expected);
}

/// A.7.5.2: a delayed signal may be one bit of a net, and copies the bit of
/// its signal the check names. With non-negative limits the copies are
/// undelayed: `w[0]` follows `d[1]` and `w[1]` follows `d[0]`, so `w` is `d`
/// reversed.
#[test]
fn a_delayed_bit_copies_the_bit_its_check_names() {
    let report = bench(
        r#"`timescale 1ns/1ns
module bs(output [1:0] q, input [1:0] d, input clk);
  wire [1:0] w;
  specify
    $setuphold(posedge clk, d[1], 1, 1, , , , , w[0]);
    $setuphold(posedge clk, d[0], 1, 1, , , , , w[1]);
  endspecify
  assign q = w;
endmodule
module t;
  reg [1:0] d;
  reg clk;
  wire [1:0] q;
  bs u(q, d, clk);
  initial begin
    $monitor("%0t q=%b", $time, q);
    clk = 0; d = 2'b01;
    #10 d = 2'b11;
    #10 d = 2'b10;
    #5 $finish;
  end
endmodule
"#,
        &[],
    );
    assert_eq!(stdout(&report), "0 q=10\n10 q=11\n20 q=01\n");
    assert!(warnings(&report).is_empty(), "{:?}", warnings(&report));
}

/// Section 15.5.1: no delays satisfy both checks, whose delayed signals
/// meet at least 10 and at most -3 after the edge. The smallest negative
/// limit, the first check's -10, is taken as 0; still none do, so the next,
/// the second check's -3, is too. The windows are then [0, 20) and (-5, 0)
/// after the edge at 10, both checks warned of at the start: `d` at 7 (rel
/// -3) violates the second at the edge, and `d` at 12 (rel 2) the first.
#[test]
fn inconsistent_negative_limits_are_taken_as_zero_smallest_first() {
    let report = bench(
        r#"`timescale 1ns/1ns
module inc(input d, input clk);
  wire dclk, dd;
  specify
    $setuphold(posedge clk, d, -10, 20, , , , dclk, dd);
    $setuphold(posedge clk, d, 5, -3, , , , dclk, dd);
  endspecify
endmodule
module t;
  reg d, clk;
  inc u(d, clk);
  initial begin d = 0; clk = 0; #7 d = 1; #3 clk = 1; #2 d = 0; #5 $finish; end
endmodule
"#,
        &[],
    );
    let first = "$setuphold(posedge clk, d, -10, 20)";
    let second = "$setuphold(posedge clk, d, 5, -3)";
    let mut expected: Vec<String> = [first, second]
        .iter()
        .map(|check| {
            format!(
                "u: {check}: no delays of its delayed signals satisfy its limits and those of \
                 the checks sharing them, so the smallest negative limit among them is taken as \
                 0 (IEEE 1364-2005 section 15.5.1)"
            )
        })
        .collect();
    expected.extend(violations(second, &[10]));
    expected.extend(violations(first, &[12]));
    assert_eq!(warnings(&report), expected);
}

/// Sections 15.2.3 and 15.5.2: with a setup limit of -2 the window is (2, 6)
/// after the edge, so the edge is the hold check's timestamp event. At the
/// edge at 10 the timestamp condition `s` is 0, so the edge records no time
/// and `d` at 13 (rel 3) has no edge to violate. With `s` at 1 the edge at
/// 20 records its time, and `d` at 23 (rel 3) violates.
#[test]
fn a_false_timestamp_condition_records_no_reference_edge() {
    let report = bench(
        r#"`timescale 1ns/1ns
module ts(input d, input clk, input s);
  specify $setuphold(posedge clk, d, -2, 6, , s, ); endspecify
endmodule
module t;
  reg d, clk, s;
  ts u(d, clk, s);
  initial begin
    d = 0; clk = 0; s = 0;
    #10 clk = 1;
    #3 d = 1;
    #2 clk = 0; s = 1;
    #5 clk = 1;
    #3 d = 0;
    #5 $finish;
  end
endmodule
"#,
        &[],
    );
    assert_eq!(
        warnings(&report),
        violations("$setuphold(posedge clk, d, -2, 6)", &[23])
    );
}

/// Sections 15.2.6 and 15.5.2: a removal limit of -2 with a recovery limit
/// of 6 leaves the window (2, 6) after the edge at 10, and the data event is
/// the recovery check's timecheck event. With the timecheck condition `k` at
/// 0, `d` at 13 (rel 3) checks nothing; with `k` back at 1, `d` at 15 (rel
/// 5) violates against the same edge.
#[test]
fn a_false_timecheck_condition_checks_no_data_edge() {
    let report = bench(
        r#"`timescale 1ns/1ns
module tc(input d, input clk, input k);
  specify $recrem(posedge clk, d, 6, -2, , , k); endspecify
endmodule
module t;
  reg d, clk, k;
  tc u(d, clk, k);
  initial begin
    d = 0; clk = 0; k = 1;
    #10 clk = 1;
    #2 k = 0;
    #1 d = 1;
    #1 k = 1;
    #1 d = 0;
    #5 $finish;
  end
endmodule
"#,
        &[],
    );
    assert_eq!(
        warnings(&report),
        violations("$recrem(posedge clk, d, 6, -2)", &[15])
    );
}

/// A negative limit whose window is empty (setup -3 with hold 2) is taken as
/// 0, as the commercial simulators take it, and the run warns of it at its
/// start. The window is then [0, 2): `d` 1 after the edge at 10 violates.
/// Taking the limit as 0 is pending the IEEE text.
#[test]
fn an_empty_negative_window_takes_its_limit_as_zero() {
    let report = bench(
        r#"`timescale 1ns/1ns
module z(input d, input c);
  specify $setuphold(posedge c, d, -3, 2); endspecify
endmodule
module t;
  reg d, c;
  z u(d, c);
  initial begin d = 0; c = 0; #10 c = 1; #1 d = 1; #5 $finish; end
endmodule
"#,
        &[],
    );
    let check = "$setuphold(posedge c, d, -3, 2)";
    let mut expected = vec![format!(
        "u: {check}: its limits sum to no more than 0, so its negative limit is taken as 0 \
         (IEEE 1364-2005 section 15.8)"
    )];
    expected.extend(violations(check, &[11]));
    assert_eq!(warnings(&report), expected);
}

/// A.7.5.3: an event on a bit-select is a change of that bit only. With
/// `$setup` violated for -3 < rel < 0, `d[0]` changing at 10 is no event of
/// `d[1]`, so the edge at 12 violates only the whole-signal check; `d[1]` at
/// 23 against the edge at 24 (rel -1) violates both; `d[2]` at 30 against
/// the edge at 31 again only the whole-signal one. `d[B]`, `B` a localparam
/// of 1, is `d[1]`.
#[test]
fn a_bit_select_event_is_a_change_of_that_bit() {
    let report = bench(
        r#"`timescale 1ns/1ns
module b(input [3:0] d, input c);
  localparam B = 1;
  specify
    $setup(d, posedge c, 3);
    $setup(d[1], posedge c, 3);
    $setup(d[B], posedge c, 3);
  endspecify
endmodule
module t;
  reg [3:0] d; reg c;
  b u(d, c);
  initial begin
    d = 0; c = 0;
    #10 d = 4'b0001;
    #2 c = 1;
    #5 c = 0;
    #6 d = 4'b0011;
    #1 c = 1;
    #5 c = 0;
    #1 d = 4'b0111;
    #1 c = 1;
    #5 $finish;
  end
endmodule
"#,
        &[],
    );
    let mut expected = violations("$setup(d, posedge c, 3)", &[12]);
    expected.extend(violations("$setup(d, posedge c, 3)", &[24]));
    expected.extend(violations("$setup(d[1], posedge c, 3)", &[24]));
    expected.extend(violations("$setup(d[B], posedge c, 3)", &[24]));
    expected.extend(violations("$setup(d, posedge c, 3)", &[31]));
    assert_eq!(warnings(&report), expected);
}

/// A.7.5.3 and section 5.2.1: a part-select's event is a change of its bits,
/// and its edge one of the least significant of them. `d[2:1]`, `d[1+:2]`
/// and `d[2-:2]` all name bits 2 and 1 of `[3:0]`. With `$hold` violated
/// for 0 <= rel < 2 after the rising edges at 10, 20 and 30: `d[2]` at 11
/// (rel 1) changes the part but is no edge of its low bit `d[1]`; `d[1]`
/// rising at 21 (rel 1) is both; `d[0]` at 31 is neither.
#[test]
fn a_part_select_event_is_a_change_of_its_bits() {
    let report = bench(
        r#"`timescale 1ns/1ns
module p(input [3:0] d, input c);
  specify
    $hold(posedge c, d[2:1], 2);
    $hold(posedge c, d[1+:2], 2);
    $hold(posedge c, d[2-:2], 2);
    $hold(posedge c, posedge d[2:1], 2);
  endspecify
endmodule
module t;
  reg [3:0] d; reg c;
  p u(d, c);
  initial begin
    d = 0; c = 0;
    #10 c = 1;
    #1 d = 4'b0100;
    #4 c = 0;
    #5 c = 1;
    #1 d = 4'b0110;
    #4 c = 0;
    #5 c = 1;
    #1 d = 4'b0111;
    #5 $finish;
  end
endmodule
"#,
        &[],
    );
    let parts = ["d[2:1]", "d[1+:2]", "d[2-:2]"];
    let mut expected = Vec::new();
    for time in [11, 21] {
        for part in parts {
            expected.extend(violations(&format!("$hold(posedge c, {part}, 2)"), &[time]));
        }
        if time == 21 {
            expected.extend(violations("$hold(posedge c, posedge d[2:1], 2)", &[21]));
        }
    }
    assert_eq!(warnings(&report), expected);
}

/// `instance: check violated at time ns`, for each instance and time.
fn named_violations(check: &str, at: &[(&str, u32)]) -> Vec<String> {
    at.iter()
        .map(|(instance, time)| format!("{instance}: {check} violated at {time} ns"))
        .collect()
}

/// IEEE 1364-2005 section 15.3, `$timeskew(posedge c, d, 4)` under each
/// combination of its flags: `t0` neither, `t1` event-based, `t2`
/// event-based and remain-active, `t3` remain-active alone. The check is
/// violated when `d` changes more than 4 after the rising edge, and never by
/// simultaneous events.
///
/// - Edge at 10, `d` at 12 (2, within), 20 (10) and 22 (12): the timer-based
///   checks end at 12; `t1` reports 20 and goes dormant; `t2` reports 20 and
///   22.
/// - Edge at 40, `d` at 47 (7) and 49 (9): the timer-based checks report 45,
///   the first tick past 44; `t1` reports 47; `t2` 47 and 49.
/// - Edge at 60, `d` at 62 (within); edge and `d` at 80 (simultaneous); `d`
///   at 90 (10): the timer-based checks end at 62 and at 80; the
///   event-based ones report 90.
///
/// `t3`'s remain-active flag has nothing to keep active: a timer-based check
/// reports when its limit elapses, not at a late event.
#[test]
fn timeskew_reports_by_timer_or_by_event_per_its_flags() {
    let report = bench(
        r#"`timescale 1ns/1ns
module s0(input d, input c);
  specify $timeskew(posedge c, d, 4); endspecify
endmodule
module s1(input d, input c);
  parameter EVENT = 1;
  specify $timeskew(posedge c, d, 4, , EVENT); endspecify
endmodule
module s2(input d, input c);
  specify $timeskew(posedge c, d, 4, , 1'b1, 1); endspecify
endmodule
module s3(input d, input c);
  specify $timeskew(posedge c, d, 4, , 0, 1); endspecify
endmodule
module t;
  reg d, c;
  s0 t0(d, c);
  s1 t1(d, c);
  s2 t2(d, c);
  s3 t3(d, c);
  initial begin
    c = 0; d = 0;
    #10 c = 1;
    #2 d = 1;
    #8 d = 0;
    #2 d = 1;
    #8 c = 0;
    #10 c = 1;
    #7 d = 0;
    #2 d = 1;
    #6 c = 0;
    #5 c = 1;
    #2 d = 0;
    #13 c = 0;
    #5 c = 1; d = 1;
    #10 d = 0;
    #10 $finish;
  end
endmodule
"#,
        &[],
    );
    assert_eq!(
        warnings(&report),
        named_violations(
            "$timeskew(posedge c, d, 4)",
            &[
                ("t1", 20),
                ("t2", 20),
                ("t2", 22),
                ("t0", 45),
                ("t3", 45),
                ("t1", 47),
                ("t2", 47),
                ("t2", 49),
                ("t1", 90),
                ("t2", 90),
            ]
        )
    );
}

/// IEEE 1364-2005 section 15.3, `$fullskew(posedge c, posedge e, 4, 2)`:
/// `e` may follow `c` by at most 4 and `c` follow `e` by at most 2, the
/// first of the two being the timestamp event. `f0` is timer-based, `f1`
/// event-based, `f2` event-based and remain-active. The rising edges of `c`
/// are at 10, 25, 41 and 55, and of `e` at 13, 20, 40, 50 and 52.
///
/// - `f0`: `c` at 10 is met by `e` at 13 (3). `e` at 20 starts the check
///   again, unmet by 22, so it reports 23; likewise `c` at 25 reports 30.
///   `e` at 40 is met by `c` at 41 (1). `e` at 50 starts it, `e` at 52
///   starts it again, and it reports 55; `c` at 55 then starts it, and it
///   reports 60.
/// - `f1`: `c` at 10, `e` at 13 within, `e` at 20 (10) reports and goes
///   dormant; `c` at 25 restarts and `e` at 40 (15) reports; `c` at 41
///   restarts and `e` at 50 (9) reports; `e` at 52, the check dormant, is a
///   timestamp event, and `c` at 55 (3) reports.
/// - `f2`: as `f1` to 50, but active still, so `e` at 52 (11 after `c` at
///   41) reports, and `c` at 55 restarts it.
#[test]
fn fullskew_takes_either_order_per_its_flags() {
    let report = bench(
        r#"`timescale 1ns/1ns
module g0(input e, input c);
  specify $fullskew(posedge c, posedge e, 4, 2); endspecify
endmodule
module g1(input e, input c);
  specify $fullskew(posedge c, posedge e, 4, 2, , 1); endspecify
endmodule
module g2(input e, input c);
  specify $fullskew(posedge c, posedge e, 4, 2, , 1, 1); endspecify
endmodule
module t;
  reg e, c;
  g0 f0(e, c);
  g1 f1(e, c);
  g2 f2(e, c);
  initial begin
    c = 0; e = 0;
    #10 c = 1;
    #3 e = 1;
    #5 c = 0;
    #1 e = 0;
    #1 e = 1;
    #5 c = 1;
    #10 c = 0;
    #2 e = 0;
    #3 e = 1;
    #1 c = 1;
    #4 c = 0;
    #4 e = 0;
    #1 e = 1;
    #1 e = 0;
    #1 e = 1;
    #3 c = 1;
    #10 $finish;
  end
endmodule
"#,
        &[],
    );
    assert_eq!(
        warnings(&report),
        named_violations(
            "$fullskew(posedge c, posedge e, 4, 2)",
            &[
                ("f1", 20),
                ("f2", 20),
                ("f0", 23),
                ("f0", 30),
                ("f1", 40),
                ("f2", 40),
                ("f1", 50),
                ("f2", 50),
                ("f2", 52),
                ("f0", 55),
                ("f1", 55),
                ("f0", 60),
            ]
        )
    );
}
