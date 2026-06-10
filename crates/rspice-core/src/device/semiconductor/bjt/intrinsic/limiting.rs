//! Intrinsic-state limiting, prediction, and continuation step-limit helpers.

use super::*;

impl Bjt {
    #[inline]
    pub(in crate::device::semiconductor::bjt) fn limit_logarithmic_step(
        vnew: Value,
        vold: Value,
        limit: Value,
    ) -> Value {
        let limit = limit.max(1e-18);
        if !vnew.is_finite() {
            return vold;
        }
        if !vold.is_finite() {
            return vnew;
        }

        if vnew > vold + limit {
            vold + limit + ((vnew - vold) / limit).log10()
        } else if vnew < vold - limit {
            vold - limit - ((vold - vnew) / limit).log10()
        } else {
            vnew
        }
    }

    #[inline]
    pub(in crate::device::semiconductor::bjt) fn junction_critical_voltage(
        vt: Value,
        isat: Value,
    ) -> Value {
        let vt = vt.max(1e-18);
        let isat = isat.abs().max(1e-18);
        vt * (vt / (core::f64::consts::SQRT_2 * isat)).ln()
    }

    #[inline]
    pub(in crate::device::semiconductor::bjt) fn vbic_limiting_parameters(
        &self,
        previous_vrth: Value,
    ) -> (Value, Value) {
        self.with_temperature_variant(previous_vrth, |model| {
            let vt = model.vt.max(1e-18);
            let vcrit = Self::junction_critical_voltage(vt, model.is);
            (vt, vcrit)
        })
    }

    #[inline]
    pub(in crate::device::semiconductor::bjt) fn vbic_nonlinear_branch_voltages(
        &self,
        internal: [Value; INTERNAL_DIM],
    ) -> VbicNonlinearBranchVoltages {
        let p = self.polarity();
        VbicNonlinearBranchVoltages {
            vbei: p * (internal[IDX_VBI] - internal[IDX_VEI]),
            vbex: p * (internal[IDX_VBX] - internal[IDX_VEI]),
            vbci: p * (internal[IDX_VBI] - internal[IDX_VCI]),
            vbcx: p * (internal[IDX_VBI] - internal[IDX_VCX]),
            vbep: p * (internal[IDX_VBX] - internal[IDX_VBP]),
            vbcp: p * (internal[IDX_VSI] - internal[IDX_VBP]),
            vrth: internal[IDX_VRTH],
        }
    }

    #[inline]
    pub(in crate::device::semiconductor::bjt) fn legacy_limiting_parameters(
        &self,
        previous_vrth: Value,
    ) -> (Value, Value, Value) {
        self.with_temperature_variant(previous_vrth, |model| {
            let vt = model.vt.max(1e-18);
            let vcrit = Self::junction_critical_voltage(vt, model.is);
            (vt, vcrit, 50.0)
        })
    }

    #[inline]
    pub(in crate::device::semiconductor::bjt) fn legacy_nonlinear_branch_voltages(
        &self,
        internal: [Value; INTERNAL_DIM],
    ) -> LegacyNonlinearBranchVoltages {
        let p = self.polarity();
        LegacyNonlinearBranchVoltages {
            vbe: p * (internal[IDX_VBI] - internal[IDX_VEI]),
            vbc: p * (internal[IDX_VBI] - internal[IDX_VCI]),
            vsub: p * (internal[IDX_VSI] - internal[IDX_VCI]),
        }
    }

