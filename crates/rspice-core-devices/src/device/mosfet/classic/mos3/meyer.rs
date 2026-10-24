use super::*;
use crate::device::mosfet::dual::Dual3;

impl Mosfet {
    pub(in crate::device::mosfet::classic) fn mos3_meyer_state_dual(
        &self,
        gate: Dual3,
        drain: Dual3,
        bulk: Dual3,
    ) -> (Dual3, Dual3) {
        let length = self.mos3_effective_length();
        let width = self.mos3_effective_width();
        let cox = self.cox.max(0.0);
        let oxide = cox * length * width;
        let phi = self.phi.max(1e-12);
        let sqrt_phi = phi.sqrt();
        let vbi = self.vto - self.polarity() * self.gamma * sqrt_phi;
        let eta = if cox > 0.0 {
            self.mos3_eta * 8.15e-22 / (cox * length.powi(3))
        } else {
            0.0
        };
        let (phibs, sqphbs) = if bulk.value <= 0.0 {
            let phibs = (phi - bulk).max_const(1e-18);
            (phibs, phibs.sqrt())
        } else {
            let sq = sqrt_phi / (1.0 + bulk / (phi + phi)).max_const(1e-12);
            (sq * sq, sq)
        };
        let xd = self.mos3_coeff_depletion_width();
        let fshort = if self.mos3_junction_depth > 0.0 && xd > 0.0 {
            let xj = self.mos3_junction_depth;
            let djonxj = self.ld / xj;
            let wponxj = xd * sqphbs / xj;
            let wconxj = 0.063_135_3 + 0.801_329_2 * wponxj - 0.011_107_77 * wponxj * wponxj;
            let argc = wponxj / (1.0 + wponxj).max_const(1e-12);
            let argb = (1.0 - argc * argc).max_const(0.0).sqrt();
            1.0 - xj / length * ((wconxj + djonxj) * argb - djonxj)
        } else {
            Dual3::constant(1.0)
        };
        let gamma = self.gamma * fshort;
        let fbodys = 0.5 * gamma / (sqphbs + sqphbs).max_const(1e-18);
        let onfbdy = 1.0 / (1.0 + fbodys + self.mos3_narrow_factor / width).max_const(1e-12);
        let qbonco = gamma * sqphbs + self.mos3_narrow_factor * phibs / width;
        let vth = self.polarity() * vbi - eta * drain + qbonco;
        let mut von = vth;
        if self.mos3_fast_surface_state_density != 0.0 && oxide > 0.0 {
            let csonco =
                CHARGE * self.mos3_fast_surface_state_density * 1e4 * length * width / oxide;
            let cdonco = qbonco / (phibs + phibs).max_const(1e-18);
            von = vth + self.vt * (1.0 + csonco + cdonco);
        } else if gate.value <= von.value {
            return (von, Dual3::constant(0.0));
        }
        let overdrive = gate.max(von) - vth;
        let fgate = 1.0 / (1.0 + self.mos3_theta * overdrive).max_const(1e-12);
        let us = self.u0.max(0.0) * 1e-4 * fgate;
        let mut saturation = overdrive * onfbdy;
        if self.mos3_max_drift_velocity > 0.0 && us.value > 0.0 {
            let vdsc = length * self.mos3_max_drift_velocity / us;
            saturation = saturation + vdsc - (saturation * saturation + vdsc * vdsc).sqrt();
        }
        if !saturation.value.is_finite() || saturation.value < 0.0 {
            saturation = Dual3::constant(0.0);
        }
        (von, saturation)
    }
}
