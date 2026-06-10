//! Intrinsic-state seed generation, nonlinear solve, and reduced linearization.

use super::*;

impl Bjt {
    pub(in crate::device::semiconductor::bjt) fn solve_intrinsic_state_with_external_continuation(
        &self,
        previous_external: [Value; EXTERNAL_DIM],
        previous_state: [Value; INTERNAL_DIM],
        target_external: [Value; EXTERNAL_DIM],
    ) -> Option<([Value; INTERNAL_DIM], Value)> {
        let mut current_external = previous_external;
        let mut current_state = previous_state;
        let mut lambda: Value = 0.0;
        let mut step: Value = 1.0;
        let use_linear_prediction = self.charge_model == BjtChargeModel::Vbic;
        let required_residual = if self.charge_model == BjtChargeModel::LegacyGummelPoon {
            1e-6
        } else {
            Value::INFINITY
        };

        while lambda < 1.0 - 1e-15 {
            let candidate_lambda = (lambda + step).min(1.0);
            let next_external = [
                previous_external[EXT_C]
                    + (target_external[EXT_C] - previous_external[EXT_C]) * candidate_lambda,
                previous_external[EXT_B]
                    + (target_external[EXT_B] - previous_external[EXT_B]) * candidate_lambda,
                previous_external[EXT_E]
                    + (target_external[EXT_E] - previous_external[EXT_E]) * candidate_lambda,
                previous_external[EXT_S]
                    + (target_external[EXT_S] - previous_external[EXT_S]) * candidate_lambda,
            ];

            let seed = if use_linear_prediction {
                self.predict_intrinsic_state_from_previous_external_bias(
                    current_external,
                    current_state,
                    next_external,
                )
                .unwrap_or(current_state)
            } else {
                current_state
            };
            let (solved_state, solved_residual) = self.solve_intrinsic_state_from_seed(
                next_external[EXT_C],
                next_external[EXT_B],
                next_external[EXT_E],
                next_external[EXT_S],
                seed,
            );

            if solved_residual.is_finite()
                && solved_residual <= required_residual
                && (!use_linear_prediction
                    || self.vbic_max_local_branch_delta(solved_state, seed) <= 0.1)
            {
                current_external = next_external;
                current_state = solved_state;
                lambda = candidate_lambda;
                step = (step * 2.0).min(1.0 - lambda).max(1e-6);
                continue;
            }

            if step <= 1.0 / 256.0 {
                return None;
            }
            step *= 0.5;
        }

        let residual = Self::intrinsic_state_residual_norm(
            &self
                .intrinsic_state_residual_jacobian(
                    target_external[EXT_C],
                    target_external[EXT_B],
                    target_external[EXT_E],
                    target_external[EXT_S],
                    current_state,
                )
                .0,
        );
        Some((current_state, residual))
    }

    #[inline]
    pub(in crate::device::semiconductor::bjt) fn has_intrinsic_state_unknowns(&self) -> bool {
        Self::series_active(self.rcx)
            || Self::series_active(self.rci)
            || Self::series_active(self.rbx)
            || Self::series_active(self.rbi)
            || Self::series_active(self.re)
            || Self::series_active(self.rs)
            || Self::series_active(self.rbp)
            || self.ibeip > 0.0
            || self.ibenp > 0.0
            || self.ibcip > 0.0
            || self.ibcnp > 0.0
            || self.self_heating_enabled()
    }

    #[inline]
    pub(in crate::device::semiconductor::bjt) fn intrinsic_state_seed_for_external_bias(
        &self,
        external: [Value; EXTERNAL_DIM],
    ) -> [Value; INTERNAL_DIM] {
        [
            external[EXT_C],
            external[EXT_C],
            external[EXT_B],
            external[EXT_B],
            external[EXT_E],
            external[EXT_C],
            external[EXT_S],
            if self.self_heating_enabled() {
                self.minimum_thermal_rise()
            } else {
                0.0
            },
        ]
    }

    #[inline]
    pub(in crate::device::semiconductor::bjt) fn legacy_startup_vcrit(&self) -> Value {
        let vt = self.vt.max(1e-12);
        let isat = self.is.max(1e-300);
        let arg = (vt / ((2.0_f64).sqrt() * isat)).max(1.0);
        vt * arg.ln()
    }

    #[inline]
    pub(in crate::device::semiconductor::bjt) fn legacy_startup_intrinsic_state_seed(
        &self,
        external: [Value; EXTERNAL_DIM],
    ) -> [Value; INTERNAL_DIM] {
        let mut seed = self.intrinsic_state_seed_for_external_bias(external);
        if self.charge_model != BjtChargeModel::LegacyGummelPoon {
            return seed;
        }

        let junction_seed = if self.initial_off {
            0.0
        } else {
            self.legacy_startup_vcrit()
        };
        let active_base = seed[IDX_VEI] + self.polarity() * junction_seed;

        seed[IDX_VCX] = active_base;
        seed[IDX_VCI] = active_base;
        seed[IDX_VBX] = active_base;
        seed[IDX_VBI] = active_base;
        seed[IDX_VBP] = active_base;
        seed
    }

