//! Noise sources lowered into the body's control-flow graph.
//!
//! A noise power is a function of the operating point, so it is work the body
//! already does. What has to be checked is that lowering it there does not
//! change it: the amplitude a source was scaled by is squared into its power,
//! and a source that control flow never reaches reads back inactive rather than
//! reading back some other path's value.
//!
//! The magnitudes here are hand-computed from the fixture. Checking them against
//! another pass of the same compiler would only show the two agree, which is
//! exactly what the last generator did while being half the output tree.

use rspice_veriloga::canonical_ir::cfg_lower::{CfgModel, CfgNoiseSource};
use rspice_veriloga::canonical_ir::{
    CanonicalIrArtifact, CanonicalNoiseSourceKind, CfgEvalInputs, ValueId, evaluate_cfg,
};
use rspice_veriloga::rust_backend::discover_veriloga_sources;
use rspice_veriloga::{CompilerOptions, VerilogACompiler};
use std::collections::{HashMap, HashSet};
use std::path::{Path, PathBuf};

/// Relative agreement demanded of a hand-computed power.
const TOLERANCE: f64 = 1.0e-12;

const TEMPERATURE: f64 = 300.15;

fn artifact(source: &str) -> CanonicalIrArtifact {
    VerilogACompiler::default()
        .compile_canonical_ir(source)
        .expect("fixture must compile to canonical IR")
}

fn lower(source: &str) -> (CanonicalIrArtifact, CfgModel) {
    let artifact = artifact(source);
    let model = CfgModel::from_hir(&artifact.hir, &artifact.mir)
        .unwrap_or_else(|diagnostics| panic!("fixture must lower: {diagnostics:?}"));
    (artifact, model)
}

/// Evaluate the lowered body at a fixed, asymmetric bias.
///
/// Asymmetric because a symmetric one makes `V(p,n)` and `V(n,p)` agree, and a
/// sign error in the amplitude would then survive.
fn evaluate(artifact: &CanonicalIrArtifact, model: &CfgModel) -> impl Fn(ValueId) -> f64 {
    let inputs = CfgEvalInputs {
        parameters: artifact
            .mir
            .parameters
            .iter()
            .map(|parameter| parameter.default.unwrap_or(0.0))
            .collect(),
        parameter_given: vec![false; artifact.mir.parameters.len()],
        event_state: Vec::new(),
        event_controls: HashMap::new(),
        node_potentials: (0..artifact.mir.nodes.len())
            .map(|index| 0.41 - 0.13 * index as f64)
            .collect(),
        branch_flows: (0..artifact.mir.branches.len())
            .map(|index| 1.0e-4 * (index as f64 + 1.0))
            .collect(),
        branch_unknown_flows: (0..artifact.mir.branch_unknowns.len())
            .map(|index| 1.0e-4 * (index as f64 + 1.0))
            .collect(),
        temperature: TEMPERATURE,
        thermal_voltage: TEMPERATURE * 8.617_333_262e-5,
        multiplicity: 1.0,
        time: 0.0,
        analyses: HashSet::new(),
        simparams: HashMap::new(),
        ddt: 0.0,
        ddt_scale: 0.0,
        idt: 0.0,
        idt_scale: 0.0,
        staged: Vec::new(),
    };
    let snapshot = evaluate_cfg(&model.function, &inputs)
        .unwrap_or_else(|error| panic!("fixture must evaluate: {error}"));
    move |value| {
        snapshot
            .value(value)
            .unwrap_or_else(|| panic!("{value} has no value at this bias"))
    }
}

#[track_caller]
fn close(actual: f64, expected: f64, what: &str) {
    let scale = expected.abs().max(actual.abs()).max(f64::MIN_POSITIVE);
    assert!(
        (actual - expected).abs() / scale <= TOLERANCE,
        "{what}: got {actual}, expected {expected}"
    );
}

