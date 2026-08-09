//! The canonical backend produces a device, and the device compiles.
//!
//! A generator test that only inspects the emitted text proves the text looks
//! right, which is not the property anyone needs. What matters is that the two
//! files fit together and fit the runtime: that `stamp.rs` names fields
//! `state.rs` actually declares, that the borrows inside a stamp are disjoint,
//! and that every stamper call matches a real signature with the right arity.
//! Only `rustc` can answer those, so this hands them to `rustc`.
//!
//! The runtime is stubbed rather than linked, because linking `rspice-core`
//! would mean writing the device into its source tree. The stub carries the
//! exact signatures the emitted code calls and nothing else — if a call shape
//! drifts, this fails at the call site with the same message the real build
//! would give.

use rspice_veriloga::rust_backend::{
    RustBackendErrorKind, RustTranspileOptions, RustTranspiler, canonical,
};
use rspice_veriloga::{PipelineControl, PipelinePhase, VerilogACompiler};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

#[test]
fn a_generated_device_compiles_against_the_runtime_contract() {
    for (name, source) in fixtures() {
        let artifact = VerilogACompiler::default()
            .compile_canonical_ir(source)
            .unwrap_or_else(|error| panic!("{name}: front end: {error}"));
        let device = canonical::generate_device(&artifact, &options())
            .unwrap_or_else(|error| panic!("{name}: generation: {error}"));

        let files: Vec<(&str, &str)> = device
            .files
            .iter()
            .map(|file| (file.relative_path.as_str(), file.contents.as_str()))
            .collect();
        let state = find(&files, "state.rs", name);
        let stamp = find(&files, "stamp.rs", name);
        let noise = find(&files, "noise.rs", name);

        if let Err(report) = compile(name, state, stamp, noise) {
            panic!("{name}: the generated device does not compile:\n{report}");
        }
    }
}

#[test]
fn generated_terminal_metadata_preserves_source_order_spelling_and_current_names() {
    let (state, _, _) = generated_parts(
        r#"
module terminal_metadata(d, FG, s, Tnode);
    input d;
    output FG;
    inout s, Tnode;
    electrical d, FG, s, Tnode;
    analog I(d, s) <+ V(d, s);
endmodule
"#,
        "terminal metadata",
    );

    assert!(state.contains(
        "GeneratedVerilogATerminalDescriptor { name: \"d\", direction: GeneratedVerilogATerminalDirection::Input, discipline: \"electrical\", current_parameter: \"id\" }"
    ));
    assert!(state.contains(
        "GeneratedVerilogATerminalDescriptor { name: \"FG\", direction: GeneratedVerilogATerminalDirection::Output, discipline: \"electrical\", current_parameter: \"ifg\" }"
    ));
    assert!(state.contains(
        "GeneratedVerilogATerminalDescriptor { name: \"Tnode\", direction: GeneratedVerilogATerminalDirection::InOut, discipline: \"electrical\", current_parameter: \"itnode\" }"
    ));
    assert!(!state.contains("TERMINAL_NAMES"));
}

#[test]
fn generated_parameter_descriptors_are_the_public_scope_authority() {
    let (state, _, _) = generated_parts(
        r#"
module parameter_metadata(p, n);
    inout p, n;
    electrical p, n;
    parameter real limit = 10.0;
    (* type = "instance", xyceAlsoModel = "yes" *)
        parameter integer gain = 2 from [0.0:limit];
    aliasparam GAIN_ALIAS = gain;
    analog I(p, n) <+ gain * V(p, n);
endmodule
"#,
        "parameter descriptor metadata",
    );

    assert!(state.contains(
        "GeneratedVerilogAParameterDescriptor { name: \"gain\", aliases: &[\"GAIN_ALIAS\"], scope: GeneratedVerilogAParameterScope::Dual, is_integer: true, default: Some(2.0), minimum: Some(GeneratedVerilogAParameterBound { value: 0.0, exclusive: false }), maximum: None, excluded_values: &[], has_dynamic_constraints: true }"
    ));
    assert!(!state.contains("pub fn parameter_scope"));
}

/// The zeros are the point, so they are checked separately from compiling.
///
/// A two-terminal resistor reaches two unknowns and no more. The tier this
/// replaces writes `multiplicity * 0.0` for the rest of the row; here the
/// entries do not exist, so the literal never appears.
#[test]
fn a_stamp_writes_no_literal_zero_entries() {
    let source = r#"
module divider(p, n);
    inout p, n;
    electrical p, n;
    electrical mid;
    parameter real r1 = 100.0;
    parameter real r2 = 220.0;
    analog begin
        I(p, mid) <+ V(p, mid) / r1;
        I(mid, n) <+ V(mid, n) / r2;
    end
endmodule
"#;
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(source)
        .expect("front end");
    let device = canonical::generate_device(&artifact, &options()).expect("generation");
    let files: Vec<(&str, &str)> = device
        .files
        .iter()
        .map(|file| (file.relative_path.as_str(), file.contents.as_str()))
        .collect();
    let stamp = find(&files, "stamp.rs", "divider");

    for (index, line) in stamp.lines().enumerate() {
        assert!(
            !line.contains("multiplicity * 0.0"),
            "divider: stamp.rs line {} writes a literal zero: {line}",
            index + 1
        );
    }
    assert!(
        stamp.contains("stamp_current_sparse_local::<2, 0>"),
        "each branch of the divider reaches exactly two nodes; stamp.rs was:\n{stamp}"
    );
}

#[test]
fn generated_dependent_parameter_defaults_finalize_after_all_overrides() {
    let source = r#"
module dependent_defaults(p, n);
    inout p, n;
    electrical p, n;
    parameter real base = 2.0;
    parameter real alias = base;
    parameter real chain = alias * 3.0;
    parameter real mode = 0.0;
    parameter real choice = mode > 0.5 ? chain + 1.0 : chain - 1.0;
    parameter real given_sensitive = $param_given(base) ? choice + 100.0 : choice;
    parameter real bounded_source = 1.0;
    parameter real bounded_dependent = bounded_source * 2.0 from [0.0:10.0];
    analog I(p, n) <+ given_sensitive * V(p, n);
endmodule
"#;
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(source)
        .expect("front end");
    let device = canonical::generate_device(&artifact, &options()).expect("generation");
    let files = device
        .files
        .iter()
        .map(|file| (file.relative_path.as_str(), file.contents.as_str()))
        .collect::<Vec<_>>();
    let state = find(&files, "state.rs", "dependent defaults");
    let stamp = find(&files, "stamp.rs", "dependent defaults");
    let noise = find(&files, "noise.rs", "dependent defaults");

    assert!(state.contains("pub fn finalize_parameters(&mut self)"));
    if let Err(report) =
        run_dependent_parameter_defaults("dependent parameter defaults", state, stamp, noise)
    {
        panic!("generated dependent parameter defaults failed:\n{report}");
    }
}

#[test]
fn generated_parameter_assignment_scope_and_dual_fallback_are_preserved() {
    let (state, stamp, noise) = generated_parts(
        r#"
module scoped_parameters(p, n);
    inout p, n;
    electrical p, n;
    parameter real model_only = 2.0;
    (* type = "instance" *) parameter real instance_only = 3.0;
    (* type = "instance", xyceAlsoModel = "yes" *) parameter real dual = 4.0;
    parameter real model_from_dual = dual * 2.0;
    (* type = "instance" *) parameter real dual_seen_given = $param_given(dual) ? 1.0 : 0.0;
    real shape;
    analog begin
        shape = model_only * model_only + instance_only * dual + model_from_dual;
        shape = shape * shape + dual_seen_given;
        I(p, n) <+ shape * V(p, n);
    end
endmodule
"#,
        "scoped parameter assignments",
    );
    let body = r#"
use runtime::{GeneratedParameterAssignment as Assignment, GeneratedParameterOrigin as Origin, GeneratedVerilogAParameterScope as Scope};
let mut instance = device::state::Instance::new(&[0, 1]);
let defaults = instance.params.values;

let scope = |name: &str| device::state::Instance::PARAMETER_DESCRIPTORS
    .iter()
    .find(|parameter| parameter.name.eq_ignore_ascii_case(name))
    .map(|parameter| parameter.scope);
assert_eq!(scope("MODEL_ONLY"), Some(Scope::Model));
assert_eq!(scope("instance_only"), Some(Scope::Instance));
assert_eq!(scope("dual"), Some(Scope::Dual));
assert_eq!(scope("missing"), None);

let error = instance.apply_parameters(&[
    Assignment::new("model_only", 7.0, Origin::Instance),
]).unwrap_err();
assert!(error.contains("model-card"), "{error}");
assert_eq!(instance.params.values, defaults);

let error = instance.apply_parameters(&[
    Assignment::new("instance_only", 7.0, Origin::ModelCard),
]).unwrap_err();
assert!(error.contains("instance"), "{error}");
assert_eq!(instance.params.values, defaults);

instance.apply_parameters(&[
    Assignment::new("MODEL_ONLY", 5.0, Origin::ModelCard),
    Assignment::new("dual", 8.0, Origin::ModelCard),
]).unwrap();
assert_eq!(instance.params.values[0], 5.0);
assert_eq!(instance.params.values[2], 8.0);
assert_eq!(instance.params.values[3], 16.0);
assert_eq!(instance.params.values[4], 0.0, "model fallback must not set the instance given bit");

instance.apply_parameters(&[
    Assignment::new("dual", 9.0, Origin::Instance),
]).unwrap();
assert_eq!(instance.params.values[2], 9.0);
assert_eq!(instance.params.values[3], 16.0, "model default must keep using dual-scope model storage");
assert_eq!(instance.params.values[4], 1.0, "instance override must set the instance given bit");

instance.apply_parameters(&[
    Assignment::new("dual", 11.0, Origin::ModelCard),
]).unwrap();
assert_eq!(instance.params.values[2], 9.0, "instance override must outrank the model fallback");
assert_eq!(instance.params.values[3], 22.0, "model default must follow the changed model fallback");
assert_eq!(instance.params.values[4], 1.0);
"#;
    run_generated_main("scoped parameter assignments", &state, &stamp, &noise, body)
        .unwrap_or_else(|report| panic!("generated parameter scope probe failed:\n{report}"));
}

#[test]
fn generated_dual_scope_dependent_defaults_follow_xyce_instance_finalization() {
    let (state, stamp, noise) = generated_parts(
        r#"
module dual_dependent_defaults(p, n);
    inout p, n;
    electrical p, n;
    (* type = "instance", xyceAlsoModel = "yes" *) parameter real base = 2.0;
    (* type = "instance", xyceAlsoModel = "yes" *) parameter real dependent = base * 3.0;
    (* type = "instance" *) parameter real dependent_seen_given = $param_given(dependent) ? 1.0 : 0.0;
    (* type = "instance", xyceAlsoModel = "yes" *) parameter real bounded = base * 2.0 from [0.0:10.0];
    analog I(p, n) <+ (dependent + dependent_seen_given + bounded) * V(p, n);
endmodule
"#,
        "dual-scope dependent defaults",
    );
    let body = r#"
use runtime::{GeneratedParameterAssignment as Assignment, GeneratedParameterOrigin as Origin};
let mut instance = device::state::Instance::new(&[0, 1]);

instance.apply_parameters(&[
    Assignment::new("base", 4.0, Origin::ModelCard),
    Assignment::new("dependent", 99.0, Origin::ModelCard),
]).unwrap();
assert_eq!(instance.params.values[0], 4.0);
assert_eq!(instance.params.values[1], 12.0, "an unset dual instance parameter must recompute its dependent default after model fallback");
assert_eq!(instance.params.values[2], 0.0, "model fallback must not set the instance given bit");
assert_eq!(instance.params.values[3], 8.0);

instance.apply_parameters(&[
    Assignment::new("base", 5.0, Origin::Instance),
]).unwrap();
assert_eq!(instance.params.values[0], 5.0);
assert_eq!(instance.params.values[1], 15.0, "a dependent dual default must use the effective instance value");
assert_eq!(instance.params.values[2], 0.0);
assert_eq!(instance.params.values[3], 10.0);

let before_values = instance.params.values;
let before_given = instance.param_given.clone();
let error = instance.apply_parameters(&[
    Assignment::new("base", 6.0, Origin::Instance),
]).unwrap_err();
assert!(error.contains("bounded"), "{error}");
assert_eq!(instance.params.values, before_values, "failed dependent-default validation must roll back every effective value");
assert_eq!(instance.param_given, before_given, "failed dependent-default validation must roll back every instance given bit");

instance.apply_parameters(&[
    Assignment::new("dependent", 21.0, Origin::Instance),
]).unwrap();
assert_eq!(instance.params.values[1], 21.0, "an explicit instance assignment must outrank the dependent default");
assert_eq!(instance.params.values[2], 1.0, "an explicit instance assignment must set the instance given bit");

instance.apply_parameters(&[
    Assignment::new("dependent", 30.0, Origin::ModelCard),
]).unwrap();
assert_eq!(instance.params.values[1], 21.0, "a later model-card assignment must not replace an explicit instance assignment");
assert_eq!(instance.params.values[2], 1.0);
"#;
    run_generated_main(
        "dual-scope dependent defaults",
        &state,
        &stamp,
        &noise,
        body,
    )
    .unwrap_or_else(|report| panic!("generated dual-scope dependent defaults failed:\n{report}"));
}

#[test]
fn generated_model_ranges_validate_against_completed_instance_geometry() {
    let (state, stamp, noise) = generated_parts(
        r#"
module cross_scope_range(p, n);
    inout p, n;
    electrical p, n;
    parameter real overlap = 1.0 from [0.0:length];
    parameter real scale = 1.0;
    (* type = "instance" *) parameter real length = 1.0;
    (* type = "instance" *) parameter real derived = scale * 2.0 from [0.0:3.0];
    analog I(p, n) <+ overlap * length * derived * V(p, n);
endmodule
"#,
        "cross scope parameter range",
    );
    let body = r#"
use runtime::{GeneratedParameterAssignment as Assignment, GeneratedParameterOrigin as Origin};
let mut instance = device::state::Instance::new(&[0, 1]);
instance.apply_parameters(&[
    Assignment::new("overlap", 2.0, Origin::ModelCard),
    Assignment::new("scale", 2.0, Origin::ModelCard),
    Assignment::new("length", 3.0, Origin::Instance),
    Assignment::new("derived", 1.0, Origin::Instance),
]).unwrap();
assert_eq!(instance.params.values, [2.0, 2.0, 3.0, 1.0]);

let before = instance.params.values;
let error = instance.apply_parameters(&[
    Assignment::new("overlap", 4.0, Origin::ModelCard),
    Assignment::new("length", 3.0, Origin::Instance),
]).unwrap_err();
assert!(error.contains("overlap"), "{error}");
assert_eq!(instance.params.values, before, "failed cross-scope validation must roll back atomically");
"#;
    run_generated_main("cross scope parameter range", &state, &stamp, &noise, body)
        .unwrap_or_else(|report| panic!("cross-scope range probe failed:\n{report}"));
}

