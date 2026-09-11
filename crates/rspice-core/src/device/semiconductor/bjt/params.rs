//! BJT model parameters, defaults, and temperature scaling.

use super::*;

#[inline]
fn model_parameter_alias(
    params: &std::collections::HashMap<String, Value>,
    names: &[&str],
) -> Option<Value> {
    names.iter().find_map(|name| params.get(*name).copied())
}

impl Bjt {
    pub(crate) const LEGACY_EMISSION_TEMPERATURE_PARAMS: [[&str; 2]; 5] = [
        ["TNF1", "TNF2"],
        ["TNR1", "TNR2"],
        ["TNE1", "TNE2"],
        ["TNC1", "TNC2"],
        ["TNS1", "TNS2"],
    ];

    pub(crate) const LEGACY_BETA_TEMPERATURE_PARAMS: [[&str; 2]; 2] =
        [["TBF1", "TBF2"], ["TBR1", "TBR2"]];
    pub(crate) const LEGACY_CURRENT_TEMPERATURE_PARAMS: [[&str; 2]; 4] = [
        ["TIS1", "TIS2"],
        ["TISE1", "TISE2"],
        ["TISC1", "TISC2"],
        ["TISS1", "TISS2"],
    ];
    pub(crate) const LEGACY_JUNCTION_TEMPERATURE_PARAMS: [[&str; 2]; 3] =
        [["CTE", "TVJE"], ["CTC", "TVJC"], ["CTS", "TVJS"]];
    pub(crate) const LEGACY_GRADING_TEMPERATURE_PARAMS: [[&str; 2]; 3] =
        [["TMJE1", "TMJE2"], ["TMJC1", "TMJC2"], ["TMJS1", "TMJS2"]];
    // Early voltages, current knees, transit times/knee and series resistances.
    pub(crate) const LEGACY_LINEAR_TEMPERATURE_PARAMS: [[&str; 2]; 12] = [
        ["TVAF1", "TVAF2"],
        ["TVAR1", "TVAR2"],
        ["TIKF1", "TIKF2"],
        ["TIKR1", "TIKR2"],
        ["TIRB1", "TIRB2"],
        ["TTF1", "TTF2"],
        ["TTR1", "TTR2"],
        ["TITF1", "TITF2"],
        ["TRC1", "TRC2"],
        ["TRE1", "TRE2"],
        ["TRB1", "TRB2"],
        ["TRM1", "TRM2"],
    ];

