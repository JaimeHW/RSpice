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
