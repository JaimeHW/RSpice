//! Module selection in multi-module files.
//!
//! Foundry releases (PSP, ...) ship several modules per .va file. Without
//! an explicit selection the compiler must never pick one arbitrarily:
//! exactly one module compiles implicitly, anything else is an error that
//! names the candidates.

use rspice_veriloga::{CompilerOptions, VerilogACompiler};

const TWO_MODULES: &str = r#"
`include "disciplines.vams"

module res_a(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 2.0 from (0:inf);
    analog I(p, n) <+ V(p, n) / r;
endmodule

module res_b(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 4.0 from (0:inf);
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#;

fn compiler() -> VerilogACompiler {
    VerilogACompiler::new(CompilerOptions::default())
}

#[test]
fn two_modules_without_selection_is_an_error_naming_both() {
    let err = compiler()
        .compile(TWO_MODULES)
        .expect_err("compiling a two-module file without a selection must fail");

    let message = err.to_string();
    assert!(
        message.contains("res_a") && message.contains("res_b"),
        "error must name both modules, got: {message}"
    );
    assert!(
        message.contains("select one"),
        "error must tell the user to select a module, got: {message}"
    );
}

#[test]
fn two_modules_with_explicit_selection_compiles_the_named_one() {
    let model_a = compiler()
        .compile_module(TWO_MODULES, Some("res_a"))
        .expect("selecting res_a must compile");
    assert_eq!(model_a.name.as_str(), "res_a");
    assert_eq!(model_a.parameters[0].default, 2.0);

    let model_b = compiler()
        .compile_module(TWO_MODULES, Some("res_b"))
        .expect("selecting res_b must compile");
    assert_eq!(model_b.name.as_str(), "res_b");
    assert_eq!(model_b.parameters[0].default, 4.0);
}

#[test]
fn selecting_an_unknown_module_is_an_error_naming_the_candidates() {
    let err = compiler()
        .compile_module(TWO_MODULES, Some("res_c"))
        .expect_err("selecting an undeclared module must fail");

    let message = err.to_string();
    assert!(
        message.contains("res_c") && message.contains("res_a") && message.contains("res_b"),
        "error must name the missing module and the candidates, got: {message}"
    );
}

#[test]
fn single_module_compiles_without_selection() {
    let source = r#"
`include "disciplines.vams"
module lone(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 8.0 from (0:inf);
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#;
    let model = compiler().compile(source).expect("single module compiles");
    assert_eq!(model.name.as_str(), "lone");

    // An explicit selection of the single module also works
    let model = compiler()
        .compile_module(source, Some("lone"))
        .expect("explicit selection of the single module compiles");
    assert_eq!(model.name.as_str(), "lone");
}
