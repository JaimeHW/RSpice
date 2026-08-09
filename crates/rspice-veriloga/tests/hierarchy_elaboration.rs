//! End-to-end qualification of structural Verilog-A elaboration.

mod support;

use rspice_veriloga::{CompilerOptions, RuntimeQualificationOptions, VerilogACompiler};
use support::DeviceFixture;

fn compile_selected(source: &str, module: &str) -> DeviceFixture {
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    let model = compiler
        .compile_module(source, Some(module))
        .expect("selected hierarchy must compile to bytecode");
    #[cfg(feature = "native")]
    {
        let canonical_ir = compiler
            .compile_canonical_ir_module(source, Some(module))
            .expect("selected hierarchy must compile to canonical IR");
        DeviceFixture {
            model,
            canonical_ir,
        }
    }
    #[cfg(not(feature = "native"))]
    {
        DeviceFixture { model }
    }
}

#[test]
fn child_parameter_override_is_symbolic_and_not_public_abi() {
    let source = r#"
`include "disciplines.vams"
module gain_cell(p, n);
    inout p, n;
    electrical p, n;
    parameter real gain = 1.0 from (0:inf);
    real effective;
    analog begin
        effective = $param_given(gain) ? gain : 10.0;
        I(p, n) <+ effective * V(p, n);
    end
endmodule

module parent(p, n);
    inout p, n;
    electrical p, n;
    parameter real scale = 3.0 from (0:inf);
    gain_cell #(.gain(2.0 * scale)) stage (.p(p), .n(n));
endmodule
"#;
    let model = compile_selected(source, "parent");
    assert_eq!(
        model
            .parameters
            .iter()
            .filter(|parameter| parameter.is_public)
            .map(|parameter| parameter.name.as_str())
            .collect::<Vec<_>>(),
        vec!["scale"]
    );
    assert_eq!(model.parameters.len(), 2, "child keeps one private slot");

    let hidden_name = model
        .parameters
        .iter()
        .find(|parameter| !parameter.is_public)
        .expect("hidden child parameter")
        .name
        .clone();
    let mut device = model.device("X1", &[1, 0]);
    assert!(
        !device.set_parameter(hidden_name.as_str(), 99.0),
        "flattened child parameters must not be externally settable"
    );
    device.update_voltages(&[2.0]);
    assert_eq!(
        device.try_evaluate().expect("hierarchy evaluates"),
        vec![12.0]
    );
    assert!(device.set_parameter("scale", 4.0));
    device
        .try_resolve_parameter_defaults()
        .expect("public changes must refresh dependent child defaults");
    assert_eq!(
        device.try_evaluate().expect("updated hierarchy evaluates"),
        vec![16.0]
    );

    let report = VerilogACompiler::new(CompilerOptions::default())
        .compile_runtime_with_qualifications(
            source,
            Some("parent"),
            RuntimeQualificationOptions::GENERATED_RUST_REQUIRED,
        )
        .expect("runtime artifacts must agree on elaborated hierarchy");
    assert_eq!(report.abi.parameters.len(), 1);
    assert_eq!(report.abi.parameters[0].name, "scale");
    let state = report
        .generated_rust
        .as_ref()
        .expect("generated hierarchy runtime")
        .files
        .iter()
        .find(|file| file.relative_path.ends_with("state.rs"))
        .expect("generated state file");
    let lookup_start = state
        .contents
        .find("const PARAMETER_LOOKUP_NAMES")
        .expect("parameter lookup table");
    let lookup_end = state.contents[lookup_start..]
        .find("];\n")
        .map(|offset| lookup_start + offset)
        .expect("parameter lookup terminator");
    let lookup = &state.contents[lookup_start..lookup_end];
    assert!(lookup.contains("scale"));
    assert!(!lookup.contains(hidden_name.as_str()));
}

#[test]
fn defaulted_child_parameter_preserves_param_given_false() {
    let model = compile_selected(
        r#"
`include "disciplines.vams"
module gain_cell(p, n);
    inout p, n;
    electrical p, n;
    parameter real gain = 2.0;
    analog I(p, n) <+ ($param_given(gain) ? gain : 7.0) * V(p, n);
endmodule
module parent(p, n);
    inout p, n;
    electrical p, n;
    gain_cell stage (p, n);
endmodule
"#,
        "parent",
    );
    let mut device = model.device("X1", &[1, 0]);
    device.update_voltages(&[2.0]);
    assert_eq!(
        device.try_evaluate().expect("hierarchy evaluates"),
        vec![14.0]
    );
}

