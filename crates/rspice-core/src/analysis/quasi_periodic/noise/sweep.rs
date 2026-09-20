//! One orbit linearization shared by every frequency, output and noise mechanism.
use super::*;
use crate::analysis::quasi_periodic::{
    QuasiPeriodicLinearConfig, small_signal::Linearization, solve::Circuit,
};

/// Authored physical frequency and exact integer anchor, with an explicit
/// noise-injection window. Circuit coupling always uses the complete QPSS basis.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QuasiPeriodicNoiseConfig {
    pub frequencies_hz: Vec<Value>,
    pub frequency_lattice: Vec<i32>,
    pub input_lattices: Vec<Vec<i32>>,
    #[serde(default)]
    pub linear: QuasiPeriodicLinearConfig,
}

/// Complete evidence for one frequency. Source covariances have the same
/// order as the supplied independent mechanisms; outputs share one operator.
/// The callback owns this point and must publish a sweep only after success.
#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QuasiPeriodicNoisePoint {
    pub adjoints: Vec<QuasiPeriodicAdjointSolution>,
    pub source_covariances: Vec<QuasiPeriodicNoiseCovariance>,
}

pub(crate) fn visit_with_abort(
    circuit: &mut impl Circuit,
    grid: Arc<QuasiPeriodicGrid>,
    config: &QuasiPeriodicNoiseConfig,
    orbit: &[Vec<Complex64>],
    observations: &[Vec<Vec<Complex64>>],
    sources: &[QuasiPeriodicNoiseSource],
    limits: &ResourceLimits,
    abort: &dyn AbortSignal,
    mut consume: impl FnMut(usize, QuasiPeriodicNoisePoint) -> Result<(), Error>,
) -> Result<(), Error> {
    check_abort(abort)?;
    config.linear.validate()?;
    let n = circuit.unknowns();
    if config.frequencies_hz.is_empty()
        || invalid_values(&config.frequencies_hz, |f| !f.is_finite(), abort)?
        || config.frequency_lattice.len() != grid.dimensions().len()
        || observations.is_empty()
    {
        return Err(invalid(
            "a finite frequency grid, exact tuple anchor and output observations are required",
        ));
    }
    for observation in observations {
        check_abort(abort)?;
        if observation.len() != n {
            return Err(invalid("observation differs from the complete MNA basis"));
        }
        let mut nonzero = false;
        for row in observation {
            if row.len() != grid.len() || invalid_values(row, |v| !finite(*v), abort)? {
                return Err(invalid(
                    "observation differs from its finite signed tone lattice",
                ));
            }
            for (i, value) in row.iter().enumerate() {
                if i.is_multiple_of(256) {
                    check_abort(abort)?;
                }
                nonzero |= *value != Complex64::ZERO;
            }
        }
        if !nonzero {
            return Err(invalid("an output observation must be nonzero"));
        }
    }
    ResourceLimitError::ensure(
        ResourceKind::AnalysisPoints,
        config
            .frequencies_hz
            .len()
            .saturating_mul(observations.len())
            .saturating_mul(sources.len().max(1)),
        limits.max_analysis_points,
    )?;
    // Charge borrowed source/observation data and the retained point alongside
    // both numerical workspaces. Future points stream, so no sweep-sized copy.
    let mut resident = observations
        .len()
        .saturating_mul(n)
        .saturating_mul(grid.len())
        .saturating_mul(4)
        .saturating_add(
            observations
                .len()
                .saturating_mul(grid.dimensions().len() + 2),
        )
        .saturating_add(
            sources
                .len()
                .saturating_mul(observations.len())
                .saturating_mul(observations.len())
                .saturating_mul(3),
        )
        .saturating_add(config.frequencies_hz.len())
        .saturating_add(
            config
                .input_lattices
                .len()
                .saturating_mul(grid.dimensions().len()),
        );
    for source in sources {
        check_abort(abort)?;
        let spectrum = match &source.spectrum {
            QuasiPeriodicNoiseSpectrum::White { density, .. } => density.len(),
            QuasiPeriodicNoiseSpectrum::PowerLaw { modulation, .. } => {
                modulation.len().saturating_mul(2)
            }
        };
        resident = resident
            .saturating_add(spectrum)
            .saturating_add(source.injections.len().saturating_mul(3));
    }
    ResourceLimitError::ensure(
        ResourceKind::ResultValues,
        resident,
        limits.max_result_values.min(32_000_000),
    )?;
    let mut noise_limits = limits.clone();
    noise_limits.max_result_values = limits
        .max_result_values
        .min(32_000_000)
        .saturating_sub(resident);
    let mut projector = QuasiPeriodicNoiseProjector::new_with_abort(
        grid.clone(),
        n,
        &config.input_lattices,
        &noise_limits,
        abort,
    )?;
    let mut workspace = projector.workspace_values(observations.len(), 0);
    for source in sources {
        if let QuasiPeriodicNoiseSpectrum::PowerLaw { modulation, .. } = &source.spectrum {
            let mut modes = 0usize;
            for (i, amplitude) in modulation.iter().enumerate() {
                if i.is_multiple_of(256) {
                    check_abort(abort)?;
                }
                modes += usize::from(*amplitude != Complex64::ZERO);
            }
            let pairs = projector.selected.len().saturating_mul(modes);
            ResourceLimitError::ensure(
                ResourceKind::AnalysisPoints,
                pairs,
                limits.max_analysis_points,
            )?;
            workspace = workspace.max(projector.workspace_values(
                observations.len(),
                pairs.saturating_mul(
                    grid.dimensions().len() + observations.len().saturating_mul(16),
                ),
            ));
        }
    }
    ResourceLimitError::ensure(
        ResourceKind::ResultValues,
        workspace,
        noise_limits.max_result_values,
    )?;
    let mut linear_limits = noise_limits;
    linear_limits.max_result_values = linear_limits.max_result_values.saturating_sub(workspace);
    let mut linear = Linearization::prepare_adjoint(
        circuit,
        grid,
        orbit,
        &config.linear,
        &linear_limits,
        abort,
    )?;
    for (frequency_index, &frequency) in config.frequencies_hz.iter().enumerate() {
        check_abort(abort)?;
        let adjoints = observations
            .iter()
            .map(|observation| {
                linear.solve_adjoint_at_frequency(
                    circuit,
                    frequency,
                    &config.frequency_lattice,
                    observation,
                    abort,
                )
            })
            .collect::<Result<Vec<_>, _>>()?;
        let source_covariances = sources
            .iter()
            .map(|source| projector.source_covariance_with_abort(&adjoints, source, abort))
            .collect::<Result<Vec<_>, _>>()?;
        check_abort(abort)?;
        consume(
            frequency_index,
            QuasiPeriodicNoisePoint {
                adjoints,
                source_covariances,
            },
        )?;
        check_abort(abort)?;
    }
    Ok(())
}
