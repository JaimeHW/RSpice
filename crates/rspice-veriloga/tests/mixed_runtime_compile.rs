//! Opt-in executable compilation of both halves of one mixed module.

use rspice_veriloga::{CompilerOptions, VerilogACompiler};

const MIXED: &str = r#"
module mixed(p, n, clk, q);
  inout p, n; electrical p, n;
  input clk; output q; wire clk; reg q;
  initial q = 1'b0;
  always @(posedge clk) q <= ~q;
  analog I(p, n) <+ V(p, n) / 1000.0;
endmodule
"#;

#[test]
fn mixed_runtime_compilation_is_explicit_and_retains_both_domains() {
    let refused = VerilogACompiler::new(CompilerOptions::default())
        .compile_runtime(MIXED, None)
        .expect_err("ordinary analog runtime compilation must remain fail-closed");
    assert_eq!(
        refused.diagnostic_code(),
        "VA-CODEGEN-UNSUPPORTED-AMS-DIGITAL"
    );

    let runtime = VerilogACompiler::new(CompilerOptions {
        enable_ams: true,
        ..CompilerOptions::default()
    })
    .compile_runtime(MIXED, None)
    .expect("an opted-in mixed host owns digital execution");
    assert_eq!(runtime.model.stamp_programs.len(), 1);
    assert_eq!(runtime.canonical_ir.mir.equations.len(), 1);
    assert_eq!(runtime.canonical_ir.digital.processes.len(), 2);
}

#[test]
fn malformed_or_unsupported_mixed_content_still_fails_closed() {
    let compiler = VerilogACompiler::new(CompilerOptions {
        enable_ams: true,
        ..CompilerOptions::default()
    });
    assert!(
        compiler
            .compile_runtime("module broken(p); always @( endmodule", None)
            .is_err()
    );

    let shared_crossing = r#"
module unsupported(p, n, q);
  inout p, n; electrical p, n;
  output q; reg q; real shared;
  always @(q) q = shared;
  analog begin shared = V(p, n); I(p, n) <+ shared; end
endmodule
"#;
    let error = compiler
        .compile_runtime(shared_crossing, None)
        .expect_err("implicit real-to-four-state crossing has no bridge contract");
    assert!(
        error.to_string().contains("analog") || error.to_string().contains("continuous-domain"),
        "{error}"
    );
}
