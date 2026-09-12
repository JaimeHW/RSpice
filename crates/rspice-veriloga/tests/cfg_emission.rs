//! The Rust emitter, checked by compiling and running what it emits.
//!
//! An emitter test that only inspects the generated text proves that the text
//! looks right, which is not the property anyone needs. This one writes the
//! source out, invokes `rustc` on it, runs the binary, and compares its numbers
//! against the reference interpreter at the same bias. That is the whole
//! contract — the generated device must agree with the interpreter — checked
//! directly rather than by proxy.
//!
//! It is `#[ignore]`d by default: it shells out to `rustc` and takes a second
//! or two per fixture. Run with `--ignored`.

use rspice_veriloga::VerilogACompiler;
use rspice_veriloga::canonical_ir::cfg_lower::CfgModel;
use rspice_veriloga::canonical_ir::{
    AdSeed, CanonicalIrArtifact, CfgEvalInputs, differentiate, evaluate_cfg, optimize_cfg,
};
use rspice_veriloga::rust_backend::emit::{EmitBindings, RUNTIME_PRELUDE, emit_body};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};
use std::process::Command;

#[test]
fn emitted_ddx_preserves_domain_errors_through_zero_derivatives_and_predicates() {
    for (expression, valid) in [
        ("ddx(V(p)%V(q),V(p))", 1.0),
        ("ddx(ddx(V(p)%V(q),V(p)),V(p))", 0.0),
        ("(ddx(V(p)%V(q),V(p))>0 ? 1 : 0)", 1.0),
    ] {
        let artifact = artifact(&format!(
            "module derivative_domain(p,q); inout p,q; electrical p,q;
            analog I(p)<+(V(q)<0 ? 3 : {expression}); endmodule"
        ));
        let cfg = CfgModel::from_hir(&artifact.hir, &artifact.mir).unwrap();
        let differentiated =
            differentiate(&cfg.function, &[AdSeed::NodePotential(0.into())]).unwrap();
        let (function, wanted) = optimize_cfg(&differentiated.function, &cfg.residuals);
        let (body, names) = emit_body(&function, &wanted, &EmitBindings::default()).unwrap();
        let guarded = format!(
            "let checked = std::panic::catch_unwind(|| {{ {body} {} }}).unwrap_or(-99.0f64);",
            names[0]
        );
        for (denominator, expected) in [(-1.0, 3.0), (0.0, -99.0), (2.0, valid)] {
            let mut bias = bias(&artifact);
            bias.node_potentials[0] = 5.0;
            bias.node_potentials[1] = denominator;
            let evaluated = evaluate_cfg(&function, &inputs(&bias));
            if denominator == 0.0 {
                assert!(evaluated.is_err(), "{expression}");
            } else {
                assert_eq!(evaluated.unwrap().value(wanted[0]).unwrap(), expected);
            }
            assert_eq!(
                compile_and_run(
                    &scratch("ddx-domain"),
                    "ddx_domain",
                    &program(&guarded, &["checked".into()], &bias),
                ),
                vec![expected],
                "{expression} at denominator {denominator}"
            );
        }
    }
}

