//! Sufficient local C1 domain for the physical GP event equations.
//!
//! This is deliberately narrower than model support. A false result keeps
//! the event uncertified; it does not reject the model or change its laws.
//! Finite sampled tangents alone are not a regularity certificate.

use super::*;

impl Bjt {
    /// Select the analytic outgoing diffusion-charge tangent at the VBE=0
    /// join. The finite rate solve must agree with the selected chart; this
    /// is not a higher-order smoothness certificate.
    pub(crate) fn legacy_event_forward_charge_limit(
        &self,
        solution: &[Value],
        rates: &[Option<Value>],
    ) -> Result<bool, &'static str> {
        if !self.uses_legacy_gummel_poon() || self.tf == 0.0 {
            return Ok(false);
        }
        let internal = self.mna_internal_state_at_solution(solution);
        if internal[IDX_VBI] != internal[IDX_VEI] {
            return Ok(false);
        }
        let rate = |index| {
            let node = self.mna_internal_node(index);
            if node == 0 {
                return Ok(0.0);
            }
            rates
                .get(node - 1)
                .copied()
                .flatten()
                .filter(|v| v.is_finite())
                .ok_or("missing physical BJT junction rate")
        };
        let base = rate(IDX_VBI)?;
        let emitter = rate(IDX_VEI)?;
        // Compare before subtracting: two finite rates can have a difference
        // outside binary64 even though their direction is unambiguous.
        Ok(if self.polarity() > 0.0 {
            base > emitter
        } else {
            base < emitter
        })
    }

    pub(crate) fn legacy_event_locally_c1(&self, solution: &[Value]) -> bool {
        if !self.uses_legacy_gummel_poon()
            || !self.mna_promoted()
            || self.avc1 != 0.0
            || !self.is.is_finite()
            || self.is <= 0.0
        {
            return false;
        }
        let internal = self.mna_internal_state_at_solution(solution);
        let p = self.polarity();
        let vbe = p * (internal[IDX_VBI] - internal[IDX_VEI]);
        let vbc = p * (internal[IDX_VBI] - internal[IDX_VCI]);
        if !vbe.is_finite() || !vbc.is_finite() || (self.tf != 0.0 && vbe == 0.0) {
            // The two diffusion-charge formulas meet at VBE=0 but their
            // tangents need not agree when the base-charge factor differs.
            return false;
        }
        // Junction exponential/cubic continuations are C1 at -3*n*VT.
        // Temperature/emission mappings are fixed during this electrical
        // event; invalid or degenerate mappings do not certify a chart.
        let mapped = self
            .legacy_junction_params
            .as_ref()
            .and_then(|j| j.temperature_parameters.as_ref());
        if !self.vt.is_finite()
            || self.vt <= 0.0
            || [self.nf, self.nr, self.nen, self.nci, self.ncn]
                .into_iter()
                .chain(mapped.into_iter().flat_map(|m| m.operating_emission))
                .any(|n| !n.is_finite() || !(n * self.vt).is_normal() || n <= 0.0)
        {
            return false;
        }
        let raw =
            1.0 - if self.var.is_finite() && self.var > 0.0 {
                vbe / self.var
            } else {
                0.0
            } - if self.vaf.is_finite() && self.vaf > 0.0 {
                vbc / self.vaf
            } else {
                0.0
            };
        let (inverse, _) = Self::smooth_positive_floor(raw, 1e-9);
        let transport = self.legacy_transport_charge_state(vbe, vbc);
        if !inverse.is_finite()
            || inverse <= 1e-18
            || !transport.q1.is_finite()
            || transport.q1 <= 1e-12
            || !transport.qb.is_finite()
            || transport.qb <= 1e-12
        {
            return false;
        }
        if self.ikf != 0.0 || self.ikr != 0.0 {
            let q2 = if self.ikf > 0.0 {
                transport.ifi / self.ikf
            } else {
                0.0
            } + if self.ikr > 0.0 {
                transport.iri / self.ikr
            } else {
                0.0
            };
            let argument = 1.0 + 4.0 * q2;
            let power = if self.nkf_given {
                self.nkf.clamp(1e-12, 1.0)
            } else {
                0.5
            };
            // The nonpositive rolloff continuation uses a different value.
            // Do not certify its join or an active derivative-inconsistent floor.
            if !argument.is_finite() || argument <= 0.0 || argument.powf(power) <= 1e-18 {
                return false;
            }
        }
        for (cap, phi, grading) in [
            (self.cje, self.vje, self.mje),
            (self.cjc, self.vjc, self.mjc),
            (self.cjcp, self.ps, self.ms),
        ] {
            // In this domain the depletion continuation is C1, including
            // its forward join, and the capacitance clamp is inactive.
            if cap != 0.0
                && (!cap.is_finite()
                    || cap < 0.0
                    || !phi.is_finite()
                    || phi <= 1e-12
                    || !(0.0..=1.0).contains(&grading))
            {
                return false;
            }
        }
        if self.tf != 0.0 && vbe > 0.0 && self.xtf != 0.0 {
            if self.vtf > 0.0 {
                let argument = vbc / (self.vtf * 1.44);
                if !(argument > -80.0 && argument < 80.0) {
                    return false;
                }
            }
            if self.itf > 0.0
                && (transport.ifi <= 0.0 || !(self.itf * self.instance_scale()).is_normal())
            {
                return false;
            }
        }
        if Self::series_active(self.rbi) {
            let (linearized, _) = self.linearize_currents_with_branches(
                internal[IDX_VBI] - internal[IDX_VEI],
                internal[IDX_VBX] - internal[IDX_VEI],
                internal[IDX_VBI] - internal[IDX_VCI],
            );
            let resistance = self
                .legacy_gp_base_resistance(linearized, self.guarded_series_resistance(self.rbi));
            if !resistance.is_finite() || resistance <= 0.0 {
                return false;
            }
            if self.irb > 0.0 {
                let raw_ratio = p * linearized.ib / self.irb;
                if !raw_ratio.is_finite() || raw_ratio == 1e-9 {
                    return false;
                }
                let ratio = raw_ratio.max(1e-9);
                let root = ratio.sqrt();
                let z = if ratio < 1.0 {
                    14.59025 * root / (2.4317 * ((1.0 + 14.59025 * ratio).sqrt() + 1.0))
                } else {
                    ((14.59025 + ratio.recip()).sqrt() - root.recip()) / 2.4317
                };
                // Exclude the polynomial/trigonometric join. The two stable
                // formulas at ratio=1 represent the same analytic z law.
                if !z.is_finite() || z.abs() == 1e-3 || z.abs() >= std::f64::consts::FRAC_PI_2 {
                    return false;
                }
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn causal_event_orders_require_a_gp_constitutive_chart() {
        let mut model = Bjt::new_npn("q".into(), 1, 2, 0).with_params(&HashMap::from([
            ("IS".into(), 1e-16),
            ("TF".into(), 1e-9),
            ("VAF".into(), 20.0),
            ("CJE".into(), 1e-12),
            ("CJC".into(), 2e-13),
        ]));
        model.assign_mna_internal_nodes(|_| panic!("fixture requires only terminal aliases"));
        assert!(model.legacy_event_locally_c1(&[2.0, 0.6]));
        assert!(model.legacy_event_locally_c1(&[2.0, -0.2]));
        assert!(!model.legacy_event_locally_c1(&[2.0, 0.0]));
        let baseline = model.clone();
        model.tf = 0.0;
        assert!(model.legacy_event_locally_c1(&[2.0, 0.0]));
        model = baseline.clone();
        model.ikf = -4.0 * model.legacy_transport_charge_state(-0.2, -2.2).ifi;
        assert!(model.ikf > 0.0);
        assert!(!model.legacy_event_locally_c1(&[2.0, -0.2]));
        model = baseline.clone();
        model.mje = 2.0;
        assert!(!model.legacy_event_locally_c1(&[2.0, 0.6]));
        model = baseline.clone();
        model.avc1 = 1.0;
        assert!(!model.legacy_event_locally_c1(&[2.0, 0.6]));
        model = baseline;
        model.xtf = 1.0;
        model.vtf = 25.0;
        assert_eq!(-2880.0 / (model.vtf * 1.44), -80.0);
        assert!(!model.legacy_event_locally_c1(&[2880.5, 0.5]));
        assert!(model.legacy_event_locally_c1(&[2844.5, 0.5]));
    }
}