#[test]
fn zero_parameter_generated_state_defines_empty_scope_metadata() {
    let (state, stamp, noise) = generated_parts(
        r#"
module parameterless(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ 2.0 * V(p, n);
endmodule
"#,
        "parameterless state",
    );
    assert!(state.contains("PARAMETER_MODEL_FLAGS: [bool; 0] = []"));
    assert!(state.contains("PARAMETER_DUAL_SCOPE_FLAGS: [bool; 0] = []"));
    compile("parameterless state", &state, &stamp, &noise)
        .unwrap_or_else(|report| panic!("zero-parameter generated state failed:\n{report}"));
}

#[test]
fn generated_static_potential_guard_opens_and_closes_one_branch() {
    let (state, stamp, noise) = generated_parts(
        r#"
module guarded_short(p, n);
    inout p, n;
    electrical p, n;
    parameter integer enabled = 0;
    real guard;
    analog begin
        guard = enabled;
        if (guard > 0)
            V(p, n) <+ 0.0;
    end
endmodule
"#,
        "guarded short",
    );
    assert!(stamp.contains("stamp_inactive_potential_branch_local"));
    let body = r#"
fn sample(enabled: f64) -> [f64; 9] {
    let mut instance = device::state::Instance::new(&[0, 1]);
    instance.set_branch_indices(&[2]);
    instance.set_parameter("enabled", enabled).unwrap();
    instance.finalize_parameters().unwrap();
    let voltages = [0.75, 0.0, 0.0];
    let ctx = runtime::GeneratedEvalContext { voltages: &voltages, temperature: 300.15 };
    let mut sink = [0.0; 9];
    let mut stamper = runtime::GeneratedStamper { sink: Some(&mut sink) };
    instance.stamp(&ctx, &mut stamper);
    sink
}
let open = sample(0.0);
assert_eq!(open[0], 0.0, "open structural coupling: {open:?}");
assert_eq!(open[1], 1.0, "open branch identity: {open:?}");
assert_eq!(open[8], 1.0, "leader ordinal must be pinned: {open:?}");
let closed = sample(1.0);
assert_eq!(closed[0], 1.0, "closed structural coupling: {closed:?}");
assert_eq!(closed[1], 0.0, "closed branch must not be pinned: {closed:?}");
assert_eq!(closed[7], 1.0, "leader ordinal must carry topology: {closed:?}");
"#;
    run_generated_main("guarded short activation", &state, &stamp, &noise, body)
        .unwrap_or_else(|report| panic!("generated guarded short failed:\n{report}"));
}

#[test]
fn generated_static_guard_uses_the_reaching_definition() {
    let (state, stamp, noise) = generated_parts(
        r#"
module captured_mode(p, n, sense);
    inout p, n, sense;
    electrical p, n, sense;
    parameter integer selector = 0;
    real scratch, mode;
    analog begin
        scratch = selector;
        mode = scratch;
        scratch = V(sense);
        if (mode == 2)
            V(p, n) <+ 0.0;
    end
endmodule
"#,
        "reaching-definition static guard",
    );
    let body = r#"
fn sample(selector: f64, sense: f64) -> [f64; 9] {
    let mut instance = device::state::Instance::new(&[0, 1, 2]);
    instance.set_branch_indices(&[3]);
    instance.set_parameter("selector", selector).unwrap();
    instance.finalize_parameters().unwrap();
    let voltages = [0.0, 0.0, sense, 0.0];
    let ctx = runtime::GeneratedEvalContext { voltages: &voltages, temperature: 300.15 };
    let mut sink = [0.0; 9];
    let mut stamper = runtime::GeneratedStamper { sink: Some(&mut sink) };
    instance.stamp(&ctx, &mut stamper);
    sink
}
let open = sample(0.0, 2.0);
assert_eq!(open[0], 0.0, "later dynamic scratch reuse must not close topology: {open:?}");
assert_eq!(open[1], 1.0, "inactive branch must be pinned: {open:?}");
let closed = sample(2.0, -3.0);
assert_eq!(closed[0], 1.0, "captured static mode must close topology: {closed:?}");
assert_eq!(closed[1], 0.0, "active branch must not be pinned: {closed:?}");
"#;
    run_generated_main(
        "reaching-definition static guard",
        &state,
        &stamp,
        &noise,
        body,
    )
    .unwrap_or_else(|report| panic!("reaching-definition guard probe failed:\n{report}"));
}

#[test]
fn generated_newton_value_uses_parameter_before_shadowing_block_local() {
    let (state, stamp, noise) = generated_parts(
        r#"
module parameter_shadow(p, n);
    inout p, n;
    electrical p, n;
    parameter real scale = 0.5;
    real captured;
    analog begin
        captured = scale;
        begin : load
            real scale;
            scale = 0.25;
            I(p, n) <+ (captured + 10.0 * scale) * V(p, n);
        end
    end
endmodule
"#,
        "parameter shadowing block local",
    );
    let body = r#"
let mut instance = device::state::Instance::new(&[0, 1]);
instance.finalize_parameters().unwrap();
let voltages = [2.0, 0.0];
let ctx = runtime::GeneratedEvalContext { voltages: &voltages, temperature: 300.15 };
let mut sink = [0.0; 10];
let mut stamper = runtime::GeneratedStamper { sink: Some(&mut sink) };
instance.stamp(&ctx, &mut stamper);
assert_eq!(sink[0], 6.0, "the parameter and shadowing local must remain distinct: {sink:?}");
"#;
    run_generated_main(
        "parameter shadowing block local",
        &state,
        &stamp,
        &noise,
        body,
    )
    .unwrap_or_else(|report| panic!("parameter-shadowing probe failed:\n{report}"));
}

#[test]
fn generated_guard_uses_a_later_dynamic_redefinition() {
    let (state, stamp, noise) = generated_parts(
        r#"
module redefined_mode(p, n, sense);
    inout p, n, sense;
    electrical p, n, sense;
    parameter integer selector = 0;
    real mode;
    analog begin
        mode = selector;
        mode = V(sense);
        if (mode > 0.0)
            V(p, n) <+ 2.0;
    end
endmodule
"#,
        "dynamic redefinition guard",
    );
    let body = r#"
fn sample(selector: f64, sense: f64) -> [f64; 9] {
    let mut instance = device::state::Instance::new(&[0, 1, 2]);
    instance.set_branch_indices(&[3]);
    instance.set_parameter("selector", selector).unwrap();
    instance.finalize_parameters().unwrap();
    let voltages = [0.0, 0.0, sense, 0.0];
    let ctx = runtime::GeneratedEvalContext { voltages: &voltages, temperature: 300.15 };
    let mut sink = [0.0; 9];
    let mut stamper = runtime::GeneratedStamper { sink: Some(&mut sink) };
    instance.stamp(&ctx, &mut stamper);
    sink
}
let runtime_false = sample(2.0, -1.0);
assert_eq!(runtime_false[0], 1.0, "dynamic guard must retain topology: {runtime_false:?}");
assert_eq!(runtime_false[1], 0.0, "dynamic branch must not be pinned: {runtime_false:?}");
assert_eq!(runtime_false[2], 0.0, "untaken dynamic contribution is zero: {runtime_false:?}");
let runtime_true = sample(0.0, 1.0);
assert_eq!(runtime_true[0], 1.0, "dynamic topology must remain fixed: {runtime_true:?}");
assert_eq!(runtime_true[2], 2.0, "taken dynamic contribution: {runtime_true:?}");
"#;
    run_generated_main("dynamic redefinition guard", &state, &stamp, &noise, body)
        .unwrap_or_else(|report| panic!("dynamic-redefinition guard probe failed:\n{report}"));
}

#[test]
fn generated_temperature_and_analysis_guards_are_topology_static() {
    let (state, stamp, noise) = generated_parts(
        r#"
module environment_guards(p, n);
    inout p, n;
    electrical p, n;
    analog begin
        if ($temperature > 350.0)
            V(p, n) <+ 1.0;
        if (analysis("ac"))
            V(p, n) <+ 2.0;
    end
endmodule
"#,
        "environment topology guards",
    );
    let body = r#"
fn sample(temperature: f64) -> [f64; 9] {
    let mut instance = device::state::Instance::new(&[0, 1]);
    instance.set_branch_indices(&[2, 3]);
    instance.finalize_parameters().unwrap();
    let voltages = [0.0, 0.0, 0.0, 0.0];
    let ctx = runtime::GeneratedEvalContext { voltages: &voltages, temperature };
    let mut sink = [0.0; 9];
    let mut stamper = runtime::GeneratedStamper { sink: Some(&mut sink) };
    instance.stamp(&ctx, &mut stamper);
    sink
}
let open = sample(300.15);
assert_eq!(open[0], 0.0, "both environment guards false open: {open:?}");
assert_eq!(open[1], 2.0, "leader and duplicate pinned: {open:?}");
let hot = sample(400.0);
assert_eq!(hot[0], 1.0, "temperature guard closes topology: {hot:?}");
assert_eq!(hot[1], 1.0, "duplicate remains pinned: {hot:?}");
assert_eq!(hot[2], 1.0, "temperature contribution: {hot:?}");
let ac = sample(123.0);
assert_eq!(ac[0], 1.0, "analysis guard closes topology: {ac:?}");
assert_eq!(ac[2], 2.0, "analysis contribution: {ac:?}");
"#;
    run_generated_main("environment topology guards", &state, &stamp, &noise, body)
        .unwrap_or_else(|report| panic!("environment guard probe failed:\n{report}"));
}

#[test]
fn generated_unguarded_and_guarded_potentials_sum_on_one_branch() {
    let (state, stamp, noise) = generated_parts(
        r#"
module mixed_sources(p, n);
    inout p, n;
    electrical p, n;
    parameter integer enabled = 0;
    analog begin
        V(p, n) <+ 1.0;
        if (enabled > 0)
            V(p, n) <+ 2.0;
    end
endmodule
"#,
        "unguarded and guarded potentials",
    );
    let body = r#"
fn sample(enabled: f64) -> [f64; 9] {
    let mut instance = device::state::Instance::new(&[0, 1]);
    instance.set_branch_indices(&[2, 3]);
    instance.set_parameter("enabled", enabled).unwrap();
    instance.finalize_parameters().unwrap();
    let voltages = [0.0; 4];
    let ctx = runtime::GeneratedEvalContext { voltages: &voltages, temperature: 300.15 };
    let mut sink = [0.0; 9];
    let mut stamper = runtime::GeneratedStamper { sink: Some(&mut sink) };
    instance.stamp(&ctx, &mut stamper);
    sink
}
let base = sample(0.0);
assert_eq!(base[0], 1.0, "unguarded source owns topology: {base:?}");
assert_eq!(base[1], 1.0, "duplicate pinned: {base:?}");
assert_eq!(base[2], 1.0, "unguarded residual: {base:?}");
let summed = sample(1.0);
assert_eq!(summed[0], 1.0, "physical branch still couples once: {summed:?}");
assert_eq!(summed[2], 3.0, "contributions sum: {summed:?}");
"#;
    run_generated_main(
        "unguarded and guarded potentials",
        &state,
        &stamp,
        &noise,
        body,
    )
    .unwrap_or_else(|report| panic!("mixed potential probe failed:\n{report}"));
}

#[test]
fn generated_one_terminal_port_flow_reads_leader_once() {
    let (state, stamp, noise) = generated_parts(
        r#"
module port_flow(p, n, out);
    inout p, n, out;
    electrical p, n, out;
    parameter integer mode = 0;
    analog begin
        if (mode == 1)
            V(p, n) <+ 0.0;
        if (mode == 2)
            V(p, n) <+ 0.0;
        I(out, n) <+ I(p);
    end
endmodule
"#,
        "one-terminal physical port flow",
    );
    let body = r#"
let mut instance = device::state::Instance::new(&[0, 1, 2]);
instance.set_branch_indices(&[3, 4]);
instance.set_parameter("mode", 2.0).unwrap();
instance.finalize_parameters().unwrap();
// The first contribution is inactive but its branch is the physical leader.
// A later active duplicate must not redirect or double-count I(p).
let voltages = [0.0, 0.0, 0.0, 0.5, 40.0];
let ctx = runtime::GeneratedEvalContext { voltages: &voltages, temperature: 300.15 };
let mut sink = [0.0; 10];
let mut stamper = runtime::GeneratedStamper { sink: Some(&mut sink) };
instance.stamp(&ctx, &mut stamper);
assert_eq!(sink[0], 1.5, "one topology call plus 0.5 A current: {sink:?}");
assert_eq!(sink[9], 0.5, "I(p) reads the physical leader exactly once: {sink:?}");
assert_eq!(sink[1], 1.0, "duplicate unknown remains pinned: {sink:?}");
"#;
    run_generated_main(
        "one terminal physical port flow",
        &state,
        &stamp,
        &noise,
        body,
    )
    .unwrap_or_else(|report| panic!("one-terminal port-flow probe failed:\n{report}"));
}

#[test]
fn generated_potential_contributions_share_one_physical_branch() {
    let (state, stamp, noise) = generated_parts(
        r#"
module grouped_sources(p, n);
    inout p, n;
    electrical p, n;
    parameter integer mode = 0;
    analog begin
        if (mode == 1)
            V(p, n) <+ 1.0;
        if (mode == 2)
            V(n, p) <+ 2.0;
        if (mode == 3) begin
            V(p, n) <+ 4.0;
            V(n, p) <+ 1.5;
        end
        if (mode == 4)
            V(p, n) <+ 3.0 * I(p, n);
    end
endmodule
"#,
        "grouped potential sources",
    );
    let body = r#"
fn sample(mode: f64) -> [f64; 9] {
    let mut instance = device::state::Instance::new(&[0, 1]);
    instance.set_branch_indices(&[2, 3, 4, 5, 6]);
    instance.set_parameter("mode", mode).unwrap();
    instance.finalize_parameters().unwrap();
    // Only the physical leader (solver index 2) is 0.5 A. Duplicate MIR
    // unknowns are deliberately large so a wrong I(p,n) mapping is obvious.
    let voltages = [0.25, 0.0, 0.5, 10.0, 20.0, 30.0, 40.0];
    let ctx = runtime::GeneratedEvalContext { voltages: &voltages, temperature: 300.15 };
    let mut sink = [0.0; 9];
    let mut stamper = runtime::GeneratedStamper { sink: Some(&mut sink) };
    instance.stamp(&ctx, &mut stamper);
    sink
}
let open = sample(0.0);
assert_eq!(open[0], 0.0, "all inactive must stay open: {open:?}");
assert_eq!(open[1], 5.0, "leader and four duplicates must be pinned: {open:?}");
assert_eq!(open[8], 15.0, "each branch ordinal pinned exactly once: {open:?}");

let forward = sample(1.0);
assert_eq!(forward[0], 1.0, "one physical coupling: {forward:?}");
assert_eq!(forward[1], 4.0, "only duplicates pinned: {forward:?}");
assert_eq!(forward[2], 1.0, "forward residual: {forward:?}");
assert_eq!(forward[5], 1.0, "residual targets leader branch: {forward:?}");

let reversed = sample(2.0);
assert_eq!(reversed[0], 1.0, "one reversed physical coupling: {reversed:?}");
assert_eq!(reversed[2], -2.0, "reversed residual must be negated: {reversed:?}");
assert_eq!(reversed[5], 1.0, "reversed residual targets leader: {reversed:?}");

let summed = sample(3.0);
assert_eq!(summed[0], 1.0, "simultaneous contributions still couple once: {summed:?}");
assert_eq!(summed[1], 4.0, "duplicates remain pinned: {summed:?}");
assert_eq!(summed[2], 2.5, "4.0 + reversed 1.5 must sum: {summed:?}");
assert_eq!(summed[5], 2.0, "both residuals target leader ordinal zero: {summed:?}");

let flow = sample(4.0);
assert_eq!(flow[2], 1.5, "I(p,n) must read leader flow 0.5 A: {flow:?}");
assert_eq!(flow[4], 3.0, "flow derivative: {flow:?}");
assert_eq!(flow[6], 1.0, "flow derivative must name leader ordinal zero: {flow:?}");
"#;
    run_generated_main("grouped potential sources", &state, &stamp, &noise, body)
        .unwrap_or_else(|report| panic!("generated grouped sources failed:\n{report}"));
}

#[test]
fn generated_potential_noise_uses_the_physical_branch_leader() {
    let (state, stamp, noise) = generated_parts(
        r#"
module grouped_potential_noise(p, n);
    inout p, n;
    electrical p, n;
    parameter integer reverse_enabled = 1;
    analog begin
        V(p, n) <+ white_noise(abs(I(p, n)) + 1.0, "forward");
        if (reverse_enabled > 0)
            V(n, p) <+ white_noise(abs(I(n, p)) + 2.0, "reverse");
    end
endmodule
"#,
        "grouped potential noise",
    );
    let body = r#"
#[derive(Default)]
struct Capture(Vec<(bool, f64)>);
impl runtime::GeneratedNoiseVisitor for Capture {
    fn visit(&mut self, _index: usize, value: runtime::GeneratedNoiseEvaluationRef<'_>) -> bool {
        self.0.push((value.active, value.psd));
        true
    }
}

assert_eq!(device::noise::NOISE_SOURCES.len(), 2);
assert_eq!(device::noise::NOISE_SOURCES[0].branch_ordinal, Some(0));
assert_eq!(device::noise::NOISE_SOURCES[1].branch_ordinal, Some(0),
    "reversed duplicate descriptor must name the physical leader");

let mut instance = device::state::Instance::new(&[0, 1]);
instance.set_branch_indices(&[2, 3]);
instance.finalize_parameters().unwrap();
// The physical leader is 0.5 A; the pinned duplicate is deliberately 40 A.
// Both I(p,n) and reversed I(n,p) must read ordinal zero exactly once.
let voltages = [0.0, 0.0, 0.5, 40.0];
let ctx = runtime::GeneratedEvalContext { voltages: &voltages, temperature: 300.15 };
let mut capture = Capture::default();
instance.evaluate_noise_sources(&ctx, &mut capture).unwrap();
assert_eq!(capture.0, vec![(true, 1.5), (true, 2.5)]);

instance.set_parameter("reverse_enabled", 0.0).unwrap();
instance.finalize_parameters().unwrap();
let mut disabled = Capture::default();
instance.evaluate_noise_sources(&ctx, &mut disabled).unwrap();
assert_eq!(disabled.0, vec![(true, 1.5), (false, 0.0)]);
"#;
    run_generated_main("grouped potential noise", &state, &stamp, &noise, body)
        .unwrap_or_else(|report| panic!("grouped potential-noise probe failed:\n{report}"));
}

#[test]
fn generated_dynamic_potential_guards_keep_static_prefix_topology() {
    let (state, stamp, noise) = generated_parts(
        r#"
module static_then_dynamic(p, n);
    inout p, n;
    electrical p, n;
    parameter integer enabled = 0;
    analog begin
        if (enabled > 0) begin
            if (V(p, n) > 0.0)
                V(p, n) <+ 2.0;
        end
    end
endmodule
"#,
        "static then dynamic guard",
    );
    let body = r#"
fn sample(enabled: f64, voltage: f64) -> [f64; 9] {
    let mut instance = device::state::Instance::new(&[0, 1]);
    instance.set_branch_indices(&[2]);
    instance.set_parameter("enabled", enabled).unwrap();
    instance.finalize_parameters().unwrap();
    let voltages = [voltage, 0.0, 0.0];
    let ctx = runtime::GeneratedEvalContext { voltages: &voltages, temperature: 300.15 };
    let mut sink = [0.0; 9];
    let mut stamper = runtime::GeneratedStamper { sink: Some(&mut sink) };
    instance.stamp(&ctx, &mut stamper);
    sink
}
let disabled = sample(0.0, 1.0);
assert_eq!(disabled[0], 0.0, "static outer false opens: {disabled:?}");
assert_eq!(disabled[1], 1.0, "static outer false pins: {disabled:?}");
let runtime_false = sample(1.0, -1.0);
assert_eq!(runtime_false[0], 1.0, "dynamic inner false retains topology: {runtime_false:?}");
assert_eq!(runtime_false[1], 0.0, "active topology is not pinned: {runtime_false:?}");
assert_eq!(runtime_false[2], 0.0, "dynamic inner false contributes zero: {runtime_false:?}");
assert_eq!(runtime_false[5], 1.0, "zero residual is still stamped on active branch: {runtime_false:?}");
let runtime_true = sample(1.0, 1.0);
assert_eq!(runtime_true[0], 1.0);
assert_eq!(runtime_true[2], 2.0, "dynamic inner true residual: {runtime_true:?}");
"#;
    run_generated_main("static then dynamic guard", &state, &stamp, &noise, body)
        .unwrap_or_else(|report| panic!("static/dynamic guard probe failed:\n{report}"));

    let (state, stamp, noise) = generated_parts(
        r#"
module dynamic_then_static(p, n);
    inout p, n;
    electrical p, n;
    parameter integer enabled = 0;
    analog begin
        if (V(p, n) > 0.0) begin
            if (enabled > 0)
                V(p, n) <+ 2.0;
        end
    end
endmodule
"#,
        "dynamic then static guard",
    );
    let body = r#"
let mut instance = device::state::Instance::new(&[0, 1]);
instance.set_branch_indices(&[2]);
instance.set_parameter("enabled", 0.0).unwrap();
instance.finalize_parameters().unwrap();
let voltages = [-1.0, 0.0, 0.0];
let ctx = runtime::GeneratedEvalContext { voltages: &voltages, temperature: 300.15 };
let mut sink = [0.0; 9];
let mut stamper = runtime::GeneratedStamper { sink: Some(&mut sink) };
instance.stamp(&ctx, &mut stamper);
assert_eq!(sink[0], 1.0, "first dynamic guard stops static peeling: {sink:?}");
assert_eq!(sink[1], 0.0, "dynamic-prefix topology remains active: {sink:?}");
assert_eq!(sink[2], 0.0, "untaken residual stays zero: {sink:?}");
"#;
    run_generated_main("dynamic then static guard", &state, &stamp, &noise, body)
        .unwrap_or_else(|report| panic!("dynamic/static guard probe failed:\n{report}"));

    let (state, stamp, noise) = generated_parts(
        r#"
module time_guard(p, n);
    inout p, n;
    electrical p, n;
    analog begin
        if ($abstime > 1.0)
            V(p, n) <+ 2.0;
    end
endmodule
"#,
        "time-dependent potential guard",
    );
    let body = r#"
fn sample(time: f64) -> [f64; 9] {
    let mut instance = device::state::Instance::new(&[0, 1]);
    instance.set_branch_indices(&[2]);
    instance.finalize_parameters().unwrap();
    instance.set_timepoint(time, 0.1, runtime::GeneratedDdtCoefficients::inactive());
    let voltages = [0.0; 3];
    let ctx = runtime::GeneratedEvalContext { voltages: &voltages, temperature: 300.15 };
    let mut sink = [0.0; 9];
    let mut stamper = runtime::GeneratedStamper { sink: Some(&mut sink) };
    instance.stamp(&ctx, &mut stamper);
    sink
}
let before = sample(0.0);
assert_eq!(before[0], 1.0, "time guard must not change topology: {before:?}");
assert_eq!(before[1], 0.0, "time-guarded branch stays active: {before:?}");
assert_eq!(before[2], 0.0, "before threshold residual is zero: {before:?}");
let after = sample(2.0);
assert_eq!(after[0], 1.0);
assert_eq!(after[2], 2.0, "after threshold residual: {after:?}");
"#;
    run_generated_main(
        "time dependent potential guard",
        &state,
        &stamp,
        &noise,
        body,
    )
    .unwrap_or_else(|report| panic!("time guard probe failed:\n{report}"));

    let (state, stamp, noise) = generated_parts(
        r#"
module stateful_call_guard(p, n);
    inout p, n;
    electrical p, n;
    parameter real selector = 1.0;
    analog begin
        if (ddt(selector) > 0.0)
            V(p, n) <+ 2.0;
    end
endmodule
"#,
        "stateful-call potential guard",
    );
    let body = r#"
let mut instance = device::state::Instance::new(&[0, 1]);
instance.set_branch_indices(&[2]);
instance.finalize_parameters().unwrap();
// A constant operand does not make ddt instance-static. At the first
// evaluation its value is zero, so the residual path is untaken while the
// physical potential topology must remain active.
let voltages = [0.0; 3];
let ctx = runtime::GeneratedEvalContext { voltages: &voltages, temperature: 300.15 };
let mut sink = [0.0; 9];
let mut stamper = runtime::GeneratedStamper { sink: Some(&mut sink) };
instance.stamp(&ctx, &mut stamper);
assert_eq!(sink[0], 1.0, "ddt guard must not change topology: {sink:?}");
assert_eq!(sink[1], 0.0, "stateful-call branch stays active: {sink:?}");
assert_eq!(sink[2], 0.0, "false ddt guard contributes zero: {sink:?}");
"#;
    run_generated_main(
        "stateful call potential guard",
        &state,
        &stamp,
        &noise,
        body,
    )
    .unwrap_or_else(|report| panic!("stateful-call guard probe failed:\n{report}"));
}

#[test]
fn generated_case_distinct_dynamic_guard_keeps_topology_active() {
    let (state, stamp, noise) = generated_parts(
        r#"
module case_distinct_guard(p, n);
    inout p, n;
    electrical p, n;
    parameter integer Enabled = 0;
    real enabled;
    analog begin
        enabled = V(p, n);
        if (enabled > 0.0)
            V(p, n) <+ 2.0;
    end
endmodule
"#,
        "case-distinct dynamic guard",
    );
    let body = r#"
fn sample(voltage: f64) -> [f64; 9] {
    let mut instance = device::state::Instance::new(&[0, 1]);
    instance.set_branch_indices(&[2]);
    instance.finalize_parameters().unwrap();
    let voltages = [voltage, 0.0, 0.0];
    let ctx = runtime::GeneratedEvalContext { voltages: &voltages, temperature: 300.15 };
    let mut sink = [0.0; 9];
    let mut stamper = runtime::GeneratedStamper { sink: Some(&mut sink) };
    instance.stamp(&ctx, &mut stamper);
    sink
}
let runtime_false = sample(-1.0);
assert_eq!(runtime_false[0], 1.0, "case-distinct dynamic guard retains topology: {runtime_false:?}");
assert_eq!(runtime_false[1], 0.0, "dynamic guard must not pin the branch: {runtime_false:?}");
assert_eq!(runtime_false[2], 0.0, "untaken contribution has zero residual: {runtime_false:?}");
let runtime_true = sample(1.0);
assert_eq!(runtime_true[0], 1.0, "topology remains fixed when guard becomes true: {runtime_true:?}");
assert_eq!(runtime_true[2], 2.0, "taken contribution residual: {runtime_true:?}");
"#;
    run_generated_main("case distinct dynamic guard", &state, &stamp, &noise, body)
        .unwrap_or_else(|report| panic!("case-distinct dynamic guard probe failed:\n{report}"));
}

#[test]
fn generated_guarded_reactive_potential_uses_group_orientation_and_clears_cache() {
    let (state, stamp, noise) = generated_parts(
        r#"
module guarded_flux(p, n);
    inout p, n;
    electrical p, n;
    parameter integer dc_enabled = 0;
    parameter integer reactive_enabled = 0;
    parameter real c = 3.0;
    analog begin
        if (dc_enabled > 0)
            V(p, n) <+ 0.0;
        if (reactive_enabled > 0)
            V(n, p) <+ ddt(c * V(p, n));
    end
endmodule
"#,
        "guarded reactive potential",
    );
    let body = r#"
fn evaluate(instance: &mut device::state::Instance, enabled: f64) -> ([f64; 9], [f64; 3]) {
    instance.set_parameter("dc_enabled", 0.0).unwrap();
    instance.set_parameter("reactive_enabled", enabled).unwrap();
    instance.finalize_parameters().unwrap();
    instance.set_timepoint(1.0, 0.5, runtime::GeneratedDdtCoefficients {
        active: true,
        derivative_scale: 2.0,
        previous_value_scale: 0.0,
        older_value_scale: 0.0,
        previous_derivative_scale: 0.0,
    });
    let voltages = [1.0, 0.0, 0.0, 99.0];
    let ctx = runtime::GeneratedEvalContext { voltages: &voltages, temperature: 300.15 };
    let mut real = [0.0; 9];
    let mut stamper = runtime::GeneratedStamper { sink: Some(&mut real) };
    instance.stamp(&ctx, &mut stamper);
    let mut reactive = [0.0; 3];
    let mut reactive_stamper = runtime::GeneratedReactiveStamper { sink: Some(&mut reactive) };
    instance.stamp_reactive(&ctx, &mut reactive_stamper);
    (real, reactive)
}
let mut instance = device::state::Instance::new(&[0, 1]);
instance.set_branch_indices(&[2, 3]);
let (active_real, active_reactive) = evaluate(&mut instance, 1.0);
assert_eq!(active_real[0], 1.0, "group couples once: {active_real:?}");
assert_eq!(active_real[1], 1.0, "duplicate reactive unknown pinned: {active_real:?}");
assert_eq!(active_real[3], -6.0, "reversed ddt derivative includes scale and sign: {active_real:?}");
assert_eq!(active_reactive[0], 1.0, "reactive row targets leader ordinal: {active_reactive:?}");
assert_eq!(active_reactive[1], -3.0, "reversed charge derivative sign: {active_reactive:?}");

let (inactive_real, inactive_reactive) = evaluate(&mut instance, 0.0);
assert_eq!(inactive_real[0], 0.0, "all static guards false open group: {inactive_real:?}");
assert_eq!(inactive_real[1], 2.0, "leader and duplicate pinned: {inactive_real:?}");
assert_eq!(inactive_reactive[1], 0.0, "inactive evaluation clears cached derivative: {inactive_reactive:?}");

let (_, active_again) = evaluate(&mut instance, 1.0);
assert_eq!(active_again[1], -3.0, "reactive cache restores after re-enable: {active_again:?}");
"#;
    run_generated_main("guarded reactive potential", &state, &stamp, &noise, body)
        .unwrap_or_else(|report| panic!("guarded reactive probe failed:\n{report}"));
}

#[test]
fn generated_static_dae_probe_excludes_dynamic_current_without_mutating_state() {
    let (state, stamp, noise) = generated_parts(
        r#"
module static_dae_current(p, n);
    inout p, n;
    electrical p, n;
    parameter real g = 2.0;
    parameter real c = 3.0;
    analog begin
        I(p, n) <+ g * V(p, n) + ddt(c * V(p, n));
    end
endmodule
"#,
        "static DAE current probe",
    );
    let body = r#"
fn instance() -> device::state::Instance {
    let mut instance = device::state::Instance::new(&[0, 1]);
    instance.finalize_parameters().unwrap();
    instance.set_timepoint(1.0, 0.5, runtime::GeneratedDdtCoefficients {
        active: true,
        derivative_scale: 2.0,
        previous_value_scale: 0.0,
        older_value_scale: 0.0,
        previous_derivative_scale: 0.0,
    });
    instance
}

let voltages = [1.0, 0.0];
let ctx = runtime::GeneratedEvalContext { voltages: &voltages, temperature: 300.15 };
assert!(device::state::Instance::ONE_STEP_DAE_SPLIT_SAFE);

let mut dynamic_instance = instance();
let mut dynamic_sink = [0.0; 10];
let mut dynamic_stamper = runtime::GeneratedStamper { sink: Some(&mut dynamic_sink) };
dynamic_instance.stamp(&ctx, &mut dynamic_stamper);
assert_eq!(dynamic_sink[9], 8.0, "static plus ddt residual: {dynamic_sink:?}");

let mut static_instance = instance();
let rollback_before = static_instance.capture_rollback_state();
runtime::set_dynamic_operators_enabled(false);
let mut static_sink = [0.0; 10];
let mut static_stamper = runtime::GeneratedStamper { sink: Some(&mut static_sink) };
static_instance.stamp(&ctx, &mut static_stamper);
runtime::set_dynamic_operators_enabled(true);
let rollback_after = static_instance.capture_rollback_state();
assert_eq!(static_sink[9], 2.0, "static probe must retain only F: {static_sink:?}");
assert_eq!(rollback_after, rollback_before, "static DAE probe mutated trial history");
"#;
    run_generated_main("static DAE current probe", &state, &stamp, &noise, body)
        .unwrap_or_else(|report| panic!("static DAE current probe failed:\n{report}"));
}

#[test]
fn generated_one_step_split_capability_rejects_dynamic_control_and_idt() {
    let (state, stamp, noise) = generated_parts(
        r#"
module ddt_control(p, n);
    inout p, n;
    electrical p, n;
    analog begin
        if (ddt(V(p, n)) > 0.0)
            I(p, n) <+ 1.0;
    end
endmodule
"#,
        "ddt control-flow capability",
    );
    run_generated_main(
        "ddt control-flow capability",
        &state,
        &stamp,
        &noise,
        "assert!(!device::state::Instance::ONE_STEP_DAE_SPLIT_SAFE);",
    )
    .unwrap_or_else(|report| panic!("ddt control capability probe failed:\n{report}"));

    let (state, stamp, noise) = generated_parts(
        r#"
module integrated_current(p, n);
    inout p, n;
    electrical p, n;
    analog I(p, n) <+ idt(V(p, n), 0.0);
endmodule
"#,
        "idt capability",
    );
    run_generated_main(
        "idt capability",
        &state,
        &stamp,
        &noise,
        "assert!(!device::state::Instance::ONE_STEP_DAE_SPLIT_SAFE);",
    )
    .unwrap_or_else(|report| panic!("idt capability probe failed:\n{report}"));
}

#[test]
fn generated_stages_follow_model_and_instance_parameter_scope() {
    let source = r#"
module scoped_stage(p, n);
    inout p, n;
    electrical p, n;
    parameter real model_gain = 2.0;
    (* type = "instance" *) parameter real width = 1.0e-6;
    (* type = "instance" *) parameter real area = width * width from [0.0:1.0e-6];
    real model_shape, geometry;
    analog begin
        model_shape = model_gain * model_gain;
        model_shape = model_shape * model_shape + 3.0 * model_gain;
        geometry = area;
        geometry = geometry * geometry * model_shape;
        I(p, n) <+ geometry * V(p, n);
    end
endmodule
"#;
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(source)
        .expect("front end");
    let device = canonical::generate_device(&artifact, &options()).expect("generation");
    let files: Vec<(&str, &str)> = device
        .files
        .iter()
        .map(|file| (file.relative_path.as_str(), file.contents.as_str()))
        .collect();
    let state = find(&files, "state.rs", "scoped stage");
    let stamp = find(&files, "stamp.rs", "scoped stage");
    let noise = find(&files, "noise.rs", "scoped stage");

    assert!(stamp.contains("fn canonical_model_stage"));
    assert!(stamp.contains("fn canonical_instance_stage"));
    assert!(stamp.contains("self.canonical_model_stage(ctx);"));
    assert!(stamp.contains("self.canonical_instance_stage(ctx);"));
    assert!(stamp.contains("static CANONICAL_MODEL_CACHE"));
    assert!(stamp.contains("canonical_model_cache_lookup"));
    assert!(stamp.contains("canonical_model_cache_intern"));
    assert!(state.contains("pub(crate) type CanonicalModelValues"));
    assert!(state.contains("Option<std::sync::Arc<CanonicalModelValues>>"));
    assert!(state.contains("pub(crate) const PARAMETER_MODEL_FLAGS: [bool; 3]"));
    assert!(state.contains("true, false, false"));
    assert!(state.contains("if PARAMETER_MODEL_FLAGS[index]"));
    assert!(state.contains("self.canonical_model_values = None;"));
    assert!(state.contains("let changed = self.multiplicity.to_bits()"));
    assert!(state.contains("self.canonical_instance_valid = false;"));

    if let Err(report) = compile("scoped stage", state, stamp, noise) {
        panic!("scoped stage: generated device does not compile:\n{report}");
    }
    if let Err(report) = run_shared_model_cache("scoped stage cache", state, stamp, noise) {
        panic!("scoped stage: shared model cache failed:\n{report}");
    }
}

#[test]
fn shared_noise_preprocessing_is_fresh_and_call_order_independent() {
    let source = fixtures()
        .into_iter()
        .find_map(|(name, source)| (name == "shared noise preprocessing").then_some(source))
        .expect("shared-noise fixture");
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(source)
        .expect("front end");
    let device = canonical::generate_device(&artifact, &options()).expect("generation");
    let files = device
        .files
        .iter()
        .map(|file| (file.relative_path.as_str(), file.contents.as_str()))
        .collect::<Vec<_>>();
    let state = find(&files, "state.rs", "shared noise preprocessing");
    let stamp = find(&files, "stamp.rs", "shared noise preprocessing");
    let noise = find(&files, "noise.rs", "shared noise preprocessing");

    assert!(stamp.contains("pub(super) fn canonical_model_preprocess"));
    assert!(noise.contains("let mut prepared = [0.0;"));
    assert!(noise.contains("canonical_model_preprocess("));
    assert!(!noise.contains("vec![0.0;"));
    if let Err(report) = run_noise_call_order("shared noise call order", state, stamp, noise) {
        panic!("shared preprocessing changed independent noise evaluation:\n{report}");
    }
}

#[test]
fn repeated_static_hot_guards_are_specialized_with_a_source_size_cap() {
    let source = repeated_structure_source();
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(&source)
        .expect("front end");
    let device = canonical::generate_device(&artifact, &options()).expect("generation");
    let files: Vec<(&str, &str)> = device
        .files
        .iter()
        .map(|file| (file.relative_path.as_str(), file.contents.as_str()))
        .collect();
    let state = find(&files, "state.rs", "repeated structure");
    let stamp = find(&files, "stamp.rs", "repeated structure");
    let noise = find(&files, "noise.rs", "repeated structure");

    assert!(
        stamp.contains("Bounded structural specialization: one dispatch replaces 3"),
        "three uses of one cached model condition should become one bounded dispatch; \
         stamp bytes={}, relevant lines:\n{}",
        stamp.len(),
        stamp
            .lines()
            .filter(|line| line.contains("if ") || line.contains("staged["))
            .take(40)
            .collect::<Vec<_>>()
            .join("\n")
    );
    assert!(
        stamp.contains("if staged[") && stamp.contains("canonical_structural_output_0"),
        "the specialized variants must rejoin through explicit scalar outputs"
    );
    if let Err(report) = compile("repeated structure", state, stamp, noise) {
        panic!("repeated structure: generated specialization does not compile:\n{report}");
    }
    if let Err(report) = run_structural_variants("repeated structure runtime", state, stamp, noise)
    {
        panic!("repeated structure: generated specialization changed behavior:\n{report}");
    }
}

fn repeated_structure_source() -> String {
    let mut coefficient_work = String::new();
    for index in 0..80 {
        coefficient_work.push_str(&format!(
            "        coefficient = coefficient * 1.0000001 + {}.0e-12;\n",
            index + 1
        ));
    }
    let mut guarded_work = String::new();
    for branch in 0..3 {
        guarded_work.push_str("        if (mode > 0.0) begin\n");
        for index in 0..16 {
            // Long runtime query leaves make the byte budget meaningfully
            // larger than the fixed indentation/scaffolding cost of the two
            // variants, without manufacturing thousands of CFG statements.
            let query = format!("specialization_{}_{}_{}", branch, index, "x".repeat(1024));
            guarded_work.push_str(&format!(
                "            current = current + $simparam(\"{query}\", 1.0e-9) * coefficient * V(p, n);\n"
            ));
        }
        guarded_work.push_str("        end\n");
    }
    format!(
        "module repeated_structure(p, n);\n\
         \x20   inout p, n;\n\
         \x20   electrical p, n;\n\
         \x20   parameter real mode = 1.0;\n\
         \x20   real coefficient, current;\n\
         \x20   analog begin\n\
         \x20       coefficient = mode + 1.0;\n\
         {coefficient_work}\
         \x20       current = 0.0;\n\
         {guarded_work}\
         \x20       I(p, n) <+ current;\n\
         \x20   end\n\
         endmodule\n"
    )
}

#[test]
fn structural_specialization_rejects_source_growth_over_two_percent() {
    let mut coefficient_work = String::new();
    for index in 0..80 {
        coefficient_work.push_str(&format!(
            "        coefficient = coefficient * 1.0000001 + {}.0e-12;\n",
            index + 1
        ));
    }
    let mut common_work = String::new();
    for index in 0..240 {
        common_work.push_str(&format!(
            "        current = current + coefficient * V(p, n) * {}.0e-9;\n",
            index + 1
        ));
    }
    let source = format!(
        "module rejected_structure(p, n);\n\
         \x20   inout p, n;\n\
         \x20   electrical p, n;\n\
         \x20   parameter real mode = 1.0;\n\
         \x20   real coefficient, current;\n\
         \x20   analog begin\n\
         \x20       coefficient = mode + 1.0;\n\
         {coefficient_work}\
         \x20       current = 0.0;\n\
         {common_work}\
         \x20       if (mode > 0.0) current = current + V(p, n) * 1.0e-12;\n\
         \x20       if (mode > 0.0) current = current + V(p, n) * 2.0e-12;\n\
         \x20       if (mode > 0.0) current = current + V(p, n) * 3.0e-12;\n\
         \x20       I(p, n) <+ current;\n\
         \x20   end\n\
         endmodule\n"
    );

    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(&source)
        .expect("front end");
    let stamp = canonical::generate_device(&artifact, &options())
        .expect("generation")
        .files
        .into_iter()
        .find(|file| file.relative_path == "stamp.rs")
        .expect("stamp.rs")
        .contents;

    assert!(
        stamp.contains("fn canonical_model_stage"),
        "the parameter prologue must be split so its repeated condition is cacheable"
    );
    assert!(
        !stamp.contains("Bounded structural specialization"),
        "duplicating the large common Newton path would violate the 2% source-size cap"
    );
    assert!(
        stamp
            .lines()
            .filter(|line| {
                let line = line.trim_start();
                line.starts_with("if ") && line.ends_with(" {") && !line.contains("staged[")
            })
            .count()
            >= 3,
        "the rejected candidate must retain its three ordinary branches"
    );
}

/// A model whose residual is a `ddt` gets a reactive stamp, and one without
/// gets an empty one rather than the conduction Jacobian by mistake.
#[test]
fn charge_storage_reaches_the_reactive_matrix_and_conduction_does_not() {
    let capacitor = r#"
module cap(p, n);
    inout p, n;
    electrical p, n;
    parameter real c = 1.0e-12;
    analog I(p, n) <+ ddt(c * V(p, n));
endmodule
"#;
    let resistor = r#"
module res(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 250.0;
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#;

    let stamp = stamp_of(capacitor, "cap");
    assert!(
        stamp.contains("stamp_current_reactive_indexed_dense_local"),
        "a capacitor stores charge and must write the reactive matrix:\n{stamp}"
    );

    let stamp = stamp_of(resistor, "res");
    assert!(
        !stamp.contains("stamp_current_reactive_indexed_dense_local"),
        "a resistor stores no charge, so its reactive stamp writes nothing:\n{stamp}"
    );
    assert!(
        stamp.contains("pub fn stamp_reactive"),
        "the reactive entry point exists whether or not it has work:\n{stamp}"
    );
}

/// A charge stored under a guard still reaches the reactive matrix.
///
/// `EPFL_HEMT_10a` writes `if (rth != 0) Pwr(t) <+ ddt(cth * Temp(t))`, and the
/// golden replay caught the whole capacitance entry going missing. A guarded
/// contribution arrives at its equation as a *merge* — the `ddt` from the arm
/// that ran, zero from the arm that did not — so a rule that matches the
/// residual against `Ddt` finds nothing and drops the charge silently. It is
/// silent in DC too: only AC and transient ever read the reactive matrix, which
/// is why a whole corpus of DC-shaped fixtures never noticed. Self-heating
/// blocks are guarded as a matter of course, so this is the common shape.
#[test]
fn a_guarded_charge_still_reaches_the_reactive_matrix() {
    let guarded = r#"
module guarded_cap(p, n);
    inout p, n;
    electrical p, n;
    parameter real c = 1.0e-12;
    parameter real enable = 1.0;
    analog begin
        if (enable != 0.0) begin
            I(p, n) <+ ddt(c * V(p, n));
        end
        I(p, n) <+ V(p, n) * 1.0e-6;
    end
endmodule
"#;
    let stamp = stamp_of(guarded, "guarded_cap");
    assert!(
        stamp.contains("stamp_current_reactive_indexed_dense_local"),
        "a guarded capacitor still stores charge on the path that runs:\n{stamp}"
    );
}

/// Linear arithmetic around a `ddt` is pushed inside it.
///
/// `EKV` writes `I(db) <+ TYPE * ddt_QD` and `I(d,b) <+ ddt(qjd)*TYPE*M`;
/// scaling a charge by a polarity or a multiplicity is idiomatic, and a rule
/// that matched only a bare `ddt` dropped every one of them. `k * ddt(q)`
/// stores `k * q` and `ddt(q1) + ddt(q2)` stores `q1 + q2`, so the operations
/// that commute with `d/dt` are followed into the charge and the product exists
/// nowhere until it is built.
#[test]
fn linear_arithmetic_around_a_ddt_still_stores_charge() {
    let scaled = r#"
module scaled_cap(p, n);
    inout p, n;
    electrical p, n;
    parameter real c = 1.0e-12;
    parameter real polarity = 1.0;
    analog I(p, n) <+ polarity * ddt(c * V(p, n));
endmodule
"#;
    let summed = r#"
module summed_cap(p, n);
    inout p, n;
    electrical p, n;
    parameter real c1 = 1.0e-12;
    parameter real c2 = 3.0e-12;
    analog I(p, n) <+ ddt(c1 * V(p, n)) + ddt(c2 * V(p, n));
endmodule
"#;
    for (source, module) in [(scaled, "scaled_cap"), (summed, "summed_cap")] {
        let stamp = stamp_of(source, module);
        assert!(
            stamp.contains("stamp_current_reactive_indexed_dense_local"),
            "{module} stores charge through linear arithmetic:\n{stamp}"
        );
    }
}

/// How far the canonical backend gets across the shipped models, and why it
/// stops where it does.
///
/// Numbers and reasons, not assertions. What it answers is the only question
/// that decides when this backend takes over from the tiers: which models it
/// carries end to end, and what each of the rest is waiting on.
#[test]
#[ignore = "generates every shipped model through the canonical backend; run with --ignored"]
fn the_whole_corpus_reports_what_the_canonical_backend_carries() {
    let root = model_root();
    let candidates =
        rspice_veriloga::rust_backend::discover_veriloga_sources(&root).expect("model tree");
    let mut carried = 0usize;
    let mut refused = 0usize;
    let mut bytes = 0usize;
    let mut stamp_bytes = 0usize;
    let mut noise_bytes = 0usize;
    let mut noise_fallbacks = 0usize;

    for candidate in &candidates {
        for module in &candidate.modules {
            let mut options = rspice_veriloga::CompilerOptions::default();
            options.include_paths.push(root.clone());
            options.defines = candidate.compile_profile.defines.clone();
            options.undefines = candidate.compile_profile.undefines.clone();
            let compiled = match VerilogACompiler::new(options)
                .compile_file_canonical_ir_with_metadata(&candidate.path, Some(module))
            {
                Ok(compiled) => compiled,
                Err(error) => {
                    refused += 1;
                    eprintln!("{module:>24}  front end: {error}");
                    continue;
                }
            };
            match std::panic::catch_unwind(|| {
                canonical::generate_device(&compiled.artifact, &RustTranspileOptions::default())
            }) {
                Ok(Ok(device)) => {
                    carried += 1;
                    let total: usize = device
                        .files
                        .iter()
                        .map(|file| file.contents.len())
                        .sum::<usize>();
                    let sized = |name: &str| {
                        device
                            .files
                            .iter()
                            .find(|file| file.relative_path == name)
                            .map_or(0, |file| file.contents.len())
                    };
                    let (stamp, noise) = (sized("stamp.rs"), sized("noise.rs"));
                    // The replaced generator replays statements through a
                    // workspace array; the canonical one emits a body. Which
                    // wrote this file is the difference between a model whose
                    // noise the CFG carries and one that fell back to keep its
                    // device, and a byte count alone does not say which.
                    let fell_back = device
                        .files
                        .iter()
                        .find(|file| file.relative_path == "noise.rs")
                        .is_some_and(|file| file.contents.contains("let mut w = [0.0;"));
                    if fell_back {
                        noise_fallbacks += 1;
                    }
                    bytes += total;
                    stamp_bytes += stamp;
                    noise_bytes += noise;
                    eprintln!(
                        "{module:>24}  {total:>10} bytes  ({stamp} stamp, {noise} noise{})",
                        if fell_back { ", fell back" } else { "" }
                    );
                }
                Ok(Err(error)) => {
                    refused += 1;
                    eprintln!("{module:>24}  refused: {error}");
                }
                Err(payload) => {
                    refused += 1;
                    eprintln!("{module:>24}  panicked: {}", panic_reason(&payload));
                }
            }
        }
    }
    eprintln!(
        "\n{carried} carried in {bytes} bytes, {refused} not \
         ({stamp_bytes} stamp, {noise_bytes} noise, \
         {noise_fallbacks} of them from the replaced generator)"
    );
}

fn panic_reason(payload: &Box<dyn std::any::Any + Send>) -> String {
    if let Some(message) = payload.downcast_ref::<String>() {
        return message.clone();
    }
    payload.downcast_ref::<&str>().map_or_else(
        || "no known payload".to_string(),
        |message| (*message).to_string(),
    )
}

fn model_root() -> PathBuf {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("..")
        .join("..")
        .join("models")
        .join("veriloga");
    assert!(root.exists(), "model tree missing: {}", root.display());
    root
}

fn stamp_of(source: &str, name: &str) -> String {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(source)
        .unwrap_or_else(|error| panic!("{name}: front end: {error}"));
    let device = canonical::generate_device(&artifact, &options())
        .unwrap_or_else(|error| panic!("{name}: generation: {error}"));
    device
        .files
        .iter()
        .find(|file| file.relative_path == "stamp.rs")
        .map(|file| file.contents.clone())
        .unwrap_or_else(|| panic!("{name}: no stamp.rs"))
}

fn options() -> RustTranspileOptions {
    RustTranspileOptions {
        runtime_path: "crate::runtime".to_string(),
        ..RustTranspileOptions::default()
    }
}

#[test]
fn transpiler_reports_hot_phases_and_exact_output_size() {
    let (name, source) = fixtures()[0];
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(source)
        .unwrap_or_else(|error| panic!("{name}: front end: {error}"));
    let generated = RustTranspiler::new(options())
        .transpile_measured(&artifact)
        .unwrap_or_else(|error| panic!("{name}: measured generation: {error}"));

    for phase in [
        PipelinePhase::CfgLowering,
        PipelinePhase::DerivativePreparation,
        PipelinePhase::Differentiation,
        PipelinePhase::DerivativeExtraction,
        PipelinePhase::NoisePlanning,
        PipelinePhase::StampPlanning,
        PipelinePhase::CfgOptimization,
        PipelinePhase::Scheduling,
        PipelinePhase::StampEmission,
        PipelinePhase::StateEmission,
        PipelinePhase::NoiseEmission,
        PipelinePhase::CheckpointFinalization,
    ] {
        assert!(
            generated.metrics.has_phase(phase),
            "missing structured metric for {phase}"
        );
    }
    let bytes = generated
        .output
        .files
        .iter()
        .map(|file| file.contents.len() as u64)
        .sum::<u64>();
    let lines = generated
        .output
        .files
        .iter()
        .map(|file| file.contents.lines().count() as u64)
        .sum::<u64>();
    assert_eq!(generated.metrics.generated_rust_bytes, bytes);
    assert_eq!(generated.metrics.generated_rust_lines, lines);
    assert!(generated.metrics.derivative_seed_count > 0);
    let derivative_values = generated
        .metrics
        .scalar_derivative_value_count
        .saturating_add(generated.metrics.packed_derivative_value_count);
    assert!(derivative_values > 0);
    assert!(generated.metrics.derivative_lane_entry_count >= derivative_values);
    assert!(generated.metrics.max_derivative_width > 0);
    assert!(generated.metrics.primal_cfg.value_count > 0);
    assert!(
        generated.metrics.differentiated_cfg.value_count
            >= generated.metrics.primal_cfg.value_count
    );
    assert!(
        generated.metrics.optimized_cfg.value_count
            <= generated.metrics.differentiated_cfg.value_count
    );
}

struct ImmediatePipelineCancellation;

impl PipelineControl for ImmediatePipelineCancellation {
    fn is_cancelled(&self) -> bool {
        true
    }
}

#[test]
fn transpiler_honors_cancellation_before_cfg_lowering() {
    let (name, source) = fixtures()[0];
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(source)
        .unwrap_or_else(|error| panic!("{name}: front end: {error}"));
    let error = RustTranspiler::new(options())
        .transpile_measured_with_control(&artifact, &ImmediatePipelineCancellation)
        .expect_err("immediate cancellation must prevent CFG lowering");

    assert_eq!(error.kind, RustBackendErrorKind::Cancelled);
    assert!(error.message.contains("cfg_lowering"), "{error}");
}

struct CancelInsideDifferentiation {
    preparation_complete: AtomicBool,
    polls_after_preparation: AtomicUsize,
}

impl PipelineControl for CancelInsideDifferentiation {
    fn is_cancelled(&self) -> bool {
        if !self.preparation_complete.load(Ordering::Relaxed) {
            return false;
        }
        // The first poll is the Differentiation phase boundary. Let the pass
        // enter, then cancel at its first internal cooperative checkpoint.
        self.polls_after_preparation.fetch_add(1, Ordering::Relaxed) >= 1
    }

    fn phase_completed(
        &self,
        timing: rspice_veriloga::PhaseTiming,
        _metrics: &rspice_veriloga::PipelineMetrics,
    ) {
        if timing.phase == PipelinePhase::DerivativePreparation {
            self.preparation_complete.store(true, Ordering::Relaxed);
        }
    }
}

#[test]
fn transpiler_polls_for_cancellation_inside_differentiation() {
    let (name, source) = fixtures()[0];
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(source)
        .unwrap_or_else(|error| panic!("{name}: front end: {error}"));
    let control = CancelInsideDifferentiation {
        preparation_complete: AtomicBool::new(false),
        polls_after_preparation: AtomicUsize::new(0),
    };
    let error = RustTranspiler::new(options())
        .transpile_measured_with_control(&artifact, &control)
        .expect_err("cancellation poll inside differentiation must stop lowering");

    assert_eq!(error.kind, RustBackendErrorKind::Cancelled);
    assert!(error.message.contains("differentiation"), "{error}");
}

struct CancelInsideStructuralSpecialization {
    scheduling_complete: AtomicBool,
    polls_after_scheduling: AtomicUsize,
}

impl PipelineControl for CancelInsideStructuralSpecialization {
    fn is_cancelled(&self) -> bool {
        if !self.scheduling_complete.load(Ordering::Relaxed) {
            return false;
        }
        // The first poll is the StampEmission boundary. Let it enter, then
        // cancel at the first poll in the variant's CFG optimization.
        self.polls_after_scheduling.fetch_add(1, Ordering::Relaxed) >= 1
    }

    fn phase_completed(
        &self,
        timing: rspice_veriloga::PhaseTiming,
        _metrics: &rspice_veriloga::PipelineMetrics,
    ) {
        if timing.phase == PipelinePhase::Scheduling {
            self.scheduling_complete.store(true, Ordering::Relaxed);
        }
    }
}

#[test]
fn structural_specialization_propagates_cancellation() {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(&repeated_structure_source())
        .expect("front end");
    let control = CancelInsideStructuralSpecialization {
        scheduling_complete: AtomicBool::new(false),
        polls_after_scheduling: AtomicUsize::new(0),
    };
    let error = RustTranspiler::new(options())
        .transpile_measured_with_control(&artifact, &control)
        .expect_err("the variant optimizer must honor cancellation");

    assert_eq!(error.kind, RustBackendErrorKind::Cancelled);
    assert!(error.message.contains("cfg_optimization"), "{error}");
    assert!(
        control.polls_after_scheduling.load(Ordering::Relaxed) >= 2,
        "the cancellation must occur after stamp emission began"
    );
}

fn find<'a>(files: &[(&'a str, &'a str)], name: &str, model: &str) -> &'a str {
    files
        .iter()
        .find(|(path, _)| *path == name)
        .map(|(_, contents)| *contents)
        .unwrap_or_else(|| panic!("{model}: no {name} was generated"))
}

fn generated_parts(source: &str, model: &str) -> (String, String, String) {
    let artifact = VerilogACompiler::default()
        .compile_canonical_ir(source)
        .unwrap_or_else(|error| panic!("{model}: front end failed: {error:?}"));
    let device = canonical::generate_device(&artifact, &options())
        .unwrap_or_else(|error| panic!("{model}: generation failed: {error}"));
    let file = |name: &str| {
        device
            .files
            .iter()
            .find(|file| file.relative_path == name)
            .map(|file| file.contents.clone())
            .unwrap_or_else(|| panic!("{model}: no {name} was generated"))
    };
    (file("state.rs"), file("stamp.rs"), file("noise.rs"))
}

fn run_generated_main(
    name: &str,
    state: &str,
    stamp: &str,
    noise: &str,
    main_body: &str,
) -> Result<(), String> {
    let root = scratch().join(name.replace(' ', "_"));
    std::fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    let main = root.join("main.rs");
    std::fs::write(
        &main,
        format!(
            "{RUNTIME_STUB}\npub mod device {{\n\
             pub mod state {{\n{}\n}}\n\
             pub mod stamp {{\n{}\n}}\n\
             pub mod noise {{\n{}\n}}\n}}\n\
             fn main() {{\n{}\n}}\n",
            indent(state),
            indent(stamp),
            indent(noise),
            indent(main_body),
        ),
    )
    .map_err(|error| error.to_string())?;
    let binary = root.join(format!("probe{}", std::env::consts::EXE_SUFFIX));
    let output = Command::new("rustc")
        .arg("--edition=2024")
        .arg("-A")
        .arg("warnings")
        .arg("-o")
        .arg(&binary)
        .arg(&main)
        .output()
        .map_err(|error| format!("could not run rustc: {error}"))?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into_owned());
    }
    let output = Command::new(&binary)
        .output()
        .map_err(|error| format!("could not run generated probe: {error}"))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).into_owned())
    }
}

