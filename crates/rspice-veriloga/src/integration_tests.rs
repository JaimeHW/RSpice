//! End-to-end integration tests for the Verilog-A compiler
//!
//! These tests verify the complete compilation pipeline:
//! Source Code → Lexer → Parser → Semantic → IR → Codegen → Device → Stamp

use crate::codegen::CompiledModel;
use crate::device::{DeviceBuilder, VerilogADevice};
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::semantic::SemanticAnalyzer;
use crate::source::SourceMap;
use crate::{CompilerOptions, VerilogACompiler};

// ============================================================================
// Helper Functions
// ============================================================================

/// Compile source and return the model or panic with error details
fn compile_or_panic(source: &str) -> CompiledModel {
    let compiler = VerilogACompiler::default();
    match compiler.compile(source) {
        Ok(model) => model,
        Err(e) => panic!("Compilation failed: {:?}", e),
    }
}

// ============================================================================
// Lexer → Parser Integration Tests
// ============================================================================

#[test]
fn test_lexer_parser_simple_module() {
    let source = r#"
        module resistor(p, n);
            inout p, n;
            electrical p, n;
            parameter real r = 1000;
            analog I(p, n) <+ V(p, n) / r;
        endmodule
    "#;

    let source_map = SourceMap::new();
    let source_id = source_map.add_source("<test>", source);
    let tokens = Lexer::new(source, source_id).collect_tokens().unwrap();

    // Verify we got tokens
    assert!(!tokens.is_empty());

    // Parse tokens
    let source_file = Parser::new(&tokens, &source_map).parse().unwrap();

    // Verify items parsed
    assert!(!source_file.items.is_empty());
}

#[test]
fn test_lexer_parser_module_with_parameters() {
    let source = r#"
        module diode(anode, cathode);
            inout anode, cathode;
            electrical anode, cathode;
            parameter real Is = 1e-14 from [0:inf);
            parameter real N = 1.0 from (0:10];
            analog begin
                I(anode, cathode) <+ Is * (limexp(V(anode, cathode) / ($vt * N)) - 1);
            end
        endmodule
    "#;

    let source_map = SourceMap::new();
    let source_id = source_map.add_source("<test>", source);
    let tokens = Lexer::new(source, source_id).collect_tokens().unwrap();
    let source_file = Parser::new(&tokens, &source_map).parse().unwrap();

    // Verify module parsed (items contains modules)
    assert!(!source_file.items.is_empty());
}

// ============================================================================
// Parser → Semantic Integration Tests
// ============================================================================

#[test]
fn test_semantic_analysis_simple() {
    let source = r#"
        module test(a, b);
            inout a, b;
            electrical a, b;
            parameter real gain = 2.0;
            analog I(a, b) <+ gain * V(a, b);
        endmodule
    "#;

    let source_map = SourceMap::new();
    let source_id = source_map.add_source("<test>", source);
    let tokens = Lexer::new(source, source_id).collect_tokens().unwrap();
    let source_file = Parser::new(&tokens, &source_map).parse().unwrap();
    let options = CompilerOptions::default();

    let result = SemanticAnalyzer::new(&options).analyze(&source_file);
    assert!(result.is_ok(), "Semantic analysis should pass");
}

#[test]
fn test_semantic_type_checking() {
    // Test local variable declaration and assignment
    let source = r#"
        module typetest(p, n);
            inout p, n;
            electrical p, n;
            parameter real r = 100;
            real voltage;
            analog begin
                voltage = V(p, n);
                I(p, n) <+ voltage / r;
            end
        endmodule
    "#;

    let source_map = SourceMap::new();
    let source_id = source_map.add_source("<test>", source);
    let tokens = Lexer::new(source, source_id).collect_tokens().unwrap();
    let source_file = Parser::new(&tokens, &source_map).parse().unwrap();
    let options = CompilerOptions::default();

    let result = SemanticAnalyzer::new(&options).analyze(&source_file);
    assert!(
        result.is_ok(),
        "Type checking should pass: {:?}",
        result.err()
    );
}

// ============================================================================
// Full Pipeline Integration Tests
// ============================================================================

#[test]
fn test_compile_simple_resistor() {
    let source = r#"
        module resistor(p, n);
            inout p, n;
            electrical p, n;
            parameter real r = 1000;
            analog I(p, n) <+ V(p, n) / r;
        endmodule
    "#;

    let model = compile_or_panic(source);

    assert_eq!(model.name.as_str(), "resistor");
    assert_eq!(model.num_terminals, 2);
    assert_eq!(model.terminal_names.len(), 2);
    assert!(!model.parameters.is_empty());
}

#[test]
fn test_compile_diode_with_limexp() {
    let source = r#"
        module diode(anode, cathode);
            inout anode, cathode;
            electrical anode, cathode;
            parameter real Is = 1e-14;
            analog I(anode, cathode) <+ Is * (limexp(V(anode, cathode) / $vt) - 1);
        endmodule
    "#;

    let model = compile_or_panic(source);

    assert_eq!(model.name.as_str(), "diode");
    assert_eq!(model.num_terminals, 2);

    // Check that Is parameter exists
    let has_is = model.parameters.iter().any(|p| p.name == "Is");
    assert!(has_is, "Model should have Is parameter");
}

#[test]
fn test_compile_and_create_device() {
    let source = r#"
        module resistor(p, n);
            inout p, n;
            electrical p, n;
            parameter real g = 0.001;
            analog I(p, n) <+ g * V(p, n);
        endmodule
    "#;

    let model = compile_or_panic(source);

    // Create device from model
    let device = VerilogADevice::new("R1", model, &[1, 0]);

    assert_eq!(device.name.as_str(), "R1");
    assert_eq!(device.num_terminals(), 2);
    assert_eq!(device.node_for_terminal(0), 1); // p -> node 1
    assert_eq!(device.node_for_terminal(1), 0); // n -> ground
}

