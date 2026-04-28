//! Convergence helpers for Newton-Raphson iteration
//!
//! This module provides:
//! - GMIN stepping for difficult circuits
//! - Source stepping for convergence
//! - Linear and nonlinear solver interfaces

#![allow(clippy::too_many_arguments)]
#![cfg_attr(test, allow(clippy::field_reassign_with_default))]
use super::{DampingStrategy, Engine, SimulationError};
use crate::abort_signal::{AbortSignal, NoAbort};
use crate::device::NonlinearConvergenceCriteria;
use crate::solver::{
    ArcLengthConfig, ArcLengthContinuation, PseudoTransient, SolverError, StaticMatrix,
};
use crate::{CircuitData, Value};

#[derive(Debug, Clone, Copy)]
struct NewtonDampingState {
    bank_rose_alpha: Value,
    prev_step_norm: Option<Value>,
}

impl Default for NewtonDampingState {
    fn default() -> Self {
        Self {
            bank_rose_alpha: 1.0,
            prev_step_norm: None,
        }
    }
}

impl Engine {
    const MAX_NODE_VOLTAGE: Value = 1000.0;
    const MAX_DELTA_VOLTAGE_LIMIT: Value = 0.5;
    const BANK_ROSE_ALPHA_MIN: Value = 0.1;
    const BANK_ROSE_ALPHA_MAX: Value = 1.0;
    const ARMIJO_C1: Value = 1e-4;
    const LINE_SEARCH_BACKTRACK: Value = 0.5;
    const LINE_SEARCH_MAX_ITERS: usize = 8;
    const ARC_LENGTH_MAX_STEPS: usize = 128;
    const ABORT_POLL_MASK: usize = 0x7;

    #[inline]
    fn should_abort_iteration(abort: &dyn AbortSignal, iteration: usize) -> bool {
        (iteration & Self::ABORT_POLL_MASK) == 0 && abort.is_aborted()
    }

    #[inline]
    fn nonlinear_iteration_budget(&self, multiplier: usize) -> usize {
        self.config.max_iterations.saturating_mul(multiplier).max(1)
    }

    #[inline]
    fn continuation_iteration_budget(&self, multiplier: usize, minimum: usize) -> usize {
        self.nonlinear_iteration_budget(multiplier).max(minimum)
    }

    #[inline]
    fn sanitize_positive_tolerance(value: Value, fallback: Value) -> Value {
        if value.is_finite() && value > 0.0 {
            value
        } else {
            fallback
        }
    }

    #[inline]
    pub(crate) fn voltage_reltol(&self) -> Value {
        Self::sanitize_positive_tolerance(self.config.convergence_config.voltage_reltol, 1e-3)
    }

    #[inline]
    pub(crate) fn voltage_abstol(&self) -> Value {
        let configured = self.config.convergence_config.voltage_abstol;
        if configured.is_finite() && configured > 0.0 {
            configured
        } else {
            Self::sanitize_positive_tolerance(self.config.tolerance, 1e-6)
        }
    }

    #[inline]
    pub(crate) fn current_abstol(&self) -> Value {
        Self::sanitize_positive_tolerance(self.config.convergence_config.current_abstol, 1e-12)
    }

    #[inline]
    pub(crate) fn charge_abstol(&self) -> Value {
        Self::sanitize_positive_tolerance(
            self.config.convergence_config.charge_abstol,
            crate::constants::CHGTOL,
        )
    }

    #[inline]
    pub(crate) fn residual_reltol(&self) -> Value {
        let configured = self.config.convergence_config.residual_reltol;
        if configured.is_finite() && configured > 0.0 {
            configured
        } else {
            self.voltage_reltol()
        }
    }

    #[inline]
    pub(crate) fn device_convergence_criteria(&self) -> NonlinearConvergenceCriteria {
        NonlinearConvergenceCriteria::new(
            self.voltage_abstol(),
            self.current_abstol(),
            self.voltage_reltol(),
        )
    }

    #[inline]
    fn stamp_nonlinear_devices_for_dc(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        solution: &[Value],
    ) {
        self.stamp_nonlinear_devices_for_operating_point(
            circuit,
            matrix,
            rhs,
            solution,
            0.0,
            crate::xspice::AnalysisType::DcOp,
        );
    }

    #[inline]
    fn stamp_nonlinear_devices_for_operating_point(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        solution: &[Value],
        time: Value,
        analysis: crate::xspice::AnalysisType,
    ) {
        circuit.update_nonlinear(solution);
        circuit.stamp_nonlinear(matrix, rhs, solution);
        circuit.stamp_behavioral(matrix, rhs, solution, time);
        if circuit.has_xspice_devices() {
            circuit.evaluate_xspice_with_analysis(time, 0.0, solution, analysis);
            circuit.stamp_xspice(matrix, rhs);
        }
    }

    #[inline]
    fn update_device_states_for_dc(&self, circuit: &mut CircuitData, solution: &[Value]) {
        self.update_device_states_for_operating_point(
            circuit,
            solution,
            0.0,
            crate::xspice::AnalysisType::DcOp,
        );
    }

    #[inline]
    fn update_device_states_for_operating_point(
        &self,
        circuit: &mut CircuitData,
        solution: &[Value],
        time: Value,
        analysis: crate::xspice::AnalysisType,
    ) {
        circuit.update_nonlinear(solution);
        if circuit.has_xspice_devices() {
            circuit.evaluate_xspice_with_analysis(time, 0.0, solution, analysis);
        }
    }

    #[inline]
    fn prime_operating_point_seed(
        &self,
        circuit: &mut CircuitData,
        solution: &[Value],
        time: Value,
        analysis: crate::xspice::AnalysisType,
    ) {
        self.update_device_states_for_operating_point(circuit, solution, time, analysis);
    }

    #[inline]
    pub(crate) fn voltage_convergence_met(&self, old: &[Value], new: &[Value]) -> bool {
        Self::check_voltage_convergence_with_tolerances(
            old,
            new,
            self.voltage_abstol(),
            self.voltage_reltol(),
        )
    }

    #[inline]
    pub(crate) fn residual_convergence_met(
        &self,
        matrix: &StaticMatrix,
        solution: &[Value],
        rhs: &[Value],
    ) -> bool {
        match matrix.scaled_residual_inf_norm(
            solution,
            rhs,
            self.current_abstol(),
            self.residual_reltol(),
        ) {
            Ok(norm) => norm.is_finite() && norm <= 1.0,
            Err(_) => false,
        }
    }

    fn nonlinear_residual_converged_with_linear_stamp<F>(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
        mut linear_stamp: F,
    ) -> bool
    where
        F: FnMut(&mut CircuitData, &mut StaticMatrix, &mut [Value]),
    {
        let size = circuit.matrix_size();
        if solution.len() != size || solution.iter().any(|v| !v.is_finite()) {
            return false;
        }

        let mut rhs = vec![0.0; size];
        matrix.clear_values();
        linear_stamp(circuit, matrix, &mut rhs);
        self.stamp_nonlinear_devices_for_dc(circuit, matrix, &mut rhs, solution);
        self.residual_convergence_met(matrix, solution, &rhs)
    }

    fn nonlinear_residual_converged(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
    ) -> bool {
        let gmin_floor = self.config.convergence_config.gmin_target.max(0.0);
        self.nonlinear_residual_converged_with_linear_stamp(
            circuit,
            matrix,
            solution,
            |circuit, matrix, rhs| {
                let node_count = circuit.num_nodes().min(rhs.len());
                for i in 0..node_count {
                    matrix.add(i, i, gmin_floor);
                }
                circuit.stamp_dc_direct(matrix, rhs);
            },
        )
    }

    fn nonlinear_residual_converged_scaled(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
        source_scale: Value,
    ) -> bool {
        let gmin_floor = self.config.convergence_config.gmin_target.max(0.0);
        self.nonlinear_residual_converged_with_linear_stamp(
            circuit,
            matrix,
            solution,
            |circuit, matrix, rhs| {
                let node_count = circuit.num_nodes().min(rhs.len());
                for i in 0..node_count {
                    matrix.add(i, i, gmin_floor);
                }
                circuit.stamp_dc_direct_scaled(matrix, rhs, source_scale);
            },
        )
    }

    fn nonlinear_residual_converged_with_gmin(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
        gmin: Value,
    ) -> bool {
        self.nonlinear_residual_converged_with_linear_stamp(
            circuit,
            matrix,
            solution,
            |circuit, matrix, rhs| {
                let node_count = circuit.num_nodes().min(rhs.len());
                for i in 0..node_count {
                    matrix.add(i, i, gmin);
                }
                circuit.stamp_dc_direct(matrix, rhs);
            },
        )
    }

    fn nonlinear_residual_converged_with_pseudo_transient(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
        anchor_solution: &[Value],
        pseudo_conductance: Value,
    ) -> bool {
        self.nonlinear_residual_converged_with_linear_stamp(
            circuit,
            matrix,
            solution,
            |circuit, matrix, rhs| {
                for i in 0..rhs.len() {
                    matrix.add(i, i, 1e-12 + pseudo_conductance);
                    rhs[i] += pseudo_conductance * anchor_solution[i];
                }
                circuit.stamp_dc_direct(matrix, rhs);
            },
        )
    }

    fn build_descending_schedule(mut start: Value, mut end: Value) -> Vec<Value> {
        if !start.is_finite() || start <= 0.0 {
            start = 1e-3;
        }
        if !end.is_finite() || end <= 0.0 {
            end = 1e-12;
        }
        if start < end {
            std::mem::swap(&mut start, &mut end);
        }

        let mut values = Vec::with_capacity(16);
        let mut current = start;
        values.push(current);

        while current > end && values.len() < 64 {
            let next = (current / 10.0).max(end);
            if (next - current).abs() < Value::EPSILON {
                break;
            }
            values.push(next);
            current = next;
        }

        if values.last().is_none_or(|&v| v > end) {
            values.push(end);
        }

        values
    }

    #[inline]
    fn gmin_linear_schedule(&self) -> Vec<Value> {
        let conv = &self.config.convergence_config;
        let start = conv.gmin_initial.max(1e-2);
        let end = conv.gmin_target.max(1e-12);
        Self::build_descending_schedule(start, end)
    }

    #[inline]
    fn gmin_nonlinear_schedule(&self) -> Vec<Value> {
        let conv = &self.config.convergence_config;
        let start = conv.gmin_initial.max(1e-3);
        let end = conv.gmin_target.max(1e-12);
        Self::build_descending_schedule(start, end)
    }

    #[inline]
    fn has_clamped_values(solution: &[Value]) -> bool {
        solution.iter().any(|&v| !v.is_finite() || v.abs() >= 999.0)
    }

    #[inline]
    fn has_suspicious_uniformity(solution: &[Value]) -> bool {
        if solution.len() <= 4 {
            return false;
        }

        let mut counts: std::collections::HashMap<i32, usize> = std::collections::HashMap::new();
        let mut min_v = Value::INFINITY;
        let mut max_v = Value::NEG_INFINITY;

        for &v in solution {
            if !v.is_finite() {
                return false;
            }
            min_v = min_v.min(v);
            max_v = max_v.max(v);
            let bucket = (v * 1000.0).round() as i32; // 1 mV quantization
            *counts.entry(bucket).or_insert(0) += 1;
        }

        let span = max_v - min_v;
        if span <= 1e-9 {
            return true;
        }

        let dominant = counts.values().copied().max().unwrap_or(0);
        let dominant_ratio = dominant as Value / solution.len() as Value;

        // Only flag near-constant stuck vectors; avoid false positives on
        // legitimate rail-distributed logic operating points.
        dominant_ratio >= 0.8 && counts.len() <= 3 && span <= 5e-2
    }

    #[inline]
    fn is_suspicious_solution(solution: &[Value]) -> bool {
        Self::has_clamped_values(solution) || Self::has_suspicious_uniformity(solution)
    }

    #[inline]
    fn step_l2_norm(old: &[Value], new: &[Value]) -> Value {
        old.iter()
            .zip(new.iter())
            .map(|(&a, &b)| {
                let d = b - a;
                d * d
            })
            .sum::<Value>()
            .sqrt()
    }

