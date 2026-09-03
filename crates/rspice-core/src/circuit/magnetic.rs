//! Transient stamping for magnetically coupled and nonlinear inductors.
//!
//! Coupled inductor pairs and multi-winding transformers need their mutual
//! terms restamped each timestep, and Jiles-Atherton cores need their
//! effective inductance refreshed from the current solution before companion
//! stamping so hysteresis state reaches the MNA coefficients.

use super::*;
use crate::device::passive::{XyceCoreStep, XyceCoreTrial};

#[inline]
fn node_voltage(solution: &[Value], node_pos: NodeId, node_neg: NodeId) -> Value {
    let pos = if node_pos == 0 {
        0.0
    } else {
        solution.get(node_pos - 1).copied().unwrap_or(0.0)
    };
    let neg = if node_neg == 0 {
        0.0
    } else {
        solution.get(node_neg - 1).copied().unwrap_or(0.0)
    };
    pos - neg
}

/// How the Xyce core companion stamp treats the step it is writing: whether
/// the integrator is in its one-step mode, whether that step is second order,
/// and whether the magnetization variable may advance. The three booleans are
/// read together and are easy to transpose as bare arguments.
#[derive(Clone, Copy)]
pub(crate) struct XyceCoreCompanionMode {
    pub one_step: bool,
    pub one_step_order2: bool,
    pub advance_magvar_update: bool,
}

impl CircuitData {
    /// Stamp ordinary transient inductor companions while leaving Xyce Core
    /// branches to their complete nonlinear DAE stamp.
    pub fn stamp_transient_inductor_companions(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        dt: Value,
        coeff: &CompanionCoefficients,
        num_nodes: usize,
    ) {
        let core_bindings = &self.jiles_atherton_inductors;
        let grouped_indices = self
            .xyce_core_groups
            .iter()
            .flat_map(|group| group.windings.iter().map(|winding| winding.inductor_index))
            .collect::<std::collections::HashSet<_>>();
        for index in 0..self.inductors.names.len() {
            let np = self.inductors.node_pos[index];
            let nn = self.inductors.node_neg[index];
            let branch = num_nodes + self.inductors.branch_indices[index];
            let is_core = core_bindings
                .iter()
                .any(|binding| binding.inductor_index == index && binding.device.is_xyce_core())
                || grouped_indices.contains(&index);
            // Core rows own the constitutive branch equation, but their MNA
            // KCL incidence entries are still needed here.  Do not
            // stamp the authored L-card companion (its 1/dt terms can be
            // many orders larger than Core's geometry-only Q coefficient).
            if is_core {
                if np > 0 {
                    matrix.add(np - 1, branch - 1, 1.0);
                }
                if nn > 0 {
                    matrix.add(nn - 1, branch - 1, -1.0);
                }
                continue;
            }

            let r_eq = coeff.inductor_req(self.inductors.inductances[index], dt);
            let v_eq = coeff.inductor_veq(
                self.inductors.inductances[index],
                dt,
                self.inductors.i_prev[index],
                self.inductors.i_prev_prev[index],
                self.inductors.v_prev[index],
            );
            if np > 0 {
                matrix.add(branch - 1, np - 1, 1.0);
                matrix.add(np - 1, branch - 1, 1.0);
            }
            if nn > 0 {
                matrix.add(branch - 1, nn - 1, -1.0);
                matrix.add(nn - 1, branch - 1, -1.0);
            }
            matrix.add(branch - 1, branch - 1, -r_eq);
            rhs[branch - 1] = -v_eq;
        }
    }

    /// Whether the circuit contains a single-winding Xyce nonlinear Core.
    /// Core branches are nonlinear transient participants even though they
    /// own their accepted-state lifecycle outside `update_nonlinear`.
    pub fn has_xyce_core_inductors(&self) -> bool {
        self.jiles_atherton_inductors
            .iter()
            .any(|binding| binding.device.is_xyce_core())
            || !self.xyce_core_groups.is_empty()
    }

    /// Whether every current Xyce Core Newton endpoint, including the
    /// eliminated LEVEL=1 hidden M equation, satisfies its scaled residual.
    pub(crate) fn xyce_core_trial_converged(&self) -> bool {
        self.jiles_atherton_inductors
            .iter()
            .filter(|binding| binding.device.is_xyce_core())
            .all(|binding| binding.device.xyce_core_trial_converged())
            && self
                .xyce_core_groups
                .iter()
                .filter(|group| group.device.is_xyce_core())
                .all(|group| group.device.xyce_core_trial_converged())
    }

    /// Whether every LEVEL=2 Xyce Core trial satisfies MutIndNonLin2's
    /// native forward-Euler magnetization limiter.
    ///
    /// Xyce stores this limiter in each device's inherited `origFlag`, which
    /// participates in the device convergence status when
    /// `ENFORCEDEVICECONV` is enabled (the runtime default). Keep the LEVEL=1
    /// hidden M/R residual policy separate: callers that need the native
    /// LEVEL=2 veto must not accidentally enable those reduced-equation checks
    /// for a mixed LEVEL=1/LEVEL=2 circuit.
    pub(crate) fn xyce_core_level2_trial_converged(&self) -> bool {
        self.jiles_atherton_inductors
            .iter()
            .filter(|binding| binding.device.is_xyce_core_level2())
            .all(|binding| binding.device.xyce_core_trial_converged())
            && self
                .xyce_core_groups
                .iter()
                .filter(|group| group.device.is_xyce_core_level2())
                .all(|group| group.device.xyce_core_trial_converged())
    }

    /// Whether every standalone inductor in the circuit is represented by a
    /// Xyce Core binding.  OneStep can split this topology into its static
    /// Core `F` and constant-charge `Q` terms; ordinary and mutually coupled
    /// inductors still require their native history mapping.
    pub fn has_only_xyce_core_inductors(&self) -> bool {
        let grouped_winding_count: usize = self
            .xyce_core_groups
            .iter()
            .map(|group| group.windings.len())
            .sum();
        !self.inductors.names.is_empty()
            && self.inductors.names.len()
                == self.jiles_atherton_inductors.len() + grouped_winding_count
            && self
                .jiles_atherton_inductors
                .iter()
                .all(|binding| binding.device.is_xyce_core())
            && self
                .xyce_core_groups
                .iter()
                .all(|group| group.device.is_xyce_core())
    }

    /// Whether a shared LEVEL=1 Core has a geometry-only branch coefficient
    /// below the scale at which the direct DampedNewton electrical solve can
    /// resolve its constitutive endpoint. Xyce's hidden magnetic equations
    /// remain physical at this scale, but the electrical Schur complement is
    /// effectively rank deficient; correction-form Newton is the stable
    /// source-equivalent solve for that topology.
    pub(crate) fn has_xyce_core_shared_level1_ill_conditioned(&self) -> bool {
        const DAMPED_MIN_VACUUM_INDUCTANCE: Value = 1.0e-10;

        self.xyce_core_groups.iter().any(|group| {
            group.device.is_xyce_core()
                && !group.device.is_xyce_core_level2()
                && group.windings.len() > 1
                && group.windings.iter().any(|winding| {
                    let vacuum_inductance = group.device.xyce_core_vacuum_mutual_inductance(
                        winding.turns,
                        winding.turns,
                        1.0,
                    );
                    vacuum_inductance.is_finite()
                        && vacuum_inductance > 0.0
                        && vacuum_inductance < DAMPED_MIN_VACUUM_INDUCTANCE
                })
        })
    }

    /// Whether any Xyce Core in the deck is LEVEL=2.
    ///
    /// GMIN continuation deforms only the nodal equations, so a deck whose
    /// Cores are all LEVEL=1 has nothing for the deformation to regularize.
    /// A LEVEL=2 Core keeps the general rescue path because its constitutive
    /// trial can still require globalization, and that is true of a single
    /// winding as much as a coupled set — unlike
    /// [`Self::has_xyce_core_shared_level2`], which asks the narrower question
    /// of whether the stabilized Picard Jacobian applies.
    pub(crate) fn has_xyce_core_level2(&self) -> bool {
        self.jiles_atherton_inductors
            .iter()
            .any(|binding| binding.device.is_xyce_core_level2())
            || self
                .xyce_core_groups
                .iter()
                .any(|group| group.device.is_xyce_core() && group.device.is_xyce_core_level2())
    }

