//! Intrinsic solve, thermal residual, convergence, and terminal stamping helpers.

use super::*;

impl Bjt {
    pub(super) fn solve_small_dense_system<const N: usize>(
        matrix: &[[Value; N]; N],
        rhs: &[Value; N],
        dim: usize,
    ) -> Option<[Value; N]> {
        if dim == 0 {
            return Some([0.0; N]);
        }

        let mut a = *matrix;
        let mut b = *rhs;

        for pivot in 0..dim {
            let mut best = pivot;
            let mut best_abs = a[pivot][pivot].abs();
            for row in (pivot + 1)..dim {
                let value = a[row][pivot].abs();
                if value > best_abs {
                    best = row;
                    best_abs = value;
                }
            }
            if best_abs < 1e-18 {
                return None;
            }
            if best != pivot {
                a.swap(pivot, best);
                b.swap(pivot, best);
            }

            let pivot_value = a[pivot][pivot];
            for row in (pivot + 1)..dim {
                let factor = a[row][pivot] / pivot_value;
                a[row][pivot] = 0.0;
                for col in (pivot + 1)..dim {
                    a[row][col] -= factor * a[pivot][col];
                }
                b[row] -= factor * b[pivot];
            }
        }

        let mut x = [0.0; N];
        for row in (0..dim).rev() {
            let mut sum = b[row];
            for col in (row + 1)..dim {
                sum -= a[row][col] * x[col];
            }
            let diag = a[row][row];
            if diag.abs() < 1e-18 {
                return None;
            }
            x[row] = sum / diag;
        }

        Some(x)
    }

