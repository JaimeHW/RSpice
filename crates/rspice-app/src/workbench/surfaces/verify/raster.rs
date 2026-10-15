//! Review renders of the Verify pages, the fixtures they read, and the page
//! geometry both the renders and the layout tests measure.
//!
//! Each page is rendered on a project that has run nothing and on one holding
//! real retained evidence for that page: a corner run with a failing and a
//! near-limit cell, forty Monte Carlo samples of which three fail, an SOA run
//! with two violations, an optimizer's iterations. The PNGs, and a text report
//! of where the toolbar, the table head, the rows and the right-aligned numbers
//! landed, go to `RSPICE_RASTER_DIR` (default: the system temp directory).
//!
//! The geometry is published by the widgets themselves through [`probe`], which
//! exists only in test builds: a measurement read back from the pixels would be
//! a second account of where the code put things.

use egui::{Rect, vec2};

use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision};
use crate::state::{
    AnalysisResult, AnalysisResultFamilyMetadata, AnalysisType, FamilyMeasurementEvidence,
    FamilyMemberId, FamilyMemberMeasurements, SimulationRun, SimulationRunLifecycle, SpecEntry,
    SpecPointScope, WaveformData,
};
use crate::workbench::RSpiceApp;
use crate::workbench::state::{VerificationPage, Workspace};
use rspice_results::yield_analysis::{
    DistributionStats, MonteCarloSamplingMode, YieldAnalysisProvenance, YieldResult, YieldSpec,
};

pub(super) const PAGE_WIDTH: f32 = 1100.0;
pub(super) const PAGE_HEIGHT: f32 = 720.0;

/// Rects the Verify widgets published this pass, by what they are.
#[derive(Debug, Clone, Default)]
struct Probes(Vec<(String, Rect)>);

fn probes_id() -> egui::Id {
    egui::Id::new("workbench.verify.raster.probes")
}

/// Publish where one element landed, for the geometry report.
pub(super) fn probe(ui: &egui::Ui, key: &str, rect: Rect) {
    ui.ctx().data_mut(|data| {
        data.get_temp_mut_or_default::<Probes>(probes_id())
            .0
            .push((key.to_owned(), rect));
    });
}

fn take_probes(ctx: &egui::Context) -> Vec<(String, Rect)> {
    ctx.data_mut(|data| {
        let probes = data.get_temp::<Probes>(probes_id()).unwrap_or_default();
        data.remove::<Probes>(probes_id());
        probes.0
    })
}

/// Where one page's elements landed on its last pass.
#[derive(Debug, Clone, Default)]
pub(super) struct PageGeometry {
    probes: Vec<(String, Rect)>,
}

impl PageGeometry {
    pub(super) fn all(&self, key: &str) -> Vec<Rect> {
        self.probes
            .iter()
            .filter(|(name, _)| name == key)
            .map(|(_, rect)| *rect)
            .collect()
    }

    pub(super) fn one(&self, key: &str) -> Option<Rect> {
        self.all(key).into_iter().next()
    }

    /// Every right-aligned table column's distinct right edges, by column.
    pub(super) fn right_edges(&self) -> Vec<(String, Vec<f32>)> {
        let mut columns: Vec<(String, Vec<f32>)> = Vec::new();
        for (name, rect) in &self.probes {
            if !name.starts_with("table.right.") {
                continue;
            }
            let edge = (rect.right() * 10.0).round() / 10.0;
            match columns.iter_mut().find(|(column, _)| column == name) {
                Some((_, edges)) if !edges.contains(&edge) => edges.push(edge),
                Some(_) => {}
                None => columns.push((name.clone(), vec![edge])),
            }
        }
        columns
    }

