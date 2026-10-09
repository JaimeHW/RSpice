//! Default OP study preparation and frozen voltage observation semantics.
use super::*;
use rspice_core::engine::MonteCarloEnvironment;
use rspice_results::family_measurements::FamilyMeasurementEvidence;
use sha2::{Digest as _, Sha256};
use std::collections::HashSet;

/// Frozen analog observation roster used by preparation and trial conversion.
pub struct VoltageBasis {
    nodes: Vec<String>,
    event_only: Vec<bool>,
    columns: Vec<(String, usize)>,
}

impl VoltageBasis {
    fn new(circuit: &rspice_core::circuit::CircuitData) -> Self {
        let nodes = std::iter::once("0".into())
            .chain(circuit.node_names_sorted())
            .collect::<Vec<_>>();
        let event_only = (0..nodes.len())
            .map(|node| circuit.event_only_net_kind(node).is_some())
            .collect::<Vec<_>>();
        let authored = nodes
            .iter()
            .map(|name| name.to_ascii_uppercase())
            .collect::<HashSet<_>>();
        let mut columns = Vec::new();
        for (node, name) in nodes.iter().enumerate().skip(1) {
            if event_only[node] {
                continue;
            }
            columns.push((format!("V({name})"), node));
            // An authored numeric node owns its name. An MNA-index alias must
            // never hide that node, including one in the event domain.
            if !authored.contains(&node.to_string()) {
                columns.push((format!("V({node})"), node));
            }
        }
        columns.sort_by(|left, right| left.0.cmp(&right.0));
        Self {
            nodes,
            event_only,
            columns,
        }
    }

    pub fn identity(&self) -> [u8; 32] {
        let mut digest = Sha256::new();
        // Bind typed voltage observations, including their limit semantics.
        digest.update(b"rspice.monte-carlo-all-node-op/v2\0");
        digest.update((self.nodes.len() as u64).to_be_bytes());
        for (name, excluded) in self.nodes.iter().zip(&self.event_only) {
            digest.update((name.len() as u64).to_be_bytes());
            digest.update(name.as_bytes());
            digest.update([u8::from(*excluded)]);
        }
        digest.finalize().into()
    }

    pub fn observe(
        &self,
        result: rspice_core::SimulationResult,
    ) -> Result<Vec<FamilyMeasurementEvidence>, SimulationError> {
        if result.node_names.len() != self.nodes.len()
            || result.node_voltages.len() != self.nodes.len()
            || result
                .node_names
                .iter()
                .zip(&self.nodes)
                .any(|(a, b)| !a.eq_ignore_ascii_case(b))
            || self
                .event_only
                .iter()
                .enumerate()
                .any(|(node, expected)| result.event_only_node_kind(node).is_some() != *expected)
        {
            return Err(SimulationError::InvalidConfig(
                "Monte Carlo trial changed the frozen analog observation basis".into(),
            ));
        }
        self.columns
            .iter()
            .map(|(name, node)| {
                let value = result.node_voltages[*node];
                if !value.is_finite() {
                    return Err(SimulationError::CircuitError(format!(
                        "Monte Carlo voltage {name} is not finite"
                    )));
                }
                Ok(FamilyMeasurementEvidence {
                    unit: Some(rspice_core::analysis::MeasurementUnit::Known("V".into())),
                    name: name.clone(),
                    value: Some(value),
                    passed: true,
                    error: None,
                })
            })
            .collect()
    }
}

/// Prepared default-study inputs and their exact voltage observation basis.
pub struct PreparedVoltages {
    circuit: rspice_core::Netlist,
    engine: rspice_core::Engine,
    study: MonteCarloStudyConfig,
    basis: VoltageBasis,
}

impl PreparedVoltages {
    /// Transfer the prepared inputs to the caller's authorized executor.
    pub fn into_parts(
        self,
    ) -> (
        rspice_core::Netlist,
        rspice_core::Engine,
        MonteCarloStudyConfig,
        VoltageBasis,
    ) {
        (self.circuit, self.engine, self.study, self.basis)
    }
}