/// The rule the whole lowering turns on: a scaled noise source carries the
/// square of its scale, because power goes as amplitude squared.
#[test]
fn a_scaled_source_carries_the_square_of_its_scale() {
    let (artifact, model) = lower(
        r#"
module scaled(p, n);
    inout p, n;
    electrical p, n;
    parameter real gain = 3.0;
    parameter real power = 7.0;
    analog begin
        I(p, n) <+ gain * white_noise(power);
    end
endmodule
"#,
    );

    assert_eq!(model.noise.len(), 1, "one source was written");
    let source = &model.noise[0];
    assert_eq!(source.kind, CanonicalNoiseSourceKind::White);
    assert_eq!(source.ordinal, 0);
    assert!(source.exponent.is_none());
    assert!(source.table.is_empty());

    let value = evaluate(&artifact, &model);
    close(value(source.active), 1.0, "active");
    close(value(source.psd), 3.0 * 3.0 * 7.0, "psd");
}

/// Division scales the amplitude too, and it is the whole amplitude that is
/// squared. Every way of getting this wrong lands somewhere else: not dividing
/// at all gives 405, dividing after squaring gives 135.
#[test]
fn a_divided_source_divides_the_amplitude_before_squaring_it() {
    let (artifact, model) = lower(
        r#"
module divided(p, n);
    inout p, n;
    electrical p, n;
    parameter real power = 5.0;
    analog begin
        I(p, n) <+ 9.0 * white_noise(power) / 3.0;
    end
endmodule
"#,
    );

    let value = evaluate(&artifact, &model);
    close(value(model.noise[0].psd), 3.0 * 3.0 * 5.0, "psd");
}

/// Flicker carries an exponent alongside its power, and only the power is
/// scaled: an exponent is a shape, not a magnitude.
#[test]
fn a_flicker_source_scales_its_power_and_not_its_exponent() {
    let (artifact, model) = lower(
        r#"
module flicker(p, n);
    inout p, n;
    electrical p, n;
    analog begin
        I(p, n) <+ 2.0 * flicker_noise(11.0, 1.5);
    end
endmodule
"#,
    );

    let source = &model.noise[0];
    assert_eq!(source.kind, CanonicalNoiseSourceKind::Flicker);
    let exponent = source.exponent.expect("flicker carries an exponent");

    let value = evaluate(&artifact, &model);
    close(value(source.psd), 4.0 * 11.0, "psd");
    close(value(exponent), 1.5, "exponent");
}

/// A source in an untaken branch is inactive and contributes nothing, and it is
/// the control flow that says so: nothing evaluated its power.
#[test]
fn a_source_under_an_untaken_branch_reads_back_inactive() {
    let source = r#"
module guarded(p, n);
    inout p, n;
    electrical p, n;
    parameter real enable = 0.0;
    analog begin
        if (enable > 0.5) begin
            I(p, n) <+ white_noise(13.0);
        end else begin
            I(p, n) <+ white_noise(17.0);
        end
    end
endmodule
"#;
    let (artifact, model) = lower(source);
    assert_eq!(model.noise.len(), 2, "both arms wrote a source");

    let value = evaluate(&artifact, &model);
    // `enable` defaults to zero, so the else arm is the one taken.
    close(value(model.noise[0].active), 0.0, "then arm active");
    close(value(model.noise[1].active), 1.0, "else arm active");
    close(value(model.noise[1].psd), 17.0, "else arm psd");
}

/// The same, for a ternary rather than a statement `if`. It lowers to the same
/// diamond, so the untaken arm's operand is equally unevaluated — which is what
/// lets a model guard a power that is not finite outside its own regime.
#[test]
fn a_source_under_an_untaken_ternary_reads_back_inactive() {
    let (artifact, model) = lower(
        r#"
module ternary(p, n);
    inout p, n;
    electrical p, n;
    parameter real enable = 0.0;
    analog begin
        I(p, n) <+ (enable > 0.5) ? white_noise(13.0) : white_noise(17.0);
    end
endmodule
"#,
    );
    assert_eq!(model.noise.len(), 2);

    let value = evaluate(&artifact, &model);
    close(value(model.noise[0].active), 0.0, "then arm active");
    close(value(model.noise[1].active), 1.0, "else arm active");
    close(value(model.noise[1].psd), 17.0, "else arm psd");
}

