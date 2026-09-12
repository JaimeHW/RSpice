//! Linear, nonlinear, and transient operating-point solve entry points.

use super::continuation::explicit_source_continuation_policy;
use super::*;
use crate::SpiceDialect;
use crate::engine::core::{StartupVoltageConstraint, StartupVoltageHints};

/// How many deficient rows the prose names before it summarizes the rest.
pub(in crate::engine::convergence) const SINGULAR_ROWS_SHOWN: usize = 8;

/// Report a device that could not evaluate finitely in preference to an
/// iteration count.
///
/// Once the direct Newton has rejected an iterate for a non-finite device
/// evaluation, every convergence aid is deforming the same unevaluable
/// equations; each reports its own exhausted budget. That number tells the
/// user nothing the original diagnostic does not tell them better.
///
/// An exhausted budget is the only thing this replaces. An aid also returns
/// outcomes that are not about the iterate at all: a cancellation or an
/// expired time budget, which say the caller stopped the run, and structural
/// `Circuit`/`Solver` failures, which describe a different fault than the one
/// the direct Newton could not evaluate. Replacing a stop loses the outcome --
/// `is_stopped()` answers false, so the materializer files a cancelled run as
/// a broken deck and a cancelled sweep takes its next point -- and replacing a
/// structural failure hides it behind a stale diagnostic.
fn prefer_nonfinite_trial_failure(
    fallback: SimulationError,
    nonfinite_direct_failure: &Option<String>,
) -> SimulationError {
    match (&fallback, nonfinite_direct_failure) {
        (SimulationError::ConvergenceFailed(_), Some(detail)) => {
            SimulationError::Circuit(detail.clone())
        }
        _ => fallback,
    }
}

/// Name the matrix rows behind a singular linear system so the user sees
/// which node or branch carries no constraining equation instead of a bare
/// "matrix is singular".
///
/// Name the deficient rows once, for both the sentence and the attribution.
///
/// Two readers of one fact: the prose a person reads and the site list a
/// canvas marks. Deriving both here keeps them from drifting into naming
/// different rows for the same singular system.
fn singular_system_failure(
    circuit: &CircuitData,
    matrix: &StaticMatrix,
) -> (String, Option<ConvergenceDiagnostic>) {
    let rows = matrix.deficient_rows();
    if rows.is_empty() {
        return (
            "matrix is singular with no structurally empty rows; the usual causes are \
             loops of ideal voltage sources/inductors or duplicate constraints on one \
             node pair (run `rspice check` for a topology report)"
                .to_string(),
            None,
        );
    }
    let node_names = circuit.node_names_sorted();
    let branch_names = circuit.branch_names_sorted();
    let num_nodes = circuit.num_nodes();
    let named = rows.len().min(ConvergenceDiagnostic::MAX_NAMED_SITES);
    let sites: Vec<ConvergenceSite> = rows[..named]
        .iter()
        .map(|&row| {
            if row < num_nodes {
                ConvergenceSite {
                    name: match node_names.get(row) {
                        Some(name) => name.clone(),
                        None => format!("#{}", row + 1),
                    },
                    kind: ConvergenceSiteKind::Node,
                    residual: None,
                }
            } else {
                ConvergenceSite {
                    name: match branch_names.get(row - num_nodes) {
                        Some(name) => name.clone(),
                        None => format!("#{}", row - num_nodes + 1),
                    },
                    kind: ConvergenceSiteKind::Branch,
                    residual: None,
                }
            }
        })
        .collect();
    let mut shown: Vec<String> = sites
        .iter()
        .take(SINGULAR_ROWS_SHOWN)
        .map(|site| {
            let noun = match site.kind {
                ConvergenceSiteKind::Node => "node",
                ConvergenceSiteKind::Branch => "branch",
            };
            // An unnamed row is shown by position, unquoted, exactly as it
            // was before this list gained a second reader.
            if site.name.starts_with('#') {
                format!("{noun} {}", site.name)
            } else {
                format!("{noun} '{}'", site.name)
            }
        })
        .collect();
    if rows.len() > SINGULAR_ROWS_SHOWN {
        shown.push(format!("... {} more", rows.len() - SINGULAR_ROWS_SHOWN));
    }
    let message = format!(
        "matrix is singular: no equation constrains {} — the node(s) are floating or the \
         element wiring is inconsistent",
        shown.join(", ")
    );
    let diagnostic = ConvergenceDiagnostic {
        class: ConvergenceFailureClass::SingularSystem,
        sites,
        elided_sites: rows.len() - named,
        failure_message: message.clone(),
    };
    (message, Some(diagnostic))
}

impl Engine {
    pub(in crate::engine::convergence) fn dc_solve_denominator_floors(
        circuit: &CircuitData,
        size: usize,
    ) -> Option<Vec<Value>> {
        (!circuit.behavioral_sources.is_empty()).then(|| vec![1.0; size])
    }

    pub(in crate::engine) fn requires_vbic_correction_form(circuit: &CircuitData) -> bool {
        // A milliohm branch can connect nearly equal voltages while carrying
        // nanoamperes. Preserve those small currents by solving for increments.
        // Thermal derivatives near an Early-voltage cutoff can also make
        // absolute companions too large to retain physical branch currents.
        circuit.bjts.devices.iter().any(|bjt| bjt.mna_promoted())
    }

    pub(in crate::engine::convergence) fn solve_direct_dc_correction(
        matrix: &mut StaticMatrix,
        rhs: &[Value],
        denominator_floors: Option<&[Value]>,
        anchor: &[Value],
        solution: &mut Vec<Value>,
    ) -> Result<(), SimulationError> {
        Self::solve_dc_linearization_system(matrix, rhs, denominator_floors, solution)?;
        for (value, previous) in solution.iter_mut().zip(anchor) {
            *value += previous;
        }
        Ok(())
    }

    pub(in crate::engine::convergence) fn solve_dc_linearization_system(
        matrix: &mut StaticMatrix,
        rhs: &[Value],
        denominator_floors: Option<&[Value]>,
        solution: &mut Vec<Value>,
    ) -> Result<(), SimulationError> {
        match matrix.solve_into(rhs, solution) {
            Ok(()) => Ok(()),
            Err(crate::solver::SolverError::InaccurateSolution(original_error)) => {
                if rhs.len() <= 128 {
                    log::debug!(
                        "sparse DC solve failed strict backward-error certification; retrying the small system with extended precision"
                    );
                    if let Ok(extended_solution) = matrix.solve_dense_extended(rhs) {
                        *solution = extended_solution;
                        return Ok(());
                    }
                }
                let Some(denominator_floors) = denominator_floors else {
                    return Err(SimulationError::Solver(
                        crate::solver::SolverError::InaccurateSolution(original_error),
                    ));
                };
                log::debug!(
                    "strict algebraic backward-error check rejected a behavioral DC Jacobian; retrying with physical row scales before the circuit residual audit"
                );
                matrix
                    .solve_into_with_row_denominator_floors(rhs, denominator_floors, solution)
                    .map_err(SimulationError::Solver)
            }
            Err(error) => Err(SimulationError::Solver(error)),
        }
    }

    /// Refuse a singular system, recording which rows carry no equation.
    fn singular_system_error(
        &self,
        circuit: &CircuitData,
        matrix: &StaticMatrix,
    ) -> SimulationError {
        let (message, diagnostic) = singular_system_failure(circuit, matrix);
        if let Some(diagnostic) = diagnostic {
            self.record_convergence(|quality| quality.record_failure_diagnostic(diagnostic));
        }
        SimulationError::Circuit(message)
    }

    /// Solve a linear circuit (no nonlinear devices)
    pub(crate) fn solve_linear(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
    ) -> Result<Vec<Value>, SimulationError> {
        let size = circuit.matrix_size();
        let mut rhs = vec![0.0; size];

        matrix.clear_values();
        rhs.fill(0.0);
        let gmin_floor = self.dc_nodal_gmin_floor(circuit);
        self.stamp_dc_direct(circuit, matrix, &mut rhs, gmin_floor);
        if !circuit.behavioral_sources.is_empty()
            && !circuit.behavioral_sources.has_solution_dependent_sources()
        {
            let zero_solution = vec![0.0; size];
            circuit
                .stamp_behavioral_sources(matrix, &mut rhs, &zero_solution, 0.0)
                .map_err(SimulationError::Circuit)?;
        }

        let direct_result = matrix.solve(&rhs);
        if let Ok(sol) = direct_result {
            return Ok(sol);
        }

        let mut last_err = direct_result.expect_err("checked Err branch");
        let conv_cfg = &self.config.convergence_config;

        if conv_cfg.gmin_stepping {
            self.record_convergence(|quality| {
                if quality.gmin_stepping_count == 0 {
                    log::warn!(
                        "direct DC solve failed; falling back to gmin stepping.                          The operating point is reached by relaxing device                          conductances, so check it against expectations."
                    );
                }
                quality.record_gmin_stepping();
            });
            match self.gmin_stepping(circuit, matrix) {
                Ok(sol) => return Ok(sol),
                Err(e) => {
                    last_err = e;
                }
            }
        }

        if conv_cfg.source_stepping {
            self.record_convergence(|quality| {
                if quality.source_stepping_count == 0 {
                    log::warn!("gmin stepping did not converge; falling back to source stepping.");
                }
                quality.record_source_stepping();
            });
            return self.source_stepping(circuit, matrix).map_err(|err| {
                if matches!(err, crate::solver::SolverError::SingularMatrix) {
                    self.singular_system_error(circuit, matrix)
                } else {
                    SimulationError::Solver(err)
                }
            });
        }

        if matches!(last_err, crate::solver::SolverError::SingularMatrix) {
            return Err(self.singular_system_error(circuit, matrix));
        }
        Err(SimulationError::Solver(last_err))
    }