    fn interpolate_solution(old: &[Value], proposal: &[Value], alpha: Value) -> Vec<Value> {
        old.iter()
            .zip(proposal.iter())
            .map(|(&old_v, &new_v)| old_v + alpha * (new_v - old_v))
            .collect()
    }

    fn limit_step_delta(old: &[Value], proposal: &[Value], max_delta: Value) -> Vec<Value> {
        old.iter()
            .zip(proposal.iter())
            .map(|(&old_v, &new_v)| {
                let delta = new_v - old_v;
                if delta.abs() > max_delta {
                    old_v + delta.signum() * max_delta
                } else {
                    new_v
                }
            })
            .collect()
    }

    fn update_bank_rose_alpha(damping_state: &mut NewtonDampingState, step_norm: Value) {
        let Some(prev_norm) = damping_state.prev_step_norm else {
            damping_state.prev_step_norm = Some(step_norm.max(1e-30));
            damping_state.bank_rose_alpha = 1.0;
            return;
        };

        let ratio = if prev_norm > 0.0 {
            step_norm / prev_norm
        } else {
            1.0
        };

        if ratio > 1.0 {
            damping_state.bank_rose_alpha *= 0.5;
        } else if ratio > 0.9 {
            damping_state.bank_rose_alpha *= 0.9;
        } else if ratio < 0.5 {
            damping_state.bank_rose_alpha *= 1.2;
        }

        damping_state.bank_rose_alpha = damping_state
            .bank_rose_alpha
            .clamp(Self::BANK_ROSE_ALPHA_MIN, Self::BANK_ROSE_ALPHA_MAX);
        damping_state.prev_step_norm = Some(step_norm.max(1e-30));
    }

    fn line_search_step<F>(old: &[Value], proposal: &[Value], merit: &mut F) -> Vec<Value>
    where
        F: FnMut(&[Value]) -> Option<Value>,
    {
        let base_merit = merit(old).unwrap_or(Value::INFINITY);
        let mut best_solution = proposal.to_vec();
        let mut best_merit = merit(proposal).unwrap_or(Value::INFINITY);

        if best_merit.is_finite() && (!base_merit.is_finite() || best_merit <= base_merit) {
            return best_solution;
        }

        let mut alpha = Self::LINE_SEARCH_BACKTRACK;
        for _ in 0..Self::LINE_SEARCH_MAX_ITERS {
            let trial = Self::interpolate_solution(old, proposal, alpha);
            if let Some(trial_merit) = merit(&trial) {
                if trial_merit < best_merit {
                    best_merit = trial_merit;
                    best_solution = trial.clone();
                }

                let armijo_ok = !base_merit.is_finite()
                    || trial_merit <= base_merit * (1.0 - Self::ARMIJO_C1 * alpha);
                if armijo_ok {
                    return trial;
                }
            }
            alpha *= Self::LINE_SEARCH_BACKTRACK;
        }

        best_solution
    }

    fn apply_damping_strategy<F>(
        &self,
        old: &[Value],
        proposal: &[Value],
        damping_state: &mut NewtonDampingState,
        mut merit: F,
    ) -> Vec<Value>
    where
        F: FnMut(&[Value]) -> Option<Value>,
    {
        match self.config.convergence_config.damping_strategy {
            DampingStrategy::None => proposal.to_vec(),
            DampingStrategy::LineSearch => Self::line_search_step(old, proposal, &mut merit),
            DampingStrategy::VoltageLimiting => {
                Self::limit_step_delta(old, proposal, Self::MAX_DELTA_VOLTAGE_LIMIT)
            }
            DampingStrategy::BankRose => {
                let step_norm = Self::step_l2_norm(old, proposal);
                Self::update_bank_rose_alpha(damping_state, step_norm);
                Self::interpolate_solution(old, proposal, damping_state.bank_rose_alpha)
            }
            DampingStrategy::Combined => {
                let limited = Self::limit_step_delta(old, proposal, Self::MAX_DELTA_VOLTAGE_LIMIT);
                let step_norm = Self::step_l2_norm(old, &limited);
                Self::update_bank_rose_alpha(damping_state, step_norm);
                let bank_rose_step =
                    Self::interpolate_solution(old, &limited, damping_state.bank_rose_alpha);
                Self::line_search_step(old, &bank_rose_step, &mut merit)
            }
        }
    }

    fn clamp_solution_to_physical_bounds(solution: &mut [Value]) {
        for v in solution.iter_mut() {
            if !v.is_finite() {
                *v = 0.0;
            } else if v.abs() > Self::MAX_NODE_VOLTAGE {
                *v = v.signum() * Self::MAX_NODE_VOLTAGE;
            }
        }
    }

    #[inline]
    pub(crate) fn limit_vbic_external_updates(
        circuit: &CircuitData,
        proposal: &mut [Value],
        previous: &[Value],
        num_nodes: usize,
        protected_nodes: Option<&[bool]>,
        excess_phase_only: bool,
    ) -> bool {
        let mut changed = false;
        for _ in 0..3 {
            let mut pass_changed = false;
            for bjt in &circuit.bjts.devices {
                if !bjt.uses_vbic_dynamic_charges() {
                    continue;
                }
                if excess_phase_only && bjt.td <= 0.0 {
                    continue;
                }

                let node_voltage = |values: &[Value], node: usize| {
                    if node == 0 {
                        0.0
                    } else {
                        values.get(node - 1).copied().unwrap_or(0.0)
                    }
                };
                let previous_external = [
                    node_voltage(previous, bjt.node_collector),
                    node_voltage(previous, bjt.node_base),
                    node_voltage(previous, bjt.node_emitter),
                    node_voltage(previous, bjt.node_substrate),
                ];
                let proposed_external = [
                    node_voltage(proposal, bjt.node_collector),
                    node_voltage(proposal, bjt.node_base),
                    node_voltage(proposal, bjt.node_emitter),
                    node_voltage(proposal, bjt.node_substrate),
                ];
                let Some(scale) = bjt
                    .vbic_external_step_limit_scale_against_previous(
                        previous_external,
                        proposed_external,
                    )
                    .filter(|scale| scale.is_finite() && *scale + 1e-6 < 1.0)
                else {
                    continue;
                };

                for node in [
                    bjt.node_collector,
                    bjt.node_base,
                    bjt.node_emitter,
                    bjt.node_substrate,
                ] {
                    if node == 0 {
                        continue;
                    }
                    let proposal_idx = node - 1;
                    if proposal_idx >= num_nodes
                        || protected_nodes
                            .and_then(|protected| protected.get(proposal_idx))
                            .copied()
                            .unwrap_or(false)
                    {
                        continue;
                    }
                    let previous_value = previous[proposal_idx];
                    let proposal_value = proposal[proposal_idx];
                    let delta = proposal_value - previous_value;
                    if !delta.is_finite() || delta.abs() <= 0.0 {
                        continue;
                    }
                    proposal[proposal_idx] = previous_value + scale * delta;
                    pass_changed = true;
                }
            }

            if !pass_changed {
                break;
            }
            changed = true;
        }

        changed
    }

    fn nonlinear_merit_with_linear_stamp<F>(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
        mut linear_stamp: F,
    ) -> Option<Value>
    where
        F: FnMut(&mut CircuitData, &mut StaticMatrix, &mut [Value]),
    {
        let size = circuit.matrix_size();
        if solution.len() != size || solution.iter().any(|v| !v.is_finite()) {
            return None;
        }

        let mut rhs = vec![0.0; size];
        matrix.clear_values();
        linear_stamp(circuit, matrix, &mut rhs);
        self.stamp_nonlinear_devices_for_dc(circuit, matrix, &mut rhs, solution);

        let next_solution = matrix.solve(&rhs).ok()?;
        Some(Self::step_l2_norm(solution, &next_solution))
    }

    fn nonlinear_merit(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
    ) -> Option<Value> {
        let gmin_floor = self.config.convergence_config.gmin_target.max(0.0);
        self.nonlinear_merit_with_linear_stamp(circuit, matrix, solution, |circuit, matrix, rhs| {
            let node_count = circuit.num_nodes().min(rhs.len());
            for i in 0..node_count {
                matrix.add(i, i, gmin_floor);
            }
            circuit.stamp_dc_direct(matrix, rhs);
        })
    }

    #[inline]
    fn stamp_transient_operating_point_linear(
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        time: Value,
        gmin: Value,
    ) {
        let node_count = circuit.num_nodes().min(rhs.len());
        for i in 0..node_count {
            matrix.add(i, i, gmin);
        }

        circuit.stamp_transient_operating_point_direct(matrix, rhs);
        let num_nodes = circuit.num_nodes();
        circuit
            .voltage_sources
            .update_transient_rhs(rhs, time, |br_ordinal| num_nodes + br_ordinal);
        circuit.current_sources.update_transient_rhs(rhs, time);
    }

    fn nonlinear_merit_scaled(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
        source_scale: Value,
    ) -> Option<Value> {
        let gmin_floor = self.config.convergence_config.gmin_target.max(0.0);
        self.nonlinear_merit_with_linear_stamp(circuit, matrix, solution, |circuit, matrix, rhs| {
            let node_count = circuit.num_nodes().min(rhs.len());
            for i in 0..node_count {
                matrix.add(i, i, gmin_floor);
            }
            circuit.stamp_dc_direct_scaled(matrix, rhs, source_scale);
        })
    }

    fn nonlinear_merit_with_gmin(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
        gmin: Value,
    ) -> Option<Value> {
        self.nonlinear_merit_with_linear_stamp(circuit, matrix, solution, |circuit, matrix, rhs| {
            let node_count = circuit.num_nodes().min(rhs.len());
            for i in 0..node_count {
                matrix.add(i, i, gmin);
            }
            circuit.stamp_dc_direct(matrix, rhs);
        })
    }

    fn nonlinear_merit_with_pseudo_transient(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
        anchor_solution: &[Value],
        pseudo_conductance: Value,
    ) -> Option<Value> {
        let gmin_floor = self.config.convergence_config.gmin_target.max(0.0);
        self.nonlinear_merit_with_linear_stamp(circuit, matrix, solution, |circuit, matrix, rhs| {
            for i in 0..rhs.len() {
                matrix.add(i, i, gmin_floor + pseudo_conductance);
                rhs[i] += pseudo_conductance * anchor_solution[i];
            }
            circuit.stamp_dc_direct(matrix, rhs);
        })
    }

    fn normalize_initial_guess(initial_guess: &[Value], size: usize) -> Vec<Value> {
        if initial_guess.len() == size {
            initial_guess.to_vec()
        } else {
            let mut guess = vec![0.0; size];
            let copy_len = initial_guess.len().min(size);
            guess[..copy_len].copy_from_slice(&initial_guess[..copy_len]);
            guess
        }
    }

    #[inline]
    fn sanitize_initial_guess(initial_guess: &[Value], size: usize) -> Vec<Value> {
        let mut guess = Self::normalize_initial_guess(initial_guess, size);
        if Self::is_suspicious_solution(&guess) {
            guess.fill(0.0);
        }
        Self::clamp_solution_to_physical_bounds(&mut guess);
        guess
    }

    fn prefer_lower_merit_scaled_seed(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        incumbent: &[Value],
        proposal: &[Value],
        source_scale: Value,
    ) -> Vec<Value> {
        let size = circuit.matrix_size();
        let incumbent = Self::sanitize_initial_guess(incumbent, size);
        let proposal = Self::sanitize_initial_guess(proposal, size);

        if incumbent == proposal {
            return incumbent;
        }

        let incumbent_merit =
            self.nonlinear_merit_scaled(circuit, matrix, &incumbent, source_scale);
        let proposal_merit = self.nonlinear_merit_scaled(circuit, matrix, &proposal, source_scale);

        match (incumbent_merit, proposal_merit) {
            (Some(current), Some(candidate))
                if current.is_finite() && candidate.is_finite() && candidate < current =>
            {
                proposal
            }
            (None, Some(candidate)) if candidate.is_finite() => proposal,
            (Some(current), None) if current.is_finite() => incumbent,
            (Some(current), Some(candidate)) if !current.is_finite() && candidate.is_finite() => {
                proposal
            }
            _ => incumbent,
        }
    }

