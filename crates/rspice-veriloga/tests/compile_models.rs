//! End-to-end compilation tests for representative Verilog-A models.

use rspice_veriloga::{CompilerOptions, VerilogACompiler};

fn compile(source: &str) -> rspice_veriloga::CompiledModel {
    VerilogACompiler::new(CompilerOptions::default())
        .compile(source)
        .expect("compilation failed")
}

#[test]
fn compiler_options_can_remove_standard_preprocessor_macros() {
    let mut options = CompilerOptions::default();
    options
        .undefines
        .push("__VAMS_COMPACT_MODELING__".to_string());
    options
        .defines
        .push(("EXTERNAL_PROFILE".to_string(), Some("1".to_string())));
    let model = VerilogACompiler::new(options)
        .compile(
            r#"
`ifdef __VAMS_COMPACT_MODELING__
module wrong_profile(p, n); endmodule
`else
`ifdef EXTERNAL_PROFILE
module selected_profile(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ V(p, n);
endmodule
`endif
`endif
"#,
        )
        .expect("configured preprocessor profile compiles");

    assert_eq!(model.name.as_str(), "selected_profile");
}

#[test]
fn resistor_with_standard_headers() {
    let model = compile(
        r#"
`include "disciplines.vams"
`include "constants.vams"

module my_resistor(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 1k from (0:inf);
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#,
    );

    assert_eq!(model.name.as_str(), "my_resistor");
    assert_eq!(model.num_terminals, 2);
    assert_eq!(model.parameters.len(), 1);
    assert_eq!(model.parameters[0].name.as_str(), "r");
    assert_eq!(model.parameters[0].default, 1e3);
    assert_eq!(model.stamp_programs.len(), 1);
}

#[test]
fn constants_header_supplies_physical_constants() {
    let model = compile(
        r#"
`include "disciplines.vams"
`include "constants.vams"

module q_source(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ `P_Q * 1e18 * V(p, n);
endmodule
"#,
    );
    assert_eq!(model.stamp_programs.len(), 1);
}

#[test]
fn define_in_raw_source_is_preprocessed() {
    let model = compile(
        r#"
`include "disciplines.vams"
`define COND 2.0

module cond_res(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ `COND * V(p, n);
endmodule
"#,
    );
    assert_eq!(model.num_terminals, 2);
}

#[test]
fn utf8_bom_at_source_start_is_ignored() {
    let model = compile(
        "\u{feff}module bom_res(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ V(p, n);
endmodule
",
    );

    assert_eq!(model.name.as_str(), "bom_res");
    assert_eq!(model.num_terminals, 2);
}
