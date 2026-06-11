use super::*;

impl Mosfet {
    /// Create a new NMOS with default parameters
    pub fn new_nmos(
        name: String,
        drain: NodeId,
        gate: NodeId,
        source: NodeId,
        bulk: NodeId,
    ) -> Self {
        Self::new(name, MosType::Nmos, drain, gate, source, bulk)
    }

    /// Create a new PMOS with default parameters
    pub fn new_pmos(
        name: String,
        drain: NodeId,
        gate: NodeId,
        source: NodeId,
        bulk: NodeId,
    ) -> Self {
        Self::new(name, MosType::Pmos, drain, gate, source, bulk)
    }

    pub(in crate::device::mosfet::mosfet) fn new(
        name: String,
        mos_type: MosType,
        drain: NodeId,
        gate: NodeId,
        source: NodeId,
        bulk: NodeId,
    ) -> Self {
        Self {
            name,
            mos_type,
            node_drain: drain,
            node_gate: gate,
            node_source: source,
            node_bulk: bulk,

            // Default geometry (1um process)
            l: 1e-6,
            w: 10e-6,
            ld: 0.0,

            // Level 1 parameters
            vto: 0.7,       // Threshold voltage
            kp: 110e-6,     // Transconductance (NMOS typical)
            gamma: 0.4,     // Body effect
            phi: 0.65,      // Surface potential
            lambda: 0.01,   // Channel-length modulation
            is_bulk: 1e-14, // Bulk diode saturation current
            js_bulk: 0.0,
            cox: 7e-4, // Oxide capacitance
            cj: 0.0,
            cjsw: 0.0,
            pb: 0.8,
            mj: 0.5,
            mjsw: 0.33,
            fc: 0.5,
            drain_bulk_cap_zero_bias: None,
            source_bulk_cap_zero_bias: None,
            drain_area: 0.0,
            source_area: 0.0,
            drain_perimeter: 0.0,
            source_perimeter: 0.0,
            junction_gmin: 0.0,

            // BSIM3-like parameters
            level: 1, // Default to Level 1
            legacy_bsim_model: None,
            legacy_bsim_sized: None,
            u0: 400.0,    // Low-field mobility (cm^2/V*s)
            ua: 2.25e-9,  // Mobility degradation coefficient
            ub: 5.87e-19, // Second-order mobility coefficient
            vsat: 1.5e5,  // Saturation velocity (m/s)
            eta0: 0.08,   // DIBL coefficient
            etab: -0.07,  // DIBL body-bias coefficient
            nfactor: 1.0, // Subthreshold swing
            pclm: 1.3,    // Channel length modulation
            rdsw: 200.0,  // S/D resistance (ohm*um)

            // Berkeley MOS2 defaults from ngspice mos2set.c
            mos2_substrate_doping: 0.0,
            mos2_narrow_factor: 0.0,
            mos2_crit_field_exp: 0.0,
            mos2_crit_field: 1.0e4,
            mos2_max_drift_vel: 0.0,
            mos2_junction_depth: 0.0,
            mos2_channel_charge: 1.0,
            mos2_fast_surface_state_density: 0.0,

            // BSIM4 parameters
            dvt0: 2.2,     // Short-channel Vth roll-off
            dvt1: 0.53,    // First-order roll-off
            dvt2: -0.032,  // Body-bias dependent roll-off
            k1: 0.53,      // First body effect coefficient
            k2: -0.186,    // Second body effect coefficient
            cgso: 2.4e-10, // Gate-source overlap cap (F/m)
            cgdo: 2.4e-10, // Gate-drain overlap cap (F/m)
            cgbo: 0.0,     // Gate-bulk overlap cap (F/m)
            rsh: 0.0,      // Sheet resistance (ohm/sq)
            kf: 0.0,
            af: 1.0,
            ef: 1.0,
            thermal_noise_gamma: 2.0 / 3.0,

            // Level 6 parameters (double-exponent model)
            kc: 110e-6,    // Current gain (similar to KP)
            nc: 1.0,       // Current exponent
            kv: 0.9,       // Voltage clipping coefficient
            nv: 0.9,       // Voltage exponent
            gamma1: 0.0,   // Secondary back-gate effect
            sigma: 0.0,    // Drain-induced threshold modulation
            lambda0: 0.01, // First-order CLM
            lambda1: 0.0,  // Second-order CLM

            vgs: 0.0,
            vds: 0.0,
            vbs: 0.0,
            eval_vgs: 0.0,
            eval_vds: 0.0,
            eval_vbs: 0.0,
            id: 0.0,
            gm: 0.0,
            gds: 0.0,
            gmb: 0.0,
            id_eq: 0.0,
            region: MosRegion::Cutoff,

            vgs_prev: 0.0,
            vds_prev: 0.0,
            vbs_prev: 0.0,
            eval_vgs_prev: 0.0,
            eval_vds_prev: 0.0,
            eval_vbs_prev: 0.0,
            id_prev: 0.0,
            gm_prev: 0.0,
            gds_prev: 0.0,
            gmb_prev: 0.0,
            ibs_prev: 0.0,
            gbs_prev: 0.0,
            ibd_prev: 0.0,
            gbd_prev: 0.0,
            has_branch_history: false,
            indices: MosfetIndices::default(),
        }
    }