    #[inline]
    pub(in crate::device::semiconductor::bjt) fn initial_forward_bias_anchor_external(
        &self,
        target_external: [Value; EXTERNAL_DIM],
    ) -> Option<[Value; EXTERNAL_DIM]> {
        let p = self.polarity();
        let max_forward_bias = 0.8;
        let mut anchor = target_external;
        let mut changed = false;

        let vbe = p * (anchor[EXT_B] - anchor[EXT_E]);
        if vbe.is_finite() && vbe > max_forward_bias {
            anchor[EXT_B] = anchor[EXT_E] + p * max_forward_bias;
            changed = true;
        }

        let vbc = p * (anchor[EXT_B] - anchor[EXT_C]);
        if vbc.is_finite() && vbc > max_forward_bias {
            anchor[EXT_B] = anchor[EXT_C] + p * max_forward_bias;
            changed = true;
        }

        changed.then_some(anchor)
    }

    pub(in crate::device::semiconductor::bjt) fn solve_intrinsic_state_from_forward_bias_anchor(
        &self,
        anchor_external: [Value; EXTERNAL_DIM],
        target_external: [Value; EXTERNAL_DIM],
    ) -> Option<([Value; INTERNAL_DIM], Value)> {
        let anchor_seed = self.intrinsic_state_seed_for_external_bias(anchor_external);
        let (anchor_state, anchor_residual_norm) = self.solve_intrinsic_state_from_seed(
            anchor_external[EXT_C],
            anchor_external[EXT_B],
            anchor_external[EXT_E],
            anchor_external[EXT_S],
            anchor_seed,
        );
        if !anchor_residual_norm.is_finite() {
            return None;
        }

        let mut best = self.solve_intrinsic_state_from_seed(
            target_external[EXT_C],
            target_external[EXT_B],
            target_external[EXT_E],
            target_external[EXT_S],
            anchor_state,
        );
        if self.charge_model == BjtChargeModel::Vbic {
            let projected_target_seed = self
                .predict_intrinsic_state_from_previous_external_bias(
                    anchor_external,
                    anchor_state,
                    target_external,
                )
                .unwrap_or(anchor_state);
            let projected = self.solve_intrinsic_state_from_seed(
                target_external[EXT_C],
                target_external[EXT_B],
                target_external[EXT_E],
                target_external[EXT_S],
                projected_target_seed,
            );
            if projected.1 + 1e-15 < best.1 {
                best = projected;
            }
        }

        if let Some(continued) = self.solve_intrinsic_state_with_external_continuation(
            anchor_external,
            anchor_state,
            target_external,
        ) && continued.1 + 1e-15 < best.1
        {
            best = continued;
        }

        best.1.is_finite().then_some(best)
    }

    pub(in crate::device::semiconductor::bjt) fn evaluate_state_fixed_temperature(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        vcx: Value,
        vci: Value,
        vbx: Value,
        vbi: Value,
        vei: Value,
        vbp: Value,
        vsi: Value,
    ) -> EvaluatedBjtState {
        let (linearized, intrinsic) = self.linearize_currents_with_branches(vbi - vei, vbi - vci);
        EvaluatedBjtState {
            linearized,
            ibe: intrinsic.ibe,
            ibc: intrinsic.ibc,
            iciei: intrinsic.iciei,
            ircx: self.ircx_branch(vc, vcx),
            irci: self.irci_branch(vcx, vci, vbi),
            irbx: self.irbx_branch(vb, vbx),
            irbi: self.irbi_branch(linearized, vbx, vbi),
            ire: self.ire_branch(ve, vei),
            ibep: self.ibep_branch(vbx, vbp),
            irbp: self.irbp_branch(vbx, vbi, vcx, vci, vbp, vsi),
            ibcp: self.ibcp_branch(vbp, vsi),
            iccp: self.iccp_branch(vbx, vbi, vci, vbp, vsi),
            irs: self.irs_branch(vs, vsi),
        }
    }

    pub(in crate::device::semiconductor::bjt) fn apply_thermal_derivative(
        base: &mut BranchLinearization,
        plus: BranchLinearization,
        minus: BranchLinearization,
        denom: Value,
    ) {
        base.d_internal[IDX_VRTH] = (plus.current - minus.current) / denom;
    }