#[test]
fn generated_integer_arithmetic_preserves_values_and_zero_tangents() {
    for (index, (operator, left, right, expected)) in [
        ("/", 5.0, 2.0, 2.0),
        ("/", -5.0, 2.0, -2.0),
        ("+", 2147483647.0, 1.0, -2147483648.0),
        ("-", -2147483648.0, 1.0, 2147483647.0),
        ("*", 2147483647.0, 2.0, -2.0),
        ("**", 2.0, 31.0, -2147483648.0),
        ("**", 2.0, -1.0, 0.0),
        ("**", -1.0, -3.0, -1.0),
        ("%", -5.0, 2.0, -1.0),
    ]
    .into_iter()
    .enumerate()
    {
        let artifact = artifact(&format!(
            "module typed(p,q); inout p,q; electrical p,q; integer a,b; analog begin a=V(p); b=V(q); I(p)<+(a {operator} b)+0.25*V(p); end endmodule"
        ));
        let cfg = CfgModel::from_hir(&artifact.hir, &artifact.mir).expect("integer lowering");
        let mut differentiated =
            differentiate(&cfg.function, &[AdSeed::NodePotential(0usize.into())]).unwrap();
        let residual = cfg.residuals[0];
        let derivative = differentiated.derivative(residual, 0).unwrap();
        let (function, wanted) = optimize_cfg(&differentiated.function, &[residual, derivative]);
        let (body, names) = emit_body(&function, &wanted, &EmitBindings::default()).unwrap();
        let mut bias = bias(&artifact);
        bias.node_potentials[0] = left;
        bias.node_potentials[1] = right;
        let actual = compile_and_run(
            &scratch("integer_arithmetic"),
            &format!("integer_{index}"),
            &program(&body, &names, &bias),
        );
        assert_eq!(
            actual,
            vec![expected + 0.25 * left, 0.25],
            "{left} {operator} {right}"
        );
        let interpreted = evaluate_cfg(&function, &inputs(&bias)).unwrap();
        assert_eq!(interpreted.value(wanted[0]).unwrap(), actual[0]);
        assert_eq!(interpreted.value(wanted[1]).unwrap(), actual[1]);
    }
}

#[test]
fn emitted_tasks_preserve_loop_execution_and_are_not_duplicated_by_derivatives() {
    use rspice_veriloga::canonical_ir::cfg::CfgValueKind;
    let artifact = artifact(
        r#"`include "disciplines.vams"
module tasks(p,n);
inout p,n; electrical p,n;
integer i;
analog begin
i=0;
while(i<3) begin $finish(i); i=i+1; end
I(p,n) <+ V(p,n);
end
endmodule"#,
    );
    let cfg = CfgModel::from_hir(&artifact.hir, &artifact.mir).unwrap();
    let effects = cfg
        .function
        .values
        .iter()
        .filter_map(|value| matches!(value.kind, CfgValueKind::AnalogTask(_)).then_some(value.id))
        .collect::<Vec<_>>();
    assert_eq!(effects.len(), 1);
    let differentiated = differentiate(&cfg.function, &[AdSeed::NodePotential(0.into())]).unwrap();
    assert_eq!(
        differentiated
            .function
            .values
            .iter()
            .filter(|value| matches!(value.kind, CfgValueKind::AnalogTask(_)))
            .count(),
        1
    );
    let (function, effects) = optimize_cfg(&differentiated.function, &effects);
    let (body, _) = emit_body(&function, &effects, &EmitBindings::default()).unwrap();
    let program = format!(
        r#"#![allow(unused_variables,unused_mut,unused_parens,dead_code)]
{RUNTIME_PRELUDE}
fn main() {{
let time=2.0f64;
let mut calls=Vec::new();
let mut analog_finish=|_site:u32,_time:f64,level:f64| calls.push(level);
{body}
for level in calls {{ println!("{{:x}}",f64::to_bits(level)); }}
}}
"#
    );
    assert_eq!(
        compile_and_run(&scratch("task-effects"), "task_effects", &program),
        vec![0.0, 1.0, 2.0]
    );
}

