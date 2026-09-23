//! Frozen configured-analysis execution on each study circuit.

mod analysis;
pub(crate) mod monte_carlo;
pub use analysis::StudyAnalysis;
#[cfg(test)]
pub(crate) use monte_carlo::run_monte_carlo;
mod hb;
mod optimization;
mod periodic;
mod pss;
mod qpss;
pub use hb::StudyHbConfig;
pub use periodic::StudyPeriodicOptions;
pub use pss::{StudyOperatingPoint, StudyPssConfig};
pub use qpss::StudyQpssConfig;
mod spectral;
pub(crate) use optimization::run_optimization;
pub use spectral::StudyPostprocess;

use super::{AnalysisExecutionEnvironment, SimulationError};
use crate::product::{AnalysisInstanceId, ObjectRevision};
use crate::services::simulation_runner as services;
use crate::simulation::dialog::McVariationSource;
use crate::simulation::{config::AnalysisConfig, engine_bridge::EngineBridge, plan::AnalysisKind};
use rspice_core::abort_signal::{AbortReason, AbortSignal, ModelRunControl};
#[cfg(test)]
use rspice_core::analysis::monte_carlo::Distribution;
use rspice_core::engine::MonteCarloEnvironment;
#[cfg(test)]
use rspice_core::engine::MonteCarloStudyConfig;
use std::path::Path;
use std::sync::{
    Mutex,
    atomic::{AtomicBool, Ordering},
};

/// A study captures the selected instance's configuration before dispatch.
/// Its Run Set point belongs to the study; its solver controls belong to the base.
#[derive(Debug, Clone)]
pub struct StudyRunConfig {
    /// Optional consumer of `analysis`, which is its exact transient, PSS or HB producer.
    pub postprocess: Option<StudyPostprocess>,
    pub constraints: Vec<crate::simulation::optimizer::OptimizationConstraint>,
    pub objective_terms: Vec<crate::simulation::optimizer::OptimizationObjectiveTerm>,
    pub instance_id: AnalysisInstanceId,
    pub source_revision: ObjectRevision,
    pub analysis: StudyAnalysis,
    pub analysis_line: String,
    pub numeric_options: String,
    pub measurements: Vec<String>,
    pub histogram_bins: usize,
}

pub(crate) fn supports_kind(kind: AnalysisKind) -> bool {
    matches!(
        kind,
        AnalysisKind::OperatingPoint
            | AnalysisKind::DcSweep
            | AnalysisKind::Transient
            | AnalysisKind::Ac
            | AnalysisKind::Noise
            | AnalysisKind::PoleZero
            | AnalysisKind::Sensitivity
            | AnalysisKind::Fourier
            | AnalysisKind::Fft
            | AnalysisKind::HarmonicBalance
            | AnalysisKind::Hbsp
            | AnalysisKind::Hbnoise
            | AnalysisKind::Pss
            | AnalysisKind::Pac
            | AnalysisKind::Pxf
            | AnalysisKind::Pnoise
            | AnalysisKind::Pstb
            | AnalysisKind::Psp
            | AnalysisKind::Qpss
            | AnalysisKind::Qpac
            | AnalysisKind::Qpxf
            | AnalysisKind::Qpnoise
    )
}

pub(crate) fn validate_measurements(names: &[String]) -> Result<(), String> {
    if names.is_empty() {
        return Err("Select at least one study measurement".into());
    }
    let mut seen = std::collections::HashSet::new();
    for name in names {
        let (mode, key) = name.split_once(':').unwrap_or(("meas", name));
        if key.trim().is_empty()
            || name.chars().any(char::is_control)
            || !["meas", "scalar", "last", "bin", "tuple"]
                .iter()
                .any(|value| mode.eq_ignore_ascii_case(value))
        {
            return Err(format!(
                "Invalid study measurement {name:?}; use a .MEAS name, scalar:name, last:signal, bin:index:quantity[:signal], or tuple:k1,k2:quantity:signal"
            ));
        }
        if mode.eq_ignore_ascii_case("bin") {
            crate::simulation::results::parse_study_bin(key)?;
        }
        if mode.eq_ignore_ascii_case("tuple") {
            crate::simulation::results::parse_study_tuple(key)?;
        }
        if !seen.insert(name.to_ascii_lowercase()) {
            return Err(format!("Repeated study measurement {name:?}"));
        }
    }
    Ok(())
}