    /// Set device geometry
    pub fn with_geometry(mut self, w: Value, l: Value) -> Self {
        self.w = w;
        self.l = l;
        self.refresh_legacy_bsim_size_params();
        self
    }

    pub(in crate::device::mosfet::mosfet) fn refresh_legacy_bsim_size_params(&mut self) {
        self.legacy_bsim_sized = self
            .legacy_bsim_model
            .as_ref()
            .and_then(|model| model.sized(self.w, self.l));
    }

    #[inline]
    pub fn set_junction_gmin(&mut self, gmin: Value) {
        self.junction_gmin = gmin.max(0.0);
    }

    #[inline]
    pub(in crate::device::mosfet::mosfet) fn terminal_voltage(
        voltages: &[Value],
        node: NodeId,
    ) -> Value {
        if node == 0 { 0.0 } else { voltages[node - 1] }
    }

    #[inline]
    pub(in crate::device::mosfet::mosfet) fn branch_voltages(
        &self,
        voltages: &[Value],
    ) -> (Value, Value, Value) {
        let vd = Self::terminal_voltage(voltages, self.node_drain);
        let vg = Self::terminal_voltage(voltages, self.node_gate);
        let vs = Self::terminal_voltage(voltages, self.node_source);
        let vb = Self::terminal_voltage(voltages, self.node_bulk);
        (vg - vs, vd - vs, vb - vs)
    }

    #[inline]
    pub(in crate::device::mosfet::mosfet) fn model_space_onset_voltage(
        &self,
        vgs: Value,
        vds: Value,
        vbs: Value,
    ) -> Value {
        if let Some(legacy) = &self.legacy_bsim_sized {
            let p = self.polarity();
            let vds_m = p * vds;
            let vbs_m = p * vbs;
            let vbd_m = vbs_m - vds_m;
            let threshold = if vds_m >= 0.0 {
                legacy.threshold(vds_m, vbs_m)
            } else {
                legacy.threshold(-vds_m, vbd_m)
            };
            if threshold.is_finite() {
                threshold
            } else {
                self.polarity() * self.vto
            }
        } else if self.level == 6 {
            self.level6_meyer_state(vgs, vds, vbs).1
        } else if self.level == 2 {
            self.level2_model_space_onset_voltage(vgs, vds, vbs)
        } else {
            self.vth(vbs)
        }
    }

    #[inline]
    pub(in crate::device::mosfet::mosfet) fn body_junction_vcrit(isat: Value) -> Value {
        let vt = Self::body_junction_thermal_voltage();
        if !isat.is_finite() || isat <= 0.0 {
            return vt * 40.0;
        }

        let arg = (vt / ((2.0_f64).sqrt() * isat)).max(1.0);
        vt * arg.ln()
    }

    #[inline]
    pub(in crate::device::mosfet::mosfet) fn source_body_vcrit(&self) -> Value {
        let isat = self.effective_body_junction_saturation_current(self.source_area);
        Self::body_junction_vcrit(isat)
    }

    #[inline]
    pub(in crate::device::mosfet::mosfet) fn drain_body_vcrit(&self) -> Value {
        let isat = self.effective_body_junction_saturation_current(self.drain_area);
        Self::body_junction_vcrit(isat)
    }

