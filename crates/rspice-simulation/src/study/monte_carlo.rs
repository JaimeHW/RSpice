//! Exact preparation shared by Monte Carlo execution and checkpoint routing.

use super::{
    StudyAnalysis, StudyRunConfig, resolved_study_environment, study_source_at_environment,
    validate_base_measurements,
};
use crate::engine_services::ServiceContext;
use crate::error::{SimulationError, ensure_not_aborted};
use rspice_core::abort_signal::AbortSignal;
use rspice_core::analysis::monte_carlo::Distribution;
use rspice_core::engine::{MonteCarloStudyConfig, MonteCarloVariationSource};
use rspice_core::netlist::{AnalysisCommand, MonteCarloDistribution};
use rspice_simulation_contract::mc_draft::McVariationSource;
use rspice_simulation_contract::study_measurement::validate_measurements;
use rspice_simulation_contract::worker_protocol::AnalysisExecutionEnvironment;
use std::path::Path;

mod voltages;
pub(crate) use voltages::prepare_voltages_with_context;
pub use voltages::{PreparedVoltages, VoltageBasis, prepare_voltages};

/// Prepared study inputs; preparing them does not authorize dispatch.
pub struct PreparedStudy {
    analysis: StudyAnalysis,
    circuit: rspice_core::Netlist,
    study: MonteCarloStudyConfig,
    engine: rspice_core::Engine,
}

impl PreparedStudy {
    /// Transfer the prepared inputs to the caller's authorized executor.
    pub fn into_parts(
        self,
    ) -> (
        StudyAnalysis,
        rspice_core::Netlist,
        MonteCarloStudyConfig,
        rspice_core::Engine,
    ) {
        (self.analysis, self.circuit, self.study, self.engine)
    }
}

pub fn prepare_study(
    base: &StudyRunConfig,
    variation_source: McVariationSource,
    source: &str,
    source_path: Option<&Path>,
    environment: Option<AnalysisExecutionEnvironment>,
    abort: &dyn AbortSignal,
) -> Result<PreparedStudy, SimulationError> {
    prepare_study_with_context(
        base,
        variation_source,
        source,
        environment,
        ServiceContext {
            source_path,
            limits: rspice_core::ResourceLimits::default(),
            abort,
        },
    )
}

pub(crate) fn prepare_study_with_context(
    base: &StudyRunConfig,
    variation_source: McVariationSource,
    source: &str,
    environment: Option<AnalysisExecutionEnvironment>,
    context: ServiceContext<'_>,
) -> Result<PreparedStudy, SimulationError> {
    let ServiceContext {
        source_path,
        limits,
        abort,
    } = context;
    ensure_not_aborted(abort).map_err(SimulationError::from)?;
    if !base.objective_terms.is_empty() || !base.constraints.is_empty() {
        return Err(SimulationError::InvalidConfig(
            "Objectives and constraints apply only to optimization".into(),
        ));
    }
    validate_measurements(&base.measurements).map_err(SimulationError::InvalidConfig)?;
    base.analysis
        .validate()
        .map_err(|errors| SimulationError::InvalidConfig(errors.join("; ")))?;
    // Preserve these cards in the parser's retained source, so expression and
    // native-statistics replay observes the same analysis and solver policy.
    let (analysis, environment) = resolved_study_environment(base, environment);
    let source = study_source_at_environment(base, source, environment.as_ref(), abort)?;
    let engine = rspice_core::Engine::new(rspice_core::SimulationConfig {
        resource_limits: limits,
        ..Default::default()
    });
    let circuit = crate::netlist_preparation::parse_analysis_netlist_with_abort(
        &source,
        source_path,
        engine.config().resource_limits,
        &Default::default(),
        abort,
    )?;
    let command = circuit
        .analyses
        .iter()
        .find_map(|analysis| match analysis {
            AnalysisCommand::MonteCarlo(command) => Some(command),
            _ => None,
        })
        .ok_or_else(|| {
            SimulationError::InvalidConfig("Configured Monte Carlo requires a .MC command".into())
        })?;
    validate_base_measurements(base, &circuit)?;
    let mut study = MonteCarloStudyConfig::new(
        command.runs,
        command.seed.unwrap_or(0x5EED_5EED),
        base.measurements.clone(),
    );
    study.first_trial = command.first_trial;
    study.distribution = match command.distribution {
        MonteCarloDistribution::Gaussian => Distribution::Gaussian {
            sigma: command.relative_spread,
        },
        MonteCarloDistribution::Uniform => Distribution::Uniform {
            tolerance: command.relative_spread,
        },
        MonteCarloDistribution::WorstCase => Distribution::WorstCase {
            tolerance: command.relative_spread,
        },
    };
    study.variation_source = match variation_source {
        McVariationSource::ParameterTolerance => MonteCarloVariationSource::ParameterTolerance,
        McVariationSource::DeckStatistics => MonteCarloVariationSource::DeckStatistics,
    };
    study.parameter_filter = command.params.clone();
    study.histogram_bins = base.histogram_bins;
    study.confidence_pct = command.confidence_pct;
    study.confidence_method = command.confidence_method.into();
    study.environment = environment;
    let engine = engine.resolved_for_netlist(&circuit);
    Ok(PreparedStudy {
        analysis,
        circuit,
        study,
        engine,
    })
}

