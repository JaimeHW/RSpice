use super::*;

impl Mosfet {
    pub(crate) fn gate_charge_branch_voltages_at(
        &self,
        voltages: &[Value],
    ) -> (Value, Value, Value) {
        let (vgs, vds, vbs) = self.eval_branch_voltages_at(voltages);
        (vgs, vgs - vds, vgs - vbs)
    }

    /// Return the cached transconductance magnitude at the operating point.
    pub fn transconductance(&self) -> Value {
        self.gm.abs()
    }

    /// Return the current thermal-noise coefficient used for channel noise.
    pub fn channel_thermal_noise_gamma(&self) -> Value {
        self.thermal_noise_gamma.max(0.0)
    }

    /// Return the effective flicker-noise coefficients, normalized by active area.
    pub fn flicker_noise_coefficients(&self) -> Option<(Value, Value, Value)> {
        if self.kf <= 0.0 || !self.kf.is_finite() {
            return None;
        }

        let area = (self.cox * self.w.max(1e-18) * self.l.max(1e-18)).max(1e-30);
        Some((self.kf / area, self.af.max(1e-12), self.ef.max(1e-12)))
    }

    //=========================================================================
    // BSIM4-style charge-based model for transient analysis
    // Q = ∫C dV ensures charge conservation (dQ/dt = I)
    //=========================================================================

    /// Calculate total gate charges (Qgs, Qgd, Qgb) using charge-based formulation
    ///
    /// # BSIM4 Charge Model
    /// The charge-based model ensures:
    /// - Charge conservation: Qg = Qgs + Qgd + Qgb
    /// - Correct transient currents: Igs = dQgs/dt
    /// - Smooth transitions between operating regions
    ///
    /// Returns (Qgs, Qgd, Qgb) in Coulombs
    pub fn gate_charges(&self) -> (Value, Value, Value) {
        let (cgs_ov, cgd_ov, cgb_ov) = self.overlap_capacitances();
        let channel_length = if self.level == 6 {
            self.level6_effective_length()
        } else {
            self.l
        };
        let cox_wl = self.cox * self.w * channel_length;

        let p = self.polarity();
        let vgs = p * self.vgs;
        let vds = p * self.vds;
        let vgd = vgs - vds;
        let vgb = vgs - p * self.vbs;
        let vth = p * self.vth(self.vbs);
        let vgt = vgs - vth;

        // Overlap charges (linear with voltage)
        let qgs_ov = cgs_ov * vgs;
        let qgd_ov = cgd_ov * vgd;
        let qgb_ov = cgb_ov * vgb;

        if vgt <= 0.0 {
            // Cutoff: Qgb = Cox * W * L * Vgb, Qgs = Qgd = overlap only
            let qgb_int = cox_wl * vgb;
            (qgs_ov, qgd_ov, qgb_int + qgb_ov)
        } else if vds < vgt {
            // Linear region: symmetric charge sharing
            // Qgs = Qgd = (Cox*W*L/2) * (Vgs - Vth + Vds/2)
            let veff = vgt - vds / 2.0;
            let qgs_int = 0.5 * cox_wl * veff;
            let qgd_int = 0.5 * cox_wl * (veff - vds);
            (qgs_int + qgs_ov, qgd_int + qgd_ov, qgb_ov)
        } else {
            // Saturation: 2/3 of channel charge to source
            // Qgs = (2/3) * Cox * W * L * Vgt
            // Qgd = 0 (pinched off)
            let qgs_int = (2.0 / 3.0) * cox_wl * vgt;
            let qgd_int = 0.0;
            (qgs_int + qgs_ov, qgd_int + qgd_ov, qgb_ov)
        }
    }

    /// Gate-source charge Qgs in Coulombs
    #[inline]
    pub fn qgs(&self) -> Value {
        self.gate_charges().0
    }

    /// Gate-drain charge Qgd in Coulombs
    #[inline]
    pub fn qgd(&self) -> Value {
        self.gate_charges().1
    }

    /// Gate-bulk charge Qgb in Coulombs
    #[inline]
    pub fn qgb(&self) -> Value {
        self.gate_charges().2
    }

    /// Calculate gate current contribution for transient: Ig = dQg/dt
    ///
    /// Given previous charges and timestep, computes:
    /// - Igs = (Qgs - Qgs_prev) / dt
    /// - Igd = (Qgd - Qgd_prev) / dt
    /// - Igb = (Qgb - Qgb_prev) / dt
    pub fn gate_currents(
        &self,
        qgs_prev: Value,
        qgd_prev: Value,
        qgb_prev: Value,
        dt: Value,
    ) -> (Value, Value, Value) {
        if dt <= 0.0 {
            return (0.0, 0.0, 0.0);
        }
        let (qgs, qgd, qgb) = self.gate_charges();
        (
            (qgs - qgs_prev) / dt,
            (qgd - qgd_prev) / dt,
            (qgb - qgb_prev) / dt,
        )
    }

    /// Calculate source/drain series resistance (per side)
    /// Returns resistance in Ohms
    pub fn source_drain_resistance(&self) -> Value {
        // Rsd = RDSW / W (per side, so total is 2x)
        // If RSH is specified, add sheet resistance contribution
        if self.rsh > 0.0 {
            // Assume 1 square of S/D diffusion
            self.rdsw / (self.w * 1e6) + self.rsh
        } else {
            self.rdsw / (self.w * 1e6)
        }
    }

    /// Calculate W/L ratio
    pub fn wl_ratio(&self) -> Value {
        self.w / self.l
    }

    /// Beta = KP * W/L
    pub fn beta(&self) -> Value {
        self.kp * self.wl_ratio()
    }
}