    /// Whether a shared winding group uses MutIndNonLin2's LEVEL=2 state
    /// update.  LEVEL=2 additionally requires the correction-form inductor
    /// residual; LEVEL=1 keeps the direct branch rows on the ordinary Newton
    /// loop after DampedNewton is disabled for the shared topology.
    pub(crate) fn has_xyce_core_shared_level2(&self) -> bool {
        self.xyce_core_groups.iter().any(|group| {
            group.device.is_xyce_core()
                && group.device.is_xyce_core_level2()
                && group.windings.len() > 1
        })
    }

    /// Return the physical residual floor for Xyce Core electrical branch
    /// equations.
    ///
    /// Xyce's global transient `RHSTOL` is intentionally permissive, but a
    /// Core branch is a constitutive closure equation whose Q/F cancellation
    /// directly appears as a winding voltage.  Letting that row stop at the
    /// global RHS norm can therefore leave a finite voltage on an otherwise
    /// ideal coupled winding.  LEVEL=2 is solved in the full electrical
    /// correction system and reaches the direct double-precision floor.  A
    /// shared LEVEL=1 Core is Schur-reduced through hidden M/R coordinates;
    /// its deliberately scaled electrical row has a larger, model-conditioned
    /// round-off floor when the vacuum coefficient is near rank deficiency.
    pub(crate) fn xyce_core_branch_residual_tolerance(&self) -> Value {
        const DIRECT_LEVEL2_FLOOR: Value = 1.0e-13;
        const REDUCED_LEVEL1_FLOOR: Value = 1.0e-11;

        if self.has_xyce_core_shared_level1_ill_conditioned() {
            REDUCED_LEVEL1_FLOOR
        } else {
            DIRECT_LEVEL2_FLOOR
        }
    }