    /// Solve nonlinear DC with optional node-voltage hint overrides.
    ///
    /// Performs a linear pre-solve to get a warm-start initial guess, which
    /// helps convergence especially for BJT circuits where starting from 0V
    /// puts the transistor in an unphysical state. `node_hints` entries are
    /// independent difference equations with node IDs using the standard
    /// 1-based non-ground circuit numbering (zero is ground).
    pub(crate) fn solve_nonlinear_with_node_hints_and_abort(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        hints: &StartupVoltageHints,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, SimulationError> {
        let node_hints = &hints.constraints;
        let size = circuit.matrix_size();
        let xyce_zero_start = self.config.spice_dialect == crate::engine::SpiceDialect::Xyce;
        let entry_state = xyce_zero_start.then(|| circuit.nonlinear_state_snapshot());
        // Xyce's DCOP Newton solve starts from the zero global solution and
        // lets each device apply its init-junction policy locally. In
        // particular, its GP BJT writes tVCrit only into the device's VBE
        // state; a resistor-only presolve followed by a generic 0.7 V BJT
        // correction takes a different NOX path and can produce a different
        // legitimately accepted approximate operating point. Native and
        // ngspice modes retain RSpice's robust linear warm start.
        let mut initial_guess = if xyce_zero_start {
            vec![0.0; size]
        } else {
            self.robust_operating_point_initial_guess(circuit, matrix, size)
        };
        Self::seed_node_voltage_hints(circuit, &mut initial_guess, node_hints);

        let primary = self.solve_nonlinear_from_seed_with_node_hints(
            circuit,
            matrix,
            initial_guess,
            hints,
            abort,
        );
        match primary {
            Ok(solution) => Ok(solution),
            Err(SimulationError::Aborted) => Err(SimulationError::Aborted),
            Err(primary_error)
                if xyce_zero_start && Self::is_recoverable_startup_error(&primary_error) =>
            {
                if abort.is_aborted() {
                    return Err(SimulationError::Aborted);
                }
                if let Some(entry_state) = entry_state {
                    circuit.restore_nonlinear_state(entry_state);
                }
                let mut recovery = self.robust_operating_point_initial_guess(circuit, matrix, size);
                Self::seed_node_voltage_hints(circuit, &mut recovery, node_hints);
                log::debug!(
                    "Xyce-compatible zero-start DC solve failed ({primary_error}); retrying from the robust linear operating-point seed."
                );
                self.solve_nonlinear_from_seed_with_node_hints(
                    circuit, matrix, recovery, hints, abort,
                )
            }
            Err(error) => Err(error),
        }
    }

    pub(in crate::engine) fn is_recoverable_startup_error(error: &SimulationError) -> bool {
        match error {
            SimulationError::ConvergenceFailed(_) => true,
            SimulationError::Solver(crate::solver::SolverError::InvalidCircuit(_)) => false,
            SimulationError::Solver(_) => true,
            _ => false,
        }
    }

    fn robust_operating_point_initial_guess(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        size: usize,
    ) -> Vec<Value> {
        let linear_presolve = self.linear_presolve_for_guess(circuit, matrix);
        let seed_neutral_bjt_junctions = linear_presolve.is_none();
        let mut guess = linear_presolve.unwrap_or_else(|| vec![0.0; size]);
        Self::apply_bjt_initial_guess_correction(&mut guess, circuit, seed_neutral_bjt_junctions);
        Self::apply_b3soi_pd_initial_guess_correction(&mut guess, circuit);
        Self::apply_bsim4_internal_gate_initial_guess_correction(&mut guess, circuit);
        Self::apply_vbic_internal_initial_guess_correction(&mut guess, circuit);
        guess
    }

    fn solve_nonlinear_from_seed_with_node_hints(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        mut initial_guess: Vec<Value>,
        hints: &StartupVoltageHints,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, SimulationError> {
        let node_hints = &hints.constraints;
        if hints.has_nodesets || !node_hints.is_empty() {
            match Self::with_nodeset_phase(circuit, hints.has_nodesets, |circuit| {
                self.solve_nonlinear_dc_startup_with_constraints_and_abort(
                    circuit,
                    matrix,
                    &initial_guess,
                    node_hints,
                    abort,
                )
            }) {
                Ok(nodeset_solution) => {
                    initial_guess = nodeset_solution;
                }
                Err(err) if Self::is_recoverable_startup_error(&err) => {
                    log::debug!(
                        "NODESET-constrained DC startup did not converge: {err}; continuing from hinted seed."
                    );
                }
                Err(err) => return Err(err),
            }
        }

        self.solve_nonlinear_with_guess_and_abort(circuit, matrix, Some(&initial_guess), abort)
    }

    /// The nodeset qualifier belongs to the temporary model/authored nodeset
    /// interval, not the generic constrained Newton solver (also used for
    /// hard `.IC` clamps and heuristic seeds). Restore equilibrium after
    /// either outcome; a restoration error must also reach the caller.
    fn with_nodeset_phase<T>(
        circuit: &mut CircuitData,
        active: bool,
        solve: impl FnOnce(&mut CircuitData) -> Result<T, SimulationError>,
    ) -> Result<T, SimulationError> {
        use rspice_veriloga_runtime::AnalogAnalysisPhase;
        if !active {
            return solve(circuit);
        }
        let result = circuit
            .set_veriloga_analysis_phase(AnalogAnalysisPhase::Nodeset)
            .map_err(SimulationError::Circuit)
            .and_then(|()| solve(circuit));
        circuit
            .set_veriloga_analysis_phase(AnalogAnalysisPhase::Equilibrium)
            .map_err(SimulationError::Circuit)?;
        result
    }

    /// Conductance ngspice's `cktload.c` clamps a constrained node with when
    /// the node's row still carries a branch current.
    const NODE_VOLTAGE_CLAMP_CONDUCTANCE: Value = 1.0e10;

    fn apply_node_voltage_constraints(
        circuit: &CircuitData,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        node_hints: &[StartupVoltageConstraint],
        reference_solution: &[Value],
    ) -> Result<(), SimulationError> {
        Self::apply_node_voltage_constraints_at(
            circuit,
            matrix,
            rhs,
            node_hints,
            reference_solution,
            false,
        )
    }

    fn apply_node_voltage_correction_constraints(
        circuit: &CircuitData,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        node_hints: &[StartupVoltageConstraint],
        reference_solution: &[Value],
    ) -> Result<(), SimulationError> {
        Self::apply_node_voltage_constraints_at(
            circuit,
            matrix,
            rhs,
            node_hints,
            reference_solution,
            true,
        )
    }

    fn apply_node_voltage_constraints_at(
        circuit: &CircuitData,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        node_hints: &[StartupVoltageConstraint],
        reference_solution: &[Value],
        correction: bool,
    ) -> Result<(), SimulationError> {
        let first_current_column = circuit.num_nodes();
        for constraint in node_hints {
            let (node_id, reference_id, voltage) =
                Self::startup_constraint_clamp_orientation(circuit, matrix, constraint);
            if !constraint.voltage.is_finite()
                || node_id == 0
                || node_id > circuit.num_nodes()
                || reference_id > circuit.num_nodes()
            {
                continue;
            }
            let row = node_id - 1;
            if row >= rhs.len() {
                continue;
            }
            let diagonal = matrix
                .force_voltage_clamp_row(
                    row,
                    first_current_column,
                    Self::NODE_VOLTAGE_CLAMP_CONDUCTANCE,
                )
                .map_err(SimulationError::Solver)?;
            let reference = if reference_id == 0 {
                0.0
            } else {
                reference_solution
                    .get(reference_id - 1)
                    .copied()
                    .unwrap_or(0.0)
            };
            rhs[row] = if correction {
                diagonal * (reference + voltage - reference_solution[row])
                    - matrix.row_product_from_column(
                        row,
                        first_current_column,
                        reference_solution,
                    )?
            } else {
                diagonal * (reference + voltage)
            };
        }
        Ok(())
    }

    /// Whether a clamped row actually pins its node's voltage.
    ///
    /// A node whose row also carries a branch current keeps that branch's own
    /// equation, and an ideal source on it simply outvotes the clamp — ngspice
    /// reports the source's voltage there, not the authored one. Only a row
    /// that became an exact identity pins the node, so only that value may be
    /// written back onto a solve.
    fn node_voltage_constraint_pins_node(
        circuit: &CircuitData,
        matrix: &StaticMatrix,
        node_id: usize,
    ) -> bool {
        let first_current_column = circuit.num_nodes();
        node_id > 0
            && node_id <= first_current_column
            && !matrix.row_has_position_from(node_id - 1, first_current_column)
    }

    /// Choose the constraint terminal whose KCL row can safely become the
    /// temporary startup equation. A terminal carrying an ideal branch
    /// current remains the reference when the opposite terminal has an
    /// ordinary nodal row, preserving both the authored difference and the
    /// ideal source equation.
    fn startup_constraint_clamp_orientation(
        circuit: &CircuitData,
        matrix: &StaticMatrix,
        constraint: &StartupVoltageConstraint,
    ) -> (usize, usize, Value) {
        let first_current_column = circuit.num_nodes();
        let positive_has_branch = constraint.positive > 0
            && matrix.row_has_position_from(constraint.positive - 1, first_current_column);
        let negative_has_branch = constraint.negative > 0
            && matrix.row_has_position_from(constraint.negative - 1, first_current_column);
        if positive_has_branch && constraint.negative > 0 && !negative_has_branch {
            (
                constraint.negative,
                constraint.positive,
                -constraint.voltage,
            )
        } else {
            (constraint.positive, constraint.negative, constraint.voltage)
        }
    }

    fn enforce_node_voltage_hints(
        circuit: &CircuitData,
        matrix: &StaticMatrix,
        solution: &mut [Value],
        node_hints: &[StartupVoltageConstraint],
    ) {
        for constraint in node_hints {
            let (node_id, reference_id, voltage) =
                Self::startup_constraint_clamp_orientation(circuit, matrix, constraint);
            if !constraint.voltage.is_finite()
                || !Self::node_voltage_constraint_pins_node(circuit, matrix, node_id)
            {
                continue;
            }
            let reference = if reference_id == 0 {
                0.0
            } else {
                solution.get(reference_id - 1).copied().unwrap_or(0.0)
            };
            if let Some(slot) = solution.get_mut(node_id - 1) {
                *slot = reference + voltage;
            }
        }
    }

    /// Seed a starting guess at the authored values.
    ///
    /// Unlike [`Self::enforce_node_voltage_hints`] this touches a guess rather
    /// than a solve, so it applies to every hinted node: an outvoted clamp
    /// still makes a reasonable place to start Newton.
    fn seed_node_voltage_hints(
        circuit: &CircuitData,
        solution: &mut [Value],
        node_hints: &[StartupVoltageConstraint],
    ) {
        for constraint in node_hints {
            let node_id = constraint.positive;
            if !constraint.voltage.is_finite()
                || node_id == 0
                || node_id > circuit.num_nodes()
                || constraint.negative > circuit.num_nodes()
            {
                continue;
            }
            let reference = if constraint.negative == 0 {
                0.0
            } else {
                solution
                    .get(constraint.negative - 1)
                    .copied()
                    .unwrap_or(0.0)
            };
            if let Some(slot) = solution.get_mut(node_id - 1) {
                *slot = reference + constraint.voltage;
            }
        }
    }

    fn constrained_dc_residual_converged(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
        node_hints: &[StartupVoltageConstraint],
    ) -> Result<bool, SimulationError> {
        let snapshot = circuit.nonlinear_state_snapshot();
        let gmin_floor = self.dc_nodal_gmin_floor(circuit);
        let junction_gmin =
            self.effective_device_junction_gmin(self.config.convergence_config.gmin_target);
        let result = matrix.with_probe_values(|probe, rhs| -> Result<bool, SimulationError> {
            Self::stamp_nodal_gmin(circuit, probe, gmin_floor);
            circuit.stamp_dc_direct(probe, rhs);
            if Self::requires_vbic_correction_form(circuit) {
                let mut correction_rhs = Vec::new();
                self.try_stamp_operating_point_correction(
                    circuit,
                    probe,
                    rhs,
                    OperatingPointProbe {
                        solution,
                        time: 0.0,
                        analysis: crate::xspice::AnalysisType::DcOp,
                        junction_gmin,
                    },
                    true,
                    &mut correction_rhs,
                )?;
                Self::apply_node_voltage_correction_constraints(
                    circuit,
                    probe,
                    &mut correction_rhs,
                    node_hints,
                    solution,
                )?;
                return Ok(self.direct_probe_correction_converged(
                    circuit,
                    probe,
                    solution,
                    &correction_rhs,
                ));
            }
            self.try_stamp_static_probe_nonlinear_devices_for_dc_with_junction_gmin(
                circuit,
                probe,
                rhs,
                solution,
                junction_gmin,
            )?;
            Self::apply_node_voltage_constraints(circuit, probe, rhs, node_hints, solution)?;
            Ok(self.residual_probe_fixed_point_converged(circuit, probe, solution, rhs))
        });
        circuit.restore_nonlinear_state(snapshot);
        result
    }

    fn constrained_transient_op_residual_converged(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
        time: Value,
        node_hints: &[StartupVoltageConstraint],
        conductances: TransientOpConductances,
    ) -> Result<bool, SimulationError> {
        let TransientOpConductances {
            nodal_gmin,
            junction_gmin,
            use_transient_current_seed,
        } = conductances;
        let snapshot = circuit.nonlinear_state_snapshot();
        let result = matrix.with_probe_values(|probe, rhs| -> Result<bool, SimulationError> {
            circuit.refresh_jiles_atherton_inductances(solution);
            if use_transient_current_seed {
                Self::stamp_transient_current_seed_linear(
                    circuit, probe, rhs, time, nodal_gmin, false,
                );
            } else {
                Self::stamp_transient_operating_point_linear(
                    circuit, probe, rhs, time, nodal_gmin, false,
                );
            }
            let mut correction_rhs = Vec::new();
            self.try_stamp_operating_point_newton_system(
                circuit,
                probe,
                rhs,
                OperatingPointProbe {
                    solution,
                    time,
                    analysis: crate::xspice::AnalysisType::Transient,
                    junction_gmin,
                },
                true,
                &mut correction_rhs,
            )?;
            if Self::requires_vbic_correction_form(circuit) {
                Self::apply_node_voltage_correction_constraints(
                    circuit, probe, rhs, node_hints, solution,
                )?;
                return Ok(self.direct_probe_correction_converged(circuit, probe, solution, rhs));
            }
            Self::apply_node_voltage_constraints(circuit, probe, rhs, node_hints, solution)?;
            Ok(self.residual_probe_fixed_point_converged(circuit, probe, solution, rhs))
        });
        circuit.restore_nonlinear_state(snapshot);
        result
    }

    pub(in crate::engine) fn solve_nonlinear_dc_startup_with_constraints_and_abort(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        initial_guess: &[Value],
        node_hints: &[StartupVoltageConstraint],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, SimulationError> {
        let size = circuit.matrix_size();
        let node_count = circuit.num_nodes().min(size);
        let mut solution = Self::sanitize_initial_guess(initial_guess, size);
        Self::seed_node_voltage_hints(circuit, &mut solution, node_hints);
        let accepted_reference = solution.clone();
        self.prime_operating_point_seed(circuit, &solution, 0.0, crate::xspice::AnalysisType::DcOp);

        let mut rhs = vec![0.0; size];
        let mut new_solution = Vec::with_capacity(size);
        let mut correction_rhs = Vec::new();
        let uses_vbic_correction = Self::requires_vbic_correction_form(circuit);
        let solve_denominator_floors = Self::dc_solve_denominator_floors(circuit, size);
        let gmin_floor = self.dc_nodal_gmin_floor(circuit);
        let max_iterations = self.continuation_iteration_budget(1, 64);

        for iteration in 0..max_iterations {
            if Self::should_abort_iteration(abort, iteration) {
                return Err(SimulationError::Aborted);
            }

            matrix.clear_values();
            rhs.fill(0.0);
            Self::stamp_nodal_gmin(circuit, matrix, gmin_floor);
            circuit.stamp_dc_direct(matrix, &mut rhs);
            self.try_stamp_operating_point_newton_system(
                circuit,
                matrix,
                &mut rhs,
                OperatingPointProbe {
                    solution: &solution,
                    time: 0.0,
                    analysis: crate::xspice::AnalysisType::DcOp,
                    junction_gmin: self
                        .effective_device_junction_gmin(self.config.convergence_config.gmin_target),
                },
                false,
                &mut correction_rhs,
            )?;
            Self::apply_node_voltage_constraints_at(
                circuit,
                matrix,
                &mut rhs,
                node_hints,
                &solution,
                uses_vbic_correction,
            )?;
            if uses_vbic_correction {
                Self::solve_direct_dc_correction(
                    matrix,
                    &rhs,
                    solve_denominator_floors.as_deref(),
                    &solution,
                    &mut new_solution,
                )?;
            } else {
                Self::solve_dc_linearization_system(
                    matrix,
                    &rhs,
                    solve_denominator_floors.as_deref(),
                    &mut new_solution,
                )?;
            }
            Self::reset_nonfinite_values(&mut new_solution);
            Self::enforce_node_voltage_hints(circuit, matrix, &mut new_solution, node_hints);

            let voltage_converged = self.dc_newton_update_convergence_met(
                &solution,
                &new_solution,
                &accepted_reference,
                node_count,
                iteration,
            );
            self.update_device_states_for_dc(circuit, &new_solution);
            let device_converged = circuit.nonlinear_converged(self.device_convergence_criteria());
            let nonlinear_residual_converged = voltage_converged
                && device_converged
                && self.constrained_dc_residual_converged(
                    circuit,
                    matrix,
                    &new_solution,
                    node_hints,
                )?;

            std::mem::swap(&mut solution, &mut new_solution);
            if voltage_converged && device_converged && nonlinear_residual_converged {
                return Ok(solution);
            }
        }

        Err(SimulationError::ConvergenceFailed(max_iterations))
    }

    /// Solve a nonlinear circuit using Newton-Raphson iteration with optional initial guess
    ///
    /// # Arguments
    /// * `circuit` - Circuit data with nonlinear devices
    /// * `matrix` - Sparse matrix structure for MNA
    /// * `initial_guess` - Optional initial solution vector (e.g., from previous DC sweep point)
    ///
    /// Using a good initial guess (like the previous sweep point solution) significantly
    /// improves convergence speed and robustness for nonlinear circuits.
    ///
    /// # Returns
    /// The converged solution vector, or error if Newton-Raphson fails to converge.
    pub(crate) fn solve_nonlinear_with_guess_and_abort(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        initial_guess: Option<&[Value]>,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, SimulationError> {
        let size = circuit.matrix_size();
        let seed_was_nonfinite = initial_guess.is_some_and(|guess| {
            let normalized = Self::normalize_initial_guess(guess, size);
            Self::has_nonfinite_values(&normalized)
        });
        // Sanitize any warm-start seed before Newton so pathological presolve
        // artifacts do not launch the iteration from non-finite values.
        let mut solution = match initial_guess {
            Some(guess) => Self::sanitize_initial_guess(guess, size),
            None => {
                let mut guess = vec![0.0; size];
                Self::apply_bjt_initial_guess_correction(&mut guess, circuit, true);
                guess
            }
        };
        if seed_was_nonfinite {
            // Sanitizing a non-finite presolve resets the vector. Reapply the
            // compact-model startup seeds afterward so a floating nonlinear
            // node does not silently fall back to the same all-zero guess.
            Self::apply_bjt_initial_guess_correction(&mut solution, circuit, true);
            Self::apply_b3soi_pd_initial_guess_correction(&mut solution, circuit);
            Self::apply_bsim4_internal_gate_initial_guess_correction(&mut solution, circuit);
            Self::apply_vbic_internal_initial_guess_correction(&mut solution, circuit);
        }
        let startup_seed = solution.clone();
        self.prime_operating_point_seed(circuit, &solution, 0.0, crate::xspice::AnalysisType::DcOp);

        // An explicit simultaneous source-step request defines the nonlinear
        // solve path; it is not merely a fallback hint. Run its lambda=0..1
        // continuation before attempting the physical-system Newton solve.
        if let Some(policy) = explicit_source_continuation_policy(
            self.config.convergence_config.nonlinear_continuation,
        ) {
            return self.source_stepping_nonlinear_with_policy_and_abort(
                circuit, matrix, &solution, policy, abort,
            );
        }
        if let Some(mode) = self.config.convergence_config.nonlinear_continuation
            && mode != crate::config::NonlinearContinuationMode::Standard
        {
            return Err(SimulationError::Circuit(format!(
                "nonlinear continuation mode {mode:?} is parsed but has no native solver implementation"
            )));
        }
        let mut rhs = vec![0.0; size];
        let mut raw_solution = Vec::with_capacity(size);
        let mut correction_rhs = Vec::new();
        let mut linearized_delta = Vec::new();
        let uses_vbic_correction = Self::requires_vbic_correction_form(circuit);
        let solve_denominator_floors = Self::dc_solve_denominator_floors(circuit, size);
        // Newton-Raphson iteration
        let mut damping_state = NewtonDampingState::default();
        let gmin_floor = self.dc_nodal_gmin_floor(circuit);
        let requires_conservative_nonlinear_limiting =
            circuit.requires_conservative_solution_damping();
        // ngspice's flat Newton: when junction devices replace their own
        // iterate voltages (pnjlim), the full node step IS the algorithm and
        // merit-based step shrinking livelocks turn-on (the raw residual
        // transiently rises along the convergent direction).
        let junction_owns_steps = Self::junction_limiting_owns_newton_steps(circuit)
            || self.b3soi_limiter_owns_global_damping(circuit);
        // Allow additional iterations for damped nonlinear operating-point solves.
        let dc_max_iterations = self.nonlinear_iteration_budget(10);
        let mut direct_iterations = 0usize;
        let mut residual_stall_iterations = 0usize;
        let mut residual_stalled = false;
        let convergence_aids_enabled = {
            let config = &self.config.convergence_config;
            config.source_stepping
                || config.pseudo_transient
                || config.gmin_stepping
                || config.arc_length
        };
        let track_limit_cycles =
            convergence_aids_enabled && solution.len() <= Self::DC_LIMIT_CYCLE_MAX_TRACKED_VALUES;
        let mut recent_iterates: std::collections::VecDeque<Vec<Value>> =
            std::collections::VecDeque::with_capacity(
                Self::DC_LIMIT_CYCLE_HISTORY.min(dc_max_iterations),
            );
        let mut limit_cycle_hits = 0usize;
        let mut limit_cycle_detected = false;
        let mut direct_solver_error = None;
        for iteration in 0..dc_max_iterations {
            direct_iterations = iteration + 1;
            if Self::should_abort_iteration(abort, iteration) {
                return Err(SimulationError::Aborted);
            }
            // Debug trace first few iterations
            if iteration < 5 {
                log::debug!(
                    "Newton iter {}: V = {:?}",
                    iteration,
                    solution
                        .iter()
                        .take(circuit.num_nodes())
                        .map(|v| format!("{:.2}", v))
                        .collect::<Vec<_>>()
                );
            }
            // Clear matrix and RHS for this iteration
            matrix.clear_values();
            rhs.fill(0.0);
            let node_count = circuit.num_nodes().min(size);
            Self::stamp_nodal_gmin(circuit, matrix, gmin_floor);
            // Stamp linear devices
            circuit.stamp_dc_direct(matrix, &mut rhs);
            // A device that cannot evaluate finitely at this iterate has
            // rejected the iterate, not the circuit: the operating point is
            // exactly where GMIN and source stepping exist to walk in from a
            // reachable one. Leave through the same door a failed linear solve
            // uses so the continuation ladder below still runs; a structural
            // stamping failure keeps ending the solve immediately.
            if let Err(error) = self.try_stamp_operating_point_newton_system(
                circuit,
                matrix,
                &mut rhs,
                OperatingPointProbe {
                    solution: &solution,
                    time: 0.0,
                    analysis: crate::xspice::AnalysisType::DcOp,
                    junction_gmin: self
                        .effective_device_junction_gmin(self.config.convergence_config.gmin_target),
                },
                false,
                &mut correction_rhs,
            ) {
                if error.nonfinite_trial_detail().is_none() {
                    return Err(error);
                }
                log::debug!("Rejecting a non-finite DC trial iterate: {error}");
                direct_solver_error = Some(error);
                break;
            }
            let solve_result = if uses_vbic_correction {
                Self::solve_direct_dc_correction(
                    matrix,
                    &rhs,
                    solve_denominator_floors.as_deref(),
                    &solution,
                    &mut raw_solution,
                )
            } else {
                Self::solve_dc_linearization_system(
                    matrix,
                    &rhs,
                    solve_denominator_floors.as_deref(),
                    &mut raw_solution,
                )
            };
            match solve_result {
                Ok(()) => {}
                Err(err) => {
                    direct_solver_error = Some(err);
                    break;
                }
            }
            // Voltage-limiting style damping is critical for strongly-coupled
            // semiconductor nonlinearities, but it can unnecessarily throttle
            // behavioral-only fixed-point updates (e.g., B-source macros that
            // legitimately require kilovolt-level solution jumps).
            let should_project_damped_constraints =
                requires_conservative_nonlinear_limiting && !junction_owns_steps;
            let mut damped_solution;
            let new_solution = if should_project_damped_constraints {
                damped_solution = self.apply_damping_strategy_for_circuit(
                    circuit.has_b3soi_devices(),
                    &circuit.non_electrical_state_mask(),
                    DampingStep {
                        old: &solution,
                        proposal: &raw_solution,
                        damping_state: &mut damping_state,
                    },
                    junction_owns_steps,
                    |trial| self.nonlinear_merit(circuit, matrix, trial),
                );
                &mut damped_solution
            } else {
                &mut raw_solution
            };
            // A raw MNA solution already satisfies its ideal-source rows and
            // must remain untouched: re-projecting a controlled voltage after
            // the solve can sacrifice KCL at an adjacent stiff impedance.
            // Damping, in contrast, deliberately blends node values and must
            // restore the exact voltage constraints before the next iterate.
            if should_project_damped_constraints {
                circuit.enforce_dc_ideal_voltage_constraints(new_solution)?;
            }
            // Device limiting and the selected damping strategy govern finite
            // Newton updates. Absolute node voltages are not a physical domain
            // constraint; acceptance still requires the nonlinear residual.
            Self::reset_nonfinite_values(new_solution);
            // Check convergence (both voltage change and device convergence)
            let voltage_converged = self.dc_newton_update_convergence_met(
                &solution,
                new_solution,
                &startup_seed,
                node_count,
                iteration,
            );
            let linearized_residual_converged = if uses_vbic_correction {
                linearized_delta.clear();
                linearized_delta.extend(
                    new_solution
                        .iter()
                        .zip(&solution)
                        .map(|(next, old)| next - old),
                );
                self.residual_convergence_met(circuit, matrix, &linearized_delta, &rhs)
            } else {
                self.residual_convergence_met(circuit, matrix, new_solution, &rhs)
            };
            // Device convergence must be checked at the candidate iterate, not the prior iterate.
            self.update_device_states_for_dc(circuit, new_solution);
            let device_converged = circuit.nonlinear_converged(self.device_convergence_criteria());
            if std::env::var("RSPICE_DC_TRACE").as_deref() == Ok("1") {
                let max_dv = solution
                    .iter()
                    .zip(new_solution.iter())
                    .map(|(a, b)| (a - b).abs())
                    .fold(0.0_f64, f64::max);
                let limited = circuit
                    .bjts
                    .devices
                    .iter()
                    .filter(|bjt| bjt.legacy_junction_limited_for_trace())
                    .count();
                let nl_res = self.nonlinear_merit(circuit, matrix, new_solution);
                log::debug!(
                    "DCTRACE iter={iteration} max_dv={max_dv:.3e} vconv={voltage_converged} dconv={device_converged} linres={linearized_residual_converged} limited_bjts={limited} merit={nl_res:?}"
                );
            }
            let nonlinear_residual_converged = voltage_converged
                && device_converged
                && self.try_nonlinear_residual_converged(circuit, matrix, new_solution)?;
            let repeats_prior_iterate = track_limit_cycles
                && !voltage_converged
                && recent_iterates
                    .iter()
                    .rev()
                    .skip(Self::DC_LIMIT_CYCLE_MIN_PERIOD - 1)
                    .any(|prior| {
                        self.node_voltage_convergence_met(prior, new_solution, node_count)
                    });
            std::mem::swap(&mut solution, new_solution);
            if voltage_converged && device_converged && nonlinear_residual_converged {
                if self.config.spice_dialect != crate::engine::SpiceDialect::Xyce
                    && let Some(refined) =
                        self.refine_fallback_candidate(circuit, matrix, &solution, abort)?
                {
                    return Ok(refined);
                }
                return Ok(solution);
            }

            if repeats_prior_iterate {
                limit_cycle_hits += 1;
                if limit_cycle_hits >= Self::DC_LIMIT_CYCLE_HIT_LIMIT {
                    limit_cycle_detected = true;
                    break;
                }
            } else {
                limit_cycle_hits = 0;
            }
            if track_limit_cycles {
                if recent_iterates.len() == Self::DC_LIMIT_CYCLE_HISTORY {
                    recent_iterates.pop_front();
                }
                recent_iterates.push_back(solution.clone());
            }

            if voltage_converged && device_converged && !nonlinear_residual_converged {
                residual_stall_iterations += 1;
                if residual_stall_iterations >= Self::DC_RESIDUAL_STALL_LIMIT {
                    residual_stalled = true;
                    break;
                }
            } else {
                residual_stall_iterations = 0;
            }
        }
        // Name the KCL equations the abort iterate left worst-violated, while
        // the linearization that produced it is still stamped. Reaching here
        // means the direct Newton gave up — a converged solve returned from
        // inside the loop — so a passing run never pays for this. A rescued
        // run pays one residual pass and then discards it, because the
        // attribution is only recorded where the failure is actually
        // returned.
        let newton_failure = if uses_vbic_correction {
            // The last Newton RHS describes increments at the preceding
            // iterate. Re-evaluate physical currents at the abort iterate
            // before attributing a failed KCL equation.
            let snapshot = circuit.nonlinear_state_snapshot();
            let failure = matrix.with_probe_values(|probe, rhs| {
                Self::stamp_nodal_gmin(circuit, probe, gmin_floor);
                circuit.stamp_dc_direct(probe, rhs);
                if self
                    .try_stamp_operating_point_newton_system(
                        circuit,
                        probe,
                        rhs,
                        OperatingPointProbe {
                            solution: &solution,
                            time: 0.0,
                            analysis: crate::xspice::AnalysisType::DcOp,
                            junction_gmin: self.effective_device_junction_gmin(
                                self.config.convergence_config.gmin_target,
                            ),
                        },
                        true,
                        &mut correction_rhs,
                    )
                    .is_err()
                {
                    return (Vec::new(), 0);
                }
                self.newton_failure_sites(circuit, probe, &solution, rhs, true)
            });
            circuit.restore_nonlinear_state(snapshot);
            failure
        } else {
            self.newton_failure_sites(circuit, matrix, &solution, &rhs, false)
        };

        // Log diagnostic information when falling back to convergence aids.
        if let Some(detail) = direct_solver_error
            .as_ref()
            .and_then(SimulationError::nonfinite_trial_detail)
        {
            log::warn!(
                "DC Newton-Raphson rejected a non-finite device trial iterate after {} iteration(s): {}. Trying configured convergence aids...",
                direct_iterations.max(1),
                detail
            );
        } else if let Some(err) = direct_solver_error.as_ref() {
            log::warn!(
                "DC Newton-Raphson linear solve failed after {} iteration(s): {}. Trying configured convergence aids...",
                direct_iterations.max(1),
                err
            );
        } else if limit_cycle_detected {
            log::info!(
                "DC Newton-Raphson entered a repeated-iterate limit cycle after {} iterations. Trying configured convergence aids...",
                direct_iterations.max(1)
            );
        } else if residual_stalled {
            log::info!(
                "DC Newton-Raphson residual checks stalled after {} iterations. Trying configured convergence aids...",
                direct_iterations.max(1)
            );
        } else {
            log::info!(
                "DC Newton-Raphson did not converge after {} iterations. Trying configured convergence aids...",
                dc_max_iterations
            );
        }

        if residual_stalled
            && !circuit.has_b3soi_devices()
            && let Some(refined) =
                self.refine_fallback_candidate(circuit, matrix, &solution, abort)?
        {
            log::info!(
                "Residual-stalled DC Newton candidate accepted after static-device polishing."
            );
            return Ok(refined);
        }

        // Every aid below deforms the same equations the direct Newton could
        // not evaluate finitely, so each one fails for that same reason and
        // reports it as an iteration count. Keep the diagnostic that names the
        // instance, the operator and the voltages, and hand it back whichever
        // aid runs out first.
        let nonfinite_direct_failure = direct_solver_error
            .as_ref()
            .and_then(SimulationError::nonfinite_trial_detail)
            .map(str::to_owned);

        let conv_cfg = &self.config.convergence_config;
        let allow_source = conv_cfg.source_stepping;
        let allow_pseudo = conv_cfg.pseudo_transient;
        let allow_gmin = conv_cfg.gmin_stepping;
        let allow_arc = conv_cfg.arc_length;
        if !allow_source && !allow_pseudo && !allow_gmin && !allow_arc {
            if let Some(err) = direct_solver_error {
                return Err(err);
            }
            return Err(self.newton_non_convergence_error(newton_failure, dc_max_iterations));
        }

        // A finite linear presolve can be far from the nonlinear basin (for
        // example a current-driven, initially cut-off MOS channel). If its
        // damped Newton path stalls, retry from the device startup seeds before
        // deforming the equations. No voltage magnitude decides seed validity.
        if residual_stalled {
            let restart_state = circuit.nonlinear_state_snapshot();
            let mut restart_seed = vec![0.0; size];
            Self::apply_bjt_initial_guess_correction(&mut restart_seed, circuit, true);
            Self::apply_b3soi_pd_initial_guess_correction(&mut restart_seed, circuit);
            Self::apply_bsim4_internal_gate_initial_guess_correction(&mut restart_seed, circuit);
            Self::apply_vbic_internal_initial_guess_correction(&mut restart_seed, circuit);
            if restart_seed != startup_seed {
                self.prime_operating_point_seed(
                    circuit,
                    &restart_seed,
                    0.0,
                    crate::xspice::AnalysisType::DcOp,
                );
                if let Some(restarted) =
                    self.warm_restart_after_fallback(circuit, matrix, &restart_seed, abort)?
                {
                    log::info!("Residual-stalled DC Newton converged from device startup seeds.");
                    return Ok(restarted);
                }
            }
            circuit.restore_nonlinear_state(restart_state);
        }

        if limit_cycle_detected && !circuit.bjts.is_empty() {
            let hints = Self::legacy_bjt_half_bias_startup_hints(circuit, &startup_seed)
                .into_iter()
                .map(|(positive, voltage)| StartupVoltageConstraint {
                    positive,
                    negative: 0,
                    voltage,
                })
                .collect::<Vec<_>>();
            if std::env::var("RSPICE_DC_TRACE").as_deref() == Ok("1") {
                log::debug!(
                    "DCTRACE legacy_bjt_half_bias hints={hints:?} startup={:?}",
                    startup_seed
                        .iter()
                        .take(circuit.num_nodes())
                        .collect::<Vec<_>>()
                );
            }
            if !hints.is_empty() {
                let startup_state = circuit.nonlinear_state_snapshot();
                circuit.reset_legacy_bjt_operating_point_history();
                match self.solve_nonlinear_dc_startup_with_constraints_and_abort(
                    circuit,
                    matrix,
                    &startup_seed,
                    &hints,
                    abort,
                ) {
                    Ok(constrained_seed) => {
                        if std::env::var("RSPICE_DC_TRACE").as_deref() == Ok("1") {
                            log::debug!(
                                "DCTRACE legacy_bjt_half_bias constrained={:?}",
                                constrained_seed
                                    .iter()
                                    .take(circuit.num_nodes())
                                    .collect::<Vec<_>>()
                            );
                        }
                        match self.warm_restart_after_fallback(
                            circuit,
                            matrix,
                            &constrained_seed,
                            abort,
                        ) {
                            Ok(Some(restarted)) => {
                                log::info!(
                                    "Legacy BJT half-bias startup escaped a rail-limited DC cycle."
                                );
                                return Ok(restarted);
                            }
                            Ok(None) => {
                                if std::env::var("RSPICE_DC_TRACE").as_deref() == Ok("1") {
                                    log::debug!(
                                        "DCTRACE legacy_bjt_half_bias unconstrained restart rejected"
                                    );
                                }
                            }
                            Err(SimulationError::Aborted) => {
                                return Err(SimulationError::Aborted);
                            }
                            Err(error) => {
                                if std::env::var("RSPICE_DC_TRACE").as_deref() == Ok("1") {
                                    log::debug!(
                                        "DCTRACE legacy_bjt_half_bias restart_error={error}"
                                    );
                                }
                                log::debug!("Legacy BJT half-bias restart failed: {error}");
                            }
                        }
                    }
                    Err(SimulationError::Aborted) => return Err(SimulationError::Aborted),
                    Err(error) => {
                        if std::env::var("RSPICE_DC_TRACE").as_deref() == Ok("1") {
                            log::debug!("DCTRACE legacy_bjt_half_bias constrained_error={error}");
                        }
                        log::debug!("Legacy BJT half-bias constrained startup failed: {error}");
                    }
                }
                circuit.restore_nonlinear_state(startup_state);
            }
        }

        if let Some(legacy_seed) = self.legacy_hfet_inverse_branch_seed(circuit, &startup_seed) {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            if let Some(restarted) =
                self.warm_restart_after_fallback(circuit, matrix, &legacy_seed, abort)?
            {
                log::info!(
                    "Legacy HFET inverse-branch restart accepted after direct Newton failed."
                );
                return Ok(restarted);
            }
        }

        let zero_seed = vec![0.0; solution.len()];
        let mut fallback_seed = if circuit.has_b3soi_devices() {
            Self::sanitize_initial_guess(&solution, solution.len())
        } else {
            self.prefer_lower_merit_scaled_seed(circuit, matrix, &solution, &zero_seed, 1.0)
        };
        let prefer_gate_generation_aids = circuit.has_jfet_gate_generation_branches();
        let prefer_gmin_aids =
            prefer_gate_generation_aids || !circuit.b3soi.is_empty() || !circuit.bjts.is_empty();
        let mut gmin_attempted = false;

        if prefer_gmin_aids && allow_gmin {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            gmin_attempted = true;
            let gmin_state = circuit.nonlinear_state_snapshot();
            match self.gmin_stepping_nonlinear_with_abort(circuit, matrix, &fallback_seed, abort) {
                Ok(gmin_solution) => {
                    if let Some(candidate) = self.evaluate_fallback_candidate(
                        circuit,
                        matrix,
                        gmin_solution.clone(),
                        "GMIN stepping",
                        abort,
                    )? {
                        return Ok(candidate);
                    }
                    fallback_seed = self.prefer_lower_merit_scaled_seed(
                        circuit,
                        matrix,
                        &fallback_seed,
                        &gmin_solution,
                        1.0,
                    );
                    if let Some(restarted) =
                        self.warm_restart_after_fallback(circuit, matrix, &fallback_seed, abort)?
                    {
                        log::info!(
                            "GMIN stepping warmed the nonlinear state; direct Newton restart accepted."
                        );
                        return Ok(restarted);
                    }
                }
                Err(e) => {
                    circuit.restore_nonlinear_state(gmin_state);
                    log::warn!(
                        "Early GMIN stepping for weakly anchored nonlinear devices failed with {}. Continuing with configured aids.",
                        e
                    );
                }
            }
        }

        if circuit.has_b3soi_self_heating() && allow_source {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let thermal_state = circuit.nonlinear_state_snapshot();
            circuit.set_b3soi_self_heating_startup_disabled(true);
            circuit.reset_b3soi_operating_point_history();
            let mut electrical_seed = fallback_seed.clone();
            circuit.zero_b3soi_self_heating_temperature_guess(&mut electrical_seed);
            let node_count = circuit.num_nodes().min(electrical_seed.len());
            electrical_seed[node_count..].fill(0.0);
            let mut electrical_result = self.source_stepping_nonlinear_with_guess_and_abort(
                circuit,
                matrix,
                &electrical_seed,
                abort,
            );
            if electrical_result.is_err() && !abort.is_aborted() {
                circuit.reset_b3soi_operating_point_history();
                if let Some(direct_electrical_solution) =
                    self.warm_restart_after_fallback(circuit, matrix, &electrical_seed, abort)?
                {
                    electrical_result = Ok(direct_electrical_solution);
                }
            }
            circuit.restore_nonlinear_state(thermal_state);
            match electrical_result {
                Ok(electrical_solution) => {
                    let mut coupled_seed = electrical_solution;
                    circuit.seed_b3soi_self_heating_temperature_guess(&mut coupled_seed);
                    circuit.prime_b3soi_operating_point_from_solution(&coupled_seed);
                    if let Some(restarted) =
                        self.warm_restart_after_fallback(circuit, matrix, &coupled_seed, abort)?
                    {
                        log::info!(
                            "B3SOI electrothermal startup warmed the nonlinear state; direct Newton restart accepted."
                        );
                        return Ok(restarted);
                    }
                    if let Some(restarted) = self.static_probe_restart_after_fallback(
                        circuit,
                        matrix,
                        &coupled_seed,
                        abort,
                    )? {
                        log::info!(
                            "B3SOI electrothermal startup required static-probe polishing and is now accepted."
                        );
                        return Ok(restarted);
                    }
                    fallback_seed = self.prefer_lower_merit_scaled_seed(
                        circuit,
                        matrix,
                        &fallback_seed,
                        &coupled_seed,
                        1.0,
                    );
                }
                Err(e) => {
                    if abort.is_aborted() {
                        return Err(SimulationError::Aborted);
                    }
                    log::warn!(
                        "B3SOI electrothermal startup failed with {}. Continuing with configured aids.",
                        e
                    );
                }
            }
        }

        if allow_source {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let source_state = circuit.nonlinear_state_snapshot();
            match self.source_stepping_nonlinear_with_guess_and_abort(
                circuit,
                matrix,
                &fallback_seed,
                abort,
            ) {
                Ok(source_stepped) => {
                    log::info!(
                        "DC operating point after source stepping ({} nodes): {:?}",
                        source_stepped.len(),
                        source_stepped.iter().take(10).collect::<Vec<_>>()
                    );
                    if let Some(candidate) = self.evaluate_fallback_candidate(
                        circuit,
                        matrix,
                        source_stepped.clone(),
                        "Source stepping",
                        abort,
                    )? {
                        return Ok(candidate);
                    }
                    fallback_seed = self.prefer_lower_merit_scaled_seed(
                        circuit,
                        matrix,
                        &fallback_seed,
                        &source_stepped,
                        1.0,
                    );
                    if let Some(restarted) =
                        self.warm_restart_after_fallback(circuit, matrix, &fallback_seed, abort)?
                    {
                        log::info!(
                            "Source stepping warmed the nonlinear state; direct Newton restart accepted."
                        );
                        return Ok(restarted);
                    }
                }
                Err(e) => {
                    circuit.restore_nonlinear_state(source_state);
                    if !allow_pseudo && !allow_gmin && !allow_arc {
                        return Err(prefer_nonfinite_trial_failure(e, &nonfinite_direct_failure));
                    }
                    log::warn!(
                        "Source stepping failed with {}. Escalating to next configured aid.",
                        e
                    );
                }
            }
        }

        if allow_pseudo {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            match self.pseudo_transient_nonlinear_with_guess_and_abort(
                circuit,
                matrix,
                &fallback_seed,
                abort,
            ) {
                Ok(pseudo_solution) => {
                    log::info!(
                        "DC operating point after pseudo-transient continuation ({} nodes): {:?}",
                        pseudo_solution.len(),
                        pseudo_solution.iter().take(10).collect::<Vec<_>>()
                    );
                    if let Some(candidate) = self.evaluate_fallback_candidate(
                        circuit,
                        matrix,
                        pseudo_solution.clone(),
                        "Pseudo-transient continuation",
                        abort,
                    )? {
                        return Ok(candidate);
                    }
                    fallback_seed = self.prefer_lower_merit_scaled_seed(
                        circuit,
                        matrix,
                        &fallback_seed,
                        &pseudo_solution,
                        1.0,
                    );
                    if let Some(restarted) =
                        self.warm_restart_after_fallback(circuit, matrix, &fallback_seed, abort)?
                    {
                        log::info!(
                            "Pseudo-transient continuation warmed the nonlinear state; direct Newton restart accepted."
                        );
                        return Ok(restarted);
                    }
                }
                Err(e) => {
                    if !allow_gmin && !allow_arc {
                        return Err(prefer_nonfinite_trial_failure(e, &nonfinite_direct_failure));
                    }
                    log::warn!(
                        "Pseudo-transient continuation failed with {}. Escalating to next configured aid.",
                        e
                    );
                }
            }
        }

        if allow_gmin && !gmin_attempted {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let gmin_state = circuit.nonlinear_state_snapshot();
            match self.gmin_stepping_nonlinear_with_abort(circuit, matrix, &fallback_seed, abort) {
                Ok(gmin_solution) => {
                    if let Some(candidate) = self.evaluate_fallback_candidate(
                        circuit,
                        matrix,
                        gmin_solution.clone(),
                        "GMIN stepping",
                        abort,
                    )? {
                        return Ok(candidate);
                    }
                    fallback_seed = self.prefer_lower_merit_scaled_seed(
                        circuit,
                        matrix,
                        &fallback_seed,
                        &gmin_solution,
                        1.0,
                    );
                    if let Some(restarted) =
                        self.warm_restart_after_fallback(circuit, matrix, &fallback_seed, abort)?
                    {
                        log::info!(
                            "GMIN stepping warmed the nonlinear state; direct Newton restart accepted."
                        );
                        return Ok(restarted);
                    }
                }
                Err(e) => {
                    circuit.restore_nonlinear_state(gmin_state);
                    if !allow_arc {
                        return Err(prefer_nonfinite_trial_failure(e, &nonfinite_direct_failure));
                    }
                    log::warn!(
                        "GMIN stepping failed with {}. Escalating to arc-length continuation.",
                        e
                    );
                }
            }
        }

        if circuit.has_jfet_gate_generation_branches() {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            match self.gate_generation_stepping_nonlinear_with_abort(
                circuit,
                matrix,
                &fallback_seed,
                abort,
            ) {
                Ok(gate_solution) => {
                    log::info!(
                        "Gate generation continuation produced a DC operating-point candidate."
                    );
                    if let Some(candidate) = self.evaluate_fallback_candidate(
                        circuit,
                        matrix,
                        gate_solution.clone(),
                        "Gate generation continuation",
                        abort,
                    )? {
                        return Ok(candidate);
                    }
                    fallback_seed = self.prefer_lower_merit_scaled_seed(
                        circuit,
                        matrix,
                        &fallback_seed,
                        &gate_solution,
                        1.0,
                    );
                    if let Some(restarted) =
                        self.warm_restart_after_fallback(circuit, matrix, &fallback_seed, abort)?
                    {
                        log::info!(
                            "Gate generation continuation warmed the nonlinear state; direct Newton restart accepted."
                        );
                        return Ok(restarted);
                    }
                }
                Err(e) => {
                    if !allow_arc {
                        return Err(prefer_nonfinite_trial_failure(e, &nonfinite_direct_failure));
                    }
                    log::warn!(
                        "Gate generation continuation failed with {}. Escalating to arc-length continuation.",
                        e
                    );
                }
            }
        }

        if allow_arc {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            let arc_solution = self.arc_length_continuation_nonlinear_with_guess_and_abort(
                circuit,
                matrix,
                &fallback_seed,
                abort,
            )?;
            if let Some(candidate) = self.evaluate_fallback_candidate(
                circuit,
                matrix,
                arc_solution.clone(),
                "Arc-length continuation",
                abort,
            )? {
                return Ok(candidate);
            }
            if let Some(restarted) =
                self.warm_restart_after_fallback(circuit, matrix, &arc_solution, abort)?
            {
                log::info!(
                    "Arc-length continuation warmed the nonlinear state; direct Newton restart accepted."
                );
                return Ok(restarted);
            }
        }
        Err(prefer_nonfinite_trial_failure(
            self.newton_non_convergence_error(newton_failure, dc_max_iterations),
            &nonfinite_direct_failure,
        ))
    }

    pub(crate) fn solve_nonlinear_transient_op_startup_with_guess_and_hints_abort(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        time: Value,
        initial_guess: &[Value],
        node_hints: &[StartupVoltageConstraint],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, SimulationError> {
        let size = circuit.matrix_size();
        let node_count = circuit.num_nodes().min(size);
        let gmin_floor = self.dc_nodal_gmin_floor(circuit);
        let junction_gmin = self.effective_device_junction_gmin(gmin_floor);
        let mut solution = Self::sanitize_initial_guess(initial_guess, size);
        Self::seed_node_voltage_hints(circuit, &mut solution, node_hints);
        self.prime_operating_point_seed(
            circuit,
            &solution,
            time,
            crate::xspice::AnalysisType::Transient,
        );

        let mut rhs = vec![0.0; size];
        let mut new_solution = Vec::with_capacity(size);
        let mut correction_rhs = Vec::new();
        let uses_vbic_correction = Self::requires_vbic_correction_form(circuit);
        let max_iterations = self.continuation_iteration_budget(1, 64);

        for iteration in 0..max_iterations {
            if Self::should_abort_iteration(abort, iteration) {
                return Err(SimulationError::Aborted);
            }

            matrix.clear_values();
            rhs.fill(0.0);
            circuit.refresh_jiles_atherton_inductances(&solution);
            Self::stamp_transient_operating_point_linear(
                circuit, matrix, &mut rhs, time, gmin_floor, false,
            );
            self.try_stamp_operating_point_newton_system(
                circuit,
                matrix,
                &mut rhs,
                OperatingPointProbe {
                    solution: &solution,
                    time,
                    analysis: crate::xspice::AnalysisType::Transient,
                    junction_gmin,
                },
                false,
                &mut correction_rhs,
            )?;
            Self::apply_node_voltage_constraints_at(
                circuit,
                matrix,
                &mut rhs,
                node_hints,
                &solution,
                uses_vbic_correction,
            )?;

            if uses_vbic_correction {
                Self::solve_direct_dc_correction(matrix, &rhs, None, &solution, &mut new_solution)?;
            } else {
                matrix
                    .solve_into(&rhs, &mut new_solution)
                    .map_err(SimulationError::Solver)?;
            }
            Self::reset_nonfinite_values(&mut new_solution);
            Self::enforce_node_voltage_hints(circuit, matrix, &mut new_solution, node_hints);

            let voltage_converged =
                self.node_voltage_convergence_met(&solution, &new_solution, node_count);
            self.update_device_states_for_operating_point(
                circuit,
                OperatingPointProbe {
                    solution: &new_solution,
                    time,
                    analysis: crate::xspice::AnalysisType::Transient,
                    junction_gmin,
                },
            );
            let device_converged = circuit.nonlinear_converged(self.device_convergence_criteria());
            let nonlinear_residual_converged = voltage_converged
                && device_converged
                && self.constrained_transient_op_residual_converged(
                    circuit,
                    matrix,
                    &new_solution,
                    time,
                    node_hints,
                    TransientOpConductances {
                        nodal_gmin: gmin_floor,
                        junction_gmin,
                        use_transient_current_seed: false,
                    },
                )?;

            std::mem::swap(&mut solution, &mut new_solution);
            if voltage_converged && device_converged && nonlinear_residual_converged {
                return Ok(solution);
            }
        }

        Err(SimulationError::ConvergenceFailed(max_iterations))
    }

    pub(in crate::engine) fn solve_linear_transient_operating_point_with_abort(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        time: Value,
        abort: &dyn AbortSignal,
    ) -> Result<TransientOperatingPointSolution, SimulationError> {
        self.solve_linear_transient_operating_point_with_constraints_and_abort(
            circuit,
            matrix,
            time,
            &[],
            abort,
        )
    }

    pub(in crate::engine) fn solve_linear_transient_operating_point_with_constraints_and_abort(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        time: Value,
        node_constraints: &[StartupVoltageConstraint],
        abort: &dyn AbortSignal,
    ) -> Result<TransientOperatingPointSolution, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }

        let nodal_gmin =
            if self.config.spice_dialect == SpiceDialect::Xyce && !node_constraints.is_empty() {
                0.0
            } else {
                self.dc_nodal_gmin_floor(circuit)
            };
        let primary = self.solve_linear_transient_constraint_system(
            circuit,
            matrix,
            LinearTransientConstraintSolve {
                time,
                nodal_gmin,
                linear_system: TransientOperatingPointLinearSystem::IdealInductorShorts,
                constraints: node_constraints,
            },
            abort,
        );
        match primary {
            Ok(values) => Ok(TransientOperatingPointSolution {
                values,
                accepted_contract: node_constraints.is_empty().then_some(
                    AcceptedTransientOperatingPointContract {
                        linear_system: TransientOperatingPointLinearSystem::IdealInductorShorts,
                        nodal_gmin,
                        junction_gmin: None,
                    },
                ),
            }),
            Err(_) if !circuit.inductors.is_empty() => {
                match self.solve_linear_transient_constraint_system(
                    circuit,
                    matrix,
                    LinearTransientConstraintSolve {
                        time,
                        nodal_gmin,
                        linear_system: TransientOperatingPointLinearSystem::CurrentSeededInductors,
                        constraints: node_constraints,
                    },
                    abort,
                ) {
                    Ok(values) => Ok(TransientOperatingPointSolution {
                        values,
                        accepted_contract: node_constraints.is_empty().then_some(
                            AcceptedTransientOperatingPointContract {
                                linear_system:
                                    TransientOperatingPointLinearSystem::CurrentSeededInductors,
                                nodal_gmin,
                                junction_gmin: None,
                            },
                        ),
                    }),
                    Err(err) => Err(err),
                }
            }
            Err(err) => Err(err),
        }
    }

    fn solve_linear_transient_constraint_system(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solve: LinearTransientConstraintSolve<'_>,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, SimulationError> {
        let LinearTransientConstraintSolve {
            time,
            nodal_gmin,
            linear_system,
            constraints,
        } = solve;
        let size = circuit.matrix_size();
        let mut roots = constraints
            .iter()
            .map(|constraint| {
                Self::startup_constraint_clamp_orientation(circuit, matrix, constraint).1
            })
            .filter(|&node| node != 0)
            .collect::<Vec<_>>();
        roots.sort_unstable();
        roots.dedup();

        let solve_for_roots = |root_values: &[Value],
                               circuit: &mut CircuitData,
                               matrix: &mut StaticMatrix|
         -> Result<Vec<Value>, SimulationError> {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            matrix.clear_values();
            let mut rhs = vec![0.0; size];
            Self::stamp_linear_transient_operating_point_system(
                circuit,
                matrix,
                &mut rhs,
                time,
                nodal_gmin,
                linear_system,
            )?;
            let mut reference_solution = vec![0.0; size];
            for (&root, &value) in roots.iter().zip(root_values) {
                reference_solution[root - 1] = value;
            }
            Self::apply_node_voltage_constraints(
                circuit,
                matrix,
                &mut rhs,
                constraints,
                &reference_solution,
            )?;
            let values = matrix.solve(&rhs).map_err(SimulationError::Solver)?;
            if values.iter().all(|value| value.is_finite()) {
                Ok(values)
            } else {
                Err(SimulationError::Solver(
                    crate::solver::SolverError::SingularMatrix,
                ))
            }
        };

        let zero_roots = vec![0.0; roots.len()];
        let base = solve_for_roots(&zero_roots, circuit, matrix)?;
        let root_values = if roots.is_empty() {
            Vec::new()
        } else {
            let mut system = vec![vec![0.0; roots.len()]; roots.len()];
            let mut rhs = roots.iter().map(|&root| base[root - 1]).collect::<Vec<_>>();
            for (column, _) in roots.iter().enumerate() {
                let mut excitation = vec![0.0; roots.len()];
                excitation[column] = 1.0;
                let response = solve_for_roots(&excitation, circuit, matrix)?;
                for (row, &root) in roots.iter().enumerate() {
                    let transfer = response[root - 1] - rhs[row];
                    system[row][column] = if row == column { 1.0 } else { 0.0 } - transfer;
                }
            }
            Self::solve_startup_common_modes(&mut system, &mut rhs)?
        };
        let mut values = solve_for_roots(&root_values, circuit, matrix)?;
        Self::enforce_node_voltage_hints(circuit, matrix, &mut values, constraints);
        for constraint in constraints {
            let (clamped, _, _) =
                Self::startup_constraint_clamp_orientation(circuit, matrix, constraint);
            if !Self::node_voltage_constraint_pins_node(circuit, matrix, clamped) {
                // Preserve SPICE's established source-conflict contract: a
                // node row carrying an ideal branch current outvotes a
                // startup clamp rather than pretending both ideal equations
                // were satisfied.
                continue;
            }
            let positive = values.get(constraint.positive.wrapping_sub(1)).copied();
            let negative = if constraint.negative == 0 {
                Some(0.0)
            } else {
                values.get(constraint.negative - 1).copied()
            };
            let Some((positive, negative)) = positive.zip(negative) else {
                continue;
            };
            let residual = positive - negative - constraint.voltage;
            let tolerance = 256.0
                * Value::EPSILON
                * positive
                    .abs()
                    .max(negative.abs())
                    .max(constraint.voltage.abs())
                    .max(1.0);
            if residual.abs() > tolerance {
                return Err(SimulationError::Circuit(format!(
                    "differential startup constraint solve residual {residual:.17e} exceeds {tolerance:.17e}"
                )));
            }
        }
        Ok(values)
    }

    fn solve_startup_common_modes(
        matrix: &mut [Vec<Value>],
        rhs: &mut [Value],
    ) -> Result<Vec<Value>, SimulationError> {
        let order = rhs.len();
        for pivot in 0..order {
            let best = (pivot..order)
                .max_by(|&left, &right| {
                    matrix[left][pivot]
                        .abs()
                        .total_cmp(&matrix[right][pivot].abs())
                })
                .unwrap_or(pivot);
            let scale = matrix[best]
                .iter()
                .map(|value| value.abs())
                .fold(0.0, Value::max)
                .max(1.0);
            if matrix[best][pivot].abs() <= 256.0 * Value::EPSILON * scale {
                return Err(SimulationError::Circuit(
                    "differential startup constraint system is rank deficient; its common-mode voltage is not determined by the circuit"
                        .to_string(),
                ));
            }
            matrix.swap(pivot, best);
            rhs.swap(pivot, best);
            for row in pivot + 1..order {
                let factor = matrix[row][pivot] / matrix[pivot][pivot];
                matrix[row][pivot] = 0.0;
                // `row > pivot` here, so the pivot row stays in `above`.
                let (above, below) = matrix.split_at_mut(row);
                let pivot_row = &above[pivot];
                let target_row = &mut below[0];
                for (target, &value) in target_row[pivot + 1..order]
                    .iter_mut()
                    .zip(&pivot_row[pivot + 1..order])
                {
                    *target -= factor * value;
                }
                rhs[row] -= factor * rhs[pivot];
            }
        }
        let mut solution = vec![0.0; order];
        for row in (0..order).rev() {
            let remainder = matrix[row][row + 1..]
                .iter()
                .zip(&solution[row + 1..])
                .map(|(coefficient, value)| coefficient * value)
                .sum::<Value>();
            solution[row] = (rhs[row] - remainder) / matrix[row][row];
        }
        Ok(solution)
    }

    pub(in crate::engine) fn solve_nonlinear_transient_op_with_node_hints_and_abort(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        time: Value,
        hints: &StartupVoltageHints,
        node_constraints: &[StartupVoltageConstraint],
        abort: &dyn AbortSignal,
    ) -> Result<TransientOperatingPointSolution, SimulationError> {
        let node_hints = &hints.constraints;
        let size = circuit.matrix_size();
        let gmin_floor =
            if self.config.spice_dialect == SpiceDialect::Xyce && !node_hints.is_empty() {
                0.0
            } else {
                self.dc_nodal_gmin_floor(circuit)
            };
        let junction_gmin = self.effective_device_junction_gmin(gmin_floor);
        let mut use_transient_current_seed = false;
        let mut solution = match self.linear_presolve_for_guess_with_linear_stamp(
            circuit,
            matrix,
            |circuit, matrix, rhs| {
                Self::stamp_transient_operating_point_linear(
                    circuit, matrix, rhs, time, 0.0, false,
                );
            },
        ) {
            Some(solution) => solution,
            None if self.config.spice_dialect == SpiceDialect::Xyce
                && circuit.has_xyce_core_inductors() =>
            {
                // Xyce permits a zero-valued transient source in parallel with
                // an inductor-backed Core winding.  The ordinary DC-short
                // startup matrix is intentionally singular because that
                // branch current is not a DC observable.  Reuse the
                // simulator's canonical current-seeded transient startup
                // equations for the nonlinear operating-point walk so the
                // seed remains deterministic without weakening ordinary DC
                // topology validation.
                let seeded = self.linear_presolve_for_guess_with_linear_stamp(
                    circuit,
                    matrix,
                    |circuit, matrix, rhs| {
                        Self::stamp_transient_current_seed_linear(
                            circuit, matrix, rhs, time, 0.0, true,
                        );
                    },
                );
                if seeded.is_some() {
                    use_transient_current_seed = true;
                }
                seeded.unwrap_or_else(|| vec![0.0; size])
            }
            None => vec![0.0; size],
        };

        Self::seed_node_voltage_hints(circuit, &mut solution, node_hints);

        solution = Self::sanitize_initial_guess(&solution, size);
        if hints.has_nodesets || !node_hints.is_empty() {
            match Self::with_nodeset_phase(circuit, hints.has_nodesets, |circuit| {
                self.solve_nonlinear_transient_op_startup_with_guess_and_hints_abort(
                    circuit, matrix, time, &solution, node_hints, abort,
                )
            }) {
                Ok(nodeset_solution) => {
                    solution = nodeset_solution;
                }
                Err(err) if Self::is_recoverable_startup_error(&err) => {
                    log::debug!(
                        "NODESET-constrained transient operating-point startup did not converge: {err}; continuing from hinted seed."
                    );
                }
                Err(err) => return Err(err),
            }
        }
        self.prime_operating_point_seed(
            circuit,
            &solution,
            time,
            crate::xspice::AnalysisType::Transient,
        );

        let requires_conservative_nonlinear_limiting =
            circuit.requires_conservative_solution_damping();
        let mut rhs = vec![0.0; size];
        let mut raw_solution = Vec::with_capacity(size);
        let mut correction_rhs = Vec::new();
        let uses_vbic_correction = Self::requires_vbic_correction_form(circuit);
        let mut damping_state = NewtonDampingState::default();
        let junction_owns_steps = Self::junction_limiting_owns_newton_steps(circuit)
            || self.b3soi_limiter_owns_global_damping(circuit);
        // ngspice floors every NIiter call to 100 iterations (ITL1); the
        // per-iterate junction walk of pnjlim devices legitimately needs
        // tens of iterations on deep TTL chains before the residual settles.
        let tranop_max_iterations = if junction_owns_steps {
            self.continuation_iteration_budget(1, 100)
        } else {
            self.continuation_iteration_budget(1, 32)
        };

        for iteration in 0..tranop_max_iterations {
            if Self::should_abort_iteration(abort, iteration) {
                return Err(SimulationError::Aborted);
            }

            matrix.clear_values();
            rhs.fill(0.0);

            circuit.refresh_jiles_atherton_inductances(&solution);
            if use_transient_current_seed {
                Self::stamp_transient_current_seed_linear(
                    circuit, matrix, &mut rhs, time, gmin_floor, false,
                );
            } else {
                Self::stamp_transient_operating_point_linear(
                    circuit, matrix, &mut rhs, time, gmin_floor, false,
                );
            }
            self.try_stamp_operating_point_newton_system(
                circuit,
                matrix,
                &mut rhs,
                OperatingPointProbe {
                    solution: &solution,
                    time,
                    analysis: crate::xspice::AnalysisType::Transient,
                    junction_gmin,
                },
                false,
                &mut correction_rhs,
            )?;
            Self::apply_node_voltage_constraints_at(
                circuit,
                matrix,
                &mut rhs,
                node_constraints,
                &solution,
                uses_vbic_correction,
            )?;

            let solve_result = if uses_vbic_correction {
                Self::solve_direct_dc_correction(matrix, &rhs, None, &solution, &mut raw_solution)
            } else {
                matrix
                    .solve_into(&rhs, &mut raw_solution)
                    .map_err(SimulationError::Solver)
            };
            match solve_result {
                Ok(()) => {}
                Err(err)
                    if !use_transient_current_seed
                        && self.config.spice_dialect == SpiceDialect::Xyce
                        && circuit.has_xyce_core_inductors() =>
                {
                    // The regular transient operating-point equations retain
                    // ideal inductor shorts.  If that matrix is singular,
                    // switch the remainder of this startup walk to the
                    // canonical current-seeded transient equations; this is
                    // the same deterministic fallback used by the linear
                    // startup path and is only enabled for Xyce Core devices.
                    log::debug!(
                        "Xyce Core transient operating-point short solve failed ({err}); retrying with current-seeded startup equations"
                    );
                    use_transient_current_seed = true;
                    continue;
                }
                Err(err) => {
                    return Err(err);
                }
            }
            let mut damped_solution;
            let new_solution = if requires_conservative_nonlinear_limiting && !junction_owns_steps {
                damped_solution = self.apply_damping_strategy_for_circuit(
                    circuit.has_b3soi_devices(),
                    &circuit.non_electrical_state_mask(),
                    DampingStep {
                        old: &solution,
                        proposal: &raw_solution,
                        damping_state: &mut damping_state,
                    },
                    junction_owns_steps,
                    |trial| {
                        self.nonlinear_merit_with_linear_stamp_for_operating_point(
                            circuit,
                            matrix,
                            OperatingPointProbe {
                                solution: trial,
                                time,
                                analysis: crate::xspice::AnalysisType::Transient,
                                junction_gmin,
                            },
                            |circuit, matrix, rhs| {
                                circuit.refresh_jiles_atherton_inductances(trial);
                                if use_transient_current_seed {
                                    Self::stamp_transient_current_seed_linear(
                                        circuit, matrix, rhs, time, gmin_floor, false,
                                    );
                                } else {
                                    Self::stamp_transient_operating_point_linear(
                                        circuit, matrix, rhs, time, gmin_floor, false,
                                    );
                                }
                            },
                        )
                    },
                );
                &mut damped_solution
            } else {
                &mut raw_solution
            };
            circuit.enforce_ideal_voltage_constraints(new_solution, time)?;
            Self::reset_nonfinite_values(new_solution);
            Self::enforce_node_voltage_hints(circuit, matrix, new_solution, node_constraints);

            let voltage_converged =
                self.node_voltage_convergence_met(&solution, new_solution, circuit.num_nodes());
            self.update_device_states_for_operating_point(
                circuit,
                OperatingPointProbe {
                    solution: new_solution,
                    time,
                    analysis: crate::xspice::AnalysisType::Transient,
                    junction_gmin,
                },
            );
            let device_converged = circuit.nonlinear_converged(self.device_convergence_criteria());
            let nonlinear_residual_converged = if !voltage_converged || !device_converged {
                false
            } else if node_constraints.is_empty() {
                self.nonlinear_residual_converged_with_linear_stamp_for_operating_point(
                    circuit,
                    matrix,
                    OperatingPointProbe {
                        solution: new_solution,
                        time,
                        analysis: crate::xspice::AnalysisType::Transient,
                        junction_gmin,
                    },
                    |circuit, matrix, rhs| {
                        circuit.refresh_jiles_atherton_inductances(new_solution);
                        if use_transient_current_seed {
                            Self::stamp_transient_current_seed_linear(
                                circuit, matrix, rhs, time, gmin_floor, false,
                            );
                        } else {
                            Self::stamp_transient_operating_point_linear(
                                circuit, matrix, rhs, time, gmin_floor, false,
                            );
                        }
                    },
                )
            } else {
                self.constrained_transient_op_residual_converged(
                    circuit,
                    matrix,
                    new_solution,
                    time,
                    node_constraints,
                    TransientOpConductances {
                        nodal_gmin: gmin_floor,
                        junction_gmin,
                        use_transient_current_seed,
                    },
                )?
            };

            std::mem::swap(&mut solution, new_solution);
            if voltage_converged && device_converged && nonlinear_residual_converged {
                let linear_system = if use_transient_current_seed {
                    TransientOperatingPointLinearSystem::CurrentSeededInductors
                } else {
                    TransientOperatingPointLinearSystem::IdealInductorShorts
                };
                return Ok(TransientOperatingPointSolution {
                    values: solution,
                    accepted_contract: node_constraints.is_empty().then_some(
                        AcceptedTransientOperatingPointContract {
                            linear_system,
                            nodal_gmin: gmin_floor,
                            junction_gmin: Some(junction_gmin),
                        },
                    ),
                });
            }
        }

        Err(SimulationError::ConvergenceFailed(tranop_max_iterations))
    }
}

#[cfg(test)]
mod correction_constraint_tests {
    use super::*;

    #[test]
    fn steep_vbic_thermal_slope_converges_through_startup_and_corrector_paths() {
        use crate::engine::convergence::fallback::CorrectorSeedMode;
        use crate::netlist::Netlist;
        let netlist = Netlist::parse(
            "VBIC direct-residual startup\nVc c 0 0.6\nVb b 0 0.7\nVth th 0 20\nQ1 c b 0 th vm SW_ET=0 M=3\n\
             .model vm NPN(LEVEL=11 IS=1e-16 IBEI=1e-18 IBCI=1e-18 ISP=0 IBEIP=0 VEF=1e15 VER=3 TCVEF=-0.049999999999999989 TCVER=-0.02 RCX=1 RCI=1 RBX=1 RBI=1 RE=1 RBP=0 RS=0 GMIN=0 TNOM=27 RTH=1000 CTH=1p CJE=1p CJC=1p TF=1n TR=2n QTF=0.3 TD=1n)\n.temp 27\n.options gmin=0\n.end\n",
        ).unwrap();
        for route in [
            "dc startup",
            "transient startup",
            "transient op",
            "limited",
            "static",
            "gmin",
        ] {
            let engine = Engine::default().resolved_for_netlist(&netlist);
            let mut circuit = engine.build_circuit(&netlist).unwrap();
            let mut matrix = engine.build_matrix(&circuit).unwrap();
            circuit.link_indices(&matrix);
            let zero = vec![0.0; circuit.matrix_size()];
            let constraints = [StartupVoltageConstraint {
                positive: circuit.get_node_by_name("th").unwrap(),
                negative: 0,
                voltage: 20.0,
            }];
            let solution = match route {
                "dc startup" => engine.solve_nonlinear_dc_startup_with_constraints_and_abort(
                    &mut circuit,
                    &mut matrix,
                    &zero,
                    &constraints,
                    &crate::NoAbort,
                ),
                "transient startup" => engine
                    .solve_nonlinear_transient_op_startup_with_guess_and_hints_abort(
                        &mut circuit,
                        &mut matrix,
                        0.0,
                        &zero,
                        &constraints,
                        &crate::NoAbort,
                    ),
                "transient op" => engine
                    .solve_nonlinear_transient_op_with_node_hints_and_abort(
                        &mut circuit,
                        &mut matrix,
                        0.0,
                        &StartupVoltageHints {
                            constraints: Vec::new(),
                            has_nodesets: false,
                        },
                        &[],
                        &crate::NoAbort,
                    )
                    .map(|point| point.values),
                "gmin" => engine.gmin_stepping_nonlinear_with_abort(
                    &mut circuit,
                    &mut matrix,
                    &zero,
                    &crate::NoAbort,
                ),
                _ => engine
                    .solve_scaled_nonlinear_corrector_with_seed_mode(
                        &mut circuit,
                        &mut matrix,
                        1.0,
                        CorrectorRun {
                            initial_solution: &zero,
                            damping_state: &mut NewtonDampingState::default(),
                            max_iterations: 100,
                        },
                        &crate::NoAbort,
                        if route == "limited" {
                            CorrectorSeedMode::Limited
                        } else {
                            CorrectorSeedMode::StaticProbeEveryIteration
                        },
                    )
                    .map(|(solution, converged, _)| {
                        assert!(converged, "{route} must converge");
                        solution
                    }),
            }
            .unwrap_or_else(|error| panic!("{route}: {error}"));
            let [_, _, _, _, _, ci, _, bi, ei, ..] = circuit.bjts.devices[0].mna_coupling_nodes();
            // Independently solved with 80-digit arithmetic; these intrinsic
            // voltages also determine the series and terminal DC currents.
            for (node, expected) in [
                (ci, 0.599810987178337),
                (bi, 0.6999971642386433),
                (ei, 0.00009639710807916378),
            ] {
                assert!(
                    (solution[node - 1] - expected).abs() < 2e-12,
                    "{route} node {node}: {} != {expected}",
                    solution[node - 1]
                );
            }
        }
    }

    #[test]
    fn voltage_correction_clamps_retain_small_branch_currents() {
        let mut circuit = CircuitData::new();
        let a = circuit.get_or_create_node("a");
        let b = circuit.get_or_create_node("b");
        let mut matrix = StaticMatrix::from_triplets(
            2,
            4,
            &[
                (0, 0, 2.0),
                (0, 1, -2.0),
                (0, 2, 1.0),
                (0, 3, -1.0),
                (1, 0, -2.0),
                (1, 1, 2.0),
            ],
        )
        .unwrap();
        let anchor = [0.7, 0.3, 1e-10, 2e-10];
        let constraints = [
            StartupVoltageConstraint {
                positive: a,
                negative: 0,
                voltage: 0.7,
            },
            StartupVoltageConstraint {
                positive: b,
                negative: a,
                voltage: -0.2,
            },
        ];
        let mut rhs = [123.0, 456.0];
        Engine::apply_node_voltage_correction_constraints(
            &circuit,
            &mut matrix,
            &mut rhs,
            &constraints,
            &anchor,
        )
        .unwrap();
        assert_eq!(
            rhs[0], 1e-10,
            "held branch currents must survive the stiff clamp"
        );
        assert!((rhs[1] - 0.2).abs() < 1e-15);
        assert_eq!(
            matrix.row_product_from_column(0, 2, &anchor).unwrap(),
            -1e-10
        );
        assert_eq!(
            matrix
                .row_product_from_column(1, 0, &[1.0, 0.0, 0.0, 0.0])
                .unwrap(),
            0.0
        );
    }
}

#[cfg(all(test, feature = "veriloga"))]
mod nodeset_phase_tests {
    use super::*;
    use crate::NoAbort;
    use crate::device::veriloga::{Compiler, VerilogADevice};

    fn circuit(analysis: u8) -> (Engine, CircuitData, StaticMatrix, usize, usize) {
        let source = r#"module nodeset_probe(hint,out);
inout hint,out; electrical hint,out;
real starts;
analog initial starts=starts+1;
analog begin
  I(hint)<+V(hint);
  I(out)<+V(out)-(starts+analysis("nodeset")+0.1*analysis("static")
      +0.01*analysis("dc")+0.001*analysis("ic")+0.0001*analysis("ac")
      +0.00001*analysis("noise")+0.000001*analysis("tran"));
end
endmodule"#;
        circuit_from_source(source, analysis)
    }

    fn circuit_from_source(
        source: &str,
        analysis: u8,
    ) -> (Engine, CircuitData, StaticMatrix, usize, usize) {
        let compiler = Compiler::default();
        let model = compiler.compile(source).unwrap();
        let canonical = compiler.compile_canonical_ir(source).unwrap();
        let mut circuit = CircuitData::new();
        let hint = circuit.get_or_create_node("hint");
        let out = circuit.get_or_create_node("out");
        circuit.add_veriloga_device(
            VerilogADevice::try_new_with_canonical_ir("x1", model, &canonical, &[hint, out])
                .unwrap(),
        );
        circuit
            .begin_veriloga_analysis_in_phase(
                analysis,
                rspice_veriloga_runtime::AnalogAnalysisPhase::Equilibrium,
            )
            .unwrap();
        let engine = Engine::default();
        let matrix = engine.build_matrix(&circuit).unwrap();
        circuit.link_indices(&matrix);
        (engine, circuit, matrix, hint, out)
    }

    #[test]
    fn exhausted_equilibrium_iterations_do_not_accept_the_nodeset_solution() {
        let source = r#"module nodeset_no_equilibrium(hint,out);
inout hint,out; electrical hint,out;
analog begin
    I(hint)<+V(hint);
    if (analysis("nodeset")) I(out)<+V(out)-1;
    else I(out)<+(V(out)>=0 ? V(out)+1 : V(out)-1);
end
endmodule"#;
        let (engine, mut circuit, mut matrix, _, _) = circuit_from_source(source, 2);
        let result = engine.solve_nonlinear_transient_op_with_node_hints_and_abort(
            &mut circuit,
            &mut matrix,
            0.0,
            &StartupVoltageHints {
                constraints: Vec::new(),
                has_nodesets: true,
            },
            &[],
            &NoAbort,
        );
        // The final equation has unit slope in both branches, so Newton
        // exhausts its budget rather than failing a singular linear solve.
        assert!(matches!(result, Err(SimulationError::ConvergenceFailed(_))));
    }

    #[test]
    fn nodeset_interval_restores_physical_analysis_and_initializer_state() {
        for (analysis, expected) in [
            (0, 1.11),
            (1, 1.1001),
            (2, 1.101001),
            (3, 1.10001),
            (4, 1.101),
        ] {
            let (engine, mut circuit, mut matrix, hint, out) = circuit(analysis);
            let constraints = [StartupVoltageConstraint {
                positive: hint,
                negative: 0,
                voltage: 0.25,
            }];
            let seed = vec![0.0; circuit.matrix_size()];
            let constrained = Engine::with_nodeset_phase(&mut circuit, true, |circuit| {
                if analysis == 2 {
                    engine.solve_nonlinear_transient_op_startup_with_guess_and_hints_abort(
                        circuit,
                        &mut matrix,
                        0.0,
                        &seed,
                        &constraints,
                        &NoAbort,
                    )
                } else {
                    engine.solve_nonlinear_dc_startup_with_constraints_and_abort(
                        circuit,
                        &mut matrix,
                        &seed,
                        &constraints,
                        &NoAbort,
                    )
                }
            })
            .unwrap();
            assert!(
                (constrained[out - 1] - (expected + 1.0)).abs() < 1e-8,
                "analysis={analysis}: {constrained:?}"
            );
            let final_point = engine
                .solve_nonlinear_dc_startup_with_constraints_and_abort(
                    &mut circuit,
                    &mut matrix,
                    &constrained,
                    &[],
                    &NoAbort,
                )
                .unwrap();
            assert!(
                (final_point[out - 1] - expected).abs() < 1e-8,
                "analysis={analysis}: {final_point:?}"
            );
        }
    }

    #[test]
    fn rejected_nodeset_interval_restores_phase() {
        let (engine, mut circuit, mut matrix, _, out) = circuit(2);
        let error = Engine::with_nodeset_phase(&mut circuit, true, |_| {
            Err::<(), _>(SimulationError::Aborted)
        })
        .unwrap_err();
        assert!(matches!(error, SimulationError::Aborted));
        let seed = vec![0.0; circuit.matrix_size()];
        let solution = engine
            .solve_nonlinear_transient_op_startup_with_guess_and_hints_abort(
                &mut circuit,
                &mut matrix,
                0.0,
                &seed,
                &[],
                &NoAbort,
            )
            .unwrap();
        assert!((solution[out - 1] - 1.101001).abs() < 1e-8, "{solution:?}");
    }
}