    pub(in crate::device::semiconductor::bjt) fn project_vbic_limited_branches_onto_internal_state(
        &self,
        raw: [Value; INTERNAL_DIM],
        limited: VbicNonlinearBranchVoltages,
    ) -> [Value; INTERNAL_DIM] {
        let p = self.polarity();
        let raw_nodes = [
            raw[IDX_VCX],
            raw[IDX_VCI],
            raw[IDX_VBX],
            raw[IDX_VBI],
            raw[IDX_VEI],
            raw[IDX_VBP],
            raw[IDX_VSI],
        ];
        let constraints = [
            [0.0, 0.0, 0.0, p, -p, 0.0, 0.0],
            [0.0, 0.0, p, 0.0, -p, 0.0, 0.0],
            [0.0, -p, 0.0, p, 0.0, 0.0, 0.0],
            [-p, 0.0, 0.0, p, 0.0, 0.0, 0.0],
            [0.0, 0.0, p, 0.0, 0.0, -p, 0.0],
            [0.0, 0.0, 0.0, 0.0, 0.0, -p, p],
        ];
        let targets = [
            limited.vbei,
            limited.vbex,
            limited.vbci,
            limited.vbcx,
            limited.vbep,
            limited.vbcp,
        ];

        let mut residual = [0.0; VBIC_LIMITED_BRANCH_DIM];
        for row in 0..VBIC_LIMITED_BRANCH_DIM {
            residual[row] = -targets[row];
            for col in 0..raw_nodes.len() {
                residual[row] += constraints[row][col] * raw_nodes[col];
            }
        }

        let mut gram = [[0.0; VBIC_LIMITED_BRANCH_DIM]; VBIC_LIMITED_BRANCH_DIM];
        for row in 0..VBIC_LIMITED_BRANCH_DIM {
            for col in 0..VBIC_LIMITED_BRANCH_DIM {
                gram[row][col] = (0..raw_nodes.len())
                    .map(|idx| constraints[row][idx] * constraints[col][idx])
                    .sum();
            }
        }

        let Some(lagrange) =
            Self::solve_small_dense_system(&gram, &residual, VBIC_LIMITED_BRANCH_DIM)
        else {
            let mut fallback = raw;
            fallback[IDX_VRTH] = limited.vrth;
            return fallback;
        };

        let mut projected = raw;
        for node_idx in 0..raw_nodes.len() {
            let correction = (0..VBIC_LIMITED_BRANCH_DIM)
                .map(|row| constraints[row][node_idx] * lagrange[row])
                .sum::<Value>();
            projected[node_idx] = raw_nodes[node_idx] - correction;
        }
        projected[IDX_VRTH] = limited.vrth;
        projected
    }

    pub(in crate::device::semiconductor::bjt) fn project_legacy_limited_branches_onto_internal_state(
        &self,
        raw: [Value; INTERNAL_DIM],
        limited: LegacyNonlinearBranchVoltages,
    ) -> [Value; INTERNAL_DIM] {
        let p = self.polarity();
        let raw_nodes = [
            raw[IDX_VCX],
            raw[IDX_VCI],
            raw[IDX_VBX],
            raw[IDX_VBI],
            raw[IDX_VEI],
            raw[IDX_VBP],
            raw[IDX_VSI],
        ];
        let constraints = [
            [0.0, 0.0, 0.0, p, -p, 0.0, 0.0],
            [0.0, -p, 0.0, p, 0.0, 0.0, 0.0],
            [0.0, -p, 0.0, 0.0, 0.0, 0.0, p],
        ];
        let targets = [limited.vbe, limited.vbc, limited.vsub];

        let mut residual = [0.0; LEGACY_LIMITED_BRANCH_DIM];
        for row in 0..LEGACY_LIMITED_BRANCH_DIM {
            residual[row] = -targets[row];
            for col in 0..raw_nodes.len() {
                residual[row] += constraints[row][col] * raw_nodes[col];
            }
        }

        let mut gram = [[0.0; LEGACY_LIMITED_BRANCH_DIM]; LEGACY_LIMITED_BRANCH_DIM];
        for row in 0..LEGACY_LIMITED_BRANCH_DIM {
            for col in 0..LEGACY_LIMITED_BRANCH_DIM {
                gram[row][col] = (0..raw_nodes.len())
                    .map(|idx| constraints[row][idx] * constraints[col][idx])
                    .sum();
            }
        }

        let Some(lagrange) =
            Self::solve_small_dense_system(&gram, &residual, LEGACY_LIMITED_BRANCH_DIM)
        else {
            return raw;
        };

        let mut projected = raw;
        for node_idx in 0..raw_nodes.len() {
            let correction = (0..LEGACY_LIMITED_BRANCH_DIM)
                .map(|row| constraints[row][node_idx] * lagrange[row])
                .sum::<Value>();
            projected[node_idx] = raw_nodes[node_idx] - correction;
        }
        projected
    }