    pub(in crate::device::semiconductor::bjt) fn evaluate_state(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        vcx: Value,
        vci: Value,
        vbx: Value,
        vbi: Value,
        vei: Value,
        vbp: Value,
        vsi: Value,
        vrth: Value,
    ) -> EvaluatedBjtState {
        let mut evaluated = self.with_temperature_variant(vrth, |model| {
            model
                .evaluate_state_fixed_temperature(vc, vb, ve, vs, vcx, vci, vbx, vbi, vei, vbp, vsi)
        });

        if !self.self_heating_enabled() {
            return evaluated;
        }

        let h = self.thermal_derivative_step(vrth);
        let plus = self.with_temperature_variant(vrth + h, |model| {
            model
                .evaluate_state_fixed_temperature(vc, vb, ve, vs, vcx, vci, vbx, vbi, vei, vbp, vsi)
        });
        let minus = self.with_temperature_variant(vrth - h, |model| {
            model
                .evaluate_state_fixed_temperature(vc, vb, ve, vs, vcx, vci, vbx, vbi, vei, vbp, vsi)
        });
        let denom = 2.0 * h;

        evaluated.linearized.dic_dvrth = (plus.linearized.ic - minus.linearized.ic) / denom;
        evaluated.linearized.dib_dvrth = (plus.linearized.ib - minus.linearized.ib) / denom;
        evaluated.linearized.dqb_dvrth = (plus.linearized.qb - minus.linearized.qb) / denom;
        Self::apply_thermal_derivative(&mut evaluated.ibe, plus.ibe, minus.ibe, denom);
        Self::apply_thermal_derivative(&mut evaluated.ibc, plus.ibc, minus.ibc, denom);
        Self::apply_thermal_derivative(&mut evaluated.iciei, plus.iciei, minus.iciei, denom);
        Self::apply_thermal_derivative(&mut evaluated.ircx, plus.ircx, minus.ircx, denom);
        Self::apply_thermal_derivative(&mut evaluated.irci, plus.irci, minus.irci, denom);
        Self::apply_thermal_derivative(&mut evaluated.irbx, plus.irbx, minus.irbx, denom);
        Self::apply_thermal_derivative(&mut evaluated.irbi, plus.irbi, minus.irbi, denom);
        Self::apply_thermal_derivative(&mut evaluated.ire, plus.ire, minus.ire, denom);
        Self::apply_thermal_derivative(&mut evaluated.ibep, plus.ibep, minus.ibep, denom);
        Self::apply_thermal_derivative(&mut evaluated.irbp, plus.irbp, minus.irbp, denom);
        Self::apply_thermal_derivative(&mut evaluated.ibcp, plus.ibcp, minus.ibcp, denom);
        Self::apply_thermal_derivative(&mut evaluated.iccp, plus.iccp, minus.iccp, denom);
        Self::apply_thermal_derivative(&mut evaluated.irs, plus.irs, minus.irs, denom);
        evaluated
    }

    pub(in crate::device::semiconductor::bjt) fn intrinsic_state_for_biases(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
    ) -> IntrinsicTerminalState {
        if self.cache_matches_external_biases(vc, vb, ve, vs) {
            IntrinsicTerminalState {
                vcx: self.vcx,
                vci: self.vci,
                vbx: self.vbx,
                vbi: self.vbi,
                vei: self.vei,
                vbp: self.vbp,
                vsi: self.vsi,
                vrth: self.vrth,
            }
        } else {
            self.solve_intrinsic_terminal_state(vc, vb, ve, vs)
        }
    }

    #[inline]
    pub(in crate::device::semiconductor::bjt) fn intrinsic_state_residual_norm(
        residual: &[Value; INTERNAL_DIM],
    ) -> Value {
        residual
            .iter()
            .fold(0.0, |max_norm, value| max_norm.max(value.abs()))
    }

    #[inline]
    pub(in crate::device::semiconductor::bjt) fn intrinsic_state_step_limit(
        iteration: usize,
        residual_norm: Value,
    ) -> Value {
        if residual_norm > 1e-2 {
            if iteration < 4 { 0.25 } else { 0.15 }
        } else if residual_norm > 1e-6 {
            0.1
        } else {
            0.05
        }
    }