    fn solve_scaled_nonlinear_corrector(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        source_scale: Value,
        initial_solution: &[Value],
        damping_state: &mut NewtonDampingState,
        max_iterations: usize,
        abort: &dyn AbortSignal,
    ) -> (Vec<Value>, bool, usize) {
        let mut solution = initial_solution.to_vec();
        self.prime_operating_point_seed(circuit, &solution, 0.0, crate::xspice::AnalysisType::DcOp);
        let mut used_iterations = 0usize;
        let gmin_floor = self.config.convergence_config.gmin_target.max(0.0);

        for iter in 0..max_iterations {
            if Self::should_abort_iteration(abort, iter) {
                return (solution, false, used_iterations);
            }
            used_iterations = iter + 1;
            let mut rhs = vec![0.0; solution.len()];
            matrix.clear_values();

            let node_count = circuit.num_nodes().min(solution.len());
            for i in 0..node_count {
                matrix.add(i, i, gmin_floor);
            }

            circuit.stamp_dc_direct_scaled(matrix, &mut rhs, source_scale);
            self.stamp_nonlinear_devices_for_dc(circuit, matrix, &mut rhs, &solution);

            let raw_solution = match matrix.solve(&rhs) {
                Ok(sol) => sol,
                Err(_) => return (solution, false, used_iterations),
            };

            let mut new_solution =
                self.apply_damping_strategy(&solution, &raw_solution, damping_state, |trial| {
                    self.nonlinear_merit_scaled(circuit, matrix, trial, source_scale)
                });
            Self::limit_vbic_external_updates(
                circuit,
                &mut new_solution,
                &solution,
                circuit.num_nodes().min(solution.len()),
                None,
                false,
            );
            Self::clamp_solution_to_physical_bounds(&mut new_solution);

            let voltage_converged = self.voltage_convergence_met(&solution, &new_solution);
            let linearized_residual_converged =
                self.residual_convergence_met(matrix, &new_solution, &rhs);
            self.update_device_states_for_dc(circuit, &new_solution);
            solution = new_solution;

            if voltage_converged
                && linearized_residual_converged
                && circuit.nonlinear_converged(self.device_convergence_criteria())
                && self.nonlinear_residual_converged_scaled(
                    circuit,
                    matrix,
                    &solution,
                    source_scale,
                )
            {
                return (solution, true, used_iterations);
            }
        }

        (solution, false, used_iterations)
    }

    fn evaluate_fallback_candidate(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        candidate: Vec<Value>,
        method_name: &str,
        abort: &dyn AbortSignal,
    ) -> Option<Vec<Value>> {
        let suspicious = Self::is_suspicious_solution(&candidate);
        let validated =
            !suspicious && self.validate_nonlinear_solution(circuit, matrix, &candidate);
        if validated {
            return Some(candidate);
        }

        if !suspicious
            && let Some(refined) =
                self.refine_fallback_candidate(circuit, matrix, &candidate, abort)
        {
            log::info!(
                "{} candidate required Newton polishing and is now accepted.",
                method_name
            );
            return Some(refined);
        }

        if suspicious {
            if Self::has_clamped_values(&candidate) {
                log::warn!(
                    "{} produced clamped/non-finite values; candidate rejected.",
                    method_name
                );
            } else {
                log::warn!(
                    "{} produced suspiciously uniform values; candidate rejected.",
                    method_name
                );
            }
        } else {
            log::warn!(
                "{} candidate failed convergence re-validation; candidate rejected.",
                method_name
            );
        }

        None
    }

    fn refine_fallback_candidate(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        candidate: &[Value],
        abort: &dyn AbortSignal,
    ) -> Option<Vec<Value>> {
        let mut damping_state = NewtonDampingState::default();
        let refinement_iterations = self.continuation_iteration_budget(4, 12);
        let (refined, converged, _) = self.solve_scaled_nonlinear_corrector(
            circuit,
            matrix,
            1.0,
            candidate,
            &mut damping_state,
            refinement_iterations,
            abort,
        );
        if converged || self.validate_nonlinear_solution(circuit, matrix, &refined) {
            Some(refined)
        } else {
            None
        }
    }

    fn warm_restart_after_fallback(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        restart_seed: &[Value],
        abort: &dyn AbortSignal,
    ) -> Option<Vec<Value>> {
        let mut damping_state = NewtonDampingState::default();
        let restart_iterations = self.nonlinear_iteration_budget(10);
        let (restarted, converged, _) = self.solve_scaled_nonlinear_corrector(
            circuit,
            matrix,
            1.0,
            restart_seed,
            &mut damping_state,
            restart_iterations,
            abort,
        );
        if converged || self.validate_nonlinear_solution(circuit, matrix, &restarted) {
            Some(restarted)
        } else {
            None
        }
    }

    fn legacy_hfet_inverse_branch_seed(
        &self,
        circuit: &CircuitData,
        base_seed: &[Value],
    ) -> Option<Vec<Value>> {
        let first_legacy_hfet = circuit
            .jfets
            .iter()
            .find(|jfet| jfet.uses_hfet_legacy_inverse_mode())?;
        let node_count = circuit.num_nodes();
        if node_count == 0 || base_seed.len() < node_count {
            return None;
        }

        let polarity = first_legacy_hfet.jfet_type.polarity();
        let threshold = (polarity * first_legacy_hfet.params.vto)
            .abs()
            .clamp(0.05, 1.0);
        let branch_bias = -polarity * threshold;
        if !branch_bias.is_finite() || branch_bias.abs() < 1.0e-12 {
            return None;
        }

        let mut seed = Self::normalize_initial_guess(base_seed, circuit.matrix_size());
        let mut fixed = vec![false; node_count + 1];
        fixed[0] = true;

        for src_idx in 0..circuit.voltage_sources.names.len() {
            let pos = circuit.voltage_sources.node_pos[src_idx];
            let neg = circuit.voltage_sources.node_neg[src_idx];
            let dc = circuit.voltage_sources.dc_values[src_idx];
            if !dc.is_finite() {
                continue;
            }
            match (pos, neg) {
                (node, 0) if node > 0 && node <= node_count => {
                    fixed[node] = true;
                    seed[node - 1] = dc;
                }
                (0, node) if node > 0 && node <= node_count => {
                    fixed[node] = true;
                    seed[node - 1] = -dc;
                }
                (pos_node, neg_node)
                    if pos_node > 0
                        && pos_node <= node_count
                        && neg_node > 0
                        && neg_node <= node_count =>
                {
                    fixed[pos_node] = true;
                    fixed[neg_node] = true;
                }
                _ => {}
            }
        }

        let mut hfet_terminal = vec![false; node_count + 1];
        for jfet in &circuit.jfets {
            if !jfet.uses_hfet_legacy_inverse_mode() {
                continue;
            }
            for node in [jfet.drain, jfet.gate, jfet.source] {
                if node <= node_count {
                    hfet_terminal[node] = true;
                }
            }
        }

        let mut resistor_graph = vec![Vec::new(); node_count + 1];
        for stamp in &circuit.resistors.stamps {
            let a = stamp.pp.row;
            let b = stamp.nn.row;
            if a <= node_count && b <= node_count && a != b {
                resistor_graph[a].push(b);
                resistor_graph[b].push(a);
            }
        }

        let mut visited = vec![false; node_count + 1];
        let mut any_seeded = false;
        let mut queue = std::collections::VecDeque::new();
        for start in 0..=node_count {
            if visited[start] {
                continue;
            }

            visited[start] = true;
            queue.push_back(start);
            let mut component = Vec::new();
            let mut anchor_voltage = None;
            let mut has_conflicting_anchors = false;
            let mut has_legacy_hfet_terminal = false;

            while let Some(node) = queue.pop_front() {
                component.push(node);
                has_legacy_hfet_terminal |= hfet_terminal[node];
                if fixed[node] {
                    let voltage = if node == 0 { 0.0 } else { seed[node - 1] };
                    match anchor_voltage {
                        None => anchor_voltage = Some(voltage),
                        Some(anchor) if (anchor - voltage).abs() <= 1.0e-12 => {}
                        Some(_) => has_conflicting_anchors = true,
                    }
                }

                for &other in &resistor_graph[node] {
                    if other <= node_count && !visited[other] {
                        visited[other] = true;
                        queue.push_back(other);
                    }
                }
            }

            if !has_legacy_hfet_terminal || has_conflicting_anchors {
                continue;
            }

            let component_seed = anchor_voltage.unwrap_or(branch_bias);
            for node in component {
                if node > 0 {
                    seed[node - 1] = component_seed;
                    any_seeded = true;
                }
            }
        }

        any_seeded.then_some(seed)
    }

    fn validate_nonlinear_solution(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        solution: &[Value],
    ) -> bool {
        let size = circuit.matrix_size();
        if solution.len() != size || solution.iter().any(|v| !v.is_finite()) {
            return false;
        }

        // Re-evaluate one Newton linearization at the candidate point and require
        // a small fixed-point update plus device-level convergence.
        let mut rhs = vec![0.0; size];
        matrix.clear_values();
        let gmin_floor = self.config.convergence_config.gmin_target.max(0.0);
        let node_count = circuit.num_nodes().min(size);
        for i in 0..node_count {
            matrix.add(i, i, gmin_floor);
        }
        circuit.stamp_dc_direct(matrix, &mut rhs);
        self.stamp_nonlinear_devices_for_dc(circuit, matrix, &mut rhs, solution);
        let residual_converged = self.residual_convergence_met(matrix, solution, &rhs);

        let Ok(next_solution) = matrix.solve(&rhs) else {
            return false;
        };

        residual_converged && self.voltage_convergence_met(solution, &next_solution) && {
            self.update_device_states_for_dc(circuit, solution);
            circuit.nonlinear_converged(self.device_convergence_criteria())
        }
    }

    /// Try solving with a specific GMIN value
    pub(crate) fn try_solve_with_gmin(
        &self,
        circuit: &CircuitData,
        matrix: &mut StaticMatrix,
        gmin: Value,
    ) -> Result<Vec<Value>, SolverError> {
        let size = circuit.matrix_size();
        let mut rhs = vec![0.0; size];

        matrix.clear_values();
        rhs.fill(0.0);

        self.stamp_dc_direct(circuit, matrix, &mut rhs, gmin);
        matrix.solve(&rhs)
    }

    /// GMIN stepping: try progressively smaller GMIN values
    pub(crate) fn gmin_stepping(
        &self,
        circuit: &CircuitData,
        matrix: &mut StaticMatrix,
    ) -> Result<Vec<Value>, SolverError> {
        let gmin_values = self.gmin_linear_schedule();

        let mut solution = None;

        for (idx, &gmin) in gmin_values.iter().enumerate() {
            match self.try_solve_with_gmin(circuit, matrix, gmin) {
                Ok(sol) => {
                    solution = Some(sol);
                    // Continue to try smaller GMIN for better accuracy
                }
                Err(_) if solution.is_some() => {
                    // Can't solve with smaller GMIN, use the last successful one
                    break;
                }
                Err(e) if idx == gmin_values.len() - 1 => {
                    // Last GMIN value failed and we have no solution
                    return Err(e);
                }
                Err(_) => {
                    // Try next GMIN value
                    continue;
                }
            }
        }

        solution.ok_or(SolverError::SingularMatrix)
    }

    /// Source stepping: ramp sources from 0 to 100%
    pub(crate) fn source_stepping(
        &self,
        circuit: &CircuitData,
        matrix: &mut StaticMatrix,
    ) -> Result<Vec<Value>, SolverError> {
        // Source stepping sequence
        const SOURCE_SCALES: &[Value] = &[0.0, 0.1, 0.25, 0.5, 0.75, 1.0];
        let gmin_floor = self.config.convergence_config.gmin_target.max(0.0);

        let size = circuit.matrix_size();
        let mut solution = vec![0.0; size]; // Start from zero

        for &scale in SOURCE_SCALES {
            let mut rhs = vec![0.0; size];

            matrix.clear_values();
            rhs.fill(0.0);

            self.stamp_dc_scaled(circuit, matrix, &mut rhs, gmin_floor, scale);

            match matrix.solve(&rhs) {
                Ok(sol) => {
                    solution = sol;
                }
                Err(e) if scale == 1.0 => {
                    return Err(e);
                }
                Err(_) => {
                    // Try to continue with the current solution
                    continue;
                }
            }
        }

        Ok(solution)
    }