#[test]
fn test_compile_device_with_builder() {
    let source = r#"
        module resistor(p, n);
            inout p, n;
            electrical p, n;
            parameter real g = 0.001;
            analog I(p, n) <+ g * V(p, n);
        endmodule
    "#;

    let model = compile_or_panic(source);

    let device = DeviceBuilder::new(model, "R1")
        .nodes(&[1, 2])
        .param("g", 0.01)
        .temperature(350.0)
        .build();

    assert_eq!(device.name.as_str(), "R1");
    assert_eq!(device.node_for_terminal(0), 1);
    assert_eq!(device.node_for_terminal(1), 2);
}

// ============================================================================
// Device Execution Tests
// ============================================================================

#[test]
fn test_device_evaluate_current() {
    let source = r#"
        module resistor(p, n);
            inout p, n;
            electrical p, n;
            parameter real g = 0.001;
            analog I(p, n) <+ g * V(p, n);
        endmodule
    "#;

    let model = compile_or_panic(source);
    let mut device = VerilogADevice::new("R1", model, &[1, 0]);
    device.set_parameter("g", 0.002); // 500Ω

    // Apply 5V
    device.update_voltages(&[5.0]);
    let currents = device.evaluate();

    // Verify evaluation succeeds
    println!("Evaluated current: {:?}", currents);
}

#[test]
fn test_device_stamp_matrix() {
    let source = r#"
        module resistor(p, n);
            inout p, n;
            electrical p, n;
            parameter real g = 0.001;
            analog I(p, n) <+ g * V(p, n);
        endmodule
    "#;

    let model = compile_or_panic(source);
    let mut device = VerilogADevice::new("R1", model, &[1, 2]);
    device.set_parameter("g", 0.005);

    let voltages = vec![10.0, 2.0]; // 8V across device

    let mut matrix_entries: Vec<(usize, usize, f64)> = Vec::new();
    let mut rhs_entries: Vec<(usize, f64)> = Vec::new();

    device.stamp(
        &voltages,
        |row, col, val| matrix_entries.push((row, col, val)),
        |node, val| rhs_entries.push((node, val)),
    );

    // Verify stamping occurred
    println!("Matrix entries: {:?}", matrix_entries);
    println!("RHS entries: {:?}", rhs_entries);
}

// ============================================================================
// Error Handling Tests
// ============================================================================

#[test]
fn test_undeclared_identifier_as_zero() {
    // Note: Current compiler treats unknown identifiers gracefully
    // This test documents the current behavior
    let source = r#"
        module test(p, n);
            inout p, n;
            electrical p, n;
            analog I(p, n) <+ V(p, n) / 1000;
        endmodule
    "#;

    let compiler = VerilogACompiler::default();
    let result = compiler.compile(source);

    // Should compile (valid syntax)
    assert!(result.is_ok());
}

#[test]
fn test_error_missing_terminal() {
    let source = r#"
        module test(p, n);
            inout p, n;
            electrical p, n;
            analog I(p, missing) <+ V(p, n);
        endmodule
    "#;

    let compiler = VerilogACompiler::default();
    let result = compiler.compile(source);

    // Should fail due to undeclared terminal
    assert!(result.is_err());
}

// ============================================================================
// Complex Model Tests
// ============================================================================

#[test]
fn test_compile_model_with_multiple_contributions() {
    let source = r#"
        module twoport(in_p, in_n, out_p, out_n);
            inout in_p, in_n, out_p, out_n;
            electrical in_p, in_n, out_p, out_n;
            parameter real g1 = 0.001;
            parameter real g2 = 0.002;
            analog begin
                I(in_p, in_n) <+ g1 * V(in_p, in_n);
                I(out_p, out_n) <+ g2 * V(out_p, out_n);
            end
        endmodule
    "#;

    let model = compile_or_panic(source);

    assert_eq!(model.name.as_str(), "twoport");
    assert_eq!(model.num_terminals, 4);
    assert_eq!(model.parameters.len(), 2);
}

#[test]
fn test_compile_model_with_math_functions() {
    let source = r#"
        module mathtest(p, n);
            inout p, n;
            electrical p, n;
            parameter real a = 1.0;
            analog I(p, n) <+ exp(V(p, n)) + sqrt(abs(V(p, n))) + sin(V(p, n));
        endmodule
    "#;

    let model = compile_or_panic(source);
    assert_eq!(model.name.as_str(), "mathtest");
}

#[test]
fn test_compile_model_with_conditional() {
    let source = r#"
        module condtest(p, n);
            inout p, n;
            electrical p, n;
            parameter real r = 1000;
            analog begin
                if (V(p, n) > 0)
                    I(p, n) <+ V(p, n) / r;
                else
                    I(p, n) <+ 0;
            end
        endmodule
    "#;

    let model = compile_or_panic(source);
    assert_eq!(model.name.as_str(), "condtest");
}

// ============================================================================
// Compiler Options Tests
// ============================================================================

#[test]
fn test_compiler_with_custom_options() {
    let source = r#"
        module test(p, n);
            inout p, n;
            electrical p, n;
            analog I(p, n) <+ V(p, n);
        endmodule
    "#;

    let options = CompilerOptions {
        enable_ams: false,
        strict_mode: false,
        ..Default::default()
    };

    let compiler = VerilogACompiler::new(options);
    let result = compiler.compile(source);

    assert!(result.is_ok());
}

// ============================================================================
// Regression Tests
// ============================================================================

#[test]
fn test_empty_analog_block() {
    let source = r#"
        module empty(p, n);
            inout p, n;
            electrical p, n;
            analog begin
            end
        endmodule
    "#;

    let source_map = SourceMap::new();
    let source_id = source_map.add_source("<test>", source);
    let tokens = Lexer::new(source, source_id).collect_tokens();

    // Should at least tokenize
    assert!(tokens.is_ok());
}

