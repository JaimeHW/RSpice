//! aliasparam: alternate instance-facing parameter names.
//!
//! `aliasparam res = r;` makes `res` an alias of parameter `r`: setting
//! the alias at instantiation writes the target. The alias is not a
//! parameter itself (no entry in the parameter table, no default) and its
//! name must not collide with any other declaration.

use rspice_veriloga::device::VerilogADevice;
use rspice_veriloga::{CompilerOptions, VerilogACompiler};

const ALIASED_RESISTOR: &str = r#"
`include "disciplines.vams"
module ares(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 1.0 from (0:inf);
    aliasparam res = r;
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#;

fn compile(source: &str) -> rspice_veriloga::CompiledModel {
    VerilogACompiler::new(CompilerOptions::default())
        .compile(source)
        .expect("compilation failed")
}

#[cfg(feature = "native")]
struct DeviceFixture {
    model: rspice_veriloga::CompiledModel,
    artifact: rspice_veriloga::canonical_ir::CanonicalIrArtifact,
}

#[cfg(feature = "native")]
fn compile_device_fixture(source: &str) -> DeviceFixture {
    let compiler = VerilogACompiler::new(CompilerOptions::default());
    DeviceFixture {
        model: compiler.compile(source).expect("compilation failed"),
        artifact: compiler
            .compile_canonical_ir(source)
            .expect("canonical IR compilation failed"),
    }
}

#[cfg(not(feature = "native"))]
fn compile_device_fixture(source: &str) -> rspice_veriloga::CompiledModel {
    compile(source)
}

#[cfg(feature = "native")]
fn new_device(name: &str, fixture: &DeviceFixture, nodes: &[usize]) -> VerilogADevice {
    VerilogADevice::try_new_with_canonical_ir(name, fixture.model.clone(), &fixture.artifact, nodes)
        .expect("native device construction failed")
}

#[cfg(not(feature = "native"))]
fn new_device(
    name: &str,
    model: &rspice_veriloga::CompiledModel,
    nodes: &[usize],
) -> VerilogADevice {
    VerilogADevice::new(name, model.clone(), nodes)
}

#[test]
fn alias_is_not_a_parameter_but_resolves_to_its_target() {
    let model = compile(ALIASED_RESISTOR);

    // The alias adds no parameter table entry (no duplicate defaults)...
    assert_eq!(model.parameters.len(), 1);
    assert_eq!(model.parameters[0].name.as_str(), "r");
    // ...it is registered as an alternate name of the target
    assert_eq!(model.parameters[0].aliases, vec!["res"]);
}

#[test]
fn setting_the_alias_behaves_identically_to_setting_the_target() {
    let fixture = compile_device_fixture(ALIASED_RESISTOR);

    // Baseline: default r=1 conducts I = V/1
    let mut device = new_device("A0", &fixture, &[1, 0]);
    device.update_voltages(&[2.0]);
    assert!((device.evaluate()[0] - 2.0).abs() < 1e-12);

    // Setting the target directly: r=5 => I = V/5
    let mut direct = new_device("A1", &fixture, &[1, 0]);
    assert!(direct.set_parameter("r", 5.0));
    direct.resolve_parameter_defaults();
    direct.update_voltages(&[2.0]);
    let via_target = direct.evaluate()[0];

    // Setting the alias: res=5 must write r and produce the same current
    let mut aliased = new_device("A2", &fixture, &[1, 0]);
    assert!(aliased.set_parameter("res", 5.0));
    aliased.resolve_parameter_defaults();
    aliased.update_voltages(&[2.0]);
    let via_alias = aliased.evaluate()[0];

    assert!((via_target - 0.4).abs() < 1e-12);
    assert!((via_alias - via_target).abs() < 1e-15);
}

#[test]
fn alias_marks_the_target_as_given() {
    // $param_given(r) must observe an override applied through the alias
    let fixture = compile_device_fixture(
        r#"
`include "disciplines.vams"
module pgalias(p, n);
    inout p, n;
    electrical p, n;
    parameter real rknob = 1.0 from (0:inf);
    aliasparam knob = rknob;
    real geff;
    analog begin
        if ($param_given(rknob))
            geff = 1.0 / rknob;
        else
            geff = 0.25;
        I(p, n) <+ geff * V(p, n);
    end
endmodule
"#,
    );

    let mut device = new_device("G1", &fixture, &[1, 0]);
    assert!(device.set_parameter("knob", 1.0));
    device.update_voltages(&[2.0]);
    assert!((device.evaluate()[0] - 2.0).abs() < 1e-12);
}

#[test]
fn alias_resolves_case_insensitively_like_parameters() {
    // SPICE decks are case-insensitive; aliases follow the same rule
    let fixture = compile_device_fixture(ALIASED_RESISTOR);
    let mut device = new_device("C1", &fixture, &[1, 0]);
    assert!(device.set_parameter("RES", 4.0));
    device.resolve_parameter_defaults();
    device.update_voltages(&[2.0]);
    assert!((device.evaluate()[0] - 0.5).abs() < 1e-12);
}

#[test]
fn alias_colliding_with_a_parameter_name_is_an_error() {
    let err = VerilogACompiler::new(CompilerOptions::default())
        .compile(
            r#"
`include "disciplines.vams"
module clash(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 1.0 from (0:inf);
    parameter real res = 2.0 from (0:inf);
    aliasparam res = r;
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#,
        )
        .expect_err("alias reusing a parameter name must fail");

    let message = err.to_string();
    assert!(
        message.contains("res") && message.to_lowercase().contains("duplicate"),
        "expected duplicate-symbol error for 'res', got: {message}"
    );
}

#[test]
fn alias_of_an_undeclared_parameter_is_an_error() {
    let err = VerilogACompiler::new(CompilerOptions::default())
        .compile(
            r#"
`include "disciplines.vams"
module dangling(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 1.0 from (0:inf);
    aliasparam res = missing;
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#,
        )
        .expect_err("alias of an undeclared parameter must fail");

    let message = err.to_string();
    assert!(
        message.contains("missing"),
        "error must name the missing target, got: {message}"
    );
}

#[test]
fn referencing_the_alias_in_the_module_body_is_an_error() {
    // Per the LRM the alias is not a parameter: the body must use the
    // target, so a body reference fails instead of silently binding
    let err = VerilogACompiler::new(CompilerOptions::default())
        .compile(
            r#"
`include "disciplines.vams"
module bodyref(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 1.0 from (0:inf);
    aliasparam res = r;
    analog I(p, n) <+ V(p, n) / res;
endmodule
"#,
        )
        .expect_err("alias reference in the analog block must fail");

    assert!(
        err.to_string().contains("res"),
        "error must name the alias, got: {err}"
    );
}

#[test]
fn duplicate_alias_names_are_an_error() {
    let err = VerilogACompiler::new(CompilerOptions::default())
        .compile(
            r#"
`include "disciplines.vams"
module dupalias(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 1.0 from (0:inf);
    parameter real c = 1.0 from (0:inf);
    aliasparam knob = r;
    aliasparam knob = c;
    analog I(p, n) <+ V(p, n) / r;
endmodule
"#,
        )
        .expect_err("two aliases sharing a name must fail");

    let message = err.to_string();
    assert!(
        message.contains("knob"),
        "error must name the duplicated alias, got: {message}"
    );
}
