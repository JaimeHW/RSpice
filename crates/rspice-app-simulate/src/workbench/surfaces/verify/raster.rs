//! The Verify pages' fixtures and the page geometry the layout tests measure.
//!
//! Each page is measured on a project that has run nothing and on one holding
//! real retained evidence for that page: a corner run and a run set's sweep
//! with a failing and a near-limit cell, forty Monte Carlo samples of which
//! three fail, an SOA run with two violations, an optimizer's iterations.
//!
//! The geometry is published by the widgets themselves through [`probe`], which
//! exists only in test builds: a measurement read back from the pixels would be
//! a second account of where the code put things.

use egui::{Rect, vec2};

use crate::product::{AnalysisInstanceId, ContentDigest, ObjectRevision};
use crate::state::VerificationPage;
use crate::state::{
    AnalysisResult, AnalysisResultFamilyMetadata, AnalysisType, FamilyMeasurementEvidence,
    FamilyMemberId, FamilyMemberMeasurements, RetainedTrace, SimulationRun, SimulationRunLifecycle,
    SpecEntry, SpecPointScope,
};
use crate::workbench::app_state::Model;
use crate::workbench::framework::feature::harness::FeatureHarness;
use crate::workbench::simulate::SimulateFeature;
use crate::workbench::state::Workspace;
use rspice_results::family_metadata::MonteCarloVariableMetadata;
use rspice_simulation_contract::analysis_kind::AnalysisKind;

pub const PAGE_WIDTH: f32 = 1100.0;
pub(super) const PAGE_HEIGHT: f32 = 720.0;

/// Publish where one element landed, for the geometry report: into the
/// kit's report, which the shared table publishes its rows and heads into.
pub(super) fn probe(ui: &egui::Ui, key: &str, rect: Rect) {
    rspice_ui_kit::probe::probe(ui, key, rect);
}

fn take_probes(ctx: &egui::Context) -> Vec<(String, Rect)> {
    rspice_ui_kit::probe::take(ctx)
}

/// Where one page's elements landed on its last pass.
#[derive(Debug, Clone, Default)]
pub struct PageGeometry {
    probes: Vec<(String, Rect)>,
}

impl PageGeometry {
    pub fn all(&self, key: &str) -> Vec<Rect> {
        self.probes
            .iter()
            .filter(|(name, _)| name == key)
            .map(|(_, rect)| *rect)
            .collect()
    }

    pub fn one(&self, key: &str) -> Option<Rect> {
        self.all(key).into_iter().next()
    }

