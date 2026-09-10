//! BJT private charge solves and accepted-history predictors.

use super::*;

impl Engine {
    #[inline]
    pub(in crate::engine::transient) fn vbic_predictor_linear_branch_state(
        bjt: &crate::device::Bjt,
        external: [Value; BJT_EXTERNAL_STATE_DIM],
        internal: [Value; BJT_INTERNAL_STATE_DIM],
    ) -> VbicPredictorLinearBranchState {
        let polarity = match bjt.bjt_type {
            BjtType::Npn => 1.0,
            BjtType::Pnp => -1.0,
        };
        VbicPredictorLinearBranchState {
            vrcx: polarity * (external[BJT_EXT_C_INDEX] - internal[BJT_VCX_STATE_INDEX]),
            vrci: polarity * (internal[BJT_VCX_STATE_INDEX] - internal[BJT_VCI_STATE_INDEX]),
            vrbx: polarity * (external[1] - internal[BJT_VBX_STATE_INDEX]),
            vrbi: polarity * (internal[BJT_VBX_STATE_INDEX] - internal[BJT_VBI_STATE_INDEX]),
            vre: polarity * (external[BJT_EXT_E_INDEX] - internal[BJT_VEI_STATE_INDEX]),
            vrbp: polarity * (internal[BJT_VBP_STATE_INDEX] - internal[BJT_VCX_STATE_INDEX]),
            vrs: polarity * (external[3] - internal[BJT_VSI_STATE_INDEX]),
        }
    }

    #[inline]
    pub(in crate::engine::transient) fn vbic_external_from_linear_history(
        bjt: &crate::device::Bjt,
        internal: &[Value; BJT_INTERNAL_STATE_DIM],
        linear: &VbicPredictorLinearBranchState,
    ) -> [Value; BJT_EXTERNAL_STATE_DIM] {
        let polarity = match bjt.bjt_type {
            BjtType::Npn => 1.0,
            BjtType::Pnp => -1.0,
        };
        [
            internal[BJT_VCX_STATE_INDEX] + polarity * linear.vrcx,
            internal[BJT_VBX_STATE_INDEX] + polarity * linear.vrbx,
            internal[BJT_VEI_STATE_INDEX] + polarity * linear.vre,
            internal[BJT_VSI_STATE_INDEX] + polarity * linear.vrs,
        ]
    }

    fn solve_legacy_bjt_ngspice_transient_snapshot(
        bjt: &crate::device::Bjt,
        external: [Value; BJT_EXTERNAL_STATE_DIM],
        step: VbicChargeStep<'_>,
        predictor: VbicPredictorHistory<'_>,
        cached_snapshot: Option<BjtChargeSnapshot>,
    ) -> Option<BjtChargeSnapshot> {
        let cached_seed = cached_snapshot.map(|snapshot| snapshot.reduction.internal_voltages);
        let predicted_seed =
            Self::legacy_bjt_internal_seed_from_history(bjt, external, predictor, step.dt);
        let [vc, vb, ve, vs] = external;
        let live_seed = bjt.dynamic_internal_state_seed(vc, vb, ve, vs);
        [cached_seed, Some(predicted_seed), Some(live_seed)]
            .into_iter()
            .flatten()
            .find_map(|seed| {
                Self::solve_legacy_bjt_ngspice_transient_snapshot_from_seed(
                    bjt, external, step, seed,
                )
            })
    }