    pub(crate) fn legacy_temperature_parameter_names() -> impl Iterator<Item = &'static str> {
        Self::LEGACY_EMISSION_TEMPERATURE_PARAMS
            .iter()
            .chain(Self::LEGACY_BETA_TEMPERATURE_PARAMS.iter())
            .chain(Self::LEGACY_CURRENT_TEMPERATURE_PARAMS.iter())
            .chain(Self::LEGACY_JUNCTION_TEMPERATURE_PARAMS.iter())
            .chain(Self::LEGACY_GRADING_TEMPERATURE_PARAMS.iter())
            .chain(Self::LEGACY_LINEAR_TEMPERATURE_PARAMS.iter())
            .flatten()
            .copied()
            .chain(["TLEV", "TLEVC", "TRC", "TRE", "TRB"])
    }

    #[inline]
    fn legacy_polynomial_delta(coefficients: [Value; 2], delta_t: Value) -> Value {
        delta_t * (coefficients[0] + delta_t * coefficients[1])
    }

    pub(crate) fn validate_legacy_temperature_parameters(&self) -> Result<(), String> {
        let Some(mapping) = self
            .legacy_junction_params
            .as_ref()
            .and_then(|junctions| junctions.temperature_parameters.as_ref())
        else {
            return Ok(());
        };
        if self.charge_model != BjtChargeModel::LegacyGummelPoon || self.xyce_compatibility {
            return Ok(());
        }
        let delta_t = self.temperature - self.tnom.max(1.0);
        let base = self
            .legacy_junction_params
            .as_ref()
            .and_then(|j| j.base_resistance);
        for (index, (value, nominal)) in [
            (self.vaf, mapping.nominal_early[0]),
            (self.var, mapping.nominal_early[1]),
            (self.ikf, self.ikf_nominal),
            (self.ikr, self.ikr_nominal),
            (self.irb, self.irb_nominal),
            (self.tf, mapping.nominal_transit[0]),
            (self.tr, mapping.nominal_transit[1]),
            (self.itf, mapping.nominal_transit[2]),
            (self.rcx, self.rcx_nominal),
            (self.re, self.re_nominal),
            (
                base.map_or(0.0, |r| r.operating[0]),
                base.map_or(0.0, |r| r.nominal[0]),
            ),
            (
                base.map_or(0.0, |r| r.operating[1]),
                base.map_or(0.0, |r| r.nominal[1]),
            ),
        ]
        .into_iter()
        .enumerate()
        {
            // An omitted Early voltage is the disabled infinite limit.
            if index < 2 && nominal == Value::INFINITY {
                continue;
            }
            let positive = index < 4 && nominal > 0.0;
            if !value.is_finite() || value < 0.0 || (positive && value == 0.0) {
                let [first, second] = Self::LEGACY_LINEAR_TEMPERATURE_PARAMS[index];
                return Err(format!(
                    "BJT '{}': {first}/{second} at {} K must yield a finite {} operating parameter, got {value}",
                    self.name,
                    self.temperature,
                    if positive { "positive" } else { "nonnegative" }
                ));
            }
        }
        for (index, (capacitance, potential, grading)) in [
            (self.cje, self.vje, self.mje),
            (self.cjc, self.vjc, self.mjc),
            (self.cjcp, self.ps, self.ms),
        ]
        .into_iter()
        .enumerate()
        {
            let [cap, pot] = Self::LEGACY_JUNCTION_TEMPERATURE_PARAMS[index];
            let [first, second] = Self::LEGACY_GRADING_TEMPERATURE_PARAMS[index];
            if !grading.is_finite() {
                return Err(format!(
                    "BJT '{}': {first}/{second} at {} K must yield a finite junction grading coefficient",
                    self.name, self.temperature
                ));
            }
            if !capacitance.is_finite() || capacitance < 0.0 {
                return Err(format!(
                    "BJT '{}': TLEVC/{cap}/{first}/{second} at {} K must yield a finite nonnegative junction capacitance",
                    self.name, self.temperature
                ));
            }
            if capacitance > 0.0 && (!potential.is_finite() || potential <= 0.0) {
                return Err(format!(
                    "BJT '{}': TLEVC/{pot} at {} K must yield a finite positive junction potential",
                    self.name, self.temperature
                ));
            }
        }
        let linear_beta = 1.0 + self.beta_exp * delta_t;
        if mapping.current_law == 1.0 && (!linear_beta.is_finite() || linear_beta <= 0.0) {
            return Err(format!(
                "BJT '{}': TLEV=1 requires 1+XTB*(T-TNOM) to be positive at {} K",
                self.name, self.temperature
            ));
        }
        for (name, value) in [("BF/TBF1/TBF2", self.bf), ("BR/TBR1/TBR2", self.br)] {
            if !value.is_finite() || value <= 0.0 {
                return Err(format!(
                    "BJT '{}': {name} at {} K must yield a finite positive current gain, got {value}",
                    self.name, self.temperature
                ));
            }
        }
        if mapping.current_law == 3.0 {
            let junctions = self
                .legacy_junction_params
                .as_ref()
                .expect("temperature parameters");
            let (be, bc) = junctions
                .split_saturation
                .unwrap_or((self.is_nominal, self.is_nominal));
            for (name, nominal, index) in [
                ("IS/IBE", be, 0),
                ("IS/IBC", bc, 0),
                ("ISE", self.iben_nominal, 1),
                ("ISC", self.ibcn_nominal, 2),
                ("ISS", junctions.substrate_saturation.unwrap_or(0.0), 3),
            ] {
                if index == 3 && junctions.substrate_saturation.is_none() {
                    continue;
                }
                let power = 1.0
                    + Self::legacy_polynomial_delta(mapping.current_coefficients[index], delta_t);
                if !power.is_finite() || (nominal == 0.0 && power < 0.0) {
                    let [first, second] = Self::LEGACY_CURRENT_TEMPERATURE_PARAMS[index];
                    return Err(format!(
                        "BJT '{}': TLEV=3 {first}/{second} gives invalid power {power} for {name}={nominal} at {} K",
                        self.name, self.temperature
                    ));
                }
            }
        }
        for (index, value) in mapping.operating_emission.iter().enumerate() {
            if !value.is_finite()
                || *value <= 0.0
                || !(value * self.vt).is_finite()
                || value * self.vt <= 0.0
            {
                let [first, second] = Self::LEGACY_EMISSION_TEMPERATURE_PARAMS[index];
                return Err(format!(
                    "BJT '{}': {first}/{second} at {} K must yield a finite positive emission coefficient and thermal voltage, got {value}",
                    self.name, self.temperature
                ));
            }
        }
        Ok(())
    }

    #[inline]
    pub(super) fn apply_legacy_spice_model_defaults(&mut self) {
        self.substrate_topology = BjtSubstrateTopology::default_for_type(self.bjt_type);
        self.is_nominal = 1e-16;
        self.is = self.is_nominal;
        self.bf = 100.0;
        self.br = 1.0;
        self.bf_nominal = self.bf;
        self.br_nominal = self.br;
        self.beta_exp = 0.0;
        self.nf_nominal = 1.0;
        self.nr_nominal = 1.0;
        self.nf = 1.0;
        self.nr = 1.0;
        self.vaf = f64::INFINITY;
        self.var = f64::INFINITY;
        self.rb = 0.0;
        self.rbx = 0.0;
        self.rbi = 0.0;
        self.irb = 0.0;
        // RBP is a VBIC parasitic base path. Legacy Gummel-Poon cards do not
        // expose that branch; clear the constructor's VBIC default before
        // applying any legacy model parameters.
        self.rbp = 0.0;
        self.rbp_nominal = 0.0;
        self.rbx_nominal = 0.0;
        self.rbi_nominal = 0.0;
        self.irb_nominal = 0.0;
        self.rc = 0.0;
        self.rcx = 0.0;
        self.rci = 0.0;
        self.rcx_nominal = 0.0;
        self.rci_nominal = 0.0;
        self.re = 0.0;
        self.re_nominal = 0.0;
        self.cje_nominal = 0.0;
        self.cjc_nominal = 0.0;
        self.cjcp_nominal = 0.0;
        self.cjep_nominal = 0.0;
        self.cbeo_nominal = 0.0;
        self.cbco_nominal = 0.0;
        self.qco_nominal = 0.0;
        self.ccso_nominal = 0.0;
        self.cje = 0.0;
        self.cjc = 0.0;
        self.xcjc = 1.0;
        self.cjcp = 0.0;
        self.cjep = 0.0;
        self.cbeo = 0.0;
        self.cbco = 0.0;
        self.qco = 0.0;
        self.ccso = 0.0;
        self.vje = 0.75;
        self.vjc = 0.75;
        self.ps = 0.75;
        self.vje_nominal = self.vje;
        self.vjc_nominal = self.vjc;
        self.ps_nominal = self.ps;
        self.mje = 0.33;
        self.mjc = 0.33;
        self.ms = 0.0;
        self.fc = 0.5;
        self.tf = 0.0;
        self.qtf = 0.0;
        self.xtf = 0.0;
        self.vtf = 0.0;
        self.itf = 0.0;
        self.tr = 0.0;
        self.td = 0.0;
        self.rth_nominal = 0.0;
        self.tcrth = 0.0;
        self.tminclip = -100.0;
        self.tmaxclip = 500.0;
        self.cth_nominal = 0.0;
        self.rth = 0.0;
        self.cth = 0.0;
        self.selft = 0.0;
        self.selft_given = false;
        self.ikf_nominal = 0.0;
        self.ikr_nominal = 0.0;
        self.ikf = 0.0;
        self.ikr = 0.0;
        self.qbm = 0.0;
        self.nkf = 0.5;
        self.nkf_given = false;
        self.ibei_nominal = 0.0;
        self.iben_nominal = 0.0;
        self.ibci_nominal = 0.0;
        self.ibcn_nominal = 0.0;
        self.ibei = self.ibei_nominal;
        self.iben = self.iben_nominal;
        self.ibci = self.ibci_nominal;
        self.ibcn = self.ibcn_nominal;
        self.nei = 1.0;
        self.nen = 1.5;
        self.nci = 1.0;
        self.ncn = 2.0;
        self.vbbe_nominal = 0.0;
        self.nbbe_nominal = 1.0;
        self.ibbe_nominal = 1e-6;
        self.vbbe = 0.0;
        self.nbbe = 1.0;
        self.ibbe = 1e-6;
        self.ebbe = 1.0;
        self.tvbbe1 = 0.0;
        self.tvbbe2 = 0.0;
        self.tnbbe = 0.0;
        self.vbic_model_gmin = None;
    }

    #[inline]
    pub(super) fn apply_vbic_model_defaults(&mut self) {
        self.substrate_topology = BjtSubstrateTopology::default_for_type(self.bjt_type);
        self.is_nominal = 1e-16;
        self.is = self.is_nominal;
        self.bf_nominal = self.bf;
        self.br_nominal = self.br;
        self.beta_exp = 0.0;
        self.nf_nominal = 1.0;
        self.nr_nominal = 1.0;
        self.nf = 1.0;
        self.nr = 1.0;
        self.vaf = 0.0;
        self.var = 0.0;
        self.tcvef = 0.0;
        self.tcver = 0.0;
        self.rb = 0.0;
        self.rc = 0.0;
        self.rbx = 0.0;
        self.rbi = 0.1;
        self.irb = 0.0;
        self.rcx = 0.0;
        self.rci = 0.1;
        self.re = 0.0;
        self.rs = 0.0;
        self.rbp = 0.1;
        self.rbx_nominal = self.rbx;
        self.rbi_nominal = self.rbi;
        self.irb_nominal = self.irb;
        self.rcx_nominal = self.rcx;
        self.rci_nominal = self.rci;
        self.re_nominal = self.re;
        self.rs_nominal = self.rs;
        self.rbp_nominal = self.rbp;
        self.fc = 0.9;
        self.cbeo_nominal = 0.0;
        self.cbco_nominal = 0.0;
        self.cbeo = 0.0;
        self.cbco = 0.0;
        self.cje_nominal = 0.0;
        self.cjc_nominal = 0.0;
        self.cjep_nominal = 0.0;
        self.cjcp_nominal = 0.0;
        self.cje = 0.0;
        self.cjc = 0.0;
        self.xcjc = 1.0;
        self.cjep = 0.0;
        self.cjcp = 0.0;
        self.vje = 0.75;
        self.vjc = 0.75;
        self.vje_nominal = self.vje;
        self.vjc_nominal = self.vjc;
        self.ps = 0.75;
        self.ps_nominal = self.ps;
        self.mje = 0.33;
        self.mjc = 0.33;
        self.ms = 0.33;
        self.aje = -0.5;
        self.ajc = -0.5;
        self.ajs = -0.5;
        self.qco_nominal = 0.0;
        self.qco = 0.0;
        self.ccso_nominal = 0.0;
        self.ccso = 0.0;
        self.ibei_nominal = 1e-18;
        self.iben_nominal = 0.0;
        self.ibci_nominal = 1e-16;
        self.ibcn_nominal = 0.0;
        self.ibei = self.ibei_nominal;
        self.iben = self.iben_nominal;
        self.ibci = self.ibci_nominal;
        self.ibcn = self.ibcn_nominal;
        self.wbe = 1.0;
        self.vbbe_nominal = 0.0;
        self.nbbe_nominal = 1.0;
        self.ibbe_nominal = 1e-6;
        self.vbbe = 0.0;
        self.nbbe = 1.0;
        self.ibbe = 1e-6;
        self.ebbe = 1.0;
        self.tvbbe1 = 0.0;
        self.tvbbe2 = 0.0;
        self.tnbbe = 0.0;
        self.vbic_model_gmin = None;
        self.nei = 1.0;
        self.nen = 2.0;
        self.nci = 1.0;
        self.ncn = 2.0;
        self.avc1 = 0.0;
        self.avc2_nominal = 0.0;
        self.avcx1 = 0.0;
        self.avcx2_nominal = 0.0;
        self.avcx2 = 0.0;
        self.tavcx = 0.0;
        self.mcx = 0.33;
        self.vbic_maxexp = 1e22;
        self.vbic_model_pnjmaxi = None;
        self.avc2 = 0.0;
        self.isp_nominal = 0.0;
        self.isp = 0.0;
        self.wsp = 1.0;
        self.nfp = 1.0;
        self.ibeip_nominal = 0.0;
        self.ibenp_nominal = 0.0;
        self.ibcip_nominal = 0.0;
        self.ibcnp_nominal = 0.0;
        self.ibeip = 0.0;
        self.ibenp = 0.0;
        self.ibcip = 0.0;
        self.ibcnp = 0.0;
        self.ncip = 1.0;
        self.ncnp = 2.0;
        self.vo_nominal = 0.0;
        self.vo = 0.0;
        self.gamm_nominal = 0.0;
        self.gamm = 0.0;
        self.hrcf = 1.0;
        self.ikf_nominal = 0.0;
        self.ikr_nominal = 0.0;
        self.ikf = 0.0;
        self.ikr = 0.0;
        self.ikp = 0.0;
        self.tf = 0.0;
        self.qtf = 0.0;
        self.xtf = 0.0;
        self.vtf = 0.0;
        self.itf = 0.0;
        self.tr = 0.0;
        self.td = 0.0;
        self.rth_nominal = 0.0;
        self.tcrth = 0.0;
        self.tminclip = -100.0;
        self.tmaxclip = 500.0;
        self.cth_nominal = 0.0;
        self.rth = 0.0;
        self.cth = 0.0;
        self.selft = 0.0;
        self.selft_given = false;
        self.kf = 0.0;
        self.af = 1.0;
        self.kfn = 0.0;
        self.afn = 1.0;
        self.bfn = 1.0;
        self.xre = 0.0;
        self.xrbi = 0.0;
        self.xrci = 0.0;
        self.xrs = 0.0;
        self.xvo = 0.0;
        self.xrbp = 0.0;
        self.ea = 1.12;
        self.eaie = 1.12;
        self.eaic = 1.12;
        self.eais = 1.12;
        self.eane = 1.12;
        self.eanc = 1.12;
        self.eans = 1.12;
        self.eap = 1.12;
        self.xis = 3.0;
        self.xii = 3.0;
        self.xin = 3.0;
        self.tnf = 0.0;
        self.tavc = 0.0;
        self.qbm = 0.0;
        self.nkf = 0.5;
        self.nkf_given = false;
        self.xikf = 0.0;
        self.xrcx = 0.0;
        self.xrbx = 0.0;
        self.isrr_nominal = 1.0;
        self.isrr = 1.0;
        self.xisr = 0.0;
        self.dear = 0.0;
    }

    #[inline]
    pub(crate) fn uses_vbic_dynamic_charges(&self) -> bool {
        self.charge_model == BjtChargeModel::Vbic
    }

    #[inline]
    pub(super) fn vbic_temp_scaled_current(
        nominal: Value,
        r_t: Value,
        vtv: Value,
        temp_exponent: Value,
        activation_energy: Value,
        emission_coeff: Value,
    ) -> Value {
        if nominal <= 0.0 {
            return 0.0;
        }

        let emission = emission_coeff.max(1e-12);
        let ratio_term = r_t.max(1e-18).powf(temp_exponent);
        let energy_term = (-activation_energy * (1.0 - r_t) / vtv.max(1e-18)).clamp(-80.0, 80.0);
        let scaled = (ratio_term * energy_term.exp()).max(0.0);
        nominal * scaled.powf(1.0 / emission)
    }

    #[inline]
    fn legacy_temp_scaled_current(
        nominal: Value,
        factlog: Value,
        log_beta_scale: Value,
        emission_coeff: Value,
        area: Value,
        multiplicity: Value,
    ) -> Value {
        if nominal <= 0.0 {
            return 0.0;
        }

        let emission = emission_coeff.max(1e-12);
        crate::numerics::scaled_exp_product(
            &[nominal, area, multiplicity],
            &[],
            factlog / emission - log_beta_scale,
        )
    }

    #[inline]
    pub(super) fn vbic_temp_scaled_resistance(
        nominal: Value,
        r_t: Value,
        temp_exponent: Value,
    ) -> Value {
        if nominal <= 0.0 {
            return 0.0;
        }

        nominal * r_t.max(1e-18).powf(temp_exponent)
    }

    /// The order also identifies electrical resistance branches in the
    /// temperature-derivative cache mask (floored bits 2..8, active bits 9..15).
    pub(super) fn vbic_series_resistance_parameters(&self) -> [(Value, Value); 7] {
        [
            (self.rcx_nominal, self.xrcx),
            (self.rci_nominal, self.xrci),
            (self.rbx_nominal, self.xrbx),
            (self.rbi_nominal, self.xrbi),
            (self.re_nominal, self.xre),
            (self.rbp_nominal, self.xrbp),
            (self.rs_nominal, self.xrs),
        ]
    }

    #[inline]
    pub(super) fn vbic_log_exp_difference(x: Value) -> Value {
        if x > 40.0 {
            x + (1.0 - (-2.0 * x).exp()).ln()
        } else {
            ((x.exp() - (-x).exp()).max(1e-300)).ln()
        }
    }

    #[inline]
    pub(super) fn vbic_temp_scaled_potential(
        nominal: Value,
        r_t: Value,
        vtv: Value,
        activation_energy: Value,
    ) -> Value {
        if nominal <= 0.0 {
            return nominal;
        }

        let vt_safe = vtv.max(1e-18);
        let ratio = r_t.max(1e-18);
        let arg = 0.5 * nominal * ratio / vt_safe;
        let psiio = 2.0 * (vt_safe / ratio) * Self::vbic_log_exp_difference(arg);
        let psiin = psiio * ratio - 3.0 * vt_safe * ratio.ln() - activation_energy * (ratio - 1.0);
        let expo = (-psiin / vt_safe).clamp(-80.0, 80.0).exp();
        let correction = 0.5 * (1.0 + (1.0 + 4.0 * expo).sqrt());
        (psiin + 2.0 * vt_safe * correction.ln()).max(1e-12)
    }

    /// The public Early voltages stay nominal. Temperature variants already
    /// carry the clipped local temperature, so evaluate the mapping once at
    /// that temperature rather than scaling a previously mapped value.
    pub(super) fn vbic_early_voltages(&self) -> (Value, Value) {
        if !self.vbic_13 {
            return (self.vaf, self.var);
        }
        let delta_t = self.temperature - self.tnom.max(1.0);
        (
            self.vaf * (1.0 + delta_t * self.tcvef),
            self.var * (1.0 + delta_t * self.tcver),
        )
    }

    pub(super) fn refresh_operating_scaling(&mut self) {
        let temp = self.requested_temperature();
        self.refresh_operating_scaling_for(temp);
        if self.vbic_13 {
            self.refresh_vbic_junction_limits();
        }
    }

    pub(crate) fn set_vbic_pnjmaxi(&mut self, value: Value) {
        self.vbic_global_pnjmaxi = value;
        self.refresh_operating_scaling();
    }

    fn refresh_vbic_junction_limits(&mut self) {
        // vbic_1p3.va initializeInstance: use the clipped ambient + TRISE,
        // nominal emission coefficients and rolloff currents, independent of M.
        // Thermal variants call refresh_operating_scaling_for directly and
        // therefore retain these initialization voltages.
        let imax = self.vbic_model_pnjmaxi.unwrap_or(self.vbic_global_pnjmaxi);
        let ratio = self.temperature / self.tnom.max(1.0);
        let vt = self.vt;
        let scaled = |isat, exponent, energy, emission| {
            Self::vbic_temp_scaled_current(isat, ratio, vt, exponent, energy, emission)
        };
        // Compute log(1 + target/isat) without overflowing the ratio or
        // rounding a small positive ratio to zero.
        let limit = |isat: Value, emission: Value, log_target: Value| {
            if isat <= 0.0 {
                return 0.0;
            }
            let x = log_target - isat.ln();
            emission * vt * (x.max(0.0) + (-x.abs()).exp().ln_1p())
        };
        let log_imax = imax.ln();
        let transport_target = |ik: Value| {
            if ik > 0.0 && imax > ik {
                (0.5_f64.ln() + log_imax + self.nkf * (4.0_f64.ln() - ik.ln())) / (1.0 - self.nkf)
            } else {
                log_imax
            }
        };
        let is = scaled(self.is_nominal, self.xis, self.ea, self.nf_nominal);
        let isrr = scaled(self.isrr_nominal, self.xisr, self.dear, self.nr_nominal);
        let base_limit = |isat, exponent, energy, emission| {
            limit(scaled(isat, exponent, energy, emission), emission, log_imax)
        };
        let ibbe = if self.ibbe_nominal > 0.0 {
            let nvt = self.nbbe_nominal * vt;
            let a = -self.vbbe_nominal / nvt;
            let b = log_imax - self.ibbe_nominal.ln();
            nvt * (a.max(b) + (-(a - b).abs()).exp().ln_1p())
        } else {
            0.0
        };
        self.vbic_junction_limits = VbicJunctionLimits {
            ifi: limit(is, self.nf_nominal, transport_target(self.ikf_nominal)),
            iri: limit(
                is * isrr,
                self.nr_nominal,
                transport_target(self.ikr_nominal),
            ),
            ip: limit(
                scaled(self.isp_nominal, self.xis, self.eap, self.nfp),
                self.nfp,
                if self.ikp > 0.0 && imax > self.ikp {
                    2.0 * log_imax - self.ikp.ln()
                } else {
                    log_imax
                },
            ),
            ibei: base_limit(self.ibei_nominal, self.xii, self.eaie, self.nei),
            iben: base_limit(self.iben_nominal, self.xin, self.eane, self.nen),
            ibci: base_limit(self.ibci_nominal, self.xii, self.eaic, self.nci),
            ibcn: base_limit(self.ibcn_nominal, self.xin, self.eanc, self.ncn),
            ibeip: base_limit(self.ibeip_nominal, self.xii, self.eaic, self.nci),
            ibenp: base_limit(self.ibenp_nominal, self.xin, self.eanc, self.ncn),
            ibcip: base_limit(self.ibcip_nominal, self.xii, self.eais, self.ncip),
            ibcnp: base_limit(self.ibcnp_nominal, self.xin, self.eans, self.ncnp),
            ibbe,
        };
    }

    #[inline]
    pub(super) fn clear_thermal_variant_cache(&self) {
        self.thermal_variant_cache.borrow_mut().clear();
    }

    pub(super) fn clone_without_thermal_variant_cache(&self) -> Self {
        let saved_cache = {
            let mut cache = self.thermal_variant_cache.borrow_mut();
            std::mem::take(&mut *cache)
        };
        let clone = self.clone();
        *self.thermal_variant_cache.borrow_mut() = saved_cache;
        clone.thermal_variant_cache.borrow_mut().clear();
        clone
    }

    pub(super) fn with_temperature_variant<R>(
        &self,
        thermal_rise: Value,
        f: impl FnOnce(&Self) -> R,
    ) -> R {
        self.with_temperature_variant_mask(thermal_rise, None, 0, f)
    }

    /// Probe regular temperature dependencies with Early voltages frozen at
    /// the anchor; their potentially singular slopes are differentiated
    /// analytically. Resistance probes retain the anchor's selected branch.
    pub(super) fn with_temperature_derivative_variant<R>(
        &self,
        thermal_rise: Value,
        anchor: Value,
        f: impl FnOnce(&Self) -> R,
    ) -> R {
        let mut mask = 0;
        if self.vbic_13 {
            let temperature = self
                .mapped_temperature(self.requested_temperature() + anchor)
                .0;
            let ratio = temperature / self.tnom.max(1.0);
            for (index, (nominal, exponent)) in self
                .vbic_series_resistance_parameters()
                .into_iter()
                .enumerate()
            {
                if nominal > 0.0 && exponent != 0.0 {
                    let floored =
                        Self::vbic_temp_scaled_resistance(nominal, ratio, exponent) <= 1e-3;
                    mask |= 1 << (index + if floored { 2 } else { 9 });
                }
            }
        }
        let early_anchor =
            (self.vbic_13 && (self.tcvef != 0.0 || self.tcver != 0.0)).then_some(anchor);
        self.with_temperature_variant_mask(thermal_rise, early_anchor, mask, f)
    }

    fn with_temperature_variant_mask<R>(
        &self,
        thermal_rise: Value,
        early_anchor: Option<Value>,
        derivative_mask: u16,
        f: impl FnOnce(&Self) -> R,
    ) -> R {
        if !self.thermal_model_enabled() {
            return f(self);
        }

        let key = BjtThermalVariantKey {
            rise_bits: thermal_rise.to_bits(),
            early_anchor_bits: early_anchor.map(Value::to_bits),
            resistance_branches: derivative_mask,
        };
        {
            let cache = self.thermal_variant_cache.borrow();
            if let Some((_, variant)) = cache.iter().find(|(cached_key, _)| *cached_key == key) {
                return f(variant.as_ref());
            }
        }

        let mut variant = self.clone_without_thermal_variant_cache();
        variant.refresh_operating_scaling_for(self.requested_temperature() + thermal_rise);
        if let Some(anchor) = early_anchor {
            let temperature = self
                .mapped_temperature(self.requested_temperature() + anchor)
                .0;
            let delta_t = temperature - self.tnom.max(1.0);
            variant.vaf = (self.vaf * (1.0 + delta_t * self.tcvef)).max(0.0);
            variant.var = (self.var * (1.0 + delta_t * self.tcver)).max(0.0);
            variant.tcvef = 0.0;
            variant.tcver = 0.0;
        }
        if derivative_mask >> 2 != 0 {
            let scale = variant.instance_scale();
            let parameters = variant.vbic_series_resistance_parameters();
            let ratio = variant.temperature / variant.tnom.max(1.0);
            for (index, resistance) in [
                &mut variant.rcx,
                &mut variant.rci,
                &mut variant.rbx,
                &mut variant.rbi,
                &mut variant.re,
                &mut variant.rbp,
                &mut variant.rs,
            ]
            .into_iter()
            .enumerate()
            {
                if derivative_mask & (1 << (index + 2)) != 0 {
                    *resistance = 1e-3 / scale;
                } else if derivative_mask & (1 << (index + 9)) != 0 {
                    // Continue the active expression across the floor only
                    // for derivatives, as an analytic branch derivative does.
                    // Shrinking the probe at the join can lose its separation
                    // when added to the much larger absolute temperature.
                    let (nominal, exponent) = parameters[index];
                    *resistance =
                        Self::vbic_temp_scaled_resistance(nominal, ratio, exponent) / scale;
                }
            }
        }
        let result = f(&variant);

        let mut cache = self.thermal_variant_cache.borrow_mut();
        if cache.len() >= Self::THERMAL_VARIANT_CACHE_CAPACITY {
            cache.remove(0);
        }
        cache.push((key, Box::new(variant)));
        result
    }

    fn junction_area_factors(&self) -> (Value, Value) {
        if self.charge_model != BjtChargeModel::LegacyGummelPoon || self.xyce_compatibility {
            return (self.area, self.area);
        }
        let areas = self.legacy_junction_params.as_deref();
        let base = areas.and_then(|areas| areas.base).unwrap_or(self.area);
        let collector = areas.and_then(|areas| areas.collector).unwrap_or(self.area);
        match self.substrate_topology {
            BjtSubstrateTopology::Vertical => (base, collector),
            BjtSubstrateTopology::Lateral => (collector, base),
        }
    }

    pub(super) fn legacy_reverse_saturation_current(&self) -> Value {
        self.legacy_junction_params
            .as_ref()
            .and_then(|junctions| junctions.bc_saturation)
            .unwrap_or(self.is * self.isrr.max(0.0))
    }

    fn refresh_legacy_junction_currents(
        &mut self,
        factlog: Value,
        log_beta_scale: Value,
        reverse_ratio: Value,
        bc_area: Value,
        substrate_area: Value,
        delta_t: Value,
    ) {
        let junctions = self.legacy_junction_params.as_deref();
        let split = junctions
            .and_then(|j| j.split_saturation)
            .filter(|_| !self.xyce_compatibility);
        let temperature = junctions
            .and_then(|j| j.temperature_parameters.as_deref())
            .filter(|_| !self.xyce_compatibility);
        let power_law = temperature.filter(|t| t.current_law == 3.0);
        let (be, bc) = split.unwrap_or((self.is_nominal, self.is_nominal));
        let substrate_nominal = junctions
            .and_then(|j| j.substrate_saturation)
            .filter(|_| !self.xyce_compatibility);
        let mut currents = [0.0; 6];
        let mut scales = [None; 6];
        // Each tuple owns its source, geometry and thermal law once. The
        // scalar value and retained scale are derived together from it.
        for (kind, mut factors, mut exponent, power_index) in [
            (
                LegacyCurrent::Forward,
                [be, self.area, self.m, 1.0, 1.0],
                if split.is_some() {
                    factlog / self.nf_nominal
                } else {
                    factlog
                },
                Some(0),
            ),
            (
                LegacyCurrent::Reverse,
                [
                    bc,
                    if split.is_some() { bc_area } else { self.area },
                    self.m,
                    if split.is_some() { 1.0 } else { reverse_ratio },
                    if split.is_some() || self.xyce_compatibility {
                        1.0
                    } else {
                        bc_area
                    },
                ],
                if split.is_some() {
                    factlog / self.nr_nominal
                } else {
                    factlog
                },
                Some(0),
            ),
            (
                LegacyCurrent::BaseLeakage,
                [self.iben_nominal, self.area, self.m, 1.0, 1.0],
                factlog / self.nen.max(1e-12) - log_beta_scale,
                Some(1),
            ),
            (
                LegacyCurrent::CollectorIdealLeakage,
                [self.ibci_nominal, bc_area, self.m, 1.0, 1.0],
                factlog / self.nci.max(1e-12) - log_beta_scale,
                None,
            ),
            (
                LegacyCurrent::CollectorLeakage,
                [self.ibcn_nominal, bc_area, self.m, 1.0, 1.0],
                factlog / self.ncn.max(1e-12) - log_beta_scale,
                Some(2),
            ),
            (
                LegacyCurrent::Substrate,
                [
                    substrate_nominal.unwrap_or(0.0),
                    if split.is_some() {
                        substrate_area
                    } else {
                        self.area
                    },
                    self.m,
                    1.0,
                    1.0,
                ],
                if split.is_some() {
                    factlog / self.nr_nominal
                } else {
                    factlog
                },
                substrate_nominal.map(|_| 3),
            ),
        ] {
            if let Some((law, index)) = power_law.zip(power_index) {
                let power_delta =
                    Self::legacy_polynomial_delta(law.current_coefficients[index], delta_t);
                // nominal^(1+delta) retains its exact nominal value at TNOM.
                // Match pow(0,0)=1; a zero base and negative power is invalid.
                if factors[0] == 0.0 {
                    factors[0] = if power_delta == -1.0 {
                        1.0
                    } else if power_delta < -1.0 {
                        Value::NAN
                    } else {
                        0.0
                    };
                    exponent = 0.0;
                } else {
                    exponent = power_delta * factors[0].ln();
                }
            }
            let mapped = crate::numerics::scaled_exp_product(&factors, &[], exponent);
            let index = kind as usize;
            currents[index] = mapped;
            // Ordinary common-IS transport retains the existing product.
            // Preserve a scale if either intermediate lost precision, even
            // when the completed reverse coefficient is normal.
            let reverse_loss = matches!(kind, LegacyCurrent::Reverse)
                && split.is_none()
                && (!currents[LegacyCurrent::Forward as usize].is_normal()
                    || !self.isrr.is_normal());
            if (!mapped.is_normal() || reverse_loss) && !factors.iter().any(|v| *v <= 0.0) {
                let (mantissa, binary_exponent) = if factors.iter().all(|v| v.is_finite()) {
                    crate::numerics::product_binary_normalization(&factors, &[])
                } else {
                    (Value::NAN, 0)
                };
                scales[index] = Some(LegacyCurrentScale {
                    mantissa,
                    binary_exponent,
                    thermal_exponent: exponent,
                });
            }
        }
        self.is = currents[LegacyCurrent::Forward as usize];
        self.iben = currents[LegacyCurrent::BaseLeakage as usize];
        self.ibci = currents[LegacyCurrent::CollectorIdealLeakage as usize];
        self.ibcn = currents[LegacyCurrent::CollectorLeakage as usize];
        // GP's ideal BE branch comes from transport/BF; preserve this unused
        // VBIC-style coefficient for existing nominal parameter snapshots.
        self.ibei = Self::legacy_temp_scaled_current(
            self.ibei_nominal,
            factlog,
            log_beta_scale,
            self.nei,
            self.area,
            self.m,
        );
        if scales.iter().any(Option::is_some) {
            let junctions = self
                .legacy_junction_params
                .get_or_insert_with(Default::default);
            if let Some(stored) = &mut junctions.current_scales {
                **stored = scales;
            } else {
                junctions.current_scales = Some(Box::new(scales));
            }
        } else if let Some(junctions) = &mut self.legacy_junction_params {
            junctions.current_scales = None;
        }
        if let Some(junctions) = &mut self.legacy_junction_params {
            junctions.bc_saturation = split.map(|_| currents[LegacyCurrent::Reverse as usize]);
            junctions.substrate_current = currents[LegacyCurrent::Substrate as usize];
        }
    }

    pub(super) fn refresh_operating_scaling_for(&mut self, temp: Value) {
        let temp = self.mapped_temperature(temp).0;
        self.clear_thermal_variant_cache();
        // A temperature or instance-parameter change changes the equations
        // even if every terminal voltage stays fixed on the next load.
        self.reduced_linearization_cache_valid.set(false);
        self.previous_reduced_linearization_valid = false;
        self.charge_snapshot_cache_valid.set(false);
        self.mna_limited_from.set(None);
        self.mna_eval = None;
        self.mna_charge_cache_valid.set(false);
        let tnom = self.tnom.max(1.0);
        let vt = self.thermal_voltage_at(temp);
        let ratio = (temp / tnom).max(1e-12);
        let delta_t = temp - tnom;
        let legacy_model = self.charge_model == BjtChargeModel::LegacyGummelPoon;
        // Classic SPICE/Xyce BJT scaling is parameterized by the model's EG
        // bandgap. EA is a distinct VBIC activation-energy parameter.
        let legacy_factlog = (ratio - 1.0) * self.eg / vt.max(1e-18) + self.xis * ratio.ln();
        let temperature_parameters = self
            .legacy_junction_params
            .as_ref()
            .and_then(|j| j.temperature_parameters.as_deref())
            .filter(|_| legacy_model && !self.xyce_compatibility);
        let current_law = temperature_parameters.map_or(0.0, |t| t.current_law);
        let (beta_scale, log_beta_scale) = match current_law {
            1.0 => (
                1.0 + self.beta_exp * delta_t,
                (self.beta_exp * delta_t).ln_1p(),
            ),
            // bfactor must start from one for every model/instance. ngspice's
            // function-local default can inherit another model's last value.
            3.0 => (1.0, 0.0),
            _ => (ratio.powf(self.beta_exp), self.beta_exp * ratio.ln()),
        };
        let beta_factors = temperature_parameters.map_or([beta_scale; 2], |t| {
            t.beta_coefficients.map(|c| {
                c.map_or(beta_scale, |c| {
                    1.0 + Self::legacy_polynomial_delta(c, delta_t)
                })
            })
        });
        let explicit_temperature = temperature_parameters.is_some();
        let grading = temperature_parameters.map_or([self.mje, self.mjc, self.ms], |t| {
            core::array::from_fn(|index| {
                let value = t.nominal_grading[index]
                    * (1.0 + Self::legacy_polynomial_delta(t.grading_coefficients[index], delta_t));
                // Match bjttemp.c's upper limit, retaining nonfinite values
                // for the builder's named operating-domain diagnostic.
                if value.is_finite() && value > 0.999 {
                    if [self.mje, self.mjc, self.ms][index] != 0.999 {
                        log::warn!(
                            "BJT '{}': {} including temperature coefficients is limited to 0.999 at {temp} K",
                            self.name,
                            ["MJE", "MJC", "MJS"][index]
                        );
                    }
                    0.999
                } else {
                    value
                }
            })
        });
        let scale = self.instance_scale();
        let (bc_area, substrate_area) = self.junction_area_factors();
        let isrr_temp = Self::vbic_temp_scaled_current(
            self.isrr_nominal,
            ratio,
            vt,
            self.xisr,
            self.dear,
            self.nr_nominal,
        );
        let gamm_ratio_term = ratio.powf(self.xis);
        let gamm_energy_term = (-self.ea * (1.0 - ratio) / vt.max(1e-18)).clamp(-80.0, 80.0);
        let gamm_temp = self.gamm_nominal * gamm_ratio_term * gamm_energy_term.exp();
        let isp_temp = Self::vbic_temp_scaled_current(
            self.isp_nominal,
            ratio,
            vt,
            self.xis,
            self.eap,
            self.nfp,
        );
        let ibeip_temp = Self::vbic_temp_scaled_current(
            self.ibeip_nominal,
            ratio,
            vt,
            self.xii,
            self.eaic,
            self.nci,
        );
        let ibenp_temp = Self::vbic_temp_scaled_current(
            self.ibenp_nominal,
            ratio,
            vt,
            self.xin,
            self.eanc,
            self.ncn,
        );
        let ibcip_temp = Self::vbic_temp_scaled_current(
            self.ibcip_nominal,
            ratio,
            vt,
            self.xii,
            self.eais,
            self.ncip,
        );
        let ibcnp_temp = Self::vbic_temp_scaled_current(
            self.ibcnp_nominal,
            ratio,
            vt,
            self.xin,
            self.eans,
            self.ncnp,
        );
        let re_temp = Self::vbic_temp_scaled_resistance(self.re_nominal, ratio, self.xre);
        let rbx_temp = Self::vbic_temp_scaled_resistance(self.rbx_nominal, ratio, self.xrbx);
        let rbi_temp = Self::vbic_temp_scaled_resistance(self.rbi_nominal, ratio, self.xrbi);
        let rcx_temp = Self::vbic_temp_scaled_resistance(self.rcx_nominal, ratio, self.xrcx);
        let rci_temp = Self::vbic_temp_scaled_resistance(self.rci_nominal, ratio, self.xrci);
        let rs_temp = Self::vbic_temp_scaled_resistance(self.rs_nominal, ratio, self.xrs);
        let rbp_temp = Self::vbic_temp_scaled_resistance(self.rbp_nominal, ratio, self.xrbp);
        let vo_temp = if self.vo_nominal > 0.0 {
            self.vo_nominal * ratio.powf(self.xvo)
        } else {
            0.0
        };
        let legacy_shifts = legacy_model.then(|| {
            // bjttemp.c / N_DEV_BJT.C silicon bandgap correction. Xyce
            // uses the operating-temperature shift in the nominal inversion
            // as well; ngspice evaluates that shift at TNOM instead.
            let shift = |temperature: Value| {
                let reference_ratio = temperature / crate::constants::TEMP_REFERENCE;
                let bandgap = 1.16 - 7.02e-4 * temperature * temperature / (temperature + 1108.0);
                bandgap
                    - reference_ratio * 1.115_087_7
                    - 3.0 * self.thermal_voltage_at(temperature) * reference_ratio.ln()
            };
            let operating = shift(temp);
            let nominal = if self.xyce_compatibility {
                operating
            } else {
                shift(tnom)
            };
            (nominal, operating)
        });
        let junction =
            |index: usize, potential: Value, capacitance: Value, energy: Value, area: Value| {
                let grading = grading[index];
                if let Some(mapping) = temperature_parameters.filter(|t| t.capacitance_law == 1.0) {
                    let [cap_coefficient, pot_coefficient] = mapping.junction_coefficients[index];
                    return (
                        potential - pot_coefficient * delta_t,
                        crate::numerics::scaled_exp_product(
                            &[capacitance, 1.0 + cap_coefficient * delta_t, area, self.m],
                            &[],
                            0.0,
                        ),
                    );
                }
                if let Some((nominal_shift, operating_shift)) = legacy_shifts {
                    let reference = crate::constants::TEMP_REFERENCE;
                    let pbo = (potential - nominal_shift) / (tnom / reference);
                    let mapped = (temp / reference) * pbo + operating_shift;
                    let old_gamma = (potential - pbo) / pbo;
                    let new_gamma = (mapped - pbo) / pbo;
                    let denominator = 1.0 + grading * (4e-4 * (tnom - reference) - old_gamma);
                    let numerator = 1.0 + grading * (4e-4 * (temp - reference) - new_gamma);
                    let capacitance = crate::numerics::scaled_exp_product(
                        &[capacitance, numerator, area, self.m],
                        &[denominator],
                        0.0,
                    );
                    (mapped, capacitance)
                } else {
                    let mapped = Self::vbic_temp_scaled_potential(potential, ratio, vt, energy);
                    (
                        mapped,
                        (capacitance * (potential / mapped.max(1e-18)).powf(grading) * scale)
                            .max(0.0),
                    )
                }
            };
        let (vje_temp, cje_temp) =
            junction(0, self.vje_nominal, self.cje_nominal, self.eaie, self.area);
        let (vjc_temp, cjc_temp) =
            junction(1, self.vjc_nominal, self.cjc_nominal, self.eaic, bc_area);
        let (ps_temp, cjcp_temp) = if legacy_model && self.xyce_compatibility {
            // Xyce's legacy substrate charge uses nominal CJS and VJS.
            (self.ps_nominal, self.cjcp_nominal * scale)
        } else {
            junction(
                2,
                self.ps_nominal,
                self.cjcp_nominal,
                self.eais,
                substrate_area,
            )
        };
        let (_, cjep_temp) = junction(1, self.vjc_nominal, self.cjep_nominal, self.eaic, self.area);
        let mut nf_temp = self.nf_nominal * (1.0 + delta_t * self.tnf);
        let mut nr_temp = self.nr_nominal * (1.0 + delta_t * self.tnf);
        if legacy_model && !self.xyce_compatibility {
            if let Some(junctions) = &mut self.legacy_junction_params {
                if let Some(mapping) = &mut junctions.temperature_parameters {
                    // bjttemp.c maps the junction slopes independently; the
                    // saturation-current temperature law still uses nominal N.
                    let nominal = [
                        self.nf_nominal,
                        self.nr_nominal,
                        self.nen,
                        self.ncn,
                        junctions.substrate_emission.unwrap_or(1.0),
                    ];
                    for ((operating, nominal), [first, second]) in mapping
                        .operating_emission
                        .iter_mut()
                        .zip(nominal)
                        .zip(mapping.emission_coefficients)
                    {
                        *operating = nominal * (1.0 + delta_t * (first + delta_t * second));
                    }
                    nf_temp = mapping.operating_emission[0];
                    nr_temp = mapping.operating_emission[1];
                }
            }
        }
        let avc2_temp = self.avc2_nominal * (1.0 + (temp - self.tnom) * self.tavc);
        let vbbe_temp = self.vbbe_nominal * (1.0 + delta_t * (self.tvbbe1 + delta_t * self.tvbbe2));
        let nbbe_temp = self.nbbe_nominal * (1.0 + delta_t * self.tnbbe);
        let ikf_temp = if self.ikf_nominal > 0.0 {
            self.ikf_nominal * ratio.powf(self.xikf)
        } else {
            0.0
        };

        self.vt = vt;
        self.temperature = temp;
        let bf = self.bf_nominal * beta_factors[0];
        let br = self.br_nominal * beta_factors[1];
        self.bf = if explicit_temperature {
            bf
        } else {
            bf.max(1e-18)
        };
        self.br = if explicit_temperature {
            br
        } else {
            br.max(1e-18)
        };
        self.nf = if explicit_temperature {
            nf_temp
        } else {
            nf_temp.max(1e-12)
        };
        self.nr = if explicit_temperature {
            nr_temp
        } else {
            nr_temp.max(1e-12)
        };
        // Xyce's VBIC equations multiply every completed current branch by
        // instance M.  Scaling each linear branch resistance by 1/(AREA*M)
        // is the equivalent native-MNA representation; the nonlinear branch
        // current and charge parameters are already multiplied by `scale`
        // below.  The same inverse-area rule is the classic BJT instance
        // contract and keeps legacy AREA/M behavior physically consistent.
        // VBIC 1.3 keeps every electrical resistance branch present. Its
        // physical 1 mOhm floor is applied after temperature mapping and
        // before multiplicity. Older families retain their zero-R topology.
        let resistance_floor = if self.vbic_13 { 1e-3 } else { 0.0 };
        self.re = re_temp.max(resistance_floor) / scale;
        self.rbx = rbx_temp.max(resistance_floor) / scale;
        self.rbi = rbi_temp.max(resistance_floor) / scale;
        // Our base current already includes AREA*M. Both reference models
        // scale IRB by AREA before applying M to their completed equations.
        self.irb = if legacy_model {
            crate::numerics::scaled_exp_product(
                &[self.irb_nominal.max(0.0), self.area, self.m],
                &[],
                0.0,
            )
        } else {
            self.irb_nominal.max(0.0)
        };
        self.rcx = rcx_temp.max(resistance_floor) / scale;
        self.rci = rci_temp.max(resistance_floor) / scale;
        self.vje = vje_temp;
        self.vjc = vjc_temp;
        self.ps = ps_temp;
        [self.mje, self.mjc, self.ms] = grading;
        self.cje = cje_temp;
        self.cjc = cjc_temp;
        self.cjcp = cjcp_temp;
        self.cjep = cjep_temp;
        self.cbeo = (self.cbeo_nominal * scale).max(0.0);
        self.cbco = (self.cbco_nominal * scale).max(0.0);
        self.qco = (self.qco_nominal * scale).max(0.0);
        self.ccso = (self.ccso_nominal * scale).max(0.0);
        self.vo = vo_temp.max(0.0);
        self.gamm = gamm_temp.max(0.0);
        self.ikf = if ikf_temp > 0.0 {
            ikf_temp * scale
        } else {
            0.0
        };
        self.ikr = if self.ikr_nominal > 0.0 {
            self.ikr_nominal * scale
        } else {
            0.0
        };
        // bjttemp.c applies the BC geometry to the already AREA-scaled IS
        // when separate IBE/IBC are absent. This includes the default
        // AREAB/AREAC=AREA. Xyce GP uses only the common AREA multiplier.
        self.isrr = isrr_temp.max(0.0)
            * if legacy_model && !self.xyce_compatibility {
                bc_area
            } else {
                1.0
            };
        if legacy_model {
            self.refresh_legacy_junction_currents(
                legacy_factlog,
                log_beta_scale,
                isrr_temp,
                bc_area,
                substrate_area,
                delta_t,
            );
        } else {
            self.is = Self::vbic_temp_scaled_current(
                self.is_nominal,
                ratio,
                vt,
                self.xis,
                self.ea,
                self.nf_nominal,
            ) * scale;
            self.ibei = (Self::vbic_temp_scaled_current(
                self.ibei_nominal,
                ratio,
                vt,
                self.xii,
                self.eaie,
                self.nei,
            ) * scale)
                .max(0.0);
            self.iben = (Self::vbic_temp_scaled_current(
                self.iben_nominal,
                ratio,
                vt,
                self.xin,
                self.eane,
                self.nen,
            ) * scale)
                .max(0.0);
            self.ibci = (Self::vbic_temp_scaled_current(
                self.ibci_nominal,
                ratio,
                vt,
                self.xii,
                self.eaic,
                self.nci,
            ) * scale)
                .max(0.0);
            self.ibcn = (Self::vbic_temp_scaled_current(
                self.ibcn_nominal,
                ratio,
                vt,
                self.xin,
                self.eanc,
                self.ncn,
            ) * scale)
                .max(0.0);
            if let Some(junctions) = &mut self.legacy_junction_params {
                junctions.bc_saturation = None;
                junctions.substrate_current = 0.0;
                junctions.current_scales = None;
            }
        }
        self.vbbe = if vbbe_temp.is_finite() {
            vbbe_temp
        } else {
            self.vbbe_nominal
        };
        self.nbbe = if nbbe_temp.is_finite() {
            nbbe_temp.max(1e-12)
        } else {
            self.nbbe_nominal.max(1e-12)
        };
        self.ibbe = (self.ibbe_nominal * scale).max(0.0);
        self.ebbe = (-self.vbbe / (self.nbbe * vt.max(1e-18)))
            .clamp(-80.0, 80.0)
            .exp();
        self.isp = (isp_temp * scale).max(0.0);
        self.ibeip = (ibeip_temp * scale).max(0.0);
        self.ibenp = (ibenp_temp * scale).max(0.0);
        self.ibcip = (ibcip_temp * scale).max(0.0);
        self.ibcnp = (ibcnp_temp * scale).max(0.0);
        self.rs = rs_temp.max(resistance_floor) / scale;
        self.rbp = rbp_temp.max(resistance_floor) / scale;
        self.avc2 = if self.vbic_13 {
            avc2_temp
        } else if avc2_temp.is_finite() {
            avc2_temp.max(0.0)
        } else {
            self.avc2_nominal
        };
        self.avcx2 = self.avcx2_nominal * (1.0 + delta_t * self.tavcx);
        self.rth = self.rth_nominal.max(0.0);
        self.cth = self.thermal_capacitance();
        self.refresh_legacy_linear_temperature(delta_t);
        self.refresh_legacy_base_resistance(delta_t, ratio);
    }

    fn refresh_legacy_base_resistance(&mut self, delta_t: Value, ratio: Value) {
        if self.charge_model != BjtChargeModel::LegacyGummelPoon {
            return;
        }
        let Some(junctions) = &mut self.legacy_junction_params else {
            return;
        };
        let Some(base) = &mut junctions.base_resistance else {
            return;
        };
        let scale = |resistance, exponent: Value| {
            crate::numerics::scaled_exp_product(
                &[resistance],
                &[self.area, self.m],
                exponent * ratio.ln(),
            )
        };
        let [whole, minimum] = base.nominal;
        // Preserve the existing RBX/RBI power extensions when no polynomial
        // was authored. An explicit zero polynomial overrides its extension.
        let minimum = scale(minimum, self.xrbx);
        let whole = if self.xrbx == self.xrbi {
            scale(whole, self.xrbi)
        } else {
            minimum + scale(whole - base.nominal[1], self.xrbi)
        };
        base.operating = [whole, minimum];
        if !self.xyce_compatibility
            && let Some(mapping) = &junctions.temperature_parameters
        {
            for index in 0..2 {
                if let Some(coefficients) = mapping.linear_coefficients[10 + index] {
                    let multiplier = 1.0 + Self::legacy_polynomial_delta(coefficients, delta_t);
                    base.operating[index] = crate::numerics::scaled_exp_product(
                        &[base.nominal[index], multiplier],
                        &[self.area, self.m],
                        0.0,
                    );
                }
            }
        }
        self.rb = base.operating[0];
        // RBI marks the private branch and supplies a predictor scale. Its
        // actual conductance uses both mapped parameters in irbi_branch.
        self.rbi = base.operating[0].max(base.operating[1]);
        self.rbx = 0.0;
    }

    fn refresh_legacy_linear_temperature(&mut self, delta_t: Value) {
        if self.charge_model != BjtChargeModel::LegacyGummelPoon || self.xyce_compatibility {
            return;
        }
        let Some(mapping) = self
            .legacy_junction_params
            .as_ref()
            .and_then(|j| j.temperature_parameters.as_ref())
        else {
            return;
        };
        let map = |index: usize,
                   nominal: Value,
                   fallback: Value,
                   factors: [Value; 2],
                   divisors: &[Value]| {
            let Some(coefficients) = mapping.linear_coefficients[index] else {
                return fallback;
            };
            let multiplier = 1.0 + Self::legacy_polynomial_delta(coefficients, delta_t);
            // Include geometry before rounding, just as for saturation currents.
            let factors = [nominal, multiplier, factors[0], factors[1]];
            crate::numerics::scaled_exp_product(&factors, divisors, 0.0)
        };
        let early = |index: usize| {
            let nominal = mapping.nominal_early[index];
            if nominal == Value::INFINITY {
                nominal
            } else {
                map(index, nominal, nominal, [1.0, 1.0], &[])
            }
        };
        self.vaf = early(0);
        self.var = early(1);
        self.ikf = map(2, self.ikf_nominal, self.ikf, [self.area, self.m], &[]);
        self.ikr = map(3, self.ikr_nominal, self.ikr, [self.area, self.m], &[]);
        self.irb = map(4, self.irb_nominal, self.irb, [self.area, self.m], &[]);
        // ITF is in model units: its evaluator applies AREA*M at the bias.
        self.tf = map(
            5,
            mapping.nominal_transit[0],
            mapping.nominal_transit[0],
            [1.0, 1.0],
            &[],
        );
        self.tr = map(
            6,
            mapping.nominal_transit[1],
            mapping.nominal_transit[1],
            [1.0, 1.0],
            &[],
        );
        self.itf = map(
            7,
            mapping.nominal_transit[2],
            mapping.nominal_transit[2],
            [1.0, 1.0],
            &[],
        );
        // Externalization clears these nominal fields. Reusing them here
        // ensures a later refresh cannot resurrect a second series resistor.
        self.rcx = map(
            8,
            self.rcx_nominal,
            self.rcx,
            [1.0, 1.0],
            &[self.area, self.m],
        );
        self.re = map(
            9,
            self.re_nominal,
            self.re,
            [1.0, 1.0],
            &[self.area, self.m],
        );
        self.rc = self.rcx + self.rci;
    }
    /// Set active device temperature (Kelvin).
    pub fn set_temperature(&mut self, temp_k: Value) {
        if temp_k.is_finite() && temp_k > 0.0 {
            self.ambient_temperature = temp_k;
            self.refresh_operating_scaling();
        }
    }

    /// Select Xyce's legacy GP compatibility contract, including its physical
    /// constants and limiter-only device-convergence flag. A subsequent
    /// temperature refresh applies the constant selection to all
    /// temperature-scaled model quantities.
    pub(crate) fn set_xyce_compatibility(&mut self, enabled: bool) {
        self.xyce_compatibility = enabled;
    }

    /// Apply Xyce's global nonlinear-device voltage-limiting policy to this
    /// native BJT. Changing the policy invalidates every bias-dependent cache
    /// because the next load may evaluate at a different junction state.
    pub(crate) fn set_voltage_limiting_enabled(&mut self, enabled: bool) {
        if self.voltage_limiting_enabled != enabled {
            self.voltage_limiting_enabled = enabled;
            self.legacy_junction_limited = false;
            self.reduced_linearization_cache_valid.set(false);
            self.charge_snapshot_cache_valid.set(false);
            self.mna_limited_from.set(None);
        }
    }

    /// Whether a native legacy Gummel-Poon BJT owns Newton globalization via
    /// its local tVcrit/pnjlim path.
    pub(crate) fn uses_legacy_junction_limiting(&self) -> bool {
        self.charge_model == BjtChargeModel::LegacyGummelPoon && self.voltage_limiting_enabled
    }

    /// Set optional substrate node (0 for ground/unconnected).
    pub fn set_substrate_node(&mut self, substrate: NodeId) {
        self.node_substrate = if self.vbic_three_terminal {
            0
        } else {
            substrate
        };
    }

    /// Use a caller-supplied node for the VBIC thermal-rise state.
    ///
    /// Xyce's native VBIC13 `LEVEL=11` has three electrical terminals and
    /// treats the first extra instance terminal as `dt`, not as a substrate.
    /// The parser keeps that token as an instance node; the builder resolves
    /// the model card and calls this method when that node is the external
    /// thermal terminal.
    pub fn set_vbic_external_thermal_node(&mut self, thermal: NodeId) {
        self.node_rth = thermal;
        self.vbic_external_thermal_node = true;
        self.rth = self.rth_nominal.max(0.0);
        self.cth = self.thermal_capacitance();
    }

    /// Set model parameters from a DeviceModel
    pub fn with_params(mut self, params: &std::collections::HashMap<String, Value>) -> Self {
        let mut has_vaf = false;
        let mut has_var = false;
        let mut has_rb = false;
        let mut has_rc = false;
        let mut has_ibei = false;
        let mut has_rth = false;
        let mut legacy_rb: Option<Value> = None;
        let mut legacy_rbm: Option<Value> = None;
        let legacy_irb = model_parameter_alias(params, &["IRB", "JRB", "IOB"])
            .filter(|v| v.is_finite() && *v >= 0.0);
        self.charge_model = if Self::uses_vbic_charge_model(params) {
            BjtChargeModel::Vbic
        } else {
            BjtChargeModel::LegacyGummelPoon
        };
        self.vbic_three_terminal = self.charge_model == BjtChargeModel::Vbic
            && params
                .get("LEVEL")
                .is_some_and(|level| (*level - 11.0).abs() <= 1e-9);
        self.vbic_13 = self.charge_model == BjtChargeModel::Vbic
            && params.get("LEVEL").is_some_and(|level| {
                [11.0, 12.0]
                    .iter()
                    .any(|expected| (*level - expected).abs() <= 1e-9)
            });
        if self.vbic_three_terminal {
            self.node_substrate = 0;
        }
        match self.charge_model {
            BjtChargeModel::LegacyGummelPoon => self.apply_legacy_spice_model_defaults(),
            BjtChargeModel::Vbic => self.apply_vbic_model_defaults(),
        }
        if let Some(junctions) = &mut self.legacy_junction_params {
            junctions.base_resistance = None;
        }
        if self.charge_model == BjtChargeModel::LegacyGummelPoon
            && let Some(v) = legacy_irb
        {
            self.irb_nominal = v;
            self.irb = v;
        }

        // Ngspice switches to split transport only when both fields are
        // authored. A lone IBE or IBC deliberately retains common IS.
        if self.charge_model == BjtChargeModel::LegacyGummelPoon {
            if ["IBE", "IBC", "ISS", "NS"]
                .iter()
                .any(|key| params.contains_key(*key))
            {
                self.legacy_junction_params
                    .get_or_insert_with(Default::default);
            }
            if let Some(junctions) = &mut self.legacy_junction_params {
                junctions.split_saturation = params
                    .get("IBE")
                    .zip(params.get("IBC"))
                    .map(|(&be, &bc)| (be, bc));
                junctions.substrate_saturation = params.get("ISS").copied();
                junctions.substrate_emission = params.get("NS").copied();
            }
        }

        // DC parameters
        if let Some(&v) = params.get("IS") {
            self.is_nominal = v.max(0.0);
        }
        if let Some(v) = model_parameter_alias(params, &["BF", "BFM"]) {
            self.bf = v;
            self.bf_nominal = v;
        }
        if let Some(v) = model_parameter_alias(params, &["BR", "BRM"]) {
            self.br = v;
            self.br_nominal = v;
        }
        if let Some(&v) = params.get("SUBS")
            && v.is_finite()
        {
            self.substrate_topology = BjtSubstrateTopology::from_ngspice_subs(v, self.bjt_type);
        }
        if let Some(&v) = params.get("NF") {
            self.nf_nominal = v;
            self.nf = v;
        }
        if let Some(&v) = params.get("NR") {
            self.nr_nominal = v;
            self.nr = v;
        }
        if let Some(v) = model_parameter_alias(params, &["VAF", "VA", "VBF"]) {
            self.vaf = v;
            has_vaf = true;
        }
        if let Some(v) = model_parameter_alias(params, &["VAR", "VB", "VRB", "BV"]) {
            self.var = v;
            has_var = true;
        }
        if let Some(&v) = params.get("RB") {
            legacy_rb = Some(v.max(0.0));
            has_rb = true;
        }
        if let Some(&v) = params.get("RBM") {
            legacy_rbm = Some(v.max(0.0));
        }
        if let Some(v) = model_parameter_alias(params, &["XTB", "TB", "TCB"])
            && v.is_finite()
        {
            self.beta_exp = v;
        }
        if let Some(&v) = params.get("RC") {
            self.rcx = v.max(0.0);
            self.rcx_nominal = self.rcx;
            self.rci = 0.0;
            self.rci_nominal = self.rci;
            self.rc = self.rcx;
            has_rc = true;
        }
        if let Some(&v) = params.get("RE") {
            self.re = v;
            self.re_nominal = self.re.max(0.0);
        }
        if let Some(&v) = params.get("RS") {
            self.rs_nominal = v.max(0.0);
            self.rs = self.rs_nominal;
        }
        if let Some(&v) = params.get("RBP") {
            self.rbp_nominal = v.max(0.0);
            self.rbp = self.rbp_nominal;
        }
        if let Some(v) = model_parameter_alias(params, &["XTI", "PT"])
            && v.is_finite()
        {
            self.xti = v;
            self.xis = v;
            self.xii = v;
            self.xin = v;
        }
        if let Some(&v) = params.get("XIS")
            && v.is_finite()
        {
            self.xis = v;
        }
        if let Some(&v) = params.get("XII")
            && v.is_finite()
        {
            self.xii = v;
        }
        if let Some(&v) = params.get("XIN")
            && v.is_finite()
        {
            self.xin = v;
        }
        if let Some(&v) = params.get("XISR")
            && v.is_finite()
        {
            self.xisr = v;
        }
        if let Some(&v) = params.get("XRE")
            && v.is_finite()
        {
            self.xre = v;
        }
        if let Some(&v) = params.get("XRBI")
            && v.is_finite()
        {
            self.xrbi = v;
        }
        if let Some(&v) = params.get("XRCI")
            && v.is_finite()
        {
            self.xrci = v;
        }
        if let Some(&v) = params.get("XRS")
            && v.is_finite()
        {
            self.xrs = v;
        }
        if let Some(&v) = params.get("XVO")
            && v.is_finite()
        {
            self.xvo = v;
        }
        if let Some(&v) = params.get("XRBP")
            && v.is_finite()
        {
            self.xrbp = v;
        }
        if let Some(&v) = params.get("TNF")
            && v.is_finite()
        {
            self.tnf = v;
        }
        if let Some(&v) = params.get("XIKF")
            && v.is_finite()
        {
            self.xikf = v;
        }
        if let Some(&v) = params.get("XRCX")
            && v.is_finite()
        {
            self.xrcx = v;
        }
        if let Some(&v) = params.get("XRBX")
            && v.is_finite()
        {
            self.xrbx = v;
        }
        if let Some(&v) = params.get("EG")
            && v.is_finite()
        {
            self.eg = v;
            self.ea = v;
            self.eaie = v;
            self.eaic = v;
            self.eais = v;
            self.eane = v;
            self.eanc = v;
            self.eans = v;
            self.eap = v;
        }
        // VBIC declares these activation energies without a lower bound.
        // Explicit zero/negative values must also override an EG fallback.
        if let Some(&v) = params.get("EA")
            && v.is_finite()
        {
            self.ea = v;
        }
        if let Some(&v) = params.get("EAIE")
            && v.is_finite()
        {
            self.eaie = v;
        }
        if let Some(&v) = params.get("EAIC")
            && v.is_finite()
        {
            self.eaic = v;
        }
        if let Some(&v) = params.get("EANE")
            && v.is_finite()
        {
            self.eane = v;
        }
        if let Some(&v) = params.get("EANC")
            && v.is_finite()
        {
            self.eanc = v;
        }
        if let Some(&v) = params.get("EAIS")
            && v.is_finite()
        {
            self.eais = v;
        }
        if let Some(&v) = params.get("EANS")
            && v.is_finite()
        {
            self.eans = v;
        }
        if let Some(&v) = params.get("EAP")
            && v.is_finite()
        {
            self.eap = v;
        }
        if let Some(&v) = params.get("DEAR")
            && v.is_finite()
        {
            self.dear = v;
        }
        if let Some(&v) = params.get("TNOM")
            && v.is_finite()
            && crate::constants::celsius_to_kelvin(v) > 0.0
        {
            // SPICE/Xyce BJT TNOM is declared U_DEGC: every authored model
            // value is Celsius, including negative temperatures and values
            // above 200 C. Device internals retain Kelvin.
            self.tnom = crate::constants::celsius_to_kelvin(v);
        }
        if self.charge_model == BjtChargeModel::Vbic
            && let Some(&v) = params.get("GMIN")
            && v.is_finite()
            && v >= 0.0
        {
            self.vbic_model_gmin = Some(v);
        }
        if let Some(&v) = params.get("KF") {
            self.kf = v;
        }
        if let Some(&v) = params.get("AF") {
            self.af = v;
        }
        if let Some(&v) = params.get("EF") {
            self.ef = v;
        }
        // Defaults 0/1/1 per vbicsetup.c. Model policy validates the domain:
        // ngspice allows finite signed exponents; VBIC 1.3 requires positive ones.
        if let Some(v) = params
            .get("KFN")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            self.kfn = v;
        }
        if let Some(v) = params.get("AFN").copied().filter(|v| v.is_finite()) {
            self.afn = v;
        }
        if let Some(v) = params.get("BFN").copied().filter(|v| v.is_finite()) {
            self.bfn = v;
        }
        // VBIC aliases used in ngspice level=4 decks.
        if !has_vaf
            && let Some(&v) = params.get("VEF")
            && v.is_finite()
            && v > 0.0
        {
            self.vaf = v;
        }
        if !has_var
            && let Some(&v) = params.get("VER")
            && v.is_finite()
            && v > 0.0
        {
            self.var = v;
        }
        if let Some(rb) = legacy_rb {
            if self.charge_model == BjtChargeModel::LegacyGummelPoon {
                let rbm = legacy_rbm.unwrap_or(rb);
                let varying_temperature = ["TRB", "TRB1", "TRB2", "TRM1", "TRM2"]
                    .iter()
                    .any(|name| params.contains_key(*name));
                if rb > 0.0 && (rbm != rb || varying_temperature) {
                    self.legacy_junction_params
                        .get_or_insert_with(Default::default)
                        .base_resistance = Some(LegacyBaseResistance {
                        nominal: [rb, rbm],
                        operating: [rb, rbm],
                    });
                    self.rbx = 0.0;
                    self.rbi = rb.max(rbm);
                } else {
                    self.rbx = rb;
                    self.rbi = 0.0;
                }
                self.rbx_nominal = self.rbx;
                self.rbi_nominal = self.rbi;
                self.rb = rb;
            } else {
                self.rbx = rb;
                self.rbi = 0.0;
                self.rbx_nominal = self.rbx;
                self.rbi_nominal = self.rbi;
                self.rb = self.rbx;
            }
        } else if legacy_rbm.is_some() && self.charge_model == BjtChargeModel::LegacyGummelPoon {
            // RB=0 (including omission) aliases base-prime to the base in
            // bjtsetup.c. RBM alone cannot introduce a resistance branch.
            self.rbx = 0.0;
            self.rbi = 0.0;
            self.rbx_nominal = self.rbx;
            self.rbi_nominal = self.rbi;
            self.rb = self.rbx;
            has_rb = true;
        }
        if !has_rb {
            // Zero collapses this branch; omission retains its model-family
            // default independently of the other half of the series path.
            for (name, value, nominal) in [
                ("RBX", &mut self.rbx, &mut self.rbx_nominal),
                ("RBI", &mut self.rbi, &mut self.rbi_nominal),
            ] {
                if let Some(&resistance) = params.get(name)
                    && resistance.is_finite()
                    && resistance >= 0.0
                {
                    *value = resistance;
                    *nominal = resistance;
                }
            }
            self.rb = self.rbx + self.rbi;
        }
        if !has_rc {
            for (name, value, nominal) in [
                ("RCX", &mut self.rcx, &mut self.rcx_nominal),
                ("RCI", &mut self.rci, &mut self.rci_nominal),
            ] {
                if let Some(&resistance) = params.get(name)
                    && resistance.is_finite()
                    && resistance >= 0.0
                {
                    *value = resistance;
                    *nominal = resistance;
                }
            }
            self.rc = self.rcx + self.rci;
        }
        if let Some(&v) = params.get("VO")
            && v.is_finite()
            && v >= 0.0
        {
            self.vo = v;
            self.vo_nominal = self.vo;
        }
        if let Some(&v) = params.get("GAMM")
            && v.is_finite()
            && v >= 0.0
        {
            self.gamm = v;
            self.gamm_nominal = self.gamm;
        }
        if let Some(&v) = params.get("HRCF")
            && v.is_finite()
            && v > 0.0
        {
            self.hrcf = v;
        }
        if let Some(&v) = params.get("AVC1")
            && v.is_finite()
        {
            self.avc1 = v.max(0.0);
        }
        if let Some(&v) = params.get("AVC2")
            && v.is_finite()
        {
            self.avc2_nominal = v.max(0.0);
            self.avc2 = self.avc2_nominal;
        }
        if let Some(&v) = params.get("TAVC")
            && v.is_finite()
        {
            self.tavc = v;
        }
        // Gummel-Poon charge parameters
        if let Some(&v) = params.get("CJE") {
            self.cje_nominal = v.max(0.0);
        }
        if let Some(&v) = params.get("CJEP") {
            self.cjep_nominal = v.max(0.0);
        }
        if let Some(v) = model_parameter_alias(params, &["MJE", "ME"]).filter(|v| v.is_finite()) {
            self.mje = v;
        }
        if let Some(v) = model_parameter_alias(params, &["VJE", "PE"])
            && v.is_finite()
            && v > 0.0
        {
            self.vje = v;
            self.vje_nominal = v;
        }
        if let Some(&v) = params.get("AJE")
            && v.is_finite()
        {
            self.aje = v;
        }
        if let Some(&v) = params.get("CJC") {
            self.cjc_nominal = v.max(0.0);
        }
        if let Some(v) = model_parameter_alias(params, &["XCJC", "CDIS"])
            && v.is_finite()
        {
            self.xcjc = v;
        }
        if let Some(&v) = params.get("CBEO")
            && v.is_finite()
        {
            self.cbeo_nominal = v.max(0.0);
        }
        if let Some(&v) = params.get("CBCO")
            && v.is_finite()
        {
            self.cbco_nominal = v.max(0.0);
        }
        if let Some(&v) = params.get("QCO")
            && v.is_finite()
        {
            self.qco_nominal = v.max(0.0);
        }
        if let Some(v) = model_parameter_alias(params, &["CJCP", "CJS", "CCS", "CSUB"]) {
            self.cjcp_nominal = v.max(0.0);
        }
        if let Some(v) = model_parameter_alias(params, &["MJC", "MC"]).filter(|v| v.is_finite()) {
            self.mjc = v;
        }
        if let Some(v) = model_parameter_alias(params, &["VJS", "PS", "PSUB"])
            && v.is_finite()
            && v > 0.0
        {
            self.ps = v;
            self.ps_nominal = v;
        }
        if let Some(v) = model_parameter_alias(params, &["MJS", "MS", "ESUB"])
            && v.is_finite()
        {
            self.ms = v;
        }
        if let Some(&v) = params.get("AJS")
            && v.is_finite()
        {
            self.ajs = v;
        }
        if let Some(v) = model_parameter_alias(params, &["VJC", "PC"])
            && v.is_finite()
            && v > 0.0
        {
            self.vjc = v;
            self.vjc_nominal = v;
        }
        if let Some(&v) = params.get("AJC")
            && v.is_finite()
        {
            self.ajc = v;
        }
        if let Some(&v) = params.get("WBE")
            && v.is_finite()
        {
            self.wbe = v;
        }
        if let Some(&v) = params.get("VBBE")
            && v.is_finite()
        {
            self.vbbe_nominal = v.max(0.0);
            self.vbbe = self.vbbe_nominal;
        }
        if let Some(&v) = params.get("NBBE")
            && v.is_finite()
            && v > 0.0
        {
            self.nbbe_nominal = v;
            self.nbbe = v;
        }
        if let Some(&v) = params.get("IBBE")
            && v.is_finite()
            && v > 0.0
        {
            self.ibbe_nominal = v;
            self.ibbe = v;
        }
        if let Some(&v) = params.get("TVBBE1")
            && v.is_finite()
        {
            self.tvbbe1 = v;
        }
        if let Some(&v) = params.get("TVBBE2")
            && v.is_finite()
        {
            self.tvbbe2 = v;
        }
        if let Some(&v) = params.get("TNBBE")
            && v.is_finite()
        {
            self.tnbbe = v;
        }
        if let Some(&v) = params.get("FC")
            && v.is_finite()
        {
            self.fc = v.clamp(0.0, 0.999_999);
        }
        if let Some(&v) = params.get("TF") {
            self.tf = v;
        }
        if let Some(&v) = params.get("QTF")
            && v.is_finite()
        {
            self.qtf = v;
        }
        if let Some(&v) = params.get("XTF")
            && v.is_finite()
        {
            self.xtf = v;
        }
        if let Some(&v) = params.get("VTF")
            && v.is_finite()
        {
            self.vtf = v.max(0.0);
        }
        if let Some(v) = model_parameter_alias(params, &["ITF", "JTF"])
            && v.is_finite()
        {
            self.itf = v.max(0.0);
        }
        if let Some(&v) = params.get("TR") {
            self.tr = v;
        }
        if let Some(&v) = params.get("TD")
            && v.is_finite()
        {
            self.td = v.max(0.0);
        }
        if self.vbic_13 {
            self.vbic_model_pnjmaxi = params
                .get("PNJMAXI")
                .copied()
                .filter(|value| value.is_finite() && *value > 0.0);
            for (name, destination) in [
                ("AVCX1", &mut self.avcx1),
                ("AVCX2", &mut self.avcx2_nominal),
                ("TAVCX", &mut self.tavcx),
                ("TCVEF", &mut self.tcvef),
                ("TCVER", &mut self.tcver),
                ("MCX", &mut self.mcx),
                ("MAXEXP", &mut self.vbic_maxexp),
            ] {
                if let Some(&value) = params.get(name).filter(|value| value.is_finite()) {
                    *destination = value;
                }
            }
            if let Some(&v) = params.get("TCRTH").filter(|v| v.is_finite()) {
                self.tcrth = v;
            }
            if let Some(&v) = params
                .get("TMINCLIP")
                .filter(|v| (-250.0..=27.0).contains(*v))
            {
                self.tminclip = v;
            }
            if let Some(&v) = params
                .get("TMAXCLIP")
                .filter(|v| (27.0..=1000.0).contains(*v))
            {
                self.tmaxclip = v;
            }
        }
        if let Some(&v) = params.get("RTH")
            && v.is_finite()
        {
            self.rth_nominal = v.max(0.0);
            self.rth = self.rth_nominal;
            has_rth = true;
        }
        if let Some(&v) = params.get("CTH")
            && v.is_finite()
        {
            self.cth_nominal = v.max(0.0);
        }
        if let Some(&v) = params.get("SELFT")
            && v.is_finite()
        {
            self.selft = if v >= 0.5 { 1.0 } else { 0.0 };
            self.selft_given = true;
        }
        if let Some(v) = model_parameter_alias(params, &["IKF", "IK", "JBF"]) {
            self.ikf_nominal = v.max(0.0);
        }
        if let Some(v) = model_parameter_alias(params, &["IKR", "JBR"]) {
            self.ikr_nominal = v.max(0.0);
        }
        if let Some(&v) = params.get("QBM")
            && v.is_finite()
        {
            self.qbm = v;
        }
        if let Some(v) = params
            .get("NK")
            .copied()
            .or_else(|| params.get("NKF").copied())
            && v.is_finite()
            && v > 0.0
        {
            self.nkf = if self.charge_model == BjtChargeModel::LegacyGummelPoon {
                v.min(1.0)
            } else {
                v
            };
            self.nkf_given = true;
        }
        if let Some(v) = params
            .get("ISRR")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            self.isrr_nominal = v;
            self.isrr = v;
        }
        if let Some(&v) = params.get("ISP") {
            self.isp_nominal = v.max(0.0);
            self.isp = self.isp_nominal;
        }
        if let Some(&v) = params.get("WSP")
            && v.is_finite()
        {
            self.wsp = v;
        }
        if let Some(&v) = params.get("NFP")
            && v.is_finite()
            && v > 0.0
        {
            self.nfp = v;
        }
        if let Some(&v) = params.get("IKP") {
            self.ikp = v.max(0.0);
        }
        if let Some(&v) = params.get("IBEI") {
            self.ibei_nominal = v.max(0.0);
            has_ibei = true;
        }
        if self.charge_model == BjtChargeModel::LegacyGummelPoon
            && let Some(v) = model_parameter_alias(params, &["ISE", "JLE"])
        {
            self.iben_nominal = v.max(0.0);
        }
        if let Some(&v) = params.get("IBEN") {
            self.iben_nominal = v.max(0.0);
        }
        if let Some(&v) = params.get("IBCI") {
            self.ibci_nominal = v.max(0.0);
        }
        if self.charge_model == BjtChargeModel::LegacyGummelPoon
            && let Some(v) = model_parameter_alias(params, &["ISC", "JLC"])
        {
            self.ibcn_nominal = v.max(0.0);
        }
        if let Some(&v) = params.get("IBCN") {
            self.ibcn_nominal = v.max(0.0);
        }
        if let Some(&v) = params.get("IBEIP") {
            self.ibeip_nominal = v.max(0.0);
            self.ibeip = self.ibeip_nominal;
        }
        if let Some(&v) = params.get("IBENP") {
            self.ibenp_nominal = v.max(0.0);
            self.ibenp = self.ibenp_nominal;
        }
        if let Some(&v) = params.get("IBCIP") {
            self.ibcip_nominal = v.max(0.0);
            self.ibcip = self.ibcip_nominal;
        }
        if let Some(&v) = params.get("IBCNP") {
            self.ibcnp_nominal = v.max(0.0);
            self.ibcnp = self.ibcnp_nominal;
        }
        if let Some(&v) = params.get("NEI")
            && v.is_finite()
            && v > 0.0
        {
            self.nei = v;
        }
        if let Some(&v) = params.get("NEN")
            && v.is_finite()
            && v > 0.0
        {
            self.nen = v;
        }
        if self.charge_model == BjtChargeModel::LegacyGummelPoon
            && let Some(v) = model_parameter_alias(params, &["NE", "NLE"])
            && v.is_finite()
            && v > 0.0
        {
            self.nen = v;
        }
        if let Some(&v) = params.get("NCI")
            && v.is_finite()
            && v > 0.0
        {
            self.nci = v;
        }
        if let Some(&v) = params.get("NCN")
            && v.is_finite()
            && v > 0.0
        {
            self.ncn = v;
        }
        if self.charge_model == BjtChargeModel::LegacyGummelPoon
            && let Some(&v) = params.get("NC")
            && v.is_finite()
            && v > 0.0
        {
            self.ncn = v;
        }
        if let Some(&v) = params.get("NCIP")
            && v.is_finite()
            && v > 0.0
        {
            self.ncip = v;
        }
        if let Some(&v) = params.get("NCNP")
            && v.is_finite()
            && v > 0.0
        {
            self.ncnp = v;
        }
        if let Some(&v) = params.get("CCSO")
            && v.is_finite()
        {
            self.ccso_nominal = v.max(0.0);
        }
        if !has_ibei && self.charge_model == BjtChargeModel::LegacyGummelPoon {
            self.ibei_nominal = self.is_nominal / self.bf.max(1e-18);
        }
        if self.charge_model == BjtChargeModel::Vbic && !self.vbic_13 && has_rth {
            // ngspice VBIC setup semantics:
            // - If RTH is provided, clamp CTH to at least 1e-12.
            if self.cth_nominal < 1e-12 {
                self.cth_nominal = 1e-12;
            }
        }
        let temperature_controls = self.charge_model == BjtChargeModel::LegacyGummelPoon
            && Self::legacy_temperature_parameter_names().any(|key| params.contains_key(key));
        if temperature_controls {
            let coefficients = core::array::from_fn(|index| {
                let [first, second] = Self::LEGACY_EMISSION_TEMPERATURE_PARAMS[index];
                // Retain the existing shared TNF extension for an axis with
                // no explicit polynomial; even an authored zero overrides it.
                let fallback =
                    if index < 2 && !params.contains_key(first) && !params.contains_key(second) {
                        self.tnf
                    } else {
                        0.0
                    };
                [
                    params.get(first).copied().unwrap_or(fallback),
                    params.get(second).copied().unwrap_or(0.0),
                ]
            });
            self.legacy_junction_params
                .get_or_insert_with(Default::default)
                .temperature_parameters = Some(Box::new(LegacyTemperatureParameters {
                emission_coefficients: coefficients,
                operating_emission: [0.0; 5],
                current_law: params.get("TLEV").copied().unwrap_or(0.0),
                capacitance_law: params.get("TLEVC").copied().unwrap_or(0.0),
                junction_coefficients: Self::LEGACY_JUNCTION_TEMPERATURE_PARAMS
                    .map(|names| names.map(|name| params.get(name).copied().unwrap_or(0.0))),
                grading_coefficients: Self::LEGACY_GRADING_TEMPERATURE_PARAMS
                    .map(|names| names.map(|name| params.get(name).copied().unwrap_or(0.0))),
                nominal_grading: [self.mje, self.mjc, self.ms],
                linear_coefficients: Self::LEGACY_LINEAR_TEMPERATURE_PARAMS.map(
                    |[first, second]| {
                        let alias = match first {
                            "TRC1" => "TRC",
                            "TRE1" => "TRE",
                            "TRB1" => "TRB",
                            _ => first,
                        };
                        let first = params.get(first).or_else(|| params.get(alias)).copied();
                        let second = params.get(second).copied();
                        (first.is_some() || second.is_some())
                            .then(|| [first.unwrap_or(0.0), second.unwrap_or(0.0)])
                    },
                ),
                nominal_early: [self.vaf, self.var],
                nominal_transit: [self.tf, self.tr, self.itf],
                beta_coefficients: Self::LEGACY_BETA_TEMPERATURE_PARAMS.map(|[first, second]| {
                    (params.contains_key(first) || params.contains_key(second)).then(|| {
                        [
                            params.get(first).copied().unwrap_or(0.0),
                            params.get(second).copied().unwrap_or(0.0),
                        ]
                    })
                }),
                current_coefficients: Self::LEGACY_CURRENT_TEMPERATURE_PARAMS.map(
                    |[first, second]| {
                        [
                            params.get(first).copied().unwrap_or(0.0),
                            params.get(second).copied().unwrap_or(0.0),
                        ]
                    },
                ),
            }));
        } else if let Some(junctions) = &mut self.legacy_junction_params {
            junctions.temperature_parameters = None;
        }
        self.refresh_operating_scaling();
        self
    }

    /// Clear the collector series resistance after the builder externalizes
    /// it onto a real circuit resistor; the nominal is cleared too so any
    /// later temperature refresh cannot resurrect the internal copy.
    pub fn clear_collector_series_resistance(&mut self) {
        self.rcx = 0.0;
        self.rcx_nominal = 0.0;
        self.rc = self.rci;
    }

    /// Move the authored collector lead onto the builder-created series
    /// resistor while retaining enough typed topology to report current at
    /// that external lead during transient analysis.
    pub(crate) fn externalize_legacy_collector_lead(
        &mut self,
        internal_node: NodeId,
        resistance: Value,
    ) {
        debug_assert!(resistance.is_finite() && resistance > 0.0);
        self.legacy_collector_lead = Some((self.node_collector, resistance.recip()));
        self.node_collector = internal_node;
    }

    /// Clear the emitter series resistance after the builder externalizes
    /// it onto a real circuit resistor.
    pub fn clear_emitter_series_resistance(&mut self) {
        self.re = 0.0;
        self.re_nominal = 0.0;
    }

    /// Move the authored emitter lead onto the builder-created series
    /// resistor while retaining its transient-output topology.
    pub(crate) fn externalize_legacy_emitter_lead(
        &mut self,
        internal_node: NodeId,
        resistance: Value,
    ) {
        debug_assert!(resistance.is_finite() && resistance > 0.0);
        self.legacy_emitter_lead = Some((self.node_emitter, resistance.recip()));
        self.node_emitter = internal_node;
    }

    /// Clear a constant base resistance after builder externalization. Varying
    /// GP resistance is retained as one private branch and is not externalized.
    /// Clear the nominal too so temperature refresh cannot restore a duplicate.
    pub fn clear_base_constant_resistance(&mut self) {
        self.rbx = 0.0;
        self.rbx_nominal = 0.0;
        self.rb = self.rbi.max(0.0);
    }

    /// Move the authored base lead onto the builder-created constant series
    /// resistor while retaining its transient-output topology. Any remaining
    /// bias-dependent base resistance stays inside the BJT.
    pub(crate) fn externalize_legacy_base_lead(
        &mut self,
        internal_node: NodeId,
        resistance: Value,
    ) {
        debug_assert!(resistance.is_finite() && resistance > 0.0);
        self.legacy_base_lead = Some((self.node_base, resistance.recip()));
        self.node_base = internal_node;
    }

    /// Resolve the thermal-noise temperature offset, bjtnoise.c/vbicnoise.c
    /// semantics: DTEMP directly, or with an absolute instance TEMP given,
    /// temp − CKTtemp + tnom in Celsius terms (ngspice's quirk, mirrored).
    pub fn refresh_noise_temperature_offset(&mut self, analysis_temp_k: Value, tnom_c: Value) {
        self.noise_temperature_offset = match self.instance_temp {
            Some(temp_k) => temp_k - analysis_temp_k + tnom_c,
            None => self.instance_dtemp,
        };
    }

    /// Cached operating-point values from the last accepted Newton solution:
    /// `(vbe, vbc, ic, ib, gm)` where `gm = dIc/dVbe` at the bias point.
    pub fn op_values(&self) -> (Value, Value, Value, Value, Value) {
        (
            self.vbe,
            self.vbc,
            self.ic,
            self.ib,
            self.intrinsic_linearization.dic_dvbe,
        )
    }

    /// Cached static terminal currents in the canonical C/B/E/S order.
    pub(crate) fn operating_point_terminal_currents(&self) -> [Value; 4] {
        [self.ic, self.ib, self.ie, self.isub]
    }

    /// Map intrinsic accepted-step terminal currents to the authored external
    /// leads. A builder-externalized RC/RB/RE owns the external lead current;
    /// its positive-to-negative orientation is authored lead to prime node,
    /// exactly matching the SPICE convention of current entering the device.
    pub(crate) fn authored_transient_lead_currents(
        &self,
        solution: &[Value],
        intrinsic: [Value; 4],
        external_bc_current: Value,
    ) -> Result<[Value; 4], String> {
        let node_voltage = |node: NodeId| {
            if node == 0 {
                Ok(0.0)
            } else {
                solution.get(node - 1).copied().ok_or_else(|| {
                    format!(
                        "BJT '{}' authored lead node {node} is outside solution length {}",
                        self.name,
                        solution.len()
                    )
                })
            }
        };
        let mapped = |lead: Option<(NodeId, Value)>,
                      internal: NodeId,
                      fallback: Value|
         -> Result<Value, String> {
            lead.map_or(Ok(fallback), |(external, conductance)| {
                Ok(conductance * (node_voltage(external)? - node_voltage(internal)?))
            })
        };
        let mut currents = [
            mapped(
                self.legacy_collector_lead,
                self.node_collector,
                intrinsic[0],
            )?,
            mapped(self.legacy_base_lead, self.node_base, intrinsic[1])?,
            mapped(self.legacy_emitter_lead, self.node_emitter, intrinsic[2])?,
            intrinsic[3],
        ];
        if self.legacy_external_bc_charge_nodes().is_some() {
            currents[1] += external_bc_current;
            // RC's measured lead current already includes the direct charge.
            if self.legacy_collector_lead.is_none() {
                currents[0] -= external_bc_current;
            }
        }
        Ok(currents)
    }

    /// Apply instance-level BJT scaling and thermal overrides.
    ///
    /// Supported keys:
    /// - `AREA`: emitter area multiplier (default 1)
    /// - `AREAB` / `AREAC`: ngspice GP base/collector areas (default AREA)
    /// - `M` / `MULT`: multiplicity (default 1)
    /// - `OFF`: start both junctions from their zero-bias state
    /// - `IC_VBE` / `IC_VCE`: the `IC=` vector components, read only by the
    ///   `UIC` transient startup
    /// - `TEMP`: absolute device temperature in Celsius
    /// - `DTEMP`: temperature delta in Celsius (VBIC also accepts TRISE/DTA)
    /// - VBIC `SW_ET` / `SW_NOISE`: heat-generation / noise switches
    pub fn with_instance_params(mut self, params: &[(String, Value)]) -> Self {
        for (name, value) in params {
            if !value.is_finite() {
                continue;
            }

            if name.eq_ignore_ascii_case("AREA") {
                if *value > 0.0 {
                    self.area = *value;
                }
                continue;
            }

            if name.eq_ignore_ascii_case("AREAB") || name.eq_ignore_ascii_case("AREAC") {
                let areas = self
                    .legacy_junction_params
                    .get_or_insert_with(Default::default);
                if name.eq_ignore_ascii_case("AREAB") {
                    areas.base = Some(*value);
                } else {
                    areas.collector = Some(*value);
                }
                continue;
            }

            if name.eq_ignore_ascii_case("M") || name.eq_ignore_ascii_case("MULT") {
                if *value > 0.0 {
                    self.m = *value;
                }
                continue;
            }

            if name.eq_ignore_ascii_case("OFF") {
                self.initial_off = *value != 0.0;
                continue;
            }

            if name.eq_ignore_ascii_case("IC_VBE") {
                self.initial_condition_vbe = Some(*value);
                continue;
            }

            if name.eq_ignore_ascii_case("IC_VCE") {
                self.initial_condition_vce = Some(*value);
                continue;
            }

            if name.eq_ignore_ascii_case("TEMP") {
                self.instance_temp = Some(*value + 273.15);
                continue;
            }

            if name.eq_ignore_ascii_case("DTEMP")
                || (self.uses_vbic_dynamic_charges()
                    && (name.eq_ignore_ascii_case("TRISE") || name.eq_ignore_ascii_case("DTA")))
            {
                self.instance_dtemp = *value;
            }
            if self.uses_vbic_dynamic_charges() {
                if name.eq_ignore_ascii_case("SW_ET") {
                    self.vbic_heat_generation = *value != 0.0;
                } else if name.eq_ignore_ascii_case("SW_NOISE") {
                    self.vbic_noise_enabled = *value != 0.0;
                }
            }
        }

        self.refresh_operating_scaling();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn legacy_temperature_mapped_charges_have_continuous_consistent_derivatives() {
        for (xyce, law) in [
            (false, None),
            (true, None),
            (false, Some(0.0)),
            (false, Some(1.0)),
        ] {
            let mut params = vec![
                ("IS", 0.0),
                ("CJE", 2e-12),
                ("VJE", 0.83),
                ("MJE", 0.37),
                ("CJC", 3e-12),
                ("VJC", 0.68),
                ("MJC", 0.41),
                ("XCJC", 0.3),
                ("CJS", 5e-12),
                ("VJS", 0.91),
                ("MJS", 0.23),
                ("FC", 0.4),
                ("TNOM", 50.0),
            ];
            if let Some(law) = law {
                params.extend([
                    ("TLEVC", law),
                    ("CTE", 0.001),
                    ("CTC", -0.0007),
                    ("CTS", 0.002),
                    ("TVJE", 0.001),
                    ("TVJC", -0.0005),
                    ("TVJS", 0.0008),
                    ("TMJE1", 0.002),
                    ("TMJE2", 0.00001),
                    ("TMJC1", -0.001),
                    ("TMJC2", 0.00002),
                    ("TMJS1", 0.003),
                    ("TMJS2", -0.00001),
                ]);
            }
            let mut model = model_with(&params);
            model.set_xyce_compatibility(xyce);
            model.set_junction_gmin(0.0);
            for temperature in [233.15, 300.15, 323.15, 398.15, 233.15, 323.15] {
                model.set_temperature(temperature);
                model.validate_legacy_temperature_parameters().unwrap();
                // A warm/cold round trip must always start from nominal M.
                let dt = temperature - 323.15;
                for (actual, nominal, first, second) in [
                    (model.mje, 0.37, 0.002, 0.00001),
                    (model.mjc, 0.41, -0.001, 0.00002),
                    (model.ms, 0.23, 0.003, -0.00001),
                ] {
                    let expected = if law.is_some() {
                        nominal * (1.0 + first * dt + second * dt * dt)
                    } else {
                        nominal
                    };
                    assert!((actual - expected).abs() < 1e-15);
                }
                let sample = |v| {
                    let state = model.legacy_transient_charge_state_with_vbx(v, v, v, -v);
                    [
                        (state.qbe, state.capbe),
                        (state.qbc, state.capbc),
                        (state.qbx, state.capbx),
                        (-state.qcs, state.capcs),
                    ]
                };
                // Keep each charge and its Jacobian continuous at the mapped
                // junction transition, including the external BC partition.
                for join in [model.fc * model.vje, model.fc * model.vjc, 0.0] {
                    let h = 1e-6;
                    for v in [-0.4, join - 1e-5, join, join + 1e-5, 0.6] {
                        for ((_, derivative), ((left, _), (right, _))) in sample(v)
                            .into_iter()
                            .zip(sample(v - h).into_iter().zip(sample(v + h)))
                        {
                            let finite_difference = (right - left) / (2.0 * h);
                            assert!(
                                (finite_difference - derivative).abs() < derivative * 2e-8,
                                "Xyce={xyce} T={temperature} V={v}: {finite_difference:e} vs {derivative:e}"
                            );
                        }
                    }
                    for ((ql, cl), (qr, cr)) in
                        sample(join - 1e-9).into_iter().zip(sample(join + 1e-9))
                    {
                        assert!((cl - cr).abs() < cl * 2e-8);
                        assert!(((qr - ql) / 2e-9 - cl).abs() < cl * 2e-6);
                    }
                }
            }
        }
    }

    #[test]
    fn legacy_transport_temperature_refresh_retains_nominals_and_externalized_resistors() {
        let mut parameters = vec![
            ("VAF", 40.0),
            ("VAR", 15.0),
            ("IKF", 2e-3),
            ("IKR", 3e-3),
            ("IRB", 1e-5),
            ("TF", 2e-9),
            ("TR", 3e-9),
            ("ITF", 5e-4),
            ("RC", 8.0),
            ("RE", 3.0),
        ];
        for [first, second] in Bjt::LEGACY_LINEAR_TEMPERATURE_PARAMS {
            parameters.extend([(first, 0.002), (second, 0.00001)]);
        }
        let mut model = model_with(&parameters)
            .with_instance_params(&[("AREA".into(), 2.0), ("M".into(), 3.0)]);
        for temperature in [233.15, 343.15, 300.15, 343.15] {
            model.set_temperature(temperature);
            model.validate_legacy_temperature_parameters().unwrap();
            let dt = temperature - 300.15;
            let factor = 1.0 + 0.002 * dt + 0.00001 * dt * dt;
            for (actual, nominal) in [
                (model.vaf, 40.0),
                (model.var, 15.0),
                (model.ikf, 12e-3),
                (model.ikr, 18e-3),
                (model.irb, 6e-5),
                (model.tf, 2e-9),
                (model.tr, 3e-9),
                (model.itf, 5e-4),
                (model.rcx, 8.0 / 6.0),
                (model.re, 0.5),
            ] {
                assert!((actual / (nominal * factor) - 1.0).abs() < 2e-15);
            }
        }
        model.clear_collector_series_resistance();
        model.clear_emitter_series_resistance();
        model.set_temperature(320.15);
        assert_eq!((model.rcx, model.rc, model.re), (0.0, 0.0, 0.0));
        model.validate_legacy_temperature_parameters().unwrap();

        // Explicit zero polynomials override the older shared power-law
        // extensions; omission retains those established extension semantics.
        for explicit in [false, true] {
            let mut parameters = vec![
                ("LEVEL", 1.0),
                ("IKF", 1.0),
                ("RC", 2.0),
                ("RE", 3.0),
                ("XIKF", 2.0),
                ("XRCX", 2.0),
                ("XRE", 2.0),
            ];
            if explicit {
                parameters.extend([("TIKF1", 0.0), ("TRC", 0.0), ("TRE2", 0.0)]);
            }
            let mut model = model_with(&parameters);
            model.set_temperature(600.3);
            let factor = if explicit { 1.0 } else { 4.0 };
            assert_eq!(
                (model.ikf, model.rcx, model.re),
                (factor, 2.0 * factor, 3.0 * factor)
            );
        }
        for xyce in [false, true] {
            let mut model = model_with(&[("JRB", 1e-5)])
                .with_instance_params(&[("AREA".into(), 2.0), ("M".into(), 3.0)]);
            model.set_xyce_compatibility(xyce);
            for temperature in [233.15, 343.15] {
                model.set_temperature(temperature);
                assert!((model.irb / 6e-5 - 1.0).abs() < 2e-15);
            }
        }
    }

    #[test]
    fn legacy_current_range_survives_temperature_refresh_and_overflow() {
        let mut model = model_with(&[("IS", 1e-14)]);
        for temperature in [10.0, 300.15, 17.5, 300.15] {
            model.set_temperature(temperature);
            let ratio = temperature / 300.15;
            let log_is = 1e-14_f64.ln() + (ratio - 1.0) * 1.11 / model.vt + 3.0 * ratio.ln();
            let expected = (log_is + 1.1 / model.vt).exp() - log_is.exp();
            let (current, slope) =
                model.legacy_junction_iv(LegacyCurrent::Forward, model.is, 1.1, 1.0);
            assert!((current / expected - 1.0).abs() < 1e-11);
            assert!((slope / ((log_is + 1.1 / model.vt).exp() / model.vt) - 1.0).abs() < 1e-11);
            let (_, critical, _) = model.legacy_limiting_parameters(0.0);
            let expected_critical =
                model.vt * (model.vt.ln() - 0.5 * core::f64::consts::LN_2 - log_is);
            assert!((critical - expected_critical).abs() < 1e-12);
            assert_eq!(
                model.legacy_current_scale(LegacyCurrent::Forward).is_some(),
                temperature != 300.15
            );
        }
        model.set_temperature(10.0);
        model = model.with_params(&[("IS".to_string(), 0.0)].into_iter().collect());
        assert_eq!(
            model.legacy_junction_iv(LegacyCurrent::Forward, model.is, 1.1, 1.0),
            (0.0, 0.0)
        );
        assert!(model.legacy_current_scale(LegacyCurrent::Forward).is_none());

        // AREA*IS overflows, but tiny signed bias and a large emission
        // coefficient give finite current AND differential conductance.
        for area in [1.0, 4.0] {
            let large = model_with(&[("IS", 1e308), ("NF", 1e6)])
                .with_instance_params(&[("AREA".to_string(), area)]);
            assert_eq!(large.is.is_infinite(), area == 4.0);
            let nvt = large.nf * large.vt;
            for voltage in [-1e-10, -1e-320, 0.0, 1e-320, 1e-10] {
                let (current, slope) =
                    large.legacy_junction_iv(LegacyCurrent::Forward, large.is, voltage, large.nf);
                // All biases are below 4e-15 nVT, where the linear term
                // differs from exp_m1 by less than 2e-15 relatively.
                let expected = (1e308 * voltage / nvt) * area;
                assert!((current - expected).abs() <= expected.abs() * 1e-13);
                assert!((slope / ((1e308 / nvt) * area) - 1.0).abs() < 1e-13);
            }
            assert_eq!(large.legacy_limiting_parameters(0.0).1, 0.0);
        }
    }

    #[test]
    fn bjt_small_instances_preserve_currents_knees_capacitances_and_resistances() {
        for level in [1.0, 4.0, 11.0, 12.0] {
            let base = model_with(&[
                ("LEVEL", level),
                ("IS", 1e-14),
                ("IKF", 1e-3),
                ("IKR", 2e-3),
                ("CJE", 1e-12),
                ("CJC", 2e-12),
                ("RE", 2.0),
                ("RCX", 3.0),
                ("TF", 1e-9),
            ]);
            for parameter in ["M", "AREA"] {
                for scale in [1e-200, 1e-30, 1e-18, 0.25, 4.0] {
                    for temperature in [280.15, 300.15, 340.15] {
                        let mut unit = base.clone();
                        unit.set_junction_gmin(0.0);
                        unit.set_temperature(temperature);
                        let scaled = unit
                            .clone()
                            .with_instance_params(&[(parameter.into(), scale)]);
                        for (actual, expected) in [
                            (scaled.is, unit.is),
                            (scaled.ikf, unit.ikf),
                            (scaled.ikr, unit.ikr),
                            (scaled.cje, unit.cje),
                            (scaled.cjc, unit.cjc),
                        ] {
                            assert!(
                                (actual / scale - expected).abs() <= expected.abs() * 2e-14,
                                "LEVEL={level} {parameter}={scale:e} T={temperature}: {actual:e} vs {expected:e}"
                            );
                        }
                        for (actual, expected) in [(scaled.re, unit.re), (scaled.rcx, unit.rcx)] {
                            assert!((actual * scale - expected).abs() <= expected.abs() * 2e-14);
                        }
                        // GP AREA also sets the default BC geometry. Compare
                        // transport at that geometry with normalized M; the
                        // simple parameter scalings above remain independent.
                        let reference = if level == 1.0 && parameter == "AREA" {
                            unit.clone().with_instance_params(&[
                                ("AREA".into(), scale),
                                ("M".into(), 1.0 / scale),
                            ])
                        } else {
                            unit.clone()
                        };
                        for (vbe, vbc) in [(0.1, -0.3), (0.7, -1.0), (-0.2, 0.7)] {
                            let expected = reference.transport_charge_state(vbe, vbc);
                            let actual = scaled.transport_charge_state(vbe, vbc);
                            for (actual, expected) in [
                                (actual.itzf, expected.itzf),
                                (actual.itzr, expected.itzr),
                                (actual.ditzf_dvbe_eff, expected.ditzf_dvbe_eff),
                                (actual.ditzr_dvbc_eff, expected.ditzr_dvbc_eff),
                            ] {
                                let expected = expected * scale;
                                // Reverse AREA^2 currents can become subnormal.
                                let tolerance = (expected.abs() * 2e-13).max(Value::from_bits(4));
                                assert!(
                                    (actual - expected).abs() <= tolerance,
                                    "LEVEL={level} {parameter}={scale:e} T={temperature} bias=({vbe},{vbc}): {actual:e} vs {expected:e}"
                                );
                            }
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn bjt_subnormal_knees_keep_finite_transport_and_derivatives() {
        for level in [1.0, 4.0, 11.0, 12.0] {
            let make = |isat| {
                let mut model =
                    model_with(&[("LEVEL", level), ("IS", isat), ("IKF", isat), ("IKR", isat)]);
                model.set_junction_gmin(0.0);
                model
            };
            let base = make(1e-10);
            let small = make(1e-310);
            for (vbe, vbc) in [(0.1, -0.1), (0.7, -0.2), (-0.1, 0.7)] {
                let reference = base.transport_charge_state(vbe, vbc);
                let actual = small.transport_charge_state(vbe, vbc);
                for (actual, expected) in [
                    (actual.itzf, reference.itzf),
                    (actual.itzr, reference.itzr),
                    (actual.ditzf_dvbe_eff, reference.ditzf_dvbe_eff),
                    (actual.ditzr_dvbc_eff, reference.ditzr_dvbc_eff),
                ] {
                    let expected = expected * 1e-300;
                    // Final subnormal currents have a fixed spacing; allow
                    // rounding in the current and derivative arithmetic.
                    let tolerance = (expected.abs() * 2e-11).max(Value::from_bits(4));
                    assert!(
                        (actual - expected).abs() <= tolerance,
                        "LEVEL={level} bias=({vbe},{vbc}): {actual:e} vs {expected:e}"
                    );
                }
            }
        }
    }

    #[test]
    fn temperature_refresh_invalidates_bjt_currents_and_charges_at_unchanged_bias() {
        for level in [1.0, 4.0, 11.0, 12.0] {
            for promoted in [false, true] {
                if promoted && level == 1.0 {
                    continue;
                }
                let make = || {
                    let mut params = vec![
                        ("LEVEL", level),
                        ("IS", 1e-16),
                        ("TF", 1e-9),
                        ("CJE", 1e-12),
                        ("RCX", 0.0),
                        ("RCI", 0.0),
                        ("RBX", 0.0),
                        ("RBI", 0.0),
                        ("RE", 0.0),
                        ("RBP", 0.0),
                        ("RS", 0.0),
                    ];
                    if level == 1.0 {
                        params.extend([
                            ("TNF1", 1e-3),
                            ("TNR2", 2e-6),
                            ("TNE1", -2e-3),
                            ("TNC2", 3e-6),
                            ("TNS1", 1e-3),
                            ("ISE", 1e-16),
                            ("ISC", 2e-16),
                            ("ISS", 3e-16),
                        ]);
                    }
                    let mut model =
                        model_with(&params).with_instance_params(&[("SW_ET".into(), 0.0)]);
                    model.set_voltage_limiting_enabled(false);
                    if promoted {
                        let mut next = 4;
                        model.assign_vbic_internal_nodes(|_| {
                            let node = next;
                            next += 1;
                            node
                        });
                    }
                    model
                };
                let mut model = make();
                let mut bias = vec![0.0; 16];
                bias[0] = 1.8;
                bias[1] = 0.7;
                if promoted {
                    for node in [model.node_cx, model.node_ci, model.node_bp] {
                        bias[node - 1] = 1.8;
                    }
                    for node in [model.node_bx, model.node_bi] {
                        bias[node - 1] = 0.7;
                    }
                }
                let charges = |bjt: &Bjt| {
                    if promoted {
                        // Exercise the cache used by the promoted MNA path,
                        // at its prescribed internal voltages, without a
                        // separate reduced internal operating-point solve.
                        bjt.mna_charge_state().0
                    } else {
                        bjt.charge_snapshot(1.8, 0.7, 0.0, 0.0).branches
                    }
                };
                if promoted {
                    model.update_mna_static_probe(&bias);
                } else {
                    model.update(&bias);
                }
                let cold = model.operating_point_currents();
                let cold_intrinsic = model.intrinsic_linearization.ic;
                let cold_charge = charges(&model);
                model.set_temperature(320.15);
                model.set_temperature(340.15);
                model.set_temperature(340.15);
                let mut fresh = make();
                fresh.set_temperature(340.15);
                if promoted {
                    fresh.update_mna_static_probe(&bias);
                } else {
                    fresh.update(&bias);
                }
                // Read-only charge requests must also reject the cold cache,
                // without requiring an intervening nonlinear update.
                let charge = charges(&model);
                let expected_charge = charges(&fresh);
                for (actual, expected) in charge.iter().zip(expected_charge.iter()) {
                    assert_eq!(
                        actual.charge, expected.charge,
                        "LEVEL={level} promoted={promoted}"
                    );
                }
                if level != 1.0 {
                    assert_ne!(charge[0].charge, cold_charge[0].charge);
                }
                model.update(&bias);
                let actual = model.operating_point_currents();
                let expected = fresh.operating_point_currents();
                if level >= 11.0 && !promoted {
                    // The resistance floors add privately solved unknowns.
                    // Warm and fresh Newton seeds can leave femtoampere KCL
                    // residuals; cache invalidation must agree within 10 fA.
                    for (actual, expected) in [actual.0, actual.1, actual.2]
                        .into_iter()
                        .zip([expected.0, expected.1, expected.2])
                    {
                        assert!(
                            (actual - expected).abs() < 1e-14,
                            "LEVEL={level}: {actual:e} != {expected:e}"
                        );
                    }
                } else {
                    assert_eq!(actual, expected, "LEVEL={level} promoted={promoted}");
                }
                if promoted {
                    // Prescribed equal prime/lead voltages carry zero series
                    // current; temperature changes the intrinsic junctions.
                    assert_eq!(
                        model.intrinsic_linearization.ic,
                        fresh.intrinsic_linearization.ic
                    );
                    assert_ne!(model.intrinsic_linearization.ic, cold_intrinsic);
                } else {
                    assert_ne!(model.operating_point_currents(), cold);
                }
            }
        }
    }

    #[test]
    fn vbic_zero_and_negative_activation_energies_preserve_temperature_laws() {
        let energies = ["EA", "EAIE", "EAIC", "EAIS", "EANE", "EANC", "EANS", "EAP"];
        for level in [4.0, 9.0, 11.0, 12.0] {
            for (selected, &selected_name) in energies.iter().enumerate() {
                for energy in [0.0, -0.1] {
                    let mut params = vec![
                        ("LEVEL", level),
                        ("EG", 0.8),
                        (selected_name, energy),
                        ("IS", 1e-16),
                        ("IBEI", 2e-17),
                        ("IBCI", 3e-17),
                        ("IBCIP", 4e-17),
                        ("IBEN", 5e-17),
                        ("IBCN", 6e-17),
                        ("IBCNP", 7e-17),
                        ("ISP", 8e-17),
                        ("IBEIP", 9e-17),
                        ("IBENP", 1e-16),
                        ("TNOM", 27.0),
                    ];
                    // Non-unit emission coefficients distinguish each law's
                    // activation/exponent divisor, including the parasitic BJT.
                    params.extend([
                        ("NF", 1.1),
                        ("NEI", 1.2),
                        ("NCI", 1.3),
                        ("NCIP", 1.4),
                        ("NEN", 1.5),
                        ("NCN", 1.6),
                        ("NCNP", 1.7),
                        ("NFP", 1.8),
                    ]);
                    let mut model = model_with(&params);
                    let stored = [
                        model.ea, model.eaie, model.eaic, model.eais, model.eane, model.eanc,
                        model.eans, model.eap,
                    ];
                    for (index, value) in stored.into_iter().enumerate() {
                        assert_eq!(value, if selected == index { energy } else { 0.8 });
                    }
                    for temperature in [280.15, 330.15] {
                        model.set_temperature(temperature);
                        let ratio = temperature / 300.15;
                        let laws = [
                            (model.is, 1e-16, model.nf_nominal, 0),
                            (model.ibei, 2e-17, model.nei, 1),
                            (model.ibci, 3e-17, model.nci, 2),
                            (model.ibcip, 4e-17, model.ncip, 3),
                            (model.iben, 5e-17, model.nen, 4),
                            (model.ibcn, 6e-17, model.ncn, 5),
                            (model.ibcnp, 7e-17, model.ncnp, 6),
                            (model.isp, 8e-17, model.nfp, 7),
                            (model.ibeip, 9e-17, model.nci, 2),
                            (model.ibenp, 1e-16, model.ncn, 5),
                        ];
                        for (actual, nominal, emission, index) in laws {
                            // Independent source equation: I0 * (T/Tnom)^(3/n)
                            // * exp(E/n * (1/Tnom - 1/T)/k).
                            let expected = nominal
                                * ratio.powf(3.0 / emission)
                                * (stored[index] / emission * (1.0 / 300.15 - 1.0 / temperature)
                                    / (model.vt / temperature))
                                    .exp();
                            assert!(
                                (actual - expected).abs() < expected.abs() * 1e-13,
                                "LEVEL={level} {}={energy}, T={temperature}, law={index}: {actual:e} != {expected:e}",
                                selected_name
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn vbic_tnf_scaling_is_idempotent_and_uses_nominal_emission_coefficients() {
        let signature = |model: &Bjt| [model.is, model.isrr, model.nf, model.nr];
        for level in [4.0, 11.0, 12.0] {
            for tnf in [-0.001, 0.001] {
                let mut bjt = model_with(&[
                    ("LEVEL", level),
                    ("IS", 1e-16),
                    ("ISRR", 0.7),
                    ("NF", 1.1),
                    ("NR", 1.2),
                    ("TNF", tnf),
                    ("XIS", 3.0),
                    ("XISR", 1.8),
                    ("EA", 1.12),
                    ("DEAR", 0.1),
                    ("RTH", 1000.0),
                    ("SELFT", 1.0),
                ])
                .with_instance_params(&[("M".into(), 3.0), ("DTEMP".into(), 20.0)]);
                for temperature in [320.15, 340.15, 280.15, 300.15] {
                    bjt.set_temperature(temperature);
                    let ratio = bjt.temperature / 300.15;
                    let dt = bjt.temperature - 300.15;
                    let expected = [
                        3e-16
                            * ratio.powf(3.0 / 1.1)
                            * (-1.12 * (1.0 - ratio) / (bjt.vt * 1.1)).exp(),
                        0.7 * ratio.powf(1.8 / 1.2) * (-0.1 * (1.0 - ratio) / (bjt.vt * 1.2)).exp(),
                        1.1 * (1.0 + dt * tnf),
                        1.2 * (1.0 + dt * tnf),
                    ];
                    let first = signature(&bjt);
                    for (actual, expected) in first.into_iter().zip(expected) {
                        assert!(
                            (actual - expected).abs() < 1e-13 * expected.abs(),
                            "LEVEL={level} TNF={tnf} T={temperature}: {actual:e} != {expected:e}"
                        );
                    }
                    for _ in 0..3 {
                        bjt.set_temperature(temperature);
                        assert_eq!(signature(&bjt), first);
                    }
                    // Cached thermal variants and direct temperature mapping
                    // must agree independently of previous evaluation order.
                    let mut direct = bjt.clone_without_thermal_variant_cache();
                    direct.refresh_operating_scaling_for(bjt.requested_temperature() + 15.0);
                    for _ in 0..2 {
                        let actual = bjt.with_temperature_variant(15.0, signature);
                        assert_eq!(actual, signature(&direct));
                        assert_eq!(signature(&bjt), first);
                    }
                }
            }
        }
    }

    #[test]
    fn vbic13_thermal_derivatives_keep_inactive_early_voltages_off() {
        for level in [11.0, 12.0] {
            let bjt = model_with(&[
                ("LEVEL", level),
                ("VEF", 5.0),
                ("VER", 3.0),
                ("TCVEF", -0.05),
                ("TCVER", -0.05),
                ("RTH", 1000.0),
            ]);
            for _ in 0..2 {
                let derivative = bjt.with_temperature_derivative_variant(19.999, 20.0, |model| {
                    model.vbic_early_voltages()
                });
                assert_eq!(derivative, (0.0, 0.0));
                // Physical evaluation must not reuse a derivative-only variant.
                let physical =
                    bjt.with_temperature_variant(19.999, |model| model.vbic_early_voltages());
                assert!(physical.0 > 0.0 && physical.1 > 0.0);
            }
            let anchor = 19.99999;
            let step = bjt.thermal_derivative_step(anchor);
            assert!(step > 1e-4);
            let expected =
                bjt.with_temperature_variant(anchor, |model| model.vbic_early_voltages());
            for rise in [anchor - step, anchor + step] {
                for _ in 0..2 {
                    let frozen = bjt.with_temperature_derivative_variant(rise, anchor, |model| {
                        model.vbic_early_voltages()
                    });
                    assert_eq!(frozen, expected);
                    // Identical probe temperatures with different anchors have
                    // different frozen parameters and must not share a cache entry.
                    let off = bjt.with_temperature_derivative_variant(rise, 20.0, |model| {
                        model.vbic_early_voltages()
                    });
                    assert_eq!(off, (0.0, 0.0));
                }
            }
            let physical =
                bjt.with_temperature_variant(anchor + step, |model| model.vbic_early_voltages());
            assert!(physical.0 < 0.0 && physical.1 < 0.0);
            assert_eq!((bjt.vaf, bjt.var), (5.0, 3.0));
        }
    }

    #[test]
    fn vbic13_early_voltage_temperature_mapping_preserves_nominal_values() {
        for level in [11.0, 12.0] {
            let mut bjt = model_with(&[
                ("LEVEL", level),
                ("VEF", 5.0),
                ("VER", 3.0),
                ("TCVEF", 0.05),
                ("TCVER", -0.05),
                ("TMAXCLIP", 100.0),
                ("RTH", 1000.0),
            ]);
            for (temperature, expected) in [
                (320.15, (10.0, 0.0)),
                (340.15, (15.0, -3.0)),
                (300.15, (5.0, 3.0)),
            ] {
                bjt.set_temperature(temperature);
                let mapped = bjt.vbic_early_voltages();
                assert!((mapped.0 - expected.0).abs() < 1e-12);
                assert!((mapped.1 - expected.1).abs() < 1e-12);
                assert_eq!((bjt.vaf, bjt.var), (5.0, 3.0));
            }
            let delta_t = 100.0 - (-2.0_f64).exp() - 27.0;
            for _ in 0..2 {
                let mapped =
                    bjt.with_temperature_variant(74.0, |variant| variant.vbic_early_voltages());
                assert!((mapped.0 - 5.0 * (1.0 + delta_t * 0.05)).abs() < 1e-12);
                assert!((mapped.1 - 3.0 * (1.0 - delta_t * 0.05)).abs() < 1e-12);
            }
            assert_eq!(bjt.vbic_early_voltages(), (5.0, 3.0));
        }
    }

    #[test]
    fn vbic13_temperature_mapping_has_exponential_tails_and_unit_slope_joins() {
        for level in [11.0, 12.0] {
            let bjt = model_with(&[("LEVEL", level), ("TMINCLIP", -50.0), ("TMAXCLIP", 100.0)]);
            for (raw_c, expected_c, slope) in [
                (-1000.0, -50.0, 0.0),
                (-51.0, -50.0 + (-2.0_f64).exp(), (-2.0_f64).exp()),
                (-49.0, -49.0, 1.0),
                (27.0, 27.0, 1.0),
                (99.0, 99.0, 1.0),
                (101.0, 100.0 - (-2.0_f64).exp(), (-2.0_f64).exp()),
                (2000.0, 100.0, 0.0),
            ] {
                let raw = raw_c + 273.15;
                let (temperature, derivative) = bjt.mapped_temperature(raw);
                assert!((temperature - (expected_c + 273.15)).abs() < 1e-12);
                assert!((derivative - slope).abs() < 1e-12);
                let h = 1e-5;
                let finite_difference = (bjt.mapped_temperature(raw + h).0
                    - bjt.mapped_temperature(raw - h).0)
                    / (2.0 * h);
                assert!((derivative - finite_difference).abs() < 3e-6);
            }
            // Mapping occurs after the thermal node offsets the raw ambient:
            // -99.85 K + 400 K is nominal, even though ambient alone clips.
            let offset = bjt.with_instance_params(&[("TRISE".into(), -400.0)]);
            offset.with_temperature_variant(400.0, |model| {
                assert!((model.temperature - 300.15).abs() < 1e-12);
            });
            assert!(offset.minimum_thermal_rise().is_infinite());
            assert!(offset.thermal_rebalance_step_limit(-1000.0).is_finite());
        }
        let legacy = model_with(&[("LEVEL", 4.0)]);
        assert_eq!(legacy.mapped_temperature(-10.0), (1.0, 0.0));
        assert_eq!(legacy.mapped_temperature(900.0), (900.0, 1.0));
    }

    #[test]
    fn vbic13_thermal_sink_jacobian_includes_resistance_and_clip_derivatives() {
        for level in [11.0, 12.0] {
            for coefficient in [-0.05, 0.0, 0.005, 0.05] {
                let bjt = model_with(&[
                    ("LEVEL", level),
                    ("RTH", 1000.0),
                    ("TCRTH", coefficient),
                    ("TMINCLIP", -50.0),
                    ("TMAXCLIP", 100.0),
                ])
                .with_instance_params(&[("M".into(), 3.0)]);
                for rise in [-1000.0, -78.0, -76.0, 0.0, 20.0, 72.0, 74.0, 1000.0] {
                    let branch = bjt.thermal_sink_branch(rise);
                    let h = 1e-5;
                    let fd = (bjt.thermal_sink_branch(rise + h).current
                        - bjt.thermal_sink_branch(rise - h).current)
                        / (2.0 * h);
                    let actual = branch.d_internal[IDX_VRTH];
                    assert!(
                        (actual - fd).abs() < 2e-5 * actual.abs().max(1e-3),
                        "level={level}, coefficient={coefficient}, rise={rise}: {actual} vs {fd}"
                    );
                    assert!(branch.current.is_finite());
                }
            }
        }
    }

    #[test]
    fn vbic13_preserves_authored_thermal_capacitance_including_zero() {
        for level in [4.0, 9.0, 11.0, 12.0] {
            for capacitance in [None, Some(0.0), Some(1e-15), Some(2e-12)] {
                let mut params = vec![("LEVEL", level), ("RTH", 1000.0), ("SELFT", 1.0)];
                if let Some(value) = capacitance {
                    params.push(("CTH", value));
                }
                let bjt = model_with(&params).with_instance_params(&[("M".into(), 3.0)]);
                let authored = capacitance.unwrap_or(0.0);
                let expected = if level < 11.0 {
                    authored.max(1e-12)
                } else {
                    authored
                };
                assert_eq!(bjt.thermal_capacitance(), 3.0 * expected);
            }
        }
    }
    use std::collections::HashMap;

    fn model_with(params: &[(&str, Value)]) -> Bjt {
        let params = params
            .iter()
            .map(|(key, value)| ((*key).to_string(), *value))
            .collect::<HashMap<_, _>>();
        Bjt::new_npn("q1".to_string(), 1, 2, 3).with_params(&params)
    }

    fn vbic_model_with(params: &[(&str, Value)]) -> Bjt {
        let mut params = params
            .iter()
            .map(|(key, value)| ((*key).to_string(), *value))
            .collect::<HashMap<_, _>>();
        params.entry("LEVEL".to_string()).or_insert(11.0);
        let mut bjt = Bjt::new_npn("q1".to_string(), 1, 2, 3);
        bjt.xyce_compatibility = true;
        bjt.with_params(&params)
    }

    #[test]
    fn explicit_celsius_tnom_matches_the_default_nominal_temperature() {
        let default = model_with(&[]);
        let explicit = model_with(&[("TNOM", 27.0)]);
        let expected = crate::constants::celsius_to_kelvin(27.0);

        assert_eq!(default.tnom.to_bits(), expected.to_bits());
        assert_eq!(explicit.tnom.to_bits(), expected.to_bits());
        assert_eq!(explicit.tnom.to_bits(), default.tnom.to_bits());
    }

    #[test]
    fn bjt_tnom_is_always_interpreted_as_celsius() {
        let below_freezing = model_with(&[("TNOM", -40.0)]);
        let hot = model_with(&[("TNOM", 300.0)]);

        assert_eq!(
            below_freezing.tnom.to_bits(),
            crate::constants::celsius_to_kelvin(-40.0).to_bits()
        );
        assert_eq!(
            hot.tnom.to_bits(),
            crate::constants::celsius_to_kelvin(300.0).to_bits()
        );
    }

    #[test]
    fn xyce_vbic_model_gmin_overrides_the_continuation_value_and_scales_with_m() {
        let mut bjt =
            vbic_model_with(&[("GMIN", 0.0)]).with_instance_params(&[("M".to_string(), 100.0)]);
        bjt.set_junction_gmin(1.0e-12);
        assert_eq!(bjt.nonlinear_branch_gmin().to_bits(), 0.0f64.to_bits());

        let mut inherited = vbic_model_with(&[]).with_instance_params(&[("M".to_string(), 100.0)]);
        inherited.set_junction_gmin(1.0e-12);
        assert_eq!(
            inherited.nonlinear_branch_gmin().to_bits(),
            1.0e-10f64.to_bits()
        );
    }

    #[test]
    fn older_vbic_series_resistance_zero_and_omission_keep_distinct_topologies() {
        let mut collapsed = vbic_model_with(&[
            ("LEVEL", 4.0),
            ("RBX", 0.0),
            ("RBI", 0.0),
            ("RCX", 0.0),
            ("RCI", 0.0),
            ("RBP", 0.0),
        ]);
        collapsed.set_temperature(350.0);
        collapsed.set_vbic_external_thermal_node(0);
        collapsed
            .assign_vbic_internal_nodes(|name| panic!("zero resistance must not allocate {name}"));
        assert_eq!(collapsed.node_bi, collapsed.node_base);
        assert_eq!(collapsed.node_ci, collapsed.node_collector);
        let explicit_outer = vbic_model_with(&[("LEVEL", 4.0), ("RBX", 3.0), ("RCX", 4.0)]);
        assert_eq!(
            [
                explicit_outer.rbx,
                explicit_outer.rbi,
                explicit_outer.rcx,
                explicit_outer.rci
            ],
            [3.0, 0.1, 4.0, 0.1]
        );
        let explicit_inner = vbic_model_with(&[("LEVEL", 4.0), ("RBI", 3.0), ("RCI", 4.0)]);
        assert_eq!(
            [
                explicit_inner.rbx,
                explicit_inner.rbi,
                explicit_inner.rcx,
                explicit_inner.rci
            ],
            [0.0, 3.0, 0.0, 4.0]
        );
    }

    #[test]
    fn vbic13_resistance_floor_retains_topology_and_parallel_scaling() {
        for level in [11.0, 12.0] {
            for resistance in [0.0, 1e-4, 1e-3] {
                for multiplicity in [1.0, 3.0, 1e12] {
                    let mut bjt = vbic_model_with(&[
                        ("LEVEL", level),
                        ("RCX", resistance),
                        ("RCI", resistance),
                        ("RBX", resistance),
                        ("RBI", resistance),
                        ("RE", resistance),
                        ("RBP", resistance),
                        ("RS", resistance),
                    ])
                    .with_instance_params(&[("M".into(), multiplicity)]);
                    bjt.set_vbic_external_thermal_node(0);
                    let mut nodes = 0;
                    bjt.assign_vbic_internal_nodes(|_| {
                        nodes += 1;
                        nodes + 4
                    });
                    assert_eq!(nodes, if level == 11.0 { 6 } else { 7 });
                    assert_ne!(bjt.node_bi, bjt.node_base);
                    assert_ne!(bjt.node_ci, bjt.node_collector);
                    let current = bjt.ire_branch(0.01, 0.0).current;
                    assert!((current / multiplicity - 10.0).abs() < 1e-12);
                    for value in [bjt.rcx, bjt.rci, bjt.rbx, bjt.rbi, bjt.re, bjt.rbp, bjt.rs] {
                        assert!((value * multiplicity - 1e-3).abs() < 1e-18);
                    }
                }
            }
        }
    }

    #[test]
    fn vbic13_resistance_floor_thermal_derivative_selects_the_anchor_branch() {
        for exponent in [-2.0, 2.0] {
            for factor in [
                0.999999,
                1.0 - Value::EPSILON,
                1.0,
                1.0 + Value::EPSILON,
                1.000001,
                2.0,
            ] {
                let bjt = vbic_model_with(&[("RE", 1e-3 * factor), ("XRE", exponent)])
                    .with_instance_params(&[("M".into(), 3.0)]);
                let h = bjt.thermal_derivative_step(0.0);
                let physical = bjt.with_temperature_variant(h, |v| v.ire_branch(0.01, 0.0).current);
                let plus = bjt.with_temperature_derivative_variant(h, 0.0, |v| {
                    v.ire_branch(0.01, 0.0).current
                });
                let minus = bjt.with_temperature_derivative_variant(-h, 0.0, |v| {
                    v.ire_branch(0.01, 0.0).current
                });
                let derivative = (plus - minus) / (2.0 * h);
                let expected = if factor <= 1.0 {
                    0.0
                } else {
                    -bjt.ire_branch(0.01, 0.0).current * exponent / bjt.temperature
                };
                assert!(
                    (derivative - expected).abs() < 1e-5 * expected.abs() + 1e-10,
                    "factor={factor} exponent={exponent} h={h}: {derivative} vs {expected}"
                );
                assert_eq!(
                    physical,
                    bjt.with_temperature_variant(h, |v| v.ire_branch(0.01, 0.0).current)
                );
            }
        }
    }

    #[test]
    fn vbic_multiplicity_scales_every_linear_parasitic_branch() {
        let params = [
            ("RE", 2.0),
            ("RBX", 3.0),
            ("RBI", 4.0),
            ("RCX", 5.0),
            ("RCI", 6.0),
            ("RS", 7.0),
            ("RBP", 8.0),
        ];
        let base = vbic_model_with(&params);
        let scaled = vbic_model_with(&params).with_instance_params(&[("M".to_string(), 100.0)]);
        for (base_value, scaled_value) in [
            (base.re, scaled.re),
            (base.rbx, scaled.rbx),
            (base.rbi, scaled.rbi),
            (base.rcx, scaled.rcx),
            (base.rci, scaled.rci),
            (base.rs, scaled.rs),
            (base.rbp, scaled.rbp),
        ] {
            assert_eq!(scaled_value.to_bits(), (base_value / 100.0).to_bits());
        }
    }

    #[test]
    fn instance_temp_is_absolute_and_outranks_dtemp_independent_of_parameter_order() {
        let ambient = crate::constants::celsius_to_kelvin(15.0);
        let expected = crate::constants::celsius_to_kelvin(25.0);
        for params in [
            vec![("TEMP".to_string(), 25.0), ("DTEMP".to_string(), 20.0)],
            vec![("DTEMP".to_string(), 20.0), ("TEMP".to_string(), 25.0)],
        ] {
            let mut bjt = model_with(&[]);
            bjt.set_temperature(ambient);
            let bjt = bjt.with_instance_params(&params);
            assert_eq!(bjt.requested_temperature().to_bits(), expected.to_bits());
        }
    }

    #[test]
    fn instance_dtemp_offsets_ambient_when_temp_is_absent() {
        let ambient = crate::constants::celsius_to_kelvin(15.0);
        let mut bjt = model_with(&[]);
        bjt.set_temperature(ambient);
        let bjt = bjt.with_instance_params(&[("DTEMP".to_string(), 20.0)]);
        assert_eq!(
            bjt.requested_temperature().to_bits(),
            crate::constants::celsius_to_kelvin(35.0).to_bits()
        );
    }

    #[test]
    fn legacy_temperature_scaling_uses_energy_gap_not_vbic_activation_energy() {
        let mut bjt = model_with(&[("IS", 1.0e-16)]);
        assert_eq!(bjt.charge_model, BjtChargeModel::LegacyGummelPoon);
        bjt.xyce_compatibility = true;
        bjt.eg = 1.07;
        bjt.ea = 1.93;

        let temp = crate::constants::celsius_to_kelvin(35.0);
        let tnom = bjt.tnom;
        let ratio = temp / tnom;
        let vt = bjt.thermal_voltage_at(temp);
        let expected = bjt.is_nominal * ((ratio - 1.0) * bjt.eg / vt + bjt.xis * ratio.ln()).exp();
        let wrong_vbic_activation_energy =
            bjt.is_nominal * ((ratio - 1.0) * bjt.ea / vt + bjt.xis * ratio.ln()).exp();

        bjt.set_temperature(temp);

        assert_eq!(bjt.is.to_bits(), expected.to_bits());
        assert_ne!(bjt.is.to_bits(), wrong_vbic_activation_energy.to_bits());
    }

    #[test]
    fn substrate_topology_defaults_match_ngspice_bjt_setup() {
        let npn_default = Bjt::new_npn("q1".to_string(), 1, 2, 3).with_params(&HashMap::new());
        let pnp_default = Bjt::new_pnp("q1".to_string(), 1, 2, 3).with_params(&HashMap::new());
        let pnp_lateral = Bjt::new_pnp("q1".to_string(), 1, 2, 3)
            .with_params(&HashMap::from([("SUBS".to_string(), -1.0)]));
        let pnp_vertical = Bjt::new_pnp("q1".to_string(), 1, 2, 3)
            .with_params(&HashMap::from([("SUBS".to_string(), 1.0)]));

        assert_eq!(
            npn_default.substrate_topology,
            BjtSubstrateTopology::Vertical
        );
        assert_eq!(
            pnp_default.substrate_topology,
            BjtSubstrateTopology::Lateral
        );
        assert_eq!(
            pnp_lateral.substrate_topology,
            BjtSubstrateTopology::Lateral
        );
        assert_eq!(
            pnp_vertical.substrate_topology,
            BjtSubstrateTopology::Vertical
        );
    }

    #[test]
    fn legacy_rb_without_rbm_stays_constant() {
        let bjt = model_with(&[("RB", 50.0)]);

        assert_eq!(bjt.rb, 50.0);
        assert_eq!(bjt.rbx, 50.0);
        assert_eq!(bjt.rbi, 0.0);
    }

    #[test]
    fn legacy_rbm_retains_one_physical_base_resistance() {
        let bjt = model_with(&[("RB", 50.0), ("RBM", 10.0)]);

        assert_eq!(bjt.rb, 50.0);
        assert_eq!(bjt.rbx, 0.0);
        assert_eq!(bjt.rbi, 50.0);
        assert_eq!(
            bjt.legacy_junction_params
                .as_ref()
                .unwrap()
                .base_resistance
                .unwrap()
                .nominal,
            [50.0, 10.0]
        );
        for fields in [vec![("RBM", 10.0)], vec![("RB", 0.0), ("RBM", 10.0)]] {
            let disabled = model_with(&fields);
            assert_eq!((disabled.rbx, disabled.rbi), (0.0, 0.0));
        }
    }

    #[test]
    fn legacy_base_temperature_refresh_preserves_whole_and_minimum_resistance() {
        for rbm in [20.0, 120.0, 180.0] {
            let mut bjt = model_with(&[
                ("RB", 120.0),
                ("RBM", rbm),
                ("TRB", -0.005),
                ("TRB2", 1e-5),
                ("TRM1", 0.002),
                ("TRM2", -5e-6),
            ])
            .with_instance_params(&[("AREA".into(), 2.0), ("M".into(), 3.0)]);
            for temperature in [233.15, 343.15, 300.15, 343.15] {
                bjt.set_temperature(temperature);
                bjt.validate_legacy_temperature_parameters().unwrap();
                let dt = temperature - 300.15;
                let rb = 20.0 * (1.0 - 0.005 * dt + 1e-5 * dt * dt);
                let rbm = rbm / 6.0 * (1.0 + 0.002 * dt - 5e-6 * dt * dt);
                let linearized = BjtLinearization {
                    qb: 0.7,
                    ..Default::default()
                };
                let branch = bjt.irbi_branch(linearized, 0.1, 0.0);
                let expected = (rbm + (rb - rbm) / 0.7).recip();
                assert!((branch.current / (0.1 * expected) - 1.0).abs() < 2e-14);
                assert_eq!(bjt.rbx, 0.0);
            }
        }
        // RB must survive cancellation against a much larger minimum at QB=1.
        let bjt = model_with(&[("RB", 1e-200), ("RBM", 1.0)]);
        let branch = bjt.irbi_branch(
            BjtLinearization {
                qb: 1.0,
                ..Default::default()
            },
            1e-200,
            0.0,
        );
        assert!((branch.current - 1.0).abs() < 1e-14);
    }

    #[test]
    fn legacy_irb_jrb_iob_aliases_select_base_current_threshold() {
        for alias in ["IRB", "JRB", "IOB"] {
            let bjt = model_with(&[(alias, 1.144e-3)]);
            assert_eq!(bjt.irb, 1.144e-3, "{alias} must map to IRB");
        }
    }

    #[test]
    fn legacy_spice_leakage_aliases_map_to_nonideal_junctions() {
        let bjt = model_with(&[
            ("ISE", 2.3e-14),
            ("NE", 1.78),
            ("ISC", 4.5e-15),
            ("NC", 2.2),
        ]);

        assert_eq!(bjt.iben_nominal, 2.3e-14);
        assert_eq!(bjt.nen, 1.78);
        assert_eq!(bjt.ibcn_nominal, 4.5e-15);
        assert_eq!(bjt.ncn, 2.2);
    }

    #[test]
    fn legacy_spice_leakage_defaults_match_bjt_reference_models() {
        let bjt = model_with(&[]);

        assert_eq!(bjt.iben_nominal, 0.0);
        assert_eq!(bjt.nen, 1.5);
        assert_eq!(bjt.ibcn_nominal, 0.0);
        assert_eq!(bjt.ncn, 2.0);
    }

    fn legacy_alias_snapshot(bjt: &Bjt) -> [u64; 21] {
        [
            bjt.bf.to_bits(),
            bjt.br.to_bits(),
            bjt.vaf.to_bits(),
            bjt.var.to_bits(),
            bjt.beta_exp.to_bits(),
            bjt.xti.to_bits(),
            bjt.ikf_nominal.to_bits(),
            bjt.ikr_nominal.to_bits(),
            bjt.iben_nominal.to_bits(),
            bjt.ibcn_nominal.to_bits(),
            bjt.nen.to_bits(),
            bjt.irb_nominal.to_bits(),
            bjt.vje.to_bits(),
            bjt.mje.to_bits(),
            bjt.vjc.to_bits(),
            bjt.mjc.to_bits(),
            bjt.xcjc.to_bits(),
            bjt.cjcp_nominal.to_bits(),
            bjt.ps.to_bits(),
            bjt.ms.to_bits(),
            bjt.itf.to_bits(),
        ]
    }

    #[test]
    fn legacy_xyce_bjt_alias_families_resolve_to_one_physical_model() {
        let canonical = model_with(&[
            ("BF", 195.3412),
            ("VAF", 53.081),
            ("IKF", 0.976),
            ("ISE", 1.60241e-14),
            ("NE", 1.4791931),
            ("BR", 1.1107942),
            ("VAR", 11.3571702),
            ("IKR", 2.4993953),
            ("ISC", 1.88505e-12),
            ("IRB", 1.50459e-4),
            ("VJE", 0.682256),
            ("MJE", 0.3358856),
            ("VJC", 0.5417393),
            ("MJC", 0.4547893),
            ("XCJC", 1.0),
            ("CJS", 0.0),
            ("VJS", 0.75),
            ("MJS", 0.0),
            ("ITF", 0.32),
            ("XTB", 1.6486),
            ("XTI", 5.8315),
        ]);
        let short_aliases = model_with(&[
            ("BFM", 195.3412),
            ("VA", 53.081),
            ("IK", 0.976),
            ("JLE", 1.60241e-14),
            ("NLE", 1.4791931),
            ("BRM", 1.1107942),
            ("VB", 11.3571702),
            ("JBR", 2.4993953),
            ("JLC", 1.88505e-12),
            ("JRB", 1.50459e-4),
            ("PE", 0.682256),
            ("ME", 0.3358856),
            ("PC", 0.5417393),
            ("MC", 0.4547893),
            ("CDIS", 1.0),
            ("CCS", 0.0),
            ("PS", 0.75),
            ("MS", 0.0),
            ("JTF", 0.32),
            ("TB", 1.6486),
            ("PT", 5.8315),
        ]);
        let long_aliases = model_with(&[
            ("BFM", 195.3412),
            ("VBF", 53.081),
            ("JBF", 0.976),
            ("JLE", 1.60241e-14),
            ("NLE", 1.4791931),
            ("BRM", 1.1107942),
            ("VRB", 11.3571702),
            ("JBR", 2.4993953),
            ("JLC", 1.88505e-12),
            ("IOB", 1.50459e-4),
            ("PE", 0.682256),
            ("ME", 0.3358856),
            ("PC", 0.5417393),
            ("MC", 0.4547893),
            ("CDIS", 1.0),
            ("CSUB", 0.0),
            ("PSUB", 0.75),
            ("ESUB", 0.0),
            ("JTF", 0.32),
            ("TCB", 1.6486),
            ("PT", 5.8315),
        ]);

        let expected = legacy_alias_snapshot(&canonical);
        assert_eq!(legacy_alias_snapshot(&short_aliases), expected);
        assert_eq!(legacy_alias_snapshot(&long_aliases), expected);
    }
}
