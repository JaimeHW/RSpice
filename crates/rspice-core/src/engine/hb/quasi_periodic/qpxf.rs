//! Authenticated QPXF: multiple source/sideband transfers into one observation.
mod delay;
mod request;
#[cfg(test)]
mod tests;

pub use super::qpac::QpacInputQuantity as QpxfQuantity;
use super::*;
use crate::analysis::quasi_periodic::QuasiPeriodicAdjointSolution;
pub use delay::QpxfGroupDelay;
pub use request::{QpxfFrequencyAxis, QpxfInputLattices, QpxfOutput, QpxfRequest, QpxfSources};

#[derive(Debug, Clone, PartialEq)]
pub struct QpxfInputSource {
    pub name: String,
    pub quantity: QpxfQuantity,
    /// Complete unit excitation direction in canonical MNA coordinates.
    pub injections: Vec<(usize, Complex64)>,
}

#[derive(Debug, Clone, PartialEq)]
pub struct QpxfTransfer {
    pub input_source: usize,
    pub input_lattice: Vec<i32>,
    pub input_frequencies_hz: Vec<Value>,
    /// Output quantity per unit input quantity, with no AC source scaling.
    pub values: Vec<Complex64>,
    /// Phase differences on the authored output-frequency grid. Undefined
    /// samples retain their reason and never masquerade as zero delay.
    pub group_delay: Option<Vec<QpxfGroupDelay>>,
}

#[derive(Debug, Clone)]
pub struct QpxfAnalysisResult {
    pub request: QpxfRequest,
    pub operating_point_identity: String,
    pub grid: QuasiPeriodicGridConfig,
    pub node_names: Vec<String>,
    pub branch_names: Vec<String>,
    pub probe_offsets_hz: Vec<Value>,
    pub output_frequencies_hz: Vec<Value>,
    pub input_sources: Vec<QpxfInputSource>,
    pub input_lattices: Vec<Vec<i32>>,
    pub solutions: Vec<QuasiPeriodicAdjointSolution>,
    /// Source-major, then authored input-tuple order.
    pub transfers: Vec<QpxfTransfer>,
}

fn qpxf_error(message: impl Into<String>) -> SimulationError {
    SimulationError::Circuit(format!("QPXF: {}", message.into()))
}

impl Engine {
    pub fn run_qpxf_from_qpss(
        &self,
        netlist: &Netlist,
        request: QpxfRequest,
        point: &QpssOperatingPoint,
    ) -> Result<QpxfAnalysisResult, SimulationError> {
        self.run_qpxf_from_qpss_with_abort(netlist, request, point, &NoAbort)
    }