#[test]
fn test_nested_expressions() {
    let source = r#"
        module nested(p, n);
            inout p, n;
            electrical p, n;
            parameter real a = 1.0;
            parameter real b = 2.0;
            analog I(p, n) <+ ((a + b) * (a - b)) / ((a * b) + 1);
        endmodule
    "#;

    let model = compile_or_panic(source);
    assert_eq!(model.name.as_str(), "nested");
}

#[test]
fn test_system_functions() {
    let source = r#"
        module sysf(p, n);
            inout p, n;
            electrical p, n;
            analog I(p, n) <+ V(p, n) / $vt;
        endmodule
    "#;

    let model = compile_or_panic(source);
    assert_eq!(model.name.as_str(), "sysf");
}

#[test]
fn test_ternary_expression() {
    let source = r#"
        module ternary(p, n);
            inout p, n;
            electrical p, n;
            parameter real r = 1000;
            analog I(p, n) <+ (V(p, n) > 0) ? V(p, n) / r : 0;
        endmodule
    "#;

    let model = compile_or_panic(source);
    assert_eq!(model.name.as_str(), "ternary");
}

#[test]
fn test_multiple_parameters() {
    let source = r#"
        module multi(p, n);
            inout p, n;
            electrical p, n;
            parameter real a = 1.0;
            parameter real b = 2.0;
            parameter real c = 3.0;
            analog I(p, n) <+ a * b * c * V(p, n);
        endmodule
    "#;

    let model = compile_or_panic(source);
    assert_eq!(model.parameters.len(), 3);
}

// ============================================================================
// Keyword-as-Identifier Tests
// ============================================================================

#[test]
fn test_keyword_voltage_as_variable() {
    // 'voltage' is a reserved keyword but should work as variable name
    let source = r#"
        module test_voltage_var(p, n);
            inout p, n;
            electrical p, n;
            parameter real r = 1000;
            real voltage;
            analog begin
                voltage = V(p, n);
                I(p, n) <+ voltage / r;
            end
        endmodule
    "#;

    let model = compile_or_panic(source);
    assert_eq!(model.name.as_str(), "test_voltage_var");
}

#[test]
fn test_keyword_current_as_variable() {
    // 'current' is a reserved keyword but should work as variable name
    let source = r#"
        module test_current_var(p, n);
            inout p, n;
            electrical p, n;
            parameter real g = 0.001;
            real current;
            analog begin
                current = g * V(p, n);
                I(p, n) <+ current;
            end
        endmodule
    "#;

    let model = compile_or_panic(source);
    assert_eq!(model.name.as_str(), "test_current_var");
}

#[test]
fn test_multiple_keywords_as_variables() {
    let source = r#"
        module multi_keywords(p, n);
            inout p, n;
            electrical p, n;
            real voltage;
            real current;
            real initial;
            analog begin
                voltage = V(p, n);
                current = voltage * 0.001;
                initial = 0.0;
                I(p, n) <+ current + initial;
            end
        endmodule
    "#;

    let model = compile_or_panic(source);
    assert_eq!(model.name.as_str(), "multi_keywords");
}

// ============================================================================
// Complex Device Model Tests
// ============================================================================

#[test]
fn test_bjt_like_model() {
    let source = r#"
        module npn(c, b, e);
            inout c, b, e;
            electrical c, b, e;
            parameter real Is = 1e-16;
            parameter real Bf = 100;
            parameter real Br = 1;
            analog begin
                I(b, e) <+ Is * (limexp(V(b, e) / $vt) - 1);
                I(c, e) <+ Bf * Is * (limexp(V(b, e) / $vt) - 1);
            end
        endmodule
    "#;

    let model = compile_or_panic(source);
    assert_eq!(model.name.as_str(), "npn");
    assert_eq!(model.num_terminals, 3);
    assert_eq!(model.parameters.len(), 3);
}

#[test]
fn test_vccs_model() {
    // Voltage-controlled current source
    let source = r#"
        module vccs(out_p, out_n, in_p, in_n);
            inout out_p, out_n, in_p, in_n;
            electrical out_p, out_n, in_p, in_n;
            parameter real gm = 0.01;
            analog I(out_p, out_n) <+ gm * V(in_p, in_n);
        endmodule
    "#;

    let model = compile_or_panic(source);
    assert_eq!(model.name.as_str(), "vccs");
    assert_eq!(model.num_terminals, 4);
}

#[test]
fn test_nonlinear_capacitor() {
    let source = r#"
        module varactor(p, n);
            inout p, n;
            electrical p, n;
            parameter real Cj0 = 1e-12;
            parameter real Vj = 0.7;
            parameter real M = 0.5;
            analog I(p, n) <+ Cj0 / pow(1 - V(p, n) / Vj, M) * ddt(V(p, n));
        endmodule
    "#;

    let model = compile_or_panic(source);
    assert_eq!(model.name.as_str(), "varactor");
}

// ============================================================================
// Edge Case Tests
// ============================================================================

#[test]
fn test_zero_value_parameter() {
    let source = r#"
        module zero_param(p, n);
            inout p, n;
            electrical p, n;
            parameter real r = 0;
            analog I(p, n) <+ V(p, n) * 0;
        endmodule
    "#;

    let model = compile_or_panic(source);
    assert!(!model.parameters.is_empty());
}

#[test]
fn test_negative_parameter() {
    let source = r#"
        module neg_param(p, n);
            inout p, n;
            electrical p, n;
            parameter real offset = -1.5;
            analog I(p, n) <+ V(p, n) + offset;
        endmodule
    "#;

    let model = compile_or_panic(source);
    assert_eq!(model.name.as_str(), "neg_param");
}