#[test]
#[ignore = "invokes rustc on the emitted source; run with --ignored"]
fn the_emitted_rust_reproduces_the_interpreter() {
    let directory = scratch("emission");
    for (name, source) in fixtures() {
        let artifact = artifact(source);
        let cfg = CfgModel::from_hir(&artifact.hir, &artifact.mir)
            .unwrap_or_else(|diagnostics| panic!("{name}: {diagnostics:?}"));

        // The whole pipeline, in the order the generator will run it.
        let lanes: Vec<AdSeed> = (0..artifact.mir.nodes.len())
            .map(|index| AdSeed::NodePotential(index.into()))
            .chain(
                (0..artifact.mir.branch_unknowns.len())
                    .map(|index| AdSeed::BranchUnknownFlow(index.into())),
            )
            .collect();
        let mut differentiated =
            differentiate(&cfg.function, &lanes).unwrap_or_else(|error| panic!("{name}: {error}"));

        let mut wanted = cfg.residuals.clone();
        for residual in &cfg.residuals {
            for lane in 0..lanes.len() {
                wanted.push(
                    differentiated
                        .derivative(*residual, lane)
                        .unwrap_or(*residual),
                );
            }
        }
        let (optimized, wanted) = optimize_cfg(&differentiated.function, &wanted);

        let bias = bias(&artifact);
        let expected = evaluate_cfg(&optimized, &inputs(&bias))
            .unwrap_or_else(|error| panic!("{name}: interpreter failed: {error}"));
        let expected: Vec<f64> = wanted
            .iter()
            .map(|value| expected.value(*value).expect("defined on every path"))
            .collect();

        let (body, names) =
            emit_body(&optimized, &wanted, &EmitBindings::default()).expect("emits");
        let program = program(&body, &names, &bias);
        let actual = compile_and_run(&directory, name, &program);

        assert_eq!(
            actual.len(),
            expected.len(),
            "{name}: emitted {} values, expected {}",
            actual.len(),
            expected.len()
        );
        for (index, (emitted, interpreted)) in actual.iter().zip(&expected).enumerate() {
            assert!(
                bit_identical(*emitted, *interpreted),
                "{name}: output {index} is {emitted} from the emitted code and \
                 {interpreted} from the interpreter"
            );
        }
    }
}

#[test]
fn one_lane_derivatives_are_scalar_and_executable() {
    let source = r#"
module one_lane(p);
    inout p;
    electrical p;
    analog I(p) <+ V(p) * V(p);
endmodule
"#;
    let artifact = artifact(source);
    let cfg = CfgModel::from_hir(&artifact.hir, &artifact.mir).expect("lowers");
    assert_eq!(artifact.mir.nodes.len(), 1);
    let lanes = [AdSeed::NodePotential(0usize.into())];
    let mut differentiated = differentiate(&cfg.function, &lanes).expect("differentiates");
    let mut wanted = cfg.residuals.clone();
    for residual in &cfg.residuals {
        wanted.extend(
            differentiated
                .derivative_row(*residual)
                .into_iter()
                .flatten(),
        );
    }
    let (optimized, wanted) = optimize_cfg(&differentiated.function, &wanted);
    let (body, names) = emit_body(&optimized, &wanted, &EmitBindings::default()).expect("emits");

    assert!(
        !body.contains("Lanes<1>") && !body.contains("Lanes(["),
        "one-lane derivatives must use plain f64:\n{body}"
    );

    let bias = bias(&artifact);
    let expected = evaluate_cfg(&optimized, &inputs(&bias)).expect("interprets");
    let expected: Vec<f64> = wanted
        .iter()
        .map(|value| expected.value(*value).expect("defined"))
        .collect();
    let actual = compile_and_run(
        &scratch("one_lane_scalar"),
        "one_lane_scalar",
        &program(&body, &names, &bias),
    );
    assert_eq!(actual.len(), expected.len());
    for (emitted, interpreted) in actual.iter().zip(expected) {
        assert!(bit_identical(*emitted, interpreted));
    }
}