    pub(in crate::device::semiconductor::bjt) fn limit_vbic_internal_state_to_previous(
        &self,
        raw: [Value; INTERNAL_DIM],
        previous: [Value; INTERNAL_DIM],
    ) -> [Value; INTERNAL_DIM] {
        if self.charge_model != BjtChargeModel::Vbic {
            return raw;
        }

        let raw_branches = self.vbic_nonlinear_branch_voltages(raw);
        let previous_branches = self.vbic_nonlinear_branch_voltages(previous);
        let (vt, vcrit) = self.vbic_limiting_parameters(previous[IDX_VRTH]);
        let limited_branches = VbicNonlinearBranchVoltages {
            vbei: Self::limit_junction_voltage(
                raw_branches.vbei,
                previous_branches.vbei,
                vt,
                vcrit,
            ),
            vbex: Self::limit_junction_voltage(
                raw_branches.vbex,
                previous_branches.vbex,
                vt,
                vcrit,
            ),
            vbci: Self::limit_junction_voltage(
                raw_branches.vbci,
                previous_branches.vbci,
                vt,
                vcrit,
            ),
            vbcx: Self::limit_junction_voltage(
                raw_branches.vbcx,
                previous_branches.vbcx,
                vt,
                vcrit,
            ),
            vbep: Self::limit_junction_voltage(
                raw_branches.vbep,
                previous_branches.vbep,
                vt,
                vcrit,
            ),
            vbcp: Self::limit_junction_voltage(
                raw_branches.vbcp,
                previous_branches.vbcp,
                vt,
                vcrit,
            ),
            vrth: if self.self_heating_enabled() {
                Self::limit_logarithmic_step(raw_branches.vrth, previous_branches.vrth, 100.0)
                    .max(self.minimum_thermal_rise())
            } else {
                0.0
            },
        };

        let projected =
            self.project_vbic_limited_branches_onto_internal_state(raw, limited_branches);
        if projected.iter().all(|value| value.is_finite()) {
            projected
        } else {
            raw
        }
    }

    pub(in crate::device::semiconductor::bjt) fn limit_legacy_internal_state_to_previous(
        &self,
        raw: [Value; INTERNAL_DIM],
        previous: [Value; INTERNAL_DIM],
    ) -> [Value; INTERNAL_DIM] {
        if self.charge_model != BjtChargeModel::LegacyGummelPoon {
            return raw;
        }

        let raw_branches = self.legacy_nonlinear_branch_voltages(raw);
        let previous_branches = self.legacy_nonlinear_branch_voltages(previous);
        let (vt, vcrit, sub_vcrit) = self.legacy_limiting_parameters(previous[IDX_VRTH]);
        let limited_branches = LegacyNonlinearBranchVoltages {
            vbe: Self::limit_junction_voltage(raw_branches.vbe, previous_branches.vbe, vt, vcrit),
            vbc: Self::limit_junction_voltage(raw_branches.vbc, previous_branches.vbc, vt, vcrit),
            vsub: Self::limit_junction_voltage(
                raw_branches.vsub,
                previous_branches.vsub,
                vt,
                sub_vcrit,
            ),
        };

        let projected =
            self.project_legacy_limited_branches_onto_internal_state(raw, limited_branches);
        if projected.iter().all(|value| value.is_finite()) {
            projected
        } else {
            raw
        }
    }

    pub(crate) fn limit_vbic_dynamic_internal_state_to_previous(
        &self,
        raw: [Value; BJT_INTERNAL_STATE_DIM],
        previous: [Value; BJT_INTERNAL_STATE_DIM],
    ) -> [Value; BJT_INTERNAL_STATE_DIM] {
        if self.charge_model != BjtChargeModel::Vbic {
            return raw;
        }

        let mut raw_static = [0.0; INTERNAL_DIM];
        raw_static.copy_from_slice(&raw[..INTERNAL_DIM]);
        let mut previous_static = [0.0; INTERNAL_DIM];
        previous_static.copy_from_slice(&previous[..INTERNAL_DIM]);

        let mut limited = raw;
        limited[..INTERNAL_DIM].copy_from_slice(
            &self.limit_vbic_internal_state_to_previous(raw_static, previous_static),
        );
        limited
    }

