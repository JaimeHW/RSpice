//! Prepared AC bias and certified solves shared by AC and every SP excitation.

use super::*;

pub(in crate::engine) struct PreparedAc {
    pub circuit: CircuitData,
    pub matrix: StaticMatrix,
    pub linearization: AcLinearization,
    pub excitation: AcExcitation,
}

pub(in crate::engine) struct AcLinearization {
    pub bias: Vec<Value>,
    denominator_floor: Option<Vec<Value>>,
}

/// A fixed RHS and the corresponding exact independent-source constraints.
pub(in crate::engine) struct AcExcitation {
    rhs: Vec<Complex64>,
    projection: AcVoltageConstraintProjection,
}

impl AcExcitation {
    fn new(circuit: &CircuitData) -> Result<Self, SimulationError> {
        Ok(Self {
            projection: AcVoltageConstraintProjection::new(circuit)?,
            rhs: Engine::build_ac_excitation_rhs(circuit),
        })
    }

    pub(in crate::engine) fn for_port(
        circuit: &CircuitData,
        source: &str,
    ) -> Result<Self, SimulationError> {
        let sources = &circuit.voltage_sources;
        let index = sources
            .names
            .iter()
            .position(|name| name.eq_ignore_ascii_case(source))
            .ok_or_else(|| {
                SimulationError::Circuit(format!("SP voltage source '{source}' was not found"))
            })?;
        let projection = AcVoltageConstraintProjection::with_excitation(circuit, |candidate| {
            Complex64::new(if candidate == index { 1.0 } else { 0.0 }, 0.0)
        })?;
        let branch = circuit.get_branch_matrix_index(sources.branch_indices[index]);
        let row = branch.checked_sub(1).ok_or_else(|| {
            SimulationError::Circuit(format!("SP voltage source '{source}' has no equation row"))
        })?;
        let mut rhs = vec![Complex64::new(0.0, 0.0); circuit.matrix_size()];
        let entry = rhs.get_mut(row).ok_or_else(|| {
            SimulationError::Circuit(format!(
                "SP voltage source '{source}' equation lies outside the solved system"
            ))
        })?;
        *entry = Complex64::new(1.0, 0.0);
        Ok(Self { rhs, projection })
    }
}

impl Engine {
    /// Initialize one AC model state and solve its accepted operating point.
    /// The caller supplies a run scope and uses the netlist-resolved Engine.
    pub(in crate::engine) fn prepare_ac_analysis(
        &self,
        netlist: &Netlist,
        abort: &dyn AbortSignal,
    ) -> Result<PreparedAc, SimulationError> {
        let circuit = self.build_circuit_with_abort(netlist, abort)?;
        self.prepare_ac_circuit(netlist, circuit, abort)
    }