/// Two sources in one contribution are told apart by their ordinal, and each
/// keeps the amplitude of the term it sat in rather than the sum's.
#[test]
fn sources_added_in_one_contribution_keep_their_own_amplitudes() {
    let (artifact, model) = lower(
        r#"
module summed(p, n);
    inout p, n;
    electrical p, n;
    analog begin
        I(p, n) <+ 2.0 * white_noise(5.0) + 3.0 * white_noise(7.0);
    end
endmodule
"#,
    );

    assert_eq!(model.noise.len(), 2);
    assert_eq!(model.noise[0].contribution, model.noise[1].contribution);
    assert_eq!(model.noise[0].ordinal, 0);
    assert_eq!(model.noise[1].ordinal, 1);

    let value = evaluate(&artifact, &model);
    close(value(model.noise[0].psd), 4.0 * 5.0, "first psd");
    close(value(model.noise[1].psd), 9.0 * 7.0, "second psd");
}

/// A source's power reads the variables in scope where it was written, not the
/// ones in scope at the end of the body. Assigning after the contribution is
/// what tells the two apart.
#[test]
fn a_power_reads_the_definitions_reaching_its_own_site() {
    let (artifact, model) = lower(
        r#"
module reaching(p, n);
    inout p, n;
    electrical p, n;
    real scale;
    analog begin
        scale = 2.0;
        I(p, n) <+ white_noise(scale);
        scale = 100.0;
        I(p, n) <+ 0.0;
    end
endmodule
"#,
    );

    let value = evaluate(&artifact, &model);
    close(value(model.noise[0].psd), 2.0, "psd at its own site");
}

/// The lowered sources must line up one-for-one with the plan the descriptors
/// come from. The plan is extracted from a second lowering of the same
/// expressions and shares no expression ids with the body, so nothing but this
/// correspondence connects a lowered power to the branch it is injected at.
#[test]
fn every_lowered_source_corresponds_to_one_plan_source() {
    for (name, source) in fixtures() {
        let (artifact, model) = lower(source);
        let plan = &artifact.noise_sources.sources;
        assert_eq!(
            model.noise.len(),
            plan.len(),
            "{name}: {} lowered sources against {} planned",
            model.noise.len(),
            plan.len()
        );
        for (index, planned) in plan.iter().enumerate() {
            let contribution = artifact.mir.equations[usize::from(planned.equation)].contribution;
            let matched = matching(&model.noise, contribution, index, plan);
            assert_eq!(
                matched.kind, planned.kind,
                "{name}: source {index} kind disagrees"
            );
            assert_eq!(
                matched.table.len(),
                planned
                    .table
                    .as_ref()
                    .map_or(0, |table| table.operands.len()),
                "{name}: source {index} table width disagrees"
            );
            assert_eq!(
                matched.exponent.is_some(),
                planned.exponent.is_some(),
                "{name}: source {index} exponent disagrees"
            );
        }
    }
}

/// The lowered source a plan entry names: same contribution, and the same
/// position among that contribution's sources.
fn matching<'a>(
    lowered: &'a [CfgNoiseSource],
    contribution: rspice_veriloga::canonical_ir::ContributionId,
    index: usize,
    plan: &[rspice_veriloga::canonical_ir::CanonicalNoiseSource],
) -> &'a CfgNoiseSource {
    let ordinal = plan[..index]
        .iter()
        .filter(|earlier| earlier.equation == plan[index].equation)
        .count();
    lowered
        .iter()
        .find(|source| source.contribution == contribution && source.ordinal == ordinal)
        .unwrap_or_else(|| {
            panic!("no lowered source for contribution {contribution} ordinal {ordinal}")
        })
}

