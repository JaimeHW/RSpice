//! Authenticated QPSS reuse and arbitrary-dimensional QPAC source/observation binding.
mod card;
use super::*;
use crate::analysis::quasi_periodic::{QuasiPeriodicAcConfig, QuasiPeriodicAcSolution};

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(deny_unknown_fields)]
pub struct QpacRequest {
    /// Ordered explicit probe offsets. Generated LIN/DEC/OCT sweeps resolve
    /// to this same grid; translated input frequency is offset + k_in·tones.
    pub offsets_hz: Vec<Value>,
    pub input_source: String,
    pub input_lattice: Vec<i32>,
    pub output_node: String,
    pub output_ref: String,
    pub output_lattice: Vec<i32>,
    pub magnitude: Value,
    pub phase_degrees: Value,
    pub solver: QuasiPeriodicAcConfig,
}

impl QpacRequest {
    pub fn validate(&self) -> Result<(), SimulationError> {
        self.solver.validate().map_err(numerical_error)?;
        if self.offsets_hz.is_empty()
            || self.offsets_hz.iter().any(|f| !f.is_finite())
            || self.offsets_hz.windows(2).any(|p| p[0] >= p[1])
        {
            return Err(qpac_error(
                "probe offsets must be finite, nonempty and strictly increasing",
            ));
        }
        if self.input_lattice.len() < 2 || self.input_lattice.len() != self.output_lattice.len() {
            return Err(qpac_error(
                "input and output tuples need the same number of coordinates (at least two)",
            ));
        }
        if [&self.input_source, &self.output_node, &self.output_ref]
            .iter()
            .any(|name| name.trim().is_empty() || name.contains(['\r', '\n']))
        {
            return Err(qpac_error(
                "input source, output node and reference must be nonempty single-line names",
            ));
        }
        if !self.magnitude.is_finite() || self.magnitude <= 0.0 || !self.phase_degrees.is_finite() {
            return Err(qpac_error(
                "drive magnitude must be positive and finite and phase must be finite",
            ));
        }
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct QpacAnalysisResult {
    pub request: QpacRequest,
    pub operating_point_identity: String,
    pub tone_frequencies_hz: Vec<Value>,
    pub tuples: Vec<Vec<i32>>,
    pub node_names: Vec<String>,
    pub branch_names: Vec<String>,
    pub input_frequencies_hz: Vec<Value>,
    pub output_frequencies_hz: Vec<Value>,
    /// Full complex response per unit input in the source's own units.
    /// The numerical residual certificate refers to this unit solve.
    pub unit_solutions: Vec<QuasiPeriodicAcSolution>,
    /// Differential output per unit input at the selected output tuple.
    pub output_transfer: Vec<Complex64>,
    /// Differential output with the authored magnitude and phase applied.
    pub output_response: Vec<Complex64>,
}

fn qpac_error(message: impl Into<String>) -> SimulationError {
    SimulationError::Circuit(format!("QPAC: {}", message.into()))
}

impl Engine {
    pub fn run_qpac_from_qpss(
        &self,
        netlist: &Netlist,
        request: QpacRequest,
        point: &QpssOperatingPoint,
    ) -> Result<QpacAnalysisResult, SimulationError> {
        self.run_qpac_from_qpss_with_abort(netlist, request, point, &NoAbort)
    }

    /// Consume the exact retained driven orbit, without re-solving QPSS or
    /// substituting a commensurate HB basis. Probe AC annotations in the deck
    /// do not override this request's explicitly authored drive.
    pub fn run_qpac_from_qpss_with_abort(
        &self,
        netlist: &Netlist,
        request: QpacRequest,
        point: &QpssOperatingPoint,
        abort: &dyn AbortSignal,
    ) -> Result<QpacAnalysisResult, SimulationError> {
        check_abort(abort)?;
        request.validate()?;
        let engine = self.resolved_for_netlist(netlist);
        engine.ensure_analysis_points(request.offsets_hz.len())?;
        let producer = state::Producer::capture(netlist, &engine.config, point.config())?;
        let grid = engine.validate_qpss_operating_point_with_abort(netlist, point, abort)?;
        let input_index = grid
            .index_of(&request.input_lattice)
            .ok_or_else(|| qpac_error("input tuple is absent from the retained QPSS lattice"))?;
        let output_index = grid
            .index_of(&request.output_lattice)
            .ok_or_else(|| qpac_error("output tuple is absent from the retained QPSS lattice"))?;
        let node = |name: &str| -> Result<Option<usize>, SimulationError> {
            let name = name.trim();
            if netlist.ground_policy().is_ground(name) {
                return Ok(None);
            }
            point
                .node_names()
                .iter()
                .position(|n| n.eq_ignore_ascii_case(name))
                .map(Some)
                .ok_or_else(|| {
                    qpac_error(format!("output node '{name}' is absent from the circuit"))
                })
        };
        let positive = node(&request.output_node)?;
        let negative = node(&request.output_ref)?;
        if positive == negative {
            return Err(qpac_error("output and reference must be distinct nodes"));
        }
        let circuit = engine.build_circuit_with_abort(netlist, abort)?;
        Self::ensure_no_mixed_signal_analysis(&circuit, "QPAC")?;
        let mut solver = engine.qpss_circuit_solver(&circuit)?;
        let source =
            Self::pac_input_port(&circuit, &request.input_source, point.node_names().len())?;
        let mut sources = vec![vec![Complex64::ZERO; grid.len()]; point.spectra().len()];
        for (row, amplitude) in source.node_injections {
            sources[row][input_index] += amplitude;
        }
        if let Some(index) = source.voltage_source_index {
            let branch = solver
                .periodic_voltage_source_branch(index)
                .ok_or_else(|| qpac_error("input voltage source has no exact MNA branch"))?;
            sources[point.node_names().len() + branch][input_index] = Complex64::new(1.0, 0.0);
        }
        let metadata_values = request
            .offsets_hz
            .len()
            .saturating_mul(8)
            .saturating_add(grid.len().saturating_mul(grid.dimensions().len()))
            .saturating_add(grid.dimensions().len());
        engine.ensure_result_values(metadata_values)?;
        let mut limits = engine.config.resource_limits.clone();
        limits.max_result_values = limits.max_result_values.saturating_sub(metadata_values);
        let solutions = solver
            .solve_quasi_periodic_ac_with_abort(
                grid.clone(),
                &request.solver,
                point.spectra(),
                &request.offsets_hz,
                &sources,
                &limits,
                abort,
            )
            .map_err(numerical_error)?;
        let drive = Complex64::from_polar(
            request.magnitude,
            (request.phase_degrees % 360.0).to_radians(),
        );
        let mut transfer = Vec::with_capacity(solutions.len());
        let mut response = Vec::with_capacity(solutions.len());
        let mut input_frequencies = Vec::with_capacity(solutions.len());
        let mut output_frequencies = Vec::with_capacity(solutions.len());
        for solution in &solutions {
            check_abort(abort)?;
            let voltage = |row: Option<usize>| {
                row.map_or(Complex64::ZERO, |row| solution.spectra[row][output_index])
            };
            let value = voltage(positive) - voltage(negative);
            let scaled = value * drive;
            if [value, scaled]
                .iter()
                .any(|v| !v.re.is_finite() || !v.im.is_finite())
            {
                return Err(qpac_error("output response overflowed"));
            }
            transfer.push(value);
            response.push(scaled);
            input_frequencies.push(solution.offset_hz + grid.frequencies_hz()[input_index]);
            output_frequencies.push(solution.offset_hz + grid.frequencies_hz()[output_index]);
        }
        if producer != state::Producer::capture(netlist, &engine.config, point.config())? {
            return Err(qpac_error("producer inputs changed during the analysis"));
        }
        check_abort(abort)?;
        Ok(QpacAnalysisResult {
            request,
            operating_point_identity: point.retained_identity().to_owned(),
            tone_frequencies_hz: grid.config().frequencies_hz.clone(),
            tuples: grid.indices().to_vec(),
            node_names: point.node_names().to_vec(),
            branch_names: point.branch_names().to_vec(),
            input_frequencies_hz: input_frequencies,
            output_frequencies_hz: output_frequencies,
            unit_solutions: solutions,
            output_transfer: transfer,
            output_response: response,
        })
    }
}

#[cfg(test)]
mod tests;