    #[inline]
    pub(crate) fn predict_vbic_dynamic_internal_state_from_previous_external_bias(
        &self,
        previous_external: [Value; EXTERNAL_DIM],
        previous_dynamic: [Value; BJT_INTERNAL_STATE_DIM],
        proposed_external: [Value; EXTERNAL_DIM],
    ) -> Option<[Value; BJT_INTERNAL_STATE_DIM]> {
        if self.charge_model != BjtChargeModel::Vbic {
            return None;
        }

        let mut previous_static = [0.0; INTERNAL_DIM];
        previous_static.copy_from_slice(&previous_dynamic[..INTERNAL_DIM]);
        let predicted_static = self.predict_intrinsic_state_from_previous_external_bias(
            previous_external,
            previous_static,
            proposed_external,
        )?;

        let mut predicted_dynamic = previous_dynamic;
        predicted_dynamic[..INTERNAL_DIM].copy_from_slice(&predicted_static);
        Some(
            self.limit_vbic_dynamic_internal_state_to_previous(predicted_dynamic, previous_dynamic),
        )
    }

    #[inline]
    pub(crate) fn vbic_dynamic_internal_state_within_local_branch_envelope(
        &self,
        state: [Value; BJT_INTERNAL_STATE_DIM],
        reference: [Value; BJT_INTERNAL_STATE_DIM],
    ) -> bool {
        let mut state_static = [0.0; INTERNAL_DIM];
        state_static.copy_from_slice(&state[..INTERNAL_DIM]);
        let mut reference_static = [0.0; INTERNAL_DIM];
        reference_static.copy_from_slice(&reference[..INTERNAL_DIM]);
        self.vbic_internal_state_within_local_branch_envelope(state_static, reference_static)
    }

    #[inline]
    pub(in crate::device::semiconductor::bjt) fn limit_intrinsic_state_against_previous(
        &self,
        raw: [Value; INTERNAL_DIM],
        previous: [Value; INTERNAL_DIM],
    ) -> [Value; INTERNAL_DIM] {
        let mut limited = if self.charge_model == BjtChargeModel::Vbic {
            self.limit_vbic_internal_state_to_previous(raw, previous)
        } else {
            self.limit_legacy_internal_state_to_previous(raw, previous)
        };

        if self.charge_model != BjtChargeModel::Vbic && self.self_heating_enabled() {
            limited[IDX_VRTH] =
                Self::limit_logarithmic_step(raw[IDX_VRTH], previous[IDX_VRTH], 100.0)
                    .max(1.0 - self.requested_temperature());
        }

        limited
    }

    pub(in crate::device::semiconductor::bjt) fn predict_intrinsic_state_from_previous_external_bias_unlimited(
        &self,
        previous_external: [Value; EXTERNAL_DIM],
        previous_internal: [Value; INTERNAL_DIM],
        proposed_external: [Value; EXTERNAL_DIM],
    ) -> Option<[Value; INTERNAL_DIM]> {
        let previous_state = self.intrinsic_state_from_internal_vector(previous_internal);
        let sensitivities = self.internal_voltage_sensitivities(
            previous_state,
            previous_external[EXT_C],
            previous_external[EXT_B],
            previous_external[EXT_E],
            previous_external[EXT_S],
        );
        let delta_external = [
            proposed_external[EXT_C] - previous_external[EXT_C],
            proposed_external[EXT_B] - previous_external[EXT_B],
            proposed_external[EXT_E] - previous_external[EXT_E],
            proposed_external[EXT_S] - previous_external[EXT_S],
        ];

        let mut predicted = previous_internal;
        for internal_idx in 0..INTERNAL_DIM {
            predicted[internal_idx] += sensitivities[internal_idx]
                .iter()
                .zip(delta_external.iter())
                .map(|(sensitivity, delta)| sensitivity * delta)
                .sum::<Value>();
        }

        predicted
            .iter()
            .all(|value| value.is_finite())
            .then_some(predicted)
    }