fn compile(name: &str, state: &str, stamp: &str, noise: &str) -> Result<(), String> {
    let root = scratch().join(name.replace(' ', "_"));
    std::fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    let lib = root.join("lib.rs");
    std::fs::write(
        &lib,
        format!(
            "{RUNTIME_STUB}\npub mod device {{\n\
             pub mod state {{\n{}\n}}\n\
             pub mod stamp {{\n{}\n}}\n\
             pub mod noise {{\n{}\n}}\n}}\n",
            indent(state),
            indent(stamp),
            indent(noise)
        ),
    )
    .map_err(|error| error.to_string())?;

    let output = Command::new("rustc")
        .arg("--edition=2024")
        .arg("--crate-type=lib")
        .arg("-A")
        .arg("warnings")
        .arg("--out-dir")
        .arg(&root)
        .arg(&lib)
        .output()
        .map_err(|error| format!("could not run rustc: {error}"))?;
    if output.status.success() {
        return Ok(());
    }
    Err(String::from_utf8_lossy(&output.stderr).into_owned())
}

fn run_shared_model_cache(name: &str, state: &str, stamp: &str, noise: &str) -> Result<(), String> {
    let root = scratch().join(name.replace(' ', "_"));
    std::fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    let main = root.join("main.rs");
    std::fs::write(
        &main,
        format!(
            "{RUNTIME_STUB}\npub mod device {{\n\
             pub mod state {{\n{}\n\
             pub fn parameter(instance: &Instance, name: &str) -> f64 {{\n\
             \x20   let index = parameter_index_for_name(name).unwrap();\n\
             \x20   instance.params.values[index]\n\
             }}\n\
             pub fn instance_cache_valid(instance: &Instance) -> bool {{ instance.canonical_instance_valid }}\n\
}}\n\
             pub mod stamp {{\n{}\n}}\n\
             pub mod noise {{\n{}\n}}\n}}\n\
             fn main() {{\n\
             \x20   let mut first = device::state::Instance::new(&[0, 1]);\n\
             \x20   let mut second = device::state::Instance::new(&[0, 1]);\n\
             \x20   first.set_parameter(\"width\", 1.0e-6).unwrap();\n\
             \x20   second.set_parameter(\"width\", 2.0e-6).unwrap();\n\
             \x20   let voltages = [0.25, 0.0];\n\
             \x20   let ctx = runtime::GeneratedEvalContext {{ voltages: &voltages, temperature: 300.15 }};\n\
             \x20   let mut stamper = runtime::GeneratedStamper::default();\n\
             \x20   first.stamp(&ctx, &mut stamper);\n\
             \x20   second.stamp(&ctx, &mut stamper);\n\
             \x20   assert!(device::state::instance_cache_valid(&first));\n\
             \x20   let first_card = first.canonical_model_values.as_ref().unwrap();\n\
             \x20   let second_card = second.canonical_model_values.as_ref().unwrap();\n\
             \x20   assert!(std::sync::Arc::ptr_eq(first_card, second_card));\n\
             \x20   let before_values = first.params.values;\n\
             \x20   let before_given = first.param_given.clone();\n\
             \x20   assert!(first.set_parameter(\"width\", 2.0).is_err());\n\
             \x20   assert_eq!(first.params.values, before_values);\n\
             \x20   assert_eq!(first.param_given, before_given);\n\
             \x20   assert!(device::state::instance_cache_valid(&first));\n\
             \x20   first.set_parameter(\"width\", 3.0e-6).unwrap();\n\
             \x20   assert!((device::state::parameter(&first, \"area\") - 9.0e-12).abs() < 1.0e-24);\n\
             \x20   assert!(!device::state::instance_cache_valid(&first));\n\
             \x20   first.finalize_parameters().unwrap();\n\
             \x20   first.finalize_parameters().unwrap();\n\
             \x20   assert!((device::state::parameter(&first, \"area\") - 9.0e-12).abs() < 1.0e-24);\n\
             \x20   assert!(!device::state::instance_cache_valid(&first));\n\
             \x20   first.stamp(&ctx, &mut stamper);\n\
             \x20   assert!(device::state::instance_cache_valid(&first));\n\
             \x20   first.set_parameter(\"model_gain\", 4.0).unwrap();\n\
             \x20   first.stamp(&ctx, &mut stamper);\n\
             \x20   let changed_card = first.canonical_model_values.as_ref().unwrap();\n\
             \x20   assert!(!std::sync::Arc::ptr_eq(changed_card, second_card));\n\
             }}\n",
            indent(state),
            indent(stamp),
            indent(noise)
        ),
    )
    .map_err(|error| error.to_string())?;

    let binary = root.join(format!(
        "shared_model_cache{}",
        std::env::consts::EXE_SUFFIX
    ));
    let output = Command::new("rustc")
        .arg("--edition=2024")
        .arg("-A")
        .arg("warnings")
        .arg("-o")
        .arg(&binary)
        .arg(&main)
        .output()
        .map_err(|error| format!("could not run rustc: {error}"))?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into_owned());
    }
    let output = Command::new(&binary)
        .output()
        .map_err(|error| format!("could not run generated cache probe: {error}"))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).into_owned())
    }
}