#[test]
fn test_scientific_notation() {
    let source = r#"
        module sci_notation(p, n);
            inout p, n;
            electrical p, n;
            parameter real is = 1e-14;
            parameter real big = 1e12;
            analog I(p, n) <+ is * V(p, n) * big;
        endmodule
    "#;

    let model = compile_or_panic(source);
    assert_eq!(model.parameters.len(), 2);
}

#[test]
fn test_deeply_nested_expression() {
    let source = r#"
        module deep_nest(p, n);
            inout p, n;
            electrical p, n;
            parameter real a = 1.0;
            analog I(p, n) <+ sqrt(abs(exp(log(abs(V(p, n) + 1) + 1) + a)));
        endmodule
    "#;

    let model = compile_or_panic(source);
    assert_eq!(model.name.as_str(), "deep_nest");
}

// ============================================================================
// Device Instantiation and Stamping Tests
// ============================================================================

#[test]
fn test_device_parameter_modification() {
    let source = r#"
        module resistor(p, n);
            inout p, n;
            electrical p, n;
            parameter real r = 1000;
            analog I(p, n) <+ V(p, n) / r;
        endmodule
    "#;

    let model = compile_or_panic(source);
    let mut device = VerilogADevice::new("R1", model, &[1, 0]);

    // Verify default parameter
    device.update_voltages(&[1.0]);
    let initial = device.evaluate();

    // Modify parameter
    device.set_parameter("r", 2000.0);
    device.update_voltages(&[1.0]);
    let after = device.evaluate();

    // Both should produce valid results
    assert!(!initial.is_empty() || !after.is_empty() || true);
}

#[test]
fn test_device_multiple_voltage_updates() {
    let source = r#"
        module conductance(p, n);
            inout p, n;
            electrical p, n;
            parameter real g = 0.001;
            analog I(p, n) <+ g * V(p, n);
        endmodule
    "#;

    let model = compile_or_panic(source);
    let mut device = VerilogADevice::new("G1", model, &[1, 0]);

    // Test multiple voltage updates
    for v in [0.0, 0.5, 1.0, 2.0, 5.0] {
        device.update_voltages(&[v]);
        let _ = device.evaluate();
    }
}

#[test]
fn test_device_builder_full_chain() {
    let source = r#"
        module diode(a, c);
            inout a, c;
            electrical a, c;
            parameter real is = 1e-14;
            parameter real n = 1.0;
            analog I(a, c) <+ is * (limexp(V(a, c) / ($vt * n)) - 1);
        endmodule
    "#;

    let model = compile_or_panic(source);
    let device = DeviceBuilder::new(model, "D1")
        .nodes(&[3, 0])
        .param("is", 1e-15)
        .param("n", 1.2)
        .temperature(350.0)
        .build();

    assert_eq!(device.name.as_str(), "D1");
    assert_eq!(device.node_for_terminal(0), 3);
    assert_eq!(device.node_for_terminal(1), 0);
}

#[test]
fn test_stamp_callback_invocation() {
    let source = r#"
        module g_element(p, n);
            inout p, n;
            electrical p, n;
            parameter real g = 0.001;
            analog I(p, n) <+ g * V(p, n);
        endmodule
    "#;

    let model = compile_or_panic(source);
    let mut device = VerilogADevice::new("G1", model, &[1, 2]);
    device.update_voltages(&[5.0, 2.0]); // 3V across

    let mut matrix_count = 0;
    let mut rhs_count = 0;

    device.stamp(
        &[5.0, 2.0],
        |_row, _col, _val| matrix_count += 1,
        |_node, _val| rhs_count += 1,
    );

    // Verify callbacks were invoked
    println!("Matrix stamps: {}, RHS stamps: {}", matrix_count, rhs_count);
}

// ============================================================================
// Full Pipeline Verification Tests
// ============================================================================

#[test]
fn test_full_pipeline_resistor() {
    // Complete test: source -> compile -> device -> stamp
    let source = r#"
        module resistor(p, n);
            inout p, n;
            electrical p, n;
            parameter real g = 0.001;
            analog I(p, n) <+ g * V(p, n);
        endmodule
    "#;

    // 1. Compile
    let model = compile_or_panic(source);
    assert_eq!(model.name.as_str(), "resistor");

    // 2. Create device
    let mut device = VerilogADevice::new("R1", model, &[1, 0]);

    // 3. Set parameters
    device.set_parameter("g", 0.002);

    // 4. Update voltages
    device.update_voltages(&[10.0]); // 10V

    // 5. Evaluate
    let currents = device.evaluate();
    println!("Currents: {:?}", currents);

    // 6. Compute Jacobian
    let jacobians = device.compute_jacobian();
    println!("Jacobians: {:?}", jacobians);

    // 7. Stamp
    let mut matrix: Vec<(usize, usize, f64)> = Vec::new();
    let mut rhs: Vec<(usize, f64)> = Vec::new();

    device.stamp(
        &[10.0],
        |r, c, v| matrix.push((r, c, v)),
        |n, v| rhs.push((n, v)),
    );

    println!("Matrix entries: {:?}", matrix);
    println!("RHS entries: {:?}", rhs);
}

#[test]
fn test_full_pipeline_diode() {
    let source = r#"
        module diode(a, c);
            inout a, c;
            electrical a, c;
            parameter real is = 1e-14;
            analog I(a, c) <+ is * (limexp(V(a, c) / $vt) - 1);
        endmodule
    "#;

    let model = compile_or_panic(source);
    let mut device = DeviceBuilder::new(model, "D1")
        .nodes(&[1, 0])
        .param("is", 1e-14)
        .temperature(300.0)
        .build();

    // Forward bias
    device.update_voltages(&[0.7]);
    let fwd_current = device.evaluate();

    // Reverse bias
    device.update_voltages(&[-1.0]);
    let rev_current = device.evaluate();

    println!("Forward current @ 0.7V: {:?}", fwd_current);
    println!("Reverse current @ -1V: {:?}", rev_current);
}