struct StudyAbort<'a> {
    parent: &'a dyn AbortSignal,
    failed: AtomicBool,
}
impl AbortSignal for StudyAbort<'_> {
    fn is_aborted(&self) -> bool {
        self.failed.load(Ordering::Acquire) || self.parent.is_aborted()
    }
    fn abort_reason(&self) -> AbortReason {
        self.parent.abort_reason()
    }
    fn model_control(&self) -> Option<&ModelRunControl> {
        self.parent.model_control()
    }
    fn observe_progress(&self, fraction: f64) {
        self.parent.observe_progress(fraction);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::simulation::config::{AcAnalysisConfig, AcSweepType, TransientAnalysisConfig};
    use crate::simulation::runner::{
        SpecExecutionOptions, worker_contract::WorkerSpecExecutionOptions,
    };
    use crate::simulation::{multi_run::AnalysisSpec, results::SimulationResult};
    use rspice_core::abort_signal::{ImmediateAbort, NoAbort};

    fn base(analysis: AnalysisConfig, names: &[&str]) -> StudyRunConfig {
        StudyRunConfig {
            postprocess: None,
            constraints: Vec::new(),
            objective_terms: Vec::new(),
            instance_id: AnalysisInstanceId::new(),
            source_revision: ObjectRevision::INITIAL,
            analysis_line: analysis.to_spice(),
            analysis: analysis.into(),
            numeric_options: ".OPTIONS RELTOL=1e-5".into(),
            measurements: names.iter().map(|name| (*name).to_owned()).collect(),
            histogram_bins: 7,
        }
    }
    fn ac() -> AnalysisConfig {
        AnalysisConfig::Ac(AcAnalysisConfig {
            start_freq: 1000.0,
            stop_freq: 1000.0,
            num_points: 1,
            sweep_type: AcSweepType::Linear,
            ..Default::default()
        })
    }
    fn spec(source: McVariationSource) -> AnalysisSpec {
        AnalysisSpec::MonteCarlo {
            variation_source: source,
            params: vec![],
        }
    }

    #[test]
    fn configured_study_worker_dispatch_runs_each_selected_base_on_varied_circuits() {
        let parameter_deck = "Configured study\n.param rval=1k\nV1 in 0 DC 1 AC 1\nR1 in out {rval}\nC1 out 0 1u\n.ic V(out)=0\n.meas AC gain FIND VM(out) AT=1k\n.meas AC bounded FIND VM(out) AT=1k GOAL=1000 TOL=0.01\n.meas TRAN settled FIND V(out) AT=1m\n.mc 3 uniform 0.2 seed 37\n.end\n";
        for (analysis, names) in [
            (ac(), vec!["gain", "last:V(out)", "bounded"]),
            (
                AnalysisConfig::Transient(TransientAnalysisConfig {
                    stop_time: 1e-3,
                    step_time: 1e-5,
                    start_time: 2e-4,
                    max_timestep: Some(1e-5),
                    uic: true,
                }),
                vec!["settled", "last:V(out)"],
            ),
            (AnalysisConfig::dc_op(), vec!["scalar:V(in)"]),
        ] {
            for source in McVariationSource::ALL {
                let deck = if source == McVariationSource::DeckStatistics {
                    parameter_deck.replace(".param rval=1k", ".param rval={unif(1k,0.2)}")
                } else {
                    parameter_deck.to_owned()
                };
                let options = SpecExecutionOptions {
                    study_base: Some(base(analysis.clone(), &names)),
                    ..Default::default()
                };
                let wire = WorkerSpecExecutionOptions::from(&options);
                let encoded = serde_json::to_string(&wire).unwrap();
                let decoded: WorkerSpecExecutionOptions = serde_json::from_str(&encoded).unwrap();
                assert_eq!(wire, decoded);
                let result = super::super::spec::run_spec_request_with_environment(
                    &EngineBridge::new(),
                    spec(source),
                    decoded.into(),
                    &deck,
                    None,
                    &crate::simulation::execution::ResolvedExecutionDependencies::default(),
                    Some(AnalysisExecutionEnvironment {
                        temperature_celsius: 75.0,
                        supply_voltage: Some(1.8),
                        nominal_supply_voltage: Some(1.0),
                        supply_source_names: vec!["V1".into()],
                    }),
                    &NoAbort,
                )
                .unwrap();
                let SimulationResult::MonteCarlo {
                    variables,
                    runs_completed,
                    num_failures,
                    member_measurements,
                    ..
                } = result
                else {
                    panic!("MC result")
                };
                assert_eq!(runs_completed, 3);
                assert_eq!(num_failures, 0);
                assert_eq!(variables.len(), names.len());
                assert_eq!(member_measurements.len(), 3);
                crate::state::FamilyMemberMeasurements::validate_monte_carlo_sequence(
                    &member_measurements,
                    37,
                    3,
                    3,
                    0,
                    variables
                        .iter()
                        .map(|variable| (variable.name.as_str(), variable.samples.as_slice())),
                )
                .unwrap();
                if matches!(analysis, AnalysisConfig::Ac(_)) {
                    let bounded = variables
                        .iter()
                        .find(|variable| variable.name == "bounded")
                        .unwrap();
                    let gain = variables
                        .iter()
                        .find(|variable| variable.name == "gain")
                        .unwrap();
                    assert_eq!(bounded.samples, gain.samples);
                    for trial in &member_measurements {
                        let measured = trial
                            .measurements
                            .iter()
                            .find(|measurement| measurement.name == "bounded")
                            .unwrap();
                        assert!(measured.value.is_some());
                        assert!(!measured.passed);
                        assert!(
                            measured
                                .error
                                .as_ref()
                                .is_some_and(|error| error.contains("GOAL"))
                        );
                    }
                }
                for variable in &variables {
                    assert_eq!(
                        variable.histogram.len(),
                        if variable.min == variable.max { 1 } else { 7 }
                    );
                    assert_eq!(variable.samples.len(), 3);
                    assert!(variable.mean_confidence.is_some());
                }
                let first = variables
                    .iter()
                    .find(|variable| variable.name == names[0])
                    .unwrap();
                match &analysis {
                    AnalysisConfig::DcOp(_) => assert!(
                        first
                            .samples
                            .iter()
                            .all(|value| (*value - 1.8).abs() < 1e-10)
                    ),
                    AnalysisConfig::Ac(_) => {
                        assert!(first.samples.windows(2).any(|pair| pair[0] != pair[1]));
                        let real = variables
                            .iter()
                            .find(|variable| variable.name == "last:V(out)")
                            .unwrap();
                        for (magnitude, real) in first.samples.iter().zip(&real.samples) {
                            assert!((magnitude * magnitude - real).abs() < 1e-10);
                            assert!(*magnitude > 0.12 && *magnitude < 0.21);
                        }
                    }
                    AnalysisConfig::Transient(_) => {
                        assert!(first.samples.windows(2).any(|pair| pair[0] != pair[1]));
                        let last = variables
                            .iter()
                            .find(|variable| variable.name == "last:V(out)")
                            .unwrap();
                        for (measured, last) in first.samples.iter().zip(&last.samples) {
                            assert!((measured - last).abs() < 1e-7);
                            assert!(*measured > 0.9 && *measured < 1.4);
                        }
                    }
                    _ => unreachable!(),
                }
            }
        }
    }

    #[test]
    fn configured_study_rejects_missing_measurements_and_preserves_abort_and_limits() {
        let deck =
            "Study\n.param r=1k\nV1 in 0 AC 1\nR1 in 0 {r}\n.mc 2 uniform 0.01 seed 7\n.end\n";
        let mut configured = base(ac(), &["absent"]);
        let error = run_monte_carlo(
            &configured,
            McVariationSource::ParameterTolerance,
            deck,
            None,
            None,
            &NoAbort,
        )
        .unwrap_err();
        assert!(
            matches!(error, SimulationError::InvalidConfig(ref message) if message.contains("absent")),
            "{error}"
        );
        configured.measurements = vec!["last:V(in)".into()];
        configured.histogram_bins = rspice_core::SimulationConfig::default()
            .resource_limits
            .max_analysis_points
            + 1;
        assert!(matches!(
            run_monte_carlo(
                &configured,
                McVariationSource::ParameterTolerance,
                deck,
                None,
                None,
                &NoAbort
            ),
            Err(SimulationError::ResourceLimit { .. })
        ));
        assert!(matches!(
            run_monte_carlo(
                &configured,
                McVariationSource::ParameterTolerance,
                deck,
                None,
                None,
                &ImmediateAbort
            ),
            Err(SimulationError::Aborted)
        ));
        configured.histogram_bins = 7;
        configured.measurements = vec!["in".into()];
        let failed = deck.replace(".end", ".meas AC in FIND VM(in) AT=9k\n.end");
        let error = run_monte_carlo(
            &configured,
            McVariationSource::ParameterTolerance,
            &failed,
            None,
            None,
            &NoAbort,
        )
        .unwrap_err();
        assert!(
            matches!(error, SimulationError::CircuitError(ref message) if message.contains("All 2") && message.contains("in")),
            "{error}"
        );
    }
    #[test]
    fn custom_monte_carlo_statistics_reach_native_trials_and_worker_transport() {
        use crate::simulation::dialog::mc::statistics::{
            McParameterCorrelation, McParameterVariation, McScope, McShape, McStatisticsConfig,
        };
        let deck = "Custom statistics\n.param X=1 Y=2 Z=0\nV1 a 0 {X}\nV2 b 0 {Y}\nV3 c 0 {Z}\nR1 a 0 1k\nR2 b 0 1k\nR3 c 0 1k\n.mc 24 seed 18446744073709551615\n.end\n";
        for (shape, scope, correlation) in [
            (McShape::Gaussian, McScope::Process, 1.0),
            (McShape::Gaussian, McScope::Process, -1.0),
            (McShape::Uniform, McScope::Process, 1.0),
            (McShape::Uniform, McScope::Process, -1.0),
            (McShape::Lognormal, McScope::Process, 1.0),
            (McShape::Gaussian, McScope::Mismatch, 1.0),
        ] {
            let mut statistics = McStatisticsConfig {
                variations: vec![
                    McParameterVariation {
                        bounds: None,
                        parameter: "X".into(),
                        scope,
                        distribution: shape,
                        spread: if shape == McShape::Lognormal {
                            0.1
                        } else {
                            10.0
                        },
                        percent: shape != McShape::Lognormal,
                    },
                    McParameterVariation {
                        bounds: None,
                        parameter: "Y".into(),
                        scope,
                        distribution: shape,
                        spread: if shape == McShape::Lognormal {
                            0.1
                        } else {
                            10.0
                        },
                        percent: shape != McShape::Lognormal,
                    },
                    McParameterVariation {
                        bounds: None,
                        parameter: "Z".into(),
                        scope,
                        distribution: McShape::Gaussian,
                        spread: 0.01,
                        percent: false,
                    },
                ],
                correlations: vec![McParameterCorrelation {
                    scope,
                    parameters: vec!["X".into(), "Y".into()],
                    coefficient: correlation,
                }],
            };
            for (index, row) in statistics.variations.iter_mut().enumerate() {
                let (lower, upper) = [(0.98, 1.02), (1.96, 2.04), (-0.005, 0.005)][index];
                row.bounds = Some(
                    crate::simulation::dialog::mc::statistics::McParameterBounds {
                        lower: Some(lower),
                        upper: Some(upper),
                        sigma_cutoff: (row.distribution != McShape::Uniform).then_some(0.75),
                        max_attempts: 10_000,
                    },
                );
            }
            let mut previous: Option<Vec<Vec<f64>>> = None;
            for configured in [false, true] {
                let options = SpecExecutionOptions {
                    mc_statistics: Some(statistics.clone()),
                    study_base: configured.then(|| {
                        base(
                            AnalysisConfig::dc_op(),
                            &["scalar:V(a)", "scalar:V(b)", "scalar:V(c)"],
                        )
                    }),
                    ..Default::default()
                };
                let wire = WorkerSpecExecutionOptions::from(&options);
                let wire: WorkerSpecExecutionOptions =
                    serde_json::from_str(&serde_json::to_string(&wire).unwrap()).unwrap();
                let result = super::super::spec::run_spec_request(
                    &EngineBridge::new(),
                    spec(McVariationSource::DeckStatistics),
                    wire.into(),
                    deck,
                    None,
                    &crate::simulation::execution::ResolvedExecutionDependencies::default(),
                    &NoAbort,
                )
                .unwrap();
                let SimulationResult::MonteCarlo {
                    variables,
                    runs_completed,
                    member_measurements,
                    ..
                } = result
                else {
                    unreachable!()
                };
                assert_eq!(runs_completed, 24, "{shape:?}/{scope:?}");
                assert_eq!(member_measurements.len(), 24);
                let samples = ["a", "b", "c"]
                    .iter()
                    .map(|node| {
                        let suffix = format!("V({node})");
                        variables
                            .iter()
                            .find(|value| {
                                value
                                    .name
                                    .rsplit(':')
                                    .next()
                                    .unwrap_or(&value.name)
                                    .eq_ignore_ascii_case(&suffix)
                            })
                            .unwrap_or_else(|| {
                                panic!(
                                    "missing {suffix}: {:?}",
                                    variables
                                        .iter()
                                        .map(|value| &value.name)
                                        .collect::<Vec<_>>()
                                )
                            })
                            .samples
                            .clone()
                    })
                    .collect::<Vec<_>>();
                assert!(samples[0].windows(2).any(|pair| pair[0] != pair[1]));
                for (samples, (lower, upper)) in
                    samples
                        .iter()
                        .zip([(0.98, 1.02), (1.96, 2.04), (-0.005, 0.005)])
                {
                    assert!(
                        samples
                            .iter()
                            .all(|value| *value >= lower && *value <= upper)
                    );
                }
                assert!(
                    samples[2].iter().any(|value| value.abs() > 1e-5),
                    "absolute spread must vary a zero nominal"
                );
                if scope == McScope::Process {
                    let expected = |a: f64| 2.0 + correlation * 2.0 * (a - 1.0);
                    assert!(
                        samples[0]
                            .iter()
                            .zip(&samples[1])
                            .all(|(a, b)| (expected(*a) - b).abs() < 1e-11),
                        "{shape:?}/{scope:?}, correlation={correlation}, configured={configured}: {:?}",
                        samples[0]
                            .iter()
                            .zip(&samples[1])
                            .find(|(a, b)| (expected(**a) - *b).abs() >= 1e-11)
                    );
                } else {
                    assert!(
                        samples[0]
                            .iter()
                            .zip(&samples[1])
                            .any(|(a, b)| (2.0 * a - b).abs() > 1e-4),
                        "mismatch scopes must draw independently for separate instances"
                    );
                }
                if shape == McShape::Uniform {
                    assert!(samples[0].iter().all(|value| (0.9..=1.1).contains(value)));
                }
                if shape == McShape::Lognormal {
                    assert!(samples[0].iter().all(|value| *value > 0.0));
                }
                if let Some(previous) = previous {
                    for (before, after) in previous.iter().zip(&samples) {
                        assert!(before.iter().zip(after).all(|(a, b)| (a - b).abs() < 1e-12));
                    }
                }
                previous = Some(samples);
            }
        }
    }
    #[test]
    fn monte_carlo_trial_ranges_replay_each_sampler_through_studio_dispatch() {
        use crate::simulation::dialog::mc::statistics::{
            McParameterVariation, McScope, McShape, McStatisticsConfig,
        };
        for source in 0..3 {
            let expression = if source == 1 {
                "{agauss(1, 0.1, 1)}"
            } else {
                "1"
            };
            for configured in [false, true] {
                let options = SpecExecutionOptions {
                    mc_statistics: (source == 2).then(|| McStatisticsConfig {
                        variations: vec![McParameterVariation {
                            bounds: Some(
                                crate::simulation::dialog::mc::statistics::McParameterBounds {
                                    lower: Some(0.99),
                                    upper: Some(1.01),
                                    sigma_cutoff: Some(1.0),
                                    max_attempts: 10_000,
                                },
                            ),
                            parameter: "X".into(),
                            scope: McScope::Mismatch,
                            distribution: McShape::Gaussian,
                            spread: 0.1,
                            percent: false,
                        }],
                        correlations: vec![],
                    }),
                    study_base: configured
                        .then(|| base(AnalysisConfig::dc_op(), &["scalar:V(out)"])),
                    ..Default::default()
                };
                let run = |first, count| {
                    let wire = WorkerSpecExecutionOptions::from(&options);
                    let wire: WorkerSpecExecutionOptions =
                        serde_json::from_str(&serde_json::to_string(&wire).unwrap()).unwrap();
                    let deck = format!(
                        "ranges\n.param X={expression}\nV1 out 0 {{X}}\nR1 out 0 1k\n.mc {count} START {first} SEED 18446744073709551615 DIST GAUSS SPREAD 0.1\n.end\n"
                    );
                    super::super::spec::run_spec_request(
                        &EngineBridge::new(),
                        spec(if source == 0 {
                            McVariationSource::ParameterTolerance
                        } else {
                            McVariationSource::DeckStatistics
                        }),
                        wire.into(),
                        &deck,
                        None,
                        &crate::simulation::execution::ResolvedExecutionDependencies::default(),
                        &NoAbort,
                    )
                    .unwrap()
                };
                let full = run(0, 8);
                let batch = run(3, 3);
                let (
                    SimulationResult::MonteCarlo {
                        variables: full,
                        member_measurements: full_members,
                        ..
                    },
                    SimulationResult::MonteCarlo {
                        variables: batch,
                        member_measurements: members,
                        runs_requested,
                        runs_completed,
                        ..
                    },
                ) = (full, batch)
                else {
                    panic!("MC");
                };
                assert_eq!((runs_requested, runs_completed), (3, 3));
                assert_eq!(
                    members
                        .iter()
                        .map(|member| member.member.index())
                        .collect::<Vec<_>>(),
                    [3, 4, 5]
                );
                for (index, member) in members.iter().enumerate() {
                    assert_eq!(member, &full_members[index + 3]);
                }
                for variable in &batch {
                    let reference = full
                        .iter()
                        .find(|candidate| candidate.name == variable.name)
                        .unwrap();
                    assert_eq!(
                        variable.samples.as_slice(),
                        &reference.samples[3..6],
                        "source={source}, configured={configured}"
                    );
                }
            }
        }
    }
}