    /// Restamp the single-winding Xyce Core branch equations with the pure
    /// constitutive endpoint evaluated at the current Newton iterate.
    ///
    /// A Core's DAE has a constant vacuum charge coefficient and a nonlinear
    /// static factor `mid(P)`. Treating that factor as an accepted-step linear
    /// inductance loses the coupled Newton solve, especially when the
    /// constitutive mid-factor is negative near zero field. This routine
    /// replaces only the branch-row linearization; KCL rows remain the normal
    /// MNA inductor rows.
    pub(crate) fn stamp_xyce_core_transient_companion(
        &mut self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        solution: &[Value],
        dt: Value,
        coeff: &CompanionCoefficients,
        mode: XyceCoreCompanionMode,
    ) {
        let XyceCoreCompanionMode {
            one_step,
            one_step_order2,
            advance_magvar_update,
        } = mode;
        // This status belongs to the current Newton assembly only.  Any
        // constitutive failure below must make the candidate non-converged;
        // do not leave the generic inductor companion as an accidental
        // substitute for the Xyce Core DAE.
        self.xyce_core_trial_invalid = false;
        self.xyce_core_transient_residuals.clear();
        let hidden_base = self.num_nodes + self.num_branches;
        // The hidden-M residual is assembled in integrated form,
        // `M-M_old-(dt/Path)P*R`.  Scale that row by Xyce's m-equation
        // coefficient directly.  Multiplying the equivalent DAE row by `dt`
        // leaves the Newton solution unchanged while avoiding catastrophic
        // cancellation in the raw residual norm at very small timesteps.
        for binding in &mut self.jiles_atherton_inductors {
            if !binding.device.is_xyce_core() {
                continue;
            }
            let index = binding.inductor_index;
            let Some(&i_prev) = self.inductors.i_prev.get(index) else {
                continue;
            };
            let i_prev_prev = self
                .inductors
                .i_prev_prev
                .get(index)
                .copied()
                .unwrap_or(i_prev);
            let v_prev = self.inductors.v_prev.get(index).copied().unwrap_or(0.0);
            let branch = self.num_nodes + self.inductors.branch_indices[index];
            let hidden_branch = binding.hidden_m_slot.map(|slot| hidden_base + slot + 1);
            let hidden_rate_branch = binding.hidden_r_slot.map(|slot| hidden_base + slot + 1);
            // Xyce exposes independent variable/equation scaling knobs for
            // the hidden M/R rows.  Keep the fixed Rust coordinate scales for
            // conditioning.  Variable scales cancel in the physical reduced
            // equations; equation scales are applied to the corresponding
            // rows below.
            let m_eq_scaling = binding.device.xyce_core_m_eq_scaling();
            let r_eq_scaling = binding.device.xyce_core_r_eq_scaling();
            let m_eq_scaling = if m_eq_scaling.is_finite() && m_eq_scaling > 0.0 {
                m_eq_scaling
            } else {
                1.0
            };
            let r_eq_scaling = if r_eq_scaling.is_finite() && r_eq_scaling > 0.0 {
                r_eq_scaling
            } else {
                1.0
            };
            let hidden_m_eq_scale = m_eq_scaling;
            let current = solution.get(branch - 1).copied().unwrap_or(i_prev);
            let voltage = if self.inductors.node_pos[index] == 0 {
                0.0
            } else {
                solution
                    .get(self.inductors.node_pos[index] - 1)
                    .copied()
                    .unwrap_or(0.0)
            } - if self.inductors.node_neg[index] == 0 {
                0.0
            } else {
                solution
                    .get(self.inductors.node_neg[index] - 1)
                    .copied()
                    .unwrap_or(0.0)
            };
            let nominal = binding.device.nominal_inductance();
            if !nominal.is_finite() || nominal <= 0.0 || !dt.is_finite() || dt <= 0.0 {
                continue;
            }

            // Xyce carries the accepted constitutive factor into the next
            // transient history evaluation.  The generic inductance slot is
            // refreshed after commit for charge-companion cancellation and
            // may be recomputed from a different magnetic endpoint near a
            // turning point; it is therefore not authoritative for F-history.
            let accepted_mid = binding.device.xyce_core_accepted_mid();
            let previous_mid = if accepted_mid.is_finite() && accepted_mid.abs() > 1.0e-12 {
                accepted_mid
            } else {
                1.0
            };
            let static_scale = if one_step_order2 { 0.5 } else { 1.0 };
            let carried_mag_update = binding.device.xyce_core_mag_update();
            let m_var_scaling = binding.device.xyce_core_m_var_scaling();
            let r_var_scaling = binding.device.xyce_core_r_var_scaling();
            let hidden_m = binding
                .hidden_m_slot
                .and_then(|slot| solution.get(hidden_base + slot).copied())
                .map(|value| value * m_var_scaling)
                .filter(|value| value.is_finite())
                .unwrap_or_else(|| binding.device.magnetization());
            let hidden_rate = binding
                .hidden_r_slot
                .and_then(|slot| solution.get(hidden_base + slot).copied())
                .map(|value| value * r_var_scaling)
                .filter(|value| value.is_finite())
                .unwrap_or_else(|| binding.device.xyce_core_level1_rate_debug());
            // OneStep uses the physical DAE charge difference directly:
            // `(Q_{n+1}-Q_n)/dt`. The ordinary trapezoidal companion has a
            // `2/dt` conductance, so its coefficient must not leak into the
            // Core's order-2 DAE row.
            let charge_coeff = if one_step { 1.0 } else { coeff.coeff_g };
            let residual = |trial_current: Value,
                            trial_voltage: Value,
                            cached_trial: Option<XyceCoreTrial>| {
                let trial = if let Some(trial) = cached_trial {
                    trial
                } else if !binding.device.is_xyce_core_level2() {
                    let trial_happ = trial_current
                        * binding
                            .device
                            .xyce_core_happ_slope_for_turns(binding.device.n_turns_for_xyce_core());
                    let old_happ = binding
                        .device
                        .xyce_core_happ_from_current(binding.device.current_value());
                    binding
                        .device
                        .xyce_core_level1_trial_at_magnetization_and_rate(
                            XyceCoreStep {
                                happ: trial_happ,
                                delta_happ: trial_happ - old_happ,
                                voltage: trial_voltage,
                                dt,
                                one_step_order2,
                            },
                            hidden_m,
                            hidden_rate,
                        )?
                } else {
                    binding.device.xyce_core_trial_with_update(
                        trial_current,
                        trial_voltage,
                        carried_mag_update,
                    )?
                };
                // MutIndNonLin2 assembles the DAE charge vector first and
                // subtracts its accepted Q history (`Q_{n+1}-Q_n`) in the
                // time integrator.  Form those products before taking the
                // difference; factoring the constant inductance as
                // `L*(I_{n+1}-I_n)` is algebraically equivalent but changes
                // the low bits at the Core's near-singular reversals.
                let q_current = binding.device.xyce_core_q_from_current(trial_current);
                let q_previous = binding.device.xyce_core_q_history();
                let mut charge_difference = charge_coeff * (q_current - q_previous);
                if !one_step && coeff.needs_two_history {
                    let q_previous_previous = nominal * i_prev_prev;
                    charge_difference +=
                        coeff.coeff_v_n_minus_1 * (q_previous - q_previous_previous);
                }
                let charge_derivative = if one_step {
                    // OneStep's Epetra assembly scales the already formed
                    // Q difference by its precomputed reciprocal timestep.
                    // Preserve that operation order; direct division is
                    // algebraically equivalent but changes Newton residual
                    // rounding near a constitutive cancellation.
                    (1.0 / dt) * charge_difference
                } else {
                    charge_difference / dt
                };
                let previous_static_voltage =
                    if previous_mid.is_finite() && previous_mid.abs() > 1.0e-12 {
                        v_prev / previous_mid
                    } else {
                        0.0
                    };
                let history = if one_step {
                    // The transient assembler supplies +1/2(F_n-B_n) from
                    // its accepted static-history snapshot.  Keep the Core
                    // branch's local history empty so that term is not
                    // counted twice.
                    0.0
                } else if coeff.coeff_i_n != 0.0 {
                    coeff.coeff_i_n * previous_static_voltage
                } else {
                    0.0
                };
                if !trial.mid.is_finite() {
                    return None;
                }
                // MutIndNonLin contributes Q=L0*I and F=-V/mid to the DAE.
                // Keep the branch residual in that source-equivalent
                // orientation so its Jacobian retains the same conditioning
                // as Xyce's device row near a constitutive zero crossing.
                let f0 = if one_step {
                    let static_branch = static_scale * (-(trial_voltage / trial.mid));
                    charge_derivative + static_branch + history
                } else {
                    // MutIndNonLin2's native transient row is also expressed
                    // as dQ/dt + F, with F=-V/mid.  Keeping this orientation
                    // (rather than its exact negative) preserves Xyce's
                    // branch-row signs and pivoting at a constitutive zero.
                    let static_branch = static_scale * trial_voltage / trial.mid;
                    charge_derivative - static_branch - history
                };
                Some((f0, trial))
            };
            let cached_trial = (!advance_magvar_update && binding.device.is_xyce_core_level2())
                .then(|| binding.device.xyce_core_cached_trial(current, voltage))
                .flatten();
            let Some((f0, trial)) = residual(current, voltage, cached_trial) else {
                self.xyce_core_trial_invalid = true;
                continue;
            };
            if f0.is_finite() {
                self.xyce_core_transient_residuals.push((branch - 1, f0));
            }
            if !f0.is_finite() {
                self.xyce_core_trial_invalid = true;
                continue;
            }
            if advance_magvar_update {
                binding
                    .device
                    .cache_xyce_core_trial(current, voltage, trial);
            } else {
                binding
                    .device
                    .cache_xyce_core_trial_endpoint(current, voltage, trial);
            }
            let jacobian_magnetization = trial.latest_magnetization;
            let current_happ_slope = binding
                .device
                .xyce_core_happ_slope_for_turns(binding.device.n_turns_for_xyce_core());
            let hidden_partials = if !binding.device.is_xyce_core_level2() {
                binding
                    .device
                    .xyce_core_level1_hidden_partials(trial, voltage, dt, one_step_order2)
            } else {
                None
            };
            let d_mid_d_current = if binding.device.is_xyce_core_level2() {
                // Keep the branch derivative in Xyce's native
                // `(1-gap/path)*dP_dI` form. Reconstructing `dP_dI` from
                // this value and multiplying by the geometry factor again
                // introduces an avoidable divide/multiply round trip.
                binding
                    .device
                    .xyce_core_dmid_d_current(current, voltage, jacobian_magnetization)
                    .unwrap_or(Value::NAN)
            } else {
                // With LEVEL=1 the hidden magnetization is an independent
                // Newton unknown.  The electrical branch row therefore uses
                // the constitutive derivative at fixed hidden M; the hidden
                // row carries the separate M/Happ coupling.
                binding
                    .device
                    .xyce_core_dmid_d_happ(
                        trial.applied_field,
                        voltage,
                        jacobian_magnetization,
                        current_happ_slope,
                    )
                    .unwrap_or(Value::NAN)
            };
            if !d_mid_d_current.is_finite()
                || (binding.device.has_xyce_core_m_equation() && hidden_partials.is_none())
            {
                self.xyce_core_trial_invalid = true;
                continue;
            }
            let mid = trial.mid;
            // MutIndNonLin (CORE LEVEL=1) also linearizes the adaptive
            // voltage-direction factor tanh(DELVSCALING*V/maxVoltageDrop).
            // LEVEL=2 intentionally returns None here because Xyce's
            // MutIndNonLin2 loadDAEdFdx omits dP/dV.  Keep the voltage term
            // in the same residual orientation as the current derivative so
            // the Newton matrix remains the exact derivative of V/mid.
            let d_mid_d_voltage = binding
                .device
                .xyce_core_dmid_d_voltage(trial.applied_field, voltage, jacobian_magnetization)
                .unwrap_or(0.0);
            let d_static_voltage_direct = if binding.device.is_xyce_core_level2() && one_step {
                let d_f_voltage = -1.0 / mid;
                static_scale * d_f_voltage
            } else {
                -static_scale / mid + static_scale * voltage * d_mid_d_voltage / (mid * mid)
            };
            let d_current_direct = if binding.device.is_xyce_core_level2() && one_step {
                let d_f_current = (voltage * d_mid_d_current) / (mid * mid);
                let fterm = static_scale * d_f_current;
                let qterm = (charge_coeff * (1.0 / dt)) * nominal;
                qterm + fterm
            } else {
                static_scale * voltage * d_mid_d_current / (mid * mid) + charge_coeff * nominal / dt
            };
            let (d_current, d_voltage) = (d_current_direct, d_static_voltage_direct);
            let mut hidden_linearized = 0.0;
            if let Some(hidden_branch) = hidden_branch {
                let fixed_mid_m = binding
                    .device
                    .xyce_core_dmid_d_magnetization(
                        trial.applied_field,
                        voltage,
                        jacobian_magnetization,
                    )
                    .unwrap_or(Value::NAN);
                let d_m = static_scale * voltage * fixed_mid_m / (mid * mid);
                if !d_m.is_finite() {
                    self.xyce_core_trial_invalid = true;
                    continue;
                }
                matrix.add(branch - 1, hidden_branch - 1, d_m * m_var_scaling);
                hidden_linearized = d_m * hidden_m;
            }
            if !d_current.is_finite() || !d_voltage.is_finite() {
                self.xyce_core_trial_invalid = true;
                continue;
            }

            let desired_rhs = -f0 + d_current * current + d_voltage * voltage + hidden_linearized;
            if self.inductors.node_pos[index] > 0 {
                matrix.add(
                    branch - 1,
                    self.inductors.node_pos[index] - 1,
                    // The Core device owns the complete branch row.  The
                    // generic companion pass contributes only the node-row
                    // KCL incidence above, so stamp Xyce's `dF/dV` directly
                    // instead of adding and then cancelling an artificial
                    // unit incidence term.  Besides matching the native
                    // MNA partition, this avoids losing a low bit when
                    // `d_voltage` is tiny near a constitutive reversal.
                    d_voltage,
                );
            }
            if self.inductors.node_neg[index] > 0 {
                matrix.add(branch - 1, self.inductors.node_neg[index] - 1, -d_voltage);
            }
            // Core branches own their complete DAE row.  The transient
            // companion pass contributes only the MNA incidence entries;
            // avoid adding and then cancelling the authored L-card
            // companion, whose 1/dt terms can dwarf the geometry-only Q row.
            matrix.add(branch - 1, branch - 1, d_current);
            rhs[branch - 1] += desired_rhs;
            if let Some(hidden_branch) = hidden_branch {
                if let Some((g_m, g_happ, g_voltage, g_rate)) = hidden_partials {
                    if !g_m.is_finite()
                        || !g_happ.is_finite()
                        || !g_voltage.is_finite()
                        || !g_rate.is_finite()
                    {
                        self.xyce_core_trial_invalid = true;
                        continue;
                    }
                    matrix.add(
                        hidden_branch - 1,
                        hidden_branch - 1,
                        g_m * m_var_scaling * hidden_m_eq_scale,
                    );
                    if let Some(hidden_rate_branch) = hidden_rate_branch {
                        matrix.add(
                            hidden_branch - 1,
                            hidden_rate_branch - 1,
                            g_rate * r_var_scaling * hidden_m_eq_scale,
                        );
                    }
                    matrix.add(
                        hidden_branch - 1,
                        branch - 1,
                        g_happ * current_happ_slope * hidden_m_eq_scale,
                    );
                    if self.inductors.node_pos[index] > 0 {
                        matrix.add(
                            hidden_branch - 1,
                            self.inductors.node_pos[index] - 1,
                            g_voltage * hidden_m_eq_scale,
                        );
                    }
                    if self.inductors.node_neg[index] > 0 {
                        matrix.add(
                            hidden_branch - 1,
                            self.inductors.node_neg[index] - 1,
                            -g_voltage * hidden_m_eq_scale,
                        );
                    }
                    rhs[hidden_branch - 1] += hidden_m_eq_scale
                        * (-trial.level1_residual
                            + g_m * hidden_m
                            + g_rate * hidden_rate
                            + g_happ * current_happ_slope * current
                            + g_voltage * voltage);
                } else {
                    matrix.add(hidden_branch - 1, hidden_branch - 1, 1.0);
                    rhs[hidden_branch - 1] += 0.0;
                }
            }
            if let Some(hidden_rate_branch) = hidden_rate_branch {
                let rate_current_derivative = (if one_step_order2 { 2.0 } else { 1.0 })
                    * binding.device.n_turns_for_xyce_core()
                    / dt;
                if !rate_current_derivative.is_finite() {
                    self.xyce_core_trial_invalid = true;
                    continue;
                }
                matrix.add(
                    hidden_rate_branch - 1,
                    hidden_rate_branch - 1,
                    r_var_scaling * r_eq_scaling,
                );
                matrix.add(
                    hidden_rate_branch - 1,
                    branch - 1,
                    -rate_current_derivative * r_eq_scaling,
                );
                rhs[hidden_rate_branch - 1] += r_eq_scaling
                    * (-trial.level1_rate_residual + hidden_rate
                        - rate_current_derivative * current);
            }
            if advance_magvar_update && binding.device.is_xyce_core_level2() {
                binding
                    .device
                    .advance_xyce_core_mag_update(trial.magnetization_update);
            }
        }

        // A nonlinear K-card with multiple windings is one MutIndNonLin2
        // instance in Xyce.  Assemble its dense constant vacuum Q matrix and
        // shared constitutive mid factor in the same branch-row orientation
        // as the single-winding path above.  Xyce's K-card bundling stores the
        // authored scalar in the `COUPLING` metadata vector, while the
        // nonlinear device's constant LO matrix uses its separate
        // `COUP_VAL` parameter, whose default is unity.  The K-card scalar
        // therefore does not scale this Q matrix; it remains independent of
        // the nonlinear constitutive `mid` factor.
        let hidden_base = self.num_nodes + self.num_branches;
        for group in &mut self.xyce_core_groups {
            if !group.device.is_xyce_core() || group.windings.len() < 2 {
                continue;
            }
            let first = &group.windings[0];
            let first_index = first.inductor_index;
            let first_turns = first.turns;
            let first_voltage = node_voltage(
                solution,
                self.inductors.node_pos[first_index],
                self.inductors.node_neg[first_index],
            );
            let mut currents = Vec::with_capacity(group.windings.len());
            let mut previous = Vec::with_capacity(group.windings.len());
            let mut previous_previous = Vec::with_capacity(group.windings.len());
            let mut voltages = Vec::with_capacity(group.windings.len());
            let mut ampere_turns = 0.0;
            let mut old_ampere_turns = if group.device.is_xyce_core_level2() {
                group.device.xyce_core_old_ampere_turns()
            } else {
                0.0
            };
            for winding in &group.windings {
                let index = winding.inductor_index;
                let branch = self.num_nodes + self.inductors.branch_indices[index];
                let current = solution
                    .get(branch - 1)
                    .copied()
                    .unwrap_or_else(|| self.inductors.i_prev[index]);
                let i_prev = self.inductors.i_prev.get(index).copied().unwrap_or(0.0);
                let i_prev_prev = self
                    .inductors
                    .i_prev_prev
                    .get(index)
                    .copied()
                    .unwrap_or(i_prev);
                let voltage = node_voltage(
                    solution,
                    self.inductors.node_pos[index],
                    self.inductors.node_neg[index],
                );
                ampere_turns += winding.turns * current;
                if !group.device.is_xyce_core_level2() {
                    old_ampere_turns += winding.turns * i_prev;
                }
                currents.push(current);
                previous.push(i_prev);
                previous_previous.push(i_prev_prev);
                voltages.push(voltage);
            }
            let happ = group.device.xyce_core_happ_from_ampere_turns(ampere_turns);
            let old_happ = group
                .device
                .xyce_core_happ_from_ampere_turns(old_ampere_turns);
            let delta_happ = happ - old_happ;
            let accepted_mid = group.device.xyce_core_accepted_mid();
            let previous_mid = if accepted_mid.is_finite() && accepted_mid.abs() > 1.0e-12 {
                accepted_mid
            } else {
                1.0
            };
            let static_scale = if one_step_order2 { 0.5 } else { 1.0 };
            let carried_mag_update = group.device.xyce_core_mag_update();
            let m_var_scaling = group.device.xyce_core_m_var_scaling();
            let r_var_scaling = group.device.xyce_core_r_var_scaling();
            let m_eq_scaling = group.device.xyce_core_m_eq_scaling();
            let r_eq_scaling = group.device.xyce_core_r_eq_scaling();
            let m_eq_scaling = if m_eq_scaling.is_finite() && m_eq_scaling > 0.0 {
                m_eq_scaling
            } else {
                1.0
            };
            let r_eq_scaling = if r_eq_scaling.is_finite() && r_eq_scaling > 0.0 {
                r_eq_scaling
            } else {
                1.0
            };
            let hidden_m_eq_scale = m_eq_scaling;
            let charge_coeff = if one_step_order2 { 1.0 } else { coeff.coeff_g };
            let hidden_m = solution
                .get(
                    group
                        .hidden_m_slot
                        .map(|slot| hidden_base + slot)
                        .unwrap_or(usize::MAX),
                )
                .copied()
                .map(|value| value * m_var_scaling)
                .filter(|value| value.is_finite())
                .unwrap_or_else(|| group.device.magnetization());
            let hidden_rate = solution
                .get(
                    group
                        .hidden_r_slot
                        .map(|slot| hidden_base + slot)
                        .unwrap_or(usize::MAX),
                )
                .copied()
                .map(|value| value * r_var_scaling)
                .filter(|value| value.is_finite())
                .unwrap_or_else(|| group.device.xyce_core_level1_rate_debug());
            let representative_current = if first_turns.abs() > 1.0e-30 {
                ampere_turns / first_turns
            } else {
                0.0
            };
            let cached_trial = (!advance_magvar_update && group.device.is_xyce_core_level2())
                .then(|| {
                    group
                        .device
                        .xyce_core_cached_trial(representative_current, first_voltage)
                })
                .flatten();
            let Some(mut trial) = (if let Some(trial) = cached_trial {
                Some(trial)
            } else if !group.device.is_xyce_core_level2() {
                group
                    .device
                    .xyce_core_level1_trial_at_magnetization_and_rate(
                        XyceCoreStep {
                            happ,
                            delta_happ,
                            voltage: first_voltage,
                            dt,
                            one_step_order2,
                        },
                        hidden_m,
                        hidden_rate,
                    )
            } else {
                group
                    .device
                    .xyce_core_trial_from_happ_with_update_and_ampere_turn_delta(
                        happ,
                        delta_happ,
                        ampere_turns - old_ampere_turns,
                        first_voltage,
                        carried_mag_update,
                    )
            }) else {
                self.xyce_core_trial_invalid = true;
                continue;
            };
            // MutIndNonLin2::acceptStep stores the source-ordered aggregate
            // branch-current sum verbatim.  A shared K-card trial is formed
            // from Happ, so retain the original sum rather than reconstructing
            // it as Happ*Path at the accepted-step boundary.
            if group.device.is_xyce_core_level2() {
                trial.applied_ampere_turns = ampere_turns;
            }
            if !trial.mid.is_finite() || trial.mid.abs() <= 1.0e-12 {
                self.xyce_core_trial_invalid = true;
                continue;
            }
            let hidden_rate_dae = group.hidden_r_slot.map(|_| {
                let rate_current_derivative = if one_step_order2 {
                    2.0 / dt
                } else if one_step {
                    1.0 / dt
                } else {
                    coeff.coeff_g / dt
                };
                let rate_history = if one_step_order2 {
                    group.device.xyce_core_level1_rate_debug()
                } else if !one_step && coeff.coeff_i_n != 0.0 {
                    coeff.coeff_i_n * group.device.xyce_core_level1_rate_debug()
                } else {
                    0.0
                };
                let mut rate_qdot = rate_current_derivative * (ampere_turns - old_ampere_turns);
                if !one_step && coeff.needs_two_history {
                    let previous_previous_ampere_turns = group
                        .windings
                        .iter()
                        .zip(&previous_previous)
                        .map(|(winding, &current)| winding.turns * current)
                        .sum::<Value>();
                    rate_qdot += (coeff.coeff_v_n_minus_1 / dt)
                        * (old_ampere_turns - previous_previous_ampere_turns);
                }
                (
                    rate_current_derivative,
                    hidden_rate + rate_history - rate_qdot,
                )
            });
            if let Some((_, rate_residual)) = hidden_rate_dae {
                // Device convergence must inspect the same R residual that is
                // stamped below; retaining the constitutive helper's BE-only
                // target would veto a correctly solved trapezoidal endpoint.
                trial.level1_rate_residual = rate_residual;
            }
            if advance_magvar_update {
                group
                    .device
                    .cache_xyce_core_trial(representative_current, first_voltage, trial);
            } else {
                group.device.cache_xyce_core_trial_endpoint(
                    representative_current,
                    first_voltage,
                    trial,
                );
            }
            // LEVEL=1's hidden M equation is coupled to every winding current
            // and to the first-winding voltage.  Use its exact local Schur
            // complement, reducing the explicit MNA rows without replacing
            // Xyce's hidden unknown by a fixed-electrical-state projection.
            let hidden_partials = if !group.device.is_xyce_core_level2() {
                group.device.xyce_core_level1_hidden_partials(
                    trial,
                    first_voltage,
                    dt,
                    one_step_order2,
                )
            } else {
                None
            };
            if group.device.has_xyce_core_m_equation() && hidden_partials.is_none() {
                self.xyce_core_trial_invalid = true;
                continue;
            }
            let fixed_mid_m = if hidden_partials.is_some() {
                group
                    .device
                    .xyce_core_dmid_d_magnetization(happ, first_voltage, trial.latest_magnetization)
                    .unwrap_or(Value::NAN)
            } else {
                0.0
            };
            let d_mid_d_first_voltage = group
                .device
                .xyce_core_dmid_d_voltage(happ, first_voltage, trial.latest_magnetization)
                .unwrap_or(0.0);
            let hidden_branch = group.hidden_m_slot.map(|slot| hidden_base + slot + 1);
            let hidden_rate_branch = group.hidden_r_slot.map(|slot| hidden_base + slot + 1);
            for (i, &entry) in voltages.iter().enumerate().take(group.windings.len()) {
                let winding_i = &group.windings[i];
                let index_i = winding_i.inductor_index;
                let branch_i = self.num_nodes + self.inductors.branch_indices[index_i];
                let mut q_current = 0.0;
                let mut q_previous_reconstructed = 0.0;
                let mut q_previous_previous = 0.0;
                for j in 0..group.windings.len() {
                    let winding_j = &group.windings[j];
                    let l0 = group.device.xyce_core_vacuum_mutual_inductance(
                        winding_i.turns,
                        winding_j.turns,
                        1.0,
                    );
                    // Xyce stores each winding's dense LO current sum as Q
                    // history.  Accumulate Q at each accepted endpoint before
                    // differencing; summing `LO*(I-I_prev)` instead loses the
                    // source operation order at sharp reversals.
                    q_current += l0 * currents[j];
                    q_previous_reconstructed += l0 * previous[j];
                    q_previous_previous += l0 * previous_previous[j];
                }
                let q_previous = group
                    .xyce_q_history
                    .get(i)
                    .copied()
                    .unwrap_or(q_previous_reconstructed);
                let mut charge_difference = charge_coeff * (q_current - q_previous);
                if !one_step_order2 && coeff.needs_two_history {
                    charge_difference +=
                        coeff.coeff_v_n_minus_1 * (q_previous - q_previous_previous);
                }
                let charge_derivative = if one_step {
                    (1.0 / dt) * charge_difference
                } else {
                    charge_difference / dt
                };
                let previous_static_voltage = self.inductors.v_prev[index_i] / previous_mid;
                let history = if one_step_order2 {
                    // The transient assembler supplies +1/2(F_n-B_n) from
                    // its accepted static-history snapshot.  Keep the Core
                    // branch's local history empty so that term is not
                    // counted twice.
                    0.0
                } else if coeff.coeff_i_n != 0.0 {
                    coeff.coeff_i_n * previous_static_voltage
                } else {
                    0.0
                };
                let static_branch = static_scale * entry / trial.mid;
                let f0 = if one_step_order2 {
                    charge_derivative - static_branch + history
                } else {
                    static_branch - charge_derivative + history
                };
                if f0.is_finite() {
                    self.xyce_core_transient_residuals.push((branch_i - 1, f0));
                }
                if !f0.is_finite() {
                    self.xyce_core_trial_invalid = true;
                    continue;
                }
                let reduced_f0 = f0;
                let first_voltage_partial = if i == 0 { d_mid_d_first_voltage } else { 0.0 };
                let d_voltage = if one_step_order2 {
                    -static_scale / trial.mid
                        + static_scale * entry * first_voltage_partial / (trial.mid * trial.mid)
                } else {
                    static_scale / trial.mid
                        - static_scale * entry * first_voltage_partial / (trial.mid * trial.mid)
                };
                // For rows belonging to a non-first winding, the same
                // constitutive mid factor still depends on V(first).  This
                // cross-row voltage derivative is absent from the ordinary
                // companion and therefore must be added explicitly.
                let cross_first_voltage = if i == 0 {
                    0.0
                } else if one_step_order2 {
                    static_scale * entry * d_mid_d_first_voltage / (trial.mid * trial.mid)
                } else {
                    -static_scale * entry * d_mid_d_first_voltage / (trial.mid * trial.mid)
                };
                let mut hidden_linearized = 0.0;
                if hidden_partials.is_some() {
                    let d_m = if one_step_order2 {
                        static_scale * entry * fixed_mid_m / (trial.mid * trial.mid)
                    } else {
                        -static_scale * entry * fixed_mid_m / (trial.mid * trial.mid)
                    };
                    if !d_m.is_finite() {
                        self.xyce_core_trial_invalid = true;
                        continue;
                    }
                    if let Some(hidden_branch) = hidden_branch {
                        matrix.add(branch_i - 1, hidden_branch - 1, d_m * m_var_scaling);
                    }
                    hidden_linearized = d_m * hidden_m;
                }
                let mut linearized = 0.0;
                for (j, &current) in currents.iter().enumerate().take(group.windings.len()) {
                    let winding_j = &group.windings[j];
                    let index_j = winding_j.inductor_index;
                    let l0 = group.device.xyce_core_vacuum_mutual_inductance(
                        winding_i.turns,
                        winding_j.turns,
                        1.0,
                    );
                    let current_happ_slope =
                        group.device.xyce_core_happ_slope_for_turns(winding_j.turns);
                    let d_mid_d_current = group.device.xyce_core_dmid_d_happ(
                        happ,
                        first_voltage,
                        trial.latest_magnetization,
                        current_happ_slope,
                    );
                    // MutIndNonLin2's Jacobian includes the constitutive
                    // dP/dI tangent for every winding.  Omitting it changes
                    // the Newton system and is not equivalent to Xyce's
                    // device.
                    let static_derivative = d_mid_d_current.map_or(0.0, |value| {
                        if one_step_order2 {
                            static_scale * entry * value / (trial.mid * trial.mid)
                        } else {
                            -static_scale * entry * value / (trial.mid * trial.mid)
                        }
                    });
                    let charge_derivative_j = if one_step_order2 {
                        charge_coeff * l0 / dt
                    } else {
                        -charge_coeff * l0 / dt
                    };
                    let direct_derivative = static_derivative + charge_derivative_j;
                    let derivative = direct_derivative;
                    if !derivative.is_finite() {
                        self.xyce_core_trial_invalid = true;
                        continue;
                    }
                    matrix.add(
                        branch_i - 1,
                        self.num_nodes + self.inductors.branch_indices[index_j] - 1,
                        derivative,
                    );
                    linearized += derivative * current;
                }
                let voltage_linear = d_voltage * entry + cross_first_voltage * first_voltage;
                let desired_rhs = -reduced_f0 + linearized + hidden_linearized + voltage_linear;
                if self.inductors.node_pos[index_i] > 0 {
                    matrix.add(
                        branch_i - 1,
                        self.inductors.node_pos[index_i] - 1,
                        d_voltage,
                    );
                }
                if self.inductors.node_neg[index_i] > 0 {
                    matrix.add(
                        branch_i - 1,
                        self.inductors.node_neg[index_i] - 1,
                        -d_voltage,
                    );
                }
                if i != 0 && cross_first_voltage.is_finite() {
                    let first_index = group.windings[0].inductor_index;
                    let first_pos = self.inductors.node_pos[first_index];
                    let first_neg = self.inductors.node_neg[first_index];
                    if first_pos > 0 {
                        matrix.add(branch_i - 1, first_pos - 1, cross_first_voltage);
                    }
                    if first_neg > 0 {
                        matrix.add(branch_i - 1, first_neg - 1, -cross_first_voltage);
                    }
                }
                // Core branches own their complete DAE row.  The transient
                // companion pass contributes only the MNA incidence entries;
                // avoid adding and then cancelling the authored L-card
                // companion, whose 1/dt terms can dwarf the geometry-only Q
                // row at tiny timesteps.
                rhs[branch_i - 1] += desired_rhs;
            }
            if let (Some(hidden_branch), Some((g_m, g_happ, g_voltage, g_rate))) =
                (hidden_branch, hidden_partials)
            {
                if !g_m.is_finite()
                    || !g_happ.is_finite()
                    || !g_voltage.is_finite()
                    || !g_rate.is_finite()
                {
                    self.xyce_core_trial_invalid = true;
                    continue;
                }
                matrix.add(
                    hidden_branch - 1,
                    hidden_branch - 1,
                    g_m * m_var_scaling * hidden_m_eq_scale,
                );
                if let Some(hidden_rate_branch) = hidden_rate_branch {
                    matrix.add(
                        hidden_branch - 1,
                        hidden_rate_branch - 1,
                        g_rate * r_var_scaling * hidden_m_eq_scale,
                    );
                }
                let mut current_linear = 0.0;
                for (winding, &current) in group.windings.iter().zip(&currents) {
                    let slope = group.device.xyce_core_happ_slope_for_turns(winding.turns);
                    let derivative = g_happ * slope;
                    matrix.add(
                        hidden_branch - 1,
                        self.num_nodes + self.inductors.branch_indices[winding.inductor_index] - 1,
                        derivative * hidden_m_eq_scale,
                    );
                    current_linear += derivative * current;
                }
                let first_index = group.windings[0].inductor_index;
                if self.inductors.node_pos[first_index] > 0 {
                    matrix.add(
                        hidden_branch - 1,
                        self.inductors.node_pos[first_index] - 1,
                        g_voltage * hidden_m_eq_scale,
                    );
                }
                if self.inductors.node_neg[first_index] > 0 {
                    matrix.add(
                        hidden_branch - 1,
                        self.inductors.node_neg[first_index] - 1,
                        -g_voltage * hidden_m_eq_scale,
                    );
                }
                rhs[hidden_branch - 1] += hidden_m_eq_scale
                    * (-trial.level1_residual
                        + g_m * hidden_m
                        + g_rate * hidden_rate
                        + current_linear
                        + g_voltage * first_voltage);
            } else if let Some(hidden_branch) = hidden_branch {
                matrix.add(hidden_branch - 1, hidden_branch - 1, 1.0);
            }
            if let Some(hidden_rate_branch) = hidden_rate_branch {
                // Xyce's hidden R row has Q_R = rEq * sum(L_i I_i) and
                // F_R = -rEq * R.  The shared companion values above use the
                // same integration coefficients as the winding Q rows.
                let Some((rate_current_derivative, rate_residual)) = hidden_rate_dae else {
                    self.xyce_core_trial_invalid = true;
                    continue;
                };
                let mut current_linear = 0.0;
                for (winding, &current) in group.windings.iter().zip(&currents) {
                    let residual_derivative = -rate_current_derivative * winding.turns;
                    if !residual_derivative.is_finite() {
                        self.xyce_core_trial_invalid = true;
                        continue;
                    }
                    let branch =
                        self.num_nodes + self.inductors.branch_indices[winding.inductor_index];
                    matrix.add(
                        hidden_rate_branch - 1,
                        branch - 1,
                        residual_derivative * r_eq_scaling,
                    );
                    current_linear += residual_derivative * current;
                }
                matrix.add(
                    hidden_rate_branch - 1,
                    hidden_rate_branch - 1,
                    r_var_scaling * r_eq_scaling,
                );
                rhs[hidden_rate_branch - 1] +=
                    r_eq_scaling * (-rate_residual + hidden_rate + current_linear);
            }
            if advance_magvar_update && group.device.is_xyce_core_level2() {
                group
                    .device
                    .advance_xyce_core_mag_update(trial.magnetization_update);
            }
        }
    }

