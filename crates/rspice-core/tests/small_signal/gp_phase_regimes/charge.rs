use super::*;

/// Independent derivatives of the GP bias-dependent forward and reverse
/// diffusion charges. All fixture VBE values are positive, and no depletion
/// charge is present. No numerical charge or device evaluator is reused.
#[derive(Clone, Copy, Debug)]
pub(super) struct ChargeLaw {
    pub xtf: f64,
    pub vtf: f64,
    pub itf: f64,
    pub tr: f64,
}

impl ChargeLaw {
    pub const UNMODULATED: Self = Self {
        xtf: 0.0,
        vtf: 1.0,
        itf: 0.0,
        tr: 0.0,
    };
    pub const BIASED: Self = Self {
        xtf: 4.0,
        vtf: 0.4,
        itf: 1e-4,
        tr: 3e-10,
    };

    pub fn parameters(self) -> String {
        format!(
            "XTF={} VTF={} ITF={} TR={}",
            self.xtf, self.vtf, self.itf, self.tr
        )
    }

    pub fn rates(self, state: &State) -> (f64, f64) {
        let (gain, gain_rate) = if self.itf == 0.0 {
            (1.0, 0.0)
        } else {
            let sum = state.forward + self.itf;
            (
                state.forward / sum,
                state.forward_rate * self.itf / sum.powi(2),
            )
        };
        let bias = ((state.base - state.collector) / (1.44 * self.vtf)).exp();
        let bias_rate = bias * (state.base_rate - state.collector_rate) / (1.44 * self.vtf);
        let modulation = self.xtf * bias * gain.powi(2);
        let modulation_rate = self.xtf * (bias_rate * gain.powi(2) + 2.0 * bias * gain * gain_rate);
        (
            TF * ((1.0 + modulation) * state.forward_transport_rate
                + state.forward_transport * modulation_rate),
            self.tr * state.reverse_rate,
        )
    }

    /// Roundoff in the large C/dt*V terms that cancel in a clamped charge
    /// solve. Near a current zero, scaling KCL only by the tiny physical lead
    /// currents demands more precision than the companion system contains.
    /// Bound it by independent charge partials and 256 machine epsilons; the
    /// separate physical-current accuracy/refinement limits remain unchanged.
    pub fn companion_roundoff(self, state: &State, dialect: SpiceDialect, step: f64) -> f64 {
        // Preserve the tighter, already qualified unmodulated-charge KCL
        // regression. Only the additional biased/reverse-charge cases need
        // the companion conditioning allowance.
        if step == 0.0 || (self.xtf == 0.0 && self.tr == 0.0) {
            return 0.0;
        }
        let vbc = state.base - state.collector;
        let (_, gf) = diode(state.base, thermal_voltage(dialect), dialect);
        let (_, gr) = diode(vbc, thermal_voltage(dialect), dialect);
        let q1 = 1.0 / (1.0 - vbc / VAF - state.base / VAR);
        let root = (1.0 + 4.0 * (state.forward / IKF + state.reverse / IKR)).sqrt();
        let qb = state.charge_factor;
        let dqb_be = q1 * qb / VAR + q1 * gf / (IKF * root);
        let dqb_bc = q1 * qb / VAF + q1 * gr / (IKR * root);
        let df_be = (gf - state.forward_transport * dqb_be) / qb;
        let df_bc = -state.forward_transport * dqb_bc / qb;
        let (gain, gain_be) = if self.itf == 0.0 {
            (1.0, 0.0)
        } else {
            let sum = state.forward + self.itf;
            (state.forward / sum, gf * self.itf / sum.powi(2))
        };
        let bias = self.xtf * (vbc / (1.44 * self.vtf)).exp();
        let modulation = bias * gain.powi(2);
        let dqbe_be = TF
            * ((1.0 + modulation) * df_be + state.forward_transport * 2.0 * bias * gain * gain_be);
        let dqbe_bc = TF
            * ((1.0 + modulation) * df_bc
                + state.forward_transport * modulation / (1.44 * self.vtf));
        let forward_norm = (dqbe_be.abs() + dqbe_bc.abs()) * state.base.abs()
            + dqbe_bc.abs() * state.collector.abs();
        let reverse_norm = (self.tr * gr).abs() * (state.base.abs() + state.collector.abs());
        256.0 * f64::EPSILON * 2.0 / step * (forward_norm + reverse_norm)
    }
}

#[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
#[cfg_attr(not(target_arch = "wasm32"), test)]
fn gp_bias_dependent_forward_and_reverse_charge_remain_instantaneous() {
    for dialect in [
        SpiceDialect::Ngspice,
        SpiceDialect::Xyce,
        SpiceDialect::BestAvailable,
    ] {
        for model in [
            GpTransientPhaseModel::ExactDelay,
            GpTransientPhaseModel::NgspiceWeil,
        ] {
            for polarity in [1.0, -1.0] {
                for regime in REGIMES {
                    let coarse =
                        run_with_charge(regime, polarity, dialect, model, 4e-11, ChargeLaw::BIASED);
                    let fine =
                        run_with_charge(regime, polarity, dialect, model, 4e-12, ChargeLaw::BIASED);
                    eprintln!(
                        "biased charge {}/{dialect:?}/{model:?}/{polarity}: coarse={coarse:?}, fine={fine:?}",
                        regime.name
                    );
                    // As with the unmodulated oracle, include first-order
                    // startup/event-restart samples at the requested 1% RELTOL.
                    assert!(
                        fine.collector < 1e-14 + 0.02 * fine.collector_peak
                            && fine.base < 1e-14 + 0.02 * fine.base_peak,
                        "{regime:?}/{dialect:?}/{model:?}/{polarity}: accuracy {fine:?}"
                    );
                    assert!(
                        fine.collector < 1e-14 + 0.7 * coarse.collector
                            && fine.base < 1e-14 + 0.7 * coarse.base,
                        "{regime:?}/{dialect:?}/{model:?}/{polarity}: refinement coarse={coarse:?}, fine={fine:?}"
                    );
                }
            }
        }
    }
}