fn validate_base_measurements(
    base: &StudyRunConfig,
    circuit: &rspice_core::Netlist,
) -> Result<(), SimulationError> {
    validate_measurements(&base.measurements).map_err(SimulationError::InvalidConfig)?;
    if let Some(postprocess) = &base.postprocess {
        return postprocess.validate_measurements(base);
    }
    if let StudyAnalysis::Pss(config) = &base.analysis {
        return config
            .validate_measurements(&base.measurements)
            .map_err(SimulationError::InvalidConfig);
    }
    if let StudyAnalysis::Qpss(config) = &base.analysis {
        return validate_qpss_measurements(&config.request, &base.measurements)
            .map_err(SimulationError::InvalidConfig);
    }
    if let StudyAnalysis::Native(spec @ crate::simulation::multi_run::AnalysisSpec::Qpss { .. }) =
        &base.analysis
    {
        return validate_qpss_measurements(spec, &base.measurements)
            .map_err(SimulationError::InvalidConfig);
    }
    let Some(analysis) = base.analysis.as_basic() else {
        if base.measurements.iter().any(|request| {
            !request
                .split_once(':')
                .is_some_and(|(mode, _)| mode.eq_ignore_ascii_case("bin"))
        }) {
            return Err(SimulationError::InvalidConfig(
                "Harmonic balance studies require bin:index:quantity[:signal] measurements".into(),
            ));
        }
        return Ok(());
    };
    let family = match analysis {
        AnalysisConfig::Ac(_) => "AC",
        AnalysisConfig::Transient(_) => "TRAN",
        AnalysisConfig::DcSweep(_) => "DC",
        AnalysisConfig::Noise(_) => "NOISE",
        _ => "",
    };
    for request in &base.measurements {
        let (mode, name) = request.split_once(':').unwrap_or(("meas", request));
        if mode.eq_ignore_ascii_case("tuple") {
            return Err(SimulationError::InvalidConfig(
                "Lattice observations require a QPSS base".into(),
            ));
        }
        if mode.eq_ignore_ascii_case("meas")
            && !circuit.measurements.iter().any(|measurement| {
                measurement.name.eq_ignore_ascii_case(name)
                    && measurement.analysis.eq_ignore_ascii_case(family)
            })
        {
            return Err(SimulationError::InvalidConfig(format!(
                "Study measurement {name:?} has no .MEAS {family} declaration for the selected base"
            )));
        }
        if mode.eq_ignore_ascii_case("scalar")
            && !matches!(
                analysis,
                AnalysisConfig::DcOp(_)
                    | AnalysisConfig::PoleZero(_)
                    | AnalysisConfig::Sensitivity(_)
                    | AnalysisConfig::Noise(_)
            )
        {
            return Err(SimulationError::InvalidConfig(format!(
                "{request:?} requires a scalar analysis; use a .MEAS name or last:signal for a waveform"
            )));
        }
        if mode.eq_ignore_ascii_case("bin")
            && !matches!(analysis, AnalysisConfig::Ac(_) | AnalysisConfig::Noise(_))
        {
            return Err(SimulationError::InvalidConfig(
                "Spectral bin measurements require a frequency-domain analysis".into(),
            ));
        }
        if mode.eq_ignore_ascii_case("last") && family.is_empty() {
            return Err(SimulationError::InvalidConfig(format!(
                "{request:?} requires an analysis with waveforms"
            )));
        }
    }
    Ok(())
}

