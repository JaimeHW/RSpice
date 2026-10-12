//! Prepared AC bias and certified solves shared by AC and every SP excitation.

use super::*;
use crate::engine::CapabilityAnalysis;
use crate::engine::small_signal_bias::{BiasSource, SmallSignalBias};

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
    /// Inject one ampere into `positive` and withdraw it from `negative`,
    /// with every independent voltage source held at zero small-signal drive.
    pub(in crate::engine) fn current_probe(
        circuit: &CircuitData,
        positive: usize,
        negative: usize,
    ) -> Result<Self, SimulationError> {
        let projection =
            AcVoltageConstraintProjection::with_excitation(circuit, |_| Complex64::new(0.0, 0.0))?;
        let mut rhs = vec![Complex64::new(0.0, 0.0); circuit.matrix_size()];
        for (node, value) in [(positive, 1.0), (negative, -1.0)] {
            if node > 0 {
                let entry = rhs.get_mut(node - 1).ok_or_else(|| {
                    SimulationError::Circuit(
                        "AC current probe lies outside the solved system".into(),
                    )
                })?;
                *entry += Complex64::new(value, 0.0);
            }
        }
        Ok(Self { rhs, projection })
    }

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
        if !circuit
            .voltage_sources
            .names
            .iter()
            .any(|name| name.eq_ignore_ascii_case(source))
        {
            return Err(SimulationError::Circuit(format!(
                "AC voltage source '{source}' was not found"
            )));
        }
        Self::for_branch(circuit, source)
    }

    /// Unit voltage across a branch equation, including current-output TF probes.
    pub(in crate::engine) fn for_branch(
        circuit: &CircuitData,
        source: &str,
    ) -> Result<Self, SimulationError> {
        let ordinal = circuit.get_branch_by_name(source).ok_or_else(|| {
            SimulationError::Circuit(format!("AC branch '{source}' was not found"))
        })?;
        let projection = AcVoltageConstraintProjection::with_excitation(circuit, |candidate| {
            Complex64::new(
                if circuit.voltage_sources.branch_indices[candidate] == ordinal {
                    1.0
                } else {
                    0.0
                },
                0.0,
            )
        })?;
        let branch = circuit.get_branch_matrix_index(ordinal);
        let row = branch.checked_sub(1).ok_or_else(|| {
            SimulationError::Circuit(format!("AC voltage source '{source}' has no equation row"))
        })?;
        let mut rhs = vec![Complex64::new(0.0, 0.0); circuit.matrix_size()];
        let entry = rhs.get_mut(row).ok_or_else(|| {
            SimulationError::Circuit(format!(
                "AC voltage source '{source}' equation lies outside the solved system"
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
        analysis: CapabilityAnalysis,
        abort: &dyn AbortSignal,
    ) -> Result<PreparedAc, SimulationError> {
        let circuit = self.build_circuit_with_abort(netlist, abort)?;
        self.prepare_ac_circuit(netlist, circuit, analysis, abort)
    }

    /// SP retains elaborated port metadata before initializing this same AC state.
    pub(in crate::engine) fn prepare_ac_circuit(
        &self,
        netlist: &Netlist,
        circuit: CircuitData,
        analysis: CapabilityAnalysis,
        abort: &dyn AbortSignal,
    ) -> Result<PreparedAc, SimulationError> {
        self.prepare_ac_circuit_with_excitation(netlist, circuit, None, analysis, abort)
    }

    /// An explicit excitation suppresses authored AC drives after elaboration,
    /// without changing the netlist, device topology, or operating point.
    ///
    /// `analysis` is the caller's own analysis — AC, S-parameters, transfer
    /// function or AC sensitivity — which is what a refusal raised while the
    /// shared bias is prepared names.
    pub(in crate::engine) fn prepare_ac_circuit_with_excitation(
        &self,
        netlist: &Netlist,
        circuit: CircuitData,
        excitation: Option<AcExcitation>,
        analysis: CapabilityAnalysis,
        abort: &dyn AbortSignal,
    ) -> Result<PreparedAc, SimulationError> {
        // The excitation is read from the elaborated topology, before the
        // bias is solved: an ideal-source loop has no unique AC branch
        // current, and saying so names the source, where the singular DC
        // solve it would otherwise reach first names none.
        Self::ensure_analysis_capability(&circuit, analysis)?;
        let excitation = match excitation {
            Some(excitation) => excitation,
            None => AcExcitation::new(&circuit)?,
        };
        let SmallSignalBias {
            circuit,
            matrix,
            bias: dc_solution,
        } = self.prepare_small_signal_bias(
            netlist,
            Some(circuit),
            analysis,
            1,
            BiasSource::OperatingPoint,
            abort,
        )?;

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