    /// Every element whose words start left of the panel inset or end past
    /// the page's right inset.
    pub(super) fn outside_insets(&self, width: f32) -> Vec<String> {
        self.probes
            .iter()
            .filter(|(name, _)| {
                name.starts_with("table.right.")
                    || name.starts_with("toolbar.")
                    || name.starts_with("empty.")
            })
            .filter(|(_, rect)| rect.left() < 9.5 || rect.right() > width - 9.5)
            .map(|(name, rect)| format!("{name} {:.1}..{:.1}", rect.left(), rect.right()))
            .collect()
    }

    pub(super) fn report(&self, name: &str, width: f32) -> String {
        let mut lines = vec![format!("{name} ({width} px)")];
        let span = |rect: Rect| {
            format!(
                "{:.1}..{:.1} (h {:.1}) x {:.1}..{:.1}",
                rect.top(),
                rect.bottom(),
                rect.height(),
                rect.left(),
                rect.right()
            )
        };
        if let Some(toolbar) = self.one("toolbar") {
            lines.push(format!("  toolbar        {}", span(toolbar)));
        }
        if let Some(title) = self.one("toolbar.title") {
            lines.push(format!("  title left x   {:.1}", title.left()));
        }
        if let Some(summary) = self.one("toolbar.summary") {
            lines.push(format!("  summary        {}", span(summary)));
        }
        let actions = self.all("toolbar.action");
        if !actions.is_empty() {
            let right = actions.iter().map(|rect| rect.right()).fold(0.0, f32::max);
            lines.push(format!(
                "  actions        {} · right gap {:.1}",
                actions
                    .iter()
                    .map(|rect| format!("{:.1}..{:.1}", rect.left(), rect.right()))
                    .collect::<Vec<_>>()
                    .join(", "),
                width - right
            ));
        }
        if let Some(empty) = self.one("empty") {
            lines.push(format!("  empty line     {}", span(empty)));
        }
        if let Some(text) = self.one("empty.text") {
            lines.push(format!("  empty text x   {:.1}", text.left()));
        }
        if let Some(head) = self.one("table.head") {
            lines.push(format!("  table head     {}", span(head)));
            if let Some(toolbar) = self.one("toolbar") {
                lines.push(format!(
                    "  toolbar→head   {:.1}",
                    head.top() - toolbar.bottom()
                ));
            }
        }
        let rows = self.all("table.row");
        if !rows.is_empty() {
            let mut heights = rows
                .iter()
                .map(|rect| format!("{:.1}", rect.height()))
                .collect::<Vec<_>>();
            heights.dedup();
            lines.push(format!(
                "  rows           {} · h {}",
                rows.len(),
                heights.join(", ")
            ));
            if let Some(head) = self.one("table.head") {
                lines.push(format!(
                    "  head→row 1     {:.1}",
                    rows[0].top() - head.bottom()
                ));
            }
        }
        for (column, edges) in self.right_edges() {
            lines.push(format!(
                "  {column} right edges {}",
                edges
                    .iter()
                    .map(|edge| format!("{edge:.1}"))
                    .collect::<Vec<_>>()
                    .join(", ")
            ));
        }
        for (key, prefix) in [("histogram", "histogram     "), ("split", "split         ")] {
            if let Some(rect) = self.one(key) {
                lines.push(format!("  {prefix} {}", span(rect)));
            }
        }
        let outside = self.outside_insets(width);
        lines.push(format!(
            "  outside insets {}",
            if outside.is_empty() {
                "none".to_owned()
            } else {
                outside.join("; ")
            }
        ));
        lines.join("\n") + "\n"
    }
}

/// Lay one page out at the review size and read back where it put things,
/// without rasterizing it.
pub(super) fn page_geometry(mut app: RSpiceApp, page: VerificationPage) -> PageGeometry {
    app.state.workbench.activate(Workspace::Verify);
    app.state.workbench.verification_page = page;
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut probes = Vec::new();
    for _ in 0..3 {
        let _ = take_probes(&ctx);
        let _ = ctx.run_ui(
            egui::RawInput {
                screen_rect: Some(Rect::from_min_size(
                    egui::Pos2::ZERO,
                    vec2(PAGE_WIDTH, PAGE_HEIGHT),
                )),
                ..Default::default()
            },
            |ctx| {
                egui::CentralPanel::default()
                    .frame(egui::Frame::NONE)
                    .show(ctx, |ui| super::show(ui, &mut app));
            },
        );
        probes = take_probes(&ctx);
    }
    PageGeometry { probes }
}