#[test]
fn generated_custom_flow_values_and_derivatives_match_physics() {
    let artifact = artifact(
        r#"
nature TestPotential units="V"; access=TestU; abstol=1e-6; endnature
nature TestFlow units="A"; access=TestQ; abstol=1e-12; endnature
discipline testdisc potential TestPotential; flow TestFlow; enddiscipline
module probe(p,n,o);
inout p,n,o; testdisc p,n; electrical o;
branch(p,n) b;
analog begin TestU(b) <+ 2.0; I(o) <+ TestQ(b)*TestU(b); end
endmodule
"#,
    );
    let cfg = CfgModel::from_hir(&artifact.hir, &artifact.mir).expect("lowers");
    let lanes = [
        AdSeed::NodePotential(0usize.into()),
        AdSeed::BranchUnknownFlow(0usize.into()),
    ];
    let mut differentiated = differentiate(&cfg.function, &lanes).expect("differentiates");
    let output = cfg.residuals[1];
    let wanted = vec![
        output,
        differentiated
            .derivative(output, 0)
            .expect("potential derivative"),
        differentiated
            .derivative(output, 1)
            .expect("flow derivative"),
    ];
    let (optimized, wanted) = optimize_cfg(&differentiated.function, &wanted);
    let (body, names) = emit_body(&optimized, &wanted, &EmitBindings::default()).expect("emits");
    let mut bias = bias(&artifact);
    bias.node_potentials.fill(0.0);
    bias.node_potentials[0] = 2.0;
    bias.branch_unknown_flows[0] = 7.0;
    let actual = compile_and_run(
        &scratch("custom_flow"),
        "custom_flow",
        &program(&body, &names, &bias),
    );
    assert_eq!(actual, vec![14.0, 7.0, 2.0]);
}

#[test]
fn solver_branch_flow_ddx_emits_portably_and_matches_the_interpreter() {
    let artifact = artifact(
        r#"
module emitted_flow_ddx(p, n, ctrl, out);
    inout p, n, ctrl, out;
    electrical p, n, ctrl, out;
    branch (p, n) sense;
    analog begin
        V(sense) <+ 0.25;
        I(out, n) <+ ddx(I(sense) * I(sense) + V(ctrl, n) * I(sense), I(sense));
    end
endmodule
"#,
    );
    let cfg = CfgModel::from_hir(&artifact.hir, &artifact.mir).expect("lowers");
    let lanes: Vec<AdSeed> = (0..artifact.mir.nodes.len())
        .map(|index| AdSeed::NodePotential(index.into()))
        .chain(
            (0..artifact.mir.branch_unknowns.len())
                .map(|index| AdSeed::BranchUnknownFlow(index.into())),
        )
        .collect();
    let mut differentiated = differentiate(&cfg.function, &lanes).expect("differentiates");
    let residual = cfg.residuals[1];
    let row = differentiated.derivative_row(residual);
    let branch_lane = lanes.len() - 1;
    let mut wanted = vec![residual];
    wanted.push(row[2].expect("ddx has a ctrl-voltage Jacobian"));
    wanted.push(row[branch_lane].expect("ddx has a branch-flow Jacobian"));
    let (optimized, wanted) = optimize_cfg(&differentiated.function, &wanted);
    let (body, names) = emit_body(&optimized, &wanted, &EmitBindings::default()).expect("emits");

    let mut bias = bias(&artifact);
    bias.node_potentials[1] = -0.2;
    bias.node_potentials[2] = 0.9;
    bias.branch_unknown_flows[0] = 0.37;
    let expected = evaluate_cfg(&optimized, &inputs(&bias)).expect("interprets");
    let expected: Vec<f64> = wanted
        .iter()
        .map(|value| expected.value(*value).expect("defined"))
        .collect();
    let actual = compile_and_run(
        &scratch("flow_ddx"),
        "flow_ddx",
        &program(&body, &names, &bias),
    );
    assert_eq!(actual.len(), expected.len());
    for (emitted, interpreted) in actual.iter().zip(expected) {
        assert!(bit_identical(*emitted, interpreted));
    }
    assert!(
        (actual[1] - 1.0).abs() <= 1.0e-12,
        "ctrl Jacobian: {actual:?}"
    );
    assert!(
        (actual[2] - 2.0).abs() <= 1.0e-12,
        "flow Jacobian: {actual:?}"
    );

    let step = 1.0e-6;
    let mut plus_ctrl = bias.clone();
    plus_ctrl.node_potentials[2] += step;
    let plus_ctrl = compile_and_run(
        &scratch("flow_ddx_plus_ctrl"),
        "flow_ddx_plus_ctrl",
        &program(&body, &names, &plus_ctrl),
    );
    let mut minus_ctrl = bias.clone();
    minus_ctrl.node_potentials[2] -= step;
    let minus_ctrl = compile_and_run(
        &scratch("flow_ddx_minus_ctrl"),
        "flow_ddx_minus_ctrl",
        &program(&body, &names, &minus_ctrl),
    );
    let numeric_ctrl = (plus_ctrl[0] - minus_ctrl[0]) / (2.0 * step);
    assert!(
        (actual[1] - numeric_ctrl).abs() <= 1.0e-9,
        "portable ctrl stamp={} finite difference={numeric_ctrl}",
        actual[1]
    );

    let mut plus_flow = bias.clone();
    plus_flow.branch_unknown_flows[0] += step;
    let plus_flow = compile_and_run(
        &scratch("flow_ddx_plus_flow"),
        "flow_ddx_plus_flow",
        &program(&body, &names, &plus_flow),
    );
    let mut minus_flow = bias;
    minus_flow.branch_unknown_flows[0] -= step;
    let minus_flow = compile_and_run(
        &scratch("flow_ddx_minus_flow"),
        "flow_ddx_minus_flow",
        &program(&body, &names, &minus_flow),
    );
    let numeric_flow = (plus_flow[0] - minus_flow[0]) / (2.0 * step);
    assert!(
        (actual[2] - numeric_flow).abs() <= 1.0e-9,
        "portable flow stamp={} finite difference={numeric_flow}",
        actual[2]
    );
}