    pub(in crate::device::semiconductor::bjt) fn predict_intrinsic_state_from_previous_external_bias(
        &self,
        previous_external: [Value; EXTERNAL_DIM],
        previous_internal: [Value; INTERNAL_DIM],
        proposed_external: [Value; EXTERNAL_DIM],
    ) -> Option<[Value; INTERNAL_DIM]> {
        let predicted = self.predict_intrinsic_state_from_previous_external_bias_unlimited(
            previous_external,
            previous_internal,
            proposed_external,
        )?;
        Some(self.limit_intrinsic_state_against_previous(predicted, previous_internal))
    }

    #[inline]
    pub(in crate::device::semiconductor::bjt) fn vbic_internal_state_within_local_branch_envelope(
        &self,
        state: [Value; INTERNAL_DIM],
        reference: [Value; INTERNAL_DIM],
    ) -> bool {
        if self.charge_model != BjtChargeModel::Vbic {
            return true;
        }

        let state_branches = self.vbic_nonlinear_branch_voltages(state);
        let reference_branches = self.vbic_nonlinear_branch_voltages(reference);
        let (vt, vcrit) = self.vbic_limiting_parameters(reference[IDX_VRTH]);
        let expected = VbicNonlinearBranchVoltages {
            vbei: Self::limit_junction_voltage(
                state_branches.vbei,
                reference_branches.vbei,
                vt,
                vcrit,
            ),
            vbex: Self::limit_junction_voltage(
                state_branches.vbex,
                reference_branches.vbex,
                vt,
                vcrit,
            ),
            vbci: Self::limit_junction_voltage(
                state_branches.vbci,
                reference_branches.vbci,
                vt,
                vcrit,
            ),
            vbcx: Self::limit_junction_voltage(
                state_branches.vbcx,
                reference_branches.vbcx,
                vt,
                vcrit,
            ),
            vbep: Self::limit_junction_voltage(
                state_branches.vbep,
                reference_branches.vbep,
                vt,
                vcrit,
            ),
            vbcp: Self::limit_junction_voltage(
                state_branches.vbcp,
                reference_branches.vbcp,
                vt,
                vcrit,
            ),
            vrth: if self.self_heating_enabled() {
                Self::limit_logarithmic_step(state_branches.vrth, reference_branches.vrth, 100.0)
                    .max(self.minimum_thermal_rise())
            } else {
                0.0
            },
        };

        [
            (state_branches.vbei, expected.vbei),
            (state_branches.vbex, expected.vbex),
            (state_branches.vbci, expected.vbci),
            (state_branches.vbcx, expected.vbcx),
            (state_branches.vbep, expected.vbep),
            (state_branches.vbcp, expected.vbcp),
            (state_branches.vrth, expected.vrth),
        ]
        .into_iter()
        .all(|(actual, limited)| (actual - limited).abs() <= 1e-12)
    }

    #[inline]
    pub(in crate::device::semiconductor::bjt) fn vbic_max_local_branch_delta(
        &self,
        lhs: [Value; INTERNAL_DIM],
        rhs: [Value; INTERNAL_DIM],
    ) -> Value {
        if self.charge_model != BjtChargeModel::Vbic {
            return lhs
                .iter()
                .zip(rhs.iter())
                .map(|(lhs, rhs)| (lhs - rhs).abs())
                .fold(0.0, Value::max);
        }

        let lhs_branches = self.vbic_nonlinear_branch_voltages(lhs);
        let rhs_branches = self.vbic_nonlinear_branch_voltages(rhs);
        [
            (lhs_branches.vbei - rhs_branches.vbei).abs(),
            (lhs_branches.vbex - rhs_branches.vbex).abs(),
            (lhs_branches.vbci - rhs_branches.vbci).abs(),
            (lhs_branches.vbcx - rhs_branches.vbcx).abs(),
            (lhs_branches.vbep - rhs_branches.vbep).abs(),
            (lhs_branches.vbcp - rhs_branches.vbcp).abs(),
            (lhs_branches.vrth - rhs_branches.vrth).abs(),
        ]
        .into_iter()
        .fold(0.0, Value::max)
    }