    /// Solve a linear circuit (no nonlinear devices)
    pub(crate) fn solve_linear(
        &self,
        circuit: &CircuitData,
        matrix: &mut StaticMatrix,
    ) -> Result<Vec<Value>, SimulationError> {
        let size = circuit.matrix_size();
        let mut rhs = vec![0.0; size];

        matrix.clear_values();
        rhs.fill(0.0);
        let gmin_floor = self.config.convergence_config.gmin_target.max(0.0);
        self.stamp_dc_direct(circuit, matrix, &mut rhs, gmin_floor);

        let direct_result = matrix.solve(&rhs);
        if let Ok(sol) = direct_result {
            return Ok(sol);
        }

        let mut last_err = direct_result.expect_err("checked Err branch");
        let conv_cfg = &self.config.convergence_config;

        if conv_cfg.gmin_stepping {
            match self.gmin_stepping(circuit, matrix) {
                Ok(sol) => return Ok(sol),
                Err(e) => {
                    last_err = e;
                }
            }
        }

        if conv_cfg.source_stepping {
            return self
                .source_stepping(circuit, matrix)
                .map_err(SimulationError::Solver);
        }

        Err(SimulationError::Solver(last_err))
    }

    /// Solve a nonlinear circuit using Newton-Raphson iteration
    ///
    /// This performs a linear pre-solve to get a warm-start initial guess,
    /// which helps convergence especially for BJT circuits where starting
    /// from 0V puts the transistor in an unphysical state.
    #[allow(dead_code)]
    pub(crate) fn solve_nonlinear(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
    ) -> Result<Vec<Value>, SimulationError> {
        self.solve_nonlinear_with_node_hints_and_abort(circuit, matrix, &[], &NoAbort)
    }

    /// Solve nonlinear DC with optional node-voltage hint overrides.
    ///
    /// `node_hints` entries are `(node_id, voltage)` with node IDs using the
    /// standard 1-based non-ground circuit numbering.
    #[allow(dead_code)]
    pub(crate) fn solve_nonlinear_with_node_hints(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        node_hints: &[(usize, Value)],
    ) -> Result<Vec<Value>, SimulationError> {
        self.solve_nonlinear_with_node_hints_and_abort(circuit, matrix, node_hints, &NoAbort)
    }

