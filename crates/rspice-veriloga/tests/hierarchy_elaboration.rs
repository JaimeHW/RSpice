//! End-to-end qualification of structural Verilog-A elaboration.

mod support;

use rspice_veriloga::canonical_ir::HirExprKind;
use rspice_veriloga::{CompilerOptions, RuntimeQualificationOptions, VerilogACompiler};
use std::collections::HashSet;
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

fn parameter_array_hierarchy(overrides: &str) -> String {
    [
        r#"
module shaped(p, n);
    inout p, n;
    electrical p, n;
    parameter integer width = 2;
    parameter real taps[width - 1:0] = '{1.0, 2.0};
    analog I(p, n) <+ 0.0;
endmodule
module parent(p, n);
    inout p, n;
    electrical p, n;
    shaped #("#,
        overrides,
        r#") stage(p, n);
endmodule
"#,
    ]
    .concat()
}

#[test]
fn hierarchy_array_shape_preserves_dependent_integer_division() {
    let source = r#"
module shaped(p, n);
    inout p, n;
    electrical p, n;
    parameter integer denominator = 2;
    parameter real left = 1 / denominator;
    parameter real taps[left:0] = '{1.0};
    analog I(p, n) <+ 0.0;
endmodule
module parent(p, n);
    inout p, n;
    electrical p, n;
    shaped stage(p, n);
endmodule
"#;

    VerilogACompiler::default()
        .compile_canonical_ir_module(source, Some("parent"))
        .expect("integer division must resolve the child array shape to one element");
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
    assert_eq!(
        model
            .noise_sources
            .iter()
            .map(|source| source.process_id)
            .collect::<Vec<_>>(),
        vec![0, 1]
    );
}

#[test]
fn hierarchy_noise_process_ids_are_dense_across_root_and_repeated_children() {
    let source = r#"
`include "disciplines.vams"
module noisy_child(p, n);
    inout p, n; electrical p, n;
    real process;
    analog begin
        process = white_noise(1.0, "child");
        I(p, n) <+ process + process;
    end
endmodule
module noisy_parent(p, n);
    inout p, n; electrical p, n;
    analog I(p, n) <+ white_noise(2.0, "root");
    noisy_child first(p, n);
    noisy_child second(p, n);
endmodule
"#;
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    let model = compiler
        .compile_module(source, Some("noisy_parent"))
        .expect("root and repeated noisy children compile");
    let artifact = compiler
        .compile_canonical_ir_module(source, Some("noisy_parent"))
        .expect("canonical noisy hierarchy compiles");
    let runtime_ids = model
        .noise_sources
        .iter()
        .map(|source| source.process_id)
        .collect::<Vec<_>>();
    assert_eq!(runtime_ids, vec![0, 1, 2]);

    let mut canonical_ids = artifact
        .hir
        .expressions
        .iter()
        .filter_map(|expression| match expression.kind {
            HirExprKind::NoiseSource { process_id, .. } => Some(process_id as usize),
            _ => None,
        })
        .collect::<Vec<_>>();
    canonical_ids.sort_unstable();
    canonical_ids.dedup();
    assert_eq!(canonical_ids, runtime_ids);
    compiler
        .compile_runtime(source, Some("noisy_parent"))
        .expect("bytecode and canonical noisy hierarchy artifacts match");
}

#[test]
fn nested_repeated_noisy_children_receive_distinct_process_ids() {
    let source = r#"
`include "disciplines.vams"
module noisy_leaf(p, n);
    inout p, n; electrical p, n;
    analog I(p, n) <+ white_noise(1.0, "leaf");
endmodule
module noisy_pair(p, n);
    inout p, n; electrical p, n;
    noisy_leaf left(p, n);
    noisy_leaf right(p, n);
endmodule
module noisy_tree(p, n);
    inout p, n; electrical p, n;
    noisy_pair first(p, n);
    noisy_pair second(p, n);
endmodule
"#;
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    let model = compiler
        .compile_module(source, Some("noisy_tree"))
        .expect("nested repeated noisy hierarchy compiles");
    assert_eq!(
        model
            .noise_sources
            .iter()
            .map(|source| source.process_id)
            .collect::<Vec<_>>(),
        vec![0, 1, 2, 3]
    );
    let names = model
        .noise_sources
        .iter()
        .map(|source| source.name.as_deref().expect("qualified leaf label"))
        .collect::<HashSet<_>>();
    assert_eq!(names.len(), 4);
    compiler
        .compile_runtime(source, Some("noisy_tree"))
        .expect("nested bytecode and canonical artifacts match");
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

#[test]
fn hierarchy_parameter_array_width_change_without_replacement_is_rejected() {
    let error = VerilogACompiler::default()
        .compile_canonical_ir_module(&parameter_array_hierarchy(".width(3)"), Some("parent"))
        .expect_err("changed child array bounds require an atomic replacement");
    let message = error.to_string();
    assert!(
        message.contains("parent.stage"),
        "unexpected error: {message}"
    );
    assert!(
        message.contains("changes parameter array 'taps' bounds from [1:0] to [2:0]"),
        "unexpected error: {message}"
    );
    assert!(
        message.contains("same instance parameter override list"),
        "unexpected error: {message}"
    );
}

#[test]
fn hierarchy_parameter_array_equal_extent_bound_change_keeps_declared_default() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir_module(&parameter_array_hierarchy(".width(0)"), Some("parent"))
        .expect("equal per-dimension extents do not require an array replacement");
    let taps = artifact
        .hir
        .parameters
        .iter()
        .find(|parameter| parameter.name.ends_with("_taps"))
        .expect("flattened child array parameter");
    let initializer = taps.default_expr.as_ref().expect("declared array default");
    assert!(matches!(
        artifact.hir.expressions[usize::from(initializer.id)].kind,
        HirExprKind::ArrayLiteral {
            ref elements,
            assignment_pattern: true,
        } if elements.len() == 2
    ));
}