#[test]
fn single_value_straight_line_merges_are_typed_if_expressions() {
    let artifact = artifact(
        r#"
module expression_merge(p, n);
    inout p, n;
    electrical p, n;
    real x;
    analog begin
        if (V(p, n) > 0.0) begin
            x = exp(V(p, n));
        end else begin
            x = -V(p, n);
        end
        I(p, n) <+ x;
    end
endmodule
"#,
    );
    let cfg = CfgModel::from_hir(&artifact.hir, &artifact.mir).expect("lowers");
    let (optimized, wanted) = optimize_cfg(&cfg.function, &cfg.residuals);
    let (body, _) = emit_body(&optimized, &wanted, &EmitBindings::default()).expect("emits");

    assert!(
        body.lines().any(|line| line.contains("=if ")),
        "a single-value straight-line merge should be one typed if expression:\n{body}"
    );
}

#[test]
fn control_only_diamonds_are_not_emitted() {
    let artifact = artifact(
        r#"
module dead_control(p, n);
    inout p, n;
    electrical p, n;
    real unused;
    analog begin
        if (V(p, n) > 0.0) begin
            if (V(p, n) > 1.0)
                unused = exp(V(p, n));
            else
                unused = V(p, n) * V(p, n);
        end else begin
            unused = -V(p, n);
        end
        I(p, n) <+ V(p, n);
    end
endmodule
"#,
    );
    let cfg = CfgModel::from_hir(&artifact.hir, &artifact.mir).expect("lowers");
    let (optimized, wanted) = optimize_cfg(&cfg.function, &cfg.residuals);
    let (body, _) = emit_body(&optimized, &wanted, &EmitBindings::default()).expect("emits");

    assert!(
        !body
            .lines()
            .any(|line| line.trim_start().starts_with("if ")),
        "diamonds that cannot affect an output must not survive emission:\n{body}"
    );
    assert!(
        !body.contains(".exp()"),
        "operands used only by a dead condition or arm must not be emitted:\n{body}"
    );
}