    pub(crate) fn solve_nonlinear_with_node_hints_and_abort(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        node_hints: &[(usize, Value)],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, SimulationError> {
        let size = circuit.matrix_size();
        let mut initial_guess = self
            .linear_presolve_for_guess(circuit, matrix)
            .unwrap_or_else(|| vec![0.0; size]);

        for &(node_id, voltage) in node_hints {
            if !voltage.is_finite() || node_id == 0 || node_id > circuit.num_nodes() {
                continue;
            }
            initial_guess[node_id - 1] = voltage;
        }

        Self::apply_bjt_initial_guess_correction(&mut initial_guess, circuit);
        self.solve_nonlinear_with_guess_and_abort(circuit, matrix, Some(&initial_guess), abort)
    }

    /// Apply BJT-specific initial guess corrections
    ///
    /// The linear presolve doesn't include BJT connections, so the base
    /// and emitter may have unrealistic voltage differences. This function
    /// corrects the initial guess to place the BJT in forward-active region:
    /// - VBE â‰ˆ 0.7V (typical forward bias)
    /// - VCE > VCE(sat) â‰ˆ 0.2V (avoid saturation)
    fn apply_bjt_initial_guess_correction(guess: &mut [Value], circuit: &CircuitData) {
        const VBE_FORWARD: Value = 0.7; // Typical forward B-E voltage
        const VCE_SAT: Value = 0.2; // Saturation voltage

        for bjt in &circuit.bjts.devices {
            let collector_node = bjt.node_collector;
            let base_node = bjt.node_base;
            let emitter_node = bjt.node_emitter;

            if base_node == 0 || emitter_node == 0 {
                continue;
            }

            let vc = if collector_node > 0 {
                guess[collector_node - 1]
            } else {
                0.0
            };
            let vb = guess[base_node - 1];
            let ve = guess[emitter_node - 1];

            // Strategy: Start with emitter voltage from linear presolve (respects resistor network)
            // Adjust base to be VBE_FORWARD above emitter
            // Adjust collector to be above base for forward-active

            let is_npn = matches!(bjt.bjt_type, crate::device::BjtType::Npn);

            if is_npn {
                // NPN: Vc > Vb > Ve, VBE â‰ˆ 0.7V, VCE > 0.2V
                // Keep emitter at linear presolve value (grounded through resistor)
                // Set base = emitter + 0.7V
                // Set collector to be above base (midpoint to VCC or similar)

                let ve_new = ve; // Keep emitter from linear presolve
                let vb_new = ve_new + VBE_FORWARD;
                let vc_new = (vb_new + vc.max(vb_new + VCE_SAT)) / 2.0; // Between base and original Vc
                let vc_new = vc_new.max(vb_new + VCE_SAT); // Ensure forward active

                if (vb - ve).abs() > 1.0 || vc < vb {
                    log::debug!(
                        "BJT {} (NPN): Correcting to forward active: Vc={:.2}->{:.2}, Vb={:.2}->{:.2}, Ve={:.2}->{:.2}",
                        bjt.name,
                        vc,
                        vc_new,
                        vb,
                        vb_new,
                        ve,
                        ve_new
                    );
                    guess[base_node - 1] = vb_new;
                    if collector_node > 0 {
                        guess[collector_node - 1] = vc_new;
                    }
                }
            } else {
                // PNP: Ve > Vb > Vc, VEB â‰ˆ 0.7V, VEC > 0.2V
                let ve_new = ve;
                let vb_new = ve_new - VBE_FORWARD;
                let vc_new = vb_new - VCE_SAT;

                if (ve - vb).abs() > 1.0 || vc > vb {
                    log::debug!(
                        "BJT {} (PNP): Correcting to forward active: Vc={:.2}->{:.2}, Vb={:.2}->{:.2}, Ve={:.2}->{:.2}",
                        bjt.name,
                        vc,
                        vc_new,
                        vb,
                        vb_new,
                        ve,
                        ve_new
                    );
                    guess[base_node - 1] = vb_new;
                    if collector_node > 0 {
                        guess[collector_node - 1] = vc_new;
                    }
                }
            }
        }
    }

    /// Perform linear pre-solve to get initial voltage guess
    ///
    /// This solves the circuit with only linear devices (nonlinear devices
    /// replaced by very high resistances) to establish DC source voltages
    /// through the resistor network. This provides a much better starting
    /// point for Newton iteration than all zeros.
    fn linear_presolve_for_guess_with_linear_stamp<F>(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        mut linear_stamp: F,
    ) -> Option<Vec<Value>>
    where
        F: FnMut(&mut CircuitData, &mut StaticMatrix, &mut [Value]),
    {
        let size = circuit.matrix_size();
        let num_nodes = circuit.num_nodes();

        matrix.clear_values();
        let mut rhs = vec![0.0; size];
        linear_stamp(circuit, matrix, &mut rhs);

        for i in 0..num_nodes {
            if let Some(idx) = matrix.get_index(i, i) {
                matrix.stamp_direct(idx, 1e-9);
            }
        }

        match matrix.solve(&rhs) {
            Ok(solution) => {
                log::debug!("Linear presolve succeeded, using as initial guess");
                for (i, &v) in solution.iter().enumerate().take(num_nodes) {
                    log::debug!("  Presolve V({}) = {:.4} V", i + 1, v);
                }
                Some(solution)
            }
            Err(_) => {
                log::debug!("Linear presolve failed, starting from zero");
                None
            }
        }
    }

    fn linear_presolve_for_guess(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
    ) -> Option<Vec<Value>> {
        self.linear_presolve_for_guess_with_linear_stamp(circuit, matrix, |circuit, matrix, rhs| {
            circuit.stamp_dc_direct(matrix, rhs);
        })
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
    #[allow(dead_code)]
    pub(crate) fn solve_nonlinear_with_guess(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        initial_guess: Option<&[Value]>,
    ) -> Result<Vec<Value>, SimulationError> {
        self.solve_nonlinear_with_guess_and_abort(circuit, matrix, initial_guess, &NoAbort)
    }

    pub(crate) fn solve_nonlinear_with_guess_and_abort(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        initial_guess: Option<&[Value]>,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, SimulationError> {
        let size = circuit.matrix_size();
        // Sanitize any warm-start seed before Newton so pathological presolve
        // artifacts do not launch the iteration from physically impossible rails.
        let mut solution = initial_guess
            .map(|guess| Self::sanitize_initial_guess(guess, size))
            .unwrap_or_else(|| vec![0.0; size]);
        Self::apply_bjt_initial_guess_correction(&mut solution, circuit);
        let startup_seed = solution.clone();
        self.prime_operating_point_seed(circuit, &solution, 0.0, crate::xspice::AnalysisType::DcOp);
        let mut rhs = vec![0.0; size];
        // Newton-Raphson iteration
        let mut hit_voltage_limit = false;
        let mut limited_nodes: Vec<usize> = Vec::new();
        let mut damping_state = NewtonDampingState::default();
        let gmin_floor = self.config.convergence_config.gmin_target.max(0.0);
        let requires_conservative_nonlinear_limiting = circuit.has_physical_nonlinear_devices();
        // Use 10x more iterations for DC nonlinear since damping limits voltage change per step
        // With MAX_DELTA_V=2V and standard max_iterations=50, we can only move 100V
        // Need 500+ iterations to traverse the full +/-1000V range if starting from a poor guess
        let dc_max_iterations = self.nonlinear_iteration_budget(10);
        for iteration in 0..dc_max_iterations {
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
            for i in 0..node_count {
                matrix.add(i, i, gmin_floor);
            }
            // Stamp linear devices
            circuit.stamp_dc_direct(matrix, &mut rhs);
            // Update nonlinear/behavioral/XSPICE devices with current solution and stamp
            self.stamp_nonlinear_devices_for_dc(circuit, matrix, &mut rhs, &solution);
            // Solve linearized system
            let raw_solution = matrix.solve(&rhs).map_err(SimulationError::Solver)?;
            // Voltage-limiting style damping is critical for strongly-coupled
            // semiconductor nonlinearities, but it can unnecessarily throttle
            // behavioral-only fixed-point updates (e.g., B-source macros that
            // legitimately require kilovolt-level solution jumps).
            let mut new_solution = if requires_conservative_nonlinear_limiting {
                self.apply_damping_strategy(&solution, &raw_solution, &mut damping_state, |trial| {
                    self.nonlinear_merit(circuit, matrix, trial)
                })
            } else {
                raw_solution
            };
            if requires_conservative_nonlinear_limiting {
                Self::limit_vbic_external_updates(
                    circuit,
                    &mut new_solution,
                    &solution,
                    circuit.num_nodes().min(size),
                    None,
                    false,
                );
            }
            // Solution limiting: prevent numerical blow-up by clamping extreme values
            // This is a critical convergence aid for circuits with strong nonlinearities
            for (i, v) in new_solution.iter_mut().enumerate() {
                if !v.is_finite() {
                    log::debug!(
                        "DC iter {}: NaN/Inf at node {}, resetting to 0",
                        iteration,
                        i + 1
                    );
                    *v = 0.0; // Replace NaN/Inf with zero
                } else if requires_conservative_nonlinear_limiting
                    && v.abs() > Self::MAX_NODE_VOLTAGE
                {
                    if !hit_voltage_limit {
                        hit_voltage_limit = true;
                        log::debug!(
                            "DC iter {}: Voltage limiting triggered - Newton-Raphson may struggle to converge",
                            iteration
                        );
                    }
                    if !limited_nodes.contains(&i) {
                        limited_nodes.push(i);
                        log::debug!(
                            "  Node {}: {:.2e}V -> clamped to {:.0}V",
                            i + 1,
                            *v,
                            v.signum() * Self::MAX_NODE_VOLTAGE
                        );
                    }
                    *v = v.signum() * Self::MAX_NODE_VOLTAGE;
                }
            }
            // Check convergence (both voltage change and device convergence)
            let voltage_converged = self.voltage_convergence_met(&solution, &new_solution);
            let linearized_residual_converged =
                self.residual_convergence_met(matrix, &new_solution, &rhs);
            // Device convergence must be checked at the candidate iterate, not the prior iterate.
            self.update_device_states_for_dc(circuit, &new_solution);
            let device_converged = circuit.nonlinear_converged(self.device_convergence_criteria());
            let nonlinear_residual_converged =
                self.nonlinear_residual_converged(circuit, matrix, &new_solution);
            solution = new_solution;
            if voltage_converged
                && linearized_residual_converged
                && device_converged
                && nonlinear_residual_converged
            {
                if hit_voltage_limit {
                    log::info!(
                        "DC operating point converged after {} iterations (voltage limiting was triggered)",
                        iteration + 1
                    );
                }
                if let Some(refined) =
                    self.refine_fallback_candidate(circuit, matrix, &solution, abort)
                {
                    return Ok(refined);
                }
                return Ok(solution);
            }
        }
        // Log diagnostic information when falling back to convergence aids.
        if hit_voltage_limit {
            log::warn!(
                "DC Newton-Raphson did not converge after {} iterations. \
                Voltage limiting triggered on {} node(s). Trying configured convergence aids...",
                dc_max_iterations,
                limited_nodes.len()
            );
        } else {
            log::info!(
                "DC Newton-Raphson did not converge after {} iterations. Trying configured convergence aids...",
                dc_max_iterations
            );
        }
        let conv_cfg = &self.config.convergence_config;
        let allow_source = conv_cfg.source_stepping;
        let allow_pseudo = conv_cfg.pseudo_transient;
        let allow_gmin = conv_cfg.gmin_stepping;
        let allow_arc = conv_cfg.arc_length;
        if !allow_source && !allow_pseudo && !allow_gmin && !allow_arc {
            return Err(SimulationError::ConvergenceFailed(dc_max_iterations));
        }

        if let Some(legacy_seed) = self.legacy_hfet_inverse_branch_seed(circuit, &startup_seed) {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            if let Some(restarted) =
                self.warm_restart_after_fallback(circuit, matrix, &legacy_seed, abort)
            {
                log::info!(
                    "Legacy HFET inverse-branch restart accepted after direct Newton failed."
                );
                return Ok(restarted);
            }
        }

        let zero_seed = vec![0.0; solution.len()];
        let mut fallback_seed =
            self.prefer_lower_merit_scaled_seed(circuit, matrix, &solution, &zero_seed, 1.0);

        if allow_source {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
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
                    ) {
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
                        self.warm_restart_after_fallback(circuit, matrix, &fallback_seed, abort)
                    {
                        log::info!(
                            "Source stepping warmed the nonlinear state; direct Newton restart accepted."
                        );
                        return Ok(restarted);
                    }
                }
                Err(e) => {
                    if !allow_pseudo && !allow_gmin && !allow_arc {
                        return Err(e);
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
                    ) {
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
                        self.warm_restart_after_fallback(circuit, matrix, &fallback_seed, abort)
                    {
                        log::info!(
                            "Pseudo-transient continuation warmed the nonlinear state; direct Newton restart accepted."
                        );
                        return Ok(restarted);
                    }
                }
                Err(e) => {
                    if !allow_gmin && !allow_arc {
                        return Err(e);
                    }
                    log::warn!(
                        "Pseudo-transient continuation failed with {}. Escalating to next configured aid.",
                        e
                    );
                }
            }
        }

        if allow_gmin {
            if abort.is_aborted() {
                return Err(SimulationError::Aborted);
            }
            match self.gmin_stepping_nonlinear_with_abort(circuit, matrix, &fallback_seed, abort) {
                Ok(gmin_solution) => {
                    if let Some(candidate) = self.evaluate_fallback_candidate(
                        circuit,
                        matrix,
                        gmin_solution.clone(),
                        "GMIN stepping",
                        abort,
                    ) {
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
                        self.warm_restart_after_fallback(circuit, matrix, &fallback_seed, abort)
                    {
                        log::info!(
                            "GMIN stepping warmed the nonlinear state; direct Newton restart accepted."
                        );
                        return Ok(restarted);
                    }
                }
                Err(e) => {
                    if !allow_arc {
                        return Err(e);
                    }
                    log::warn!(
                        "GMIN stepping failed with {}. Escalating to arc-length continuation.",
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
            ) {
                return Ok(candidate);
            }
            if let Some(restarted) =
                self.warm_restart_after_fallback(circuit, matrix, &arc_solution, abort)
            {
                log::info!(
                    "Arc-length continuation warmed the nonlinear state; direct Newton restart accepted."
                );
                return Ok(restarted);
            }
        }
        Err(SimulationError::ConvergenceFailed(dc_max_iterations))
    }

    pub(crate) fn solve_linear_transient_operating_point_with_abort(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        time: Value,
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, SimulationError> {
        if abort.is_aborted() {
            return Err(SimulationError::Aborted);
        }

        let size = circuit.matrix_size();
        matrix.clear_values();
        let mut rhs = vec![0.0; size];
        Self::stamp_transient_operating_point_linear(
            circuit,
            matrix,
            &mut rhs,
            time,
            self.config.convergence_config.gmin_target.max(0.0),
        );
        matrix.solve(&rhs).map_err(SimulationError::Solver)
    }

    pub(crate) fn solve_nonlinear_transient_op_with_node_hints_and_abort(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        time: Value,
        node_hints: &[(usize, Value)],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, SimulationError> {
        let size = circuit.matrix_size();
        let gmin_floor = self.config.convergence_config.gmin_target.max(0.0);
        let mut solution = self
            .linear_presolve_for_guess_with_linear_stamp(circuit, matrix, |circuit, matrix, rhs| {
                Self::stamp_transient_operating_point_linear(circuit, matrix, rhs, time, 0.0);
            })
            .unwrap_or_else(|| vec![0.0; size]);

        for &(node_id, voltage) in node_hints {
            if !voltage.is_finite() || node_id == 0 || node_id > circuit.num_nodes() {
                continue;
            }
            solution[node_id - 1] = voltage;
        }

        solution = Self::sanitize_initial_guess(&solution, size);
        Self::apply_bjt_initial_guess_correction(&mut solution, circuit);
        self.prime_operating_point_seed(
            circuit,
            &solution,
            time,
            crate::xspice::AnalysisType::Transient,
        );

        let requires_conservative_nonlinear_limiting = circuit.has_physical_nonlinear_devices();
        let mut rhs = vec![0.0; size];
        let mut damping_state = NewtonDampingState::default();
        let tranop_max_iterations = self.nonlinear_iteration_budget(10);

        for iteration in 0..tranop_max_iterations {
            if Self::should_abort_iteration(abort, iteration) {
                return Err(SimulationError::Aborted);
            }

            matrix.clear_values();
            rhs.fill(0.0);

            circuit.refresh_jiles_atherton_inductances(&solution);
            Self::stamp_transient_operating_point_linear(
                circuit, matrix, &mut rhs, time, gmin_floor,
            );
            self.stamp_nonlinear_devices_for_operating_point(
                circuit,
                matrix,
                &mut rhs,
                &solution,
                time,
                crate::xspice::AnalysisType::Transient,
            );

            let raw_solution = matrix.solve(&rhs).map_err(SimulationError::Solver)?;
            let mut new_solution = if requires_conservative_nonlinear_limiting {
                self.apply_damping_strategy(&solution, &raw_solution, &mut damping_state, |trial| {
                    self.nonlinear_merit_with_linear_stamp(
                        circuit,
                        matrix,
                        trial,
                        |circuit, matrix, rhs| {
                            circuit.refresh_jiles_atherton_inductances(trial);
                            Self::stamp_transient_operating_point_linear(
                                circuit, matrix, rhs, time, gmin_floor,
                            );
                        },
                    )
                })
            } else {
                raw_solution
            };
            if requires_conservative_nonlinear_limiting {
                Self::limit_vbic_external_updates(
                    circuit,
                    &mut new_solution,
                    &solution,
                    circuit.num_nodes().min(size),
                    None,
                    false,
                );
            }
            Self::clamp_solution_to_physical_bounds(&mut new_solution);

            let voltage_converged = self.voltage_convergence_met(&solution, &new_solution);
            let linearized_residual_converged =
                self.residual_convergence_met(matrix, &new_solution, &rhs);
            self.update_device_states_for_operating_point(
                circuit,
                &new_solution,
                time,
                crate::xspice::AnalysisType::Transient,
            );
            let device_converged = circuit.nonlinear_converged(self.device_convergence_criteria());
            let nonlinear_residual_converged = self.nonlinear_residual_converged_with_linear_stamp(
                circuit,
                matrix,
                &new_solution,
                |circuit, matrix, rhs| {
                    circuit.refresh_jiles_atherton_inductances(&new_solution);
                    Self::stamp_transient_operating_point_linear(
                        circuit, matrix, rhs, time, gmin_floor,
                    );
                },
            );

            solution = new_solution;
            if voltage_converged
                && linearized_residual_converged
                && device_converged
                && nonlinear_residual_converged
            {
                return Ok(solution);
            }
        }

        Err(SimulationError::ConvergenceFailed(tranop_max_iterations))
    }

    /// Check if voltage solution has converged using legacy signature.
    ///
    /// Uses `tolerance` as an absolute voltage tolerance with default SPICE-like
    /// relative tolerance of 1e-3.
    #[allow(dead_code)]
    pub(crate) fn check_voltage_convergence(
        old: &[Value],
        new: &[Value],
        tolerance: Value,
    ) -> bool {
        Self::check_voltage_convergence_with_tolerances(old, new, tolerance, 1e-3)
    }

    /// Check voltage convergence using explicit absolute and relative tolerances.
    ///
    /// Criterion: `|Î”V| <= VABSTOL + RELTOL * max(|Vnew|, |Vold|)`
    pub(crate) fn check_voltage_convergence_with_tolerances(
        old: &[Value],
        new: &[Value],
        voltage_abstol: Value,
        voltage_reltol: Value,
    ) -> bool {
        if old.len() != new.len() {
            return false;
        }
        let abstol = Self::sanitize_positive_tolerance(voltage_abstol, 1e-12);
        let reltol = Self::sanitize_positive_tolerance(voltage_reltol, 1e-3);

        for (&v_old, &v_new) in old.iter().zip(new.iter()) {
            if !v_old.is_finite() || !v_new.is_finite() {
                return false;
            }

            let delta = (v_new - v_old).abs();
            let limit = abstol + reltol * v_new.abs().max(v_old.abs());
            if delta > limit {
                return false;
            }
        }
        true
    }

    /// Source stepping for nonlinear circuits (starts from zero)
    #[allow(dead_code)]
    pub(crate) fn source_stepping_nonlinear(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
    ) -> Result<Vec<Value>, SimulationError> {
        let size = circuit.matrix_size();
        let zero_guess = vec![0.0; size];
        self.source_stepping_nonlinear_with_guess_and_abort(circuit, matrix, &zero_guess, &NoAbort)
    }

    /// Source stepping for nonlinear circuits with initial guess
    ///
    /// # Arguments
    /// * `circuit` - Circuit data with nonlinear devices
    /// * `matrix` - Sparse matrix structure  
    /// * `initial_guess` - Starting solution (e.g., from failed Newton iteration or previous sweep point)
    ///
    /// Source stepping ramps sources from 0% to 100% in steps, which helps
    /// find operating points in difficult circuits with strong nonlinearities.
    /// Uses finer granularity (11 steps) for commercial-grade convergence handling.
    #[allow(dead_code)]
    pub(crate) fn source_stepping_nonlinear_with_guess(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        initial_guess: &[Value],
    ) -> Result<Vec<Value>, SimulationError> {
        self.source_stepping_nonlinear_with_guess_and_abort(
            circuit,
            matrix,
            initial_guess,
            &NoAbort,
        )
    }

    pub(crate) fn source_stepping_nonlinear_with_guess_and_abort(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        initial_guess: &[Value],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, SimulationError> {
        // Finer source stepping for difficult circuits (mirrors Spectre/HSPICE approach)
        // Smaller initial steps help with BJT/MOSFET turn-on regions
        const SOURCE_SCALES: &[Value] =
            &[0.0, 0.01, 0.05, 0.1, 0.2, 0.35, 0.5, 0.65, 0.8, 0.9, 1.0];

        let size = circuit.matrix_size();

        let zero_guess = vec![0.0; size];
        let mut solution =
            self.prefer_lower_merit_scaled_seed(circuit, matrix, initial_guess, &zero_guess, 0.0);
        let mut damping_state = NewtonDampingState::default();
        let source_iterations = self.continuation_iteration_budget(20, 16);
        let (bootstrap_solution, _, _) = self.solve_scaled_nonlinear_corrector(
            circuit,
            matrix,
            0.0,
            &solution,
            &mut damping_state,
            source_iterations,
            abort,
        );
        solution = bootstrap_solution;
        let gmin_floor = self.config.convergence_config.gmin_target.max(0.0);

        for (scale_idx, &scale) in SOURCE_SCALES.iter().enumerate().skip(1) {
            if Self::should_abort_iteration(abort, scale_idx) {
                return Err(SimulationError::Aborted);
            }
            // Run Newton iterations at this source level
            // Use robust iteration budget so continuation still has work even
            // when the base direct-Newton budget is intentionally small.
            for iteration in 0..source_iterations {
                if Self::should_abort_iteration(abort, iteration) {
                    return Err(SimulationError::Aborted);
                }
                let mut rhs = vec![0.0; size];

                matrix.clear_values();

                // Add GMIN
                let node_count = circuit.num_nodes().min(size);
                for i in 0..node_count {
                    matrix.add(i, i, gmin_floor);
                }

                // Stamp linear devices with scaled sources
                circuit.stamp_dc_direct_scaled(matrix, &mut rhs, scale);

                // Stamp nonlinear/behavioral/XSPICE devices
                self.stamp_nonlinear_devices_for_dc(circuit, matrix, &mut rhs, &solution);

                match matrix.solve(&rhs) {
                    Ok(raw_solution) => {
                        let mut new_solution = self.apply_damping_strategy(
                            &solution,
                            &raw_solution,
                            &mut damping_state,
                            |trial| self.nonlinear_merit_scaled(circuit, matrix, trial, scale),
                        );
                        Self::limit_vbic_external_updates(
                            circuit,
                            &mut new_solution,
                            &solution,
                            circuit.num_nodes().min(size),
                            None,
                            false,
                        );
                        Self::clamp_solution_to_physical_bounds(&mut new_solution);

                        let converged = self.voltage_convergence_met(&solution, &new_solution);
                        let linearized_residual_converged =
                            self.residual_convergence_met(matrix, &new_solution, &rhs);
                        self.update_device_states_for_dc(circuit, &new_solution);
                        solution = new_solution;
                        if converged
                            && linearized_residual_converged
                            && circuit.nonlinear_converged(self.device_convergence_criteria())
                            && self.nonlinear_residual_converged_scaled(
                                circuit, matrix, &solution, scale,
                            )
                        {
                            break;
                        }
                    }
                    Err(e) if scale == 1.0 => {
                        return Err(SimulationError::Solver(e));
                    }
                    Err(_) => {
                        break; // Try next scale
                    }
                }
            }
        }

        Ok(solution)
    }

    /// Pseudo-transient continuation for difficult nonlinear circuits.
    ///
    /// Uses pseudo-capacitor anchoring (`Gpseudo * (x - x_prev)`) and grows the
    /// pseudo timestep as the solution stabilizes. This is a robust fallback for
    /// hard DC operating points where direct Newton/source/GMIN struggle.
    #[allow(dead_code)]
    pub(crate) fn pseudo_transient_nonlinear_with_guess(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        initial_guess: &[Value],
    ) -> Result<Vec<Value>, SimulationError> {
        self.pseudo_transient_nonlinear_with_guess_and_abort(
            circuit,
            matrix,
            initial_guess,
            &NoAbort,
        )
    }

    pub(crate) fn pseudo_transient_nonlinear_with_guess_and_abort(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        initial_guess: &[Value],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, SimulationError> {
        let size = circuit.matrix_size();
        let mut solution = Self::normalize_initial_guess(initial_guess, size);
        if Self::is_suspicious_solution(&solution) {
            solution.fill(0.0);
        }
        let mut anchor_solution = solution.clone();
        Self::clamp_solution_to_physical_bounds(&mut solution);
        Self::clamp_solution_to_physical_bounds(&mut anchor_solution);

        let mut pseudo = PseudoTransient::new();
        let mut damping_state = NewtonDampingState::default();
        let pseudo_iterations = self.continuation_iteration_budget(12, 16);

        let mut stage = 0usize;
        while !pseudo.is_complete() {
            if Self::should_abort_iteration(abort, stage) {
                return Err(SimulationError::Aborted);
            }
            let pseudo_conductance = pseudo.conductance(0);
            let mut stage_converged = false;

            for iteration in 0..pseudo_iterations {
                if Self::should_abort_iteration(abort, iteration) {
                    return Err(SimulationError::Aborted);
                }
                let mut rhs = vec![0.0; size];
                matrix.clear_values();

                for i in 0..size {
                    matrix.add(i, i, 1e-12 + pseudo_conductance);
                    rhs[i] += pseudo.current(anchor_solution[i]);
                }

                circuit.stamp_dc_direct(matrix, &mut rhs);
                self.stamp_nonlinear_devices_for_dc(circuit, matrix, &mut rhs, &solution);

                let raw_solution = match matrix.solve(&rhs) {
                    Ok(sol) => sol,
                    Err(_) => break,
                };

                let mut new_solution = self.apply_damping_strategy(
                    &solution,
                    &raw_solution,
                    &mut damping_state,
                    |trial| {
                        self.nonlinear_merit_with_pseudo_transient(
                            circuit,
                            matrix,
                            trial,
                            &anchor_solution,
                            pseudo_conductance,
                        )
                    },
                );
                Self::limit_vbic_external_updates(
                    circuit,
                    &mut new_solution,
                    &solution,
                    circuit.num_nodes().min(size),
                    None,
                    false,
                );
                Self::clamp_solution_to_physical_bounds(&mut new_solution);

                let converged = self.voltage_convergence_met(&solution, &new_solution);
                let linearized_residual_converged =
                    self.residual_convergence_met(matrix, &new_solution, &rhs);
                self.update_device_states_for_dc(circuit, &new_solution);
                solution = new_solution;

                if converged
                    && linearized_residual_converged
                    && circuit.nonlinear_converged(self.device_convergence_criteria())
                    && self.nonlinear_residual_converged_with_pseudo_transient(
                        circuit,
                        matrix,
                        &solution,
                        &anchor_solution,
                        pseudo_conductance,
                    )
                {
                    stage_converged = true;
                    break;
                }
            }

            if stage_converged {
                anchor_solution = solution.clone();
                pseudo.advance_on_success();
            } else if !pseudo.reduce_on_failure() {
                return Err(SimulationError::ConvergenceFailed(pseudo_iterations));
            }
            stage += 1;
        }

        Ok(solution)
    }

    /// Arc-length continuation fallback for strongly nonlinear or multi-solution circuits.
    ///
    /// Uses predictor-corrector continuation on source scale (lambda: 0 -> 1) with
    /// adaptive arc-length control. This improves robustness near turning points.
    #[allow(dead_code)]
    pub(crate) fn arc_length_continuation_nonlinear_with_guess(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        initial_guess: &[Value],
    ) -> Result<Vec<Value>, SimulationError> {
        self.arc_length_continuation_nonlinear_with_guess_and_abort(
            circuit,
            matrix,
            initial_guess,
            &NoAbort,
        )
    }

    pub(crate) fn arc_length_continuation_nonlinear_with_guess_and_abort(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        initial_guess: &[Value],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, SimulationError> {
        let size = circuit.matrix_size();
        let mut current_solution = Self::normalize_initial_guess(initial_guess, size);
        if Self::is_suspicious_solution(&current_solution) {
            current_solution.fill(0.0);
        }
        Self::clamp_solution_to_physical_bounds(&mut current_solution);
        let arc_newton_iters = self.continuation_iteration_budget(8, 16);

        let mut arc_cfg = ArcLengthConfig {
            tolerance: self.config.tolerance,
            max_steps: Self::ARC_LENGTH_MAX_STEPS,
            max_newton_iters: arc_newton_iters,
            ..ArcLengthConfig::default()
        };
        // Keep arc steps conservative for DC fallback stability.
        arc_cfg.initial_step = arc_cfg.initial_step.min(0.15);
        arc_cfg.max_step = arc_cfg.max_step.min(0.25);

        let mut arc = ArcLengthContinuation::with_config(size, arc_cfg.clone());
        let mut damping_state = NewtonDampingState::default();
        // Bootstrap to a consistent lambda=0 operating point before continuation.
        let (bootstrap_solution, _, _) = self.solve_scaled_nonlinear_corrector(
            circuit,
            matrix,
            0.0,
            &current_solution,
            &mut damping_state,
            arc_newton_iters,
            abort,
        );
        current_solution = bootstrap_solution;
        arc.initialize(&current_solution);

        let mut arc_step = 0usize;
        while !arc.is_complete() && !arc.is_failed() {
            if Self::should_abort_iteration(abort, arc_step) {
                return Err(SimulationError::Aborted);
            }
            let (predicted_solution, target_lambda) = arc.predict(&current_solution);
            let (corrected_solution, converged, newton_iters) = self
                .solve_scaled_nonlinear_corrector(
                    circuit,
                    matrix,
                    target_lambda,
                    &predicted_solution,
                    &mut damping_state,
                    arc_cfg.max_newton_iters,
                    abort,
                );

            if converged {
                arc.accept_step(&corrected_solution, target_lambda, newton_iters);
                current_solution = corrected_solution;
            } else if !arc.reject_step() {
                break;
            }
            arc_step += 1;
        }

        if arc.is_complete() {
            Ok(current_solution)
        } else {
            log::warn!(
                "Arc-length continuation did not reach lambda=1. Falling back to monotonic source continuation."
            );
            self.source_stepping_nonlinear_with_guess_and_abort(
                circuit,
                matrix,
                &current_solution,
                abort,
            )
        }
    }

    /// GMIN stepping for very difficult nonlinear circuits
    ///
    /// GMIN stepping starts with a large GMIN (1e-3) added to each node,
    /// then progressively reduces it to the normal value (1e-12).
    /// This helps BJT and other semiconductor devices find their operating
    /// point by initially providing a strong DC path that's gradually removed.
    /// This is a technique used in commercial SPICE simulators like Spectre/HSPICE.
    #[allow(dead_code)]
    pub(crate) fn gmin_stepping_nonlinear(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        initial_guess: &[Value],
    ) -> Result<Vec<Value>, SimulationError> {
        self.gmin_stepping_nonlinear_with_abort(circuit, matrix, initial_guess, &NoAbort)
    }

    pub(crate) fn gmin_stepping_nonlinear_with_abort(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        initial_guess: &[Value],
        abort: &dyn AbortSignal,
    ) -> Result<Vec<Value>, SimulationError> {
        let gmin_scales = self.gmin_nonlinear_schedule();

        let size = circuit.matrix_size();

        // Check for suspicious values - not just clamped at Â±999V but also
        // suspiciously uniform values that indicate failed source stepping.
        // Reset to zero if the guess looks like garbage.
        let is_garbage = Self::has_suspicious_uniformity(initial_guess);

        let mut solution: Vec<Value> = if is_garbage {
            log::debug!("GMIN stepping: resetting garbage initial guess to zero");
            vec![0.0; size]
        } else {
            initial_guess
                .iter()
                .map(|&v| if v.abs() >= 999.0 { 0.0 } else { v })
                .collect()
        };
        self.prime_operating_point_seed(circuit, &solution, 0.0, crate::xspice::AnalysisType::DcOp);
        let mut damping_state = NewtonDampingState::default();
        let gmin_iterations = self.continuation_iteration_budget(10, 12);

        for (step, &gmin) in gmin_scales.iter().enumerate() {
            if Self::should_abort_iteration(abort, step) {
                return Err(SimulationError::Aborted);
            }
            log::debug!("GMIN stepping: step {} with GMIN = {:.2e}", step + 1, gmin);

            // Use more iterations for GMIN stepping to allow convergence
            for iteration in 0..gmin_iterations {
                if Self::should_abort_iteration(abort, iteration) {
                    return Err(SimulationError::Aborted);
                }
                let mut rhs = vec![0.0; size];

                matrix.clear_values();

                // Add current GMIN only to node-voltage equations.
                let node_count = circuit.num_nodes().min(size);
                for i in 0..node_count {
                    matrix.add(i, i, gmin);
                }

                // Stamp linear and nonlinear devices
                circuit.stamp_dc_direct(matrix, &mut rhs);
                self.stamp_nonlinear_devices_for_dc(circuit, matrix, &mut rhs, &solution);

                // Log RHS on first iteration of first GMIN step for debugging
                if step == 0 && iteration == 0 {
                    log::debug!(
                        "GMIN step 1 iter 1 - RHS: {:?}",
                        rhs.iter().map(|v| format!("{:.2e}", v)).collect::<Vec<_>>()
                    );
                }

                match matrix.solve(&rhs) {
                    Ok(raw_solution) => {
                        let mut new_solution = self.apply_damping_strategy(
                            &solution,
                            &raw_solution,
                            &mut damping_state,
                            |trial| self.nonlinear_merit_with_gmin(circuit, matrix, trial, gmin),
                        );
                        Self::limit_vbic_external_updates(
                            circuit,
                            &mut new_solution,
                            &solution,
                            circuit.num_nodes().min(size),
                            None,
                            false,
                        );
                        Self::clamp_solution_to_physical_bounds(&mut new_solution);

                        let converged = self.voltage_convergence_met(&solution, &new_solution);
                        let linearized_residual_converged =
                            self.residual_convergence_met(matrix, &new_solution, &rhs);
                        self.update_device_states_for_dc(circuit, &new_solution);
                        solution = new_solution;
                        if converged
                            && linearized_residual_converged
                            && circuit.nonlinear_converged(self.device_convergence_criteria())
                            && self.nonlinear_residual_converged_with_gmin(
                                circuit, matrix, &solution, gmin,
                            )
                        {
                            break;
                        }
                    }
                    Err(e) if step == gmin_scales.len() - 1 => {
                        log::error!("GMIN stepping failed at final step: {:?}", e);
                        return Err(SimulationError::Solver(e));
                    }
                    Err(_) => {
                        log::debug!(
                            "Matrix solve failed at GMIN step {}, continuing...",
                            step + 1
                        );
                        break; // Try next GMIN level
                    }
                }
            }
            // Log solution after each GMIN step for debugging
            log::debug!(
                "GMIN step {} (GMIN={:.0e}) solution: {:?}",
                step + 1,
                gmin,
                solution
                    .iter()
                    .take(8)
                    .map(|v| format!("{:.2}", v))
                    .collect::<Vec<_>>()
            );
        }

        // Log the actual GMIN stepping result for debugging
        log::info!(
            "DC operating point after GMIN stepping ({} nodes): {:?}",
            solution.len(),
            solution
                .iter()
                .take(10)
                .map(|v| format!("{:.2}", v))
                .collect::<Vec<_>>()
        );

        // Final check: detect both clamped values and suspicious uniformity
        let has_clamped = solution.iter().any(|&v| v.abs() >= 999.0);

        // Check for suspicious uniformity (same issue as source stepping)
        let has_suspicious_uniformity = Self::has_suspicious_uniformity(&solution);

        if has_clamped {
            log::warn!(
                "GMIN stepping completed but solution still contains clamped values. \
                Circuit may need additional biasing or convergence aids."
            );
        } else if has_suspicious_uniformity {
            let unique_values: std::collections::HashSet<i32> =
                solution.iter().map(|v| (v * 100.0) as i32).collect();
            log::warn!(
                "GMIN stepping completed but solution has suspiciously uniform values ({} unique). \
                DC operating point may be incorrect.",
                unique_values.len()
            );
        } else {
            log::info!("GMIN stepping converged successfully");
        }

        Ok(solution)
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    /// Test voltage convergence checking with identical vectors.
    #[test]
    fn test_voltage_convergence_identical() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![1.0, 2.0, 3.0];
        assert!(Engine::check_voltage_convergence(&a, &b, 1e-9));
    }

    /// Test voltage convergence with small absolute differences.
    #[test]
    fn test_voltage_convergence_small_absolute() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![1.0 + 1e-10, 2.0 + 1e-10, 3.0 + 1e-10];
        assert!(Engine::check_voltage_convergence(&a, &b, 1e-9));
    }

    /// Test voltage convergence fails with large differences.
    #[test]
    fn test_voltage_convergence_large_diff() {
        let a = vec![1.0, 2.0, 3.0];
        let b = vec![1.0, 2.0, 4.0]; // 33% relative diff
        assert!(!Engine::check_voltage_convergence(&a, &b, 1e-9));
    }

    /// Test voltage convergence with near-zero values (special handling).
    #[test]
    fn test_voltage_convergence_near_zero() {
        let a = vec![1e-15, 0.0, 1e-14];
        let b = vec![0.0, 1e-15, 0.0];
        // Near-zero values should use absolute tolerance only
        assert!(Engine::check_voltage_convergence(&a, &b, 1e-9));
    }

    /// Test voltage convergence with mismatched lengths fails.
    #[test]
    fn test_voltage_convergence_mismatched_length() {
        let a = vec![1.0, 2.0];
        let b = vec![1.0, 2.0, 3.0];
        assert!(!Engine::check_voltage_convergence(&a, &b, 1e-9));
    }

    /// Test voltage convergence with empty vectors.
    #[test]
    fn test_voltage_convergence_empty() {
        let a: Vec<f64> = vec![];
        let b: Vec<f64> = vec![];
        assert!(Engine::check_voltage_convergence(&a, &b, 1e-9));
    }

    /// Test convergence at tolerance boundary.
    #[test]
    fn test_voltage_convergence_at_tolerance() {
        let tolerance: f64 = 1e-6;
        let a = vec![1.0];
        let b = vec![1.0 + tolerance * 0.9]; // Just under tolerance
        assert!(Engine::check_voltage_convergence(&a, &b, tolerance));
    }

    /// Test that relative tolerance kicks in for large values.
    #[test]
    fn test_voltage_convergence_relative_tolerance() {
        // For large values, relative tolerance of 1e-3 is used
        let a = vec![1000.0];
        let b = vec![1000.5]; // 0.05% difference, under 0.1%
        // 0.5 absolute diff > 1e-6 tolerance, but relative = 0.0005 < 1e-3
        assert!(Engine::check_voltage_convergence(&a, &b, 1e-6));
    }

    /// Test voltage convergence with negative values.
    #[test]
    fn test_voltage_convergence_negative_values() {
        let a = vec![-5.0, -10.0];
        let b = vec![-5.0 + 1e-10, -10.0 - 1e-10];
        assert!(Engine::check_voltage_convergence(&a, &b, 1e-9));
    }

    /// Test convergence with mixed positive/negative.
    #[test]
    fn test_voltage_convergence_mixed_signs() {
        let a = vec![5.0, -5.0, 0.0];
        let b = vec![5.0, -5.0, 0.0];
        assert!(Engine::check_voltage_convergence(&a, &b, 1e-9));
    }

    #[test]
    fn test_voltage_convergence_with_explicit_tolerances_uses_spice_rule() {
        let old = vec![1000.0, 1e-8];
        let new = vec![1000.8, 2e-8];

        assert!(Engine::check_voltage_convergence_with_tolerances(
            &old, &new, 1e-6, 1e-3
        ));
        assert!(!Engine::check_voltage_convergence_with_tolerances(
            &old, &new, 1e-6, 1e-5
        ));
    }

    #[test]
    fn test_voltage_convergence_with_explicit_tolerances_rejects_non_finite() {
        let old = vec![0.0, 1.0];
        let new_nan = vec![0.0, f64::NAN];
        let new_inf = vec![0.0, f64::INFINITY];
        assert!(!Engine::check_voltage_convergence_with_tolerances(
            &old, &new_nan, 1e-6, 1e-3
        ));
        assert!(!Engine::check_voltage_convergence_with_tolerances(
            &old, &new_inf, 1e-6, 1e-3
        ));
    }

    #[test]
    fn test_engine_voltage_tolerance_falls_back_to_legacy_tolerance() {
        let mut config = crate::engine::SimulationConfig::default();
        config.tolerance = 2e-5;
        config.convergence_config.voltage_abstol = 0.0;
        config.convergence_config.voltage_reltol = 1e-3;
        let engine = Engine::new(config);

        let old = vec![1.0];
        let new = vec![1.0 + 1.5e-5];
        assert!(engine.voltage_convergence_met(&old, &new));
    }

    #[test]
    fn test_device_convergence_criteria_uses_configured_tolerances() {
        let mut config = crate::engine::SimulationConfig::default();
        config.tolerance = 1e-3;
        config.convergence_config.voltage_abstol = 7e-7;
        config.convergence_config.current_abstol = 4e-13;
        config.convergence_config.voltage_reltol = 2e-3;
        let engine = Engine::new(config);
        let criteria = engine.device_convergence_criteria();
        assert!((criteria.voltage_abs - 7e-7).abs() < 1e-18);
        assert!((criteria.current_abs - 4e-13).abs() < 1e-24);
        assert!((criteria.rel - 2e-3).abs() < 1e-18);
    }

    #[test]
    fn test_current_abstol_uses_configured_current_tolerance() {
        let mut config = crate::engine::SimulationConfig::default();
        config.convergence_config.current_abstol = 4e-13;
        let engine = Engine::new(config);
        assert!((engine.current_abstol() - 4e-13).abs() < 1e-24);
    }

    #[test]
    fn test_residual_reltol_uses_configured_residual_tolerance() {
        let mut config = crate::engine::SimulationConfig::default();
        config.convergence_config.residual_reltol = 6e-4;
        let engine = Engine::new(config);
        assert!((engine.residual_reltol() - 6e-4).abs() < 1e-18);
    }

    #[test]
    fn test_residual_reltol_falls_back_to_voltage_reltol_when_invalid() {
        let mut config = crate::engine::SimulationConfig::default();
        config.convergence_config.voltage_reltol = 7e-4;
        config.convergence_config.residual_reltol = 0.0;
        let engine = Engine::new(config);
        assert!((engine.residual_reltol() - 7e-4).abs() < 1e-18);
    }

    #[test]
    fn test_residual_convergence_met_accepts_exact_solution() {
        let engine = Engine::new(crate::engine::SimulationConfig::default());
        let triplets = vec![(0, 0, 2.0), (0, 1, 1.0), (1, 0, 1.0), (1, 1, 3.0)];
        let matrix = StaticMatrix::from_triplets(2, 2, &triplets).unwrap();
        let rhs = vec![5.0, 7.0];
        let solution = vec![1.6, 1.8];
        assert!(engine.residual_convergence_met(&matrix, &solution, &rhs));
    }

    #[test]
    fn test_residual_convergence_met_rejects_large_equation_residual() {
        let engine = Engine::new(crate::engine::SimulationConfig::default());
        let triplets = vec![(0, 0, 2.0), (0, 1, 1.0), (1, 0, 1.0), (1, 1, 3.0)];
        let matrix = StaticMatrix::from_triplets(2, 2, &triplets).unwrap();
        let rhs = vec![5.0, 7.0];
        let solution = vec![1.2, 2.4];
        assert!(!engine.residual_convergence_met(&matrix, &solution, &rhs));
    }

    #[test]
    fn test_residual_convergence_met_rejects_non_finite_solution() {
        let engine = Engine::new(crate::engine::SimulationConfig::default());
        let triplets = vec![(0, 0, 1.0)];
        let matrix = StaticMatrix::from_triplets(1, 1, &triplets).unwrap();
        let rhs = vec![0.0];
        let solution = vec![f64::NAN];
        assert!(!engine.residual_convergence_met(&matrix, &solution, &rhs));
    }

    #[test]
    fn test_residual_convergence_met_uses_residual_reltol_not_voltage_reltol() {
        let triplets = vec![(0, 0, 1.0)];
        let matrix = StaticMatrix::from_triplets(1, 1, &triplets).unwrap();
        let rhs = vec![1.0];
        let solution = vec![1.001];

        let mut loose_cfg = crate::engine::SimulationConfig::default();
        loose_cfg.convergence_config.current_abstol = 1e-12;
        loose_cfg.convergence_config.voltage_reltol = 1.0;
        loose_cfg.convergence_config.residual_reltol = 1e-3;
        let loose_engine = Engine::new(loose_cfg);
        assert!(loose_engine.residual_convergence_met(&matrix, &solution, &rhs));

        let mut tight_cfg = crate::engine::SimulationConfig::default();
        tight_cfg.convergence_config.current_abstol = 1e-12;
        tight_cfg.convergence_config.voltage_reltol = 1.0;
        tight_cfg.convergence_config.residual_reltol = 1e-4;
        let tight_engine = Engine::new(tight_cfg);
        assert!(!tight_engine.residual_convergence_met(&matrix, &solution, &rhs));
    }

    /// Test that configured GMIN schedule is in decreasing order.
    #[test]
    fn test_gmin_stepping_values_order() {
        let engine = Engine::new(crate::engine::SimulationConfig::default());
        let gmin_values = engine.gmin_linear_schedule();
        for i in 1..gmin_values.len() {
            assert!(
                gmin_values[i] < gmin_values[i - 1],
                "GMIN values should be decreasing"
            );
        }
    }

    #[test]
    fn test_gmin_schedules_respect_custom_config_with_robust_floors() {
        let mut config = crate::engine::SimulationConfig::default();
        config.convergence_config.gmin_initial = 1e-1;
        config.convergence_config.gmin_target = 1e-6;
        let engine = Engine::new(config);

        let linear = engine.gmin_linear_schedule();
        let nonlinear = engine.gmin_nonlinear_schedule();

        assert!(linear.first().copied().unwrap_or_default() >= 1e-1);
        assert!(nonlinear.first().copied().unwrap_or_default() >= 1e-1);
        assert!(
            linear.last().copied().unwrap_or_default() <= 1e-6,
            "linear schedule should descend to configured/floored target"
        );
        assert!(
            nonlinear.last().copied().unwrap_or_default() <= 1e-6,
            "nonlinear schedule should descend to configured/floored target"
        );
    }

    /// Test that source stepping scales are in increasing order.
    #[test]
    fn test_source_stepping_values_order() {
        const SOURCE_SCALES: &[f64] = &[0.0, 0.1, 0.25, 0.5, 0.75, 1.0];
        for i in 1..SOURCE_SCALES.len() {
            assert!(
                SOURCE_SCALES[i] > SOURCE_SCALES[i - 1],
                "Source scales should be increasing"
            );
        }
        // Must start at 0 and end at 1
        assert!((SOURCE_SCALES[0] - 0.0).abs() < 1e-10);
        assert!((SOURCE_SCALES[SOURCE_SCALES.len() - 1] - 1.0).abs() < 1e-10);
    }

    #[test]
    fn test_nonlinear_iteration_budget_never_returns_zero() {
        let mut config = crate::engine::SimulationConfig::default();
        config.max_iterations = 0;
        let engine = Engine::new(config);

        assert_eq!(engine.nonlinear_iteration_budget(10), 1);
        assert_eq!(engine.nonlinear_iteration_budget(20), 1);
    }

    #[test]
    fn test_suspicious_solution_detection_flags_clamped_and_non_finite_values() {
        let clamped = vec![0.0, 1.0, 1000.0, -2.0];
        let non_finite = vec![0.0, f64::INFINITY, 1.0, 2.0];

        assert!(Engine::has_clamped_values(&clamped));
        assert!(Engine::has_clamped_values(&non_finite));
        assert!(Engine::is_suspicious_solution(&clamped));
        assert!(Engine::is_suspicious_solution(&non_finite));
    }

    #[test]
    fn test_suspicious_solution_detection_flags_uniform_stuck_vectors() {
        let stuck = vec![0.0, 0.0, 0.0, 0.01, 0.0, 0.0];
        let healthy = vec![0.0, 0.17, -0.32, 0.91, -1.4, 2.3];

        assert!(Engine::has_suspicious_uniformity(&stuck));
        assert!(!Engine::has_suspicious_uniformity(&healthy));
        assert!(Engine::is_suspicious_solution(&stuck));
        assert!(!Engine::is_suspicious_solution(&healthy));
    }

    #[test]
    fn test_limit_step_delta_clamps_large_updates() {
        let old = vec![0.0, -1.0, 0.25];
        let proposal = vec![2.0, -4.0, 0.50];
        let limited = Engine::limit_step_delta(&old, &proposal, 0.5);

        assert!((limited[0] - 0.5).abs() < 1e-12);
        assert!((limited[1] - (-1.5)).abs() < 1e-12);
        assert!((limited[2] - 0.50).abs() < 1e-12);
    }

    #[test]
    fn test_interpolate_solution_scales_newton_step() {
        let old = vec![1.0, -2.0];
        let proposal = vec![5.0, 2.0];
        let half_step = Engine::interpolate_solution(&old, &proposal, 0.5);
        let quarter_step = Engine::interpolate_solution(&old, &proposal, 0.25);

        assert!((half_step[0] - 3.0).abs() < 1e-12);
        assert!((half_step[1] - 0.0).abs() < 1e-12);
        assert!((quarter_step[0] - 2.0).abs() < 1e-12);
        assert!((quarter_step[1] - (-1.0)).abs() < 1e-12);
    }

    #[test]
    fn test_bank_rose_alpha_adapts_to_step_quality() {
        let mut state = NewtonDampingState::default();

        Engine::update_bank_rose_alpha(&mut state, 1.0);
        assert!((state.bank_rose_alpha - 1.0).abs() < 1e-12);

        // Larger step norm implies worse behavior -> alpha should shrink.
        Engine::update_bank_rose_alpha(&mut state, 2.0);
        assert!(state.bank_rose_alpha < 1.0);
        let reduced = state.bank_rose_alpha;

        // Improved step norm should recover alpha.
        Engine::update_bank_rose_alpha(&mut state, 0.5);
        assert!(state.bank_rose_alpha > reduced);
    }

    #[test]
    fn test_line_search_step_backtracks_to_lower_merit() {
        let old = vec![0.0];
        let proposal = vec![10.0];

        let mut merit = |x: &[Value]| Some((x[0] - 1.0).powi(2));
        let chosen = Engine::line_search_step(&old, &proposal, &mut merit);

        assert!(chosen[0] < proposal[0], "line search should backtrack");
        assert!((chosen[0] - 1.0).abs() < (proposal[0] - 1.0).abs());
    }

    #[test]
    fn test_apply_damping_strategy_none_preserves_proposal() {
        let mut config = crate::engine::SimulationConfig::default();
        config.convergence_config = crate::engine::ConvergenceConfig::fast()
            .with_damping(crate::engine::DampingStrategy::None);
        let engine = Engine::new(config);

        let old = vec![0.0, 0.0];
        let proposal = vec![2.0, -3.0];
        let mut state = NewtonDampingState::default();
        let damped = engine.apply_damping_strategy(&old, &proposal, &mut state, |_| Some(0.0));

        assert_eq!(damped, proposal);
    }

    #[test]
    fn test_apply_damping_strategy_voltage_limiting_clamps_delta() {
        let mut config = crate::engine::SimulationConfig::default();
        config.convergence_config = crate::engine::ConvergenceConfig::default()
            .with_damping(crate::engine::DampingStrategy::VoltageLimiting);
        let engine = Engine::new(config);

        let old = vec![0.0, 0.0];
        let proposal = vec![2.0, -3.0];
        let mut state = NewtonDampingState::default();
        let damped = engine.apply_damping_strategy(&old, &proposal, &mut state, |_| Some(0.0));

        assert!((damped[0] - Engine::MAX_DELTA_VOLTAGE_LIMIT).abs() < 1e-12);
        assert!((damped[1] + Engine::MAX_DELTA_VOLTAGE_LIMIT).abs() < 1e-12);
    }

    #[test]
    fn test_normalize_initial_guess_handles_size_mismatch() {
        let guess = vec![1.0, 2.0, 3.0];
        let normalized_short = Engine::normalize_initial_guess(&guess, 2);
        let normalized_long = Engine::normalize_initial_guess(&guess, 5);

        assert_eq!(normalized_short, vec![1.0, 2.0]);
        assert_eq!(normalized_long, vec![1.0, 2.0, 3.0, 0.0, 0.0]);
    }

    #[test]
    fn test_sanitize_initial_guess_resets_suspicious_vectors_and_clamps_extremes() {
        let suspicious = vec![0.0, 0.0, 0.0, 0.01, 0.0, 0.0];
        let sanitized_suspicious = Engine::sanitize_initial_guess(&suspicious, suspicious.len());
        assert_eq!(
            sanitized_suspicious,
            vec![0.0; suspicious.len()],
            "stuck continuation seeds should be reset to a neutral bootstrap state"
        );

        let extreme = vec![f64::INFINITY, -5000.0, 12.5];
        let sanitized_extreme = Engine::sanitize_initial_guess(&extreme, extreme.len());
        assert_eq!(
            sanitized_extreme,
            vec![0.0; extreme.len()],
            "clamped or non-finite continuation seeds should be discarded wholesale"
        );
    }

    #[test]
    fn test_clamp_solution_to_physical_bounds_limits_extremes() {
        let mut solution = vec![f64::INFINITY, -5000.0, 12.5];
        Engine::clamp_solution_to_physical_bounds(&mut solution);

        assert_eq!(solution[0], 0.0);
        assert_eq!(solution[1], -Engine::MAX_NODE_VOLTAGE);
        assert_eq!(solution[2], 12.5);
    }

    #[test]
    fn test_solve_nonlinear_with_guess_and_abort_stops_immediately() {
        let netlist = crate::Netlist::parse(
            r#"
V1 1 0 DC 1
D1 1 0 DMOD
.MODEL DMOD D (IS=1e-14)
.end
"#,
        )
        .expect("netlist parse should succeed");
        let engine = Engine::default();
        let mut circuit = engine
            .build_circuit(&netlist)
            .expect("circuit build should succeed");
        let mut matrix = engine
            .build_matrix(&circuit)
            .expect("matrix build should succeed");
        circuit.link_indices(&matrix);

        let abort = crate::abort_signal::ImmediateAbort;
        let result =
            engine.solve_nonlinear_with_guess_and_abort(&mut circuit, &mut matrix, None, &abort);
        assert!(
            matches!(result, Err(crate::engine::SimulationError::Aborted)),
            "expected immediate abort, got: {:?}",
            result
        );
    }

    #[test]
    fn test_legacy_hfet_inverse_branch_seed_biases_floating_hfet_terminals() {
        let deck_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("../../tests/hfet/inverter.cir");
        let source = std::fs::read_to_string(&deck_path).expect("read hfet inverter deck");
        let netlist = crate::Netlist::parse(&source).expect("parse hfet inverter deck");
        let engine = Engine::default();
        let mut circuit = engine.build_circuit(&netlist).expect("build circuit");
        let mut matrix = engine.build_matrix(&circuit).expect("build matrix");
        circuit.link_indices(&matrix);
        let linear_seed = engine
            .linear_presolve_for_guess(&mut circuit, &mut matrix)
            .expect("linear presolve should produce a finite seed");

        let seed = engine
            .legacy_hfet_inverse_branch_seed(&circuit, &linear_seed)
            .expect("legacy HFET deck should receive an inverse-branch seed");

        let node_voltage = |name: &str| {
            let node = circuit
                .get_node_by_name(name)
                .unwrap_or_else(|| panic!("missing node {name}"));
            seed[node - 1]
        };

        for name in [
            "3",
            "4",
            "x1.Z2.__dint",
            "x1.Z1.__sint",
            "x2.Z2.__dint",
            "x2.Z1.__sint",
        ] {
            assert!(
                (node_voltage(name) + 0.3).abs() < 1e-12,
                "{name} should be seeded onto the inverse HFET branch"
            );
        }

        assert!(
            (node_voltage("1") - 2.0).abs() < 1e-12,
            "ground-referenced supplies must remain fixed"
        );
        assert!(
            node_voltage("2").abs() < 1e-12,
            "ground-referenced inputs must remain fixed"
        );
        assert!(
            (node_voltage("x1.Z1.__dint") - 2.0).abs() < 1e-12,
            "series nodes tied to fixed supplies must retain their presolved bias"
        );
        assert!(
            node_voltage("x1.Z2.__sint").abs() < 1e-12,
            "series nodes tied to fixed inputs must retain their presolved bias"
        );
    }
}