    pub(in crate::device::semiconductor::bjt) fn solve_intrinsic_state_from_seed_with_thermal_scale(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        thermal_scale: Value,
        mut state: [Value; INTERNAL_DIM],
    ) -> ([Value; INTERNAL_DIM], Value) {
        let mut best_state = state;
        let mut best_residual_norm = Value::INFINITY;

        let max_iterations = if self.charge_model == BjtChargeModel::LegacyGummelPoon
            && self.has_intrinsic_state_unknowns()
        {
            128
        } else {
            32
        };

        for iteration in 0..max_iterations {
            let (residual, jacobian) = self.intrinsic_state_residual_jacobian_with_thermal_scale(
                vc,
                vb,
                ve,
                vs,
                state,
                thermal_scale,
            );
            let residual_norm = Self::intrinsic_state_residual_norm(&residual);
            if residual_norm < best_residual_norm {
                best_residual_norm = residual_norm;
                best_state = state;
            }
            if !residual_norm.is_finite() || residual_norm < 1e-14 {
                break;
            }

            let rhs = residual.map(|value| -value);
            let Some(delta) = Self::solve_small_dense_system(&jacobian, &rhs, INTERNAL_DIM) else {
                break;
            };

            let max_raw_delta = delta
                .iter()
                .fold(0.0_f64, |max_delta, value| max_delta.max(value.abs()));
            if max_raw_delta < 1e-13 {
                break;
            }

            let base_limit = Self::intrinsic_state_step_limit(iteration, residual_norm);
            let mut alpha = if max_raw_delta > base_limit {
                base_limit / max_raw_delta
            } else {
                1.0
            };
            alpha = alpha.clamp(1e-3, 1.0);

            let mut accepted = false;
            let mut candidate = state;
            let mut candidate_residual_norm = residual_norm;
            let mut best_candidate = state;
            let mut best_candidate_residual_norm = residual_norm;
            for _ in 0..12 {
                for idx in 0..INTERNAL_DIM {
                    candidate[idx] = state[idx] + alpha * delta[idx];
                }
                candidate = self.limit_intrinsic_state_against_previous(candidate, state);
                let (candidate_residual, _) = self
                    .intrinsic_state_residual_jacobian_with_thermal_scale(
                        vc,
                        vb,
                        ve,
                        vs,
                        candidate,
                        thermal_scale,
                    );
                candidate_residual_norm = Self::intrinsic_state_residual_norm(&candidate_residual);
                if candidate_residual_norm.is_finite()
                    && candidate_residual_norm < best_candidate_residual_norm
                {
                    best_candidate = candidate;
                    best_candidate_residual_norm = candidate_residual_norm;
                }
                if candidate_residual_norm.is_finite() && candidate_residual_norm < residual_norm {
                    accepted = true;
                    break;
                }
                alpha *= 0.5;
            }

            if !accepted && best_candidate_residual_norm < residual_norm {
                candidate = best_candidate;
                candidate_residual_norm = best_candidate_residual_norm;
                accepted = true;
            }

            if !accepted {
                break;
            }

            state = candidate;
            if candidate_residual_norm < best_residual_norm {
                best_residual_norm = candidate_residual_norm;
                best_state = state;
            }
            if candidate_residual_norm < 1e-14 {
                break;
            }
        }

        (best_state, best_residual_norm)
    }

    pub(in crate::device::semiconductor::bjt) fn solve_intrinsic_state_from_seed(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        state: [Value; INTERNAL_DIM],
    ) -> ([Value; INTERNAL_DIM], Value) {
        self.solve_intrinsic_state_from_seed_with_thermal_scale(vc, vb, ve, vs, 1.0, state)
    }

    pub(in crate::device::semiconductor::bjt) fn solve_intrinsic_state_with_self_heating_continuation(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        state: [Value; INTERNAL_DIM],
    ) -> ([Value; INTERNAL_DIM], Value) {
        let (direct_state, direct_residual_norm) =
            self.solve_intrinsic_state_from_seed(vc, vb, ve, vs, state);
        if !self.self_heating_enabled() {
            return (direct_state, direct_residual_norm);
        }

        let minimum_vrth = 0.0_f64.max(self.minimum_thermal_rise());
        let mut continuation_state = state;
        continuation_state[IDX_VRTH] = continuation_state[IDX_VRTH].max(minimum_vrth);
        for thermal_scale in [0.0, 0.05, 0.125, 0.25, 0.5, 0.75, 1.0] {
            if thermal_scale == 0.0 {
                continuation_state[IDX_VRTH] = minimum_vrth;
            }
            let (solved_state, _) = self.solve_intrinsic_state_from_seed_with_thermal_scale(
                vc,
                vb,
                ve,
                vs,
                thermal_scale,
                continuation_state,
            );
            continuation_state = solved_state;
        }

        let (continued_state, continued_residual_norm) =
            self.solve_intrinsic_state_from_seed(vc, vb, ve, vs, continuation_state);
        if continued_residual_norm < direct_residual_norm {
            (continued_state, continued_residual_norm)
        } else {
            (direct_state, direct_residual_norm)
        }
    }