#[test]
fn unconditional_loop_entry_initializes_carried_values_directly() {
    let artifact = artifact(
        r#"
module direct_loop_entry(p, n);
    inout p, n;
    electrical p, n;
    parameter integer steps = 4;
    real total;
    integer i;
    analog begin
        total = 3.0;
        i = 1;
        while (i < steps) begin
            total = total + V(p, n);
            i = i + 1;
        end
        I(p, n) <+ total;
    end
endmodule
"#,
    );
    let cfg = CfgModel::from_hir(&artifact.hir, &artifact.mir).expect("lowers");
    let (optimized, wanted) = optimize_cfg(&cfg.function, &cfg.residuals);
    let (body, _) = emit_body(&optimized, &wanted, &EmitBindings::default()).expect("emits");
    let before_loop = body.split("loop{").next().expect("loop prefix");

    for declaration in before_loop
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with("let mut "))
    {
        let name = declaration
            .strip_prefix("let mut ")
            .and_then(|line| line.split_once('='))
            .map(|(name, _)| name)
            .expect("mutable declaration");
        assert!(
            !before_loop
                .lines()
                .map(str::trim)
                .any(|line| line.starts_with(&format!("{name}="))),
            "an unconditional loop entry must initialize {name} directly:\n{body}"
        );
    }
}

/// The emitter and the interpreter evaluate the same operations in the same
/// order, so anything less than bit equality is a real difference in meaning
/// rather than a rounding artefact.
fn bit_identical(left: f64, right: f64) -> bool {
    left.to_bits() == right.to_bits() || (left.is_nan() && right.is_nan())
}

/// Wrap the emitted body in a program that prints every output.
fn program(body: &str, names: &[String], bias: &Bias) -> String {
    let list = |values: &[f64]| {
        values
            .iter()
            .map(|value| format!("{value:e}f64"))
            .collect::<Vec<_>>()
            .join(", ")
    };
    format!(
        r#"#![allow(unused_variables, unused_parens, unused_mut, clippy::all)]
{RUNTIME_PRELUDE}

fn analysis(_name: &str) -> f64 {{ 0.0 }}
fn simparam_required(name: &str) -> f64 {{ match name {{ "gmin" => 1e-12, "tnom" => 27.0, "simulatorVersion" => 1.0, "simulatorSubversion" => 0.0, _ => panic!("missing query") }} }}
fn has_simparam(name: &str) -> bool {{ matches!(name,"gmin"|"tnom"|"simulatorVersion"|"simulatorSubversion") }}
fn ddt(_operator: usize, _input: f64) -> f64 {{ 0.0 }}
fn ddt_derivative(_operator: usize, primal: f64, input: f64) -> f64 {{ assert!(primal.is_finite() && input.is_finite()); 0.0 }}
fn limit(_operator: usize, _proposed: f64, candidate: f64) -> f64 {{ candidate }}
fn limit_previous(_operator: usize, proposed: f64) -> f64 {{ proposed }}

fn main() {{
    let parameters: [f64; {parameter_count}] = [{parameters}];
    let parameter_given: [bool; {parameter_count}] = [{given}];
    let node_potentials: [f64; {node_count}] = [{nodes}];
    let branch_flows: [f64; {branch_count}] = [{branches}];
    let branch_unknown_flows: [f64; {unknown_count}] = [{unknowns}];
    let temperature: f64 = {temperature:e}f64;
    let thermal_voltage: f64 = {thermal_voltage:e}f64;
    let multiplicity: f64 = 1.0f64;
    let time: f64 = 0.0f64;

{body}
    let outputs: [f64; {output_count}] = [{outputs}];
    for value in outputs.iter() {{
        println!("{{:x}}", f64::to_bits(*value));
    }}
}}
"#,
        parameter_count = bias.parameters.len().max(1),
        parameters = if bias.parameters.is_empty() {
            "0.0f64".to_string()
        } else {
            list(&bias.parameters)
        },
        given = if bias.parameters.is_empty() {
            "false".to_string()
        } else {
            vec!["false"; bias.parameters.len()].join(", ")
        },
        node_count = bias.node_potentials.len().max(1),
        nodes = if bias.node_potentials.is_empty() {
            "0.0f64".to_string()
        } else {
            list(&bias.node_potentials)
        },
        branch_count = bias.branch_flows.len().max(1),
        branches = if bias.branch_flows.is_empty() {
            "0.0f64".to_string()
        } else {
            list(&bias.branch_flows)
        },
        unknown_count = bias.branch_unknown_flows.len().max(1),
        unknowns = if bias.branch_unknown_flows.is_empty() {
            "0.0f64".to_string()
        } else {
            list(&bias.branch_unknown_flows)
        },
        temperature = 300.15,
        thermal_voltage = 300.15 * 8.617_333_262e-5,
        output_count = names.len(),
        outputs = names.join(", "),
    )
}