/// The same population contract the runner checks, without executing a trial.
/// Prepared decks already seal include contents and therefore have no source path.
pub fn prepared_population_identity(
    base: Option<&StudyRunConfig>,
    histogram_bins: usize,
    variation_source: McVariationSource,
    statistics: Option<&rspice_simulation_contract::mc_statistics::McStatisticsConfig>,
    source: &str,
    environment: Option<AnalysisExecutionEnvironment>,
) -> Result<[u8; 32], SimulationError> {
    prepared_population_identity_with_context(
        base,
        histogram_bins,
        variation_source,
        statistics,
        source,
        environment,
        ServiceContext {
            source_path: None,
            limits: rspice_core::ResourceLimits::default(),
            abort: &rspice_core::NoAbort,
        },
    )
}

pub(crate) fn prepared_population_identity_with_context(
    base: Option<&StudyRunConfig>,
    histogram_bins: usize,
    variation_source: McVariationSource,
    statistics: Option<&rspice_simulation_contract::mc_statistics::McStatisticsConfig>,
    source: &str,
    environment: Option<AnalysisExecutionEnvironment>,
    context: ServiceContext<'_>,
) -> Result<[u8; 32], SimulationError> {
    ensure_not_aborted(context.abort)?;
    let source = source_with_statistics(source, variation_source, statistics)?;
    let Some(base) = base else {
        return voltages::population_identity(
            &source,
            variation_source,
            histogram_bins,
            environment,
            context,
        );
    };
    let prepared =
        prepare_study_with_context(base, variation_source, &source, environment, context)?;
    let evaluation = *crate::execution_identity::monte_carlo_evaluator_digest(base).as_bytes();
    prepared
        .engine
        .new_monte_carlo_checkpoint(
            &prepared.circuit,
            &prepared.study,
            evaluation,
            context.abort,
        )
        .map(|checkpoint| checkpoint.population_identity())
        .map_err(|error| SimulationError::from_engine(&rspice_core::Engine::default(), error))
}

pub fn source_with_statistics<'a>(
    source: &'a str,
    variation_source: McVariationSource,
    statistics: Option<&rspice_simulation_contract::mc_statistics::McStatisticsConfig>,
) -> Result<std::borrow::Cow<'a, str>, SimulationError> {
    let Some(statistics) = statistics else {
        return Ok(source.into());
    };
    if variation_source != McVariationSource::DeckStatistics {
        return Err(SimulationError::InvalidConfig(
            "Custom statistics require the native statistics sampler".into(),
        ));
    }
    let directive = statistics
        .parser_directive()
        .map_err(SimulationError::InvalidConfig)?;
    Ok(crate::netlist_preparation::splice_before_terminal_end_card(source, &directive).into())
}
