mod support;

use rspice_veriloga::{
    CodeGenerator, CompilerOptions, Lexer, Parser, SemanticAnalyzer, SourceMap, VerilogACompiler,
};
use support::DeviceFixture;

fn assert_unsupported(source: &str, expected_context: &str) {
    let error = VerilogACompiler::new(CompilerOptions::default())
        .compile(source)
        .expect_err("unsupported syntax must not compile by being discarded");
    let message = error.to_string();
    assert!(
        message.contains(expected_context),
        "expected diagnostic containing {expected_context:?}, got {message:?}"
    );
}

#[test]
fn unsupported_module_instances_are_not_silently_discarded() {
    assert_unsupported(
        r#"
`include "disciplines.vams"
module unsupported_instance(p, n);
    inout p, n;
    electrical p, n;
    child u1(p, n);
    analog I(p, n) <+ V(p, n);
endmodule
"#,
        "Undefined module: 'child'",
    );
}

#[test]
fn declared_module_hierarchy_executes_child_behavior() {
    let source = r#"
`include "disciplines.vams"
module child(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ 2.0 * V(p, n);
endmodule
module parent(p, n);
    inout p, n;
    electrical p, n;
    child u1(p, n);
endmodule
"#;
    let model = VerilogACompiler::new(CompilerOptions::default())
        .compile_module(source, Some("parent"))
        .expect("declared child hierarchy must elaborate");
    assert_eq!(model.stamp_programs.len(), 1);
    assert_eq!(model.stamp_programs[0].value_program.instructions.len(), 3);
}

#[test]
fn direct_bytecode_generation_uses_the_same_hierarchy_elaborator() {
    let source = r#"
module child(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ V(p, n);
endmodule
module parent(p, n);
    inout p, n;
    electrical p, n;
    child u1(p, n);
endmodule
"#;
    let source_map = SourceMap::new();
    let source_id = source_map.add_source("<direct-codegen>", source);
    let tokens = Lexer::new(source, source_id).collect_tokens().unwrap();
    let parsed = Parser::new(&tokens).parse().unwrap();
    let analyzed = SemanticAnalyzer::new().analyze(&parsed).unwrap();

    let model = CodeGenerator::new()
        .generate_module(&analyzed, Some("parent"))
        .expect("public bytecode generation must elaborate hierarchy");
    assert_eq!(model.stamp_programs.len(), 1);
    assert_eq!(model.num_terminals, 2);
}

#[test]
fn unsupported_keyword_module_items_are_not_silently_discarded() {
    assert_unsupported(
        r#"
`include "disciplines.vams"
module unsupported_wire(p, n);
    inout p, n;
    electrical p, n;
    wire hidden;
    analog I(p, n) <+ V(p, n);
endmodule
"#,
        "Unsupported module item: wire",
    );
}

#[test]
fn unsupported_nature_properties_are_not_silently_discarded() {
    assert_unsupported(
        r#"
nature custom_potential;
    access = CP;
    unsupported_property = 1;
endnature
module invalid_nature;
endmodule
"#,
        "Unsupported nature property: unsupported_property",
    );
}

#[test]
fn analog_functions_accept_direct_body_statements() {
    let model = DeviceFixture::compile(
        r#"
`include "disciplines.vams"
module direct_function_body(p, n);
    inout p, n;
    electrical p, n;

    analog function real square;
        input x;
        square = x * x;
    endfunction

    analog I(p, n) <+ square(V(p, n));
endmodule
"#,
    );
    let mut device = model.device("A1", &[1, 0]);
    device.update_voltages(&[2.0]);
    assert_eq!(
        device.try_evaluate().expect("direct function evaluates"),
        vec![4.0]
    );
}