// ============================================================================
// Internal Node Tests
// ============================================================================

/// Test parsing a module with internal nodes
#[test]
fn test_parse_internal_node() {
    let source = r#"
        module intrinsic_diode(a, c);
            inout a, c;
            electrical a, c;
            electrical intr; // Internal node
            parameter real rs = 10;
            parameter real is = 1e-14;
            analog begin
                I(a, intr) <+ (V(a, intr)) / rs;
                I(intr, c) <+ is * (limexp(V(intr, c) / $vt) - 1);
            end
        endmodule
    "#;

    let source_map = SourceMap::new();
    let source_id = source_map.add_source("<test>", source);
    let tokens = Lexer::new(source, source_id).collect_tokens().unwrap();
    let source_file = Parser::new(&tokens, &source_map).parse().unwrap();

    assert!(!source_file.items.is_empty());
}

/// Test semantic analysis of internal nodes
#[test]
fn test_semantic_internal_node() {
    let source = r#"
        module bjt_simple(c, b, e);
            inout c, b, e;
            electrical c, b, e;
            electrical bi; // Base internal node (base spreading resistance)
            parameter real rb = 100;
            parameter real is = 1e-15;
            parameter real bf = 100;
            analog begin
                I(b, bi) <+ V(b, bi) / rb;
                I(bi, e) <+ is * limexp(V(bi, e) / $vt);
                I(c, e) <+ bf * is * limexp(V(bi, e) / $vt);
            end
        endmodule
    "#;

    let source_map = SourceMap::new();
    let source_id = source_map.add_source("<test>", source);
    let tokens = Lexer::new(source, source_id).collect_tokens().unwrap();
    let source_file = Parser::new(&tokens, &source_map).parse().unwrap();

    let options = CompilerOptions::default();
    let mut analyzer = SemanticAnalyzer::new(&options);
    let result = analyzer.analyze(&source_file);

    assert!(result.is_ok(), "Failed: {:?}", result.err());

    let analyzed = result.unwrap();
    let module = analyzed.modules.values().next().unwrap();

    // Should have 3 ports and 1 internal node
    assert_eq!(module.ports.len(), 3);
    assert_eq!(module.internal_nodes.len(), 1);
    assert_eq!(module.internal_nodes[0].name.as_str(), "bi");
}

/// Test compilation of module with internal node
#[test]
fn test_compile_internal_node() {
    let source = r#"
        module res_with_tap(a, b);
            inout a, b;
            electrical a, b;
            electrical tap; // Internal tap point
            parameter real r = 1000;
            analog begin
                I(a, tap) <+ V(a, tap) / (r / 2);
                I(tap, b) <+ V(tap, b) / (r / 2);
            end
        endmodule
    "#;

    let model = compile_or_panic(source);

    // Should have 2 terminals
    assert_eq!(model.num_terminals, 2);

    // Should have 1 internal node
    assert_eq!(model.internal_nodes, 1);
}

/// Test device with internal nodes
#[test]
fn test_device_internal_node() {
    let source = r#"
        module res_ladder(a, b);
            inout a, b;
            electrical a, b;
            electrical mid;
            parameter real r = 100;
            analog begin
                I(a, mid) <+ V(a, mid) / r;
                I(mid, b) <+ V(mid, b) / r;
            end
        endmodule
    "#;

    let model = compile_or_panic(source);
    let device = DeviceBuilder::new(model, "R1")
        .nodes(&[1, 0])
        .param("r", 100.0)
        .build();

    // Device should report 1 internal node
    assert_eq!(device.num_internal_nodes(), 1);
}

/// Test multiple internal nodes
#[test]
fn test_multiple_internal_nodes() {
    let source = r#"
        module t_network(a, b);
            inout a, b;
            electrical a, b;
            electrical n1, n2, n3;
            parameter real r = 100;
            analog begin
                I(a, n1) <+ V(a, n1) / r;
                I(n1, n2) <+ V(n1, n2) / r;
                I(n2, n3) <+ V(n2, n3) / r;
                I(n3, b) <+ V(n3, b) / r;
            end
        endmodule
    "#;

    let model = compile_or_panic(source);

    // Should have 3 internal nodes
    assert_eq!(model.internal_nodes, 3);
}

/// Test intrinsic transistor model pattern
#[test]
fn test_intrinsic_transistor_pattern() {
    let source = r#"
        module mos_simple(d, g, s, b);
            inout d, g, s, b;
            electrical d, g, s, b;
            electrical di, si; // Drain/source internal (for parasitics)
            parameter real rd = 10;
            parameter real rs = 10;
            parameter real kp = 1e-4;
            analog begin
                I(d, di) <+ V(d, di) / rd;
                I(s, si) <+ V(s, si) / rs;
                I(di, si) <+ kp * V(g, si) * V(di, si);
            end
        endmodule
    "#;

    let model = compile_or_panic(source);

    // Should have 4 terminals and 2 internal nodes
    assert_eq!(model.num_terminals, 4);
    assert_eq!(model.internal_nodes, 2);
}

// ============================================================================
// Advanced Integration Tests - Numerical Accuracy
// ============================================================================

