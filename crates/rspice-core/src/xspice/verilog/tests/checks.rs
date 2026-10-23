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