fn attributed(analysis: AnalysisResult) -> AnalysisResult {
    attributed_to(analysis, AnalysisInstanceId::new())
}

fn attributed_to(analysis: AnalysisResult, source_instance_id: AnalysisInstanceId) -> AnalysisResult {
    analysis.with_provenance(
        crate::state::AnalysisResultProvenance::new(
            source_instance_id,
            ObjectRevision::INITIAL,
            ContentDigest::from_bytes([0x5a; 32]),
            Vec::new(),
        )
        .expect("fixture provenance is internally valid"),
    )
}

/// A run of `plan_id`, sealed by a prepared receipt whose one task produced
/// `analysis`.
pub(super) fn sealed_run(
    run_number: u64,
    plan_id: crate::product::SimulationPlanId,
    source_instance_id: AnalysisInstanceId,
    analysis_kind_tag: u8,
    analysis: AnalysisResult,
) -> SimulationRun {
    let task = crate::state::PreparedRunTaskReceipt::new(
        source_instance_id,
        ObjectRevision::INITIAL,
        Vec::new(),
        analysis_kind_tag,
        ContentDigest::from_bytes([analysis_kind_tag; 32]),
    )
    .expect("fixture task receipt is internally valid");
    let receipt = crate::state::PreparedRunReceipt::new(crate::state::PreparedRunReceiptInput {
        source_domain: crate::state::AnalysisResultSourceDomain::SimulationPlan,
        simulation_plan_id: Some(plan_id),
        project_revision: ObjectRevision::INITIAL,
        prepared_snapshot_digest: ContentDigest::from_bytes([0x5a; 32]),
        source_content_digest: ContentDigest::from_bytes([0x41; 32]),
        source_check_receipt: crate::state::PreparedSourceCheckReceipt::SchematicDrc(
            ContentDigest::from_bytes([0x42; 32]),
        ),
        project_model_sources: Vec::new(),
        specifications: Vec::new(),
        specification_policy: rspice_results::specification::PreparedSpecificationPolicy::default(),
        tasks: vec![task],
    })
    .expect("fixture run receipt is internally valid");
    let mut run = SimulationRun::new_prepared(run_number, receipt);
    run.add_analysis(attributed_to(analysis, source_instance_id));
    run
}

/// The canonical execution tag a transient task carries, which the receipt
/// checks its result against.
const TRANSIENT_TASK_TAG: u8 = 5;

/// A second-order step response settling to 1 V.
fn step_response(damping: f64, natural_rad_s: f64, time: &[f64]) -> Vec<f64> {
    let root = (1.0 - damping * damping).sqrt();
    let damped = natural_rad_s * root;
    time.iter()
        .map(|t| {
            1.0 - (-damping * natural_rad_s * t).exp()
                * ((damped * t).cos() + damping / root * (damped * t).sin())
        })
        .collect()
}

/// Three microseconds of a step, at 12.5 ns.
fn step_time() -> Vec<f64> {
    (0..=240).map(|index| f64::from(index) * 12.5e-9).collect()
}

/// A transient step of the active plan's one analysis, with its measurements.
fn step_analysis(damping: f64, natural_rad_s: f64, measurements: &[(&str, f64)]) -> AnalysisResult {
    let time = step_time();
    AnalysisResult::new(1, AnalysisType::Transient, "Step")
        .with_waveforms(vec![
            WaveformData::new(
                "V(out)",
                time.clone(),
                step_response(damping, natural_rad_s, &time),
                "#4c9be8",
            )
            .with_unit("V"),
        ])
        .with_measurements(
            measurements
                .iter()
                .map(|(name, value)| rspice_core::MeasureResult::success(name, *value))
                .collect(),
        )
}