    #[inline]
    fn solve_legacy_bjt_ngspice_transient_snapshot_from_seed(
        bjt: &crate::device::Bjt,
        external: [Value; BJT_EXTERNAL_STATE_DIM],
        step: VbicChargeStep<'_>,
        seed: [Value; BJT_INTERNAL_STATE_DIM],
    ) -> Option<BjtChargeSnapshot> {
        let VbicChargeStep {
            coeff,
            dt,
            q_prev,
            q_prev_prev,
            cq_prev,
        } = step;
        let [vc, vb, ve, vs] = external;
        let mut snapshot = bjt.charge_snapshot_for_dynamic_state(vc, vb, ve, vs, seed);
        let mut linearization = Self::assemble_vbic_transient_linearization(
            bjt,
            &snapshot,
            VbicChargeStep {
                coeff,
                dt,
                q_prev,
                q_prev_prev,
                cq_prev,
            },
        )?;
        let residual = Self::vbic_internal_equation_residual(
            &linearization,
            &snapshot.reduction.external_voltages,
            &snapshot.reduction.internal_voltages,
        );
        let mut norm = Self::vbic_dynamic_static_core_residual_norm(&residual);
        for _ in 0..18 {
            let target_internal = Self::solve_vbic_static_core_from_linearization(
                &linearization,
                &snapshot.reduction.external_voltages,
                &snapshot.reduction.internal_voltages,
            )?;
            let current_internal = snapshot.reduction.internal_voltages;
            let max_delta = target_internal
                .iter()
                .zip(current_internal.iter())
                .take(BJT_STATIC_CORE_STATE_DIM)
                .map(|(target, current)| (target - current).abs())
                .fold(0.0_f64, Value::max);
            if !max_delta.is_finite() {
                return None;
            }
            // A small instance has small KCL currents even far from its
            // private voltage root. Certify the voltage correction from the
            // scaled linear solve, independently of AREA/M and row units.
            if max_delta <= 1e-12 {
                return Some(snapshot);
            }

            let mut accepted = None;
            let mut alpha = 1.0;
            for _ in 0..12 {
                let mut candidate_internal = current_internal;
                for idx in 0..BJT_STATIC_CORE_STATE_DIM {
                    candidate_internal[idx] = Self::interpolate_newton_value(
                        current_internal[idx],
                        target_internal[idx],
                        alpha,
                    );
                }
                let candidate_snapshot =
                    bjt.charge_snapshot_for_dynamic_state(vc, vb, ve, vs, candidate_internal);
                let Some(candidate_linearization) = Self::assemble_vbic_transient_linearization(
                    bjt,
                    &candidate_snapshot,
                    VbicChargeStep {
                        coeff,
                        dt,
                        q_prev,
                        q_prev_prev,
                        cq_prev,
                    },
                ) else {
                    alpha *= 0.5;
                    continue;
                };
                let candidate_residual = Self::vbic_internal_equation_residual(
                    &candidate_linearization,
                    &candidate_snapshot.reduction.external_voltages,
                    &candidate_snapshot.reduction.internal_voltages,
                );
                let candidate_norm =
                    Self::vbic_dynamic_static_core_residual_norm(&candidate_residual);

                if candidate_norm.is_finite() && candidate_norm <= norm * 0.8 {
                    accepted = Some((candidate_snapshot, candidate_linearization, candidate_norm));
                    break;
                }

                alpha *= 0.5;
            }

            let Some((next_snapshot, next_linearization, next_norm)) = accepted else {
                break;
            };
            snapshot = next_snapshot;
            linearization = next_linearization;
            norm = next_norm;
        }

        None
    }

    /// Resolve a legacy Gummel-Poon device's private charge state. Native
    /// VBIC devices are integrated on their promoted MNA nodes instead.
    pub(in crate::engine::transient) fn resolve_legacy_bjt_transient_snapshot(
        bjt: &crate::device::Bjt,
        external: [Value; BJT_EXTERNAL_STATE_DIM],
        step: VbicChargeStep<'_>,
        predictor: VbicPredictorHistory<'_>,
        cached_snapshot: Option<BjtChargeSnapshot>,
    ) -> Option<BjtChargeSnapshot> {
        if bjt.uses_vbic_dynamic_charges() {
            return None;
        }
        if !Self::legacy_bjt_ngspice_backend_enabled() {
            return Some(bjt.charge_snapshot(external[0], external[1], external[2], external[3]));
        }

        // Equal external voltages do not authenticate an internal charge
        // state: dt, coefficients or accepted history may have changed.
        // Reuse the cached voltage as a Newton seed, then check the root.
        Self::solve_legacy_bjt_ngspice_transient_snapshot(
            bjt,
            external,
            step,
            predictor,
            cached_snapshot,
        )
        .or_else(|| {
            let snapshot = bjt.charge_snapshot(external[0], external[1], external[2], external[3]);
            // A chargeless device legitimately has no dynamic companion.
            // An active charge solve must not fall back to its DC root.
            (!snapshot.branches.iter().any(BjtChargeBranch::is_active)).then_some(snapshot)
        })
    }

