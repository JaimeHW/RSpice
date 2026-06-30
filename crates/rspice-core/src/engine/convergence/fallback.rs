//! Initial-guess preparation, fallback candidates, and warm restart helpers.

use super::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(in crate::engine::convergence) enum CorrectorSeedMode {
    Limited,
    StaticJfet,
    StaticJfetEveryIteration,
}

impl Engine {
    pub(in crate::engine::convergence) fn normalize_initial_guess(
        initial_guess: &[Value],
        size: usize,
    ) -> Vec<Value> {
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
    pub(in crate::engine::convergence) fn sanitize_initial_guess(
        initial_guess: &[Value],
        size: usize,
        node_count: usize,
    ) -> Vec<Value> {
        let mut guess = Self::normalize_initial_guess(initial_guess, size);
        if Self::is_suspicious_solution(&guess, node_count) {
            guess.fill(0.0);
        }
        Self::clamp_solution_to_physical_bounds(&mut guess, node_count);
        guess
    }

    pub(in crate::engine::convergence) fn apply_b3soi_pd_initial_guess_correction(
        guess: &mut [Value],
        circuit: &CircuitData,
    ) {
        #[inline]
        fn voltage(guess: &[Value], node: usize) -> Value {
            if node > 0 {
                guess.get(node - 1).copied().unwrap_or(0.0)
            } else {
                0.0
            }
        }

        #[inline]
        fn set_voltage(guess: &mut [Value], node: usize, value: Value) {
            if node > 0
                && let Some(slot) = guess.get_mut(node - 1)
            {
                *slot = value;
            }
        }

        for dev in &circuit.b3soi_pd.devices {
            if dev.body_mode != crate::device::mosfet::b3soi::pd::BodyMode::Floating {
                continue;
            }

            let source = voltage(guess, dev.node_source);
            let vbs_seed = dev.sized.vth0.abs().max(0.5);
            set_voltage(guess, dev.node_body, source + vbs_seed / dev.mtype);
        }
    }

    pub(in crate::engine::convergence) fn prefer_lower_merit_scaled_seed(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        incumbent: &[Value],
        proposal: &[Value],
        source_scale: Value,
    ) -> Vec<Value> {
        let size = circuit.matrix_size();
        let node_count = circuit.num_nodes().min(size);
        let incumbent = Self::sanitize_initial_guess(incumbent, size, node_count);
        let proposal = Self::sanitize_initial_guess(proposal, size, node_count);

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

    pub(in crate::engine::convergence) fn solve_scaled_nonlinear_corrector(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        source_scale: Value,
        initial_solution: &[Value],
        damping_state: &mut NewtonDampingState,
        max_iterations: usize,
        abort: &dyn AbortSignal,
    ) -> (Vec<Value>, bool, usize) {
        self.solve_scaled_nonlinear_corrector_with_seed_mode(
            circuit,
            matrix,
            source_scale,
            initial_solution,
            damping_state,
            max_iterations,
            abort,
            CorrectorSeedMode::Limited,
        )
    }

    pub(in crate::engine::convergence) fn solve_scaled_nonlinear_corrector_with_seed_mode(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        source_scale: Value,
        initial_solution: &[Value],
        damping_state: &mut NewtonDampingState,
        max_iterations: usize,
        abort: &dyn AbortSignal,
        seed_mode: CorrectorSeedMode,
    ) -> (Vec<Value>, bool, usize) {
        let mut solution = initial_solution.to_vec();
        self.prime_operating_point_seed(circuit, &solution, 0.0, crate::xspice::AnalysisType::DcOp);
        if matches!(
            seed_mode,
            CorrectorSeedMode::StaticJfet | CorrectorSeedMode::StaticJfetEveryIteration
        ) {
            circuit.update_jfet_static_linearizations(&solution);
        }
        let mut used_iterations = 0usize;
        let gmin_floor = self.dc_nodal_gmin_floor(circuit);
        let mut residual_stall_iterations = 0usize;

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
            if seed_mode == CorrectorSeedMode::StaticJfetEveryIteration {
                self.stamp_static_probe_nonlinear_devices_for_dc(
                    circuit, matrix, &mut rhs, &solution,
                );
            } else {
                self.stamp_nonlinear_devices_for_dc(circuit, matrix, &mut rhs, &solution);
            }

            let raw_solution = match matrix.solve(&rhs) {
                Ok(sol) => sol,
                Err(_) => return (solution, false, used_iterations),
            };

            let mut new_solution = self.apply_damping_strategy_with_junction_ownership(
                &solution,
                &raw_solution,
                damping_state,
                Self::junction_limiting_owns_newton_steps(circuit)
                    || self.b3soi_limiter_owns_global_damping(circuit),
                |trial| self.nonlinear_merit_scaled(circuit, matrix, trial, source_scale),
            );
            Self::clamp_solution_to_physical_bounds(&mut new_solution, node_count);

            let voltage_converged =
                self.node_voltage_convergence_met(&solution, &new_solution, node_count);
            let linearized_residual_converged =
                self.residual_convergence_met(matrix, &new_solution, &rhs);
            if seed_mode == CorrectorSeedMode::StaticJfetEveryIteration {
                self.update_device_states_for_dc(circuit, &new_solution);
                circuit.update_jfet_static_linearizations(&new_solution);
            } else {
                self.update_device_states_for_dc(circuit, &new_solution);
            }
            let device_converged = circuit.nonlinear_converged(self.device_convergence_criteria());
            let nonlinear_residual_converged = voltage_converged
                && device_converged
                && self.nonlinear_residual_converged_scaled(
                    circuit,
                    matrix,
                    &new_solution,
                    source_scale,
                );
            solution = new_solution;

            if voltage_converged && device_converged && nonlinear_residual_converged {
                return (solution, true, used_iterations);
            }

            if voltage_converged
                && device_converged
                && !(linearized_residual_converged || nonlinear_residual_converged)
            {
                residual_stall_iterations += 1;
                if residual_stall_iterations >= Self::DC_RESIDUAL_STALL_LIMIT {
                    break;
                }
            } else {
                residual_stall_iterations = 0;
            }
        }

        (solution, false, used_iterations)
    }

    pub(in crate::engine::convergence) fn evaluate_fallback_candidate(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        candidate: Vec<Value>,
        method_name: &str,
        abort: &dyn AbortSignal,
    ) -> Option<Vec<Value>> {
        let node_count = circuit.num_nodes().min(candidate.len());
        let suspicious = Self::is_suspicious_solution(&candidate, node_count);
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
            if Self::has_clamped_values(&candidate, node_count) {
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

    pub(in crate::engine::convergence) fn refine_fallback_candidate(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        candidate: &[Value],
        abort: &dyn AbortSignal,
    ) -> Option<Vec<Value>> {
        let mut damping_state = NewtonDampingState::default();
        let refinement_iterations = self.continuation_iteration_budget(4, 12);
        let (refined, converged, _) = self.solve_scaled_nonlinear_corrector_with_seed_mode(
            circuit,
            matrix,
            1.0,
            candidate,
            &mut damping_state,
            refinement_iterations,
            abort,
            CorrectorSeedMode::StaticJfetEveryIteration,
        );
        if converged || self.validate_nonlinear_solution(circuit, matrix, &refined) {
            Some(refined)
        } else {
            None
        }
    }

    pub(in crate::engine::convergence) fn warm_restart_after_fallback(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
        restart_seed: &[Value],
        abort: &dyn AbortSignal,
    ) -> Option<Vec<Value>> {
        let mut damping_state = NewtonDampingState::default();
        let restart_iterations = self.nonlinear_iteration_budget(10);
        let (restarted, converged, _) = self.solve_scaled_nonlinear_corrector_with_seed_mode(
            circuit,
            matrix,
            1.0,
            restart_seed,
            &mut damping_state,
            restart_iterations,
            abort,
            CorrectorSeedMode::StaticJfet,
        );
        if converged || self.validate_nonlinear_solution(circuit, matrix, &restarted) {
            Some(restarted)
        } else {
            None
        }
    }

    pub(in crate::engine::convergence) fn legacy_hfet_inverse_branch_seed(
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

    /// Apply BJT-specific initial guess corrections
    ///
    /// The linear presolve doesn't include BJT connections, so the base
    /// and emitter may have unrealistic voltage differences. This function
    /// corrects the initial guess to place the BJT in forward-active region:
    /// - VBE ≈ 0.7V (typical forward bias)
    /// - VCE > VCE(sat) ≈ 0.2V (avoid saturation)
    pub(in crate::engine::convergence) fn apply_bjt_initial_guess_correction(
        guess: &mut [Value],
        circuit: &CircuitData,
    ) {
        const VBE_FORWARD: Value = 0.7; // Typical forward B-E voltage
        const VCE_SAT: Value = 0.2; // Saturation voltage

        for bjt in &circuit.bjts.devices {
            if bjt.is_initially_off() {
                continue;
            }

            let collector_node = bjt.node_collector;
            let base_node = bjt.node_base;
            let emitter_node = bjt.node_emitter;

            if base_node == 0 {
                continue;
            }

            let node_voltage = |node: usize| {
                if node > 0 { guess[node - 1] } else { 0.0 }
            };
            let vc = node_voltage(collector_node);
            let vb = guess[base_node - 1];
            let ve = node_voltage(emitter_node);

            // Strategy: Start with emitter voltage from linear presolve (respects resistor network)
            // Adjust base to be VBE_FORWARD above emitter
            // Adjust collector to be above base for forward-active

            let is_npn = matches!(bjt.bjt_type, crate::device::BjtType::Npn);

            if is_npn {
                // NPN: Vc > Vb > Ve, VBE ≈ 0.7V, VCE > 0.2V
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
                // PNP: Ve > Vb > Vc, VEB ≈ 0.7V, VEC > 0.2V
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
    pub(in crate::engine::convergence) fn linear_presolve_for_guess_with_linear_stamp<F>(
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

    pub(in crate::engine::convergence) fn linear_presolve_for_guess(
        &self,
        circuit: &mut CircuitData,
        matrix: &mut StaticMatrix,
    ) -> Option<Vec<Value>> {
        self.linear_presolve_for_guess_with_linear_stamp(circuit, matrix, |circuit, matrix, rhs| {
            circuit.stamp_dc_direct(matrix, rhs);
        })
    }
}