/// A plan with one tuned resistor and two limits, and two runs of it: Run 3,
/// the baseline the tuning session opened on, whose overshoot fails, and
/// Run 4, the committed candidate, whose settling time is near its limit.
pub(super) fn tuning_app() -> RSpiceApp {
    use crate::state::{
        DesignVariable, DesignVariableOverridePolicy, DesignVariableQuantity, DesignVariableRange,
        DesignVariableScope, DesignVariableSweepEligibility,
    };

    let mut app = RSpiceApp::test_instance();
    let plan_id = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .expect("default plan")
        .id();
    let variable = DesignVariable::new(
        "RGAIN",
        "499 ohm",
        DesignVariableQuantity::Resistance,
        DesignVariableScope::Project,
        "Closed-loop gain-setting resistor",
        Some(DesignVariableRange {
            minimum: "350 ohm".to_owned(),
            maximum: "750 ohm".to_owned(),
        }),
        DesignVariableSweepEligibility::NestedSweepAndOptimization,
        DesignVariableOverridePolicy::ExplicitTestLocalOverride,
    )
    .expect("valid tuning variable");
    app.state
        .workspace
        .content
        .add_design_variable(plan_id, variable)
        .expect("the variable enters the active plan");
    set_limits(
        &mut app,
        vec![
            limit("tsettle", None, Some(2.0e-6), "s"),
            limit("overshoot", None, Some(10.0), "%"),
        ],
    );
    let source = AnalysisInstanceId::new();
    let baseline = sealed_run(
        3,
        plan_id,
        source,
        TRANSIENT_TASK_TAG,
        step_analysis(0.55, 4.49e6, &[("tsettle", 1.62e-6), ("overshoot", 12.6)]),
    );
    let candidate = sealed_run(
        4,
        plan_id,
        source,
        TRANSIENT_TASK_TAG,
        step_analysis(0.65, 3.22e6, &[("tsettle", 1.91e-6), ("overshoot", 6.8)]),
    );
    app.state.simulation.retained.runs = vec![baseline, candidate].into();
    app.state.simulation.view.active_run_idx = Some(0);
    super::tuning::sync_tuning_session(&mut app);
    app.state.simulation.view.active_run_idx = Some(1);
    app
}