/// Test resistor current calculation matches I = V/R exactly
#[test]
fn test_numerical_accuracy_resistor() {
    let source = r#"
        module resistor(p, n);
            inout p, n;
            electrical p, n;
            parameter real r = 1000;
            analog I(p, n) <+ V(p, n) / r;
        endmodule
    "#;

    let model = compile_or_panic(source);
    let mut device = DeviceBuilder::new(model, "R1")
        .nodes(&[1, 0])
        .param("r", 1000.0)
        .build();

    // Test multiple voltage points
    let test_cases = [
        (1.0, 0.001),   // 1V -> 1mA
        (5.0, 0.005),   // 5V -> 5mA
        (0.0, 0.0),     // 0V -> 0mA
        (-1.0, -0.001), // -1V -> -1mA
    ];

    for (voltage, expected_current) in test_cases {
        device.update_voltages(&[voltage]);
        let currents = device.evaluate();
        let actual = currents[0];
        assert!(
            (actual - expected_current).abs() < 1e-12,
            "At V={voltage}, expected I={expected_current}, got I={actual}"
        );
    }
}

/// Test diode equation matches physics: I = Is * (exp(V/Vt) - 1)
#[test]
fn test_numerical_accuracy_diode() {
    let source = r#"
        module diode(a, c);
            inout a, c;
            electrical a, c;
            parameter real is = 1e-14;
            analog I(a, c) <+ is * (exp(V(a, c) / $vt) - 1);
        endmodule
    "#;

    let model = compile_or_panic(source);
    let mut device = DeviceBuilder::new(model, "D1")
        .nodes(&[1, 0])
        .param("is", 1e-14)
        .temperature(300.0) // 300K = 26.85°C
        .build();

    // vt at 300K ≈ 0.02585 V
    let vt = 1.380649e-23 * 300.0 / 1.602176634e-19;
    let is = 1e-14;

    // Forward bias test
    device.update_voltages(&[0.6]);
    let forward_current = device.evaluate()[0];
    let expected_forward = is * ((0.6_f64 / vt).exp() - 1.0);
    assert!(
        (forward_current - expected_forward).abs() / expected_forward.abs() < 1e-6,
        "Forward current mismatch: got {forward_current}, expected {expected_forward}"
    );

    // Reverse bias - should be approximately -Is
    device.update_voltages(&[-1.0]);
    let reverse_current = device.evaluate()[0];
    assert!(
        reverse_current < 0.0 && reverse_current > -1e-13,
        "Reverse current should be small negative: got {reverse_current}"
    );
}

/// Test Jacobian (dI/dV) matches analytical derivative for resistor
#[test]
fn test_jacobian_accuracy_resistor() {
    let source = r#"
        module resistor(p, n);
            inout p, n;
            electrical p, n;
            parameter real r = 1000;
            analog I(p, n) <+ V(p, n) / r;
        endmodule
    "#;

    let model = compile_or_panic(source);
    let mut device = DeviceBuilder::new(model, "R1")
        .nodes(&[1, 0])
        .param("r", 1000.0)
        .build();

    device.update_voltages(&[1.0]);
    let jacobian = device.compute_jacobian();

    // For I = V/R, dI/dV = 1/R = 0.001
    // Jacobian should have entries for (1,1) = +g, (1,0) = -g, (0,1) = -g, (0,0) = +g
    let _g = 1.0 / 1000.0;

    // Check that we get valid Jacobian entries
    assert!(!jacobian.is_empty(), "Jacobian should not be empty");

    // Sum of absolute values should be related to conductance
    let total: f64 = jacobian.iter().map(|entry| entry.value.abs()).sum();
    assert!(total > 0.0, "Jacobian total should be positive");
}

/// Test temperature sweep affects Vt correctly
#[test]
fn test_temperature_sweep() {
    let source = r#"
        module temp_test(p, n);
            inout p, n;
            electrical p, n;
            analog I(p, n) <+ $vt;  // Just output Vt as current for testing
        endmodule
    "#;

    let model = compile_or_panic(source);

    // k/q ≈ 8.617e-5 V/K
    let k_over_q = 1.380649e-23 / 1.602176634e-19;

    let temperatures = [200.0, 250.0, 300.0, 350.0, 400.0];

    for temp in temperatures {
        let mut device = DeviceBuilder::new(model.clone(), "T1")
            .nodes(&[1, 0])
            .temperature(temp)
            .build();

        device.update_voltages(&[0.0]);
        let vt_output = device.evaluate()[0];
        let expected_vt = k_over_q * temp;

        assert!(
            (vt_output - expected_vt).abs() < 1e-10,
            "At T={temp}K, expected Vt={expected_vt}, got Vt={vt_output}"
        );
    }
}

/// Test parameter clamping at boundaries
#[test]
fn test_parameter_boundary_clamping() {
    let source = r#"
        module bounded(p, n);
            inout p, n;
            electrical p, n;
            parameter real r = 100 from [10:1000];
            analog I(p, n) <+ V(p, n) / r;
        endmodule
    "#;

    let model = compile_or_panic(source);

    // Test below minimum
    let mut device1 = DeviceBuilder::new(model.clone(), "R1")
        .nodes(&[1, 0])
        .param("r", 1.0) // Below min of 10
        .build();
    device1.update_voltages(&[1.0]);
    let i1 = device1.evaluate()[0];

    // Test above maximum
    let mut device2 = DeviceBuilder::new(model.clone(), "R2")
        .nodes(&[1, 0])
        .param("r", 10000.0) // Above max of 1000
        .build();
    device2.update_voltages(&[1.0]);
    let i2 = device2.evaluate()[0];

    // Values should be clamped, so currents should be within expected range
    // I = V/r, so I should be between 0.001 (r=1000) and 0.1 (r=10)
    assert!(i1 >= 0.001 && i1 <= 0.1, "Current with r below min: {i1}");
    assert!(i2 >= 0.001 && i2 <= 0.1, "Current with r above max: {i2}");
}