#[test]
fn named_and_ordered_ports_preserve_port_connected_semantics() {
    let model = compile_selected(
        r#"
`include "disciplines.vams"
module optional_cell(p, n, opt);
    inout p, n, opt;
    electrical p, n, opt;
    analog I(p, n) <+ ($port_connected(opt) ? 10.0 : 1.0) * V(p, n);
endmodule
module parent(p, n);
    inout p, n;
    electrical p, n;
    optional_cell named (.p(p), .n(n), .opt());
    optional_cell ordered (p, n, p);
endmodule
"#,
        "parent",
    );
    let mut device = model.device("X1", &[1, 0]);
    device.update_voltages(&[2.0]);
    let total: f64 = device
        .try_evaluate()
        .expect("both child instances evaluate")
        .into_iter()
        .sum();
    assert_eq!(total, 22.0);
}

#[test]
fn nested_instances_retain_internal_nodes_variables_arrays_and_named_branches() {
    let source = r#"
`include "disciplines.vams"
module segment(p, n);
    inout p, n;
    electrical p, n;
    parameter real g = 2.0;
    real cache[0:1];
    branch (p, n) path;
    analog begin
        cache[0] = g;
        cache[1] = V(path);
        I(path) <+ cache[0] * cache[1];
    end
endmodule
module divider(p, n);
    inout p, n;
    electrical p, n, mid;
    segment first (p, mid);
    segment second (mid, n);
endmodule
module parent(p, n);
    inout p, n;
    electrical p, n;
    divider network (p, n);
endmodule
"#;
    let model = compile_selected(source, "parent");
    assert_eq!(model.internal_nodes, 1);
    assert_eq!(model.stamp_programs.len(), 2);
    assert!(model.num_variables >= 4);
    assert_eq!(
        model
            .parameters
            .iter()
            .filter(|parameter| !parameter.is_public)
            .count(),
        2
    );
    VerilogACompiler::new(CompilerOptions::default())
        .compile_runtime(source, Some("parent"))
        .expect("bytecode and canonical hierarchy artifacts must match");
}

#[test]
#[cfg(not(feature = "native"))]
fn runtime_indexed_arrays_are_rebased_per_child_instance() {
    let model = compile_selected(
        r#"
`include "disciplines.vams"
module indexed_cell(p, n);
    inout p, n;
    electrical p, n;
    parameter integer selected = 0 from [0:1];
    real cache[0:1];
    analog begin
        cache[selected] = V(p, n);
        I(p, n) <+ cache[selected];
    end
endmodule
module parent(p, n);
    inout p, n;
    electrical p, n;
    indexed_cell #(0) first (p, n);
    indexed_cell #(1) second (p, n);
endmodule
"#,
        "parent",
    );
    assert!(model.num_variables >= 4);
    let mut device = model.device("X1", &[1, 0]);
    device.update_voltages(&[3.0]);
    assert_eq!(
        device.try_evaluate().expect("indexed children evaluate"),
        vec![3.0, 3.0]
    );
}

#[test]
fn repeated_child_noise_labels_are_instance_qualified() {
    let model = compile_selected(
        r#"
`include "disciplines.vams"
module noisy(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ white_noise(1.0, "thermal");
endmodule
module parent(p, n);
    inout p, n;
    electrical p, n;
    noisy first (p, n);
    noisy second (p, n);
endmodule
"#,
        "parent",
    );
    let names = model
        .noise_sources
        .iter()
        .map(|noise| noise.name.as_deref().expect("named source"))
        .collect::<Vec<_>>();
    assert_eq!(names.len(), 2);
    assert_ne!(names[0], names[1]);
    assert!(names.iter().all(|name| name.ends_with(":thermal")));
}

#[test]
fn child_parameter_ranges_are_enforced_after_override_binding() {
    let model = compile_selected(
        r#"
`include "disciplines.vams"
module constrained(p, n);
    inout p, n;
    electrical p, n;
    parameter real gain = 1.0 from (0:inf);
    analog I(p, n) <+ gain * V(p, n);
endmodule
module parent(p, n);
    inout p, n;
    electrical p, n;
    constrained #(.gain(-1.0)) invalid (p, n);
endmodule
"#,
        "parent",
    );
    let error = model
        .try_device("X1", &[1, 0])
        .expect_err("child override must satisfy the child's declared range");
    assert!(error.to_string().contains("violates range"), "{error}");
}

#[test]
fn hierarchy_rejects_cycles_mixed_bindings_and_discipline_mismatches() {
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    for (source, expected) in [
        (
            r#"
module parent(p); inout p; electrical p; child next(p); endmodule
module child(p); inout p; electrical p; parent next(p); endmodule
"#,
            "Circular dependency",
        ),
        (
            r#"
module child(p, n); inout p, n; electrical p, n; endmodule
module parent(p, n); inout p, n; electrical p, n; child bad (p, .n(n)); endmodule
"#,
            "mixed named and ordered module port connections",
        ),
        (
            r#"
nature Heat; access = Temp; endnature
discipline thermal; potential Heat; enddiscipline
module child(p); inout p; thermal p; endmodule
module parent(p); inout p; electrical p; child bad(p); endmodule
"#,
            "discipline mismatch",
        ),
        (
            r#"
module child(p); inout p; electrical p; endmodule
module parent(p); inout p; electrical p; child duplicate (.p(), .p()); endmodule
"#,
            "Duplicate symbol",
        ),
    ] {
        let error = compiler
            .compile_module(source, Some("parent"))
            .expect_err("invalid hierarchy must fail closed");
        assert!(
            error.to_string().contains(expected),
            "unexpected error: {error}"
        );
    }
}