pub fn prepare_voltages(
    source: &str,
    source_path: Option<&Path>,
    variation_source: McVariationSource,
    histogram_bins: usize,
    environment: Option<AnalysisExecutionEnvironment>,
    abort: &dyn AbortSignal,
) -> Result<PreparedVoltages, SimulationError> {
    prepare_voltages_with_context(
        source,
        variation_source,
        histogram_bins,
        environment,
        ServiceContext {
            source_path,
            limits: rspice_core::ResourceLimits::default(),
            abort,
        },
    )
}

pub(crate) fn prepare_voltages_with_context(
    source: &str,
    variation_source: McVariationSource,
    histogram_bins: usize,
    environment: Option<AnalysisExecutionEnvironment>,
    context: ServiceContext<'_>,
) -> Result<PreparedVoltages, SimulationError> {
    let abort = context.abort;
    ensure_not_aborted(abort).map_err(SimulationError::from)?;
    if histogram_bins == 0 {
        return Err(SimulationError::InvalidConfig(
            "Histogram bins must be at least one".into(),
        ));
    }
    let source = match &environment {
        Some(point) => crate::error::run_abort_aware_service(abort, || {
            crate::netlist_preparation::source_with_run_temperature_with_abort(
                source,
                point.temperature_celsius,
                abort,
            )
        })?,
        None => source.to_owned(),
    };
    let circuit = crate::error::run_abort_aware_service(abort, || context.parse(&source))?;
    let engine = rspice_core::Engine::new(context.engine_config(&circuit));
    // Elaborate the roster without running Newton. A nonconvergent nominal OP
    // must not prevent otherwise valid randomized trials from being evaluated.
    let built = engine
        .build_circuit_with_abort(&circuit, abort)
        .map_err(|error| SimulationError::from_engine(&rspice_core::Engine::default(), error))?;
    let basis = VoltageBasis::new(&built);
    drop(built);
    if basis.columns.is_empty() {
        return Err(SimulationError::InvalidConfig(
            "Monte Carlo requires at least one analog node voltage".into(),
        ));
    }
    let command = circuit
        .analyses
        .iter()
        .find_map(|analysis| match analysis {
            AnalysisCommand::MonteCarlo(command) => Some(command),
            _ => None,
        })
        .ok_or_else(|| {
            SimulationError::InvalidConfig("Monte Carlo requires a .MC command".into())
        })?;
    let mut study = MonteCarloStudyConfig::new(
        command.runs,
        command.seed.unwrap_or(0x5EED_5EED),
        basis.columns.iter().map(|(name, _)| name.clone()).collect(),
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
    study.histogram_bins = histogram_bins;
    study.confidence_pct = command.confidence_pct;
    study.confidence_method = command.confidence_method.into();
    study.environment = environment.map(|point| MonteCarloEnvironment {
        temperature_celsius: point.temperature_celsius,
        supply_voltage: point.supply_voltage,
        nominal_supply_voltage: point.nominal_supply_voltage,
        supply_source_names: point.supply_source_names,
    });
    Ok(PreparedVoltages {
        circuit,
        engine,
        study,
        basis,
    })
}

pub(super) fn population_identity(
    source: &str,
    variation_source: McVariationSource,
    histogram_bins: usize,
    environment: Option<AnalysisExecutionEnvironment>,
    context: ServiceContext<'_>,
) -> Result<[u8; 32], SimulationError> {
    let prepared = prepare_voltages_with_context(
        source,
        variation_source,
        histogram_bins,
        environment,
        context,
    )?;
    prepared
        .engine
        .new_monte_carlo_checkpoint(
            &prepared.circuit,
            &prepared.study,
            prepared.basis.identity(),
            context.abort,
        )
        .map(|value| value.population_identity())
        .map_err(|error| SimulationError::from_engine(&rspice_core::Engine::default(), error))
}