    /// Stamp the accepted static `F` contribution of each Xyce Core.
    ///
    /// OneStep order-2 stores the accepted `F-B` vector separately from the
    /// transient `Q` companion.  Core branches are not part of the ordinary
    /// linear-inductor static stamp, so include their KCL current and
    /// nonlinear branch voltage term explicitly in that history probe.
    pub fn stamp_xyce_core_static_residual(&self, matrix: &mut StaticMatrix, solution: &[Value]) {
        for binding in &self.jiles_atherton_inductors {
            if !binding.device.is_xyce_core() {
                continue;
            }
            let index = binding.inductor_index;
            let branch = self.num_nodes + self.inductors.branch_indices[index];
            let current = solution.get(branch - 1).copied().unwrap_or(0.0);
            let voltage = if self.inductors.node_pos[index] == 0 {
                0.0
            } else {
                solution
                    .get(self.inductors.node_pos[index] - 1)
                    .copied()
                    .unwrap_or(0.0)
            } - if self.inductors.node_neg[index] == 0 {
                0.0
            } else {
                solution
                    .get(self.inductors.node_neg[index] - 1)
                    .copied()
                    .unwrap_or(0.0)
            };
            let mid = binding.device.xyce_core_static_mid(current, voltage);
            if !mid.is_finite() || mid.abs() <= 1.0e-12 {
                continue;
            }
            // Xyce's static F vector includes the inductor's KCL current
            // contributions as well as its branch constitutive equation.
            // The transient companion stamp is intentionally omitted from
            // this probe, so reproduce those node-row incidences here.
            if self.inductors.node_pos[index] > 0 {
                matrix.add(self.inductors.node_pos[index] - 1, branch - 1, 1.0);
                matrix.add(branch - 1, self.inductors.node_pos[index] - 1, -1.0 / mid);
            }
            if self.inductors.node_neg[index] > 0 {
                matrix.add(self.inductors.node_neg[index] - 1, branch - 1, -1.0);
                matrix.add(branch - 1, self.inductors.node_neg[index] - 1, 1.0 / mid);
            }
        }
        for group in &self.xyce_core_groups {
            if !group.device.is_xyce_core() || group.windings.len() < 2 {
                continue;
            }
            let first = &group.windings[0];
            let ampere_turns = group
                .windings
                .iter()
                .map(|winding| {
                    let index = winding.inductor_index;
                    let branch = self.num_nodes + self.inductors.branch_indices[index];
                    winding.turns * solution.get(branch - 1).copied().unwrap_or(0.0)
                })
                .sum::<Value>();
            let representative_current = if first.turns.abs() > 1.0e-30 {
                ampere_turns / first.turns
            } else {
                0.0
            };
            let first_voltage = node_voltage(
                solution,
                self.inductors.node_pos[first.inductor_index],
                self.inductors.node_neg[first.inductor_index],
            );
            let mid = group
                .device
                .xyce_core_static_mid(representative_current, first_voltage);
            if !mid.is_finite() || mid.abs() <= 1.0e-12 {
                continue;
            }
            for winding in &group.windings {
                let index = winding.inductor_index;
                let branch = self.num_nodes + self.inductors.branch_indices[index];
                if self.inductors.node_pos[index] > 0 {
                    matrix.add(self.inductors.node_pos[index] - 1, branch - 1, 1.0);
                    matrix.add(branch - 1, self.inductors.node_pos[index] - 1, -1.0 / mid);
                }
                if self.inductors.node_neg[index] > 0 {
                    matrix.add(self.inductors.node_neg[index] - 1, branch - 1, -1.0);
                    matrix.add(branch - 1, self.inductors.node_neg[index] - 1, 1.0 / mid);
                }
            }
        }
    }