fn resolved_study_environment(
    base: &StudyRunConfig,
    environment: Option<AnalysisExecutionEnvironment>,
) -> (StudyAnalysis, Option<MonteCarloEnvironment>) {
    let mut environment = environment.map(|point| MonteCarloEnvironment {
        temperature_celsius: point.temperature_celsius,
        supply_voltage: point.supply_voltage,
        nominal_supply_voltage: point.nominal_supply_voltage,
        supply_source_names: point.supply_source_names,
    });
    // Adapt supply exactly once using the actual Run Set. A temperature-only
    // materialization context must not erase an OP's explicit supply settings.
    let analysis = analysis_for_environment(base, environment.as_ref());
    let temperature = match &analysis {
        StudyAnalysis::Basic(AnalysisConfig::DcOp(op)) => Some(op.temperature_celsius),
        StudyAnalysis::Pss(pss) => Some(pss.operating_point.config.temperature_celsius),
        StudyAnalysis::Qpss(qpss) => Some(qpss.operating_point.config.temperature_celsius),
        StudyAnalysis::Hb(hb) => Some(hb.operating_point.config.temperature_celsius),
        _ => None,
    };
    if let Some(temperature_celsius) = temperature {
        match &mut environment {
            Some(point) => point.temperature_celsius = temperature_celsius,
            None => {
                environment = Some(MonteCarloEnvironment {
                    temperature_celsius,
                    supply_voltage: None,
                    nominal_supply_voltage: None,
                    supply_source_names: Vec::new(),
                })
            }
        }
    }
    (analysis, environment)
}