    pub(in crate::device::semiconductor::bjt) fn rebalance_intrinsic_thermal_state(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        state: [Value; INTERNAL_DIM],
    ) -> [Value; INTERNAL_DIM] {
        if !self.self_heating_enabled() {
            return state;
        }

        let mut current_state = state;
        let mut best_state = state;
        let mut best_residual = Value::INFINITY;
        let minimum_vrth = self.minimum_thermal_rise();

        for _ in 0..8 {
            let (residual, jacobian) =
                self.intrinsic_state_residual_jacobian(vc, vb, ve, vs, current_state);
            let thermal_residual = residual[IDX_VRTH];
            let thermal_residual_abs = thermal_residual.abs();
            if thermal_residual_abs.is_finite() && thermal_residual_abs < best_residual {
                best_residual = thermal_residual_abs;
                best_state = current_state;
            }
            let thermal_derivative = jacobian[IDX_VRTH][IDX_VRTH];
            if !thermal_residual.is_finite()
                || !thermal_derivative.is_finite()
                || thermal_derivative.abs() < 1e-18
                || thermal_residual_abs < 1e-12
            {
                break;
            }

            let current_vrth = current_state[IDX_VRTH];
            let max_step = (current_vrth - minimum_vrth + 10.0).max(1.0) * 0.5;
            let step = (-thermal_residual / thermal_derivative).clamp(-max_step, max_step);
            if step.abs() < 1e-12 {
                break;
            }

            let mut alpha = 1.0;
            let mut accepted = false;
            let mut best_candidate = current_state;
            let mut best_candidate_residual = thermal_residual_abs;
            for _ in 0..10 {
                let raw_vrth = current_vrth + alpha * step;
                let candidate_vrth =
                    Self::limit_logarithmic_step(raw_vrth, current_vrth, 100.0).max(minimum_vrth);
                if (candidate_vrth - current_vrth).abs() < 1e-12 {
                    break;
                }

                let mut candidate = current_state;
                candidate[IDX_VRTH] = candidate_vrth;
                let candidate_residual = self
                    .intrinsic_state_residual_jacobian(vc, vb, ve, vs, candidate)
                    .0[IDX_VRTH]
                    .abs();
                if candidate_residual.is_finite() && candidate_residual < best_candidate_residual {
                    best_candidate = candidate;
                    best_candidate_residual = candidate_residual;
                }
                if candidate_residual.is_finite() && candidate_residual < thermal_residual_abs {
                    current_state = candidate;
                    accepted = true;
                    break;
                }
                alpha *= 0.5;
            }

            if accepted {
                continue;
            }
            if best_candidate_residual + 1e-15 < thermal_residual_abs {
                current_state = best_candidate;
                continue;
            }
            break;
        }

        best_state
    }

    pub(in crate::device::semiconductor::bjt) fn intrinsic_state_residual_jacobian_with_thermal_scale(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        state: [Value; INTERNAL_DIM],
        thermal_scale: Value,
    ) -> ([Value; INTERNAL_DIM], [[Value; INTERNAL_DIM]; INTERNAL_DIM]) {
        let has_rcx = Self::series_active(self.rcx);
        let has_rci = Self::series_active(self.rci);
        let has_rbx = Self::series_active(self.rbx);
        let has_rbi = Self::series_active(self.rbi);
        let has_re = Self::series_active(self.re);
        let has_rs = Self::series_active(self.rs);
        let has_self_heat = self.self_heating_enabled();
        let solve_vbp = self.vbic_solves_vbp();

        let [
            mut vcx,
            mut vci,
            mut vbx,
            mut vbi,
            mut vei,
            mut vbp,
            mut vsi,
            mut vrth,
        ] = state;
        if !has_rcx {
            vcx = vc;
        }
        if !has_rci {
            vci = vcx;
        }
        if !has_rbx {
            vbx = vb;
        }
        if !has_rbi {
            vbi = vbx;
        }
        if !has_re {
            vei = ve;
        }
        if !has_rs {
            vsi = vs;
        }
        if !solve_vbp {
            vbp = vcx;
        }
        if !has_self_heat {
            vrth = 0.0;
        }

        let eval = self.evaluate_state(vc, vb, ve, vs, vcx, vci, vbx, vbi, vei, vbp, vsi, vrth);
        let (collector_d, base_d, emitter_d) = self.intrinsic_terminal_derivatives(eval.linearized);
        let collector_internal = Self::branch_from_internal(eval.linearized.ic, collector_d);
        let base_internal = Self::branch_from_internal(eval.linearized.ib, base_d);
        let emitter_internal =
            Self::branch_from_internal(-(eval.linearized.ic + eval.linearized.ib), emitter_d);
        let thermal_sink = self.thermal_sink_branch(vrth);
        let thermal_power = Self::scale_branch(
            self.thermal_power_branch(eval, [vc, vb, ve, vs], state),
            thermal_scale,
        );

        let mut jacobian = [[0.0; INTERNAL_DIM]; INTERNAL_DIM];
        let mut residual = [0.0; INTERNAL_DIM];

        if has_rcx {
            let row = Self::sub_branches(
                Self::add_branches(eval.ircx, eval.irbp),
                if has_rci {
                    eval.irci
                } else {
                    collector_internal
                },
            );
            residual[IDX_VCX] = row.current;
            jacobian[IDX_VCX] = row.d_internal;
        } else {
            residual[IDX_VCX] = vcx - vc;
            jacobian[IDX_VCX][IDX_VCX] = 1.0;
        }

        if has_rci {
            let row = Self::sub_branches(eval.irci, collector_internal);
            residual[IDX_VCI] = row.current;
            jacobian[IDX_VCI] = row.d_internal;
        } else {
            residual[IDX_VCI] = vci - vcx;
            jacobian[IDX_VCI][IDX_VCI] = 1.0;
            jacobian[IDX_VCI][IDX_VCX] = -1.0;
        }

        if has_rbx {
            let row = Self::sub_branches(
                Self::sub_branches(
                    Self::sub_branches(eval.irbx, if has_rbi { eval.irbi } else { base_internal }),
                    eval.ibep,
                ),
                eval.iccp,
            );
            residual[IDX_VBX] = row.current;
            jacobian[IDX_VBX] = row.d_internal;
        } else {
            residual[IDX_VBX] = vbx - vb;
            jacobian[IDX_VBX][IDX_VBX] = 1.0;
        }

        if has_rbi {
            let row = Self::sub_branches(eval.irbi, base_internal);
            residual[IDX_VBI] = row.current;
            jacobian[IDX_VBI] = row.d_internal;
        } else {
            residual[IDX_VBI] = vbi - vbx;
            jacobian[IDX_VBI][IDX_VBI] = 1.0;
            jacobian[IDX_VBI][IDX_VBX] = -1.0;
        }

        if has_re {
            let row = Self::sub_branches(eval.ire, emitter_internal);
            residual[IDX_VEI] = row.current;
            jacobian[IDX_VEI] = row.d_internal;
        } else {
            residual[IDX_VEI] = vei - ve;
            jacobian[IDX_VEI][IDX_VEI] = 1.0;
        }

        if solve_vbp {
            let row = Self::sub_branches(Self::add_branches(eval.ibep, eval.ibcp), eval.irbp);
            residual[IDX_VBP] = row.current;
            jacobian[IDX_VBP] = row.d_internal;
        } else {
            residual[IDX_VBP] = vbp - vcx;
            jacobian[IDX_VBP][IDX_VBP] = 1.0;
            jacobian[IDX_VBP][IDX_VCX] = -1.0;
        }

        if has_rs {
            let row = Self::sub_branches(Self::add_branches(eval.irs, eval.iccp), eval.ibcp);
            residual[IDX_VSI] = row.current;
            jacobian[IDX_VSI] = row.d_internal;
        } else {
            residual[IDX_VSI] = vsi - vs;
            jacobian[IDX_VSI][IDX_VSI] = 1.0;
        }

        if has_self_heat {
            let row = Self::sub_branches(thermal_sink, thermal_power);
            residual[IDX_VRTH] = row.current;
            jacobian[IDX_VRTH] = row.d_internal;
        } else {
            residual[IDX_VRTH] = vrth;
            jacobian[IDX_VRTH][IDX_VRTH] = 1.0;
        }

        (residual, jacobian)
    }