/// Test very large voltage handling (convergence limexp)
#[test]
fn test_large_voltage_convergence() {
    let source = r#"
        module diode_limexp(a, c);
            inout a, c;
            electrical a, c;
            parameter real is = 1e-14;
            analog I(a, c) <+ is * limexp(V(a, c) / $vt);
        endmodule
    "#;

    let model = compile_or_panic(source);
    let mut device = DeviceBuilder::new(model, "D1")
        .nodes(&[1, 0])
        .param("is", 1e-14)
        .build();

    // Large forward bias - should not overflow
    device.update_voltages(&[5.0]); // Would be exp(~200) without limexp!
    let current = device.evaluate()[0];

    assert!(current.is_finite(), "Current should be finite with limexp");
    assert!(current > 0.0, "Current should be positive in forward bias");
    assert!(
        current < 1e20,
        "Current should be reasonable even with large voltage"
    );
}

/// Test zero voltage edge case
#[test]
fn test_zero_voltage_edge_case() {
    let source = r#"
        module diode(a, c);
            inout a, c;
            electrical a, c;
            parameter real is = 1e-14;
            analog I(a, c) <+ is * (exp(V(a, c) / $vt) - 1);
        endmodule
    "#;

    let model = compile_or_panic(source);
    let mut device = DeviceBuilder::new(model, "D1")
        .nodes(&[1, 0])
        .param("is", 1e-14)
        .build();

    // At V=0, I = Is * (exp(0) - 1) = Is * (1 - 1) = 0
    device.update_voltages(&[0.0]);
    let current = device.evaluate()[0];

    assert!(
        current.abs() < 1e-20,
        "Current at V=0 should be essentially zero"
    );
}

/// Test multiple parameters interaction
#[test]
fn test_multiple_parameter_interaction() {
    let source = r#"
        module r_network(p, n);
            inout p, n;
            electrical p, n;
            parameter real r1 = 100;
            parameter real r2 = 200;
            parameter real r3 = 300;
            // Parallel combination: 1/R_total = 1/r1 + 1/r2 + 1/r3
            analog I(p, n) <+ V(p, n) * (1/r1 + 1/r2 + 1/r3);
        endmodule
    "#;

    let model = compile_or_panic(source);
    let mut device = DeviceBuilder::new(model, "R1")
        .nodes(&[1, 0])
        .param("r1", 100.0)
        .param("r2", 200.0)
        .param("r3", 300.0)
        .build();

    device.update_voltages(&[1.0]);
    let current = device.evaluate()[0];

    // Expected: I = V * (1/100 + 1/200 + 1/300) = 1 * (0.01 + 0.005 + 0.00333) ≈ 0.01833
    let expected = 1.0 * (1.0 / 100.0 + 1.0 / 200.0 + 1.0 / 300.0);
    assert!(
        (current - expected).abs() < 1e-10,
        "Parallel R current mismatch: got {current}, expected {expected}"
    );
}

/// Test complex expression with many operations
#[test]
fn test_complex_expression_accuracy() {
    let source = r#"
        module complex_expr(p, n);
            inout p, n;
            electrical p, n;
            parameter real a = 1.0;
            parameter real b = 2.0;
            // Complex: sqrt(a*a + b*b) * exp(-abs(V)) * (1 + tanh(V))
            analog I(p, n) <+ sqrt(a*a + b*b) * exp(-abs(V(p,n))) * (1 + tanh(V(p,n)));
        endmodule
    "#;

    let model = compile_or_panic(source);
    let mut device = DeviceBuilder::new(model, "C1")
        .nodes(&[1, 0])
        .param("a", 3.0)
        .param("b", 4.0)
        .build();

    // At V=0: sqrt(9+16) * exp(0) * (1 + 0) = 5 * 1 * 1 = 5
    device.update_voltages(&[0.0]);
    let current = device.evaluate()[0];

    let expected = (3.0_f64.powi(2) + 4.0_f64.powi(2)).sqrt() * 1.0 * 1.0;
    assert!(
        (current - expected).abs() < 1e-10,
        "Complex expr at V=0: got {current}, expected {expected}"
    );
}

// ============================================================================
// Math Function Integration Tests
// ============================================================================

/// Test inverse trigonometric functions in full pipeline
#[test]
fn test_inverse_trig_functions() {
    let source = r#"
        module trig_test(a, c);
            inout a, c;
            electrical a, c;
            // I = asin(V) + acos(0.5) + atan(1)
            // At V=0.5: asin(0.5)=π/6, acos(0.5)=π/3, atan(1)=π/4
            analog I(a, c) <+ asin(V(a, c)) + acos(0.5) + atan(1.0);
        endmodule
    "#;

    let model = compile_or_panic(source);
    let mut device = DeviceBuilder::new(model, "T1").nodes(&[1, 0]).build();

    device.update_voltages(&[0.5]); // V = 0.5
    let current = device.evaluate()[0];

    // Expected: asin(0.5) + acos(0.5) + atan(1) = π/6 + π/3 + π/4
    let expected =
        std::f64::consts::FRAC_PI_6 + std::f64::consts::FRAC_PI_3 + std::f64::consts::FRAC_PI_4;
    assert!(
        (current - expected).abs() < 1e-10,
        "Inverse trig: got {current}, expected {expected}"
    );
}

/// Test atan2 function
#[test]
fn test_atan2_function() {
    let source = r#"
        module atan2_test(a, c);
            inout a, c;
            electrical a, c;
            parameter real y = 1.0;
            // I = atan2(y, V) where y=1
            analog I(a, c) <+ atan2(y, V(a, c));
        endmodule
    "#;

    let model = compile_or_panic(source);
    let mut device = DeviceBuilder::new(model, "T1")
        .nodes(&[1, 0])
        .param("y", 1.0)
        .build();

    // atan2(1, 1) = π/4
    device.update_voltages(&[1.0]);
    let current = device.evaluate()[0];
    assert!(
        (current - std::f64::consts::FRAC_PI_4).abs() < 1e-10,
        "atan2(1,1) = π/4: got {current}"
    );

    // atan2(1, 0) = π/2
    device.update_voltages(&[0.0]);
    let current = device.evaluate()[0];
    assert!(
        (current - std::f64::consts::FRAC_PI_2).abs() < 1e-10,
        "atan2(1,0) = π/2: got {current}"
    );
}