#[test]
fn hierarchy_parameter_array_mismatched_replacement_is_rejected() {
    let error = VerilogACompiler::default()
        .compile_canonical_ir_module(
            &parameter_array_hierarchy(".width(3), .taps('{1.0, 2.0})"),
            Some("parent"),
        )
        .expect_err("replacement must exactly match the effective child shape");
    let message = error.to_string();
    assert!(
        message.contains("override of parameter array 'taps' at instance 'parent.stage'"),
        "unexpected error: {message}"
    );
    assert!(
        message.contains("effective shape [3]"),
        "unexpected error: {message}"
    );
    assert!(
        message.contains("dimension 1 has 2 elements"),
        "unexpected error: {message}"
    );
}

#[test]
fn hierarchy_parameter_array_matching_replacement_is_accepted_and_keeps_direction() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir_module(
            &parameter_array_hierarchy(".width(3), .taps('{1.0, 2.0, 3.0})"),
            Some("parent"),
        )
        .expect("matching atomic child array replacement must elaborate");
    let taps = artifact
        .hir
        .parameters
        .iter()
        .find(|parameter| parameter.name.ends_with("_taps"))
        .expect("flattened child array parameter");
    let initializer = taps
        .default_expr
        .as_ref()
        .map(|expression| &artifact.hir.expressions[usize::from(expression.id)].kind)
        .expect("array replacement expression");
    assert!(matches!(
        initializer,
        HirExprKind::ArrayLiteral {
            elements,
            assignment_pattern: true,
        } if elements.len() == 3
    ));

    let dimension = &taps.dimensions[0];
    assert!(matches!(
        &artifact.hir.expressions[usize::from(dimension.left.id)].kind,
        HirExprKind::Binary { op, .. } if op == "Sub"
    ));
    assert!(matches!(
        &artifact.hir.expressions[usize::from(dimension.right.id)].kind,
        HirExprKind::Number { value, .. } if *value == 0.0
    ));
}