    pub(in crate::device::semiconductor::bjt) fn intrinsic_state_residual_jacobian(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        state: [Value; INTERNAL_DIM],
    ) -> ([Value; INTERNAL_DIM], [[Value; INTERNAL_DIM]; INTERNAL_DIM]) {
        self.intrinsic_state_residual_jacobian_with_thermal_scale(vc, vb, ve, vs, state, 1.0)
    }

    pub(in crate::device::semiconductor::bjt) fn internal_kcl_linearization_from_eval(
        &self,
        state: IntrinsicTerminalState,
        eval: EvaluatedBjtState,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
    ) -> (
        [[Value; INTERNAL_DIM]; INTERNAL_DIM],
        [[Value; EXTERNAL_DIM]; INTERNAL_DIM],
        [Value; INTERNAL_DIM],
    ) {
        let has_rcx = Self::series_active(self.rcx);
        let has_rci = Self::series_active(self.rci);
        let has_rbx = Self::series_active(self.rbx);
        let has_rbi = Self::series_active(self.rbi);
        let has_re = Self::series_active(self.re);
        let has_rs = Self::series_active(self.rs);
        let has_self_heat = self.self_heating_enabled();
        let solve_vbp = self.vbic_solves_vbp();
        let (collector_d, base_d, emitter_d) = self.intrinsic_terminal_derivatives(eval.linearized);
        let collector_internal = Self::branch_from_internal(eval.linearized.ic, collector_d);
        let base_internal = Self::branch_from_internal(eval.linearized.ib, base_d);
        let emitter_internal =
            Self::branch_from_internal(-(eval.linearized.ic + eval.linearized.ib), emitter_d);
        let thermal_sink = self.thermal_sink_branch(state.vrth);
        let thermal_power = self.thermal_power_branch(
            eval,
            [vc, vb, ve, vs],
            [
                state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp, state.vsi,
                state.vrth,
            ],
        );

        let mut jacobian = [[0.0; INTERNAL_DIM]; INTERNAL_DIM];
        let mut external_partials = [[0.0; EXTERNAL_DIM]; INTERNAL_DIM];
        let mut source = [0.0; INTERNAL_DIM];
        let internal = [
            state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp, state.vsi, state.vrth,
        ];
        let external = [vc, vb, ve, vs];
        let assign_row = |row_idx: usize,
                          row: BranchLinearization,
                          jacobian: &mut [[Value; INTERNAL_DIM]; INTERNAL_DIM],
                          external_partials: &mut [[Value; EXTERNAL_DIM]; INTERNAL_DIM],
                          source: &mut [Value; INTERNAL_DIM]| {
            jacobian[row_idx] = row.d_internal;
            external_partials[row_idx] = row.d_external;
            source[row_idx] = row
                .d_internal
                .iter()
                .zip(internal.iter())
                .map(|(d, v)| d * v)
                .sum::<Value>()
                + row
                    .d_external
                    .iter()
                    .zip(external.iter())
                    .map(|(d, v)| d * v)
                    .sum::<Value>()
                - row.current;
        };

        if has_rcx {
            let row = Self::sub_branches(
                Self::add_branches(eval.ircx, eval.irbp),
                if has_rci {
                    eval.irci
                } else {
                    collector_internal
                },
            );
            assign_row(
                IDX_VCX,
                row,
                &mut jacobian,
                &mut external_partials,
                &mut source,
            );
        } else {
            jacobian[IDX_VCX][IDX_VCX] = 1.0;
            external_partials[IDX_VCX][EXT_C] = -1.0;
        }

        if has_rci {
            let row = Self::sub_branches(eval.irci, collector_internal);
            assign_row(
                IDX_VCI,
                row,
                &mut jacobian,
                &mut external_partials,
                &mut source,
            );
        } else {
            jacobian[IDX_VCI][IDX_VCI] = 1.0;
            jacobian[IDX_VCI][IDX_VCX] = -1.0;
        }

        if has_rbx {
            let row = Self::sub_branches(
                Self::sub_branches(
                    Self::sub_branches(eval.irbx, if has_rbi { eval.irbi } else { base_internal }),
                    eval.ibep,
                ),
                eval.iccp,
            );
            assign_row(
                IDX_VBX,
                row,
                &mut jacobian,
                &mut external_partials,
                &mut source,
            );
        } else {
            jacobian[IDX_VBX][IDX_VBX] = 1.0;
            external_partials[IDX_VBX][EXT_B] = -1.0;
        }

        if has_rbi {
            let row = Self::sub_branches(eval.irbi, base_internal);
            assign_row(
                IDX_VBI,
                row,
                &mut jacobian,
                &mut external_partials,
                &mut source,
            );
        } else {
            jacobian[IDX_VBI][IDX_VBI] = 1.0;
            jacobian[IDX_VBI][IDX_VBX] = -1.0;
        }

        if has_re {
            let row = Self::sub_branches(eval.ire, emitter_internal);
            assign_row(
                IDX_VEI,
                row,
                &mut jacobian,
                &mut external_partials,
                &mut source,
            );
        } else {
            jacobian[IDX_VEI][IDX_VEI] = 1.0;
            external_partials[IDX_VEI][EXT_E] = -1.0;
        }

        if solve_vbp {
            let row = Self::sub_branches(Self::add_branches(eval.ibep, eval.ibcp), eval.irbp);
            assign_row(
                IDX_VBP,
                row,
                &mut jacobian,
                &mut external_partials,
                &mut source,
            );
        } else {
            jacobian[IDX_VBP][IDX_VBP] = 1.0;
            jacobian[IDX_VBP][IDX_VCX] = -1.0;
        }

        if has_rs {
            let row = Self::sub_branches(Self::add_branches(eval.irs, eval.iccp), eval.ibcp);
            assign_row(
                IDX_VSI,
                row,
                &mut jacobian,
                &mut external_partials,
                &mut source,
            );
        } else {
            jacobian[IDX_VSI][IDX_VSI] = 1.0;
            external_partials[IDX_VSI][EXT_S] = -1.0;
        }

        if has_self_heat {
            let row = Self::sub_branches(thermal_sink, thermal_power);
            assign_row(
                IDX_VRTH,
                row,
                &mut jacobian,
                &mut external_partials,
                &mut source,
            );
        } else {
            jacobian[IDX_VRTH][IDX_VRTH] = 1.0;
        }

        (jacobian, external_partials, source)
    }