fn run_dependent_parameter_defaults(
    name: &str,
    state: &str,
    stamp: &str,
    noise: &str,
) -> Result<(), String> {
    let root = scratch().join(name.replace(' ', "_"));
    std::fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    let main = root.join("main.rs");
    std::fs::write(
        &main,
        format!(
            "{RUNTIME_STUB}\npub mod device {{\n\
             pub mod state {{\n{}\n\
             pub fn parameter(instance: &Instance, name: &str) -> f64 {{\n\
             \x20   let index = parameter_index_for_name(name).unwrap();\n\
             \x20   read_parameter_slot(instance.params.as_ref(), index)\n\
             }}\n\
             pub fn parameter_given(instance: &Instance, name: &str) -> bool {{\n\
             \x20   let index = parameter_index_for_name(name).unwrap();\n\
             \x20   instance.param_given[index]\n\
             }}\n}}\n\
             pub mod stamp {{\n{}\n}}\n\
             pub mod noise {{\n{}\n}}\n}}\n\
             fn value(instance: &device::state::Instance, name: &str) -> f64 {{\n\
             \x20   device::state::parameter(instance, name)\n\
             }}\n\
             fn main() {{\n\
             \x20   let mut defaults = device::state::Instance::new(&[0, 1]);\n\
             \x20   defaults.finalize_parameters().unwrap();\n\
             \x20   defaults.validate_parameters().unwrap();\n\
             \x20   assert_eq!(value(&defaults, \"alias\"), 2.0);\n\
             \x20   assert_eq!(value(&defaults, \"chain\"), 6.0);\n\
             \x20   assert_eq!(value(&defaults, \"choice\"), 5.0);\n\
             \x20   assert_eq!(value(&defaults, \"given_sensitive\"), 5.0);\n\
             \x20   assert!(!device::state::parameter_given(&defaults, \"alias\"));\n\
             \x20   let mut overridden_base = device::state::Instance::new(&[0, 1]);\n\
             \x20   overridden_base.set_parameter(\"mode\", 1.0).unwrap();\n\
             \x20   overridden_base.set_parameter(\"base\", 4.0).unwrap();\n\
             \x20   overridden_base.finalize_parameters().unwrap();\n\
             \x20   overridden_base.validate_parameters().unwrap();\n\
             \x20   assert_eq!(value(&overridden_base, \"alias\"), 4.0);\n\
             \x20   assert_eq!(value(&overridden_base, \"chain\"), 12.0);\n\
             \x20   assert_eq!(value(&overridden_base, \"choice\"), 13.0);\n\
             \x20   assert_eq!(value(&overridden_base, \"given_sensitive\"), 113.0);\n\
             \x20   assert!(!device::state::parameter_given(&overridden_base, \"choice\"));\n\
             \x20   let mut explicit_dependent = device::state::Instance::new(&[0, 1]);\n\
             \x20   explicit_dependent.set_parameter(\"choice\", 7.0).unwrap();\n\
             \x20   explicit_dependent.set_parameter(\"base\", 4.0).unwrap();\n\
             \x20   explicit_dependent.set_parameter(\"mode\", 1.0).unwrap();\n\
             \x20   explicit_dependent.finalize_parameters().unwrap();\n\
             \x20   explicit_dependent.validate_parameters().unwrap();\n\
             \x20   assert_eq!(value(&explicit_dependent, \"chain\"), 12.0);\n\
             \x20   assert_eq!(value(&explicit_dependent, \"choice\"), 7.0);\n\
             \x20   assert_eq!(value(&explicit_dependent, \"given_sensitive\"), 107.0);\n\
             \x20   assert!(device::state::parameter_given(&explicit_dependent, \"choice\"));\n\
             \x20   let mut same_value_given = device::state::Instance::new(&[0, 1]);\n\
             \x20   same_value_given.set_parameter(\"base\", 2.0).unwrap();\n\
             \x20   same_value_given.finalize_parameters().unwrap();\n\
             \x20   same_value_given.validate_parameters().unwrap();\n\
             \x20   assert_eq!(value(&same_value_given, \"given_sensitive\"), 105.0);\n\
             \x20   let mut post_construction = device::state::Instance::new(&[0, 1]);\n\
             \x20   post_construction.set_parameter(\"base\", 3.0).unwrap();\n\
             \x20   assert_eq!(value(&post_construction, \"chain\"), 9.0);\n\
             \x20   post_construction.set_parameter(\"base\", 5.0).unwrap();\n\
             \x20   assert_eq!(value(&post_construction, \"chain\"), 15.0);\n\
             \x20   post_construction.finalize_parameters().unwrap();\n\
             \x20   post_construction.finalize_parameters().unwrap();\n\
             \x20   assert_eq!(value(&post_construction, \"chain\"), 15.0);\n\
             \x20   let before_values = post_construction.params.values;\n\
             \x20   let before_given = post_construction.param_given.clone();\n\
             \x20   let error = post_construction.set_parameter(\"bounded_source\", 6.0).unwrap_err();\n\
             \x20   assert!(error.contains(\"bounded_dependent\"), \"{{error}}\");\n\
             \x20   assert_eq!(post_construction.params.values, before_values);\n\
             \x20   assert_eq!(post_construction.param_given, before_given);\n\
             }}\n",
            indent(state),
            indent(stamp),
            indent(noise)
        ),
    )
    .map_err(|error| error.to_string())?;

    let binary = root.join(format!(
        "dependent_parameter_defaults{}",
        std::env::consts::EXE_SUFFIX
    ));
    let output = Command::new("rustc")
        .arg("--edition=2024")
        .arg("-A")
        .arg("warnings")
        .arg("-o")
        .arg(&binary)
        .arg(&main)
        .output()
        .map_err(|error| format!("could not run rustc: {error}"))?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into_owned());
    }
    let output = Command::new(&binary)
        .output()
        .map_err(|error| format!("could not run generated parameter probe: {error}"))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).into_owned())
    }
}