    #[inline]
    pub(in crate::engine::transient) fn vbic_dynamic_internal_seed_from_linear_history(
        bjt: &crate::device::Bjt,
        target_external: [Value; BJT_EXTERNAL_STATE_DIM],
        history_internal_prev: &[Value; BJT_INTERNAL_STATE_DIM],
        history_linear_prev: &VbicPredictorLinearBranchState,
    ) -> Option<[Value; BJT_INTERNAL_STATE_DIM]> {
        Self::vbic_dynamic_internal_seed_from_predicted_linear_history(
            bjt,
            target_external,
            history_internal_prev,
            history_linear_prev,
            None,
            0.0,
            0.0,
        )
    }

    #[inline]
    pub(in crate::engine::transient) fn vbic_predictor_linear_branch_state_is_finite(
        linear: &VbicPredictorLinearBranchState,
    ) -> bool {
        [
            linear.vrcx,
            linear.vrci,
            linear.vrbx,
            linear.vrbi,
            linear.vre,
            linear.vrbp,
            linear.vrs,
        ]
        .iter()
        .all(|value| value.is_finite())
    }

    #[inline]
    pub(in crate::engine::transient) fn predict_vbic_linear_branch_state_from_history(
        history_linear_prev: &VbicPredictorLinearBranchState,
        history_linear_prev_prev: Option<&VbicPredictorLinearBranchState>,
        dt: Value,
        previous_dt: Value,
    ) -> VbicPredictorLinearBranchState {
        let predict_component = |previous: Value, previous_previous: Option<Value>| {
            Self::predict_transient_history_value(previous, previous_previous, dt, previous_dt)
        };

        VbicPredictorLinearBranchState {
            vrcx: predict_component(
                history_linear_prev.vrcx,
                history_linear_prev_prev.map(|prev_prev| prev_prev.vrcx),
            ),
            vrci: predict_component(
                history_linear_prev.vrci,
                history_linear_prev_prev.map(|prev_prev| prev_prev.vrci),
            ),
            vrbx: predict_component(
                history_linear_prev.vrbx,
                history_linear_prev_prev.map(|prev_prev| prev_prev.vrbx),
            ),
            vrbi: predict_component(
                history_linear_prev.vrbi,
                history_linear_prev_prev.map(|prev_prev| prev_prev.vrbi),
            ),
            vre: predict_component(
                history_linear_prev.vre,
                history_linear_prev_prev.map(|prev_prev| prev_prev.vre),
            ),
            vrbp: predict_component(
                history_linear_prev.vrbp,
                history_linear_prev_prev.map(|prev_prev| prev_prev.vrbp),
            ),
            vrs: predict_component(
                history_linear_prev.vrs,
                history_linear_prev_prev.map(|prev_prev| prev_prev.vrs),
            ),
        }
    }