fn study_source_at_environment(
    base: &StudyRunConfig,
    source: &str,
    environment: Option<&MonteCarloEnvironment>,
    abort: &dyn AbortSignal,
) -> Result<String, SimulationError> {
    let source = base.execution_source(source)?;
    match environment {
        Some(point) => super::spec::run_abort_aware_service(abort, || {
            services::source_with_run_temperature_with_abort(
                &source,
                point.temperature_celsius,
                abort,
            )
        }),
        None => Ok(source),
    }
}

fn analysis_for_environment(
    base: &StudyRunConfig,
    environment: Option<&MonteCarloEnvironment>,
) -> StudyAnalysis {
    let mut analysis = base.analysis.clone();
    let operating_point = match &mut analysis {
        StudyAnalysis::Pss(pss) => Some(&mut pss.operating_point.config),
        StudyAnalysis::Qpss(qpss) => Some(&mut qpss.operating_point.config),
        StudyAnalysis::Hb(hb) => Some(&mut hb.operating_point.config),
        _ => None,
    };
    if let Some(op) = operating_point {
        if let Some(environment) = environment {
            if matches!(
                op.temperature_mode,
                crate::simulation::dialog::OpTemperatureMode::PvtRunSet
                    | crate::simulation::dialog::OpTemperatureMode::ActiveRunSetAxis
            ) {
                op.temperature_celsius = environment.temperature_celsius;
            }
            op.run_point.supply_voltage = None;
            op.run_point.nominal_supply_voltage = None;
            op.run_point.supply_source_names = environment.supply_source_names.clone();
        }
        return analysis;
    }
    let Some(config) = analysis.as_basic_mut() else {
        return analysis;
    };
    if let AnalysisConfig::DcOp(op) = config {
        // The study applies the supply exactly once, after statistical replay.
        // Outside a Run Set retain the selected OP's explicit supply point.
        if let Some(environment) = environment {
            if matches!(
                op.temperature_mode,
                crate::simulation::dialog::OpTemperatureMode::PvtRunSet
                    | crate::simulation::dialog::OpTemperatureMode::ActiveRunSetAxis
            ) {
                op.temperature_celsius = environment.temperature_celsius;
            }
            op.run_point.supply_voltage = None;
            op.run_point.nominal_supply_voltage = None;
            op.run_point.supply_source_names = environment.supply_source_names.clone();
        }
    }
    if let (AnalysisConfig::Noise(noise), Some(environment)) = (config, environment) {
        noise.temperature_kelvin =
            rspice_core::constants::celsius_to_kelvin(environment.temperature_celsius);
    }
    analysis
}