    /// Replace reconstructed Xyce Core branch entries with the exact static
    /// DAE `F` values loaded by the native device.
    ///
    /// The Core Jacobian stamps `-1/mid`, but `loadDAEFVector` evaluates the
    /// branch contribution as `-(voltage / mid)`.  Multiplying the stamped
    /// reciprocal by the solution is algebraically equivalent but has a
    /// different floating-point contract, so OneStep history must retain the
    /// directly evaluated value.
    pub(crate) fn overwrite_xyce_core_static_residual(
        &self,
        residual: &mut [Value],
        solution: &[Value],
    ) {
        for binding in &self.jiles_atherton_inductors {
            if !binding.device.is_xyce_core() {
                continue;
            }
            let index = binding.inductor_index;
            let branch = self.num_nodes + self.inductors.branch_indices[index];
            let current = solution.get(branch - 1).copied().unwrap_or(0.0);
            let voltage = node_voltage(
                solution,
                self.inductors.node_pos[index],
                self.inductors.node_neg[index],
            );
            let mid = binding.device.xyce_core_static_mid(current, voltage);
            if mid.is_finite()
                && mid.abs() > 1.0e-12
                && let Some(entry) = residual.get_mut(branch - 1)
            {
                *entry = -(voltage / mid);
            }
        }

        for group in &self.xyce_core_groups {
            if !group.device.is_xyce_core() || group.windings.len() < 2 {
                continue;
            }
            let first = &group.windings[0];
            let ampere_turns = group
                .windings
                .iter()
                .map(|winding| {
                    let index = winding.inductor_index;
                    let branch = self.num_nodes + self.inductors.branch_indices[index];
                    winding.turns * solution.get(branch - 1).copied().unwrap_or(0.0)
                })
                .sum::<Value>();
            let representative_current = if first.turns.abs() > 1.0e-30 {
                ampere_turns / first.turns
            } else {
                0.0
            };
            let first_voltage = node_voltage(
                solution,
                self.inductors.node_pos[first.inductor_index],
                self.inductors.node_neg[first.inductor_index],
            );
            let mid = group
                .device
                .xyce_core_static_mid(representative_current, first_voltage);
            if !mid.is_finite() || mid.abs() <= 1.0e-12 {
                continue;
            }
            for winding in &group.windings {
                let index = winding.inductor_index;
                let branch = self.num_nodes + self.inductors.branch_indices[index];
                let voltage = node_voltage(
                    solution,
                    self.inductors.node_pos[index],
                    self.inductors.node_neg[index],
                );
                if let Some(entry) = residual.get_mut(branch - 1) {
                    *entry = -(voltage / mid);
                }
            }
        }
    }

