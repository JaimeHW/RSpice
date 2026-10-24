//! SystemVerilog packages and imports (IEEE 1800-2017 clause 26) and real
//! nettypes (section 6.6.7), end to end.
//!
//! The visibility cases print what Icarus Verilog 13.0 printed for the same
//! sources (`iverilog -g2012`, `vvp -n`) in RSpice-authored probes, or are
//! refused where it refused them. Icarus 13 does not parse `nettype`, so the
//! nettype cases are closed forms. Each source selects SystemVerilog with
//! `` `begin_keywords "1800-2017" `` (section 22.14), as a `.v` file must.

use super::*;

const PACKAGES: &str = r#"`begin_keywords "1800-2017"
package p;
  localparam real K = 1.5;
  localparam real L = K * 2.0;
endpackage
package q;
  localparam real K = 7.0;
endpackage
`end_keywords
"#;

/// The packages above, then `module`, as two files of one unit.
fn with_packages(module: &str) -> Result<HdlTestbenchReport, DigitalRunError> {
    let sources = [
        (std::path::PathBuf::from("pkgs.v"), PACKAGES.to_owned()),
        (
            std::path::PathBuf::from("m.v"),
            format!("`begin_keywords \"1800-2017\"\n{module}`end_keywords\n"),
        ),
    ];
    run_digital_testbench(&sources, &HdlTestbenchOptions::default())
}

fn printed(module: &str) -> String {
    stdout(&with_packages(module).expect("the testbench runs"))
}

fn refused(module: &str) -> String {
    match with_packages(module) {
        Err(DigitalRunError::Compile { detail }) => detail,
        Err(error) => panic!("refused, but not by the compiler: {error}"),
        Ok(report) => panic!("ran: {:?}", stdout(&report)),
    }
}

/// Section 26.3: a local declaration hides a wildcard-imported name when it
/// is declared before any use, and the package's own parameter keeps its
/// package's meaning of the name: `p::L` is `p::K * 2.0` = 3.0, not 5.0.
#[test]
fn a_local_declaration_hides_a_wildcard_import() {
    assert_eq!(
        printed(
            "module shadow;
  import p::*;
  localparam real K = 2.5;
  initial $display(\"shadow K=%f L=%f\", K, L);
endmodule
"
        ),
        "shadow K=2.500000 L=3.000000\n"
    );
}

/// Section 26.3: an explicitly imported name may not also be declared in
/// the importing scope. Icarus: "'K' has already been imported into this
/// scope from package 'p'".
#[test]
fn an_explicit_import_and_a_local_declaration_of_one_name_conflict() {
    let detail = refused(
        "module explicit_conflict;
  import p::K;
  localparam real K = 2.5;
  initial $display(\"%f\", K);
endmodule
",
    );
    assert!(
        detail.contains("import p::K;") && detail.contains("26.3"),
        "{detail}"
    );
}

/// Section 26.3: a name two wildcard imports provide is ambiguous where it
/// is used, and nowhere else. Icarus: "Ambiguous use of 'K'. It is exported
/// by both 'p' and by 'q'", and `unref L=3.000000` without the use.
#[test]
fn two_wildcard_sources_are_ambiguous_only_where_the_name_is_used() {
    let detail = refused(
        "module ambiguous;
  import p::*;
  import q::*;
  initial $display(\"%f\", K);
endmodule
",
    );
    assert!(
        detail.contains("`K` is ambiguous")
            && detail.contains("package `p`")
            && detail.contains("package `q`"),
        "{detail}"
    );
    assert_eq!(
        printed(
            "module unreferenced;
  import p::*;
  import q::*;
  initial $display(\"unref L=%f\", L);
endmodule
"
        ),
        "unref L=3.000000\n"
    );
}

/// Section 26.3: a use that a wildcard import satisfies, before the local
/// declaration of the name, binds the import, and the declaration then
/// conflicts with it. Icarus refuses this source too.
#[test]
fn a_use_before_the_local_declaration_binds_the_import() {
    let detail = refused(
        "module before_local;
  import p::*;
  initial $display(\"%f\", K);
  localparam real K = 2.5;
endmodule
",
    );
    assert!(
        detail.contains("before its declaration") && detail.contains("package `p`"),
        "{detail}"
    );
}

/// `p::name` reads a package's declaration without importing it, and an
/// import in the module header (section 26.4) serves the port and parameter
/// lists as well as the body.
#[test]
fn scoped_names_and_header_imports_reach_package_parameters() {
    assert_eq!(
        printed(
            "module scoped;
  initial $display(\"scoped p::L=%f q::K=%f\", p::L, q::K);
endmodule
"
        ),
        "scoped p::L=3.000000 q::K=7.000000\n"
    );
    assert_eq!(
        printed(
            "module header import p::*; ();
  initial $display(\"header L=%f\", L);
endmodule
"
        ),
        "header L=3.000000\n"
    );
}

/// The HDL sources of a run are one compilation unit, and its packages are
/// collected before any name is bound, so an importer may come first. That
/// is this compiler's rule: Icarus 13 reads files in order and refuses the
/// import of a package it has not yet read.
#[test]
fn an_importer_may_precede_its_package() {
    let sources = [
        (
            std::path::PathBuf::from("m.v"),
            "`begin_keywords \"1800-2017\"\nmodule early;\n  import p::*;\n  initial \
             $display(\"early L=%f\", L);\nendmodule\n`end_keywords\n"
                .to_owned(),
        ),
        (std::path::PathBuf::from("pkgs.v"), PACKAGES.to_owned()),
    ];
    let report = run_digital_testbench(&sources, &HdlTestbenchOptions::default()).expect("it runs");
    assert_eq!(stdout(&report), "early L=3.000000\n");
}

/// A real nettype net (section 6.6.7) carries what its one driver drives,
/// through an alias of the nettype too, and a process reads it.
#[test]
fn a_real_nettype_net_carries_its_driver() {
    let report = bench(
        r#"`begin_keywords "1800-2017"
package pk;
  nettype real rnet;
endpackage
module top;
  import pk::*;
  nettype rnet r2;
  rnet n;
  r2 m;
  pk::rnet o;
  real v;
  assign n = v;
  assign m = 2.0 * n;
  assign o = m + 0.25;
  initial begin
    v = 1.25;
    #1 $display("n=%f m=%f o=%f", n, m, o);
  end
endmodule
`end_keywords
"#,
        &[],
    );
    assert_eq!(stdout(&report), "n=1.250000 m=2.500000 o=2.750000\n");
}