    /// Every right-aligned table column's distinct right edges, by column.
    pub fn right_edges(&self) -> Vec<(String, Vec<f32>)> {
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
    pub fn outside_insets(&self, width: f32) -> Vec<String> {
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
}

/// The review-size screen, with `events` arriving on it.
pub fn page_input(events: Vec<egui::Event>) -> egui::RawInput {
    egui::RawInput {
        screen_rect: Some(Rect::from_min_size(
            egui::Pos2::ZERO,
            vec2(PAGE_WIDTH, PAGE_HEIGHT),
        )),
        events,
        ..Default::default()
    }
}

/// One pass of `page` over `input` in an unframed central panel, and where
/// its widgets put things on it.
pub fn page_pass(
    ctx: &egui::Context,
    input: egui::RawInput,
    mut page: impl FnMut(&mut egui::Ui),
) -> (egui::FullOutput, PageGeometry) {
    let _ = take_probes(ctx);
    let output = ctx.run_ui(input, |ctx| {
        egui::CentralPanel::default()
            .frame(egui::Frame::NONE)
            .show(ctx, |ui| page(ui));
    });
    let probes = take_probes(ctx);
    (output, PageGeometry { probes })
}

/// What a Verify page is drawn over: the host opens the Verify workspace on
/// a page and draws the surface. The drivers below measure; a stage only
/// adapts the two steps to its host.
pub trait VerifyStage {
    /// Open the Verify workspace on `page`.
    fn open(&mut self, page: VerificationPage);
    /// Draw the Verify surface into `ui`.
    fn draw(&mut self, ui: &mut egui::Ui);
}

impl VerifyStage for FeatureHarness<SimulateFeature> {
    fn open(&mut self, page: VerificationPage) {
        self.wb.activate(Workspace::Verify);
        self.state.verification_page = page;
    }

    fn draw(&mut self, ui: &mut egui::Ui) {
        // The app's frame judges the selected run before it draws.
        refresh_verification(&mut self.model);
        super::show(ui, &mut self.cx());
    }
}

/// Lay one page out at the review size and read back where it put things,
/// without rasterizing it.
pub fn page_geometry(mut stage: impl VerifyStage, page: VerificationPage) -> PageGeometry {
    stage.open(page);
    let ctx = egui::Context::default();
    crate::ui::Theme::default().apply(&ctx);
    let mut geometry = PageGeometry::default();
    for _ in 0..3 {
        geometry = page_pass(&ctx, page_input(Vec::new()), |ui| stage.draw(ui)).1;
    }
    geometry
}

/// Judge the selected run against the plan's limits, as the app's frame does
/// before it draws.
pub fn refresh_verification(model: &mut Model) {
    model
        .simulation
        .refresh_verification(&model.sim_setup, &model.workspace.content);
}

fn attributed(analysis: AnalysisResult) -> AnalysisResult {
    attributed_to(analysis, AnalysisInstanceId::new(), ObjectRevision::INITIAL)
}

/// `analysis`, as the task of `source` expanded to the run set's `point`
/// produced it.
fn attributed_at(
    analysis: AnalysisResult,
    source: AnalysisInstanceId,
    point: crate::state::AnalysisResultPvtPoint,
) -> AnalysisResult {
    analysis.with_provenance(
        crate::state::AnalysisResultProvenance::new(
            source,
            ObjectRevision::INITIAL,
            ContentDigest::from_bytes([0x5a; 32]),
            Vec::new(),
        )
        .expect("fixture provenance is internally valid")
        .with_pvt_point(Some(point)),
    )
}

/// One point a run set expanded a task to. `TT 1.8 V 27 °C` is the nominal
/// one.
fn run_set_point(process: &str, volts: f64, celsius: f64) -> crate::state::AnalysisResultPvtPoint {
    let nominal = process == "TT" && volts == 1.8 && celsius == 27.0;
    crate::state::AnalysisResultPvtPoint::new(process, Some(volts), celsius, None, nominal)
        .expect("a finite point")
}

/// Make a completed Run 3 of `analyses` the model's one run and the run
/// Verify reads.
fn seed_analyses(model: &mut Model, analyses: Vec<AnalysisResult>) {
    let mut run = SimulationRun::new(3);
    run.restore_lifecycle(SimulationRunLifecycle::Completed, 12.4)
        .expect("a finite elapsed time");
    for analysis in analyses {
        run.add_analysis(analysis);
    }
    model.simulation.retained.runs = vec![run].into();
    model.simulation.view.active_run_idx = Some(0);
}

fn attributed_to(
    analysis: AnalysisResult,
    source_instance_id: AnalysisInstanceId,
    source_revision: ObjectRevision,
) -> AnalysisResult {
    analysis.with_provenance(
        crate::state::AnalysisResultProvenance::new(
            source_instance_id,
            source_revision,
            ContentDigest::from_bytes([0x5a; 32]),
            Vec::new(),
        )
        .expect("fixture provenance is internally valid"),
    )
}

/// A run of `plan_id`, sealed by a prepared receipt whose one task produced
/// `analysis`.
pub fn sealed_run(
    run_number: u64,
    plan_id: crate::product::SimulationPlanId,
    source_instance_id: AnalysisInstanceId,
    analysis_kind_tag: u8,
    analysis: AnalysisResult,
) -> SimulationRun {
    sealed_run_at(
        run_number,
        plan_id,
        (source_instance_id, ObjectRevision::INITIAL),
        analysis_kind_tag,
        analysis,
    )
}

/// [`sealed_run`] of the analysis `source` names at the plan revision it
/// names, as a run of an analysis the plan holds is sealed.
fn sealed_run_at(
    run_number: u64,
    plan_id: crate::product::SimulationPlanId,
    source: (AnalysisInstanceId, ObjectRevision),
    analysis_kind_tag: u8,
    analysis: AnalysisResult,
) -> SimulationRun {
    let (source_instance_id, source_revision) = source;
    let task = crate::state::PreparedRunTaskReceipt::new(
        source_instance_id,
        source_revision,
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
    run.add_analysis(attributed_to(analysis, source_instance_id, source_revision));
    run
}

pub(super) fn limit(
    measurement: &str,
    min: Option<f64>,
    max: Option<f64>,
    unit: &str,
) -> SpecEntry {
    SpecEntry {
        measurement: measurement.to_owned(),
        expression: String::new(),
        min,
        max,
        unit: unit.to_owned(),
        scope: SpecPointScope::AllPoints,
    }
}

/// Make Run 3, holding `analysis`, the model's one run and the run Verify reads.
fn seed_run(model: &mut Model, analysis: AnalysisResult) {
    let mut run = SimulationRun::new(3);
    run.restore_lifecycle(SimulationRunLifecycle::Completed, 12.4)
        .expect("a finite elapsed time");
    run.add_analysis(attributed(analysis));
    model.simulation.retained.runs = vec![run].into();
    model.simulation.view.active_run_idx = Some(0);
}

/// The seed the Monte Carlo fixture's campaign ran from.
const MONTE_CARLO_SEED: u64 = 0x1A2B;

/// A Monte Carlo analysis with one trial per sample, each of which measured
/// `gain` in dB.
fn gain_campaign(samples: Vec<f64>) -> AnalysisResult {
    campaign(
        MONTE_CARLO_SEED,
        samples
            .into_iter()
            .map(|sample| vec![("gain", "dB", sample)])
            .collect(),
    )
}

/// A Monte Carlo analysis seeded from `seed`, one trial per entry of
/// `trials`, each measuring its own (name, unit, value)s. Only an output
/// every trial measured is a variable of the family.
fn campaign(seed: u64, trials: Vec<Vec<(&str, &str, f64)>>) -> AnalysisResult {
    let count = trials.len();
    let names = trials
        .first()
        .map(|first| first.iter().map(|(name, _, _)| *name).collect::<Vec<_>>())
        .unwrap_or_default();
    let variables = names
        .iter()
        .filter_map(|name| {
            let samples = trials
                .iter()
                .map(|trial| {
                    trial
                        .iter()
                        .find(|(measured, _, _)| measured == name)
                        .map(|(_, _, value)| *value)
                })
                .collect::<Option<Vec<f64>>>()?;
            let mean = samples.iter().sum::<f64>() / count as f64;
            let std_dev = (samples
                .iter()
                .map(|sample| (sample - mean).powi(2))
                .sum::<f64>()
                / (count as f64 - 1.0))
                .sqrt();
            Some(MonteCarloVariableMetadata {
                mean_confidence: None,
                name: (*name).to_owned(),
                mean,
                std_dev,
                min: samples.iter().copied().fold(f64::INFINITY, f64::min),
                max: samples.iter().copied().fold(f64::NEG_INFINITY, f64::max),
                samples,
            })
        })
        .collect();
    let members = trials
        .iter()
        .enumerate()
        .map(|(index, measured)| {
            FamilyMemberMeasurements::new(
                FamilyMemberId::MonteCarloTrial {
                    index,
                    seed: seed + index as u64,
                },
                measured
                    .iter()
                    .map(|(name, unit, value)| FamilyMeasurementEvidence {
                        unit: Some(rspice_core::analysis::MeasurementUnit::Known(
                            (*unit).to_owned(),
                        )),
                        name: (*name).to_owned(),
                        value: Some(*value),
                        passed: true,
                        error: None,
                    })
                    .collect(),
            )
        })
        .collect();
    AnalysisResult::new(1, AnalysisType::MonteCarlo, "Monte Carlo").with_family_metadata(
        AnalysisResultFamilyMetadata::MonteCarlo {
            seed,
            runs_requested: count,
            runs_completed: count,
            failures: 0,
            all_converged: true,
            variables,
            member_measurements: members,
        },
    )
}

/// The Monte Carlo fixture's seeding: forty trials of the plan's Monte Carlo
/// analysis and its 40 dB gain floor, judged as the frame judges them.
pub fn seed_monte_carlo(model: &mut Model) {
    let plan = model
        .sim_setup
        .stable_analysis_plan_mut()
        .expect("default plan");
    let (source, _) = plan
        .insert(AnalysisKind::MonteCarlo)
        .expect("the plan takes a Monte Carlo analysis");
    let (plan_id, revision) = (plan.id(), plan.revision());
    model.sim_setup.refresh_legacy_analysis_projections();
    let samples = (0..40)
        .map(|index| match index {
            7 => 39.2,
            19 => 39.7,
            31 => 38.9,
            _ => 40.4 + f64::from(index % 9) * 0.35,
        })
        .collect::<Vec<f64>>();
    let mut run = sealed_run_at(
        3,
        plan_id,
        (source, revision),
        AnalysisKind::MonteCarlo.canonical_kind().tag(),
        gain_campaign(samples),
    );
    run.restore_lifecycle(SimulationRunLifecycle::Completed, 12.4)
        .expect("a finite elapsed time");
    model.simulation.retained.runs = vec![run].into();
    assert!(model.simulation.select_run(0));
    set_limits(model, vec![limit("gain", Some(40.0), None, "dB")]);
    model
        .simulation
        .refresh_yield_evidence(&model.sim_setup, &model.workspace.content);
    refresh_verification(model);
}

/// Forty gain trials at the nominal point and forty at SS 1.62 V 125 °C, in
/// one run: three fail at the nominal point and seven at the slow corner.
pub fn seed_monte_carlo_at_two_points(model: &mut Model) {
    let source = AnalysisInstanceId::new();
    let trials = |shift: f64, failing: &[usize], seed: u64| {
        let samples = (0..40)
            .map(|index| {
                let value = if failing.contains(&index) {
                    39.0 + f64::from(u8::try_from(index % 5).unwrap_or(0)) * 0.15
                } else {
                    40.4 + shift + f64::from(u8::try_from(index % 9).unwrap_or(0)) * 0.35
                };
                vec![("gain", "dB", value)]
            })
            .collect();
        campaign(seed, samples)
    };
    seed_analyses(
        model,
        vec![
            attributed_at(
                trials(0.0, &[7, 19, 31], MONTE_CARLO_SEED),
                source,
                run_set_point("TT", 1.8, 27.0),
            ),
            attributed_at(
                trials(-0.2, &[2, 9, 14, 22, 27, 33, 38], MONTE_CARLO_SEED + 0x100),
                source,
                run_set_point("SS", 1.62, 125.0),
            ),
        ],
    );
    set_limits(model, vec![limit("gain", Some(40.0), None, "dB")]);
    refresh_verification(model);
}

/// Forty trials that all measured gain, of which only the first thirty
/// measured supply current: the two limits were judged over different
/// trials.
pub fn seed_monte_carlo_over_different_trials(model: &mut Model) {
    let trials = (0..40)
        .map(|index| {
            let mut measured = vec![(
                "gain",
                "dB",
                40.4 + f64::from(u8::try_from(index % 9).unwrap_or(0)) * 0.35,
            )];
            if index < 30 {
                measured.push((
                    "idd",
                    "A",
                    0.8e-3 + f64::from(u8::try_from(index % 7).unwrap_or(0)) * 0.02e-3,
                ));
            }
            measured
        })
        .collect();
    seed_analyses(model, vec![attributed(campaign(MONTE_CARLO_SEED, trials))]);
    set_limits(
        model,
        vec![
            limit("gain", Some(40.0), None, "dB"),
            limit("idd", None, Some(1.2e-3), "A"),
        ],
    );
    refresh_verification(model);
}

/// A run set's sweep with no corner analysis: one transient analysis at each
/// point of `processes` × `supplies` × `temperatures`, every one measuring
/// gain and supply current, against a 40 dB floor and a 1.2 mA ceiling. Gain
/// fails at SS 1.62 V 125 °C and the current sits near its ceiling at FF
/// 1.98 V 125 °C, when the sweep holds those points.
pub fn seed_run_set(model: &mut Model, processes: &[&str], supplies: &[f64], temperatures: &[f64]) {
    let source = AnalysisInstanceId::new();
    let mut analyses = Vec::new();
    for process in processes {
        for volts in supplies {
            for celsius in temperatures {
                let (process, volts, celsius) = (*process, *volts, *celsius);
                let gain = if (process, volts, celsius) == ("SS", 1.62, 125.0) {
                    39.4
                } else {
                    43.0 + 0.3 * (volts - 1.8) / 0.18 - 0.002 * (celsius - 27.0)
                };
                let idd = if (process, volts, celsius) == ("FF", 1.98, 125.0) {
                    1.16e-3
                } else {
                    0.8e-3 + 0.05e-3 * (volts - 1.8) / 0.18 + 0.0005e-3 * (celsius - 27.0)
                };
                let id = analyses.len() as u64 + 1;
                analyses.push(attributed_at(
                    AnalysisResult::new(id, AnalysisType::Transient, "Transient")
                        .with_measurements(vec![
                            rspice_core::MeasureResult::success("gain", gain),
                            rspice_core::MeasureResult::success("idd", idd),
                        ]),
                    source,
                    run_set_point(process, volts, celsius),
                ));
            }
        }
    }
    seed_analyses(model, analyses);
    set_limits(
        model,
        vec![
            limit("gain", Some(40.0), None, "dB"),
            limit("idd", None, Some(1.2e-3), "A"),
        ],
    );
    refresh_verification(model);
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

/// The corner fixture's seeding: its run is the model's one run, and the
/// limits its outputs are judged by.
pub fn seed_corner(model: &mut Model) {
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
            RetainedTrace::new(
                "V(out)",
                axis.clone(),
                vec![0.912, 0.884, 0.901, 0.934, 0.951, 0.903, 0.915],
                "#4c9be8",
            ),
            RetainedTrace::new(
                "V(bias)",
                axis,
                vec![0.612, 0.655, 0.641, 0.578, 0.590, 0.620, 0.606],
                "#d0a03c",
            ),
        ]);
    seed_run(model, analysis);
    set_limits(
        model,
        vec![
            limit("gain", Some(40.0), None, "dB"),
            limit("bw", Some(10.0e6), None, "Hz"),
            limit("pm", Some(45.0), None, "deg"),
            limit("idd", None, Some(1.2e-3), "A"),
        ],
    );
    refresh_verification(model);
}

/// Make `specs` the limits the active plan judges its outputs by.
pub(super) fn set_limits(model: &mut Model, specs: Vec<SpecEntry>) {
    match model
        .sim_setup
        .stable_analysis_plan()
        .ok()
        .map(|plan| plan.id())
    {
        Some(plan_id) => model.workspace.content.replace_active_specs(plan_id, specs),
        None => model.workspace.content.specs = specs,
    }
}

/// An SOA run of five checks on four devices: two violations, one check near
/// its limit, two clean. Its transient checked two `.ASSERT` cards, one broken
/// twice, and reported an HDL `$warning`. The run is the model's one run, and
/// its checks are read as the frame reads them.
pub fn seed_soa(model: &mut Model) {
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
            RetainedTrace::new(
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
                super::checks::soa_parameter_display(parameter)
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
    seed_run(
        model,
        AnalysisResult::new(1, AnalysisType::Soa, "SOA")
            .with_family_metadata(AnalysisResultFamilyMetadata::Soa { time })
            .with_waveforms(waveforms)
            .with_result_payload(crate::state::AnalysisResultPayload::Soa {
                source_history: None,
                evaluations,
                violations,
            }),
    );
    model.simulation.retained.runs[0].add_analysis(attributed(checked_transient()));
    model
        .simulation
        .refresh_verification(&model.sim_setup, &model.workspace.content);
}

/// The SOA fixture's transient: an `.ASSERT` at error level that `V(out)`
/// broke twice, one that held, and an HDL `$warning`.
fn checked_transient() -> AnalysisResult {
    use rspice_core::analysis::{AssertViolation, AssertionReport};
    use rspice_core::config::{HdlOrigin, HdlPoint, HdlSeverity};
    use rspice_core::netlist::{AssertAnalysis, AssertLevel, NetlistSourceLocation};

    let report = |name: &str, line, violations| AssertionReport {
        name: name.to_owned(),
        analysis: AssertAnalysis::Tran,
        target: "V(out)".to_owned(),
        instance: None,
        level: AssertLevel::Error,
        message: Some("out left its band".to_owned()),
        origin: NetlistSourceLocation::in_memory(line),
        violations,
        unknown_stretches: 0,
        error: None,
        after_stop: None,
    };
    let stretch = |start: f64, end: f64, extreme: f64| AssertViolation {
        start: Some(start),
        end: Some(end),
        extreme: Some(extreme),
        at: Some((start + end) / 2.0),
    };
    let time = (0..64)
        .map(|index| f64::from(index) * 1.0e-8)
        .collect::<Vec<_>>();
    let out = time
        .iter()
        .map(|time| 1.2 + 0.7 * (time * 2.0e7).sin())
        .collect::<Vec<_>>();
    let mut analysis =
        AnalysisResult::new(2, AnalysisType::Transient, "Transient").with_waveforms(vec![
            RetainedTrace::new("V(out)", time, out, "#00aaff").with_unit("V"),
        ]);
    analysis.checks.assertions = vec![
        report(
            "out_band",
            12,
            vec![stretch(1.2e-7, 1.5e-7, 1.9), stretch(3.4e-7, 3.55e-7, 1.86)],
        ),
        report("out_floor", 13, Vec::new()),
    ];
    analysis.checks.hdl = vec![rspice_results::analysis_result::HdlDiagnostic {
        severity: HdlSeverity::Warning,
        text: "bias settled late\n".to_owned(),
        point: HdlPoint::Transient { time: 2.5e-7 },
        origin: HdlOrigin {
            scope: "xtb.ubias".to_owned(),
            file: "tb.v".to_owned(),
            line: 41,
        },
    }];
    analysis
}

/// The optimization fixture's seeding: its run is the model's one run.
pub fn seed_optimization(model: &mut Model) {
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
    seed_run(
        model,
        AnalysisResult::new(1, AnalysisType::Optimization, "Optimization")
            .with_waveforms(vec![
                RetainedTrace::new("OPT_COST", iterations.clone(), cost, "#4c9be8"),
                RetainedTrace::new("OPT_W1", iterations.clone(), width.clone(), "#d0a03c"),
                RetainedTrace::new("OPT_IBIAS", iterations.clone(), bias.clone(), "#8bc34a"),
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
    );
}

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

/// A transient step: three microseconds of the output at 12.5 ns, and its
/// measurements, each in the unit its producer states (none for a ratio).
fn step_analysis(
    damping: f64,
    natural_rad_s: f64,
    measurements: &[(&str, f64, &str)],
) -> AnalysisResult {
    use rspice_core::analysis::{MeasurementUnit, MeasurementUnits};
    let measured = |(name, value, unit): &(&str, f64, &str)| {
        let mut measurement = rspice_core::MeasureResult::success(name, *value);
        if !unit.is_empty() {
            measurement.units = Some(MeasurementUnits {
                value: MeasurementUnit::Known((*unit).to_owned()),
                raw_value: MeasurementUnit::Known((*unit).to_owned()),
                axis: MeasurementUnit::Known("s".to_owned()),
            });
        }
        measurement
    };
    let time = (0..=240)
        .map(|index| f64::from(index) * 12.5e-9)
        .collect::<Vec<_>>();
    AnalysisResult::new(1, AnalysisType::Transient, "Step")
        .with_waveforms(vec![
            RetainedTrace::new(
                "V(out)",
                time.clone(),
                step_response(damping, natural_rad_s, &time),
                "#4c9be8",
            )
            .with_unit("V"),
        ])
        .with_measurements(measurements.iter().map(measured).collect())
}

/// The Reference fixture's seeding: Run 4 of the plan's transient read
/// against its reference, Run 3 — four measurements and the output waveform,
/// under the plan's tolerances. Bandwidth breaks its 2 % rule and overshoot
/// has no rule.
pub fn seed_reference(model: &mut Model) {
    use crate::state::{
        RegressionComparisonMethod, RegressionComparisonWindow, RegressionSpecificationPolicy,
        RegressionTargetKind, RegressionTargetSelector, RegressionToleranceRule, RunRetention,
    };

    let plan = model
        .sim_setup
        .stable_analysis_plan_mut()
        .expect("default plan");
    let (source, _) = plan
        .insert(AnalysisKind::Transient)
        .expect("the plan takes a transient");
    let (plan_id, revision) = (plan.id(), plan.revision());
    model.sim_setup.refresh_legacy_analysis_projections();
    let run = |number: u64, damping: f64, natural: f64, values: [f64; 4]| {
        let mut run = sealed_run_at(
            number,
            plan_id,
            (source, revision),
            AnalysisKind::Transient.canonical_kind().tag(),
            step_analysis(
                damping,
                natural,
                &[
                    ("gain", values[0], "dB"),
                    ("bw", values[1], "Hz"),
                    ("tsettle", values[2], "s"),
                    ("overshoot", values[3], ""),
                ],
            ),
        );
        run.restore_lifecycle(SimulationRunLifecycle::Completed, 3.1)
            .expect("a finite elapsed time");
        run
    };
    let reference = run(3, 0.55, 4.49e6, [42.30, 18.2e6, 1.62e-6, 12.6]);
    let current = run(4, 0.56, 4.45e6, [42.28, 17.1e6, 1.64e-6, 12.1]);
    let rule = |target: RegressionTargetSelector, absolute: f64, relative: f64| {
        let waveform = target.kind == RegressionTargetKind::Waveform;
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
    let mut rules = super::derive_regression_checks(&reference, &current)
        .into_iter()
        .filter_map(|check| match check.name.as_str() {
            "gain" => Some(rule(check.target, 0.1, 0.005)),
            "bw" => Some(rule(check.target, 0.0, 0.02)),
            "tsettle" => Some(rule(check.target, 0.0, 0.05)),
            _ => None,
        })
        .collect::<Vec<_>>();
    let waveform = super::regression_waveform_pairs(&reference, &current)
        .first()
        .map(|pair| pair.target.clone())
        .expect("both runs hold V(out)");
    rules.push(rule(waveform, 0.02, 0.02));
    let reference_id = reference.run_id;
    let payload = model.workspace.content.ensure_active_plan_data(plan_id);
    payload.regression_tolerances = rules;
    payload.specification_policy.regression = RegressionSpecificationPolicy::LimitAndWaveform;
    payload.regression_baseline_run = Some(reference_id);
    model.simulation.retained.runs = vec![reference, current].into();
    assert!(
        model
            .simulation
            .set_run_retention(reference_id, RunRetention::GoldenBaseline)
    );
    assert!(model.simulation.select_run(1));
}

/// The yield evidence memo of the run Verify reads, which the inspector and
/// the session still read.
pub fn fixture_yield_results(model: &Model) -> Vec<rspice_results::yield_analysis::YieldResult> {
    model
        .simulation
        .active_run()
        .and_then(|run| {
            model
                .simulation
                .retained
                .yield_results_for_dataset(run.dataset_id)
        })
        .map(<[_]>::to_vec)
        .unwrap_or_default()
}