fn run_noise_call_order(name: &str, state: &str, stamp: &str, noise: &str) -> Result<(), String> {
    let root = scratch().join(name.replace(' ', "_"));
    std::fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    let main = root.join("main.rs");
    std::fs::write(
        &main,
        format!(
            "{RUNTIME_STUB}\npub mod device {{\n\
             pub mod state {{\n{}\n}}\n\
             pub mod stamp {{\n{}\n}}\n\
             pub mod noise {{\n{}\n}}\n}}\n\
             #[derive(Default)]\n\
             struct Capture(Vec<(usize, bool, u64, Option<u64>, Vec<u64>)>);\n\
             impl runtime::GeneratedNoiseVisitor for Capture {{\n\
             \x20   fn visit(&mut self, index: usize, value: runtime::GeneratedNoiseEvaluationRef<'_>) -> bool {{\n\
             \x20       self.0.push((index, value.active, value.psd.to_bits(), value.exponent.map(f64::to_bits), value.table_operands.iter().map(|value| value.to_bits()).collect()));\n\
             \x20       true\n\
             \x20   }}\n\
             }}\n\
             fn noise(instance: &device::state::Instance, ctx: &runtime::GeneratedEvalContext<'_>) -> Vec<(usize, bool, u64, Option<u64>, Vec<u64>)> {{\n\
             \x20   let mut capture = Capture::default();\n\
             \x20   instance.evaluate_noise_sources(ctx, &mut capture).unwrap();\n\
             \x20   capture.0\n\
             }}\n\
             fn stamp(instance: &mut device::state::Instance, ctx: &runtime::GeneratedEvalContext<'_>) {{\n\
             \x20   let mut stamper = runtime::GeneratedStamper::default();\n\
             \x20   instance.stamp(ctx, &mut stamper);\n\
             }}\n\
             fn main() {{\n\
             \x20   let bias_a = [0.25, 0.0];\n\
             \x20   let bias_b = [-0.4, 0.1];\n\
             \x20   let ctx_a = runtime::GeneratedEvalContext {{ voltages: &bias_a, temperature: 300.15 }};\n\
             \x20   let ctx_b = runtime::GeneratedEvalContext {{ voltages: &bias_b, temperature: 340.0 }};\n\
             \x20   let mut instance = device::state::Instance::new(&[0, 1]);\n\
             \x20   let fresh = noise(&instance, &ctx_a);\n\
             \x20   assert_eq!(noise(&instance, &ctx_a), fresh);\n\
             \x20   stamp(&mut instance, &ctx_a);\n\
             \x20   assert_eq!(noise(&instance, &ctx_a), fresh);\n\
             \x20   stamp(&mut instance, &ctx_b);\n\
             \x20   assert_eq!(noise(&instance, &ctx_a), fresh);\n\
             \x20   let saved = instance.capture_persistent_state();\n\
             \x20   let mut restored = device::state::Instance::new(&[0, 1]);\n\
             \x20   restored.restore_persistent_state(&saved).unwrap();\n\
             \x20   assert_eq!(noise(&restored, &ctx_a), fresh);\n\
             \x20   instance.set_parameter(\"width\", 2.0e-6).unwrap();\n\
             \x20   let changed = noise(&instance, &ctx_a);\n\
             \x20   let mut changed_fresh = device::state::Instance::new(&[0, 1]);\n\
             \x20   changed_fresh.set_parameter(\"width\", 2.0e-6).unwrap();\n\
             \x20   assert_eq!(changed, noise(&changed_fresh, &ctx_a));\n\
             \x20   assert_eq!(noise(&instance, &ctx_b), noise(&changed_fresh, &ctx_b));\n\
             }}\n",
            indent(state),
            indent(stamp),
            indent(noise)
        ),
    )
    .map_err(|error| error.to_string())?;

    let binary = root.join(format!("noise_call_order{}", std::env::consts::EXE_SUFFIX));
    let output = Command::new("rustc")
        .arg("--edition=2024")
        .arg("-O")
        .arg("-A")
        .arg("warnings")
        .arg("-o")
        .arg(&binary)
        .arg(&main)
        .output()
        .map_err(|error| format!("could not run rustc: {error}"))?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into_owned());
    }
    let output = Command::new(&binary)
        .output()
        .map_err(|error| format!("could not run generated noise probe: {error}"))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).into_owned())
    }
}