    /// Replace reconstructed correction residuals for Xyce Core branch rows
    /// with the exact constitutive residual captured during stamping.
    pub(crate) fn overwrite_xyce_core_transient_correction_rhs(
        &self,
        correction_rhs: &mut [Value],
        one_step_order2: bool,
        static_history: Option<&[Value]>,
    ) {
        for &(row, residual) in &self.xyce_core_transient_residuals {
            let Some(rhs) = correction_rhs.get_mut(row) else {
                continue;
            };
            let previous = if one_step_order2 {
                static_history
                    .and_then(|history| history.get(row))
                    .copied()
                    .unwrap_or(0.0)
                    * 0.5
            } else {
                0.0
            };
            *rhs = -residual - previous;
        }
    }

    /// Refresh effective inductance values for all Jiles-Atherton inductors.
    ///
    /// Call this with the latest solution vector before transient companion
    /// stamping so nonlinear core state updates feed into the MNA coefficients.
    pub fn refresh_jiles_atherton_inductances(&mut self, solution: &[Value]) {
        use crate::device::NonlinearDevice;

        let num_nodes = self.num_nodes;
        for idx in 0..self.jiles_atherton_inductors.len() {
            let (inductor_index, l_eff) = {
                let binding = &mut self.jiles_atherton_inductors[idx];
                if binding.device.is_xyce_core() {
                    // Xyce solves the magnetic state as part of its coupled
                    // M/R DAE.  The native companion uses the last accepted
                    // differential inductance while Newton probes the next
                    // electrical state; mutating M during those probes makes
                    // the residual path-dependent and prevents convergence.
                    continue;
                }
                let branch_matrix_index = num_nodes + binding.branch_ordinal;
                binding.device.set_branch_index(branch_matrix_index);
                binding.device.update(solution);
                (
                    binding.inductor_index,
                    binding.device.effective_inductance(),
                )
            };

            if let Some(slot) = self.inductors.inductances.get_mut(inductor_index)
                && l_eff.is_finite()
                && l_eff > 0.0
            {
                *slot = l_eff.max(1e-18);
            }
        }
    }