fn validate_qpss_measurements(
    spec: &crate::simulation::multi_run::AnalysisSpec,
    measurements: &[String],
) -> Result<(), String> {
    let config = spec.driven_qpss_config()?;
    let grid = rspice_core::analysis::quasi_periodic::QuasiPeriodicGrid::new_with_abort(
        config.grid,
        &rspice_core::ResourceLimits::default(),
        &rspice_core::NoAbort,
    )
    .map_err(|error| error.to_string())?;
    for request in measurements {
        let (mode, key) = request.split_once(':').unwrap_or(("meas", request));
        if mode.eq_ignore_ascii_case("tuple") {
            let (tuple, _, _) = crate::simulation::results::parse_study_tuple(key)?;
            if grid.index_of(&tuple).is_none() {
                return Err(format!("QPSS does not retain lattice tuple {tuple:?}"));
            }
        } else if mode.eq_ignore_ascii_case("scalar") {
            if !["qpss.iterations", "qpss.normalized_residual"]
                .iter()
                .any(|name| key.eq_ignore_ascii_case(name))
            {
                return Err(
                    "QPSS scalar must be qpss.iterations or qpss.normalized_residual".into(),
                );
            }
        } else if !mode.eq_ignore_ascii_case("bin") && !mode.eq_ignore_ascii_case("last") {
            return Err("QPSS studies require tuple:k1,k2:quantity:signal, bin:index:quantity:signal, last:signal or scalar:qpss.iterations/normalized_residual".into());
        }
    }
    Ok(())
}

#[cfg(test)]
mod quasi_periodic_tests;

#[cfg(test)]
mod temperature_tests;