    /// All source/tuple paths share the exact QPSS orbit and one adjoint solve
    /// per output frequency. The producer is authenticated before and after
    /// execution; this never re-solves QPSS or substitutes a common-period HB.
    pub fn run_qpxf_from_qpss_with_abort(
        &self,
        netlist: &Netlist,
        request: QpxfRequest,
        point: &QpssOperatingPoint,
        abort: &dyn AbortSignal,
    ) -> Result<QpxfAnalysisResult, SimulationError> {
        check_abort(abort)?;
        request.validate()?;
        let engine = self.resolved_for_netlist(netlist);
        engine.ensure_analysis_points(request.frequencies_hz.len())?;
        let producer = state::Producer::capture(netlist, &engine.config, point.config())?;
        let grid = engine.validate_qpss_operating_point_with_abort(netlist, point, abort)?;
        let output_index = grid
            .index_of(&request.output_lattice)
            .ok_or_else(|| qpxf_error("output tuple is absent from the retained QPSS lattice"))?;
        let input_lattices = request.input_lattices.resolve(&grid)?;
        let anchor = match request.frequency_axis {
            QpxfFrequencyAxis::Output => request.output_lattice.clone(),
            QpxfFrequencyAxis::Offset => vec![0; request.output_lattice.len()],
        };
        let origin = vec![0; anchor.len()];
        let offsets = request
            .frequencies_hz
            .iter()
            .map(|f| {
                grid.frequency_relative_to(*f, &anchor, &origin)
                    .map_err(numerical_error)
            })
            .collect::<Result<Vec<_>, _>>()?;
        let output_frequencies = request
            .frequencies_hz
            .iter()
            .map(|f| {
                grid.frequency_relative_to(*f, &anchor, &request.output_lattice)
                    .map_err(numerical_error)
            })
            .collect::<Result<Vec<_>, _>>()?;
        if output_frequencies.windows(2).any(|f| f[0] >= f[1]) {
            return Err(qpxf_error(
                "output frequencies cannot resolve the requested grid at this tone tuple",
            ));
        }
        let circuit = engine.build_circuit_with_abort(netlist, abort)?;
        Self::ensure_no_mixed_signal_analysis(&circuit, "QPXF")?;
        let mut solver = engine.qpss_circuit_solver(&circuit)?;
        let mut observation = vec![vec![Complex64::ZERO; grid.len()]; point.spectra().len()];
        match &request.output {
            QpxfOutput::Voltage { positive, negative } => {
                let node = |name: &str| -> Result<Option<usize>, SimulationError> {
                    if netlist.ground_policy().is_ground(name.trim()) {
                        return Ok(None);
                    }
                    point
                        .node_names()
                        .iter()
                        .position(|n| n.eq_ignore_ascii_case(name.trim()))
                        .map(Some)
                        .ok_or_else(|| {
                            qpxf_error(format!("output node '{name}' is absent from the circuit"))
                        })
                };
                let positive = node(positive)?;
                let negative = node(negative)?;
                if positive == negative {
                    return Err(qpxf_error("output and reference must be distinct nodes"));
                }
                if let Some(row) = positive {
                    observation[row][output_index] = Complex64::ONE;
                }
                if let Some(row) = negative {
                    observation[row][output_index] = -Complex64::ONE;
                }
            }
            QpxfOutput::BranchCurrent { branch } => {
                let row = point.branch_names().iter().position(|n| n.eq_ignore_ascii_case(branch.trim()))
                    .ok_or_else(|| qpxf_error(format!("output '{branch}' is not a retained MNA current; select a voltage-source/current-probe, inductor or another exact branch current")))?;
                observation[point.node_names().len() + row][output_index] = Complex64::ONE;
            }
        }
        let names = match &request.input_sources {
            QpxfSources::AllIndependent => {
                let mut names: Vec<_> = circuit
                    .voltage_sources
                    .names
                    .iter()
                    .chain(&circuit.current_sources.names)
                    .cloned()
                    .collect();
                names.sort_by_key(|s| s.to_ascii_lowercase());
                names
            }
            QpxfSources::Named(names) => names.clone(),
        };
        if names.is_empty() {
            return Err(qpxf_error("no independent input sources were selected"));
        }
        let paths = names.len().saturating_mul(input_lattices.len());
        let metadata_values = request
            .frequencies_hz
            .len()
            .saturating_mul(
                paths
                    .saturating_mul(if request.group_delay { 5 } else { 3 })
                    .saturating_add(4),
            )
            .saturating_add(names.len().saturating_mul(8))
            .saturating_add(
                input_lattices
                    .len()
                    .saturating_mul(request.output_lattice.len()),
            );
        engine.ensure_result_values(metadata_values)?;
        let mut inputs = Vec::with_capacity(names.len());
        for name in names {
            check_abort(abort)?;
            let source = Self::pac_input_port(&circuit, &name, point.node_names().len())?;
            let (quantity, injections, name) = if let Some(index) = source.voltage_source_index {
                let row = solver
                    .periodic_voltage_source_branch(index)
                    .ok_or_else(|| qpxf_error("input voltage source has no exact MNA branch"))?;
                (
                    QpxfQuantity::Voltage,
                    vec![(point.node_names().len() + row, Complex64::ONE)],
                    circuit.voltage_sources.names[index].clone(),
                )
            } else {
                let canonical = circuit
                    .current_sources
                    .names
                    .iter()
                    .find(|n| n.eq_ignore_ascii_case(name.trim()))
                    .expect("bound current source");
                (
                    QpxfQuantity::Current,
                    source.node_injections,
                    canonical.clone(),
                )
            };
            inputs.push(QpxfInputSource {
                name,
                quantity,
                injections,
            });
        }
        let mut limits = engine.config.resource_limits.clone();
        limits.max_result_values = limits.max_result_values.saturating_sub(metadata_values);
        let solutions = solver
            .solve_quasi_periodic_adjoint_at_frequency_with_abort(
                grid.clone(),
                &request.linear,
                point.spectra(),
                &request.frequencies_hz,
                &anchor,
                &observation,
                &limits,
                abort,
            )
            .map_err(numerical_error)?;
        let mut transfers = Vec::with_capacity(paths);
        for (source_index, source) in inputs.iter().enumerate() {
            for tuple in &input_lattices {
                check_abort(abort)?;
                let index = grid.index_of(tuple).expect("validated input tuple");
                let input_frequencies_hz: Vec<_> = request
                    .frequencies_hz
                    .iter()
                    .enumerate()
                    .map(|(i, f)| {
                        if i % 256 == 0 {
                            check_abort(abort)?;
                        }
                        grid.frequency_relative_to(*f, &anchor, tuple)
                            .map_err(numerical_error)
                    })
                    .collect::<Result<_, _>>()?;
                if input_frequencies_hz.iter().any(|f| !f.is_finite()) {
                    return Err(qpxf_error(
                        "input frequencies cannot resolve the requested grid at this tone tuple",
                    ));
                }
                let values: Vec<Complex64> = solutions
                    .iter()
                    .enumerate()
                    .map(|(i, s)| {
                        if i % 256 == 0 {
                            check_abort(abort)?;
                        }
                        Ok(source
                            .injections
                            .iter()
                            .map(|(row, value)| s.sensitivities[*row][index].conj() * value)
                            .sum())
                    })
                    .collect::<Result<_, SimulationError>>()?;
                if values
                    .iter()
                    .any(|v| !v.re.is_finite() || !v.im.is_finite())
                {
                    return Err(qpxf_error("source transfer projection overflowed"));
                }
                let group_delay = request
                    .group_delay
                    .then(|| {
                        delay::from_samples(
                            &output_frequencies,
                            &values,
                            request.group_delay_magnitude_floor,
                            abort,
                        )
                    })
                    .transpose()?;
                transfers.push(QpxfTransfer {
                    input_source: source_index,
                    input_lattice: tuple.clone(),
                    input_frequencies_hz,
                    values,
                    group_delay,
                });
            }
        }
        if producer != state::Producer::capture(netlist, &engine.config, point.config())? {
            return Err(qpxf_error("producer inputs changed during the analysis"));
        }
        check_abort(abort)?;
        Ok(QpxfAnalysisResult {
            request,
            operating_point_identity: point.retained_identity().to_owned(),
            grid: grid.config().clone(),
            node_names: point.node_names().to_vec(),
            branch_names: point.branch_names().to_vec(),
            probe_offsets_hz: offsets,
            output_frequencies_hz: output_frequencies,
            input_sources: inputs,
            input_lattices,
            solutions,
            transfers,
        })
    }
}