    /// Seed Xyce's accepted charge-vector history from the current accepted
    /// inductor histories.  This is called after DC initialization and after
    /// checkpoint injection, before the first transient residual is formed.
    pub fn initialize_xyce_core_q_histories(&mut self) {
        for binding in &mut self.jiles_atherton_inductors {
            if !binding.device.is_xyce_core() {
                continue;
            }
            let current = self
                .inductors
                .i_prev
                .get(binding.inductor_index)
                .copied()
                .unwrap_or(0.0);
            binding.device.initialize_xyce_core_q_history(current);
        }
        for group in &mut self.xyce_core_groups {
            if !group.device.is_xyce_core() {
                continue;
            }
            if group.xyce_q_history.len() != group.windings.len() {
                group.xyce_q_history.resize(group.windings.len(), 0.0);
            }
            for i in 0..group.windings.len() {
                let winding_i = &group.windings[i];
                let mut q = 0.0;
                for winding_j in &group.windings {
                    let current = self
                        .inductors
                        .i_prev
                        .get(winding_j.inductor_index)
                        .copied()
                        .unwrap_or(0.0);
                    let l0 = group.device.xyce_core_vacuum_mutual_inductance(
                        winding_i.turns,
                        winding_j.turns,
                        1.0,
                    );
                    q += l0 * current;
                }
                group.xyce_q_history[i] = q;
            }
        }
    }