    pub(super) fn solve_intrinsic_terminal_state(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
    ) -> IntrinsicTerminalState {
        let has_rcx = Self::series_active(self.rcx);
        let has_rci = Self::series_active(self.rci);
        let has_rbx = Self::series_active(self.rbx);
        let has_rbi = Self::series_active(self.rbi);
        let has_re = Self::series_active(self.re);
        let has_rs = Self::series_active(self.rs);
        let has_self_heat = self.self_heating_enabled();
        let reuse_previous_state = self.reduced_linearization_cache_valid.get();
        let solve_vbp = self.vbic_solves_vbp();

        let mut vcx = if reuse_previous_state {
            self.vcx
        } else if has_rcx {
            vc - self.ic * self.rcx.max(0.0)
        } else {
            vc
        };
        let mut vci = if reuse_previous_state {
            self.vci
        } else if has_rci {
            vcx - self.ic * self.rci.max(0.0)
        } else {
            vcx
        };
        let mut vbx = if reuse_previous_state {
            self.vbx
        } else if has_rbx {
            vb - self.ib * self.rbx.max(0.0)
        } else {
            vb
        };
        let mut vbi = if reuse_previous_state {
            self.vbi
        } else if has_rbi {
            vbx - self.ib * self.rbi.max(0.0)
        } else {
            vbx
        };
        let mut vei = if reuse_previous_state {
            self.vei
        } else if has_re {
            ve - self.ie * self.re.max(0.0)
        } else {
            ve
        };
        let mut vsi = if reuse_previous_state {
            self.vsi
        } else if has_rs {
            vs - self.isub * self.rs.max(0.0)
        } else {
            vs
        };
        // The external-collector voltage seeds vbp whether or not the
        // parasitic node is solved this pass.
        let mut vbp = if reuse_previous_state { self.vbp } else { vcx };
        let mut vrth = if reuse_previous_state {
            self.vrth
        } else if has_self_heat {
            let seed_internal = [vcx, vci, vbx, vbi, vei, vbp, vsi, 0.0];
            let first_guess =
                self.branchwise_thermal_rise_guess_from_internal(vc, vb, ve, vs, seed_internal);
            let second_guess = self.branchwise_thermal_rise_guess_from_internal(
                vc,
                vb,
                ve,
                vs,
                [vcx, vci, vbx, vbi, vei, vbp, vsi, first_guess],
            );
            if second_guess.is_finite() {
                second_guess
            } else if first_guess.is_finite() {
                first_guess
            } else {
                (vc * self.ic + vb * self.ib + ve * self.ie + vs * self.isub)
                    / self.thermal_conductance().max(1e-18)
            }
        } else {
            0.0
        };
        if !has_self_heat {
            vrth = 0.0;
        }

        let state =
            if !reuse_previous_state && self.charge_model == BjtChargeModel::LegacyGummelPoon {
                let mut seed = self.legacy_startup_intrinsic_state_seed([vc, vb, ve, vs]);
                if has_self_heat {
                    seed[IDX_VRTH] = vrth;
                }
                seed
            } else {
                [vcx, vci, vbx, vbi, vei, vbp, vsi, vrth]
            };
        let previous_external = [self.vc_ext, self.vb_ext, self.ve_ext, self.vs_ext];
        let predicted_state = if reuse_previous_state {
            self.predict_intrinsic_state_from_previous_external_bias(
                previous_external,
                state,
                [vc, vb, ve, vs],
            )
        } else {
            None
        };
        let solve_from_seed = |seed: [Value; INTERNAL_DIM]| {
            if has_self_heat && !reuse_previous_state {
                self.solve_intrinsic_state_with_self_heating_continuation(vc, vb, ve, vs, seed)
            } else {
                self.solve_intrinsic_state_from_seed(vc, vb, ve, vs, seed)
            }
        };

        let (mut best_state, mut best_residual_norm) =
            solve_from_seed(predicted_state.unwrap_or(state));
        if self.charge_model == BjtChargeModel::Vbic
            && reuse_previous_state
            && self.vbic_max_local_branch_delta(best_state, predicted_state.unwrap_or(state)) > 0.1
        {
            if let Some((continued_state, continued_residual_norm)) = self
                .solve_intrinsic_state_with_external_continuation(
                    previous_external,
                    state,
                    [vc, vb, ve, vs],
                )
            {
                if continued_residual_norm + 1e-15 < best_residual_norm
                    || self.vbic_max_local_branch_delta(
                        continued_state,
                        predicted_state.unwrap_or(state),
                    ) <= 0.1
                {
                    best_state = continued_state;
                    best_residual_norm = continued_residual_norm;
                }
            }
        }
        if predicted_state.is_some()
            && best_residual_norm > 1e-9
            && self.vbic_max_local_branch_delta(best_state, predicted_state.unwrap_or(state)) > 0.1
        {
            let (fallback_state, fallback_residual_norm) = solve_from_seed(state);
            if fallback_residual_norm + 1e-15 < best_residual_norm {
                best_state = fallback_state;
                best_residual_norm = fallback_residual_norm;
            }
        }
        if !reuse_previous_state
            && self.charge_model == BjtChargeModel::LegacyGummelPoon
            && self.has_intrinsic_state_unknowns()
            && best_residual_norm > 1e-9
            && let Some(anchor_external) =
                self.initial_forward_bias_anchor_external([vc, vb, ve, vs])
            && let Some((continued_state, continued_residual_norm)) = self
                .solve_intrinsic_state_from_forward_bias_anchor(anchor_external, [vc, vb, ve, vs])
            && continued_residual_norm + 1e-15 < best_residual_norm
        {
            best_state = continued_state;
            best_residual_norm = continued_residual_norm;
        }
        if has_self_heat {
            for _ in 0..4 {
                let rebalanced_state =
                    self.rebalance_intrinsic_thermal_state(vc, vb, ve, vs, best_state);
                let (refined_state, refined_residual_norm) =
                    self.solve_intrinsic_state_from_seed(vc, vb, ve, vs, rebalanced_state);
                if refined_residual_norm + 1e-15 < best_residual_norm {
                    best_state = refined_state;
                    best_residual_norm = refined_residual_norm;
                    continue;
                }
                break;
            }
        }

        [vcx, vci, vbx, vbi, vei, vbp, vsi, vrth] = best_state;

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

    pub(super) fn internal_voltage_sensitivities(
        &self,
        state: IntrinsicTerminalState,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
    ) -> [[Value; EXTERNAL_DIM]; INTERNAL_DIM] {
        let has_rcx = Self::series_active(self.rcx);
        let has_rci = Self::series_active(self.rci);
        let has_rbx = Self::series_active(self.rbx);
        let has_rbi = Self::series_active(self.rbi);
        let has_re = Self::series_active(self.re);
        let has_rs = Self::series_active(self.rs);
        let has_self_heat = self.self_heating_enabled();
        let solve_vbp = self.vbic_solves_vbp();

        let eval = self.evaluate_state(
            vc, vb, ve, vs, state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp,
            state.vsi, state.vrth,
        );
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

        if has_rcx {
            let row = Self::sub_branches(
                Self::add_branches(eval.ircx, eval.irbp),
                if has_rci {
                    eval.irci
                } else {
                    collector_internal
                },
            );
            for idx in 0..INTERNAL_DIM {
                jacobian[IDX_VCX][idx] = row.d_internal[idx];
            }
            external_partials[IDX_VCX] = row.d_external;
        } else {
            jacobian[IDX_VCX][IDX_VCX] = 1.0;
            external_partials[IDX_VCX][EXT_C] = -1.0;
        }

        if has_rci {
            let row = Self::sub_branches(eval.irci, collector_internal);
            for idx in 0..INTERNAL_DIM {
                jacobian[IDX_VCI][idx] = row.d_internal[idx];
            }
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
            for idx in 0..INTERNAL_DIM {
                jacobian[IDX_VBX][idx] = row.d_internal[idx];
            }
            external_partials[IDX_VBX] = row.d_external;
        } else {
            jacobian[IDX_VBX][IDX_VBX] = 1.0;
            external_partials[IDX_VBX][EXT_B] = -1.0;
        }

        if has_rbi {
            let row = Self::sub_branches(eval.irbi, base_internal);
            for idx in 0..INTERNAL_DIM {
                jacobian[IDX_VBI][idx] = row.d_internal[idx];
            }
        } else {
            jacobian[IDX_VBI][IDX_VBI] = 1.0;
            jacobian[IDX_VBI][IDX_VBX] = -1.0;
        }

        if has_re {
            let row = Self::sub_branches(eval.ire, emitter_internal);
            for idx in 0..INTERNAL_DIM {
                jacobian[IDX_VEI][idx] = row.d_internal[idx];
            }
            external_partials[IDX_VEI] = row.d_external;
        } else {
            jacobian[IDX_VEI][IDX_VEI] = 1.0;
            external_partials[IDX_VEI][EXT_E] = -1.0;
        }

        if solve_vbp {
            let row = Self::sub_branches(Self::add_branches(eval.ibep, eval.ibcp), eval.irbp);
            for idx in 0..INTERNAL_DIM {
                jacobian[IDX_VBP][idx] = row.d_internal[idx];
            }
        } else {
            jacobian[IDX_VBP][IDX_VBP] = 1.0;
            jacobian[IDX_VBP][IDX_VCX] = -1.0;
        }

        if has_rs {
            let row = Self::sub_branches(Self::add_branches(eval.irs, eval.iccp), eval.ibcp);
            for idx in 0..INTERNAL_DIM {
                jacobian[IDX_VSI][idx] = row.d_internal[idx];
            }
            external_partials[IDX_VSI] = row.d_external;
        } else {
            jacobian[IDX_VSI][IDX_VSI] = 1.0;
            external_partials[IDX_VSI][EXT_S] = -1.0;
        }

        if has_self_heat {
            let row = Self::sub_branches(thermal_sink, thermal_power);
            for idx in 0..INTERNAL_DIM {
                jacobian[IDX_VRTH][idx] = row.d_internal[idx];
            }
            external_partials[IDX_VRTH] = row.d_external;
        } else {
            jacobian[IDX_VRTH][IDX_VRTH] = 1.0;
        }

        let mut sensitivities = [[0.0; EXTERNAL_DIM]; INTERNAL_DIM];
        for external in 0..EXTERNAL_DIM {
            let rhs = external_partials.map(|partials| -partials[external]);
            if let Some(solution) = Self::solve_small_dense_system(&jacobian, &rhs, INTERNAL_DIM) {
                for idx in 0..INTERNAL_DIM {
                    sensitivities[idx][external] = solution[idx];
                }
            }
        }

        sensitivities
    }

    pub(super) fn external_terminal_branches(
        &self,
        eval: EvaluatedBjtState,
    ) -> [BranchLinearization; EXTERNAL_DIM] {
        let (collector_d, base_d, emitter_d) = self.intrinsic_terminal_derivatives(eval.linearized);
        let collector_internal = Self::branch_from_internal(eval.linearized.ic, collector_d);
        let base_internal = Self::branch_from_internal(eval.linearized.ib, base_d);
        let emitter_internal =
            Self::branch_from_internal(-(eval.linearized.ic + eval.linearized.ib), emitter_d);

        let collector = if Self::series_active(self.rcx) {
            eval.ircx
        } else {
            Self::sub_branches(
                if Self::series_active(self.rci) {
                    eval.irci
                } else {
                    collector_internal
                },
                eval.irbp,
            )
        };
        let base = if Self::series_active(self.rbx) {
            eval.irbx
        } else {
            Self::add_branches(
                Self::add_branches(
                    if Self::series_active(self.rbi) {
                        eval.irbi
                    } else {
                        base_internal
                    },
                    eval.ibep,
                ),
                eval.iccp,
            )
        };
        let emitter = if Self::series_active(self.re) {
            eval.ire
        } else {
            emitter_internal
        };
        let substrate = if Self::series_active(self.rs) {
            eval.irs
        } else {
            Self::sub_branches(eval.ibcp, eval.iccp)
        };

        [collector, base, emitter, substrate]
    }

    pub(super) fn thermal_sink_branch(&self, vrth: Value) -> BranchLinearization {
        let mut branch = BranchLinearization::default();
        let gth = self.thermal_conductance();
        if gth <= 0.0 {
            return branch;
        }
        branch.current = gth * vrth;
        branch.d_internal[IDX_VRTH] = gth;
        branch
    }

    pub(super) fn thermal_power_branch(
        &self,
        eval: EvaluatedBjtState,
        external: [Value; EXTERNAL_DIM],
        internal: [Value; INTERNAL_DIM],
    ) -> BranchLinearization {
        if !self.self_heating_enabled() {
            return BranchLinearization::default();
        }

        let [vcx, vci, vbx, vbi, vei, vbp, vsi, _vrth] = internal;
        let [vc, vb, ve, vs] = external;
        let zero_external = [0.0; EXTERNAL_DIM];
        let mut power = BranchLinearization::default();

        let add_power = |acc: &mut BranchLinearization,
                         current: BranchLinearization,
                         voltage: Value,
                         d_voltage_internal: [Value; INTERNAL_DIM],
                         d_voltage_external: [Value; EXTERNAL_DIM]| {
            *acc = Self::add_branches(
                *acc,
                Self::power_from_branch(current, voltage, d_voltage_internal, d_voltage_external),
            );
        };

        add_power(
            &mut power,
            eval.ibe,
            vbi - vei,
            [0.0, 0.0, 0.0, 1.0, -1.0, 0.0, 0.0, 0.0],
            zero_external,
        );
        add_power(
            &mut power,
            eval.ibc,
            vbi - vci,
            [0.0, -1.0, 0.0, 1.0, 0.0, 0.0, 0.0, 0.0],
            zero_external,
        );
        add_power(
            &mut power,
            eval.iciei,
            vci - vei,
            [0.0, 1.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0],
            zero_external,
        );
        add_power(
            &mut power,
            eval.ibep,
            vbx - vbp,
            [0.0, 0.0, 1.0, 0.0, 0.0, -1.0, 0.0, 0.0],
            zero_external,
        );
        add_power(
            &mut power,
            eval.ibcp,
            vsi - vbp,
            [0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 1.0, 0.0],
            zero_external,
        );
        add_power(
            &mut power,
            eval.iccp,
            vbx - vsi,
            [0.0, 0.0, 1.0, 0.0, 0.0, 0.0, -1.0, 0.0],
            zero_external,
        );
        add_power(
            &mut power,
            eval.ircx,
            vc - vcx,
            [-1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [1.0, 0.0, 0.0, 0.0],
        );
        add_power(
            &mut power,
            eval.irci,
            vcx - vci,
            [1.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            zero_external,
        );
        add_power(
            &mut power,
            eval.irbx,
            vb - vbx,
            [0.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
        );
        add_power(
            &mut power,
            eval.irbi,
            vbx - vbi,
            [0.0, 0.0, 1.0, -1.0, 0.0, 0.0, 0.0, 0.0],
            zero_external,
        );
        add_power(
            &mut power,
            eval.ire,
            ve - vei,
            [0.0, 0.0, 0.0, 0.0, -1.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
        );
        add_power(
            &mut power,
            eval.irbp,
            vbp - vcx,
            [-1.0, 0.0, 0.0, 0.0, 0.0, 1.0, 0.0, 0.0],
            zero_external,
        );
        add_power(
            &mut power,
            eval.irs,
            vs - vsi,
            [0.0, 0.0, 0.0, 0.0, 0.0, 0.0, -1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        );

        power
    }

    #[inline]
    pub(super) fn branchwise_thermal_rise_guess_from_internal(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        internal: [Value; INTERNAL_DIM],
    ) -> Value {
        let gth = self.thermal_conductance();
        if gth <= 0.0 {
            return 0.0;
        }

        let state = self.intrinsic_state_from_internal_vector(internal);
        let eval = self.evaluate_state(
            vc, vb, ve, vs, state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp,
            state.vsi, state.vrth,
        );
        self.thermal_power_branch(eval, [vc, vb, ve, vs], internal)
            .current
            / gth
    }

    #[inline]
    pub(crate) fn minimum_thermal_rise(&self) -> Value {
        1.0 - self.requested_temperature()
    }

    pub(crate) fn vbic_dynamic_thermal_residual_and_derivative(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        internal: [Value; BJT_INTERNAL_STATE_DIM],
    ) -> (Value, Value) {
        let static_internal = [
            internal[IDX_VCX],
            internal[IDX_VCI],
            internal[IDX_VBX],
            internal[IDX_VBI],
            internal[IDX_VEI],
            internal[IDX_VBP],
            internal[IDX_VSI],
            internal[IDX_VRTH],
        ];
        let state = self.intrinsic_state_from_internal_vector(static_internal);
        let eval = self.evaluate_state(
            vc, vb, ve, vs, state.vcx, state.vci, state.vbx, state.vbi, state.vei, state.vbp,
            state.vsi, state.vrth,
        );
        let static_row = Self::sub_branches(
            self.thermal_sink_branch(state.vrth),
            self.thermal_power_branch(eval, [vc, vb, ve, vs], static_internal),
        );
        let mut residual = static_row.current;
        let mut derivative = static_row.d_internal[IDX_VRTH];

        let reduction = self.dynamic_reduction_for_internal_state(vc, vb, ve, vs, internal);
        let thermal_branch = self.vbic_delay_static_thermal_branch(&reduction);
        if thermal_branch.is_active() {
            residual += thermal_branch.current;
            derivative += thermal_branch.d_internal[IDX_VRTH];
        }

        (residual, derivative)
    }

    #[inline]
    pub(super) fn internal_state_vector(&self) -> [Value; INTERNAL_DIM] {
        [
            self.vcx, self.vci, self.vbx, self.vbi, self.vei, self.vbp, self.vsi, self.vrth,
        ]
    }

    #[inline]
    pub(super) fn previous_internal_state_vector(&self) -> [Value; INTERNAL_DIM] {
        [
            self.vcx_prev,
            self.vci_prev,
            self.vbx_prev,
            self.vbi_prev,
            self.vei_prev,
            self.vbp_prev,
            self.vsi_prev,
            self.vrth_prev,
        ]
    }

    #[inline]
    pub(super) fn vbic_convergence_voltage_vector_for_state(
        &self,
        internal: [Value; INTERNAL_DIM],
    ) -> [Value; 9] {
        let p = self.polarity();
        let [vcx, vci, vbx, vbi, vei, vbp, vsi, _vrth] = internal;
        [
            p * (vbi - vei),
            p * (vbx - vei),
            p * (vbi - vci),
            p * (vbi - vcx),
            p * (vbx - vbp),
            p * (vcx - vci),
            p * (vbx - vbi),
            p * (vbp - vcx),
            p * (vsi - vbp),
        ]
    }

    pub(super) fn vbic_convergence_branches_for_state(
        &self,
        internal: [Value; INTERNAL_DIM],
    ) -> [BranchLinearization; Self::VBIC_CONVERGENCE_BRANCH_COUNT] {
        let [vcx, vci, vbx, vbi, vei, vbp, vsi, vrth] = internal;
        let eval = self.evaluate_state(0.0, 0.0, 0.0, 0.0, vcx, vci, vbx, vbi, vei, vbp, vsi, vrth);
        [
            eval.ibe, eval.ibep, eval.iciei, eval.ibc, eval.irci, eval.irbi, eval.irbp, eval.ibcp,
            eval.iccp,
        ]
    }

    pub(crate) fn vbic_transient_convergence_state_for_snapshot(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
        snapshot: &BjtChargeSnapshot,
    ) -> VbicTransientConvergenceState {
        let [vcx, vci, vbx, vbi, vei, vbp, vsi, vrth, _vxf1, _vxf2] =
            snapshot.reduction.internal_voltages;
        let eval = self.evaluate_state(vc, vb, ve, vs, vcx, vci, vbx, vbi, vei, vbp, vsi, vrth);
        let delay_branches = self.vbic_delay_static_branches(&snapshot.reduction);

        let mut currents = [0.0; VBIC_TRANSIENT_CONVERGENCE_BRANCH_COUNT];
        let mut d_currents_d_internal =
            [[0.0; BJT_INTERNAL_STATE_DIM]; VBIC_TRANSIENT_CONVERGENCE_BRANCH_COUNT];

        let static_branches = [
            eval.ibe, eval.ibep, eval.iciei, eval.ibc, eval.irci, eval.irbi, eval.irbp, eval.ibcp,
            eval.iccp,
        ];
        for (branch_idx, branch) in static_branches.iter().enumerate() {
            currents[branch_idx] = branch.current;
            d_currents_d_internal[branch_idx][..INTERNAL_DIM].copy_from_slice(&branch.d_internal);
        }

        if self.uses_vbic_dynamic_charges() && self.td > 0.0 {
            currents[VBIC_TRANSIENT_CONVERGENCE_ICIEI_INDEX] += delay_branches[0].current;
            for idx in 0..BJT_INTERNAL_STATE_DIM {
                d_currents_d_internal[VBIC_TRANSIENT_CONVERGENCE_ICIEI_INDEX][idx] +=
                    delay_branches[0].d_internal[idx];
            }
        }

        let p = self.polarity();
        let voltages = [
            p * (vbi - vei),
            p * (vbx - vei),
            p * (vbi - vci),
            p * (vbi - vcx),
            p * (vbx - vbp),
            p * (vcx - vci),
            p * (vbx - vbi),
            p * (vbp - vcx),
            p * (vsi - vbp),
        ];

        VbicTransientConvergenceState {
            voltages,
            currents,
            d_currents_d_internal,
        }
    }

    #[inline]
    pub(super) fn vbic_predicted_branch_current(
        previous: &BranchLinearization,
        delta_internal: &[Value; INTERNAL_DIM],
    ) -> Value {
        previous.current
            + previous
                .d_internal
                .iter()
                .zip(delta_internal.iter())
                .enumerate()
                .filter(|(idx, _)| *idx != IDX_VRTH)
                .map(|(_, (derivative, delta))| derivative * delta)
                .sum::<Value>()
    }

    pub(super) fn vbic_is_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        let reltol = criteria.relative_tolerance();
        let voltage_tol = criteria.voltage_tolerance();
        let current_tol = criteria.current_tolerance();
        let current_state = self.internal_state_vector();
        let previous_state = self.previous_internal_state_vector();
        let mut delta_internal = [0.0; INTERNAL_DIM];
        for idx in 0..INTERNAL_DIM {
            delta_internal[idx] = current_state[idx] - previous_state[idx];
        }

        let current_voltages = self.vbic_convergence_voltage_vector_for_state(current_state);
        let previous_voltages = self.vbic_convergence_voltage_vector_for_state(previous_state);
        let previous_branches = self.vbic_convergence_branches_for_state(previous_state);
        let current_branches = self.vbic_convergence_branches_for_state(current_state);
        let voltages_converged =
            current_voltages
                .iter()
                .zip(previous_voltages.iter())
                .all(|(current, previous)| {
                    let diff = (current - previous).abs();
                    let tol = reltol * current.abs().max(previous.abs()) + voltage_tol;
                    diff <= tol
                });

        let currents_converged = current_branches
            .iter()
            .zip(previous_branches.iter())
            .enumerate()
            .all(|(branch_idx, (current, previous))| {
                if self.uses_vbic_dynamic_charges()
                    && branch_idx == VBIC_TRANSIENT_CONVERGENCE_ICIEI_INDEX
                {
                    return true;
                }
                let predicted = Self::vbic_predicted_branch_current(previous, &delta_internal);
                let actual = current.current;
                let tol = reltol * predicted.abs().max(actual.abs()) + current_tol;
                (predicted - actual).abs() <= tol
            });

        voltages_converged && currents_converged
    }

    pub(super) fn legacy_bjt_is_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        if !self.previous_reduced_linearization_valid {
            return false;
        }

        let reltol = criteria.relative_tolerance();
        let current_tol = criteria.current_tolerance();
        let p = self.polarity();
        let previous = self.intrinsic_linearization_prev;
        let current = self.intrinsic_linearization;
        let previous_vbe = p * self.vbe_prev;
        let previous_vbc = p * self.vbc_prev;
        let current_vbe = p * self.vbe;
        let current_vbc = p * self.vbc;

        let delvbe = current_vbe - previous_vbe;
        let delvbc = current_vbc - previous_vbc;
        let previous_cc = p * previous.ic;
        let current_cc = p * current.ic;
        let previous_cb = p * previous.ib;
        let current_cb = p * current.ib;
        let cchat = previous_cc + previous.dic_dvbe * delvbe + previous.dic_dvbc * delvbc;
        let cbhat = previous_cb + previous.dib_dvbe * delvbe + previous.dib_dvbc * delvbc;

        let values = [
            previous_vbe,
            previous_vbc,
            current_vbe,
            current_vbc,
            delvbe,
            delvbc,
            previous_cc,
            current_cc,
            previous_cb,
            current_cb,
            cchat,
            cbhat,
            previous.dic_dvbe,
            previous.dic_dvbc,
            previous.dib_dvbe,
            previous.dib_dvbc,
        ];
        if !values.iter().all(|value| value.is_finite()) {
            return false;
        }

        let collector_tol = reltol * cchat.abs().max(current_cc.abs()) + current_tol;
        if (cchat - current_cc).abs() > collector_tol {
            return false;
        }

        let base_tol = reltol * cbhat.abs().max(current_cb.abs()) + current_tol;
        (cbhat - current_cb).abs() <= base_tol
    }

    pub(super) fn small_signal_row_coefficients(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
    ) -> BjtConductanceMatrix {
        self.reduced_linearization(vc, vb, ve, vs).g_reduced
    }

    pub(crate) fn stamped_reduced_external_system(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
    ) -> (BjtConductanceMatrix, [Value; EXTERNAL_DIM]) {
        let rows = self.small_signal_row_coefficients(vc, vb, ve, vs);
        let biases = [vc, vb, ve, vs];
        let currents = self.external_terminal_currents_at_bias(vc, vb, ve, vs);
        let mut rhs = [0.0; EXTERNAL_DIM];
        for row in 0..EXTERNAL_DIM {
            rhs[row] = -currents[row];
            for col in 0..EXTERNAL_DIM {
                rhs[row] += rows[row][col] * biases[col];
            }
        }
        (rows, rhs)
    }

    pub(crate) fn stamp_small_signal_ac(
        &self,
        voltages: &[Value],
        matrix: &mut impl MatrixStamper,
    ) {
        if self.vbic_mna_promoted() {
            // The promoted static system (terminal rows, internal KCL rows,
            // and excess-phase algebraic rows) is the small-signal real part;
            // AC stampers ignore the rhs source terms.
            self.stamp_vbic_mna(matrix);
            return;
        }
        let [vc, vb, ve, vs] = self.external_terminal_voltages(voltages);
        let rows = self.small_signal_row_coefficients(vc, vb, ve, vs);
        let nodes = self.external_terminal_nodes();
        for row_idx in 0..EXTERNAL_DIM {
            for col_idx in 0..EXTERNAL_DIM {
                matrix.stamp(nodes[row_idx], nodes[col_idx], rows[row_idx][col_idx]);
            }
        }
    }

    /// Get base-emitter junction conductance
    /// Includes minimum conductance floor for numerical stability
    pub(super) fn gbe(&self, vbe: Value) -> Value {
        let vp = self.polarity() * vbe;
        let g = self.diode_conductance_with_is(self.ibei, vp, self.nei)
            + self.diode_conductance_with_is(self.iben, vp, self.nen);
        g.max(1e-15) // Minimum floor prevents singular matrix
    }

    /// Junction voltage limiting (Nagel's algorithm from SPICE)
    ///
    /// This is critical for Newton-Raphson convergence with BJTs. The exponential
    /// I-V characteristic means that large voltage changes can cause currents to
    /// blow up, diverging NR. This function limits how much a junction voltage
    /// can change between iterations.
    ///
    /// Algorithm from: L.W. Nagel, "SPICE2: A Computer Program to Simulate
    /// Semiconductor Circuits", UCB/ERL M520, 1975
    ///
    /// Used by commercial simulators: Spectre, HSPICE, PSpice, etc.
    pub(super) fn limit_junction_voltage(
        vnew: Value,
        vold: Value,
        vt: Value,
        vcrit: Value,
    ) -> Value {
        let vt = vt.max(1e-18);
        if !vnew.is_finite() {
            return vold;
        }
        if !vold.is_finite() {
            return vnew;
        }

        if vnew > vcrit && (vnew - vold).abs() > 2.0 * vt {
            if vold > 0.0 {
                let arg = (vnew - vold) / vt;
                if arg > 0.0 {
                    vold + vt * (2.0 + (arg - 2.0).ln())
                } else {
                    vold - vt * (2.0 + (2.0 - arg).ln())
                }
            } else {
                vt * (vnew / vt).max(1e-18).ln()
            }
        } else if vnew < 0.0 {
            let arg = if vold > 0.0 {
                -vold - 1.0
            } else {
                2.0 * vold - 1.0
            };
            if vnew < arg { arg } else { vnew }
        } else {
            vnew
        }
    }
}
