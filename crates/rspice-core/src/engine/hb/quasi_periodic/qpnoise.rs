//! Authenticated multi-output noise about an independent-tone operating point.
mod derived;
mod integration;
mod request;
mod result;
#[cfg(test)]
mod tests;
mod transport;
mod validation;
pub use super::qpxf::{QpxfOutput as QpnoiseObservation, QpxfQuantity as QpnoiseQuantity};
use super::*;
use crate::analysis::quasi_periodic::{
    QuasiPeriodicLinearConfig, QuasiPeriodicNoiseConfig, QuasiPeriodicNoiseCovariance,
    QuasiPeriodicNoisePoint, QuasiPeriodicNoiseSource, QuasiPeriodicNoiseSpectrum,
};
use crate::{ResourceKind, ResourceLimitError, ResourceLimits};
pub use request::*;
pub use result::*;
pub use transport::{QpnoiseSourceLayout, QpnoiseSpectrumLayout, QpnoiseTransferMetadata};
fn qpnoise_error(message: impl Into<String>) -> SimulationError {
    SimulationError::Circuit(format!("QPNOISE: {}", message.into()))
}

impl Engine {
    pub fn run_qpnoise_from_qpss(
        &self,
        netlist: &Netlist,
        request: QpnoiseRequest,
        point: &QpssOperatingPoint,
    ) -> Result<QpnoiseAnalysisResult, SimulationError> {
        self.run_qpnoise_from_qpss_with_abort(netlist, request, point, &NoAbort)
    }
    /// Evaluate all selected stochastic mechanisms with one shared adjoint
    /// linearization. The caller's authenticated QPSS orbit is never re-solved.
    pub fn run_qpnoise_from_qpss_with_abort(
        &self,
        netlist: &Netlist,
        request: QpnoiseRequest,
        point: &QpssOperatingPoint,
        abort: &dyn AbortSignal,
    ) -> Result<QpnoiseAnalysisResult, SimulationError> {
        check_abort(abort)?;
        request.validate()?;
        let engine = self.resolved_for_netlist(netlist);
        engine.ensure_analysis_points(
            request
                .frequencies_hz
                .len()
                .saturating_mul(request.outputs.len()),
        )?;
        let producer = state::Producer::capture(netlist, &engine.config, point.config())?;
        let grid = engine.validate_qpss_operating_point_with_abort(netlist, point, abort)?;
        let input_lattices = request.input_lattices.resolve(&grid)?;
        for output in &request.outputs {
            if grid.index_of(&output.lattice).is_none() {
                return Err(qpnoise_error("output tuple is absent from retained QPSS"));
            }
        }
        if let Some(input) = &request.input
            && grid.index_of(&input.lattice).is_none()
        {
            return Err(qpnoise_error(
                "input referral tuple is absent from retained QPSS",
            ));
        }
        // Refuse unresolvable translated display grids before physical source
        // sampling or a linear solve, retaining the exact authored anchor.
        let anchor = request.frequency_anchor();
        for output in &request.outputs {
            let mut previous = None;
            for (i, frequency) in request.frequencies_hz.iter().enumerate() {
                if i.is_multiple_of(256) {
                    check_abort(abort)?;
                }
                let translated = grid
                    .frequency_relative_to(*frequency, &anchor, &output.lattice)
                    .map_err(numerical_error)?;
                if previous.is_some_and(|f| f >= translated) {
                    return Err(qpnoise_error(
                        "output frequency grid collapses at a translated tuple",
                    ));
                }
                previous = Some(translated);
            }
        }
        engine.ensure_result_values(
            request
                .frequencies_hz
                .len()
                .saturating_mul(request.outputs.len())
                .saturating_mul(point.spectra().len())
                .saturating_mul(grid.len())
                .saturating_mul(2),
        )?;
        let circuit = engine.build_circuit_with_abort(netlist, abort)?;
        Self::ensure_no_mixed_signal_analysis(&circuit, "QPNOISE")?;
        engine.validate_cyclostationary_noise_circuit(&circuit, "qpnoise")?;
        let mut solver = engine.qpss_circuit_solver(&circuit)?;
        let input_source = request
            .input
            .as_ref()
            .map(|input| {
                bindings::input(&circuit, &solver, &input.source, point.node_names().len())
            })
            .transpose()?;
        let observations = request
            .outputs
            .iter()
            .map(|output| bindings::output(netlist, point, &output.observation))
            .collect::<Result<Vec<_>, _>>()?;
        let sources = request
            .sources
            .select(engine.prepare_quasi_periodic_noise_sources(
                &circuit,
                &mut solver,
                point,
                grid.clone(),
                abort,
            )?)?;
        let reference = if let Some(figure) = &request.noise_figure {
            let input = request.input.as_ref().expect("validated input");
            if input_source.as_ref().expect("bound input").quantity != QpnoiseQuantity::Voltage {
                return Err(qpnoise_error(
                    "noise figure requires an ideal voltage source",
                ));
            }
            let source_resistor = engine.validate_noise_figure_source(
                netlist,
                &input.source,
                &figure.source_resistor,
                abort,
            )?;
            let (resistance, temperature) =
                super::super::noise_figure::resolve_noise_figure_resistor(
                    &circuit,
                    &source_resistor,
                    engine.config.temperature,
                )?;
            let label = format!("{source_resistor} thermal");
            let source_index=sources.iter().position(|s|s.name.eq_ignore_ascii_case(&label)).ok_or_else(||qpnoise_error("noise figure requires the source resistor thermal contribution in the selected mechanisms"))?;
            let lattices = figure
                .reference_lattices
                .clone()
                .unwrap_or_else(|| vec![input.lattice.clone()]);
            if lattices.iter().any(|t| !input_lattices.contains(t)) {
                return Err(qpnoise_error(
                    "noise figure reference tuples must be included in the noise-input window",
                ));
            }
            Some(QpnoiseReference {
                source_index,
                source_resistor,
                resistance,
                temperature,
                boltzmann: super::super::pnoise::pnoise_physical_constants(
                    engine.config.spice_dialect,
                )
                .boltzmann,
                lattices,
            })
        } else {
            None
        };
        let metadata = QpnoiseResultMetadata {
            version: 1,
            retained_identity: String::new(),
            operating_point_identity: point.retained_identity().to_owned(),
            request,
            grid: grid.config().clone(),
            node_names: point.node_names().to_vec(),
            branch_names: point.branch_names().to_vec(),
            ground_policy: netlist.ground_policy(),
            observations,
            input_source,
            input_lattices,
            reference,
        };
        // Reserve the entire retained result plus derivation workspace before
        // solving. The kernel separately accounts for its borrowed sources,
        // observations, one streamed point and numerical workspaces.
        let retained = validation::retained_values(&metadata, &sources, &grid);
        ResourceLimitError::ensure(
            ResourceKind::ResultValues,
            retained,
            engine
                .config
                .resource_limits
                .max_result_values
                .min(32_000_000),
        )?;
        let mut limits = engine.config.resource_limits.clone();
        limits.max_result_values = limits
            .max_result_values
            .min(32_000_000)
            .saturating_sub(retained);
        let output_values = metadata
            .request
            .outputs
            .len()
            .saturating_mul(point.spectra().len())
            .saturating_mul(grid.len())
            .saturating_mul(2);
        ResourceLimitError::ensure(
            ResourceKind::ResultValues,
            output_values,
            limits.max_result_values,
        )?;
        let mut observations = vec![
            vec![vec![Complex64::ZERO; grid.len()]; point.spectra().len()];
            metadata.observations.len()
        ];
        for (index, directions) in metadata.observations.iter().enumerate() {
            let tuple = grid
                .index_of(&metadata.request.outputs[index].lattice)
                .expect("validated output");
            for &(row, value) in directions {
                observations[index][row][tuple] = value;
            }
        }
        let config = QuasiPeriodicNoiseConfig {
            frequencies_hz: metadata.request.frequencies_hz.clone(),
            frequency_lattice: metadata.request.frequency_anchor(),
            input_lattices: metadata.input_lattices.clone(),
            linear: metadata.request.linear.clone(),
        };
        let mut points = Vec::with_capacity(config.frequencies_hz.len());
        solver
            .visit_quasi_periodic_noise_with_abort(
                grid.clone(),
                &config,
                point.spectra(),
                &observations,
                &sources,
                &limits,
                abort,
                |_, point| {
                    points.push(point);
                    Ok(())
                },
            )
            .map_err(numerical_error)?;
        if producer != state::Producer::capture(netlist, &engine.config, point.config())? {
            return Err(qpnoise_error("producer changed during noise execution"));
        }
        drop(observations);
        let (total_covariances, outputs) = derived::reconstruct(&metadata, &points, &grid, abort)?;
        let mut result = QpnoiseAnalysisResult {
            metadata,
            sources,
            points,
            total_covariances,
            outputs,
        };
        result.metadata.retained_identity = result.payload_identity(abort)?;
        result.validate_retained_payload_with_abort(&engine.config.resource_limits, abort)?;
        Ok(result)
    }
}