    /// Advance Xyce Core states from an accepted transient solution and make
    /// the resulting differential inductance active for the next step.
    pub fn commit_xyce_core_inductances(
        &mut self,
        solution: &[Value],
        dt: Value,
        one_step_order2: bool,
    ) {
        let num_nodes = self.num_nodes;
        let hidden_base = self.num_nodes + self.num_branches;
        for binding in &mut self.jiles_atherton_inductors {
            if !binding.device.is_xyce_core() {
                continue;
            }
            let branch_matrix_index = num_nodes + binding.branch_ordinal;
            binding.device.set_branch_index(branch_matrix_index);
            let m_var_scaling = binding.device.xyce_core_m_var_scaling();
            let r_var_scaling = binding.device.xyce_core_r_var_scaling();
            let hidden_state = if !binding.device.is_xyce_core_level2() {
                let magnetization = binding
                    .hidden_m_slot
                    .and_then(|slot| solution.get(hidden_base + slot).copied())
                    .map(|value| value * m_var_scaling)
                    .unwrap_or_else(|| binding.device.magnetization());
                let rate = binding
                    .hidden_r_slot
                    .and_then(|rate_slot| solution.get(hidden_base + rate_slot).copied())
                    .unwrap_or(0.0)
                    * r_var_scaling;
                Some((magnetization, rate))
            } else {
                None
            };
            binding
                .device
                .commit_xyce_core_solution(solution, hidden_state, dt, one_step_order2);
        }
        for group in &mut self.xyce_core_groups {
            if !group.device.is_xyce_core() || group.windings.len() < 2 {
                continue;
            }
            let mut ampere_turns = 0.0;
            let first = &group.windings[0];
            let mut accepted_currents = Vec::with_capacity(group.windings.len());
            for winding in &group.windings {
                let index = winding.inductor_index;
                let branch = num_nodes + self.inductors.branch_indices[index];
                let current = solution
                    .get(branch - 1)
                    .copied()
                    .unwrap_or_else(|| self.inductors.i_prev[index]);
                accepted_currents.push(current);
                ampere_turns += winding.turns * current;
            }
            if group.xyce_q_history.len() != group.windings.len() {
                group.xyce_q_history.resize(group.windings.len(), 0.0);
            }
            // Commit the dense LOI entries in Xyce's winding/column order.
            // These values are the exact accepted qHistory[0] snapshot used
            // by the next OneStep residual.
            for i in 0..group.windings.len() {
                let winding_i = &group.windings[i];
                let mut q = 0.0;
                for (j, winding_j) in group.windings.iter().enumerate() {
                    let l0 = group.device.xyce_core_vacuum_mutual_inductance(
                        winding_i.turns,
                        winding_j.turns,
                        1.0,
                    );
                    q += l0 * accepted_currents[j];
                }
                group.xyce_q_history[i] = q;
            }
            let happ = group.device.xyce_core_happ_from_ampere_turns(ampere_turns);
            // MutIndNonLin2 retains the exact source-ordered aggregate sum
            // from its previous accepted step.  Preserve that value across a
            // shared K-card commit instead of recovering it through the
            // representative winding current.
            let (previous_happ, raw_ampere_turns) = if group.device.is_xyce_core_level2() {
                let previous_ampere_turns = group.device.xyce_core_old_ampere_turns();
                (
                    group
                        .device
                        .xyce_core_happ_from_ampere_turns(previous_ampere_turns),
                    Some((ampere_turns, ampere_turns - previous_ampere_turns)),
                )
            } else {
                let previous_ampere_turns = first.turns * group.device.current_value();
                (
                    group
                        .device
                        .xyce_core_happ_from_ampere_turns(previous_ampere_turns),
                    None,
                )
            };
            let first_voltage = node_voltage(
                solution,
                self.inductors.node_pos[first.inductor_index],
                self.inductors.node_neg[first.inductor_index],
            );
            let m_var_scaling = group.device.xyce_core_m_var_scaling();
            let r_var_scaling = group.device.xyce_core_r_var_scaling();
            let hidden_state = if !group.device.is_xyce_core_level2() {
                let magnetization = group
                    .hidden_m_slot
                    .and_then(|slot| solution.get(hidden_base + slot).copied())
                    .map(|value| value * m_var_scaling)
                    .unwrap_or_else(|| group.device.magnetization());
                let rate = group
                    .hidden_r_slot
                    .and_then(|rate_slot| solution.get(hidden_base + rate_slot).copied())
                    .unwrap_or(0.0)
                    * r_var_scaling;
                Some((magnetization, rate))
            } else {
                None
            };
            group.device.commit_xyce_core_group_solution(
                XyceCoreStep {
                    happ,
                    delta_happ: happ - previous_happ,
                    voltage: first_voltage,
                    dt,
                    one_step_order2,
                },
                hidden_state,
                raw_ampere_turns,
            );
        }
    }

    /// Stamp coupled inductor mutual-coupling overlays for transient analysis.
    ///
    /// The standalone inductors stamp their own self-inductance rows; each
    /// pair adds only the -r12 cross terms and mutual history sources.
    pub fn stamp_coupled_inductor_pairs_transient(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        dt: Value,
        coeff: &CompanionCoefficients,
    ) {
        let mut stamper = StaticMatrixStamper { matrix, rhs };
        for binding in &self.coupled_inductor_pairs {
            let core_owned = self.xyce_core_groups.iter().any(|group| {
                group.windings.iter().any(|winding| {
                    self.inductors.branch_indices[winding.inductor_index] == binding.branch1_ordinal
                }) && group.windings.iter().any(|winding| {
                    self.inductors.branch_indices[winding.inductor_index] == binding.branch2_ordinal
                })
            });
            if core_owned {
                continue;
            }
            let br1 = self.num_nodes + binding.branch1_ordinal;
            let br2 = self.num_nodes + binding.branch2_ordinal;
            binding
                .device
                .stamp_transient_mutual(br1, br2, dt, coeff, &mut stamper);
        }
    }

    /// Stamp multi-winding transformer companion models for transient analysis.
    pub fn stamp_multi_winding_transformers_transient(
        &self,
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        dt: Value,
        coeff: &CompanionCoefficients,
    ) {
        let mut stamper = StaticMatrixStamper { matrix, rhs };
        for binding in &self.multi_winding_transformers {
            binding
                .device
                .stamp_transient_companion(dt, coeff, &mut stamper, &mut []);
        }
    }

    /// Replace inductive branch entries in `b - A*x` with DAE residuals
    /// evaluated from current differences. This is the correction-form
    /// counterpart to the absolute companion stamps above.
    pub fn stabilize_inductor_transient_correction_rhs(
        &self,
        correction_rhs: &mut [Value],
        iterate: &[Value],
        dt: Value,
        coeff: &CompanionCoefficients,
    ) {
        let core_indices = self
            .jiles_atherton_inductors
            .iter()
            .filter(|binding| binding.device.is_xyce_core())
            .map(|binding| binding.inductor_index)
            .collect::<Vec<_>>();
        let mut core_indices = core_indices;
        core_indices.extend(
            self.xyce_core_groups
                .iter()
                .flat_map(|group| group.windings.iter().map(|winding| winding.inductor_index)),
        );
        self.inductors.overwrite_transient_correction_rhs_excluding(
            correction_rhs,
            iterate,
            dt,
            coeff,
            self.num_nodes,
            &core_indices,
        );
        for binding in &self.coupled_inductor_pairs {
            let branch1 = self.num_nodes + binding.branch1_ordinal;
            let branch2 = self.num_nodes + binding.branch2_ordinal;
            binding.device.add_transient_mutual_correction_rhs(
                branch1,
                branch2,
                correction_rhs,
                iterate,
                dt,
                coeff,
            );
        }
        for binding in &self.multi_winding_transformers {
            binding
                .device
                .overwrite_transient_correction_rhs(correction_rhs, iterate, dt, coeff);
        }
    }

    /// Update coupled inductor transient history from an accepted solution.
    pub fn update_coupled_inductor_pair_state(&mut self, solution: &[Value]) {
        let num_nodes = self.num_nodes;
        for binding in &mut self.coupled_inductor_pairs {
            let br1 = num_nodes + binding.branch1_ordinal;
            let br2 = num_nodes + binding.branch2_ordinal;
            binding
                .device
                .update_state_with_branches(solution, br1, br2);
        }
    }

    /// Update multi-winding transformer transient history from an accepted solution.
    pub fn update_multi_winding_transformer_state(&mut self, solution: &[Value]) {
        for binding in &mut self.multi_winding_transformers {
            binding.device.update_state_from_solution(solution);
        }
    }
}