    pub(in crate::device::semiconductor::bjt) fn reduced_linearization_from_state_and_eval(
        &self,
        state: IntrinsicTerminalState,
        eval: EvaluatedBjtState,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
    ) -> BjtReducedLinearization {
        let (g_ii, g_ie, z_i_static) =
            self.internal_kcl_linearization_from_eval(state, eval, vc, vb, ve, vs);
        let terminal_currents = self.external_terminal_branches(eval);
        let (g_ei, g_ee, g_reduced) =
            Self::linearized_terminal_conductance_matrices(&g_ii, &g_ie, &terminal_currents);
        let internal = [
            state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp, state.vsi, state.vrth,
        ];
        let external = [vc, vb, ve, vs];
        let mut z_e_static = [0.0; EXTERNAL_DIM];
        for row in 0..EXTERNAL_DIM {
            z_e_static[row] = terminal_currents[row]
                .d_internal
                .iter()
                .zip(internal.iter())
                .map(|(d, v)| d * v)
                .sum::<Value>()
                + terminal_currents[row]
                    .d_external
                    .iter()
                    .zip(external.iter())
                    .map(|(d, v)| d * v)
                    .sum::<Value>()
                - terminal_currents[row].current;
        }
        let cached_dynamic_inputs = if self.uses_vbic_dynamic_charges() {
            let internal = [
                state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp, state.vsi,
                state.vrth, 0.0, 0.0,
            ];
            Some(if self.self_heating_enabled() {
                self.with_temperature_variant(state.vrth, |model| {
                    model.dynamic_charge_inputs(external, internal)
                })
            } else {
                self.dynamic_charge_inputs(external, internal)
            })
        } else {
            None
        };

        BjtReducedLinearization {
            internal_voltages: [
                state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp, state.vsi,
                state.vrth,
            ],
            external_voltages: [vc, vb, ve, vs],
            g_ii,
            g_ie,
            g_ei,
            g_ee,
            g_reduced,
            z_i_static,
            z_e_static,
            cached_dynamic_inputs,
        }
    }