/// Run 4 of the active plan against its baseline, Run 3: four measurements and
/// the output waveform. Bandwidth breaks its 2 % rule, overshoot has no rule,
/// and the waveform is selected, so its comparison opens under the table.
pub(super) fn regression_app() -> RSpiceApp {
    use crate::state::{
        RegressionComparisonMethod, RegressionComparisonWindow, RegressionSpecificationPolicy,
        RegressionTargetSelector, RegressionToleranceRule,
    };

    let mut app = RSpiceApp::test_instance();
    let plan_id = app
        .state
        .sim_setup
        .stable_analysis_plan()
        .expect("default plan")
        .id();
    let source = AnalysisInstanceId::new();
    let run = |number: u64, damping: f64, natural: f64, values: [f64; 4]| {
        sealed_run(
            number,
            plan_id,
            source,
            TRANSIENT_TASK_TAG,
            step_analysis(
                damping,
                natural,
                &[
                    ("gain", values[0]),
                    ("bw", values[1]),
                    ("tsettle", values[2]),
                    ("overshoot", values[3]),
                ],
            ),
        )
    };
    let baseline = run(3, 0.55, 4.49e6, [42.30, 18.2e6, 1.62e-6, 12.6]);
    let candidate = run(4, 0.56, 4.45e6, [42.28, 17.1e6, 1.64e-6, 12.1]);
    let baseline_id = baseline.run_id;
    let rule = |target: RegressionTargetSelector, absolute: f64, relative: f64| {
        let waveform = target.kind == crate::state::RegressionTargetKind::Waveform;
        RegressionToleranceRule {
            target,
            method: RegressionComparisonMethod::AbsoluteRelativeEnvelope,
            absolute_tolerance: absolute,
            relative_tolerance: relative,
            time_skew_allowance: if waveform { 20.0e-9 } else { 0.0 },
            comparison_window: waveform.then_some(RegressionComparisonWindow {
                start: 0.0,
                end: 3.0e-6,
            }),
        }
    };
    let mut rules = super::derive_regression_checks(&baseline, &candidate)
        .into_iter()
        .filter_map(|check| match check.name.as_str() {
            "gain" => Some(rule(check.target, 0.1, 0.005)),
            "bw" => Some(rule(check.target, 0.0, 0.02)),
            "tsettle" => Some(rule(check.target, 0.0, 0.05)),
            _ => None,
        })
        .collect::<Vec<_>>();
    let waveform = super::regression_waveform_pairs(&baseline, &candidate)
        .first()
        .map(|pair| pair.target.clone())
        .expect("both runs hold V(out)");
    rules.push(rule(waveform.clone(), 0.02, 0.02));
    let payload = app.state.workspace.content.ensure_active_plan_data(plan_id);
    payload.regression_tolerances = rules;
    payload.specification_policy.regression = RegressionSpecificationPolicy::LimitAndWaveform;
    app.state.simulation.retained.runs = vec![baseline, candidate].into();
    assert!(app.state.simulation.select_run(1));
    super::commit_regression_baseline(&mut app, baseline_id).expect("the baseline commits");
    app.state.workbench.verification.regression_selected_target = Some(waveform);
    app
}

fn limit(measurement: &str, min: Option<f64>, max: Option<f64>, unit: &str) -> SpecEntry {
    SpecEntry {
        measurement: measurement.to_owned(),
        expression: String::new(),
        min,
        max,
        unit: unit.to_owned(),
        scope: SpecPointScope::AllPoints,
    }
}

/// An app whose one run, Run 3, holds `analysis` and is the run Verify reads.
fn app_with_run(analysis: AnalysisResult) -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    let mut run = SimulationRun::new(3);
    run.restore_lifecycle(SimulationRunLifecycle::Completed, 12.4)
        .expect("a finite elapsed time");
    run.add_analysis(attributed(analysis));
    app.state.simulation.retained.runs = vec![run].into();
    app.state.simulation.view.active_run_idx = Some(0);
    app
}

/// Forty Monte Carlo samples of a 40 dB gain floor, three of them under it.
pub(super) fn monte_carlo_app() -> RSpiceApp {
    let mut app = RSpiceApp::test_instance();
    set_limits(&mut app, vec![limit("gain", Some(40.0), None, "dB")]);
    let samples = (0..40)
        .map(|index| match index {
            7 => 39.2,
            19 => 39.7,
            31 => 38.9,
            _ => 40.4 + f64::from(index % 9) * 0.35,
        })
        .collect::<Vec<f64>>();
    let trail = samples
        .iter()
        .map(|value| *value >= 40.0)
        .collect::<Vec<_>>();
    let pass_count = trail.iter().filter(|passes| **passes).count();
    let result = YieldResult {
        spec: YieldSpec::lower("gain", 40.0, "dB"),
        total_runs: samples.len(),
        pass_count,
        fail_count: samples.len() - pass_count,
        yield_percent: pass_count as f64 / samples.len() as f64 * 100.0,
        stats: DistributionStats::default(),
        samples,
        trail,
    };
    let mut run = SimulationRun::new(3);
    run.restore_lifecycle(SimulationRunLifecycle::Completed, 12.4)
        .expect("a finite elapsed time");
    let provenance = YieldAnalysisProvenance {
        source_run_id: run.run_id,
        source_dataset_id: run.dataset_id,
        seed: 0x1A2B,
        runs_requested: 40,
        runs_completed: 40,
        sampling_mode: MonteCarloSamplingMode::PseudoRandom,
    };
    app.state.simulation.retained.runs = vec![run].into();
    app.state.simulation.view.active_run_idx = Some(0);
    app.state
        .simulation
        .replace_yield_evidence(vec![result], Some(provenance));
    app
}