    #[inline]
    pub(in crate::engine::transient) fn vbic_dynamic_internal_seed_from_predicted_linear_history(
        bjt: &crate::device::Bjt,
        target_external: [Value; BJT_EXTERNAL_STATE_DIM],
        history_internal_prev: &[Value; BJT_INTERNAL_STATE_DIM],
        history_linear_prev: &VbicPredictorLinearBranchState,
        history_linear_prev_prev: Option<&VbicPredictorLinearBranchState>,
        dt: Value,
        previous_dt: Value,
    ) -> Option<[Value; BJT_INTERNAL_STATE_DIM]> {
        let predicted_linear = Self::predict_vbic_linear_branch_state_from_history(
            history_linear_prev,
            history_linear_prev_prev,
            dt,
            previous_dt,
        );
        if !Self::vbic_predictor_linear_branch_state_is_finite(&predicted_linear) {
            return None;
        }

        let polarity = match bjt.bjt_type {
            BjtType::Npn => 1.0,
            BjtType::Pnp => -1.0,
        };

        let mut seed_internal = *history_internal_prev;
        seed_internal[BJT_VCX_STATE_INDEX] =
            target_external[BJT_EXT_C_INDEX] - polarity * predicted_linear.vrcx;
        seed_internal[BJT_VCI_STATE_INDEX] =
            seed_internal[BJT_VCX_STATE_INDEX] - polarity * predicted_linear.vrci;
        seed_internal[BJT_VBX_STATE_INDEX] =
            target_external[BJT_EXT_B_INDEX] - polarity * predicted_linear.vrbx;
        seed_internal[BJT_VBI_STATE_INDEX] =
            seed_internal[BJT_VBX_STATE_INDEX] - polarity * predicted_linear.vrbi;
        seed_internal[BJT_VEI_STATE_INDEX] =
            target_external[BJT_EXT_E_INDEX] - polarity * predicted_linear.vre;
        seed_internal[BJT_VBP_STATE_INDEX] =
            seed_internal[BJT_VCX_STATE_INDEX] + polarity * predicted_linear.vrbp;
        seed_internal[BJT_VSI_STATE_INDEX] =
            target_external[BJT_EXT_S_INDEX] - polarity * predicted_linear.vrs;

        seed_internal
            .iter()
            .take(BJT_THERMAL_STATE_INDEX)
            .all(|value| value.is_finite())
            .then_some(seed_internal)
    }