    /// SP retains elaborated port metadata before initializing this same AC state.
    pub(in crate::engine) fn prepare_ac_circuit(
        &self,
        netlist: &Netlist,
        mut circuit: CircuitData,
        abort: &dyn AbortSignal,
    ) -> Result<PreparedAc, SimulationError> {
        circuit
            .begin_veriloga_equilibrium_analysis(1)
            .map_err(SimulationError::Circuit)?;
        Self::deliver_initial_analog_tasks(&mut circuit, abort)?;
        Self::ensure_no_mixed_signal_analysis(&circuit, "AC analysis")?;
        // Coupled multiconductor lines have no small-signal load (ngspice's
        // CPL registers none and its AC solve fails with a singular matrix);
        // refuse explicitly instead of returning silently dead ports.
        if !circuit.coupled_tlines.is_empty() {
            return Err(SimulationError::unsupported_capability(
                "analysis.ac.device.coupled_transmission_line",
                "AC analysis does not support coupled multiconductor (CPL) transmission lines",
            ));
        }
        Self::ensure_supported_ac_dynamic_charges(&circuit)?;
        circuit
            .prepare_veriloga_equilibrium_analysis_point(1, true, false)
            .map_err(SimulationError::Circuit)?;
        let mut matrix = if circuit.matrix_size() == 0 {
            Self::model_observation_matrix()?
        } else {
            self.build_matrix(&circuit)?
        };
        circuit.link_indices(&matrix);
        let excitation = AcExcitation::new(&circuit)?;

        // Get DC operating point
        let has_nonlinear = circuit.has_nonlinear_devices();
        let dc_solution = if circuit.matrix_size() == 0 {
            Vec::new()
        } else if circuit.can_use_zero_bias_for_explicit_xspice_ac() {
            log::debug!(
                "using zero-bias small-signal state for explicit XSPICE transmission-line AC"
            );
            vec![0.0; circuit.matrix_size()]
        } else {
            self.solve_dc_operating_point_with_abort(netlist, &mut circuit, &mut matrix, abort)?
        };
        if has_nonlinear && !dc_solution.is_empty() {
            self.try_observe_dc_operating_point(&mut circuit, &mut matrix, &dc_solution)?;
        }
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if let Some(message) = circuit.take_xspice_evaluation_error() {
            return Err(SimulationError::Circuit(format!(
                "XSPICE evaluation failed: {message}"
            )));
        }
        self.accept_frequency_operating_point(
            netlist,
            &mut circuit,
            &mut matrix,
            &dc_solution,
            1,
            abort,
        )?;
        circuit
            .finish_veriloga_equilibrium_operating_point(1)
            .map_err(SimulationError::Circuit)?;
        circuit.refresh_jiles_atherton_inductances(&dc_solution);
        if has_nonlinear {
            // Align stateful nonlinear models (limited junction voltages,
            // operating region) with the final converged operating point.
            Self::prepare_small_signal_state(&mut circuit, &dc_solution)?;
        } else {
            // Behavioral source caches may still be present on an otherwise
            // linear circuit.
            circuit
                .prepare_behavioral_small_signal(&dc_solution)
                .map_err(SimulationError::Circuit)?;
        }

        let size = circuit.matrix_size();
        let ac_solve_denominator_floor = if excitation.projection.is_empty() {
            None
        } else {
            let mut floors = vec![0.0; size];
            for &branch_ordinal in &circuit.voltage_sources.branch_indices {
                let branch = circuit.get_branch_matrix_index(branch_ordinal);
                let row = branch.checked_sub(1).ok_or_else(|| {
                    SolverError::InvalidCircuit(
                        "independent voltage source has no AC equation row".to_string(),
                    )
                })?;
                let Some(floor) = floors.get_mut(row) else {
                    return Err(SolverError::InvalidCircuit(
                        "independent voltage-source AC equation lies outside the solved system"
                            .to_string(),
                    )
                    .into());
                };
                // Homogeneous ideal-source rows can carry only roundoff-scale
                // leakage before their exact post-solve projection. Give
                // those known voltage equations the same one-volt coordinate
                // floor as the projection validator; every other MNA row
                // remains under the strict componentwise solve certificate.
                *floor = 1.0;
            }
            Some(floors)
        };

        Ok(PreparedAc {
            circuit,
            matrix,
            excitation,
            linearization: AcLinearization {
                bias: dc_solution,
                denominator_floor: ac_solve_denominator_floor,
            },
        })
    }
}

impl AcLinearization {
    pub(in crate::engine) fn prepare_frequency(
        &self,
        circuit: &mut CircuitData,
        ac_matrix: &mut ComplexMatrix,
        frequency: Value,
        final_step: bool,
        abort: &dyn AbortSignal,
    ) -> Result<(), SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        circuit
            .prepare_veriloga_frequency_analysis_point(1, final_step)
            .map_err(SimulationError::Circuit)?;
        if circuit.matrix_size() == 0 {
            return Ok(());
        }
        circuit
            .prepare_behavioral_small_signal_at_frequency(&self.bias, frequency)
            .map_err(SimulationError::Circuit)?;
        Engine::try_fill_small_signal_matrix_with_vbic_delay_mode(
            circuit,
            ac_matrix,
            &self.bias,
            2.0 * PI * frequency,
            SmallSignalAnalysisKind::Ac,
            true,
            true,
        )
    }

    /// Consecutive excitations reuse this matrix's cached factorization.
    pub(in crate::engine) fn solve(
        &self,
        ac_matrix: &mut ComplexMatrix,
        excitation: &AcExcitation,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Complex64>, SimulationError> {
        let rhs = &excitation.rhs;
        let sparse_solution = match self.denominator_floor.as_deref() {
            Some(floor) => ac_matrix.solve_with_row_denominator_floors(rhs, floor),
            None => ac_matrix.solve(rhs),
        };
        let mut solution = match sparse_solution {
            Ok(solution) => solution,
            Err(SolverError::InaccurateSolution(_)) if rhs.len() <= 64 => {
                log::debug!(
                    "sparse AC solve failed strict backward-error certification; retrying the small complex system with extended precision"
                );
                ac_matrix
                    .solve_dense_extended(rhs)
                    .map_err(SimulationError::Solver)?
            }
            Err(error) => return Err(SimulationError::Solver(error)),
        };
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }
        if !excitation.projection.is_empty() {
            excitation.projection.project(&mut solution)?;
            ac_matrix
                .certify_solution(&solution, rhs)
                .map_err(SimulationError::Solver)?;
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
        }

        Ok(solution)
    }
}