/// The corners of the corner fixture: process, supply, temperature.
pub(super) const FIXTURE_CORNERS: [(&str, f64, f64); 7] = [
    ("TT", 1.8, 27.0),
    ("SS", 1.62, 125.0),
    ("SS", 1.62, -40.0),
    ("FF", 1.98, 125.0),
    ("FF", 1.98, -40.0),
    ("SF", 1.8, 27.0),
    ("FS", 1.8, 27.0),
];

/// A seven-corner run of four limited outputs and two node voltages: gain
/// fails at SS 125 °C, and phase margin and supply current sit near their
/// limits at FF 125 °C.
pub(super) fn corner_app() -> RSpiceApp {
    let outputs: [(&str, [f64; 7]); 4] = [
        ("gain", [42.3, 39.4, 42.6, 43.0, 44.2, 42.4, 42.9]),
        (
            "bw",
            [18.2e6, 11.9e6, 14.0e6, 24.1e6, 26.5e6, 16.0e6, 17.1e6],
        ),
        ("pm", [62.0, 58.1, 60.4, 47.0, 52.3, 61.0, 63.2]),
        (
            "idd",
            [
                0.82e-3, 0.71e-3, 0.66e-3, 1.16e-3, 0.95e-3, 0.80e-3, 0.84e-3,
            ],
        ),
    ];
    let labels = FIXTURE_CORNERS
        .iter()
        .map(|(process, volts, celsius)| format!("{process}_{volts:.6}V_{celsius:.6}C"))
        .collect::<Vec<_>>();
    let members = labels
        .iter()
        .enumerate()
        .map(|(index, label)| {
            FamilyMemberMeasurements::new(
                FamilyMemberId::Corner {
                    index,
                    label: label.clone(),
                },
                outputs
                    .iter()
                    .map(|(name, values)| FamilyMeasurementEvidence {
                        unit: None,
                        name: (*name).to_owned(),
                        value: Some(values[index]),
                        passed: true,
                        error: None,
                    })
                    .collect(),
            )
        })
        .collect();
    let axis = (0..FIXTURE_CORNERS.len())
        .map(|index| index as f64)
        .collect::<Vec<_>>();
    let analysis = AnalysisResult::new(1, AnalysisType::Corner, "Corners")
        .with_family_metadata(AnalysisResultFamilyMetadata::Corner {
            x_values: axis.clone(),
            x_label: "Corner Index".to_owned(),
            x_unit: String::new(),
            temperatures_c: FIXTURE_CORNERS.iter().map(|corner| corner.2).collect(),
            corner_labels: labels,
            failed_corners: 0,
            member_measurements: members,
        })
        .with_waveforms(vec![
            WaveformData::new(
                "V(out)",
                axis.clone(),
                vec![0.912, 0.884, 0.901, 0.934, 0.951, 0.903, 0.915],
                "#4c9be8",
            ),
            WaveformData::new(
                "V(bias)",
                axis,
                vec![0.612, 0.655, 0.641, 0.578, 0.590, 0.620, 0.606],
                "#d0a03c",
            ),
        ]);
    let mut app = app_with_run(analysis);
    set_limits(
        &mut app,
        vec![
            limit("gain", Some(40.0), None, "dB"),
            limit("bw", Some(10.0e6), None, "Hz"),
            limit("pm", Some(45.0), None, "deg"),
            limit("idd", None, Some(1.2e-3), "A"),
        ],
    );
    app
}

/// Make `specs` the limits the active plan judges its outputs by.
fn set_limits(app: &mut RSpiceApp, specs: Vec<SpecEntry>) {
    match app
        .state
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .map(|plan| plan.id())
    {
        Some(plan_id) => app
            .state
            .workspace
            .content
            .replace_active_specs(plan_id, specs),
        None => app.state.workspace.content.specs = specs,
    }
}