    #[inline]
    fn legacy_bjt_internal_seed_from_history(
        bjt: &crate::device::Bjt,
        target_external: [Value; BJT_EXTERNAL_STATE_DIM],
        history: VbicPredictorHistory<'_>,
        dt: Value,
    ) -> [Value; BJT_INTERNAL_STATE_DIM] {
        let VbicPredictorHistory {
            internal_prev: history_internal_prev,
            linear_prev: history_linear_prev,
            linear_prev_prev: history_linear_prev_prev,
            previous_dt,
        } = history;
        let [vc, vb, ve, vs] = target_external;
        let live_seed = bjt.dynamic_internal_state_seed(vc, vb, ve, vs);
        let Some(history_internal_prev) = history_internal_prev else {
            return live_seed;
        };
        if !history_internal_prev.iter().all(|value| value.is_finite()) {
            return live_seed;
        }
        let history_linear_prev = history_linear_prev
            .filter(|linear| Self::vbic_predictor_linear_branch_state_is_finite(linear));
        let history_linear_prev_prev = history_linear_prev_prev
            .filter(|linear| Self::vbic_predictor_linear_branch_state_is_finite(linear));

        // Reconstruct private nodes from accepted branch-voltage history,
        // then refine that seed against this step's charge equations.
        history_linear_prev
            .and_then(|history_linear_prev| {
                Self::vbic_dynamic_internal_seed_from_predicted_linear_history(
                    bjt,
                    target_external,
                    history_internal_prev,
                    history_linear_prev,
                    history_linear_prev_prev,
                    dt,
                    previous_dt,
                )
            })
            .or_else(|| {
                history_linear_prev.and_then(|history_linear_prev| {
                    Self::vbic_dynamic_internal_seed_from_linear_history(
                        bjt,
                        target_external,
                        history_internal_prev,
                        history_linear_prev,
                    )
                })
            })
            .unwrap_or(live_seed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn private_rc_bjt(m: Value, polarity: Value, xyce: bool) -> crate::device::Bjt {
        let mut bjt = if polarity > 0.0 {
            crate::device::Bjt::new_npn("q".into(), 1, 2, 0)
        } else {
            crate::device::Bjt::new_pnp("q".into(), 1, 2, 0)
        }
        .with_params(
            &[
                ("IS".into(), 0.0),
                ("RB".into(), 5e3),
                ("RBM".into(), 1e3),
                ("CJE".into(), 3e-12),
                ("MJE".into(), 0.0),
            ]
            .into_iter()
            .collect(),
        )
        .with_instance_params(&[("M".into(), m)]);
        bjt.set_xyce_compatibility(xyce);
        bjt.set_junction_gmin(0.0);
        bjt
    }

    fn resolve_rc_snapshot(
        bjt: &crate::device::Bjt,
        polarity: Value,
        step: VbicChargeStep<'_>,
        cached: Option<BjtChargeSnapshot>,
    ) -> BjtChargeSnapshot {
        Engine::resolve_legacy_bjt_transient_snapshot(
            bjt,
            [0.0, polarity * 0.001, 0.0, 0.0],
            step,
            VbicPredictorHistory {
                internal_prev: None,
                linear_prev: None,
                linear_prev_prev: None,
                previous_dt: 0.0,
            },
            cached,
        )
        .expect("private RC state must solve")
    }

    #[test]
    fn private_bjt_transient_internal_voltage_preserves_instance_scaling() {
        let zero = [0.0; BJT_DYNAMIC_CHARGE_COUNT];
        for xyce in [false, true] {
            for polarity in [1.0, -1.0] {
                for m in [1.0, 1e-20, 1e-200] {
                    let bjt = private_rc_bjt(m, polarity, xyce);
                    for coeff in [
                        CompanionCoefficients::backward_euler(),
                        CompanionCoefficients::trapezoidal(),
                    ] {
                        let snapshot = resolve_rc_snapshot(
                            &bjt,
                            polarity,
                            VbicChargeStep {
                                coeff: &coeff,
                                dt: 15e-9,
                                q_prev: &zero,
                                q_prev_prev: &zero,
                                cq_prev: &zero,
                            },
                            None,
                        );
                        // tau=RC=15ns: one step from zero charge is
                        // Vin/(1+ag0*tau), independently of instance size.
                        let expected = polarity * 0.001 / (1.0 + coeff.coeff_g);
                        let actual = snapshot.reduction.internal_voltages[BJT_VBI_STATE_INDEX];
                        assert!(
                            (actual - expected).abs() < 2e-12,
                            "xyce={xyce} polarity={polarity} M={m:e} {coeff:?}: {actual:e} vs {expected:e}"
                        );
                    }
                }
            }
        }
    }

    #[test]
    fn private_bjt_cached_voltage_does_not_authenticate_charge_step() {
        let bjt = private_rc_bjt(1.0, 1.0, false);
        let zero = [0.0; BJT_DYNAMIC_CHARGE_COUNT];
        let be = CompanionCoefficients::backward_euler();
        let trap = CompanionCoefficients::trapezoidal();
        let step = VbicChargeStep {
            coeff: &be,
            dt: 15e-9,
            q_prev: &zero,
            q_prev_prev: &zero,
            cq_prev: &zero,
        };
        let cached = resolve_rc_snapshot(&bjt, 1.0, step, None);
        let mut previous = zero;
        previous[BJT_QBE_BRANCH_INDEX] = 3e-12 * 0.00075;
        for (changed, expected) in [
            (
                VbicChargeStep {
                    coeff: &trap,
                    ..step
                },
                0.001 / 3.0,
            ),
            (VbicChargeStep { dt: 30e-9, ..step }, 0.002 / 3.0),
            (
                VbicChargeStep {
                    q_prev: &previous,
                    ..step
                },
                0.000875,
            ),
        ] {
            let actual = resolve_rc_snapshot(&bjt, 1.0, changed, Some(cached))
                .reduction
                .internal_voltages[BJT_VBI_STATE_INDEX];
            assert!(
                (actual - expected).abs() < 2e-12,
                "{actual:e} vs {expected:e}"
            );
        }
    }
}