fn run_structural_variants(
    name: &str,
    state: &str,
    stamp: &str,
    noise: &str,
) -> Result<(), String> {
    let root = scratch().join(name.replace(' ', "_"));
    std::fs::create_dir_all(&root).map_err(|error| error.to_string())?;
    let main = root.join("main.rs");
    std::fs::write(
        &main,
        format!(
            "{RUNTIME_STUB}\npub mod device {{\n\
             pub mod state {{\n{}\n}}\n\
             pub mod stamp {{\n{}\n}}\n\
             pub mod noise {{\n{}\n}}\n}}\n\
             fn evaluate(instance: &mut device::state::Instance, mode: f64) -> f64 {{\n\
             \x20   instance.set_parameter(\"mode\", mode).unwrap();\n\
             \x20   let voltages = [0.25, 0.0];\n\
             \x20   let ctx = runtime::GeneratedEvalContext {{ voltages: &voltages, temperature: 300.15 }};\n\
             \x20   let mut sink = [0.0];\n\
             \x20   let mut stamper = runtime::GeneratedStamper {{ sink: Some(&mut sink) }};\n\
             \x20   instance.stamp(&ctx, &mut stamper);\n\
             \x20   sink[0]\n\
             }}\n\
             fn main() {{\n\
             \x20   let mut instance = device::state::Instance::new(&[0, 1]);\n\
             \x20   let enabled = evaluate(&mut instance, 1.0);\n\
             \x20   let disabled = evaluate(&mut instance, -1.0);\n\
             \x20   let enabled_again = evaluate(&mut instance, 1.0);\n\
             \x20   assert!(enabled.is_finite() && enabled > 0.0, \"{{enabled}}\");\n\
             \x20   assert_eq!(disabled.to_bits(), 0.0f64.to_bits());\n\
             \x20   assert_eq!(enabled_again.to_bits(), enabled.to_bits());\n\
             }}\n",
            indent(state),
            indent(stamp),
            indent(noise)
        ),
    )
    .map_err(|error| error.to_string())?;

    let binary = root.join(format!(
        "structural_variants{}",
        std::env::consts::EXE_SUFFIX
    ));
    let output = Command::new("rustc")
        .arg("--edition=2024")
        .arg("-O")
        .arg("-A")
        .arg("warnings")
        .arg("-o")
        .arg(&binary)
        .arg(&main)
        .output()
        .map_err(|error| format!("could not run rustc: {error}"))?;
    if !output.status.success() {
        return Err(String::from_utf8_lossy(&output.stderr).into_owned());
    }
    let output = Command::new(&binary)
        .output()
        .map_err(|error| format!("could not run generated specialization probe: {error}"))?;
    if output.status.success() {
        Ok(())
    } else {
        Err(String::from_utf8_lossy(&output.stderr).into_owned())
    }
}

/// One generated file, as a module beside its siblings.
///
/// The three become sibling modules of one crate rather than one flat module,
/// which is the shape the real tree has: `stamp.rs` and `noise.rs` both reach
/// `Instance` through `super::state`, and both import from the runtime under
/// their own names. Flattening them makes those imports collide over nothing.
///
/// Only the inner attributes come out, because a `#![..]` is legal at the top of
/// a module but not after the module's first item, and the generated file writes
/// one that the surrounding stub already covers.
fn indent(source: &str) -> String {
    source
        .lines()
        .filter(|line| !line.starts_with("#!["))
        .collect::<Vec<_>>()
        .join("\n")
}