    #[inline]
    pub(in crate::device::mosfet::mosfet) fn dev_limvds(vnew: Value, vold: Value) -> Value {
        if vold >= 3.5 {
            if vnew > vold {
                vnew.min(3.0 * vold + 2.0)
            } else if vnew < 3.5 {
                vnew.max(2.0)
            } else {
                vnew
            }
        } else if vnew > vold {
            vnew.min(4.0)
        } else {
            vnew.max(-0.5)
        }
    }

    #[inline]
    pub(in crate::device::mosfet::mosfet) fn dev_pnjlim(
        vnew: Value,
        vold: Value,
        vt: Value,
        vcrit: Value,
    ) -> Value {
        if vnew > vcrit && (vnew - vold).abs() > 2.0 * vt {
            if vold > 0.0 {
                let arg = (vnew - vold) / vt;
                if arg > 0.0 {
                    vold + vt * (2.0 + (arg - 2.0).ln())
                } else {
                    vold - vt * (2.0 + (2.0 - arg).ln())
                }
            } else {
                vt * (vnew / vt).max(1.0).ln()
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

    #[inline]
    pub(in crate::device::mosfet::mosfet) fn dev_fetlim(
        vnew: Value,
        vold: Value,
        vto: Value,
    ) -> Value {
        let vtsthi = (2.0 * (vold - vto)).abs() + 2.0;
        let vtstlo = (vold - vto).abs() + 1.0;
        let vtox = vto + 3.5;
        let delv = vnew - vold;

        if vold >= vto {
            if vold >= vtox {
                if delv <= 0.0 {
                    if vnew >= vtox {
                        if -delv > vtstlo { vold - vtstlo } else { vnew }
                    } else {
                        vnew.max(vto + 2.0)
                    }
                } else if delv >= vtsthi {
                    vold + vtsthi
                } else {
                    vnew
                }
            } else if delv <= 0.0 {
                vnew.max(vto - 0.5)
            } else {
                vnew.min(vto + 4.0)
            }
        } else if delv <= 0.0 {
            if -delv > vtsthi { vold - vtsthi } else { vnew }
        } else {
            let vtemp = vto + 0.5;
            if vnew <= vtemp {
                if delv > vtstlo { vold + vtstlo } else { vnew }
            } else {
                vtemp
            }
        }
    }

    #[inline]
    pub(in crate::device::mosfet::mosfet) fn limited_branch_voltages_for_eval(
        &self,
        vgs: Value,
        vds: Value,
        vbs: Value,
    ) -> (Value, Value, Value) {
        if !self.has_branch_history
            || !self.eval_vgs_prev.is_finite()
            || !self.eval_vds_prev.is_finite()
            || !self.eval_vbs_prev.is_finite()
        {
            return (vgs, vds, vbs);
        }

        let p = self.polarity();
        let mut vgs_m = p * vgs;
        let mut vds_m = p * vds;
        let mut vbs_m = p * vbs;
        let old_vgs = self.eval_vgs_prev;
        let old_vds = self.eval_vds_prev;
        let old_vbs = self.eval_vbs_prev;
        let vold_vgs = p * old_vgs;
        let vold_vds = p * old_vds;
        let vold_vbs = p * old_vbs;
        let mut vbd_m = vbs_m - vds_m;
        let vgd_initial_m = vgs_m - vds_m;
        let vgdo = vold_vgs - vold_vds;
        let von = self.model_space_onset_voltage(old_vgs, old_vds, old_vbs);

        if vold_vds >= 0.0 {
            vgs_m = Self::dev_fetlim(vgs_m, vold_vgs, von);
            vds_m = vgs_m - vgd_initial_m;
            vds_m = Self::dev_limvds(vds_m, vold_vds);
        } else {
            let mut vgd_m = vgd_initial_m;
            vgd_m = Self::dev_fetlim(vgd_m, vgdo, von);
            vds_m = vgs_m - vgd_m;
            vds_m = -Self::dev_limvds(-vds_m, -vold_vds);
            vgs_m = vgd_m + vds_m;
        }

        let vt = Self::body_junction_thermal_voltage();
        if vds_m >= 0.0 {
            vbs_m = Self::dev_pnjlim(vbs_m, vold_vbs, vt, self.source_body_vcrit());
        } else {
            let vold_vbd = vold_vbs - vold_vds;
            vbd_m = Self::dev_pnjlim(vbd_m, vold_vbd, vt, self.drain_body_vcrit());
            vbs_m = vbd_m + vds_m;
        }

        (p * vgs_m, p * vds_m, p * vbs_m)
    }

    #[inline]
    pub(in crate::device::mosfet::mosfet) fn linearized_operating_point(
        &self,
        vgs: Value,
        vds: Value,
        vbs: Value,
    ) -> (Value, MosRegion, Value, Value, Value, Value) {
        if self.legacy_bsim_sized.is_some() {
            let (id, region, gm, gds, gmb) =
                self.legacy_bsim_linearized_operating_point(vgs, vds, vbs);
            let id_eq = id - gm * vgs - gds * vds - gmb * vbs;
            return (id, region, gm, gds, gmb, id_eq);
        }

        if self.level == 6 {
            let (id, region, gm, gds, gmb) = self.level6_operating_point(vgs, vds, vbs);
            let id_eq = id - gm * vgs - gds * vds - gmb * vbs;
            return (id, region, gm, gds, gmb, id_eq);
        }

        if self.level == 2 {
            let (id, region, gm, gds, gmb) = self.level2_operating_point(vgs, vds, vbs);
            let id_eq = id - gm * vgs - gds * vds - gmb * vbs;
            return (id, region, gm, gds, gmb, id_eq);
        }

        if self.level == 1 {
            let (id, region, gm, gds, gmb) = self.level1_operating_point(vgs, vds, vbs);
            let id_eq = id - gm * vgs - gds * vds - gmb * vbs;
            return (id, region, gm, gds, gmb, id_eq);
        }

        let (id, region) = self.calculate_id(vgs, vds, vbs);
        let (gm, gds, gmb) = self.small_signal(vgs, vds, vbs);
        let id_eq = id - gm * vgs - gds * vds - gmb * vbs;
        (id, region, gm, gds, gmb, id_eq)
    }

    /// Whether the linearization cached by `update` is valid for the given
    /// already-limited evaluation voltages. The caller computes the limited
    /// triple once and shares it between this check and any re-evaluation;
    /// limiting is deterministic, so comparing limited values is equivalent
    /// to re-deriving them.
    #[inline]
    pub(in crate::device::mosfet::mosfet) fn cached_linearization_matches_eval(
        &self,
        eval_vgs: Value,
        eval_vds: Value,
        eval_vbs: Value,
    ) -> bool {
        self.eval_vgs.is_finite()
            && self.eval_vds.is_finite()
            && self.eval_vbs.is_finite()
            && self.eval_vgs == eval_vgs
            && self.eval_vds == eval_vds
            && self.eval_vbs == eval_vbs
    }

    /// Set model parameters from a DeviceModel
    pub fn with_params(mut self, params: &std::collections::HashMap<String, Value>) -> Self {
        const EPS0: Value = 8.854_214_871e-12;
        const EPS_SI_REL: Value = 11.7;
        const EPS_OX_REL: Value = 3.9;
        const Q_E: Value = 1.602_176_634e-19;
        const V_T_REF: Value = 0.025_85;
        const REFTEMP: Value = 300.15;
        const N_I_CM3: Value = 1.45e10;

        if let Some(level) = params
            .get("LEVEL")
            .copied()
            .filter(|v| v.is_finite())
            .map(|v| v as i32)
        {
            self.level = level;
        }

        if self.level == 2 {
            // Berkeley MOS2 has distinct model-card defaults from the generic
            // level-1 constructor; explicit parameters below still override.
            self.vto = 0.0;
            self.gamma = 0.0;
            self.phi = 0.6;
            self.lambda = 0.0;
            self.u0 = 600.0;
            self.cox = EPS_OX_REL * EPS0 / 1.0e-7;
            self.kp = self.u0 * 1.0e-4 * self.cox;
            self.cgso = 0.0;
            self.cgdo = 0.0;
            self.cgbo = 0.0;
        }
        if self.level == 6 {
            // Berkeley MOS6 has its own model-card defaults in ngspice
            // mos6set.c; explicit parameters below still override.
            self.vto = 0.0;
            self.gamma = 0.0;
            self.phi = 0.6;
            self.lambda = 0.0;
            self.u0 = 600.0;
            self.cox = 0.0;
            self.kp = 0.0;
            self.cgso = 0.0;
            self.cgdo = 0.0;
            self.cgbo = 0.0;
            self.cj = 0.0;
            self.cjsw = 0.0;
            self.pb = 0.8;
            self.mj = 0.5;
            self.mjsw = 0.5;
            self.fc = 0.5;
            self.kc = 5.0e-5;
            self.nc = 1.0;
            self.kv = 2.0;
            self.nv = 0.5;
            self.gamma1 = 0.0;
            self.sigma = 0.0;
            self.lambda0 = 0.0;
            self.lambda1 = 0.0;
        }

        let kp_explicit = params
            .get("KP")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0);
        let kc_explicit = params
            .get("KC")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0);
        let gamma_explicit = params
            .get("GAMMA")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0);
        let phi_explicit = params
            .get("PHI")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0);
        let vto_explicit = params
            .get("VTO")
            .or_else(|| params.get("VT0"))
            .or_else(|| params.get("VTH0"))
            .copied()
            .filter(|v| v.is_finite());