fn fixtures() -> Vec<(&'static str, &'static str)> {
    vec![
        (
            "resistor",
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
            "guarded",
            r#"
module guarded_noise(p, n);
    inout p, n;
    electrical p, n;
    parameter real enable = 1.0;
    analog begin
        I(p, n) <+ V(p, n);
        if (enable > 0.5) begin
            I(p, n) <+ white_noise(1.0e-20, "on");
        end
    end
endmodule
"#,
        ),
        (
            "flicker and white together",
            r#"
module both(d, s);
    inout d, s;
    electrical d, s;
    parameter real kf = 1.0e-25;
    parameter real af = 1.2;
    analog begin
        I(d, s) <+ V(d, s) * 1.0e-3;
        I(d, s) <+ white_noise(1.0e-21, "shot") + flicker_noise(kf, af, "flicker");
    end
endmodule
"#,
        ),
        (
            "scaled by a bias-dependent amplitude",
            r#"
module scaled_noise(d, s);
    inout d, s;
    electrical d, s;
    parameter real gm = 1.0e-3;
    analog begin
        I(d, s) <+ gm * V(d, s);
        I(d, s) <+ (1.0 + V(d, s) * V(d, s)) * white_noise(1.0e-22, "channel");
    end
endmodule
"#,
        ),
    ]
}

/// How much of the shipped corpus lowers its noise sources into the body, and
/// where the rest stops.
///
/// Ignored because it compiles every shipped model; run it when the walk
/// changes, since it is the only thing that reports a model whose plan and body
/// disagree about how many sources there are.
#[test]
#[ignore = "compiles the whole shipped corpus"]
fn the_whole_corpus_reports_which_noise_plans_the_body_reproduces() {
    let root = model_root();
    let candidates = discover_veriloga_sources(&root).expect("model tree");
    let mut carried = 0usize;
    let mut sources = 0usize;
    let mut refused = 0usize;
    let mut silent = 0usize;
    let mut unlowered = 0usize;
    let mut uncompiled = 0usize;

    for (candidate, module) in candidates.iter().flat_map(|candidate| {
        candidate
            .modules
            .iter()
            .map(move |module| (candidate, module.to_string()))
    }) {
        // The same include paths and defines the device census uses. Compiling
        // a CMC model without its compile profile fails on the include line,
        // which would leave most of the corpus counted as "no noise".
        let mut options = CompilerOptions::default();
        options.include_paths.push(root.clone());
        options.defines = candidate.compile_profile.defines.clone();
        options.undefines = candidate.compile_profile.undefines.clone();
        // Counted, not skipped: a model that never reached the plan is not a
        // model whose plan the body reproduced, and a census that says nothing
        // about it reads as coverage it does not have.
        let Ok(compiled) = VerilogACompiler::new(options)
            .compile_file_canonical_ir_with_metadata(&candidate.path, Some(&module))
        else {
            uncompiled += 1;
            continue;
        };
        let artifact = compiled.artifact;
        if artifact.noise_sources.sources.is_empty() {
            silent += 1;
            continue;
        }
        match CfgModel::from_hir(&artifact.hir, &artifact.mir) {
            Ok(model) if model.noise.len() == artifact.noise_sources.sources.len() => {
                carried += 1;
                sources += model.noise.len();
                eprintln!("{module:>24}  {} sources", model.noise.len());
            }
            Ok(model) => {
                refused += 1;
                eprintln!(
                    "{module:>24}  lowered {} of {} sources",
                    model.noise.len(),
                    artifact.noise_sources.sources.len()
                );
            }
            // A body that does not lower has no noise to disagree about. It is
            // the CFG gap the device backend already refuses on, counted apart
            // so it cannot be read as this walk getting a plan wrong.
            Err(_) => unlowered += 1,
        }
    }

    eprintln!(
        "\n{carried} models reproduce their whole noise plan ({sources} sources), \
         {refused} disagree; {silent} carry no noise, {unlowered} do not lower, \
         {uncompiled} did not compile"
    );
    assert_eq!(refused, 0, "a model's body disagrees with its noise plan");
    assert!(carried > 0, "no model reproduced its noise plan");
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