fn scratch() -> PathBuf {
    let root = Path::new(env!("CARGO_TARGET_TMPDIR")).join("canonical-device");
    std::fs::create_dir_all(&root).expect("scratch directory");
    root
}

fn fixtures() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "resistor",
            r#"
module resistor(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 250.0;
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#,
        ),
        (
            "capacitor",
            r#"
module cap(p, n);
    inout p, n;
    electrical p, n;
    parameter real c = 1.0e-12;
    analog I(p, n) <+ ddt(c * V(p, n));
endmodule
"#,
        ),
        (
            "diode",
            r#"
module diode(a, c);
    inout a, c;
    electrical a, c;
    parameter real is = 1.0e-14;
    parameter real n = 1.0;
    analog begin
        I(a, c) <+ is * (exp(V(a, c) / (n * $vt)) - 1.0);
    end
endmodule
"#,
        ),
        // A guard, a temperature fold and a parameter prologue: the shape that
        // makes the invalidation split worth taking, so this exercises the
        // staged slots as well as the body.
        (
            "staged transistor",
            r#"
module staged(g, d, s);
    inout g, d, s;
    electrical g, d, s;
    parameter real width = 1.0e-6;
    parameter real vth0 = 0.4;
    parameter real tnom = 300.15;
    real geometry, vth, vov, ids;
    analog begin
        geometry = width * width * 1.0e12;
        if (geometry > 1.0e-3) begin
            geometry = geometry * 2.0;
        end
        vth = vth0 - 1.0e-3 * ($temperature - tnom);
        vov = V(g, s) - vth;
        if (vov > 0.0) begin
            ids = geometry * vov * vov;
        end else begin
            ids = 0.0;
        end
        I(d, s) <+ 1.0e-6 * ids;
    end
endmodule
"#,
        ),
        // The same prologue, plus a contribution that reads no unknown at all.
        // Its residual is instance-class, so the stamp reads it from a slot —
        // and the Newton body has no staged operand of its own, which is what
        // makes the slot array's binding independent of what the body reads.
        (
            "staged transistor with a leakage floor",
            r#"
module floored(g, d, s);
    inout g, d, s;
    electrical g, d, s;
    parameter real width = 1.0e-6;
    parameter real vth0 = 0.4;
    parameter real tnom = 300.15;
    parameter real ileak = 1.0e-12;
    real geometry, vth, vov, ids;
    analog begin
        geometry = width * width * 1.0e12;
        if (geometry > 1.0e-3) begin
            geometry = geometry * 2.0;
        end
        vth = vth0 - 1.0e-3 * ($temperature - tnom);
        vov = V(g, s) - vth;
        if (vov > 0.0) begin
            ids = geometry * vov * vov;
        end else begin
            ids = 0.0;
        end
        I(d, s) <+ 1.0e-6 * ids;
        I(d, s) <+ ileak * ileak;
    end
endmodule
"#,
        ),
        // A potential contribution, which stamps through a branch unknown
        // rather than a node pair.
        (
            "voltage source",
            r#"
module vsrc(p, n);
    inout p, n;
    electrical p, n;
    parameter real dc = 1.0;
    parameter real rs = 1.0e-3;
    analog V(p, n) <+ dc + rs * I(p, n);
endmodule
"#,
        ),
        // `idt`, which needs a history slot of its own and an initial condition
        // that is returned rather than integrated when there is no step.
        (
            "integrator",
            r#"
module integrator(p, n);
    inout p, n;
    electrical p, n;
    parameter real gain = 1.0e-6;
    parameter real start = 0.25;
    analog begin
        I(p, n) <+ gain * idt(V(p, n), start);
    end
endmodule
"#,
        ),
        // Noise, in the three shapes the descriptors distinguish. The table one
        // is here because its operands are the only magnitudes that reach the
        // visitor as a slice, and the guarded flicker because an inactive source
        // still has to be visited with the index its descriptor sits at.
        (
            "noisy resistor",
            r#"
module noisy_resistor(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 1000.0;
    analog begin
        I(p, n) <+ V(p, n) / r;
        I(p, n) <+ white_noise(4.0 * 1.380649e-23 * $temperature / r, "thermal");
    end
endmodule
"#,
        ),
        (
            "shared noise preprocessing",
            r#"
module shared_noise_preprocessing(p, n);
    inout p, n;
    electrical p, n;
    parameter real model_gain = 2.0;
    (* type = "instance" *) parameter real width = 1.0e-6;
    real a, b, c, d, e, geometry, thermal, current;
    analog begin
        a = model_gain * model_gain + 1.0;
        b = a * a + model_gain;
        c = sqrt(b + a);
        d = ln(c + b);
        e = exp(d * 0.01) + c;
        geometry = width * width * e;
        thermal = geometry * ($temperature + 273.15);
        current = thermal * V(p, n);
        I(p, n) <+ current;
        I(p, n) <+ white_noise(abs(thermal) * (1.0 + abs(V(p, n))), "shared");
    end
endmodule
"#,
        ),
        (
            "guarded flicker and table noise",
            r#"
module noisy_transistor(d, g, s);
    inout d, g, s;
    electrical d, g, s;
    parameter real kf = 1.0e-25;
    parameter real af = 1.2;
    parameter real beta = 1.0e-3;
    parameter real vth = 0.4;
    real ids;
    analog begin
        ids = 0.0;
        if (V(g, s) > vth) begin
            ids = beta * (V(g, s) - vth) * (V(g, s) - vth);
            I(d, s) <+ flicker_noise(kf * ids, af, "flicker");
        end
        I(d, s) <+ ids;
        I(d, s) <+ white_noise(2.0 * 1.602176634e-19 * ids, "shot");
        I(g, s) <+ noise_table({1.0, 1.0e-20, 1.0e6, 1.0e-22}, "gate");
    end
endmodule
"#,
        ),
        // Parameter arithmetic *inside* a guard, read by a bias-dependent
        // expression inside the same guard. That makes the split's export a
        // value defined in an `if` arm, which has no name after it in Rust —
        // and the export list is emitted at the end of the stage.
        //
        // Every fixture above is one function deep, so none of them reaches
        // this and the corpus shipped source that would not compile. The
        // arithmetic is deliberately several operations long: `worth_splitting`
        // only slices when a stage removes enough work, and a two-line
        // instance section would decline and prove nothing.
        (
            "a guarded stage export",
            r#"
module guarded_stage_export(p, n);
    inout p, n;
    electrical p, n;
    parameter real sel = 1.0;
    parameter real a = 2.0;
    parameter real b = 3.0;
    real t1, t2, t3, t4, t5, t6;
    analog begin
        if (sel > 0.5) begin
            t1 = a * b;
            t2 = sqrt(t1 + a);
            t3 = ln(t2 + b);
            t4 = exp(t3 * 0.1);
            t5 = t4 * t3 + t2;
            t6 = t5 / (t1 + 1.0);
            I(p, n) <+ t6 * V(p, n);
        end else begin
            I(p, n) <+ a * V(p, n);
        end
    end
endmodule
"#,
        ),
    ]
}

/// Only what the emitted code calls, with the signatures it calls them by.
const RUNTIME_STUB: &str = r#"
#![allow(dead_code, non_snake_case, unused_parens, unused_variables, unused_mut, unused_imports)]