        let tox = params
            .get("TOX")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0);
        let nsub = params
            .get("NSUB")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0);
        if let Some(v) = nsub {
            self.mos2_substrate_doping = v;
        }

        // Level 1 parameters
        if let Some(v) = vto_explicit {
            self.vto = v;
        }
        if let Some(v) = kp_explicit {
            self.kp = v;
        }
        if let Some(v) = gamma_explicit {
            self.gamma = v;
        }
        if let Some(v) = phi_explicit {
            self.phi = v;
        }
        if let Some(&v) = params.get("LAMBDA") {
            self.lambda = v;
        }
        if let Some(v) = params
            .get("IS")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            self.is_bulk = v;
        }
        if let Some(v) = params
            .get("JS")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            self.js_bulk = v;
        }
        if let Some(&v) = params.get("L") {
            self.l = v;
        }
        if let Some(&v) = params.get("W") {
            self.w = v;
        }
        if let Some(&v) = params.get("LD") {
            self.ld = v.max(0.0);
        }
        if let Some(tox) = tox {
            self.cox = EPS_OX_REL * EPS0 / tox;
        }
        if let Some(v) = params
            .get("CJ")
            .or_else(|| params.get("CJ0"))
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            self.cj = v;
        }
        if let Some(v) = params
            .get("CJSW")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            self.cjsw = v;
        }
        if let Some(v) = params
            .get("PB")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            self.pb = v;
        }
        if let Some(v) = params
            .get("MJ")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            self.mj = v;
        }
        if let Some(v) = params
            .get("MJSW")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            self.mjsw = v;
        }
        if let Some(v) = params
            .get("FC")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0 && *v < 1.0)
        {
            self.fc = v;
        }
        if let Some(v) = params
            .get("CBD")
            .or_else(|| params.get("CAPBD"))
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            self.drain_bulk_cap_zero_bias = Some(v);
        }
        if let Some(v) = params
            .get("CBS")
            .or_else(|| params.get("CAPBS"))
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            self.source_bulk_cap_zero_bias = Some(v);
        }
        if kp_explicit.is_none() {
            // SPICE convention: U0/UO is in cm^2/(V*s), convert to m^2/(V*s).
            let u0_cm = params
                .get("U0")
                .or_else(|| params.get("UO"))
                .copied()
                .filter(|v| v.is_finite() && *v > 0.0);
            let u0_cm = u0_cm.or_else(|| {
                (self.level == 2 && self.u0.is_finite() && self.u0 > 0.0).then_some(self.u0)
            });
            if let Some(u0_cm) = u0_cm {
                let u0_m = u0_cm * 1e-4;
                let kp_derived = u0_m * self.cox;
                if kp_derived.is_finite() && kp_derived > 0.0 {
                    self.kp = kp_derived;
                }
            }
        }
        if (gamma_explicit.is_none() || phi_explicit.is_none())
            && let Some(nsub_cm3) = nsub
        {
            // NSUB is cm^-3 in SPICE model cards.
            let nsub_m3 = nsub_cm3 * 1e6;
            if phi_explicit.is_none() && nsub_cm3 > N_I_CM3 {
                let phi_derived = 2.0 * V_T_REF * (nsub_cm3 / N_I_CM3).ln();
                if phi_derived.is_finite() && phi_derived > 0.0 {
                    self.phi = phi_derived;
                }
            }
            if gamma_explicit.is_none() && self.cox > 0.0 && self.cox.is_finite() {
                let gamma_derived = (2.0 * Q_E * EPS_SI_REL * EPS0 * nsub_m3).sqrt() / self.cox;
                if gamma_derived.is_finite() && gamma_derived >= 0.0 {
                    self.gamma = gamma_derived;
                }
            }
        }
        if vto_explicit.is_none()
            && let Some(nsub_cm3) = nsub
            && nsub_cm3 > N_I_CM3
            && self.cox > 0.0
            && self.cox.is_finite()
            && self.phi > 0.0
        {
            // Match the Berkeley MOS1/2/3/6 preprocessing path: when VT0 is
            // omitted, ngspice derives it from flat-band voltage, substrate
            // doping, surface-state density, and gate material at nominal temp.
            let egfet = 1.16 - (7.02e-4 * REFTEMP * REFTEMP) / (REFTEMP + 1108.0);
            let p = self.polarity();
            let gate_type = params
                .get("TPG")
                .or_else(|| params.get("GATE"))
                .copied()
                .filter(|v| v.is_finite())
                .unwrap_or(1.0);
            let fermis = p * 0.5 * self.phi;
            let wkfng = if gate_type == 0.0 {
                3.2
            } else {
                let fermig = p * gate_type * 0.5 * egfet;
                3.25 + 0.5 * egfet - fermig
            };
            let wkfngs = wkfng - (3.25 + 0.5 * egfet + fermis);
            let surface_state_density = params
                .get("NSS")
                .copied()
                .filter(|v| v.is_finite())
                .unwrap_or(0.0);
            let vfb = wkfngs - surface_state_density * 1.0e4 * Q_E / self.cox;
            let derived_vto = vfb + p * (self.gamma * self.phi.sqrt() + self.phi);
            if derived_vto.is_finite() {
                self.vto = derived_vto;
            }
        }
        // BSIM3 parameters
        if let Some(&v) = params.get("U0").or_else(|| params.get("UO")) {
            self.u0 = v;
        }
        if self.level == 6
            && kc_explicit.is_none()
            && tox.is_some()
            && self.u0.is_finite()
            && self.u0 > 0.0
            && self.cox.is_finite()
            && self.cox > 0.0
        {
            self.kc = 0.5 * self.u0 * 1.0e-4 * self.cox;
        }
        if let Some(&v) = params.get("UA") {
            self.ua = v;
        }
        if let Some(&v) = params.get("UB") {
            self.ub = v;
        }
        if let Some(&v) = params.get("VSAT") {
            self.vsat = v;
        }
        if let Some(&v) = params.get("ETA0") {
            self.eta0 = v;
        }
        if let Some(&v) = params.get("ETAB") {
            self.etab = v;
        }
        if let Some(&v) = params.get("NFACTOR") {
            self.nfactor = v;
        }
        if let Some(&v) = params.get("PCLM") {
            self.pclm = v;
        }
        if let Some(&v) = params.get("RDSW") {
            self.rdsw = v;
        }
        if let Some(v) = params
            .get("DELTA")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            self.mos2_narrow_factor = v;
        }
        if let Some(v) = params
            .get("UEXP")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            self.mos2_crit_field_exp = v;
        }
        if let Some(v) = params
            .get("UCRIT")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            self.mos2_crit_field = v;
        }
        if let Some(v) = params
            .get("VMAX")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            self.mos2_max_drift_vel = v;
        }
        if let Some(v) = params
            .get("XJ")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            self.mos2_junction_depth = v;
        }
        if let Some(v) = params
            .get("NEFF")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            self.mos2_channel_charge = v;
        }
        if let Some(v) = params
            .get("NFS")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            self.mos2_fast_surface_state_density = v;
        }
        // BSIM4 parameters
        if let Some(&v) = params.get("DVT0") {
            self.dvt0 = v;
        }
        if let Some(&v) = params.get("DVT1") {
            self.dvt1 = v;
        }
        if let Some(&v) = params.get("DVT2") {
            self.dvt2 = v;
        }
        if let Some(&v) = params.get("K1") {
            self.k1 = v;
        }
        if let Some(&v) = params.get("K2") {
            self.k2 = v;
        }
        if let Some(&v) = params.get("CGSO") {
            self.cgso = v;
        }
        if let Some(&v) = params.get("CGDO") {
            self.cgdo = v;
        }
        if let Some(&v) = params.get("CGBO") {
            self.cgbo = v;
        }
        if let Some(&v) = params.get("RSH") {
            self.rsh = v;
        }
        if let Some(v) = params
            .get("KF")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            self.kf = v;
        }
        if let Some(v) = params
            .get("AF")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            self.af = v;
        }
        if let Some(v) = params
            .get("EF")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            self.ef = v;
        }
        if let Some(v) = params
            .get("TNOIA")
            .or_else(|| params.get("NOIA"))
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            self.thermal_noise_gamma = (2.0 / 3.0) * v;
        }
        if let Some(v) = params
            .get("GAMMA_NOISE")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            self.thermal_noise_gamma = v;
        }
        // Level 6 parameters
        if let Some(v) = kc_explicit {
            self.kc = v;
        }
        if let Some(&v) = params.get("NC") {
            self.nc = v;
        }
        if let Some(&v) = params.get("KV") {
            self.kv = v;
        }
        if let Some(&v) = params.get("NV") {
            self.nv = v;
        }
        if let Some(&v) = params.get("GAMMA1") {
            self.gamma1 = v;
        }
        if let Some(&v) = params.get("SIGMA") {
            self.sigma = v;
        }
        if let Some(&v) = params.get("LAMBDA0") {
            self.lambda0 = v;
        } else if let Some(&v) = params.get("LAMDA0") {
            self.lambda0 = v;
        } else if let Some(&v) = params.get("LAMBDA") {
            // Some model cards only provide legacy LAMBDA even for higher levels.
            self.lambda0 = v;
        }
        if let Some(&v) = params.get("LAMBDA1") {
            self.lambda1 = v;
        } else if let Some(&v) = params.get("LAMDA1") {
            self.lambda1 = v;
        }

        self.legacy_bsim_model = LegacyBsimModel::from_level_and_params(self.level, params);
        self.refresh_legacy_bsim_size_params();
        self
    }

    /// Set model level (1 = Level 1, 3 = BSIM3-like)
    pub fn with_level(mut self, level: i32) -> Self {
        self.level = level;
        if level != 4 && level != 5 {
            self.legacy_bsim_model = None;
            self.legacy_bsim_sized = None;
        }
        self
    }

    /// Apply MOSFET instance parameters (W/L/M/NF).
    ///
    /// Model-card parameters are expected to be applied first via `with_params`.
    /// Instance parameters then override geometry and optional multiplicity.
    pub fn with_instance_params(mut self, params: &[(String, Value)]) -> Self {
        let mut width_override: Option<Value> = None;
        let mut length_override: Option<Value> = None;
        let mut multiplier = 1.0;
        let mut nf = 1.0;

        for (name, value) in params {
            if !value.is_finite() {
                continue;
            }

            if name.eq_ignore_ascii_case("W") {
                if *value > 0.0 {
                    width_override = Some(*value);
                }
                continue;
            }

            if name.eq_ignore_ascii_case("L") {
                if *value > 0.0 {
                    length_override = Some(*value);
                }
                continue;
            }

            if name.eq_ignore_ascii_case("M") || name.eq_ignore_ascii_case("MULT") {
                if *value > 0.0 {
                    multiplier = *value;
                }
                continue;
            }

            if name.eq_ignore_ascii_case("NF") && *value > 0.0 {
                nf = *value;
                continue;
            }

            if name.eq_ignore_ascii_case("AD") && *value >= 0.0 {
                self.drain_area = *value;
                continue;
            }

            if name.eq_ignore_ascii_case("AS") && *value >= 0.0 {
                self.source_area = *value;
                continue;
            }

            if name.eq_ignore_ascii_case("PD") && *value >= 0.0 {
                self.drain_perimeter = *value;
                continue;
            }

            if name.eq_ignore_ascii_case("PS") && *value >= 0.0 {
                self.source_perimeter = *value;
            }
        }

        if let Some(w) = width_override {
            self.w = w;
        }
        if let Some(l) = length_override {
            self.l = l;
        }

        let scale = multiplier * nf;
        if scale.is_finite() && scale > 0.0 {
            self.w *= scale;
        }

        self.refresh_legacy_bsim_size_params();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn level2_model_defaults_match_berkeley_mos2_baseline() {
        let mut params = HashMap::new();
        params.insert("LEVEL".to_string(), 2.0);

        let mos = Mosfet::new_nmos("m1".to_string(), 1, 2, 3, 4).with_params(&params);

        assert_eq!(mos.level, 2);
        assert_eq!(mos.vto, 0.0);
        assert_eq!(mos.gamma, 0.0);
        assert_eq!(mos.phi, 0.6);
        assert_eq!(mos.lambda, 0.0);
        assert_eq!(mos.u0, 600.0);
        assert_eq!(mos.cgso, 0.0);
        assert_eq!(mos.cgdo, 0.0);
        assert_eq!(mos.cgbo, 0.0);
        assert!(mos.kp > 0.0);
    }

    #[test]
    fn level2_model_defaults_preserve_explicit_parameters() {
        let mut params = HashMap::new();
        params.insert("LEVEL".to_string(), 2.0);
        params.insert("LAMBDA".to_string(), 0.025);
        params.insert("KP".to_string(), 2.0e-5);
        params.insert("PHI".to_string(), 0.7);
        params.insert("CGSO".to_string(), 1.5e-9);

        let mos = Mosfet::new_nmos("m1".to_string(), 1, 2, 3, 4).with_params(&params);

        assert_eq!(mos.lambda, 0.025);
        assert_eq!(mos.kp, 2.0e-5);
        assert_eq!(mos.phi, 0.7);
        assert_eq!(mos.cgso, 1.5e-9);
    }

    #[test]
    fn level6_model_defaults_match_berkeley_mos6_baseline() {
        let mut params = HashMap::new();
        params.insert("LEVEL".to_string(), 6.0);

        let mos = Mosfet::new_nmos("m1".to_string(), 1, 2, 3, 4).with_params(&params);

        assert_eq!(mos.level, 6);
        assert_eq!(mos.vto, 0.0);
        assert_eq!(mos.gamma, 0.0);
        assert_eq!(mos.phi, 0.6);
        assert_eq!(mos.lambda, 0.0);
        assert_eq!(mos.u0, 600.0);
        assert_eq!(mos.cox, 0.0);
        assert_eq!(mos.kp, 0.0);
        assert_eq!(mos.cgso, 0.0);
        assert_eq!(mos.cgdo, 0.0);
        assert_eq!(mos.cgbo, 0.0);
        assert_eq!(mos.cj, 0.0);
        assert_eq!(mos.cjsw, 0.0);
        assert_eq!(mos.pb, 0.8);
        assert_eq!(mos.mj, 0.5);
        assert_eq!(mos.mjsw, 0.5);
        assert_eq!(mos.fc, 0.5);
        assert_eq!(mos.kc, 5.0e-5);
        assert_eq!(mos.nc, 1.0);
        assert_eq!(mos.kv, 2.0);
        assert_eq!(mos.nv, 0.5);
        assert_eq!(mos.gamma1, 0.0);
        assert_eq!(mos.sigma, 0.0);
        assert_eq!(mos.lambda0, 0.0);
        assert_eq!(mos.lambda1, 0.0);
    }

    #[test]
    fn level6_derives_kc_from_tox_and_mobility_when_omitted() {
        let tox = 1.0e-7;
        let mut params = HashMap::new();
        params.insert("LEVEL".to_string(), 6.0);
        params.insert("TOX".to_string(), tox);

        let mos = Mosfet::new_nmos("m1".to_string(), 1, 2, 3, 4).with_params(&params);

        let cox = 3.9 * 8.854_214_871e-12 / tox;
        let expected_kc = 0.5 * 600.0 * 1.0e-4 * cox;
        assert!((mos.kc - expected_kc).abs() <= expected_kc * 1.0e-12);
    }
}
