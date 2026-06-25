//! End-to-end compilation tests for representative Verilog-A models.

use rspice_veriloga::{CompilerOptions, VerilogACompiler};

fn compile(source: &str) -> rspice_veriloga::CompiledModel {
    VerilogACompiler::new(CompilerOptions::default())
        .compile(source)
        .expect("compilation failed")
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
fn angle_bracket_terminal_branch_current_compiles() {
    let model = compile(
        r#"
`include "disciplines.vams"

module terminal_probe(p, n);
    inout p, n;
    electrical p, n;
    real ip;
    analog begin
        I(p, n) <+ V(p, n);
        ip = I(<p>);
        I(p, n) <+ 0.0 * ip;
    end
endmodule
"#,
    );

    assert_eq!(model.name.as_str(), "terminal_probe");
    assert_eq!(model.num_terminals, 2);
}

#[test]
fn constants_header_supplies_legacy_nist1998_aliases() {
    let model = compile(
        r#"
`include "disciplines.vams"
`include "constants.vams"

module nist_aliases(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ (`P_Q_NIST1998 / `P_K_NIST1998) * V(p, n);
endmodule
"#,
    );

    assert_eq!(model.name.as_str(), "nist_aliases");
    assert_eq!(model.stamp_programs.len(), 1);
}

#[test]
fn builtin_function_arity_errors_are_compile_time_diagnostics() {
    let err = VerilogACompiler::new(CompilerOptions::default())
        .compile(
            r#"
`include "disciplines.vams"

module bad_arity(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ sqrt(V(p, n), 2.0) + pow(V(p, n));
endmodule
"#,
        )
        .expect_err("wrong built-in function arity must be rejected");
    let text = err.to_string();
    assert!(
        text.contains("sqrt") || text.contains("pow"),
        "diagnostic should name the malformed function, got: {text}"
    );
}

#[test]
fn log10_builtin_arity_errors_are_compile_time_diagnostics() {
    let err = VerilogACompiler::new(CompilerOptions::default())
        .compile(
            r#"
`include "disciplines.vams"

module bad_log10_arity(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ log10(V(p, n), 2.0);
endmodule
"#,
        )
        .expect_err("wrong log10 arity must be rejected");
    let text = err.to_string();
    assert!(
        text.contains("log10"),
        "diagnostic should name the malformed function, got: {text}"
    );
}

#[test]
fn system_function_arity_errors_are_compile_time_diagnostics() {
    let err = VerilogACompiler::new(CompilerOptions::default())
        .compile(
            r#"
`include "disciplines.vams"

module bad_system_arity(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ $vt(300.15, 301.15) * V(p, n);
endmodule
"#,
        )
        .expect_err("wrong system function arity must be rejected");
    let text = err.to_string();
    assert!(
        text.contains("$vt") || text.contains("$thermal_vt"),
        "diagnostic should name the malformed system function, got: {text}"
    );
}

#[test]
fn unknown_system_task_is_a_compile_time_diagnostic() {
    let err = VerilogACompiler::new(CompilerOptions::default())
        .compile(
            r#"
`include "disciplines.vams"

module unknown_system_task(p, n);
    inout p, n;
    electrical p, n;
    analog begin
        $rshout_into_the_void(1.0);
        I(p, n) <+ V(p, n);
    end
endmodule
"#,
        )
        .expect_err("unknown system tasks must be rejected");
    let text = err.to_string();
    assert!(
        text.contains("$rshout_into_the_void"),
        "diagnostic should name the unknown system task, got: {text}"
    );
}

#[test]
fn system_task_arity_errors_are_compile_time_diagnostics() {
    for task in ["$bound_step(1.0, 2.0)", "$discontinuity(0, 1)"] {
        let source = format!(
            r#"
`include "disciplines.vams"

module bad_system_task_arity(p, n);
    inout p, n;
    electrical p, n;
    analog begin
        {task};
        I(p, n) <+ V(p, n);
    end
endmodule
"#,
        );
        let err = VerilogACompiler::new(CompilerOptions::default())
            .compile(&source)
            .expect_err("wrong system task arity must be rejected");
        let text = err.to_string();
        assert!(
            text.contains(task.split('(').next().unwrap()),
            "diagnostic should name the malformed system task {task}, got: {text}"
        );
    }
}

#[test]
fn discontinuity_argument_must_be_numeric() {
    let err = VerilogACompiler::new(CompilerOptions::default())
        .compile(
            r#"
`include "disciplines.vams"

module bad_discontinuity_arg(p, n);
    inout p, n;
    electrical p, n;
    analog begin
        $discontinuity("not a degree");
        I(p, n) <+ V(p, n);
    end
endmodule
"#,
        )
        .expect_err("$discontinuity must reject non-numeric arguments");
    let text = err.to_string();
    assert!(
        text.contains("$discontinuity") || text.contains("numeric"),
        "diagnostic should identify the malformed system task argument, got: {text}"
    );
}

#[test]
fn simparam_requires_string_name_even_with_default() {
    let err = VerilogACompiler::new(CompilerOptions::default())
        .compile(
            r#"
`include "disciplines.vams"

module bad_simparam_name(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ $simparam(V(p, n), 1.0) * V(p, n);
endmodule
"#,
        )
        .expect_err("$simparam must reject non-string query names");
    let text = err.to_string();
    assert!(
        text.contains("$simparam"),
        "diagnostic should name the malformed system function, got: {text}"
    );
}

#[test]
fn port_connected_requires_terminal_name() {
    let err = VerilogACompiler::new(CompilerOptions::default())
        .compile(
            r#"
`include "disciplines.vams"

module bad_port_connected(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ ($port_connected(1.0) ? 1.0 : 0.0) * V(p, n);
endmodule
"#,
        )
        .expect_err("$port_connected must reject non-terminal arguments");
    let text = err.to_string();
    assert!(
        text.contains("$port_connected"),
        "diagnostic should name the malformed system function, got: {text}"
    );
}

#[test]
fn analog_operator_extra_args_are_compile_time_diagnostics() {
    let err = VerilogACompiler::new(CompilerOptions::default())
        .compile(
            r#"
`include "disciplines.vams"

module bad_operator_arity(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ ddt(V(p, n), 1.0);
endmodule
"#,
        )
        .expect_err("unsupported analog operator arguments must be rejected");
    let text = err.to_string();
    assert!(
        text.contains("ddt"),
        "diagnostic should name the malformed analog operator, got: {text}"
    );
}

#[test]
fn keyword_analog_operator_extra_args_are_compile_time_diagnostics() {
    let err = VerilogACompiler::new(CompilerOptions::default())
        .compile(
            r#"
`include "disciplines.vams"

module bad_keyword_operator_arity(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ transition(V(p, n), 0.0, 1.0n, 1.0n, 99.0);
endmodule
"#,
        )
        .expect_err("extra keyword analog operator arguments must be rejected");
    let text = err.to_string();
    assert!(
        text.contains("transition"),
        "diagnostic should name the malformed analog operator, got: {text}"
    );
}

#[test]
fn unsupported_last_crossing_is_a_compile_time_diagnostic() {
    let err = VerilogACompiler::new(CompilerOptions::default())
        .compile(
            r#"
`include "disciplines.vams"

module unsupported_last_crossing(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ last_crossing(V(p, n), 1);
endmodule
"#,
        )
        .expect_err("last_crossing must not compile to a fake constant");
    let text = err.to_string();
    assert!(
        text.contains("last_crossing"),
        "diagnostic should name the unsupported operator, got: {text}"
    );
}

#[test]
fn cross_direction_must_be_constant() {
    let err = VerilogACompiler::new(CompilerOptions::default())
        .compile(
            r#"
`include "disciplines.vams"

module bad_cross_direction(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ (cross(V(p, n), V(p, n)) ? 1.0 : 0.0) * V(p, n);
endmodule
"#,
        )
        .expect_err("dynamic cross direction must be rejected");
    let text = err.to_string();
    assert!(
        text.contains("cross"),
        "diagnostic should name the malformed analog operator, got: {text}"
    );
}

#[test]
fn event_cross_direction_must_be_constant() {
    let err = VerilogACompiler::new(CompilerOptions::default())
        .compile(
            r#"
`include "disciplines.vams"

module bad_event_cross_direction(p, n);
    inout p, n;
    electrical p, n;
    analog @(cross(V(p, n), V(p, n))) I(p, n) <+ V(p, n);
endmodule
"#,
        )
        .expect_err("dynamic event cross direction must be rejected");
    let text = err.to_string();
    assert!(
        text.contains("cross"),
        "diagnostic should name the malformed event expression, got: {text}"
    );
}

#[test]
fn event_cross_time_tolerance_is_a_compile_time_diagnostic() {
    let err = VerilogACompiler::new(CompilerOptions::default())
        .compile(
            r#"
`include "disciplines.vams"

module bad_event_cross_tolerance(p, n);
    inout p, n;
    electrical p, n;
    analog @(cross(V(p, n), 1, 1.0n)) I(p, n) <+ V(p, n);
endmodule
"#,
        )
        .expect_err("unsupported event cross time tolerance must be rejected");
    let text = err.to_string();
    assert!(
        text.contains("cross") || text.contains("time"),
        "diagnostic should name the ignored cross time tolerance, got: {text}"
    );
}

#[test]
fn event_timer_extra_args_are_compile_time_diagnostics() {
    let err = VerilogACompiler::new(CompilerOptions::default())
        .compile(
            r#"
`include "disciplines.vams"

module bad_event_timer_arity(p, n);
    inout p, n;
    electrical p, n;
    analog @(timer(0.0, 1.0n, 2.0n)) I(p, n) <+ V(p, n);
endmodule
"#,
        )
        .expect_err("extra timer event arguments must be rejected");
    let text = err.to_string();
    assert!(
        text.contains("timer"),
        "diagnostic should name the malformed event expression, got: {text}"
    );
}

#[test]
fn event_above_extra_args_are_compile_time_diagnostics() {
    let err = VerilogACompiler::new(CompilerOptions::default())
        .compile(
            r#"
`include "disciplines.vams"

module bad_event_above_arity(p, n);
    inout p, n;
    electrical p, n;
    analog @(above(V(p, n), 1.0)) I(p, n) <+ V(p, n);
endmodule
"#,
        )
        .expect_err("extra above event arguments must be rejected");
    let text = err.to_string();
    assert!(
        text.contains("above"),
        "diagnostic should name the malformed event expression, got: {text}"
    );
}

#[test]
fn step_event_analysis_args_are_compile_time_diagnostics() {
    for event in ["initial_step", "final_step"] {
        let source = format!(
            r#"
`include "disciplines.vams"

module bad_step_event_args(p, n);
    inout p, n;
    electrical p, n;
    analog @({event}("tran")) I(p, n) <+ V(p, n);
endmodule
"#,
        );
        let err = VerilogACompiler::new(CompilerOptions::default())
            .compile(&source)
            .expect_err("unsupported step-event analysis lists must be rejected");
        let text = err.to_string();
        assert!(
            text.contains(event),
            "diagnostic should name the malformed event expression {event}, got: {text}"
        );
    }
}

#[test]
fn cross_direction_accepts_folded_constant() {
    let model = compile(
        r#"
`include "disciplines.vams"

module folded_cross_direction(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ cross(V(p, n), 1 - 2) * V(p, n);
endmodule
"#,
    );

    assert_eq!(model.name.as_str(), "folded_cross_direction");
}

#[test]
fn unsupported_event_time_tolerance_args_are_compile_time_diagnostics() {
    for (operator, expression) in [
        ("cross", "cross(V(p, n), 1, 1.0n)"),
        ("above", "above(V(p, n), 0.0, 1.0n)"),
    ] {
        let source = format!(
            r#"
`include "disciplines.vams"

module ignored_time_tolerance(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ ({expression} ? 1.0 : 0.0) * V(p, n);
endmodule
"#,
        );
        let err = VerilogACompiler::new(CompilerOptions::default())
            .compile(&source)
            .expect_err("unsupported event time tolerance arguments must be rejected");
        let text = err.to_string();
        assert!(
            text.contains(operator) || text.contains("time"),
            "diagnostic should name the ignored {operator} time tolerance, got: {text}"
        );
    }
}

#[test]
fn unknown_analysis_name_is_a_compile_time_diagnostic() {
    let err = VerilogACompiler::new(CompilerOptions::default())
        .compile(
            r#"
`include "disciplines.vams"

module unknown_analysis_name(p, n);
    inout p, n;
    electrical p, n;
    analog begin
        if (analysis("smoke"))
            I(p, n) <+ V(p, n);
    end
endmodule
"#,
        )
        .expect_err("unknown analysis() names must be rejected");
    let text = err.to_string();
    assert!(
        text.contains("analysis") && text.contains("smoke"),
        "diagnostic should name the unknown analysis query, got: {text}"
    );
}

#[test]
fn noise_source_name_arguments_must_be_strings() {
    for (operator, expression) in [
        ("white_noise", "white_noise(1.0, V(p, n))"),
        ("flicker_noise", "flicker_noise(1.0, 1.0, V(p, n))"),
        ("noise_table", "noise_table({1.0, 2.0}, V(p, n))"),
    ] {
        let source = format!(
            r#"
`include "disciplines.vams"

module bad_noise_name(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ {expression};
endmodule
"#,
        );
        let err = VerilogACompiler::new(CompilerOptions::default())
            .compile(&source)
            .expect_err("noise source name arguments must be string literals");
        let text = err.to_string();
        assert!(
            text.contains(operator) || text.contains("string"),
            "diagnostic should name the malformed {operator} name argument, got: {text}"
        );
    }
}

#[test]
fn malformed_exponent_literal_is_a_compile_error() {
    for literal in ["1e", "1e+"] {
        let source = format!(
            r#"
`include "disciplines.vams"

module bad_number(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = {literal};
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#,
        );
        let err = VerilogACompiler::new(CompilerOptions::default())
            .compile(&source)
            .expect_err("malformed numeric literals must not silently become zero");
        let text = err.to_string();
        assert!(
            text.contains("number") || text.contains("literal") || text.contains(literal),
            "diagnostic should identify the malformed numeric literal {literal}, got: {text}"
        );
    }
}

#[test]
fn overflowed_numeric_literal_is_a_compile_error() {
    let err = VerilogACompiler::new(CompilerOptions::default())
        .compile(
            r#"
`include "disciplines.vams"

module overflowing_number(p, n);
    inout p, n;
    electrical p, n;
    parameter real gain = 1e9999;
    analog I(p, n) <+ gain * V(p, n);
endmodule
"#,
        )
        .expect_err("overflowed numeric literals must not silently become infinity");
    let text = err.to_string();
    assert!(
        text.contains("number") || text.contains("literal") || text.contains("1e9999"),
        "diagnostic should identify the overflowed numeric literal, got: {text}"
    );
}

#[test]
fn leading_dot_real_literal_compiles() {
    let model = compile(
        r#"
`include "disciplines.vams"

module leading_dot_number(p, n);
    inout p, n;
    electrical p, n;
    parameter real gain = .5;
    analog I(p, n) <+ gain * V(p, n);
endmodule
"#,
    );

    assert_eq!(model.parameters[0].default, 0.5);
}

#[test]
fn duplicate_port_declaration_is_a_compile_error() {
    let err = VerilogACompiler::new(CompilerOptions::default())
        .compile(
            r#"
`include "disciplines.vams"

module duplicate_port_declaration(p, n);
    inout p, n;
    input p;
    electrical p, n;
    analog I(p, n) <+ V(p, n);
endmodule
"#,
        )
        .expect_err("duplicate port declarations must be rejected");
    let text = err.to_string();
    assert!(
        text.contains("Duplicate") || text.contains("p"),
        "diagnostic should identify the duplicate port declaration, got: {text}"
    );
}

#[test]
fn unknown_access_function_contribution_is_a_compile_error() {
    let err = VerilogACompiler::new(CompilerOptions::default())
        .compile(
            r#"
`include "disciplines.vams"

module unknown_access_contribution(p, n);
    inout p, n;
    electrical p, n;
    analog NotAnAccess(p, n) <+ V(p, n);
endmodule
"#,
        )
        .expect_err("unknown access functions must not become potential contributions");
    let text = err.to_string();
    assert!(
        text.contains("NotAnAccess") || text.contains("access"),
        "diagnostic should identify the unknown access function, got: {text}"
    );
}

#[test]
fn access_functions_must_match_node_disciplines() {
    for (case, body) in [
        (
            "thermal flow contribution on electrical port",
            "analog Pwr(p) <+ V(p, n);",
        ),
        (
            "thermal potential read on electrical port",
            "analog I(p, n) <+ Temp(p);",
        ),
    ] {
        let source = format!(
            r#"
`include "disciplines.vams"

module incompatible_access(p, n);
    inout p, n;
    electrical p, n;
    {body}
endmodule
"#,
        );
        let err = match VerilogACompiler::new(CompilerOptions::default()).compile(&source) {
            Ok(_) => panic!("{case} must be rejected"),
            Err(err) => err,
        };
        let text = err.to_string();
        assert!(
            text.contains("discipline")
                || text.contains("access")
                || text.contains("Pwr")
                || text.contains("Temp"),
            "diagnostic should identify incompatible access for {case}, got: {text}"
        );
    }
}

#[test]
fn undefined_disciplines_are_compile_errors() {
    for (case, declaration) in [
        ("port discipline", "phantom p, n;"),
        (
            "internal node discipline",
            "electrical p, n;\n    phantom x;",
        ),
    ] {
        let source = format!(
            r#"
`include "disciplines.vams"

module bad_discipline(p, n);
    inout p, n;
    {declaration}
    analog I(p, n) <+ V(p, n);
endmodule
"#,
        );

        let err = match VerilogACompiler::new(CompilerOptions::default()).compile(&source) {
            Ok(_) => panic!("{case} must reject undefined discipline names"),
            Err(err) => err,
        };
        let text = err.to_string();
        assert!(
            text.contains("phantom") || text.contains("discipline"),
            "diagnostic should identify undefined {case}, got: {text}"
        );
    }
}

#[test]
fn user_defined_natures_and_disciplines_define_access_functions() {
    let model = compile(
        r#"
nature CustomTemp;
    units = "K";
    access = CTemp;
    abstol = 1.0e-6;
endnature

nature CustomPower;
    units = "W";
    access = CPwr;
    abstol = 1.0e-12;
endnature

discipline customthermal;
    potential CustomTemp;
    flow CustomPower;
enddiscipline

module custom_thermal_model(t);
    inout t;
    customthermal t;
    analog CPwr(t) <+ CTemp(t);
endmodule
"#,
    );

    assert_eq!(model.name.as_str(), "custom_thermal_model");
    assert_eq!(model.num_terminals, 1);
    assert_eq!(model.stamp_programs.len(), 1);
}

#[test]
fn user_defined_discipline_can_appear_in_non_ansi_port_declaration() {
    let model = compile(
        r#"
nature CustomTemp;
    units = "K";
    access = CTemp;
    abstol = 1.0e-6;
endnature

nature CustomPower;
    units = "W";
    access = CPwr;
    abstol = 1.0e-12;
endnature

discipline customthermal;
    potential CustomTemp;
    flow CustomPower;
enddiscipline

module custom_thermal_port_decl(t);
    inout customthermal t;
    analog CPwr(t) <+ CTemp(t);
endmodule
"#,
    );

    assert_eq!(model.name.as_str(), "custom_thermal_port_decl");
    assert_eq!(model.num_terminals, 1);
    assert_eq!(model.stamp_programs.len(), 1);
}