pub mod runtime {
    pub type Value = f64;

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum GeneratedParameterOrigin {
        DeclaredScope,
        ModelCard,
        Instance,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum GeneratedVerilogAParameterScope {
        Model,
        Instance,
        Dual,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum GeneratedVerilogATerminalDirection {
        Input,
        Output,
        InOut,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct GeneratedVerilogATerminalDescriptor {
        pub name: &'static str,
        pub direction: GeneratedVerilogATerminalDirection,
        pub discipline: &'static str,
        pub current_parameter: &'static str,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct GeneratedVerilogAParameterBound {
        pub value: Value,
        pub exclusive: bool,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct GeneratedVerilogAParameterDescriptor {
        pub name: &'static str,
        pub aliases: &'static [&'static str],
        pub scope: GeneratedVerilogAParameterScope,
        pub is_integer: bool,
        pub default: Option<Value>,
        pub minimum: Option<GeneratedVerilogAParameterBound>,
        pub maximum: Option<GeneratedVerilogAParameterBound>,
        pub excluded_values: &'static [Value],
        pub has_dynamic_constraints: bool,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct GeneratedParameterAssignment<'a> {
        pub name: &'a str,
        pub value: Value,
        pub origin: GeneratedParameterOrigin,
    }

    impl<'a> GeneratedParameterAssignment<'a> {
        pub const fn new(
            name: &'a str,
            value: Value,
            origin: GeneratedParameterOrigin,
        ) -> Self {
            Self { name, value, origin }
        }

        pub const fn for_declared_scope(name: &'a str, value: Value) -> Self {
            Self::new(name, value, GeneratedParameterOrigin::DeclaredScope)
        }
    }

    #[derive(Clone, Copy)]
    pub struct Lanes<const N: usize>(pub [f64; N]);

    impl<const N: usize> core::ops::Add for Lanes<N> {
        type Output = Self;
        fn add(self, rhs: Self) -> Self {
            let mut out = self.0;
            let mut i = 0;
            while i < N {
                out[i] = self.0[i] + rhs.0[i];
                i += 1;
            }
            Self(out)
        }
    }

    impl<const N: usize> core::ops::Sub for Lanes<N> {
        type Output = Self;
        fn sub(self, rhs: Self) -> Self {
            let mut out = self.0;
            let mut i = 0;
            while i < N {
                out[i] = self.0[i] - rhs.0[i];
                i += 1;
            }
            Self(out)
        }
    }

    impl<const N: usize> core::ops::Mul<f64> for Lanes<N> {
        type Output = Self;
        fn mul(self, rhs: f64) -> Self {
            let mut out = self.0;
            let mut i = 0;
            while i < N {
                out[i] = self.0[i] * rhs;
                i += 1;
            }
            Self(out)
        }
    }

    impl<const N: usize> core::ops::Div<f64> for Lanes<N> {
        type Output = Self;
        fn div(self, rhs: f64) -> Self {
            let mut out = self.0;
            let mut i = 0;
            while i < N {
                out[i] = self.0[i] / rhs;
                i += 1;
            }
            Self(out)
        }
    }

    impl<const N: usize> core::ops::Index<usize> for Lanes<N> {
        type Output = f64;
        fn index(&self, index: usize) -> &f64 {
            &self.0[index]
        }
    }

    macro_rules! define_fixed_lanes {
        ($name:ident, $width:literal, [$($index:tt),+ $(,)?]) => {
            #[repr(transparent)]
            #[derive(Clone, Copy)]
            pub struct $name(pub [f64; $width]);

            impl core::ops::Add for $name {
                type Output = Self;
                fn add(self, rhs: Self) -> Self {
                    Self([$((self.0[$index] + rhs.0[$index])),+])
                }
            }
            impl core::ops::Sub for $name {
                type Output = Self;
                fn sub(self, rhs: Self) -> Self {
                    Self([$((self.0[$index] - rhs.0[$index])),+])
                }
            }
            impl core::ops::Mul<f64> for $name {
                type Output = Self;
                fn mul(self, rhs: f64) -> Self {
                    Self([$((self.0[$index] * rhs)),+])
                }
            }
            impl core::ops::Div<f64> for $name {
                type Output = Self;
                fn div(self, rhs: f64) -> Self {
                    Self([$((self.0[$index] / rhs)),+])
                }
            }
            impl core::ops::Index<usize> for $name {
                type Output = f64;
                fn index(&self, index: usize) -> &f64 {
                    &self.0[index]
                }
            }
        };
    }

    define_fixed_lanes!(L2, 2, [0, 1]);
    define_fixed_lanes!(L3, 3, [0, 1, 2]);
    define_fixed_lanes!(L4, 4, [0, 1, 2, 3]);
    define_fixed_lanes!(L5, 5, [0, 1, 2, 3, 4]);
    define_fixed_lanes!(L6, 6, [0, 1, 2, 3, 4, 5]);
    define_fixed_lanes!(L7, 7, [0, 1, 2, 3, 4, 5, 6]);
    define_fixed_lanes!(L8, 8, [0, 1, 2, 3, 4, 5, 6, 7]);
    define_fixed_lanes!(L9, 9, [0, 1, 2, 3, 4, 5, 6, 7, 8]);
    define_fixed_lanes!(L10, 10, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
    define_fixed_lanes!(L11, 11, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10]);
    define_fixed_lanes!(L12, 12, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11]);
    define_fixed_lanes!(L13, 13, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12]);
    define_fixed_lanes!(L14, 14, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]);
    define_fixed_lanes!(L15, 15, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14]);
    define_fixed_lanes!(L16, 16, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]);
    define_fixed_lanes!(L17, 17, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16]);
    define_fixed_lanes!(L18, 18, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17]);
    define_fixed_lanes!(L19, 19, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18]);
    define_fixed_lanes!(L20, 20, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19]);
    define_fixed_lanes!(L21, 21, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20]);
    define_fixed_lanes!(L22, 22, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21]);
    define_fixed_lanes!(L23, 23, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22]);
    define_fixed_lanes!(L24, 24, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23]);
    define_fixed_lanes!(L25, 25, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24]);
    define_fixed_lanes!(L26, 26, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25]);
    define_fixed_lanes!(L27, 27, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26]);
    define_fixed_lanes!(L28, 28, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27]);
    define_fixed_lanes!(L29, 29, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28]);
    define_fixed_lanes!(L30, 30, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29]);
    define_fixed_lanes!(L31, 31, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30]);
    define_fixed_lanes!(L32, 32, [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31]);

    pub fn install_generated_stage_values(
        destination: &mut [f64],
        values: &[f64],
        slots: &[u32],
    ) {
        assert_eq!(values.len(), slots.len());
        for (&value, &slot) in values.iter().zip(slots) {
            destination[slot as usize] = value;
        }
    }

    pub fn install_generated_parameter_aliases(
        values: &mut [f64],
        aliases: &[(u16, u16)],
        validate: fn(usize, f64) -> Result<(), String>,
    ) -> Result<(), String> {
        for &(destination, source) in aliases {
            let value = values[usize::from(source)];
            let destination = usize::from(destination);
            values[destination] = value;
            validate(destination, value)?;
        }
        Ok(())
    }

    pub fn find_generated_parameter_index(
        sorted_names: &[&str],
        parameter_indices: &[u16],
        name: &str,
    ) -> Option<usize> {
        assert_eq!(sorted_names.len(), parameter_indices.len());
        let mut left = 0usize;
        let mut right = sorted_names.len();
        while left < right {
            let middle = left + (right - left) / 2;
            if sorted_names[middle] < name {
                left = middle + 1;
            } else {
                right = middle;
            }
        }
        (sorted_names.get(left).copied() == Some(name))
            .then(|| usize::from(parameter_indices[left]))
    }

    pub fn rspice_limexp(x: f64) -> f64 {
        if x < 80.0 {
            x.exp()
        } else {
            (80.0f64).exp() * (x - 80.0 + 1.0)
        }
    }

    pub fn rspice_limited_exp(x: f64) -> f64 {
        if x > 80.0 {
            5.54062238439351e34 * (x - 80.0 + 1.0)
        } else if x < -80.0 {
            1.804851387e-35
        } else {
            x.exp()
        }
    }

    pub fn rspice_limited_exp_derivative(x: f64) -> f64 {
        if x > 80.0 {
            5.54062238439351e34
        } else if x < -80.0 {
            0.0
        } else {
            x.exp()
        }
    }

    pub fn rspice_eval_idt<const STATE_COUNT: usize>(
        current: &mut [f64; STATE_COUNT],
        previous: &mut [f64; STATE_COUNT],
        initialized: &mut [bool; STATE_COUNT],
        active: bool,
        step: f64,
        slot: usize,
        value: f64,
        ic: f64,
    ) -> f64 {
        let started_from = if initialized[slot] { previous[slot] } else { ic };
        let total = if active { started_from + value * step } else { ic };
        current[slot] = total;
        if !active {
            previous[slot] = total;
            initialized[slot] = true;
        }
        total
    }

    #[allow(clippy::too_many_arguments)]
    pub fn rspice_eval_ddt<const STATE_COUNT: usize>(
        current: &mut [f64; STATE_COUNT],
        previous: &mut [f64; STATE_COUNT],
        older: &mut [f64; STATE_COUNT],
        initialized: &mut [bool; STATE_COUNT],
        derivative_current: &mut [f64; STATE_COUNT],
        derivative_previous: &mut [f64; STATE_COUNT],
        active: bool,
        scale: f64,
        previous_value_scale: f64,
        older_value_scale: f64,
        previous_derivative_scale: f64,
        slot: usize,
        value: f64,
    ) -> f64 {
        let previous_value = if initialized[slot] { previous[slot] } else { value };
        let older_value = if initialized[slot] { older[slot] } else { value };
        current[slot] = value;
        if active {
            let result = value * scale
                - previous_value * previous_value_scale
                - older_value * older_value_scale
                - derivative_previous[slot] * previous_derivative_scale;
            derivative_current[slot] = result;
            result
        } else {
            previous[slot] = value;
            older[slot] = value;
            derivative_current[slot] = 0.0;
            derivative_previous[slot] = 0.0;
            initialized[slot] = true;
            0.0
        }
    }

    #[derive(Copy, Clone)]
    pub struct GeneratedParameterBound {
        pub value: f64,
        pub label: &'static str,
    }

    pub const GENERATED_PARAMETER_BOUND_NONE: u16 = 0;
    pub const GENERATED_PARAMETER_MIN_EXCLUSIVE_FLAG: u8 = 1;
    pub const GENERATED_PARAMETER_MAX_EXCLUSIVE_FLAG: u8 = 2;

    pub fn validate_generated_finite_parameter(name: &str, value: f64) -> Result<(), String> {
        if !value.is_finite() {
            return Err(format!("parameter '{}' must be finite, got {}", name, value));
        }
        Ok(())
    }

    pub fn validate_generated_parameter_bounds(
        name: &str,
        value: f64,
        flags: u8,
        min: Option<GeneratedParameterBound>,
        max: Option<GeneratedParameterBound>,
        excluded: &[GeneratedParameterBound],
    ) -> Result<(), String> {
        if let Some(min) = min {
            let invalid = if flags & GENERATED_PARAMETER_MIN_EXCLUSIVE_FLAG != 0 {
                value <= min.value
            } else {
                value < min.value
            };
            if invalid {
                let operator = if flags & GENERATED_PARAMETER_MIN_EXCLUSIVE_FLAG != 0 { ">" } else { ">=" };
                return Err(format!("parameter '{}' must be {} {}, got {}", name, operator, min.label, value));
            }
        }
        if let Some(max) = max {
            let invalid = if flags & GENERATED_PARAMETER_MAX_EXCLUSIVE_FLAG != 0 {
                value >= max.value
            } else {
                value > max.value
            };
            if invalid {
                let operator = if flags & GENERATED_PARAMETER_MAX_EXCLUSIVE_FLAG != 0 { "<" } else { "<=" };
                return Err(format!("parameter '{}' must be {} {}, got {}", name, operator, max.label, value));
            }
        }
        for excluded in excluded {
            if value == excluded.value {
                return Err(format!("parameter '{}' must not equal {}, got {}", name, excluded.label, value));
            }
        }
        Ok(())
    }

    pub fn resolve_generated_parameter_bound(
        pool: &[GeneratedParameterBound],
        encoded: u16,
    ) -> Option<GeneratedParameterBound> {
        if encoded == GENERATED_PARAMETER_BOUND_NONE {
            None
        } else {
            Some(*pool.get(usize::from(encoded - 1)).expect("generated parameter-bound index is outside its pool"))
        }
    }

    pub fn validate_generated_parameter_bound_indices(
        name: &str,
        value: f64,
        flags: u8,
        pool: &[GeneratedParameterBound],
        min: u16,
        max: u16,
        excluded: &[u16],
    ) -> Result<(), String> {
        validate_generated_parameter_bounds(
            name,
            value,
            flags,
            resolve_generated_parameter_bound(pool, min),
            resolve_generated_parameter_bound(pool, max),
            &[],
        )?;
        for &encoded in excluded {
            let excluded = resolve_generated_parameter_bound(pool, encoded)
                .expect("generated parameter exclusion uses the absence sentinel");
            if value == excluded.value {
                return Err(format!("parameter '{}' must not equal {}, got {}", name, excluded.label, value));
            }
        }
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub fn validate_generated_parameter(
        name: &str,
        value: f64,
        integer: bool,
        min: Option<(f64, &str)>,
        min_exclusive: bool,
        max: Option<(f64, &str)>,
        max_exclusive: bool,
        excluded: &[(f64, &str)],
    ) -> Result<(), String> {
        validate_generated_finite_parameter(name, value)?;
        if integer && value.fract() != 0.0 {
            return Err(format!("parameter '{}' must be an integer, got {}", name, value));
        }
        if integer && (value < i32::MIN as f64 || value > i32::MAX as f64) {
            return Err(format!("parameter '{}' must fit in a 32-bit signed integer, got {}", name, value));
        }
        if let Some((min, label)) = min {
            if (min_exclusive && value <= min) || (!min_exclusive && value < min) {
                let operator = if min_exclusive { ">" } else { ">=" };
                return Err(format!("parameter '{}' must be {} {}, got {}", name, operator, label, value));
            }
        }
        if let Some((max, label)) = max {
            if (max_exclusive && value >= max) || (!max_exclusive && value > max) {
                let operator = if max_exclusive { "<" } else { "<=" };
                return Err(format!("parameter '{}' must be {} {}, got {}", name, operator, label, value));
            }
        }
        for (excluded, label) in excluded {
            if value == *excluded {
                return Err(format!("parameter '{}' must not equal {}, got {}", name, label, value));
            }
        }
        Ok(())
    }

    pub fn boxed_zero_f64_array<const N: usize>() -> Box<[f64; N]> {
        let mut boxed = Box::<[f64; N]>::new_uninit();
        unsafe {
            std::ptr::write_bytes(boxed.as_mut_ptr(), 0, 1);
            boxed.assume_init()
        }
    }

    pub fn boxed_zero_bool_array<const N: usize>() -> Box<[bool; N]> {
        let mut boxed = Box::<[bool; N]>::new_uninit();
        unsafe {
            std::ptr::write_bytes(boxed.as_mut_ptr(), 0, 1);
            boxed.assume_init()
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct GeneratedDdtCoefficients {
        pub active: bool,
        pub derivative_scale: Value,
        pub previous_value_scale: Value,
        pub older_value_scale: Value,
        pub previous_derivative_scale: Value,
    }

    impl GeneratedDdtCoefficients {
        pub const fn inactive() -> Self {
            Self {
                active: false,
                derivative_scale: 0.0,
                previous_value_scale: 0.0,
                older_value_scale: 0.0,
                previous_derivative_scale: 0.0,
            }
        }
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct GeneratedVerilogAPersistentState {
        pub ddt_previous: Vec<Value>,
        pub ddt_older: Vec<Value>,
        pub ddt_derivative_previous: Vec<Value>,
        pub ddt_initialized: Vec<bool>,
        pub idt_previous: Vec<Value>,
        pub idt_initialized: Vec<bool>,
        pub limiter_anchor: Vec<Value>,
        pub limiter_initialized: Vec<bool>,
    }

    #[derive(Debug, Clone, Default, PartialEq)]
    pub struct GeneratedVerilogARollbackState {
        pub values: Vec<Value>,
        pub flags: Vec<bool>,
    }

    static DYNAMIC_OPERATORS_ENABLED: std::sync::atomic::AtomicBool =
        std::sync::atomic::AtomicBool::new(true);

    pub fn set_dynamic_operators_enabled(enabled: bool) {
        DYNAMIC_OPERATORS_ENABLED.store(enabled, std::sync::atomic::Ordering::SeqCst);
    }

    pub struct GeneratedEvalContext<'a> {
        pub voltages: &'a [Value],
        pub temperature: Value,
    }

    impl GeneratedEvalContext<'_> {
        pub fn node_voltage(&self, node: usize) -> Value {
            self.voltages.get(node).copied().unwrap_or(0.0)
        }
        pub fn branch_current(&self, branch: usize) -> Value {
            self.voltages.get(branch).copied().unwrap_or(0.0)
        }
        pub fn temperature(&self) -> Value {
            self.temperature
        }
        pub fn thermal_voltage(&self) -> Value {
            self.temperature * 8.617_333_262e-5
        }
        pub fn analysis(&self, query: &str) -> bool {
            query.eq_ignore_ascii_case("ac") && self.temperature == 123.0
        }
        pub fn simparam_or(&self, _name: &str, fallback: Value) -> Value {
            fallback
        }
        pub fn dynamic_operators_enabled(&self) -> bool {
            DYNAMIC_OPERATORS_ENABLED.load(std::sync::atomic::Ordering::SeqCst)
        }
    }

    #[derive(Default)]
    pub struct GeneratedStamper<'a> {
        pub sink: Option<&'a mut [Value]>,
    }

    impl GeneratedStamper<'_> {
        pub fn stamp_current_sparse_local<const NODE_COUNT: usize, const BRANCH_COUNT: usize>(
            &mut self,
            _pos: Option<usize>,
            _neg: Option<usize>,
            _value: Value,
            _node_indices: [usize; NODE_COUNT],
            _node_derivatives: [Value; NODE_COUNT],
            _branch_indices: [usize; BRANCH_COUNT],
            _branch_derivatives: [Value; BRANCH_COUNT],
            _scale: Value,
        ) {
            if let Some(sink) = self.sink.as_deref_mut()
                && let Some(first) = sink.first_mut()
            {
                *first += _value;
            }
            if let Some(sink) = self.sink.as_deref_mut()
                && let Some(value) = sink.get_mut(9)
            {
                *value += _value;
            }
        }

        pub fn stamp_potential_branch_local(
            &mut self,
            _pos: Option<usize>,
            _neg: Option<usize>,
            _branch: usize,
            _multiplicity: Value,
        ) {
            if let Some(sink) = self.sink.as_deref_mut() {
                if let Some(value) = sink.get_mut(0) { *value += 1.0; }
                if let Some(value) = sink.get_mut(7) { *value += _branch as f64 + 1.0; }
            }
        }

        pub fn stamp_inactive_potential_branch_local(&mut self, _branch: usize) {
            if let Some(sink) = self.sink.as_deref_mut() {
                if let Some(value) = sink.get_mut(1) { *value += 1.0; }
                if let Some(value) = sink.get_mut(8) { *value += _branch as f64 + 1.0; }
            }
        }

        pub fn stamp_potential_sparse_local<const NODE_COUNT: usize, const BRANCH_COUNT: usize>(
            &mut self,
            _branch: usize,
            _value: Value,
            _node_indices: [usize; NODE_COUNT],
            _node_derivatives: [Value; NODE_COUNT],
            _branch_indices: [usize; BRANCH_COUNT],
            _branch_derivatives: [Value; BRANCH_COUNT],
        ) {
            if let Some(sink) = self.sink.as_deref_mut() {
                if let Some(value) = sink.get_mut(2) { *value += _value; }
                if let Some(value) = sink.get_mut(3) {
                    *value += _node_derivatives.first().copied().unwrap_or(0.0);
                }
                if let Some(value) = sink.get_mut(4) {
                    *value += _branch_derivatives.iter().sum::<f64>();
                }
                if let Some(value) = sink.get_mut(5) { *value += _branch as f64 + 1.0; }
                if let Some(value) = sink.get_mut(6) {
                    *value += _branch_indices.iter().map(|index| *index as f64 + 1.0).sum::<f64>();
                }
            }
        }
    }

    #[derive(Default)]
    pub struct GeneratedReactiveStamper<'a> {
        pub sink: Option<&'a mut [Value]>,
    }

    impl GeneratedReactiveStamper<'_> {
        pub fn stamp_current_reactive_indexed_dense_local(
            &mut self,
            _pos: Option<usize>,
            _neg: Option<usize>,
            _nodes: &[usize],
            _node_derivatives: &[Value],
            _branches: &[usize],
            _branch_derivatives: &[Value],
            _scale: Value,
        ) {
        }

        pub fn stamp_potential_reactive_indexed_dense_local(
            &mut self,
            _branch: usize,
            _nodes: &[usize],
            _node_derivatives: &[Value],
            _branches: &[usize],
            _branch_derivatives: &[Value],
        ) {
            if let Some(sink) = self.sink.as_deref_mut() {
                if let Some(value) = sink.get_mut(0) { *value += _branch as f64 + 1.0; }
                if let Some(value) = sink.get_mut(1) {
                    *value += _node_derivatives.first().copied().unwrap_or(0.0);
                }
                if let Some(value) = sink.get_mut(2) {
                    *value += _branch_derivatives.iter().sum::<f64>();
                }
            }
        }
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub enum GeneratedNoiseKind {
        White,
        Flicker,
        Table,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct GeneratedNoiseEndpoint {
        pub local_node: Option<usize>,
        pub name: &'static str,
        pub is_internal: bool,
    }

    #[derive(Debug, Clone, Copy, PartialEq, Eq)]
    pub struct GeneratedNoiseDescriptor {
        pub mechanism: &'static str,
        pub label: Option<&'static str>,
        pub kind: GeneratedNoiseKind,
        pub equation: usize,
        pub is_current: bool,
        pub branch_ordinal: Option<usize>,
        pub pos: GeneratedNoiseEndpoint,
        pub neg: GeneratedNoiseEndpoint,
        pub table_len: usize,
        pub table_log_interp: bool,
    }

    #[derive(Debug, Clone, PartialEq)]
    pub struct GeneratedNoiseEvaluation {
        pub active: bool,
        pub psd: Value,
        pub exponent: Option<Value>,
        pub table_operands: Vec<Value>,
    }

    #[derive(Debug, Clone, Copy, PartialEq)]
    pub struct GeneratedNoiseEvaluationRef<'a> {
        pub active: bool,
        pub psd: Value,
        pub exponent: Option<Value>,
        pub table_operands: &'a [Value],
    }

    pub trait GeneratedNoiseVisitor {
        fn visit(&mut self, index: usize, evaluation: GeneratedNoiseEvaluationRef<'_>) -> bool;
    }

    #[derive(Debug, Clone, PartialEq)]
    pub enum GeneratedNoiseEvaluationError {
        SourceIndexOutOfRange { index: usize, count: usize },
        NonFinite { index: usize, quantity: &'static str, value: Value },
        NegativePower { index: usize, value: Value },
        InvalidMultiplicity { value: Value },
    }
}
"#;
