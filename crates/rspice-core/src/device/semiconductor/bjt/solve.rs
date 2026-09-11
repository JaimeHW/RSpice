//! Intrinsic solve, thermal residual, convergence, and terminal stamping helpers.

use super::*;

impl Bjt {
    pub(super) fn solve_intrinsic_terminal_state(
        &self,
        vc: Value,
        vb: Value,
        ve: Value,
        vs: Value,
    ) -> IntrinsicTerminalState {
        if !self.has_intrinsic_state_unknowns() {
            // Every series element is externalized or absent and there are
            // no parasitic-transport or self-heating unknowns: the internal
            // nodes ARE the terminals. The post-solve collapse below would
            // overwrite every branch with exactly these identities anyway,
            // so the 8-dim internal Newton (whose vcrit-style legacy seed
            // makes it iterate on every uncached call) has nothing to say —
            // this is bjtload's direct evaluation case, and on promoted GP
            // decks it is the difference between ~1 µs and ~25 µs per call.
            return IntrinsicTerminalState {
                vcx: vc,
                vci: vc,
                vbx: vb,
                vbi: vb,
                vei: ve,
                vbp: vc,
                vsi: if self.vbic_three_terminal { 0.0 } else { vs },
                vrth: 0.0,
            };
        }
        let has_rcx = Self::series_active(self.rcx);
        let has_rci = Self::series_active(self.rci);
        let has_rbx = Self::series_active(self.rbx);
        let has_rbi = Self::series_active(self.rbi);
        let has_re = Self::series_active(self.re);
        let has_rs = self.has_substrate_resistance();
        let has_self_heat = self.thermal_model_enabled();
        let reuse_previous_state = self.reduced_linearization_cache_valid.get();
        let vcx = if reuse_previous_state {
            self.vcx
        } else if has_rcx {
            vc - self.ic * self.rcx.max(0.0)
        } else {
            vc
        };
        let vci = if reuse_previous_state {
            self.vci
        } else if has_rci {
            vcx - self.ic * self.rci.max(0.0)
        } else {
            vcx
        };
        let vbx = if reuse_previous_state {
            self.vbx
        } else if has_rbx {
            vb - self.ib * self.rbx.max(0.0)
        } else {
            vb
        };
        let vbi = if reuse_previous_state {
            self.vbi
        } else if has_rbi {
            vbx - self.ib * self.rbi.max(0.0)
        } else {
            vbx
        };
        let vei = if reuse_previous_state {
            self.vei
        } else if has_re {
            ve - self.ie * self.re.max(0.0)
        } else {
            ve
        };
        let vsi = if self.vbic_three_terminal {
            0.0
        } else if reuse_previous_state {
            self.vsi
        } else if has_rs {
            vs - self.isub * self.rs.max(0.0)
        } else {
            vs
        };
        // The external-collector voltage seeds vbp whether or not the
        // parasitic node is solved this pass.
        let vbp = if reuse_previous_state { self.vbp } else { vcx };
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
                    / self.thermal_conductance_at(0.0).0.max(1e-18)
            }
        } else {
            0.0
        };
        if !has_self_heat {
            vrth = 0.0;
        }

        let state = if !reuse_previous_state
            && self.charge_model == BjtChargeModel::LegacyGummelPoon
            && (self.uses_legacy_junction_limiting() || self.initial_off)
        {
            let mut seed = self.legacy_startup_intrinsic_state_seed([vc, vb, ve, vs]);
            // The global device initializer still applies SPICE's vcrit.
            // The private resistive solve need not invent a larger forward
            // drive than the terminals supply: vcrit grows with -ln(M) and
            // otherwise consumes hundreds of Newton steps for tiny devices.
            let applied_forward_bias = (self.polarity() * (vb - ve)).max(0.0);
            let seeded_forward_bias = self.polarity() * (seed[IDX_VBI] - seed[IDX_VEI]);
            if seeded_forward_bias > applied_forward_bias {
                let base = seed[IDX_VEI] + self.polarity() * applied_forward_bias;
                for index in [IDX_VCX, IDX_VCI, IDX_VBX, IDX_VBI, IDX_VBP] {
                    seed[index] = base;
                }
            }
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
                let direct = self.solve_intrinsic_state_from_seed(vc, vb, ve, vs, seed);
                // A warm prediction can fail after the terminals change.
                // Judge that solve by its remaining state correction, not
                // by a current residual whose size depends on multiplicity.
                if has_self_heat {
                    let (residual, jacobian) =
                        self.intrinsic_state_residual_jacobian(vc, vb, ve, vs, direct.0);
                    let rhs = residual.map(|value| -value);
                    let unresolved =
                        crate::numerics::solve_small_dense(&jacobian, &rhs, INTERNAL_DIM)
                            .is_none_or(|delta| delta.iter().any(|value| value.abs() >= 1e-13));
                    if unresolved {
                        return self.solve_intrinsic_state_with_self_heating_continuation(
                            vc, vb, ve, vs, seed,
                        );
                    }
                }
                direct
            }
        };

        let (mut best_state, mut best_residual_norm) =
            solve_from_seed(predicted_state.unwrap_or(state));
        if self.charge_model == BjtChargeModel::Vbic
            && reuse_previous_state
            && self.vbic_max_local_branch_delta(best_state, predicted_state.unwrap_or(state)) > 0.1
            && let Some((continued_state, continued_residual_norm)) = self
                .solve_intrinsic_state_with_external_continuation(
                    previous_external,
                    state,
                    [vc, vb, ve, vs],
                )
            && (continued_residual_norm + 1e-15 < best_residual_norm
                || self
                    .vbic_max_local_branch_delta(continued_state, predicted_state.unwrap_or(state))
                    <= 0.1)
        {
            best_state = continued_state;
            best_residual_norm = continued_residual_norm;
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
            && self.uses_legacy_junction_limiting()
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
                if refined_residual_norm < best_residual_norm {
                    best_state = refined_state;
                    best_residual_norm = refined_residual_norm;
                    continue;
                }
                break;
            }
        }

        self.impose_intrinsic_node_constraints(&mut best_state, vc, vb, ve, vs);
        self.intrinsic_state_from_internal_vector(best_state)
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
        let has_rs = self.has_substrate_resistance();
        let has_self_heat = self.thermal_model_enabled();
        let solve_vbp = self.vbic_solves_vbp();

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
        let [collector_internal, base_internal, emitter_internal] =
            self.intrinsic_terminal_branches(&eval);
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
            jacobian[IDX_VCX][..INTERNAL_DIM].copy_from_slice(&row.d_internal[..INTERNAL_DIM]);
            external_partials[IDX_VCX] = row.d_external;
        } else {
            jacobian[IDX_VCX][IDX_VCX] = 1.0;
            external_partials[IDX_VCX][EXT_C] = -1.0;
        }

        if has_rci {
            let row = Self::sub_branches(eval.irci, collector_internal);
            jacobian[IDX_VCI][..INTERNAL_DIM].copy_from_slice(&row.d_internal[..INTERNAL_DIM]);
        } else {
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
            jacobian[IDX_VBX][..INTERNAL_DIM].copy_from_slice(&row.d_internal[..INTERNAL_DIM]);
            external_partials[IDX_VBX] = row.d_external;
        } else {
            jacobian[IDX_VBX][IDX_VBX] = 1.0;
            external_partials[IDX_VBX][EXT_B] = -1.0;
        }

        if has_rbi {
            let row = Self::sub_branches(eval.irbi, base_internal);
            jacobian[IDX_VBI][..INTERNAL_DIM].copy_from_slice(&row.d_internal[..INTERNAL_DIM]);
        } else {
            jacobian[IDX_VBI][IDX_VBI] = 1.0;
            jacobian[IDX_VBI][IDX_VBX] = -1.0;
        }

        if has_re {
            let row = Self::sub_branches(Self::add_branches(eval.ire, eval.ibex), emitter_internal);
            jacobian[IDX_VEI][..INTERNAL_DIM].copy_from_slice(&row.d_internal[..INTERNAL_DIM]);
            external_partials[IDX_VEI] = row.d_external;
        } else {
            jacobian[IDX_VEI][IDX_VEI] = 1.0;
            external_partials[IDX_VEI][EXT_E] = -1.0;
        }

        if solve_vbp {
            let row = Self::sub_branches(Self::add_branches(eval.ibep, eval.ibcp), eval.irbp);
            jacobian[IDX_VBP][..INTERNAL_DIM].copy_from_slice(&row.d_internal[..INTERNAL_DIM]);
        } else {
            jacobian[IDX_VBP][IDX_VBP] = 1.0;
            jacobian[IDX_VBP][IDX_VCX] = -1.0;
        }

        if has_rs {
            let row = Self::sub_branches(Self::add_branches(eval.irs, eval.iccp), eval.ibcp);
            jacobian[IDX_VSI][..INTERNAL_DIM].copy_from_slice(&row.d_internal[..INTERNAL_DIM]);
            external_partials[IDX_VSI] = row.d_external;
        } else {
            jacobian[IDX_VSI][IDX_VSI] = 1.0;
            if !self.vbic_three_terminal {
                external_partials[IDX_VSI][EXT_S] = -1.0;
            }
        }

        if has_self_heat {
            let row = Self::sub_branches(thermal_sink, thermal_power);
            jacobian[IDX_VRTH][..INTERNAL_DIM].copy_from_slice(&row.d_internal[..INTERNAL_DIM]);
            external_partials[IDX_VRTH] = row.d_external;
        } else {
            jacobian[IDX_VRTH][IDX_VRTH] = 1.0;
        }

        let mut sensitivities = [[0.0; EXTERNAL_DIM]; INTERNAL_DIM];
        for external in 0..EXTERNAL_DIM {
            let rhs = external_partials.map(|partials| -partials[external]);
            if let Some(solution) =
                crate::numerics::solve_small_dense(&jacobian, &rhs, INTERNAL_DIM)
            {
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

        sensitivities
    }

    /// Current delivered from BP into CX. When RBP is an ideal short,
    /// folding the BP row must carry its junction currents into CX as well.
    #[inline]
    pub(super) fn parasitic_base_collector_branch(
        &self,
        eval: &EvaluatedBjtState,
    ) -> BranchLinearization {
        if self.charge_model == BjtChargeModel::LegacyGummelPoon {
            // GP's substrate current is folded at its intrinsic connection.
            BranchLinearization::default()
        } else if self.vbic_solves_vbp() {
            eval.irbp
        } else {
            Self::add_branches(eval.ibep, eval.ibcp)
        }
    }

    /// Intrinsic terminal rows shared by Newton, sensitivity and reduction.
    /// GP's substrate branch shares the charge network's intrinsic endpoint;
    /// it must flow through RC/RB before reaching the external terminal.
    pub(super) fn intrinsic_terminal_branches(
        &self,
        eval: &EvaluatedBjtState,
    ) -> [BranchLinearization; 3] {
        let (collector_d, base_d, emitter_d) = self.intrinsic_terminal_derivatives(eval.linearized);
        let mut collector = Self::branch_from_internal(eval.linearized.ic, collector_d);
        let mut base = Self::branch_from_internal(eval.linearized.ib, base_d);
        let emitter =
            Self::branch_from_internal(-(eval.linearized.ic + eval.linearized.ib), emitter_d);
        if self.charge_model == BjtChargeModel::LegacyGummelPoon {
            match self.substrate_topology {
                BjtSubstrateTopology::Vertical => {
                    collector = Self::sub_branches(collector, eval.ibcp)
                }
                BjtSubstrateTopology::Lateral => base = Self::sub_branches(base, eval.ibcp),
            }
        }
        [collector, base, emitter]
    }

    pub(super) fn external_terminal_branches(
        &self,
        eval: EvaluatedBjtState,
    ) -> [BranchLinearization; EXTERNAL_DIM] {
        let mut rows = [BranchLinearization::default(); EXTERNAL_DIM];
        self.visit_external_kcl_terms(eval, false, |index, terms| {
            rows[index] = Self::sum_branch_terms(terms);
        });
        rows
    }

    pub(super) fn sum_branch_terms(terms: &[BranchLinearization]) -> BranchLinearization {
        terms
            .iter()
            .copied()
            .reduce(Self::add_branches)
            .unwrap_or_default()
    }

    pub(super) fn visit_external_kcl_terms(
        &self,
        eval: EvaluatedBjtState,
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
            visit(EXT_C, &[eval.ircx]);
        } else {
            visit(
                EXT_C,
                &[
                    if Self::series_active(self.rci) {
                        eval.irci
                    } else {
                        collector
                    },
                    negative(self.parasitic_base_collector_branch(&eval)),
                    negative(eval.igcx),
                ],
            );
        }
        if Self::series_active(self.rbx) {
            visit(EXT_B, &[eval.irbx]);
        } else {
            visit(
                EXT_B,
                &[
                    if Self::series_active(self.rbi) {
                        rbi
                    } else {
                        base
                    },
                    eval.ibex,
                    eval.ibep,
                    eval.iccp,
                    eval.igcx,
                ],
            );
        }
        if Self::series_active(self.re) {
            visit(EXT_E, &[eval.ire]);
        } else {
            visit(EXT_E, &[emitter, negative(eval.ibex)]);
        }
        if self.has_substrate_resistance() {
            visit(EXT_S, &[eval.irs]);
        } else {
            visit(EXT_S, &[eval.ibcp, negative(eval.iccp)]);
        }
    }

    pub(super) fn thermal_sink_branch(&self, vrth: Value) -> BranchLinearization {
        let mut branch = BranchLinearization::default();
        let (gth, d_gth) = self.thermal_conductance_at(vrth);
        if gth <= 0.0 {
            return branch;
        }
        branch.current = gth * vrth;
        branch.d_internal[IDX_VRTH] = gth + vrth * d_gth;
        branch
    }

    pub(super) fn thermal_power_branch(
        &self,
        mut eval: EvaluatedBjtState,
        external: [Value; EXTERNAL_DIM],
        internal: [Value; INTERNAL_DIM],
    ) -> BranchLinearization {
        if !self.thermal_model_enabled() || !self.vbic_heat_generation {
            return BranchLinearization::default();
        }

        let [vcx, vci, vbx, vbi, vei, vbp, vsi, _vrth] = internal;
        let [vc, vb, ve, vs] = external;
        if self.vbic_13 {
            // Xyce computes power before adding its numerical GMIN parallels.
            // Igcx is likewise absent from the model's defined power sum.
            let gmin = self.nonlinear_branch_gmin();
            let remove = |branch: &mut BranchLinearization, pos: usize, neg: usize| {
                branch.current -= gmin * (internal[pos] - internal[neg]);
                branch.d_internal[pos] -= gmin;
                branch.d_internal[neg] += gmin;
            };
            remove(&mut eval.ibe, IDX_VBI, IDX_VEI);
            remove(&mut eval.ibex, IDX_VBX, IDX_VEI);
            remove(&mut eval.ibc, IDX_VBI, IDX_VCI);
            remove(&mut eval.ibep, IDX_VBX, IDX_VBP);
            if !self.vbic_three_terminal {
                remove(&mut eval.ibcp, IDX_VSI, IDX_VBP);
            }
        }
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
            eval.ibex,
            vbx - vei,
            [0.0, 0.0, 1.0, 0.0, -1.0, 0.0, 0.0, 0.0],
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
        let gth = self.thermal_conductance_at(internal[IDX_VRTH]).0;
        if gth <= 0.0 {
            return 0.0;
        }

        let state = self.intrinsic_state_from_internal_vector(internal);
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
        self.thermal_power_branch(eval, [vc, vb, ve, vs], internal)
            .current
            / gth
    }

    /// Lower bound on the raw thermal node. VBIC 1.3 limits the mapped
    /// temperature instead, so its raw node has no lower bound.
    #[inline]
    pub(crate) fn minimum_thermal_rise(&self) -> Value {
        if self.vbic_13 {
            Value::NEG_INFINITY
        } else {
            1.0 - self.requested_temperature()
        }
    }

    pub(super) fn thermal_rebalance_step_limit(&self, rise: Value) -> Value {
        let minimum = self.minimum_thermal_rise();
        let half_distance = if minimum.is_finite() {
            0.5 * rise - 0.5 * minimum
        } else {
            0.5 * rise.abs() + 0.5 * self.requested_temperature().abs()
        };
        (half_distance + 5.0).max(0.5)
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
    ) -> [Value; VBIC_TRANSIENT_CONVERGENCE_VOLTAGE_COUNT] {
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
            if self.vbic_three_terminal {
                0.0
            } else {
                p * (vsi - vbp)
            },
            p * (vbx - vcx),
        ]
    }

    pub(super) fn vbic_convergence_branches_for_state(
        &self,
        internal: [Value; INTERNAL_DIM],
    ) -> [BranchLinearization; Self::VBIC_CONVERGENCE_BRANCH_COUNT] {
        let [vcx, vci, vbx, vbi, vei, vbp, vsi, vrth] = internal;
        let eval = self.evaluate_state(
            BjtNodeVoltages {
                vc: self.vc_ext,
                vb: self.vb_ext,
                ve: self.ve_ext,
                vs: self.vs_ext,
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
        [
            eval.ibe, eval.ibep, eval.iciei, eval.ibc, eval.irci, eval.irbi, eval.irbp, eval.ibcp,
            eval.iccp, eval.ibex, eval.igcx,
        ]
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
        // ngspice CKTnoncon: an iterate whose junction voltages pnjlim
        // replaced is by definition not converged -- the stamped companion
        // intentionally disagrees with the raw solution until the limiter
        // disengages.
        if self.legacy_junction_limited {
            return false;
        }

        // Preserve the internal bias accuracy formerly supplied by the
        // private solve. Absolute current tolerances alone lose this accuracy
        // for small AREA/M. Direct GP keeps the native dialect's stopping rule.
        if self.mna_promoted() {
            let tolerance = criteria.voltage_tolerance().min(1e-10);
            for (current, previous) in [
                (self.vbe, self.vbe_prev),
                (self.vbc, self.vbc_prev),
                (self.vsi, self.vsi_prev),
            ] {
                let rounding = 64.0 * Value::EPSILON * current.abs().max(previous.abs());
                if !current.is_finite() || (current - previous).abs() > tolerance + rounding {
                    return false;
                }
            }
        }

        if self.xyce_compatibility {
            // Xyce's native GP BJT contributes to allDevicesConverged only
            // through origFlag: the device is converged once pnjlim leaves
            // both junction voltages unchanged. Its DC NOX status test then
            // applies the global weighted-update and raw-residual gates.
            return true;
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
        let reduced = self.reduced_linearization(vc, vb, ve, vs);
        let mut rows = reduced.g_reduced;
        let anchor = self.companion_anchor(vc, vb, ve, vs);
        let mut currents = self.external_terminal_currents_at_bias(vc, vb, ve, vs);
        if self.uses_legacy_gummel_poon() && self.has_intrinsic_state_unknowns() {
            // Junction limiting can move a private node away from internal
            // KCL. Schur reduction must eliminate that residual as well as
            // its Jacobian: Fe_reduced = Fe - Gei * Gii^-1 * Fi. Evaluate Fi
            // directly at the companion anchor, avoiding cancellation of
            // absolute-voltage products in Gi * V - zi.
            let [ac, ab, ae, asub] = anchor;
            let (residual, _) =
                self.intrinsic_state_residual_jacobian(ac, ab, ae, asub, reduced.internal_voltages);
            let correction =
                crate::numerics::solve_small_dense(&reduced.g_ii, &residual, INTERNAL_DIM)
                    .unwrap_or([Value::NAN; INTERNAL_DIM]);
            for (current, row) in currents.iter_mut().zip(&reduced.g_ei) {
                *current -= row
                    .iter()
                    .zip(&correction)
                    .map(|(g, dv)| g * dv)
                    .sum::<Value>();
            }
        }
        let reference = self.project_legacy_tied_terminal_system(&mut rows, &mut currents);
        let mut rhs = [0.0; EXTERNAL_DIM];
        for row in 0..EXTERNAL_DIM {
            // A tied port uses independent voltage differences. Forming
            // its Norton source from absolute voltages can lose the small
            // current beside cancelling common-mode products. Subtract the
            // current after the dot product, as in the original static stamp.
            rhs[row] = (0..EXTERNAL_DIM)
                .map(|col| {
                    let voltage = reference.map_or(anchor[col], |idx| anchor[col] - anchor[idx]);
                    rows[row][col] * voltage
                })
                .sum::<Value>()
                - currents[row];
        }
        (rows, rhs)
    }

    /// Reduce a conservative electrical port before adding it to the global
    /// matrix. The shared C/B/E node is the reference: its dependent row and
    /// column must never be summed into an already populated matrix slot.
    /// Keep the independent terminal entries and recover the reference from
    /// KCL and voltage-shift invariance. This also retains a distinct substrate.
    pub(crate) fn project_legacy_tied_terminal_system<T>(
        &self,
        matrix: &mut [[T; EXTERNAL_DIM]; EXTERNAL_DIM],
        rhs: &mut [T; EXTERNAL_DIM],
    ) -> Option<usize>
    where
        T: Copy + Default + std::ops::AddAssign + std::ops::SubAssign,
    {
        if self.charge_model != BjtChargeModel::LegacyGummelPoon {
            return None;
        }
        let nodes = self.external_terminal_nodes();
        let reference = if nodes[EXT_C] == nodes[EXT_B] || nodes[EXT_C] == nodes[EXT_E] {
            EXT_C
        } else if nodes[EXT_B] == nodes[EXT_E] {
            EXT_B
        } else {
            return None;
        };
        let representatives: [usize; EXTERNAL_DIM] =
            std::array::from_fn(|idx| (0..=idx).find(|&other| nodes[other] == nodes[idx]).unwrap());
        let mut reduced = [[T::default(); EXTERNAL_DIM]; EXTERNAL_DIM];
        let mut source = [T::default(); EXTERNAL_DIM];
        for row in 0..EXTERNAL_DIM {
            if nodes[row] == nodes[reference] {
                continue;
            }
            let i = representatives[row];
            source[i] += rhs[row];
            source[reference] -= rhs[row];
            for col in 0..EXTERNAL_DIM {
                if nodes[col] == nodes[reference] {
                    continue;
                }
                let j = representatives[col];
                let value = matrix[row][col];
                reduced[i][j] += value;
                reduced[i][reference] -= value;
                reduced[reference][j] -= value;
                reduced[reference][reference] += value;
            }
        }
        *matrix = reduced;
        *rhs = source;
        Some(reference)
    }

    pub(crate) fn stamp_small_signal_ac(
        &self,
        voltages: &[Value],
        matrix: &mut impl MatrixStamper,
    ) {
        if self.mna_promoted() {
            // The promoted static system (terminal rows, internal KCL rows,
            // and excess-phase algebraic rows) is the small-signal real part;
            // AC stampers ignore the rhs source terms.
            self.stamp_mna(matrix);
            return;
        }
        let [vc, vb, ve, vs] = self.external_terminal_voltages(voltages);
        let mut rows = self.small_signal_row_coefficients(vc, vb, ve, vs);
        self.project_legacy_tied_terminal_system(&mut rows, &mut [0.0; EXTERNAL_DIM]);
        let nodes = self.external_terminal_nodes();
        for row_idx in 0..EXTERNAL_DIM {
            for col_idx in 0..EXTERNAL_DIM {
                matrix.stamp(nodes[row_idx], nodes[col_idx], rows[row_idx][col_idx]);
            }
        }
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

    /// Xyce 7.10's native GP-BJT junction limiter. Xyce intentionally uses
    /// its original `DeviceSupport::pnjlim` for this device rather than the
    /// later ngspice-derived negative-voltage limiter.
    pub(super) fn limit_xyce_junction_voltage(
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
        if vnew <= vcrit || (vnew - vold).abs() <= 2.0 * vt {
            return vnew;
        }
        if vold > 0.0 {
            let arg = 1.0 + (vnew - vold) / vt;
            if arg > 0.0 {
                vold + vt * arg.ln()
            } else {
                vcrit
            }
        } else {
            vt * (vnew / vt).ln()
        }
    }
}