    #[inline]
    pub(in crate::device::semiconductor::bjt) fn vbic_cached_external_matches(
        &self,
        external: [Value; EXTERNAL_DIM],
        voltage_abstol: Value,
        reltol: Value,
    ) -> bool {
        let cached = [self.vc_ext, self.vb_ext, self.ve_ext, self.vs_ext];
        cached
            .iter()
            .zip(external.iter())
            .all(|(cached, external)| {
                let diff = (cached - external).abs();
                let tol = reltol * cached.abs().max(external.abs()) + voltage_abstol;
                diff <= tol
            })
    }

    #[inline]
    pub(in crate::device::semiconductor::bjt) fn vbic_branch_limit_scale(
        previous: Value,
        raw: Value,
        limited: Value,
    ) -> Option<Value> {
        let raw_delta = raw - previous;
        if !raw_delta.is_finite() || raw_delta.abs() <= 1e-18 {
            return None;
        }
        let limited_delta = limited - previous;
        if !limited_delta.is_finite() {
            return Some(0.0);
        }
        Some((limited_delta.abs() / raw_delta.abs()).clamp(0.0, 1.0))
    }

    pub(crate) fn vbic_external_step_limit_scale_from_state(
        &self,
        previous_external: [Value; EXTERNAL_DIM],
        previous_internal: [Value; INTERNAL_DIM],
        proposed_external: [Value; EXTERNAL_DIM],
    ) -> Option<Value> {
        if self.charge_model != BjtChargeModel::Vbic {
            return None;
        }

        let delta_external = [
            proposed_external[EXT_C] - previous_external[EXT_C],
            proposed_external[EXT_B] - previous_external[EXT_B],
            proposed_external[EXT_E] - previous_external[EXT_E],
            proposed_external[EXT_S] - previous_external[EXT_S],
        ];
        let max_delta = delta_external
            .iter()
            .map(|value| value.abs())
            .fold(0.0, Value::max);
        if !max_delta.is_finite() || max_delta <= 1e-15 {
            return None;
        }

        let Some(raw_internal) = self
            .predict_intrinsic_state_from_previous_external_bias_unlimited(
                previous_external,
                previous_internal,
                proposed_external,
            )
        else {
            return Some(0.5);
        };
        if !raw_internal.iter().all(|value| value.is_finite()) {
            return Some(0.5);
        }

        let limited_internal =
            self.limit_intrinsic_state_against_previous(raw_internal, previous_internal);
        let previous_branches = self.vbic_nonlinear_branch_voltages(previous_internal);
        let raw_branches = self.vbic_nonlinear_branch_voltages(raw_internal);
        let limited_branches = self.vbic_nonlinear_branch_voltages(limited_internal);

        let mut scale: Value = 1.0;
        let mut engaged = false;
        for branch_scale in [
            Self::vbic_branch_limit_scale(
                previous_branches.vbei,
                raw_branches.vbei,
                limited_branches.vbei,
            ),
            Self::vbic_branch_limit_scale(
                previous_branches.vbex,
                raw_branches.vbex,
                limited_branches.vbex,
            ),
            Self::vbic_branch_limit_scale(
                previous_branches.vbci,
                raw_branches.vbci,
                limited_branches.vbci,
            ),
            Self::vbic_branch_limit_scale(
                previous_branches.vbcx,
                raw_branches.vbcx,
                limited_branches.vbcx,
            ),
            Self::vbic_branch_limit_scale(
                previous_branches.vbep,
                raw_branches.vbep,
                limited_branches.vbep,
            ),
            Self::vbic_branch_limit_scale(
                previous_branches.vbcp,
                raw_branches.vbcp,
                limited_branches.vbcp,
            ),
            if self.self_heating_enabled() {
                Self::vbic_branch_limit_scale(
                    previous_branches.vrth,
                    raw_branches.vrth,
                    limited_branches.vrth,
                )
            } else {
                None
            },
        ]
        .into_iter()
        .flatten()
        {
            if branch_scale + 1e-15 < 1.0 {
                engaged = true;
            }
            scale = scale.min(branch_scale);
        }

        engaged.then_some(scale.max(0.0))
    }

