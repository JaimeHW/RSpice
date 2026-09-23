use super::*;

impl Mosfet {
    /// Differentiate the scalar onset/saturation equations used by Meyer.
    /// The historical MOS2 current Jacobian deliberately omits some VDSAT
    /// sensitivities; those conventions are not derivatives of C(v).
    pub(in crate::device::mosfet::classic) fn level2_meyer_state_dual(
        &self,
        gate: Dual3,
        drain: Dual3,
        bulk: Dual3,
    ) -> (Dual3, Dual3) {
        let length = self.level2_effective_length();
        let phi = self.phi.max(1e-12);
        let sqrt_phi = phi.sqrt();
        let phi_body = phi - bulk;
        let cox = self.cox.max(0.0);
        let oxide = cox * length * self.w.max(1e-18);
        let xd = self.level2_depletion_width_factor();
        let vbi = self.vto - self.polarity() * self.gamma * sqrt_phi;
        let depletion = |body: Dual3| {
            let arg = if body.value <= 0.0 {
                (phi - body).max_const(1e-18).sqrt()
            } else {
                sqrt_phi / (1.0 + 0.5 * body / phi)
            };
            let derivative = if body.value <= 0.0 {
                -0.5 / arg
            } else {
                -0.5 * arg * arg / (phi * sqrt_phi)
            };
            (arg, derivative)
        };
        let (sarg, dsrgdb) = depletion(bulk);
        let (barg, dbrgdb) = depletion(bulk - drain);
        let factor = if oxide > 0.0 {
            0.125 * self.mos2_narrow_factor * 2.0 * std::f64::consts::PI * EPSSIL / oxide * length
        } else {
            0.0
        };
        let eta = 1.0 + factor;
        let vbin = self.polarity() * vbi + factor * phi_body;
        let gamma = self.level2_short_channel_gamma_dual(sarg, barg, length, xd);
        // This derivative is itself part of the scalar surface-state law.
        // Differentiate its formula to retain the mixed body/drain terms.
        let mut dgamma_db = Dual3::constant(0.0);
        if (self.gamma > 0.0 || self.mos2_substrate_doping > 0.0)
            && self.mos2_junction_depth > 0.0
            && xd > 0.0
        {
            let scale = 2.0 / self.mos2_junction_depth;
            let args = (1.0 + xd * sarg * scale).max_const(0.0).sqrt();
            let argd = (1.0 + xd * barg * scale).max_const(0.0).sqrt();
            if args.value > 0.0 && argd.value > 0.0 {
                dgamma_db = -self.gamma
                    * (0.5 / length * xd * dsrgdb / args + 0.5 / length * xd * dbrgdb / argd);
            }
        }
        let mut von = vbin + gamma * sarg;
        let fast = self.mos2_fast_surface_state_density != 0.0 && oxide > 0.0;
        if fast {
            let cfs = XYCE_CHARGE * self.mos2_fast_surface_state_density * 1e4;
            let cdonco = -(gamma * dsrgdb + dgamma_db * sarg) + factor;
            let shift = self.level2_xyce_thermal_voltage() * (1.0 + cfs / cox + cdonco);
            if shift.value.is_finite() && shift.value > 0.0 {
                von = von + shift;
            }
        } else if gate.value <= vbin.value {
            return (von, Dual3::constant(0.0));
        }
        let overdrive = gate - von;
        let critical = if cox > 0.0 {
            self.mos2_crit_field * 100.0 * EPSSIL / cox
        } else {
            Value::INFINITY
        };
        let mobility = if overdrive.value > critical && critical > 0.0 {
            (critical / overdrive).powf(self.mos2_crit_field_exp)
        } else {
            Dual3::constant(1.0)
        };
        let gamma = gamma / eta;
        let active_gate = if fast { gate.max(von) } else { gate };
        let drive = (active_gate - vbin) / eta;
        let mut saturation = Dual3::constant(0.0);
        if gamma.value > 0.0 {
            let gamma2 = gamma * gamma;
            let arg = drive + phi_body;
            if arg.value > 0.0 {
                saturation = (drive + gamma2 * (1.0 - (1.0 + 4.0 * arg / gamma2).sqrt()) / 2.0)
                    .max_const(0.0);
            }
        } else {
            saturation = drive.max_const(0.0);
        }
        if self.mos2_max_drift_vel > 0.0 {
            let ueff = self.u0_card * 1e-4 * mobility;
            if ueff.value > 0.0 {
                let xv = self.mos2_max_drift_vel * length / ueff;
                let sarg3 = sarg * sarg * sarg;
                if let Some(value) = Self::level2_vmax_vdsat(
                    active_gate.value,
                    vbin.value,
                    eta,
                    gamma.value,
                    phi_body.value,
                    sarg3.value,
                    xv.value,
                ) {
                    // Implicit derivative of the selected Baum quartic root.
                    // Root selection/value remains the native scalar algorithm.
                    let x = (value + phi_body.value).sqrt();
                    let v1 = drive + phi_body;
                    let a = gamma / 0.75;
                    let b = -2.0 * (v1 + xv);
                    let c = -2.0 * gamma * xv;
                    let d = 2.0 * v1 * (phi_body + xv)
                        - phi_body * phi_body
                        - 4.0 / 3.0 * gamma * sarg3;
                    let partial = a * (x * x * x) + b * (x * x) + c * x + d;
                    let slope =
                        4.0 * x * x * x + 3.0 * a.value * x * x + 2.0 * b.value * x + c.value;
                    saturation = Dual3 {
                        value,
                        derivative: std::array::from_fn(|i| {
                            -2.0 * x * partial.derivative[i] / slope - phi_body.derivative[i]
                        }),
                    };
                }
            }
        }
        (von, saturation)
    }
}
