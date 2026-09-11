//! Intrinsic-state seed generation, nonlinear solve, and reduced linearization.

use super::*;

/// The node voltages one BJT evaluation is taken at: the four external
/// terminals followed by the seven internal nodes the parasitic network adds.
/// Eleven positional `Value`s were indistinguishable to the compiler; a
/// transposed pair changed the answer without changing the types.
#[derive(Clone, Copy)]
pub(in crate::device::semiconductor::bjt) struct BjtNodeVoltages {
    pub vc: Value,
    pub vb: Value,
    pub ve: Value,
    pub vs: Value,
    pub vcx: Value,
    pub vci: Value,
    pub vbx: Value,
    pub vbi: Value,
    pub vei: Value,
    pub vbp: Value,
    pub vsi: Value,
}

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
    pub(crate) fn has_intrinsic_state_unknowns(&self) -> bool {
        Self::series_active(self.rcx)
            || Self::series_active(self.rci)
            || Self::series_active(self.rbx)
            || Self::series_active(self.rbi)
            || Self::series_active(self.re)
            || self.has_substrate_resistance()
            || Self::series_active(self.rbp)
            || self.ibeip > 0.0
            || self.ibenp > 0.0
            || (!self.vbic_three_terminal && (self.ibcip > 0.0 || self.ibcnp > 0.0))
            || self.thermal_model_enabled()
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
            if self.vbic_three_terminal {
                0.0
            } else {
                external[EXT_S]
            },
            if self.thermal_model_enabled() && !self.vbic_13 {
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
        if self.charge_model != BjtChargeModel::LegacyGummelPoon
            || (!self.uses_legacy_junction_limiting() && !self.initial_off)
        {
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
        if !self.uses_legacy_junction_limiting() {
            return None;
        }
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
        voltages: BjtNodeVoltages,
    ) -> EvaluatedBjtState {
        let BjtNodeVoltages {
            vc,
            vb,
            ve,
            vs,
            vcx,
            vci,
            vbx,
            vbi,
            vei,
            vbp,
            vsi,
        } = voltages;
        let (linearized, intrinsic) =
            self.linearize_currents_with_branches(vbi - vei, vbx - vei, vbi - vci);
        EvaluatedBjtState {
            linearized,
            ibe: intrinsic.ibe,
            ibex: intrinsic.ibex,
            ibc: intrinsic.ibc,
            iciei: intrinsic.iciei,
            ircx: self.ircx_branch(vc, vcx),
            irci: self.irci_branch(vcx, vci, vbi),
            irbx: self.irbx_branch(vb, vbx),
            irbi: self.irbi_branch(linearized, vbx, vbi),
            ire: self.ire_branch(ve, vei),
            ibep: self.ibep_branch(vbx, vbp),
            irbp: self.irbp_branch(vbx, vbi, vcx, vci, vbp, vsi),
            ibcp: self.ibcp_branch(voltages),
            iccp: self.iccp_branch(vbx, vbi, vci, vbp, vsi),
            irs: self.irs_branch(vs, vsi),
            igcx: self.igcx_branch(vc, vcx, vbx),
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
        voltages: BjtNodeVoltages,
        vrth: Value,
    ) -> EvaluatedBjtState {
        self.evaluate_state_with_rbi_current(voltages, vrth, None)
    }

    pub(in crate::device::semiconductor::bjt) fn evaluate_state_with_rbi_current(
        &self,
        voltages: BjtNodeVoltages,
        vrth: Value,
        rbi_current: Option<Value>,
    ) -> EvaluatedBjtState {
        let BjtNodeVoltages {
            vci, vbx, vbi, vei, ..
        } = voltages;
        let evaluate = |model: &Bjt| {
            let mut state = model.evaluate_state_fixed_temperature(voltages);
            if let Some(current) = rbi_current {
                // Its independent current column is stamped by the MNA
                // owner; all voltage/temperature partials here hold I fixed.
                state.irbi = BranchLinearization {
                    current,
                    ..BranchLinearization::default()
                };
            }
            state
        };
        let mut evaluated = self.with_temperature_variant(vrth, evaluate);

        if !self.thermal_model_enabled() {
            return evaluated;
        }

        let h = self.thermal_derivative_step(vrth);
        let plus = self.with_temperature_derivative_variant(vrth + h, vrth, evaluate);
        let minus = self.with_temperature_derivative_variant(vrth - h, vrth, evaluate);
        let denom = 2.0 * h;

        evaluated.linearized.dic_dvrth = (plus.linearized.ic - minus.linearized.ic) / denom;
        evaluated.linearized.dib_dvrth = (plus.linearized.ib - minus.linearized.ib) / denom;
        evaluated.linearized.dqb_dvrth = (plus.linearized.qb - minus.linearized.qb) / denom;
        Self::apply_thermal_derivative(&mut evaluated.ibe, plus.ibe, minus.ibe, denom);
        Self::apply_thermal_derivative(&mut evaluated.ibex, plus.ibex, minus.ibex, denom);
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
        Self::apply_thermal_derivative(&mut evaluated.igcx, plus.igcx, minus.igcx, denom);
        if !self.vbic_13 || (self.tcvef == 0.0 && self.tcver == 0.0) {
            return evaluated;
        }
        let temperature_slope = self
            .mapped_temperature(self.requested_temperature() + vrth)
            .1;
        self.with_temperature_variant(vrth, |model| {
            let p = model.polarity();
            let vbe_eff = p * (vbi - vei);
            let vbc_eff = p * (vbi - vci);
            let transport = model.transport_charge_state(vbe_eff, vbc_eff);
            let early = model.vbic_early_thermal_derivatives(
                vbe_eff,
                vbc_eff,
                transport,
                temperature_slope,
            );
            let d_transport = p * (early.itzf - early.itzr);
            let avalanche = if model.vbic_13 && model.avc1 > 0.0 {
                model
                    .vbic13_avalanche_factor(vbc_eff, model.vjc, model.mjc, model.avc1, model.avc2)
                    .0
            } else {
                0.0
            };
            let d_ibc = -avalanche * d_transport;
            evaluated.iciei.d_internal[IDX_VRTH] += d_transport;
            evaluated.ibc.d_internal[IDX_VRTH] += d_ibc;
            evaluated.linearized.dic_dvrth += d_transport - d_ibc;
            evaluated.linearized.dib_dvrth += d_ibc;
            evaluated.linearized.dqb_dvrth += early.qb;
            if rbi_current.is_none() && Self::series_active(model.rbi) {
                evaluated.irbi.d_internal[IDX_VRTH] +=
                    (vbx - vbi) / model.guarded_series_resistance(model.rbi) * early.qb;
            }
        });
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
        crate::numerics::infinity_norm(residual)
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
        self.impose_intrinsic_node_constraints(&mut state, vc, vb, ve, vs);
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
            // A small current residual alone says nothing about the voltage
            // error of a small instance. Test the Newton voltage correction
            // below before accepting a nonzero residual.
            if !residual_norm.is_finite() || residual_norm == 0.0 {
                break;
            }

            let rhs = residual.map(|value| -value);
            let Some(delta) = crate::numerics::solve_small_dense(&jacobian, &rhs, INTERNAL_DIM)
            else {
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
                self.impose_intrinsic_node_constraints(&mut candidate, vc, vb, ve, vs);
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
        if !self.thermal_model_enabled() {
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
        if !self.thermal_model_enabled() {
            return state;
        }

        let mut current_state = state;
        let minimum_vrth = self.minimum_thermal_rise();

        for _ in 0..8 {
            let (residual, jacobian) =
                self.intrinsic_state_residual_jacobian(vc, vb, ve, vs, current_state);
            let thermal_residual = residual[IDX_VRTH];
            let thermal_residual_abs = thermal_residual.abs();
            let thermal_derivative = jacobian[IDX_VRTH][IDX_VRTH];
            // Residual and derivative both scale with instance multiplicity.
            // Their absolute sizes cannot certify a temperature equilibrium.
            if !thermal_residual.is_finite()
                || !thermal_derivative.is_finite()
                || thermal_derivative == 0.0
                || thermal_residual == 0.0
            {
                break;
            }

            let current_vrth = current_state[IDX_VRTH];
            let max_step = self.thermal_rebalance_step_limit(current_vrth);
            let step = (-thermal_residual / thermal_derivative).clamp(-max_step, max_step);
            if step == 0.0 {
                break;
            }

            let mut alpha = 1.0;
            let mut accepted = false;
            for _ in 0..10 {
                let raw_vrth = current_vrth + alpha * step;
                if !raw_vrth.is_finite() {
                    alpha *= 0.5;
                    continue;
                }
                let candidate_vrth =
                    Self::limit_logarithmic_step(raw_vrth, current_vrth, 100.0).max(minimum_vrth);
                if candidate_vrth == current_vrth {
                    break;
                }

                let mut candidate = current_state;
                candidate[IDX_VRTH] = candidate_vrth;
                let candidate_residual = self
                    .intrinsic_state_residual_jacobian(vc, vb, ve, vs, candidate)
                    .0[IDX_VRTH]
                    .abs();
                if candidate_residual.is_finite() && candidate_residual < thermal_residual_abs {
                    current_state = candidate;
                    accepted = true;
                    break;
                }
                alpha *= 0.5;
            }

            if !accepted {
                break;
            }
        }

        // Every accepted candidate strictly improves the thermal residual,
        // including the candidate accepted on the final allowed iteration.
        current_state
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
        let has_rs = self.has_substrate_resistance();
        let has_self_heat = self.thermal_model_enabled();
        let solve_vbp = self.vbic_solves_vbp();

        // Limiters and predictors can displace nodes joined by ideal wires.
        // Evaluate the candidate itself so the identity rows below measure
        // those displacements and agree with their Jacobian derivatives.
        let [vcx, vci, vbx, vbi, vei, vbp, vsi, vrth] = state;

        let eval = self.evaluate_state(
            BjtNodeVoltages {
                vc,
                vb,
                ve,
                vs,
                vcx,
                vci,
                vbx,
                vbi,
                vei,
                vbp,
                vsi,
            },
            vrth,
        );
        let [collector_internal, base_internal, emitter_internal] =
            self.intrinsic_terminal_branches(&eval);
        let thermal_sink = self.thermal_sink_branch(vrth);
        let thermal_power = Self::scale_branch(
            self.thermal_power_branch(eval, [vc, vb, ve, vs], state),
            thermal_scale,
        );

        let mut jacobian = [[0.0; INTERNAL_DIM]; INTERNAL_DIM];
        let mut residual = [0.0; INTERNAL_DIM];

        if has_rcx {
            let row = Self::sub_branches(
                Self::add_branches(
                    Self::add_branches(eval.ircx, self.parasitic_base_collector_branch(&eval)),
                    eval.igcx,
                ),
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
                    Self::sub_branches(
                        Self::sub_branches(
                            eval.irbx,
                            if has_rbi { eval.irbi } else { base_internal },
                        ),
                        eval.ibex,
                    ),
                    eval.ibep,
                ),
                Self::add_branches(eval.iccp, eval.igcx),
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
            let row = Self::sub_branches(Self::add_branches(eval.ire, eval.ibex), emitter_internal);
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
            residual[IDX_VSI] = vsi - if self.vbic_three_terminal { 0.0 } else { vs };
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
        let internal = [
            state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp, state.vsi, state.vrth,
        ];
        let external = [vc, vb, ve, vs];
        self.internal_kcl_linearization_from_eval_with_source(state, eval, external, |row| {
            row.source(&internal, &external)
        })
    }

    pub(in crate::device::semiconductor::bjt) fn internal_kcl_linearization_from_eval_with_source(
        &self,
        state: IntrinsicTerminalState,
        eval: EvaluatedBjtState,
        external: [Value; EXTERNAL_DIM],
        source_for_branch: impl Fn(BranchLinearization) -> Value,
    ) -> (
        [[Value; INTERNAL_DIM]; INTERNAL_DIM],
        [[Value; EXTERNAL_DIM]; INTERNAL_DIM],
        [Value; INTERNAL_DIM],
    ) {
        let mut jacobian = [[0.0; INTERNAL_DIM]; INTERNAL_DIM];
        let mut external_partials = [[0.0; EXTERNAL_DIM]; INTERNAL_DIM];
        let mut source = [0.0; INTERNAL_DIM];
        // Inactive private coordinates retain their exact alias constraints.
        for (row, active, internal_parent, external_parent) in [
            (IDX_VCX, Self::series_active(self.rcx), None, Some(EXT_C)),
            (IDX_VCI, Self::series_active(self.rci), Some(IDX_VCX), None),
            (IDX_VBX, Self::series_active(self.rbx), None, Some(EXT_B)),
            (IDX_VBI, Self::series_active(self.rbi), Some(IDX_VBX), None),
            (IDX_VEI, Self::series_active(self.re), None, Some(EXT_E)),
            (IDX_VBP, self.vbic_solves_vbp(), Some(IDX_VCX), None),
            (
                IDX_VSI,
                self.has_substrate_resistance(),
                None,
                (!self.vbic_three_terminal).then_some(EXT_S),
            ),
            (IDX_VRTH, self.thermal_model_enabled(), None, None),
        ] {
            if !active {
                jacobian[row][row] = 1.0;
                if let Some(column) = internal_parent {
                    jacobian[row][column] = -1.0;
                }
                if let Some(column) = external_parent {
                    external_partials[row][column] = -1.0;
                }
            }
        }
        self.visit_internal_kcl_terms(state, eval, external, false, |index, terms| {
            let row = Self::sum_branch_terms(terms);
            jacobian[index] = row.d_internal;
            external_partials[index] = row.d_external;
            source[index] = source_for_branch(row);
        });
        (jacobian, external_partials, source)
    }

    /// Physical terms of each active private KCL/heat row. Keeping the terms
    /// separate lets periodic convergence retain their scale after cancellation.
    /// The independent RBI port may own its two linear electrical incidences;
    /// its current still participates in the native heat equation.
    pub(in crate::device::semiconductor::bjt) fn visit_internal_kcl_terms(
        &self,
        state: IntrinsicTerminalState,
        eval: EvaluatedBjtState,
        external: [Value; EXTERNAL_DIM],
        external_rbi_port: bool,
        mut visit: impl FnMut(usize, &[BranchLinearization]),
    ) {
        let [collector, base, emitter] = self.intrinsic_terminal_branches(&eval);
        let negative = |branch| Self::scale_branch(branch, -1.0);
        let rbi = if external_rbi_port {
            BranchLinearization::default()
        } else {
            eval.irbi
        };
        if Self::series_active(self.rcx) {
            visit(
                IDX_VCX,
                &[
                    eval.ircx,
                    self.parasitic_base_collector_branch(&eval),
                    eval.igcx,
                    negative(if Self::series_active(self.rci) {
                        eval.irci
                    } else {
                        collector
                    }),
                ],
            );
        }
        if Self::series_active(self.rci) {
            visit(IDX_VCI, &[eval.irci, negative(collector)]);
        }
        if Self::series_active(self.rbx) {
            visit(
                IDX_VBX,
                &[
                    eval.irbx,
                    negative(if Self::series_active(self.rbi) {
                        rbi
                    } else {
                        base
                    }),
                    negative(eval.ibex),
                    negative(eval.ibep),
                    negative(eval.iccp),
                    negative(eval.igcx),
                ],
            );
        }
        if Self::series_active(self.rbi) {
            visit(IDX_VBI, &[rbi, negative(base)]);
        }
        if Self::series_active(self.re) {
            visit(IDX_VEI, &[eval.ire, eval.ibex, negative(emitter)]);
        }
        if self.vbic_solves_vbp() {
            visit(IDX_VBP, &[eval.ibep, eval.ibcp, negative(eval.irbp)]);
        }
        if self.has_substrate_resistance() {
            visit(IDX_VSI, &[eval.irs, eval.iccp, negative(eval.ibcp)]);
        }
        if self.thermal_model_enabled() {
            let power = self.thermal_power_branch(
                eval,
                external,
                [
                    state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp, state.vsi,
                    state.vrth,
                ],
            );
            visit(
                IDX_VRTH,
                &[self.thermal_sink_branch(state.vrth), negative(power)],
            );
        }
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
            Some(if self.thermal_model_enabled() {
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
            BjtNodeVoltages {
                vc,
                vb,
                ve,
                vs,
                vcx: state.vcx,
                vci: state.vci,
                vbx: state.vbx,
                vbi: state.vbi,
                vei: state.vei,
                vbp: state.vbp,
                vsi: state.vsi,
            },
            state.vrth,
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
            if let Some(solution) = crate::numerics::solve_small_dense(g_ii, &rhs, INTERNAL_DIM) {
                for idx in 0..INTERNAL_DIM {
                    sensitivities[idx][external] = solution[idx];
                }
            } else {
                // A failed private solve is not a zero terminal derivative.
                // Nonfinite reduction evidence must reach the engine checks.
                for row in &mut sensitivities {
                    row[external] = Value::NAN;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn thermal_rebalance_preserves_equilibrium_across_instance_and_rise_scales() {
        for level in [4.0, 9.0, 11.0, 12.0] {
            for m in [1.0, 1e-12, 1e-20, 1e-100, 1e-200] {
                let mut bjt = Bjt::new_npn("q".into(), 1, 2, 0)
                    .with_params(
                        &[
                            ("LEVEL".into(), level),
                            ("RTH".into(), 1000.0),
                            ("SELFT".into(), 1.0),
                        ]
                        .into_iter()
                        .collect(),
                    )
                    .with_instance_params(&[("M".into(), m)]);
                bjt.set_junction_gmin(0.0);
                assert!(bjt.thermal_model_enabled());
                for rise in [20.0, 1e-14, 800.0] {
                    // At zero electrical bias there is no dissipated power.
                    // M scales both thermal residual and derivative, while
                    // the equilibrium temperature rise remains exactly zero.
                    // An 800 K seed needs the last of the eight allowed
                    // limited steps; do not return the preceding iterate.
                    let mut state = [0.0; INTERNAL_DIM];
                    state[IDX_VRTH] = rise;
                    let residual = bjt
                        .intrinsic_state_residual_jacobian(0.0, 0.0, 0.0, 0.0, state)
                        .0[IDX_VRTH];
                    assert!(residual.is_finite() && residual > 0.0);
                    let actual = bjt.rebalance_intrinsic_thermal_state(0.0, 0.0, 0.0, 0.0, state);
                    assert!(
                        actual[IDX_VRTH].abs() <= rise * 4e-15,
                        "LEVEL={level} M={m:e} initial rise={rise:e}: {}",
                        actual[IDX_VRTH],
                    );
                    assert_eq!(&actual[..IDX_VRTH], &state[..IDX_VRTH]);

                    // A cached warm state must retain the correction when
                    // the coupled solver compares its refined candidate.
                    bjt.vrth = rise;
                    bjt.reduced_linearization_cache_valid.set(true);
                    let solved = bjt.solve_intrinsic_terminal_state(0.0, 0.0, 0.0, 0.0);
                    assert!(
                        solved.vrth.abs() <= rise * 4e-15,
                        "coupled LEVEL={level} M={m:e} initial rise={rise:e}: {}",
                        solved.vrth,
                    );
                }
            }
        }
    }

    #[test]
    fn private_intrinsic_identity_rows_measure_displaced_wire_nodes() {
        let bjt = Bjt::new_npn("q".into(), 1, 2, 0)
            .with_params(&[("IS".into(), 0.0)].into_iter().collect());
        let state = [1.0, 2.0, 3.0, 5.0, 7.0, 11.0, 13.0, 17.0];
        let (residual, jacobian) = bjt.intrinsic_state_residual_jacobian(1.0, 2.0, 0.0, 0.0, state);
        assert_eq!(residual, [0.0, 1.0, 1.0, 2.0, 7.0, 10.0, 13.0, 17.0]);
        for col in 0..INTERNAL_DIM {
            let mut perturbed = state;
            perturbed[col] += 0.25;
            let shifted = bjt
                .intrinsic_state_residual_jacobian(1.0, 2.0, 0.0, 0.0, perturbed)
                .0;
            for row in 0..INTERNAL_DIM {
                assert_eq!((shifted[row] - residual[row]) / 0.25, jacobian[row][col]);
            }
        }
    }

    #[test]
    fn private_vbic_cached_bias_preserves_instance_scaling() {
        for level in [4.0, 9.0, 11.0, 12.0] {
            let make = |m| {
                let mut bjt = Bjt::new_npn("q".into(), 1, 2, 0)
                    .with_params(
                        &[
                            ("LEVEL".into(), level),
                            ("RTH".into(), 1000.0),
                            ("SELFT".into(), 1.0),
                            ("IS".into(), 1e-16),
                        ]
                        .into_iter()
                        .collect(),
                    )
                    .with_instance_params(&[("M".into(), m)]);
                bjt.set_junction_gmin(0.0);
                bjt
            };
            for m in [1e-20, 1e-100, 1e-200] {
                let mut reference = make(1.0);
                let mut scaled = make(m);
                for base in [0.7, 0.0, 0.65] {
                    reference.update(&[1.0, base]);
                    scaled.update(&[1.0, base]);
                    let (rc, rb, re) = reference.operating_point_currents();
                    let (sc, sb, se) = scaled.operating_point_currents();
                    if base == 0.0 {
                        assert!(scaled.vrth.abs() < 2e-10, "scaled off rise {}", scaled.vrth);
                        assert!(
                            reference.vrth.abs() < 2e-10,
                            "reference off rise {}",
                            reference.vrth
                        );
                        for current in [rc, rb, re, sc / m, sb / m, se / m] {
                            assert!(current.abs() < 2e-13, "off current {current:e}");
                        }
                    }
                    for (actual, expected) in [(sc, rc), (sb, rb), (se, re)] {
                        // The 0.1 ohm series branches lose currents below
                        // voltage resolution; allow that absolute floor.
                        assert!(
                            (actual / m - expected).abs() <= 2e-13 + 2e-10 * expected.abs(),
                            "LEVEL={level} M={m:e} VB={base}: {} vs {expected}",
                            actual / m,
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn thermal_rebalance_step_limit_preserves_finite_range() {
        for level in [4.0, 9.0, 11.0, 12.0] {
            let mut bjt = Bjt::new_npn("q".into(), 1, 2, 0).with_params(
                &[("LEVEL".into(), level), ("TMAXCLIP".into(), 100.0)]
                    .into_iter()
                    .collect(),
            );
            assert_eq!(bjt.thermal_rebalance_step_limit(1e308), 5e307);
            if bjt.vbic_13 {
                bjt.set_temperature(1e308);
                assert_eq!(bjt.thermal_rebalance_step_limit(1e308), 1e308);
                assert_eq!(bjt.thermal_rebalance_step_limit(-1e308), 1e308);
            }
        }
    }

    #[test]
    fn private_intrinsic_newton_preserves_voltage_across_instance_scales() {
        for xyce in [false, true] {
            for p in [1.0, -1.0] {
                let make = |m| {
                    let mut bjt = if p > 0.0 {
                        Bjt::new_npn("q".into(), 1, 2, 0)
                    } else {
                        Bjt::new_pnp("q".into(), 1, 2, 0)
                    }
                    .with_params(
                        &[
                            ("IS".into(), 1e-14),
                            ("BF".into(), 100.0),
                            ("RB".into(), 5e3),
                            ("RBM".into(), 1e3),
                        ]
                        .into_iter()
                        .collect(),
                    )
                    .with_instance_params(&[("M".into(), m)]);
                    bjt.set_xyce_compatibility(xyce);
                    bjt.set_junction_gmin(0.0);
                    bjt
                };
                let reference = make(1.0).solve_intrinsic_terminal_state(p, 0.7 * p, 0.0, 0.0);
                for m in [1e-20, 1e-100, 1e-200] {
                    let bjt = make(m);
                    let actual = bjt.solve_intrinsic_terminal_state(p, 0.7 * p, 0.0, 0.0);
                    assert!(
                        (actual.vbi - reference.vbi).abs() < 2e-12,
                        "xyce={xyce} p={p} M={m:e}: vbi={} vs {}",
                        actual.vbi,
                        reference.vbi
                    );
                }
            }
        }
    }

    #[test]
    fn intrinsic_residual_norm_cannot_accept_an_invalid_equation_as_zero() {
        for lane in 0..INTERNAL_DIM {
            let mut residual = [0.0; INTERNAL_DIM];
            residual[lane] = Value::NAN;
            assert_eq!(
                Bjt::intrinsic_state_residual_norm(&residual),
                Value::INFINITY
            );
        }
    }
}
