//! `defparam` end to end (IEEE 1364-2005 section 12.2.1; design P2 section
//! 4.2).
//!
//! Expected text is what Icarus Verilog 13.0 printed for the same source
//! (`iverilog -g2005`, `vvp -n`), an RSpice-authored probe.

use super::*;

const LEAF: &str = "`timescale 1ns/1ns\n\
     module leaf; parameter P = 1, Q = 2;\n\
     \x20 initial #(P) $display(\"%0t %m P=%0d Q=%0d\", $time, P, Q);\n\
     endmodule\n\
     module mid; parameter M = 5; leaf l();\n\
     \x20 initial #(M) $display(\"%0t %m M=%0d\", $time, M);\n\
     endmodule\n";

const TOP: &str = "`timescale 1ns/1ns\n\
     module top;\n\
     \x20 parameter T = 4;\n\
     \x20 leaf #(2, 3) u1();\n\
     \x20 leaf #(.Q(7)) u2();\n\
     \x20 mid m();\n\
     \x20 leaf u3();\n\
     \x20 defparam u1.P = 10;\n\
     \x20 defparam u2.P = T + 1, m.l.Q = 30;\n\
     \x20 defparam u3.P = 11;\n\
     \x20 defparam u3.P = 12;\n\
     \x20 defparam m.M = 6;\n\
     endmodule\n";

const EXPECTED: &str = "1 top.m.l P=1 Q=30\n\
     5 top.u2 P=5 Q=7\n\
     6 top.m M=6\n\
     10 top.u1 P=10 Q=3\n\
     12 top.u3 P=12 Q=2\n";

/// A `defparam` overrides an instance's named and ordered `#(...)` values,
/// reads its own module's parameters, reaches through an instance of a
/// module instantiated once, and the last of two on one parameter wins.
#[test]
fn defparams_take_precedence_over_instance_overrides_as_icarus_runs_them() {
    let report = bench(&format!("{LEAF}{TOP}"), &[]);
    assert_eq!(stdout(&report), EXPECTED);
}

/// The `defparam`s and the modules they reach are in different files of one
/// compilation unit.
#[test]
fn a_defparam_reaches_a_module_another_file_declares() {
    let report = testbench(
        &[("top.v", TOP), ("leaf.v", LEAF)],
        &HdlTestbenchOptions::default(),
    );
    assert_eq!(stdout(&report), EXPECTED);
}

const PORTED: &str = "`timescale 1ns/1ns\n\
     module bank #(parameter W = 2, parameter [3:0] K = 4'd3, N = 5);\n\
     \x20 parameter B = W + 1;\n\
     \x20 initial #(W) $display(\"%0t %m W=%0d K=%0d N=%0d B=%0d\", $time, W, K, N, B);\n\
     endmodule\n\
     module holder;\n\
     \x20 bank c();\n\
     \x20 defparam c.N = 9;\n\
     endmodule\n";

/// A module parameter port list (IEEE 1364-2005 section 12.1) declares the
/// overridable parameters, and a `parameter` of the body is local (section
/// 12.2). A module that only holds instances is not a root. The two lines at
/// 2 ns are simultaneous, so they are compared in sorted order.
#[test]
fn a_parameter_port_list_takes_overrides_and_defparams() {
    let report = bench(
        &format!(
            "{PORTED}module top;\n\
             \x20 bank #(.W(4)) u();\n\
             \x20 holder h1();\n\
             \x20 holder h2();\n\
             \x20 defparam u.K = 4'd7;\n\
             endmodule\n"
        ),
        &[],
    );
    let mut lines: Vec<_> = stdout(&report).lines().map(str::to_owned).collect();
    lines.sort();
    assert_eq!(
        lines,
        [
            "2 top.h1.c W=2 K=3 N=9 B=3",
            "2 top.h2.c W=2 K=3 N=9 B=3",
            "4 top.u W=4 K=7 N=5 B=5",
        ]
    );
}

/// With a parameter port list, an override of a body `parameter` is refused
/// (section 12.2), by `#(...)` and by `defparam` alike.
#[test]
fn a_body_parameter_of_a_ported_module_takes_no_override() {
    for top in [
        "module top; bank #(.B(7)) u(); endmodule\n",
        "module top; bank u(); defparam u.B = 7; endmodule\n",
    ] {
        let error = run_digital_testbench(
            &[(std::path::PathBuf::from("tb.v"), format!("{PORTED}{top}"))],
            &HdlTestbenchOptions::default(),
        )
        .expect_err("a local parameter takes no override");
        let message = error.to_string();
        assert!(message.contains('B'), "{message}");
    }
}