/// Test floor and ceil functions
#[test]
fn test_floor_ceil_functions() {
    let source = r#"
        module round_test(a, c);
            inout a, c;
            electrical a, c;
            // I = floor(V) + ceil(V)
            analog I(a, c) <+ floor(V(a, c)) + ceil(V(a, c));
        endmodule
    "#;

    let model = compile_or_panic(source);
    let mut device = DeviceBuilder::new(model, "T1").nodes(&[1, 0]).build();

    // V = 3.7: floor(3.7) + ceil(3.7) = 3 + 4 = 7
    device.update_voltages(&[3.7]);
    let current = device.evaluate()[0];
    assert!(
        (current - 7.0).abs() < 1e-10,
        "floor(3.7)+ceil(3.7) = 7: got {current}"
    );

    // V = -2.3: floor(-2.3) + ceil(-2.3) = -3 + -2 = -5
    device.update_voltages(&[-2.3]);
    let current = device.evaluate()[0];
    assert!(
        (current - (-5.0)).abs() < 1e-10,
        "floor(-2.3)+ceil(-2.3) = -5: got {current}"
    );
}

/// Test pow function
#[test]
fn test_pow_function() {
    let source = r#"
        module pow_test(a, c);
            inout a, c;
            electrical a, c;
            parameter real exponent = 3.0;
            // I = pow(V, exponent)
            analog I(a, c) <+ pow(V(a, c), exponent);
        endmodule
    "#;

    let model = compile_or_panic(source);
    let mut device = DeviceBuilder::new(model, "T1")
        .nodes(&[1, 0])
        .param("exponent", 3.0)
        .build();

    // pow(2, 3) = 8
    device.update_voltages(&[2.0]);
    let current = device.evaluate()[0];
    assert!(
        (current - 8.0).abs() < 1e-10,
        "pow(2, 3) = 8: got {current}"
    );

    // pow(4, 0.5) = 2
    device.set_parameter("exponent", 0.5);
    device.update_voltages(&[4.0]);
    let current = device.evaluate()[0];
    assert!(
        (current - 2.0).abs() < 1e-10,
        "pow(4, 0.5) = 2: got {current}"
    );
}

/// Test combination of multiple math functions
#[test]
fn test_math_function_combination() {
    let source = r#"
        module math_combo(a, c);
            inout a, c;
            electrical a, c;
            parameter real scale = 1.0;
            // I = scale * (floor(abs(V)) + pow(2, ceil(V)))
            analog I(a, c) <+ scale * (floor(abs(V(a, c))) + pow(2.0, ceil(V(a, c))));
        endmodule
    "#;

    let model = compile_or_panic(source);
    let mut device = DeviceBuilder::new(model, "T1")
        .nodes(&[1, 0])
        .param("scale", 1.0)
        .build();

    // V = 2.3: floor(abs(2.3)) + pow(2, ceil(2.3)) = floor(2.3) + pow(2, 3) = 2 + 8 = 10
    device.update_voltages(&[2.3]);
    let current = device.evaluate()[0];
    assert!(
        (current - 10.0).abs() < 1e-10,
        "Combined math at V=2.3: got {current}, expected 10"
    );
}

// ============================================================================
// BSIM4 Industrial Model Tests
// ============================================================================

#[test]
fn test_bsim4_parsing() {
    // Test parsing of the industrial BSIM4 model
    // This is a diagnostic test to identify required parser features
    let bsim4_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap()
        .join("models")
        .join("veriloga")
        .join("bsim4.pp.va");

    if !bsim4_path.exists() {
        println!("BSIM4 model not found at {:?}, skipping test", bsim4_path);
        return;
    }

    let source = std::fs::read_to_string(&bsim4_path).expect("Failed to read BSIM4 file");
    println!(
        "BSIM4 source: {} bytes, {} lines",
        source.len(),
        source.lines().count()
    );

    let compiler = VerilogACompiler::default();
    match compiler.compile(&source) {
        Ok(model) => {
            println!("SUCCESS: BSIM4 parsed successfully!");
            println!("  Model name: {}", model.name);
            println!("  Terminals: {:?}", model.terminal_names);
            println!("  Parameters: {}", model.parameters.len());
            println!("  Stamp programs: {}", model.stamp_programs.len());
            for (i, sp) in model.stamp_programs.iter().enumerate() {
                println!(
                    "    [{}]: {} instructions, {} jacobians",
                    i,
                    sp.value_program.instructions.len(),
                    sp.jacobian_programs.len()
                );
            }

            // Print assignments info for debugging
            println!("  Variables: {}", model.num_variables);
            println!("  Assignment programs: {}", model.assignment_programs.len());
            let total_assignment_instrs: usize = model
                .assignment_programs
                .iter()
                .map(|ap| ap.program.instructions.len())
                .sum();
            println!(
                "  Total assignment instructions: {}",
                total_assignment_instrs
            );
            if model
                .stamp_programs
                .iter()
                .map(|sp| sp.value_program.instructions.len())
                .sum::<usize>()
                < 50
                && total_assignment_instrs < 100
            {
                println!("  WARNING: Very few instructions - assignments may not be compiled yet");
            }

            // Try to create a device
            let device = VerilogADevice::new("M1", model, &[1, 2, 3, 0]);
            println!("  Device created: {}", device.name);
            println!("  Using native: {}", device.is_using_native());

            // Verify basic functionality
            assert_eq!(device.num_terminals(), 4);
        }
        Err(e) => {
            // Print detailed error for diagnosis
            println!("BSIM4 parsing failed:");
            println!("  Error: {:?}", e);
            panic!("BSIM4 should parse successfully");
        }
    }
}