fn compile_and_run(directory: &Path, name: &str, program: &str) -> Vec<f64> {
    let slug: String = name
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect();
    let source = directory.join(format!("{slug}.rs"));
    let binary = directory.join(format!("{slug}{}", std::env::consts::EXE_SUFFIX));
    std::fs::write(&source, program).expect("scratch directory is writable");

    let compile = Command::new("rustc")
        .arg("--edition=2021")
        .arg("-O")
        .arg("-o")
        .arg(&binary)
        .arg(&source)
        .output()
        .expect("rustc must be on PATH");
    assert!(
        compile.status.success(),
        "{name}: the emitted source did not compile\n{}\n--- source ---\n{program}",
        String::from_utf8_lossy(&compile.stderr)
    );

    let run = Command::new(&binary).output().expect("the program runs");
    assert!(
        run.status.success(),
        "{name}: the emitted program failed\n{}",
        String::from_utf8_lossy(&run.stderr)
    );
    String::from_utf8_lossy(&run.stdout)
        .lines()
        .map(|line| {
            f64::from_bits(u64::from_str_radix(line.trim(), 16).expect("a hexadecimal bit pattern"))
        })
        .collect()
}

fn scratch(name: &str) -> PathBuf {
    let directory = std::env::temp_dir().join("rspice-cfg-emission").join(name);
    std::fs::create_dir_all(&directory).expect("scratch directory is creatable");
    directory
}

#[derive(Clone)]
struct Bias {
    parameters: Vec<f64>,
    /// One flag per declared port, all connected — the CFG level's own
    /// convention, where `$port_connected` folds to a constant one. A shorter
    /// vector would read as *unconnected*, which these fixtures do not mean.
    port_connected: Vec<bool>,
    node_potentials: Vec<f64>,
    branch_flows: Vec<f64>,
    branch_unknown_flows: Vec<f64>,
}

fn bias(artifact: &CanonicalIrArtifact) -> Bias {
    Bias {
        parameters: artifact
            .mir
            .parameters
            .iter()
            .map(|parameter| parameter.default.unwrap_or(0.0))
            .collect(),
        port_connected: vec![true; artifact.hir.ports.len()],
        node_potentials: (0..artifact.mir.nodes.len())
            .map(|index| 0.41 - 0.13 * index as f64)
            .collect(),
        branch_flows: (0..artifact.mir.branches.len())
            .map(|index| 1.0e-4 * (index as f64 + 1.0))
            .collect(),
        branch_unknown_flows: (0..artifact.mir.branch_unknowns.len())
            .map(|index| 1.0e-4 * (index as f64 + 1.0))
            .collect(),
    }
}

fn inputs(bias: &Bias) -> CfgEvalInputs<f64> {
    CfgEvalInputs {
        parameters: bias.parameters.clone(),
        parameter_given: vec![false; bias.parameters.len()],
        port_connected: bias.port_connected.clone(),
        event_state: Vec::new(),
        event_controls: HashMap::new(),
        node_potentials: bias.node_potentials.clone(),
        branch_flows: bias.branch_flows.clone(),
        branch_unknown_flows: bias.branch_unknown_flows.clone(),
        temperature: 300.15,
        thermal_voltage: 300.15 * 8.617_333_262e-5,
        multiplicity: 1.0,
        time: 0.0,
        analyses: HashSet::new(),
        simparams: Default::default(),
        ddt: 0.0,
        ddt_scale: 0.0,
        idt: 0.0,
        idt_scale: 0.0,
        integral_derivatives: Default::default(),
        staged: Vec::new(),
    }
}