/// An SOA run of five checks on four devices: two violations, one check near
/// its limit, two clean.
pub(super) fn soa_app() -> RSpiceApp {
    use crate::state::{
        SoaEvaluationEvidence, SoaParameterEvidence, SoaRuleVerdictEvidence, SoaViolationEvidence,
        SoaViolationSeverityEvidence,
    };

    const SAMPLES: usize = 64;
    let time: Vec<f64> = (0..SAMPLES).map(|index| index as f64 * 1.0e-9).collect();
    let checks = [
        (
            "M1",
            SoaParameterEvidence::GateSourceVoltage,
            1.98,
            1.20,
            "V",
        ),
        (
            "M1",
            SoaParameterEvidence::DrainSourceVoltage,
            3.3,
            3.52,
            "V",
        ),
        (
            "M2",
            SoaParameterEvidence::DrainSourceVoltage,
            3.3,
            3.08,
            "V",
        ),
        (
            "M3",
            SoaParameterEvidence::DrainSourceVoltage,
            3.3,
            3.61,
            "V",
        ),
        (
            "M4",
            SoaParameterEvidence::DrainCurrent,
            10.0e-3,
            4.1e-3,
            "A",
        ),
    ];
    let mut waveforms = Vec::new();
    let mut evaluations = Vec::new();
    let mut violations = Vec::new();
    for (device, parameter, limit_value, worst, unit) in checks {
        let verdict = match crate::results::safety::SoaThresholds::default()
            .verdict(worst, limit_value)
        {
            crate::results::safety::SoARuleVerdict::Pass => SoaRuleVerdictEvidence::Pass,
            crate::results::safety::SoARuleVerdict::Warning => SoaRuleVerdictEvidence::Warning,
            crate::results::safety::SoARuleVerdict::Violation => SoaRuleVerdictEvidence::Violation,
            crate::results::safety::SoARuleVerdict::Critical => SoaRuleVerdictEvidence::Critical,
        };
        let worst_time_s = time[SAMPLES - 1];
        waveforms.push(
            WaveformData::new(
                crate::results::safety::soa_stress_waveform_name(
                    device,
                    parameter.runtime_parameter(),
                ),
                time.clone(),
                (0..SAMPLES)
                    .map(|index| worst * (index as f64 + 1.0) / SAMPLES as f64)
                    .collect::<Vec<_>>(),
                "#00aaff",
            )
            .with_unit(unit),
        );
        evaluations.push(SoaEvaluationEvidence {
            duration: None,
            thresholds: Default::default(),
            envelope: None,
            derating: None,
            device_id: device.to_owned(),
            parameter,
            limit_value,
            worst_actual_value: worst,
            worst_time_s,
            sample_count: SAMPLES as u64,
            unit: unit.to_owned(),
            description: format!(
                "{device} {}",
                super::safety::soa_parameter_display(parameter)
            ),
            verdict,
        });
        let severity = match verdict {
            SoaRuleVerdictEvidence::Pass => None,
            SoaRuleVerdictEvidence::Warning => Some(SoaViolationSeverityEvidence::Warning),
            SoaRuleVerdictEvidence::Violation => Some(SoaViolationSeverityEvidence::Violation),
            SoaRuleVerdictEvidence::Critical => Some(SoaViolationSeverityEvidence::Critical),
        };
        if let Some(severity) = severity {
            violations.push(SoaViolationEvidence {
                device_id: device.to_owned(),
                parameter,
                limit_value,
                actual_value: worst,
                time_s: worst_time_s,
                severity,
            });
        }
    }
    app_with_run(
        AnalysisResult::new(1, AnalysisType::Soa, "SOA")
            .with_family_metadata(AnalysisResultFamilyMetadata::Soa { time })
            .with_waveforms(waveforms)
            .with_result_payload(crate::state::AnalysisResultPayload::Soa {
                source_history: None,
                evaluations,
                violations,
            }),
    )
}