    #[inline]
    pub(in crate::device::semiconductor::bjt) fn intrinsic_state_from_internal_vector(
        &self,
        internal: [Value; INTERNAL_DIM],
    ) -> IntrinsicTerminalState {
        let [vcx, vci, vbx, vbi, vei, vbp, vsi, vrth] = internal;

        IntrinsicTerminalState {
            vcx,
            vci,
            vbx,
            vbi,
            vei,
            vbp,
            vsi,
            vrth,
        }
    }

    pub(in crate::device::semiconductor::bjt) fn compute_reduced_linearization(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
    ) -> BjtReducedLinearization {
        let state = self.intrinsic_state_for_biases(vc, vb, ve, vs);
        let eval = self.evaluate_state(
            vc, vb, ve, vs, state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp,
            state.vsi, state.vrth,
        );
        self.reduced_linearization_from_state_and_eval(state, eval, vc, vb, ve, vs)
    }

    pub(crate) fn reduced_linearization(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
    ) -> BjtReducedLinearization {
        if self.reduced_linearization_cache_valid.get()
            && self.cache_matches_external_biases(vc, vb, ve, vs)
        {
            return self.reduced_linearization_cache.get();
        }

        let reduced = self.compute_reduced_linearization(vc, vb, ve, vs);
        if self.cache_matches_external_biases(vc, vb, ve, vs) {
            self.reduced_linearization_cache.set(reduced);
            self.reduced_linearization_cache_valid.set(true);
        }
        reduced
    }

    pub(in crate::device::semiconductor::bjt) fn linearized_terminal_conductance_matrices(
        g_ii: &[[Value; INTERNAL_DIM]; INTERNAL_DIM],
        g_ie: &[[Value; EXTERNAL_DIM]; INTERNAL_DIM],
        terminal_currents: &[BranchLinearization; EXTERNAL_DIM],
    ) -> (
        [[Value; INTERNAL_DIM]; EXTERNAL_DIM],
        [[Value; EXTERNAL_DIM]; EXTERNAL_DIM],
        BjtConductanceMatrix,
    ) {
        let mut g_ei = [[0.0; INTERNAL_DIM]; EXTERNAL_DIM];
        let mut g_ee = [[0.0; EXTERNAL_DIM]; EXTERNAL_DIM];
        for row in 0..EXTERNAL_DIM {
            g_ei[row] = terminal_currents[row].d_internal;
            g_ee[row] = terminal_currents[row].d_external;
        }

        let mut sensitivities = [[0.0; EXTERNAL_DIM]; INTERNAL_DIM];
        for external in 0..EXTERNAL_DIM {
            let rhs = g_ie.map(|partials| -partials[external]);
            if let Some(solution) = Self::solve_small_dense_system(g_ii, &rhs, INTERNAL_DIM) {
                for idx in 0..INTERNAL_DIM {
                    sensitivities[idx][external] = solution[idx];
                }
            }
        }

        let mut g_reduced = [[0.0; EXTERNAL_DIM]; EXTERNAL_DIM];
        for row in 0..EXTERNAL_DIM {
            for col in 0..EXTERNAL_DIM {
                let mut value = g_ee[row][col];
                for internal in 0..INTERNAL_DIM {
                    value += g_ei[row][internal] * sensitivities[internal][col];
                }
                g_reduced[row][col] = value;
            }
        }

        (g_ei, g_ee, g_reduced)
    }
}
