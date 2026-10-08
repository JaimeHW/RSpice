//! Local smoothness of the canonical diode F/Q equations at physical events.
//! Model parameters do not themselves create a discontinuity. The active
//! analytic branch and its actual switching boundaries determine regularity.

use super::*;

fn positive_normal(value: Value) -> bool {
    value.is_normal() && value > 0.0
}

impl Diode {
    /// Sufficient local C2 chart, shared by charge and finite-current event
    /// continuity. False retains the event; it never changes device equations
    /// or admits an unstamped term. No equality of sampled tangents establishes
    /// continuity across a piecewise constitutive join.
    pub(crate) fn physical_event_locally_c2(&self, vd: Value) -> bool {
        if !vd.is_finite()
            || !self.junction_locally_c2(vd, self.bottom_saturation_current(), self.n, true)
        {
            return false;
        }
        if self.sidewall_current_given && (!self.xyce_dialect || self.sidewall_emission_given) {
            let emission = if self.sidewall_emission_given {
                self.sidewall_emission_coefficient
            } else {
                self.n
            };
            if !self.junction_locally_c2(
                vd,
                self.sidewall_saturation_current * self.sidewall_perimeter,
                emission,
                self.sidewall_emission_given,
            ) {
                return false;
            }
        }
        if self.tunneling.bottom_given || self.tunneling.sidewall_given {
            let thermal = self.tunneling.emission.max(EPSMIN) * self.vt;
            if !positive_normal(thermal) || -vd / thermal == MAX_EXP_ARG {
                return false;
            }
        }
        if self.recombination_saturation_current != 0.0 {
            // Each generation factor is a power of (1-V/VJ)^2 + .005,
            // strictly positive. Only dialect cutoffs and the exponential
            // continuation introduce voltage-dependent joins in this law.
            let thermal = self.recombination_emission_coefficient * self.vt;
            let boundary = -3.0 * self.n.max(EPSMIN) * self.vt;
            if !positive_normal(thermal)
                || !self.recombination_saturation_current.is_finite()
                || self.recombination_saturation_current < 0.0
                || !positive_normal(self.vj)
                || !self.m.is_finite()
                || ((self.ngspice_dialect || self.xyce_dialect) && vd == boundary)
            {
                return false;
            }
            let evaluation = if self.ngspice_dialect {
                vd.max(boundary)
            } else {
                vd
            };
            if !(self.xyce_dialect && vd < boundary) {
                let normalized = 1.0 - evaluation / self.vj;
                let base = normalized * normalized + 0.005;
                let factor = base.powf(0.5 * self.m);
                let derivative = -self.m * normalized / self.vj * base.powf(0.5 * self.m - 1.0);
                if evaluation / thermal == MAX_EXP_ARG
                    || !base.is_finite()
                    || !factor.is_finite()
                    || !derivative.is_finite()
                {
                    return false;
                }
            }
        }
        let components = self.current_components_before_knees(vd, self.stamped_junction_gmin());
        if components
            .into_iter()
            .any(|(i, g)| !i.is_finite() || !g.is_finite())
        {
            return false;
        }
        // The authored knees act on a summed current, not on voltage alone.
        // Retain ngspice's +/-1e-18 A switches and Xyce's distinct domain.
        let forward = vd >= -3.0 * self.n * self.vt;
        let knee = if forward {
            self.forward_knee_current
        } else {
            self.reverse_knee_current
        };
        let bottom_smooth = if self.xyce_forward_injection(vd) {
            // Xyce has no +/-1e-18 current switch. Its analytic square-root
            // chart requires finite IKF and Inorm with Inorm/IKF > -1.
            // A positive overflowing ratio is still on the smooth chart.
            knee.is_finite() && components[0].0 / knee > -1.0
        } else {
            knee_locally_c2(components[0].0, knee, forward)
        };
        if !bottom_smooth
            || !knee_locally_c2(
                components[1].0,
                self.sidewall_knee_current * self.sidewall_perimeter,
                true,
            )
            || ((self.forward_knee_current > 0.0 || self.reverse_knee_current > 0.0)
                && vd == -3.0 * self.n * self.vt)
        {
            return false;
        }
        for (c, phi, grading, fc) in [
            (self.cj0, self.vj, self.m, self.fc),
            (
                self.sidewall_cj0 * self.sidewall_perimeter,
                self.sidewall_vj,
                self.sidewall_m,
                self.sidewall_fc,
            ),
        ] {
            if c == 0.0 {
                continue;
            }
            if !c.is_finite()
                || c < 0.0
                || !positive_normal(phi)
                || !grading.is_finite()
                || !fc.is_finite()
                || vd == fc.clamp(0.0, 0.95) * phi
            {
                return false;
            }
            // The depletion exponential clamp does not share the unclamped
            // charge derivative; do not certify that chart or its join.
            if vd < fc.clamp(0.0, 0.95) * phi && -grading * (1.0 - vd / phi).ln() >= MAX_EXP_ARG {
                return false;
            }
        }
        let (f, g) = self.stamped_current_and_conductance(vd);
        let (q, c) = self.junction_charge_and_capacitance(vd);
        [f, g, q, c].into_iter().all(Value::is_finite)
    }

    fn junction_locally_c2(
        &self,
        vd: Value,
        isat: Value,
        emission: Value,
        breakdown: bool,
    ) -> bool {
        if !isat.is_finite() || isat < 0.0 || !emission.is_finite() {
            return false;
        }
        if isat == 0.0 {
            return true;
        }
        let thermal = emission.max(EPSMIN) * self.vt;
        if !positive_normal(thermal) || vd == -3.0 * thermal {
            return false;
        }
        if vd > -3.0 * thermal {
            return vd / thermal != MAX_EXP_ARG;
        }
        if let Some(voltage) = self.active_breakdown_voltage() {
            if vd == -voltage {
                return false;
            }
            if breakdown && vd < -voltage {
                let thermal = self.breakdown_emission_coefficient.max(EPSMIN) * self.vt;
                return positive_normal(thermal)
                    && -(voltage + vd) / thermal != BREAKDOWN_EXP_ARG_MAX;
            }
        }
        // Cubic reverse continuation or the constant zero sidewall branch.
        true
    }
}

fn knee_locally_c2(current: Value, knee: Value, forward: bool) -> bool {
    if !knee.is_finite() || knee < 0.0 {
        return false;
    }
    if knee == 0.0 {
        return true;
    }
    let signed = if forward { current } else { -current };
    signed != 1e-18 && (signed < 1e-18 || positive_normal(signed / knee))
}

#[cfg(test)]
mod tests;
