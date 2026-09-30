//! Frozen study requests, source preparation and measurement validation.

mod analysis;
mod hb;
mod periodic;
mod pss;
mod qpss;
mod spectral;
pub use analysis::StudyAnalysis;
pub use hb::StudyHbConfig;
pub use periodic::StudyPeriodicOptions;
pub use pss::{StudyOperatingPoint, StudyPssConfig};
pub use qpss::StudyQpssConfig;
pub use spectral::StudyPostprocess;

use crate::error::SimulationError;
use rspice_app_types::product::{AnalysisInstanceId, ObjectRevision};
use rspice_core::engine::MonteCarloEnvironment;
use rspice_simulation_contract::config::AnalysisConfig;
use rspice_simulation_contract::study_measurement::validate_measurements;
use rspice_simulation_contract::worker_protocol::AnalysisExecutionEnvironment;

/// A study captures the selected instance's configuration before dispatch.
/// Its Run Set point belongs to the study; its solver controls belong to the base.
#[derive(Debug, Clone)]
pub struct StudyRunConfig {
    /// Optional consumer of `analysis`, which is its exact transient, PSS or HB producer.
    pub postprocess: Option<StudyPostprocess>,
    pub constraints: Vec<rspice_results::optimization::OptimizationConstraint>,
    pub objective_terms: Vec<rspice_results::optimization::OptimizationObjectiveTerm>,
    pub instance_id: AnalysisInstanceId,
    pub source_revision: ObjectRevision,
    pub analysis: StudyAnalysis,
    pub analysis_line: String,
    pub numeric_options: String,
    pub measurements: Vec<String>,
    pub histogram_bins: usize,
}

pub fn validate_base_measurements(
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
    if let StudyAnalysis::Native(
        spec @ rspice_simulation_contract::analysis_spec::AnalysisSpec::Qpss { .. },
    ) = &base.analysis
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
pub fn resolved_study_environment(
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
pub fn analysis_for_environment(
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
                rspice_simulation_contract::config::OpTemperatureMode::PvtRunSet
                    | rspice_simulation_contract::config::OpTemperatureMode::ActiveRunSetAxis
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
                rspice_simulation_contract::config::OpTemperatureMode::PvtRunSet
                    | rspice_simulation_contract::config::OpTemperatureMode::ActiveRunSetAxis
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
    spec: &rspice_simulation_contract::analysis_spec::AnalysisSpec,
    measurements: &[String],
) -> Result<(), String> {
    let config = spec.qpss_config()?;
    let grid = rspice_core::analysis::quasi_periodic::QuasiPeriodicGrid::new_with_abort(
        config.grid,
        &rspice_core::ResourceLimits::default(),
        &rspice_core::NoAbort,
    )
    .map_err(|error| error.to_string())?;
    for request in measurements {
        let (mode, key) = request.split_once(':').unwrap_or(("meas", request));
        if mode.eq_ignore_ascii_case("tuple") {
            let (tuple, _, _) =
                rspice_simulation_contract::study_measurement::parse_study_tuple(key)?;
            if grid.index_of(&tuple).is_none() {
                return Err(format!("QPSS does not retain lattice tuple {tuple:?}"));
            }
        } else if mode.eq_ignore_ascii_case("scalar") {
            if key.eq_ignore_ascii_case("qpss.oscillator_frequency_hz")
                && config.oscillator.is_some()
            {
                continue;
            }
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
