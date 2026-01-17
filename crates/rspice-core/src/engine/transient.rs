//! Transient Time-Domain Analysis
//!
//! This module provides time-domain simulation using:
//! - Adaptive timestep control with LTE estimation
//! - TrapGear method switching for stability
//! - Optional waveform compression for long simulations

use super::{Engine, SimulationError, TransientResult};
use crate::analysis::transient::{
    BreakpointManager, CompanionCoefficients, IntegrationMethod, LteEstimator, TimestepController,
    TrapGearController,
};
use crate::analysis::waveform::{CompressionConfig, TransientResultCompressed, WaveformRecorder};
use crate::{Netlist, Value};

impl Engine {
    /// Run transient time-domain analysis
    ///
    /// Uses adaptive integration with automatic method switching (TrapGear).
    /// Trapezoidal integration is used normally for efficiency, but switches
    /// to Gear2/BDF2 when oscillations are detected for stability.
    pub fn run_tran(
        &self,
        netlist: &Netlist,
        tstop: Value,
        max_step: Value,
    ) -> Result<TransientResult, SimulationError> {
        let mut circuit = self.build_circuit(netlist)?;
        let mut matrix = self.build_matrix(&circuit)?;
        circuit.link_indices(&matrix);

        // Get DC operating point as initial condition
        let dc_solution = if circuit.has_nonlinear_devices() {
            self.solve_nonlinear(&mut circuit, &mut matrix)?
        } else {
            self.solve_linear(&circuit, &mut matrix)?
        };

        let num_nodes = circuit.num_nodes();
        let size = circuit.matrix_size();

        // Initialize timestep controller
        let initial_step = (max_step / 10.0).min(tstop / 100.0);
        let mut timestep =
            TimestepController::new(initial_step, self.config.min_timestep, max_step);
        let mut breakpoints = BreakpointManager::new();
        let mut lte_estimator = LteEstimator::new(self.config.tolerance);

        // Initialize TrapGear controller for automatic method switching
        let mut trapgear = TrapGearController::new();

        // Track integration method order for LTE scaling
        let method_order = |method: IntegrationMethod| -> u32 {
            match method {
                IntegrationMethod::BackwardEuler => 1,
                _ => 2, // Trapezoidal and Gear2 are both order 2
            }
        };

        // Initialize result storage
        let mut result = TransientResult {
            time: vec![0.0],
            voltages: (0..num_nodes)
                .map(|i| vec![dc_solution.get(i).copied().unwrap_or(0.0)])
                .collect(),
            num_nodes,
        };

        let mut solution = dc_solution;
        let mut t = 0.0;

        // Initialize capacitor voltage history from DC solution
        for (cap_idx, cap) in circuit.capacitors.stamps.iter().enumerate() {
            let np = cap.pp.row;
            let nn = cap.nn.row;
            let v_dc = if np == 0 { 0.0 } else { solution[np - 1] }
                - if nn == 0 { 0.0 } else { solution[nn - 1] };
            circuit.capacitors.v_prev[cap_idx] = v_dc;
            circuit.capacitors.v_prev_prev[cap_idx] = v_dc;
        }

        // Initialize inductor current and voltage history from DC solution
        for l_idx in 0..circuit.inductors.names.len() {
            let np = circuit.inductors.node_pos[l_idx];
            let nn = circuit.inductors.node_neg[l_idx];
            let br = circuit.inductors.branch_indices[l_idx];

            // Initialize voltage across inductor from DC solution
            let v_dc = if np == 0 { 0.0 } else { solution[np - 1] }
                - if nn == 0 { 0.0 } else { solution[nn - 1] };
            circuit.inductors.v_prev[l_idx] = v_dc;

            // Initialize branch currents from DC solution
            if br > 0 {
                let br_idx = circuit.num_nodes() + br - 1;
                let i_dc = solution[br_idx];
                circuit.inductors.i_prev[l_idx] = i_dc;
                circuit.inductors.i_prev_prev[l_idx] = i_dc;
            }
        }

        // Main transient loop
        while t < tstop {
            let (dt, _at_breakpoint) = breakpoints.limit_step(t, timestep.dt());
            let dt = dt.min(tstop - t); // Don't overshoot tstop

            // Prepare for Newton iteration at this timestep
            let mut new_solution = solution.clone();
            let mut rhs = vec![0.0; size];

            // Newton-Raphson iteration for this timestep
            let mut converged = false;
            for _iter in 0..self.config.max_iterations {
                matrix.clear_values();
                rhs.fill(0.0);

                // Add GMIN diagonal
                for i in 0..size {
                    matrix.add(i, i, 1e-12);
                }

                // Stamp linear devices (R, V, I)
                circuit.stamp_dc_direct(&mut matrix, &mut rhs);

                // Get current integration method from TrapGear controller
                let current_method = trapgear.current_method();
                let coeff = CompanionCoefficients::for_method(current_method);

                // Stamp capacitor companion models for transient
                for (cap_idx, cap) in circuit.capacitors.stamps.iter().enumerate() {
                    let capacitance = circuit.capacitors.capacitances[cap_idx];
                    let np = cap.pp.row;
                    let nn = cap.nn.row;

                    let v_n = circuit.capacitors.v_prev[cap_idx];
                    let v_n_minus_1 = circuit.capacitors.v_prev_prev[cap_idx];

                    let geq = coeff.capacitor_geq(capacitance, dt);
                    let ieq = coeff.capacitor_ieq(capacitance, dt, v_n, v_n_minus_1);

                    if np > 0 {
                        matrix.add(np - 1, np - 1, geq);
                        if nn > 0 {
                            matrix.add(np - 1, nn - 1, -geq);
                        }
                    }
                    if nn > 0 {
                        if np > 0 {
                            matrix.add(nn - 1, np - 1, -geq);
                        }
                        matrix.add(nn - 1, nn - 1, geq);
                    }

                    if np > 0 {
                        rhs[np - 1] += ieq;
                    }
                    if nn > 0 {
                        rhs[nn - 1] -= ieq;
                    }
                }

                // Stamp inductor companion models for transient
                for l_idx in 0..circuit.inductors.names.len() {
                    let np = circuit.inductors.node_pos[l_idx];
                    let nn = circuit.inductors.node_neg[l_idx];
                    let br = circuit.inductors.branch_indices[l_idx];
                    let inductance = circuit.inductors.inductances[l_idx];

                    let i_n = circuit.inductors.i_prev[l_idx];
                    let i_n_minus_1 = circuit.inductors.i_prev_prev[l_idx];
                    let v_n = circuit.inductors.v_prev[l_idx];

                    let req = coeff.inductor_req(inductance, dt);
                    let veq = coeff.inductor_veq(inductance, dt, i_n, i_n_minus_1, v_n);

                    if np > 0 && br > 0 {
                        let br_idx = circuit.num_nodes() + br - 1;
                        matrix.add(br_idx, np - 1, 1.0);
                        matrix.add(np - 1, br_idx, 1.0);
                    }
                    if nn > 0 && br > 0 {
                        let br_idx = circuit.num_nodes() + br - 1;
                        matrix.add(br_idx, nn - 1, -1.0);
                        matrix.add(nn - 1, br_idx, -1.0);
                    }
                    if br > 0 {
                        let br_idx = circuit.num_nodes() + br - 1;
                        matrix.add(br_idx, br_idx, -req);
                        rhs[br_idx] = veq;
                    }
                }

                // Stamp nonlinear devices if present
                if circuit.has_nonlinear_devices() {
                    circuit.update_nonlinear(&new_solution);
                    circuit.stamp_nonlinear(&mut matrix, &mut rhs, &new_solution);
                }

                // Solve and check convergence
                match matrix.solve(&rhs) {
                    Ok(sol) => {
                        let voltage_converged = Self::check_voltage_convergence(
                            &new_solution,
                            &sol,
                            self.config.tolerance,
                        );
                        let device_converged = !circuit.has_nonlinear_devices()
                            || circuit.nonlinear_converged(self.config.tolerance);

                        new_solution = sol;

                        if voltage_converged && device_converged {
                            converged = true;
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }

            if !converged {
                timestep.adjust(1.0);
                continue;
            }

            // Check LTE for physics accuracy
            let (lte, accept) = lte_estimator.estimate(&new_solution, dt);
            if !accept {
                let scale = lte_estimator.recommend_scale(lte);
                timestep.adjust(lte / scale);
                continue;
            }

            // Accept this timestep
            t += dt;
            lte_estimator.record(&new_solution, dt);
            lte_estimator.set_method_order(method_order(trapgear.current_method()));
            trapgear.update(&new_solution, dt);

            // Update capacitor voltage history
            for (cap_idx, cap) in circuit.capacitors.stamps.iter().enumerate() {
                let np = cap.pp.row;
                let nn = cap.nn.row;
                let v_new = if np == 0 { 0.0 } else { new_solution[np - 1] }
                    - if nn == 0 { 0.0 } else { new_solution[nn - 1] };
                circuit.capacitors.v_prev_prev[cap_idx] = circuit.capacitors.v_prev[cap_idx];
                circuit.capacitors.v_prev[cap_idx] = v_new;
            }

            // Update inductor current history
            for l_idx in 0..circuit.inductors.names.len() {
                let br = circuit.inductors.branch_indices[l_idx];
                if br > 0 {
                    let br_idx = circuit.num_nodes() + br - 1;
                    let i_new = new_solution[br_idx];
                    circuit.inductors.i_prev_prev[l_idx] = circuit.inductors.i_prev[l_idx];
                    circuit.inductors.i_prev[l_idx] = i_new;

                    let np = circuit.inductors.node_pos[l_idx];
                    let nn = circuit.inductors.node_neg[l_idx];
                    let v_new = if np == 0 { 0.0 } else { new_solution[np - 1] }
                        - if nn == 0 { 0.0 } else { new_solution[nn - 1] };
                    circuit.inductors.v_prev[l_idx] = v_new;
                }
            }

            solution = new_solution;

            // Store results
            result.time.push(t);
            for (i, voltages) in result.voltages.iter_mut().enumerate() {
                voltages.push(solution.get(i).copied().unwrap_or(0.0));
            }

            let scale = lte_estimator.recommend_scale(lte);
            timestep.adjust(lte / scale);
        }

        Ok(result)
    }

    /// Run transient analysis with waveform compression
    ///
    /// Uses the `WaveformRecorder` to achieve 10-100x memory reduction for long
    /// simulations. The compression uses linear interpolation-based point decimation
    /// that preserves all significant signal transitions.
    pub fn run_tran_compressed(
        &self,
        netlist: &Netlist,
        tstop: Value,
        max_step: Value,
        compression: CompressionConfig,
    ) -> Result<TransientResultCompressed, SimulationError> {
        let mut circuit = self.build_circuit(netlist)?;
        let mut matrix = self.build_matrix(&circuit)?;
        circuit.link_indices(&matrix);

        let dc_solution = if circuit.has_nonlinear_devices() {
            self.solve_nonlinear(&mut circuit, &mut matrix)?
        } else {
            self.solve_linear(&circuit, &mut matrix)?
        };

        let num_nodes = circuit.num_nodes();
        let size = circuit.matrix_size();

        let initial_step = (max_step / 10.0).min(tstop / 100.0);
        let mut timestep =
            TimestepController::new(initial_step, self.config.min_timestep, max_step);
        let mut breakpoints = BreakpointManager::new();
        let mut lte_estimator = LteEstimator::new(self.config.tolerance);

        let initial_values: Vec<Value> = (0..num_nodes)
            .map(|i| dc_solution.get(i).copied().unwrap_or(0.0))
            .collect();
        let mut recorder = WaveformRecorder::new(num_nodes, 0.0, &initial_values, compression);

        let mut solution = dc_solution;
        let mut t = 0.0;

        // Initialize capacitor and inductor history
        for (cap_idx, cap) in circuit.capacitors.stamps.iter().enumerate() {
            let np = cap.pp.row;
            let nn = cap.nn.row;
            let v_dc = if np == 0 { 0.0 } else { solution[np - 1] }
                - if nn == 0 { 0.0 } else { solution[nn - 1] };
            circuit.capacitors.v_prev[cap_idx] = v_dc;
            circuit.capacitors.v_prev_prev[cap_idx] = v_dc;
        }

        for l_idx in 0..circuit.inductors.names.len() {
            let np = circuit.inductors.node_pos[l_idx];
            let nn = circuit.inductors.node_neg[l_idx];
            let br = circuit.inductors.branch_indices[l_idx];

            let v_dc = if np == 0 { 0.0 } else { solution[np - 1] }
                - if nn == 0 { 0.0 } else { solution[nn - 1] };
            circuit.inductors.v_prev[l_idx] = v_dc;

            if br > 0 {
                let br_idx = circuit.num_nodes() + br - 1;
                let i_dc = solution[br_idx];
                circuit.inductors.i_prev[l_idx] = i_dc;
                circuit.inductors.i_prev_prev[l_idx] = i_dc;
            }
        }

        // Main transient loop (simplified companion model - trapezoidal only)
        while t < tstop {
            let (dt, _at_breakpoint) = breakpoints.limit_step(t, timestep.dt());
            let dt = dt.min(tstop - t);

            let mut new_solution = solution.clone();
            let mut rhs = vec![0.0; size];

            let mut converged = false;
            for _iter in 0..self.config.max_iterations {
                matrix.clear_values();
                rhs.fill(0.0);

                for i in 0..size {
                    matrix.add(i, i, 1e-12);
                }

                circuit.stamp_dc_direct(&mut matrix, &mut rhs);

                // Trapezoidal capacitor companion
                for (cap_idx, cap) in circuit.capacitors.stamps.iter().enumerate() {
                    let capacitance = circuit.capacitors.capacitances[cap_idx];
                    let geq = 2.0 * capacitance / dt;
                    let np = cap.pp.row;
                    let nn = cap.nn.row;
                    let v_prev = circuit.capacitors.v_prev[cap_idx];

                    if np > 0 {
                        matrix.add(np - 1, np - 1, geq);
                        if nn > 0 {
                            matrix.add(np - 1, nn - 1, -geq);
                        }
                    }
                    if nn > 0 {
                        if np > 0 {
                            matrix.add(nn - 1, np - 1, -geq);
                        }
                        matrix.add(nn - 1, nn - 1, geq);
                    }

                    let ieq = geq * v_prev;
                    if np > 0 {
                        rhs[np - 1] += ieq;
                    }
                    if nn > 0 {
                        rhs[nn - 1] -= ieq;
                    }
                }

                // Trapezoidal inductor companion
                for l_idx in 0..circuit.inductors.names.len() {
                    let np = circuit.inductors.node_pos[l_idx];
                    let nn = circuit.inductors.node_neg[l_idx];
                    let br = circuit.inductors.branch_indices[l_idx];
                    let inductance = circuit.inductors.inductances[l_idx];
                    let req = 2.0 * inductance / dt;
                    let i_prev = circuit.inductors.i_prev[l_idx];
                    let v_prev = circuit.inductors.v_prev[l_idx];
                    let veq = req * i_prev + v_prev;

                    if np > 0 && br > 0 {
                        let br_idx = circuit.num_nodes() + br - 1;
                        matrix.add(br_idx, np - 1, 1.0);
                        matrix.add(np - 1, br_idx, 1.0);
                    }
                    if nn > 0 && br > 0 {
                        let br_idx = circuit.num_nodes() + br - 1;
                        matrix.add(br_idx, nn - 1, -1.0);
                        matrix.add(nn - 1, br_idx, -1.0);
                    }
                    if br > 0 {
                        let br_idx = circuit.num_nodes() + br - 1;
                        matrix.add(br_idx, br_idx, -req);
                        rhs[br_idx] = veq;
                    }
                }

                if circuit.has_nonlinear_devices() {
                    circuit.update_nonlinear(&new_solution);
                    circuit.stamp_nonlinear(&mut matrix, &mut rhs, &new_solution);
                }

                match matrix.solve(&rhs) {
                    Ok(sol) => {
                        let voltage_converged = Self::check_voltage_convergence(
                            &new_solution,
                            &sol,
                            self.config.tolerance,
                        );
                        let device_converged = !circuit.has_nonlinear_devices()
                            || circuit.nonlinear_converged(self.config.tolerance);

                        new_solution = sol;

                        if voltage_converged && device_converged {
                            converged = true;
                            break;
                        }
                    }
                    Err(_) => break,
                }
            }

            if !converged {
                timestep.adjust(1.0);
                continue;
            }

            let (lte, accept) = lte_estimator.estimate(&new_solution, dt);
            if !accept {
                let scale = lte_estimator.recommend_scale(lte);
                timestep.adjust(lte / scale);
                continue;
            }

            t += dt;
            lte_estimator.record(&new_solution, dt);

            // Update history
            for (cap_idx, cap) in circuit.capacitors.stamps.iter().enumerate() {
                let np = cap.pp.row;
                let nn = cap.nn.row;
                let v_new = if np == 0 { 0.0 } else { new_solution[np - 1] }
                    - if nn == 0 { 0.0 } else { new_solution[nn - 1] };
                circuit.capacitors.v_prev_prev[cap_idx] = circuit.capacitors.v_prev[cap_idx];
                circuit.capacitors.v_prev[cap_idx] = v_new;
            }

            for l_idx in 0..circuit.inductors.names.len() {
                let br = circuit.inductors.branch_indices[l_idx];
                if br > 0 {
                    let br_idx = circuit.num_nodes() + br - 1;
                    let i_new = new_solution[br_idx];
                    circuit.inductors.i_prev_prev[l_idx] = circuit.inductors.i_prev[l_idx];
                    circuit.inductors.i_prev[l_idx] = i_new;
                }
            }

            solution = new_solution;

            let values: Vec<Value> = (0..num_nodes)
                .map(|i| solution.get(i).copied().unwrap_or(0.0))
                .collect();
            recorder.record(t, &values);

            let scale = lte_estimator.recommend_scale(lte);
            timestep.adjust(lte / scale);
        }

        let final_values: Vec<Value> = (0..num_nodes)
            .map(|i| solution.get(i).copied().unwrap_or(0.0))
            .collect();
        recorder.finalize(tstop, &final_values);

        Ok(recorder.to_transient_result())
    }
}