fn artifact(source: &str) -> CanonicalIrArtifact {
    VerilogACompiler::default()
        .compile_canonical_ir(source)
        .expect("fixture must compile to canonical IR")
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
        // Both current read-back forms, so the emitted code is checked to read
        // the same accumulator the interpreter does rather than a stale one.
        (
            "probed contributions",
            r#"
module probed(d, s, b);
    inout d, s, b;
    electrical d, s, b;
    electrical di;
    parameter real beta = 1.0e-3;
    parameter real rd = 20.0;
    real ids, terminal;
    analog begin
        I(d, di) <+ V(d, di) / rd;
        I(di, s) <+ beta * V(di, s) * V(di, s);
        ids = I(di, s);
        terminal = I(<d>);
        I(b, s) <+ 1.0e-6 * ids + 1.0e-6 * terminal;
    end
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
    analog I(a, c) <+ is * (exp(V(a, c) / 0.02585) - 1.0);
endmodule
"#,
        ),
        // The intrinsics the level gained last. Emission is where a name that
        // lowers can still be wrong — `atan2`'s operand order is the whole
        // difference between an angle and its complement, and no shape check
        // upstream of here would see it.
        (
            "inverse and two-argument intrinsics",
            r#"
module intrinsics(p, n);
    inout p, n;
    electrical p, n;
    parameter real g = 1.0e-3;
    analog begin
        I(p, n) <+ g * asin(0.31 * V(p, n))
                 + g * acos(0.31 * V(p, n))
                 + g * atanh(0.31 * V(p, n))
                 + g * acosh(1.5 + V(p, n) * V(p, n))
                 + g * log10(2.0 + V(p, n))
                 + g * hypot(V(p, n), 0.25 + V(p, n))
                 + g * atan2(V(p, n), 0.75 - V(p, n));
    end
endmodule
"#,
        ),
        (
            "guarded",
            r#"
module guarded(g, d, s);
    inout g, d, s;
    electrical g, d, s;
    parameter real beta = 1.0e-3;
    parameter real vth = 0.4;
    real vov;
    analog begin
        vov = V(g, s) - vth;
        if (vov > 0.0) begin
            I(d, s) <+ beta * vov * vov * (1.0 + 0.02 * V(d, s));
        end else begin
            I(d, s) <+ 1.0e-12 * V(d, s);
        end
    end
endmodule
"#,
        ),
        (
            "nested conditionals",
            r#"
module nested(p, n);
    inout p, n;
    electrical p, n;
    parameter real mode = 2.0;
    real g;
    analog begin
        g = 1.0e-3;
        if (mode > 0.5) begin
            if (mode > 1.5) begin
                g = 3.0e-3 * exp(V(p, n));
            end else begin
                g = 2.0e-3;
            end
        end
        I(p, n) <+ g * V(p, n);
    end
endmodule
"#,
        ),
        (
            "ternary",
            r#"
module ternary(p, n);
    inout p, n;
    electrical p, n;
    parameter real is = 1.0e-14;
    analog I(p, n) <+ (V(p, n) > 0.0)
        ? is * (exp(V(p, n) / 0.02585) - 1.0)
        : is * V(p, n) / 0.02585;
endmodule
"#,
        ),
        (
            "loop",
            r#"
module summed(p, n);
    inout p, n;
    electrical p, n;
    parameter integer steps = 4;
    parameter real k = 1.0e-3;
    real total;
    integer i;
    analog begin
        total = 0.0;
        i = 0;
        while (i < steps) begin
            total = total + k * V(p, n);
            i = i + 1;
        end
        I(p, n) <+ total;
    end
endmodule
"#,
        ),
        (
            "internal node",
            r#"
module series(p, n);
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
"#,
        ),
        (
            "potential contribution",
            r#"
module source(p, n);
    inout p, n;
    electrical p, n;
    parameter real r = 50.0;
    analog V(p, n) <+ r * I(p, n);
endmodule
"#,
        ),
    ]
}