/// Twenty-four optimizer iterations over two variables, converged.
pub(super) fn optimization_app() -> RSpiceApp {
    let iterations = (0..24).map(f64::from).collect::<Vec<_>>();
    let cost = iterations
        .iter()
        .map(|step| 0.8 * (-step / 5.0).exp() + 0.012)
        .collect::<Vec<_>>();
    let width = iterations
        .iter()
        .map(|step| 2.0e-6 + 1.6e-6 * (1.0 - (-step / 6.0).exp()))
        .collect::<Vec<_>>();
    let bias = iterations
        .iter()
        .map(|step| 40.0e-6 - 12.0e-6 * (1.0 - (-step / 4.0).exp()))
        .collect::<Vec<_>>();
    let best_cost = cost.iter().copied().fold(f64::INFINITY, f64::min);
    app_with_run(
        AnalysisResult::new(1, AnalysisType::Optimization, "Optimization")
            .with_waveforms(vec![
                WaveformData::new("OPT_COST", iterations.clone(), cost, "#4c9be8"),
                WaveformData::new("OPT_W1", iterations.clone(), width.clone(), "#d0a03c"),
                WaveformData::new("OPT_IBIAS", iterations.clone(), bias.clone(), "#8bc34a"),
            ])
            .with_family_metadata(AnalysisResultFamilyMetadata::Optimization {
                iterations,
                best_cost,
                best_variables: [("W1".to_owned(), width[23]), ("IBIAS".to_owned(), bias[23])]
                    .into_iter()
                    .collect(),
                best_objectives: Vec::new(),
                best_constraints: Vec::new(),
                converged: true,
            }),
    )
}

/// The fixture that seeds one page with real evidence.
fn seeded(page: VerificationPage) -> RSpiceApp {
    match page {
        VerificationPage::Corners => corner_app(),
        VerificationPage::DeviceSafety => soa_app(),
        VerificationPage::Optimization => optimization_app(),
        VerificationPage::Yield => monte_carlo_app(),
        VerificationPage::Tuning => tuning_app(),
        VerificationPage::Regression => regression_app(),
    }
}

#[test]
#[ignore = "writes PNGs for a human to look at; run with --ignored"]
fn render_every_verify_page() {
    use std::io::Write as _;

    let directory = std::env::var("RSPICE_RASTER_DIR")
        .map_or_else(|_| std::env::temp_dir(), std::path::PathBuf::from);
    std::fs::create_dir_all(&directory).expect("raster output directory");
    let stderr = std::io::stderr();
    let mut report = stderr.lock();
    for page in VerificationPage::NAVIGATION {
        for state in ["empty", "seeded"] {
            let fixture = || {
                if state == "seeded" {
                    seeded(page)
                } else {
                    RSpiceApp::test_instance()
                }
            };
            let name = format!("verify-{page:?}-{state}").to_lowercase();
            let geometry = page_geometry(fixture(), page);
            let mut app = fixture();
            app.state.workbench.activate(Workspace::Verify);
            app.state.workbench.verification_page = page;
            let canvas =
                crate::ui::raster::render(vec2(PAGE_WIDTH, PAGE_HEIGHT), |ui, background| {
                    egui::CentralPanel::default()
                        .frame(egui::Frame::NONE.fill(background))
                        .show(ui, |ui| super::show(ui, &mut app));
                });
            let height = canvas.content_height().max(1);
            let path = directory.join(format!("{name}.png"));
            std::fs::write(&path, canvas.png(height)).expect("write page render");
            std::fs::write(
                directory.join(format!("{name}.txt")),
                geometry.report(&name, PAGE_WIDTH),
            )
            .expect("write page geometry");
            writeln!(report, "{} {}x{}", path.display(), canvas.width(), height)
                .expect("write raster report");
        }
    }
}