    /// Legacy Gummel-Poon analog of the VBIC external step-limit scale.
    ///
    /// Mirrors ngspice's per-iteration `pnjlim` discipline: predict where the
    /// internal junction voltages would land for a proposed external update,
    /// apply SPICE junction limiting against the previous iterate, and derive
    /// the largest external step fraction that respects the limited junction
    /// motion. This is what allows the transient Newton loop to take full
    /// node updates (no global trust region) without exponential-junction
    /// overshoot, matching ngspice's flat-MNA-plus-pnjlim behavior.
    pub(crate) fn legacy_external_step_limit_scale_from_state(
        &self,
        previous_external: [Value; EXTERNAL_DIM],
        previous_internal: [Value; INTERNAL_DIM],
        proposed_external: [Value; EXTERNAL_DIM],
    ) -> Option<Value> {
        if self.charge_model != BjtChargeModel::LegacyGummelPoon {
            return None;
        }

        let delta_external = [
            proposed_external[EXT_C] - previous_external[EXT_C],
            proposed_external[EXT_B] - previous_external[EXT_B],
            proposed_external[EXT_E] - previous_external[EXT_E],
            proposed_external[EXT_S] - previous_external[EXT_S],
        ];
        let max_delta = delta_external
            .iter()
            .map(|value| value.abs())
            .fold(0.0, Value::max);
        if !max_delta.is_finite() || max_delta <= 1e-15 {
            return None;
        }

        let Some(raw_internal) = self
            .predict_intrinsic_state_from_previous_external_bias_unlimited(
                previous_external,
                previous_internal,
                proposed_external,
            )
        else {
            return Some(0.5);
        };
        if !raw_internal.iter().all(|value| value.is_finite()) {
            return Some(0.5);
        }

        let limited_internal =
            self.limit_legacy_internal_state_to_previous(raw_internal, previous_internal);
        let previous_branches = self.legacy_nonlinear_branch_voltages(previous_internal);
        let raw_branches = self.legacy_nonlinear_branch_voltages(raw_internal);
        let limited_branches = self.legacy_nonlinear_branch_voltages(limited_internal);

        let mut scale: Value = 1.0;
        let mut engaged = false;
        for branch_scale in [
            Self::vbic_branch_limit_scale(
                previous_branches.vbe,
                raw_branches.vbe,
                limited_branches.vbe,
            ),
            Self::vbic_branch_limit_scale(
                previous_branches.vbc,
                raw_branches.vbc,
                limited_branches.vbc,
            ),
            Self::vbic_branch_limit_scale(
                previous_branches.vsub,
                raw_branches.vsub,
                limited_branches.vsub,
            ),
        ]
        .into_iter()
        .flatten()
        {
            if branch_scale + 1e-15 < 1.0 {
                engaged = true;
            }
            scale = scale.min(branch_scale);
        }

        engaged.then_some(scale.max(0.0))
    }

    /// Per-iteration junction-limiting scale for any BJT charge model.
    ///
    /// Dispatches to the legacy Gummel-Poon limiter; returns `None` when the
    /// proposed update needs no limiting. MNA-promoted VBIC devices run
    /// ngspice's pnjlim discipline on their own internal junctions inside
    /// `update`, so their external nodes take full Newton steps.
    pub(crate) fn junction_external_step_limit_scale_against_previous(
        &self,
        previous_external: [Value; EXTERNAL_DIM],
        proposed_external: [Value; EXTERNAL_DIM],
    ) -> Option<Value> {
        match self.charge_model {
            BjtChargeModel::Vbic => None,
            BjtChargeModel::LegacyGummelPoon => {
                let previous_internal = if self
                    .vbic_cached_external_matches(previous_external, 1e-12, 1e-9)
                {
                    self.internal_state_vector()
                } else {
                    let solved_previous = self.solve_intrinsic_terminal_state(
                        previous_external[EXT_C],
                        previous_external[EXT_B],
                        previous_external[EXT_E],
                        previous_external[EXT_S],
                    );
                    [
                        solved_previous.vcx,
                        solved_previous.vci,
                        solved_previous.vbx,
                        solved_previous.vbi,
                        solved_previous.vei,
                        solved_previous.vbp,
                        solved_previous.vsi,
                        solved_previous.vrth,
                    ]
                };
                self.legacy_external_step_limit_scale_from_state(
                    previous_external,
                    previous_internal,
                    proposed_external,
                )
            }
        }
    }

}
