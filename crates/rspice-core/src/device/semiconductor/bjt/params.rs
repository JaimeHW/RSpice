//! BJT model parameters, defaults, and temperature scaling.

use super::*;

impl Bjt {
    #[inline]
    pub(super) fn apply_legacy_spice_model_defaults(&mut self) {
        self.substrate_topology = BjtSubstrateTopology::default_for_type(self.bjt_type);
        self.is_nominal = 1e-16;
        self.is = self.is_nominal;
        self.bf = 100.0;
        self.br = 1.0;
        self.nf_nominal = 1.0;
        self.nr_nominal = 1.0;
        self.nf = 1.0;
        self.nr = 1.0;
        self.vaf = f64::INFINITY;
        self.var = f64::INFINITY;
        self.rb = 0.0;
        self.rbx = 0.0;
        self.rbi = 0.0;
        self.rbx_nominal = 0.0;
        self.rbi_nominal = 0.0;
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
    }

    #[inline]
    pub(super) fn apply_vbic_model_defaults(&mut self) {
        self.substrate_topology = BjtSubstrateTopology::default_for_type(self.bjt_type);
        self.is_nominal = 1e-16;
        self.is = self.is_nominal;
        self.nf_nominal = 1.0;
        self.nr_nominal = 1.0;
        self.nf = 1.0;
        self.nr = 1.0;
        self.vaf = 0.0;
        self.var = 0.0;
        self.rb = 0.0;
        self.rc = 0.0;
        self.rbx = 0.0;
        self.rbi = 0.1;
        self.rcx = 0.0;
        self.rci = 0.1;
        self.re = 0.0;
        self.rs = 0.0;
        self.rbp = 0.1;
        self.rbx_nominal = self.rbx;
        self.rbi_nominal = self.rbi;
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
        self.nei = 1.0;
        self.nen = 2.0;
        self.nci = 1.0;
        self.ncn = 2.0;
        self.avc1 = 0.0;
        self.avc2_nominal = 0.0;
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
    pub(crate) fn has_vbic_self_heating(&self) -> bool {
        self.self_heating_enabled()
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

    pub(super) fn refresh_operating_scaling(&mut self) {
        let temp = self.requested_temperature();
        self.refresh_operating_scaling_for(temp);
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

    pub(crate) fn vbic_collector_substrate_charge_homotopy_variant(&self, lambda: Value) -> Self {
        let scale = lambda.clamp(0.0, 1.0);
        let mut variant = self.clone_without_thermal_variant_cache();
        variant.reduced_linearization_cache_valid.set(false);
        variant.previous_reduced_linearization_valid = false;
        variant.charge_snapshot_cache_valid.set(false);

        if variant.charge_model != BjtChargeModel::Vbic {
            return variant;
        }

        variant.qco_nominal *= scale;
        variant.cjcp_nominal *= scale;
        variant.ccso_nominal *= scale;
        variant.qco *= scale;
        variant.cjcp *= scale;
        variant.ccso *= scale;
        variant
    }

    pub(super) fn with_temperature_variant<R>(
        &self,
        thermal_rise: Value,
        f: impl FnOnce(&Self) -> R,
    ) -> R {
        if !self.self_heating_enabled() {
            return f(self);
        }

        let key = thermal_rise.to_bits();
        {
            let cache = self.thermal_variant_cache.borrow();
            if let Some((_, variant)) = cache.iter().find(|(cached_key, _)| *cached_key == key) {
                return f(variant.as_ref());
            }
        }

        let mut variant = self.clone_without_thermal_variant_cache();
        variant
            .refresh_operating_scaling_for((self.requested_temperature() + thermal_rise).max(1.0));
        let result = f(&variant);

        let mut cache = self.thermal_variant_cache.borrow_mut();
        if cache.len() >= Self::THERMAL_VARIANT_CACHE_CAPACITY {
            cache.remove(0);
        }
        cache.push((key, Box::new(variant)));
        result
    }

    pub(super) fn refresh_operating_scaling_for(&mut self, temp: Value) {
        self.clear_thermal_variant_cache();
        let tnom = self.tnom.max(1.0);
        let vt = Self::thermal_voltage_at(temp);
        let ratio = (temp / tnom).max(1e-12);
        let delta_t = temp - tnom;
        let is_temp =
            Self::vbic_temp_scaled_current(self.is_nominal, ratio, vt, self.xis, self.ea, self.nf);
        let scale = self.instance_scale();
        let isrr_temp = Self::vbic_temp_scaled_current(
            self.isrr_nominal,
            ratio,
            vt,
            self.xisr,
            self.dear,
            self.nr,
        );
        let gamm_ratio_term = ratio.powf(self.xis);
        let gamm_energy_term = (-self.ea * (1.0 - ratio) / vt.max(1e-18)).clamp(-80.0, 80.0);
        let gamm_temp = self.gamm_nominal * gamm_ratio_term * gamm_energy_term.exp();
        let ibei_temp = Self::vbic_temp_scaled_current(
            self.ibei_nominal,
            ratio,
            vt,
            self.xii,
            self.eaie,
            self.nei,
        );
        let iben_temp = Self::vbic_temp_scaled_current(
            self.iben_nominal,
            ratio,
            vt,
            self.xin,
            self.eane,
            self.nen,
        );
        let ibci_temp = Self::vbic_temp_scaled_current(
            self.ibci_nominal,
            ratio,
            vt,
            self.xii,
            self.eaic,
            self.nci,
        );
        let ibcn_temp = Self::vbic_temp_scaled_current(
            self.ibcn_nominal,
            ratio,
            vt,
            self.xin,
            self.eanc,
            self.ncn,
        );
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
        let vje_temp = Self::vbic_temp_scaled_potential(self.vje_nominal, ratio, vt, self.eaie);
        let vjc_temp = Self::vbic_temp_scaled_potential(self.vjc_nominal, ratio, vt, self.eaic);
        let ps_temp = Self::vbic_temp_scaled_potential(self.ps_nominal, ratio, vt, self.eais);
        let nf_temp = self.nf_nominal * (1.0 + delta_t * self.tnf);
        let nr_temp = self.nr_nominal * (1.0 + delta_t * self.tnf);
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
        self.is = (is_temp * scale).max(1e-30);
        self.nf = nf_temp.max(1e-12);
        self.nr = nr_temp.max(1e-12);
        self.re = re_temp.max(0.0);
        self.rbx = rbx_temp.max(0.0);
        self.rbi = rbi_temp.max(0.0);
        self.rcx = rcx_temp.max(0.0);
        self.rci = rci_temp.max(0.0);
        self.vje = vje_temp;
        self.vjc = vjc_temp;
        self.ps = ps_temp;
        self.cje =
            (self.cje_nominal * (self.vje_nominal / vje_temp.max(1e-18)).powf(self.mje) * scale)
                .max(0.0);
        self.cjc =
            (self.cjc_nominal * (self.vjc_nominal / vjc_temp.max(1e-18)).powf(self.mjc) * scale)
                .max(0.0);
        self.cjcp =
            (self.cjcp_nominal * (self.ps_nominal / ps_temp.max(1e-18)).powf(self.ms) * scale)
                .max(0.0);
        self.cjep =
            (self.cjep_nominal * (self.vjc_nominal / vjc_temp.max(1e-18)).powf(self.mjc) * scale)
                .max(0.0);
        self.cbeo = (self.cbeo_nominal * scale).max(0.0);
        self.cbco = (self.cbco_nominal * scale).max(0.0);
        self.qco = (self.qco_nominal * scale).max(0.0);
        self.ccso = (self.ccso_nominal * scale).max(0.0);
        self.vo = vo_temp.max(0.0);
        self.gamm = gamm_temp.max(0.0);
        self.ikf = if ikf_temp > 0.0 {
            (ikf_temp * scale).max(1e-18)
        } else {
            0.0
        };
        self.ikr = if self.ikr_nominal > 0.0 {
            (self.ikr_nominal * scale).max(1e-18)
        } else {
            0.0
        };
        self.isrr = isrr_temp.max(0.0);
        self.ibei = (ibei_temp * scale).max(0.0);
        self.iben = (iben_temp * scale).max(0.0);
        self.ibci = (ibci_temp * scale).max(0.0);
        self.ibcn = (ibcn_temp * scale).max(0.0);
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
        self.rs = rs_temp.max(0.0);
        self.rbp = rbp_temp.max(0.0);
        self.avc2 = if avc2_temp.is_finite() {
            avc2_temp.max(0.0)
        } else {
            self.avc2_nominal
        };
        self.rth = self.rth_nominal.max(0.0);
        self.cth = self.thermal_capacitance();
    }
    /// Set active device temperature (Kelvin).
    pub fn set_temperature(&mut self, temp_k: Value) {
        if temp_k.is_finite() && temp_k > 0.0 {
            self.ambient_temperature = temp_k;
            self.refresh_operating_scaling();
        }
    }

    /// Set optional substrate node (0 for ground/unconnected).
    pub fn set_substrate_node(&mut self, substrate: NodeId) {
        self.node_substrate = substrate;
    }

    /// Set model parameters from a DeviceModel
    pub fn with_params(mut self, params: &std::collections::HashMap<String, Value>) -> Self {
        let mut has_vaf = false;
        let mut has_var = false;
        let mut has_rb = false;
        let mut has_rc = false;
        let mut has_ibei = false;
        let mut has_ibci = false;
        let mut has_rth = false;
        let mut legacy_rb: Option<Value> = None;
        let mut legacy_rbm: Option<Value> = None;
        self.charge_model = if Self::uses_vbic_charge_model(params) {
            BjtChargeModel::Vbic
        } else {
            BjtChargeModel::LegacyGummelPoon
        };
        match self.charge_model {
            BjtChargeModel::LegacyGummelPoon => self.apply_legacy_spice_model_defaults(),
            BjtChargeModel::Vbic => self.apply_vbic_model_defaults(),
        }

        // DC parameters
        if let Some(&v) = params.get("IS") {
            self.is_nominal = v.max(0.0);
        }
        if let Some(&v) = params.get("BF") {
            self.bf = v;
        }
        if let Some(&v) = params.get("BR") {
            self.br = v;
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
        if let Some(&v) = params.get("VAF") {
            self.vaf = v;
            has_vaf = true;
        }
        if !has_vaf && let Some(&v) = params.get("VA") {
            self.vaf = v;
            has_vaf = true;
        }
        if let Some(&v) = params.get("VAR") {
            self.var = v;
            has_var = true;
        }
        if !has_var && let Some(&v) = params.get("VB") {
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
        if let Some(&v) = params.get("XTI")
            && v.is_finite()
            && v > 0.0
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
            && v > 0.0
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
        if let Some(&v) = params.get("EA")
            && v.is_finite()
            && v > 0.0
        {
            self.ea = v;
        }
        if let Some(&v) = params.get("EAIE")
            && v.is_finite()
            && v > 0.0
        {
            self.eaie = v;
        }
        if let Some(&v) = params.get("EAIC")
            && v.is_finite()
            && v > 0.0
        {
            self.eaic = v;
        }
        if let Some(&v) = params.get("EANE")
            && v.is_finite()
            && v > 0.0
        {
            self.eane = v;
        }
        if let Some(&v) = params.get("EANC")
            && v.is_finite()
            && v > 0.0
        {
            self.eanc = v;
        }
        if let Some(&v) = params.get("EAIS")
            && v.is_finite()
            && v > 0.0
        {
            self.eais = v;
        }
        if let Some(&v) = params.get("EANS")
            && v.is_finite()
            && v > 0.0
        {
            self.eans = v;
        }
        if let Some(&v) = params.get("EAP")
            && v.is_finite()
            && v > 0.0
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
            && v > 0.0
        {
            self.tnom = if v > 200.0 { v } else { v + 273.15 };
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
        // VBIC flicker noise: KFN/AFN/BFN ride on the intrinsic B-E current
        // (vbicnoise.c FLBENOIZ). Defaults 0/1/1 per vbicsetup.c:230-238.
        if let Some(v) = params
            .get("KFN")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            self.kfn = v;
        }
        if let Some(v) = params
            .get("AFN")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            self.afn = v;
        }
        if let Some(v) = params
            .get("BFN")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
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
                let rbm = legacy_rbm.unwrap_or(rb).min(rb);
                self.rbx = rbm;
                self.rbi = (rb - rbm).max(0.0);
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
        } else if let Some(rbm) = legacy_rbm
            && self.charge_model == BjtChargeModel::LegacyGummelPoon
        {
            self.rbx = rbm;
            self.rbi = 0.0;
            self.rbx_nominal = self.rbx;
            self.rbi_nominal = self.rbi;
            self.rb = self.rbx;
            has_rb = true;
        }
        if !has_rb {
            let rbx = params
                .get("RBX")
                .copied()
                .filter(|v| v.is_finite() && *v > 0.0)
                .unwrap_or(0.0);
            let rbi = params
                .get("RBI")
                .copied()
                .filter(|v| v.is_finite() && *v > 0.0)
                .unwrap_or(0.0);
            if rbx > 0.0 || rbi > 0.0 {
                self.rbx = rbx;
                self.rbi = rbi;
                self.rbx_nominal = rbx;
                self.rbi_nominal = rbi;
                self.rb = (rbx + rbi).max(1e-12);
            }
        }
        if !has_rc {
            let rcx = params
                .get("RCX")
                .copied()
                .filter(|v| v.is_finite() && *v > 0.0)
                .unwrap_or(0.0);
            let rci = params
                .get("RCI")
                .copied()
                .filter(|v| v.is_finite() && *v > 0.0)
                .unwrap_or(0.0);
            if rcx > 0.0 || rci > 0.0 {
                self.rcx = rcx;
                self.rci = rci;
                self.rcx_nominal = rcx;
                self.rci_nominal = rci;
                self.rc = (rcx + rci).max(1e-12);
            }
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
        if let Some(v) = params
            .get("MJE")
            .copied()
            .or_else(|| params.get("ME").copied())
            .filter(|v| v.is_finite())
        {
            self.mje = v;
        }
        if let Some(&v) = params.get("PE")
            && v.is_finite()
            && v > 0.0
        {
            self.vje = v;
            self.vje_nominal = v;
        }
        if let Some(&v) = params.get("VJE")
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
        if let Some(&v) = params.get("XCJC")
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
        if let Some(v) = params
            .get("CJCP")
            .copied()
            .or_else(|| params.get("CJS").copied())
            .or_else(|| params.get("CSUB").copied())
            .or_else(|| params.get("CCS").copied())
        {
            self.cjcp_nominal = v.max(0.0);
        }
        if let Some(v) = params
            .get("MJC")
            .copied()
            .or_else(|| params.get("MC").copied())
            .filter(|v| v.is_finite())
        {
            self.mjc = v;
        }
        if let Some(&v) = params.get("PS").or_else(|| params.get("VJS"))
            && v.is_finite()
            && v > 0.0
        {
            self.ps = v;
            self.ps_nominal = v;
        }
        if let Some(&v) = params.get("MS").or_else(|| params.get("MJS"))
            && v.is_finite()
        {
            self.ms = v;
        }
        if let Some(&v) = params.get("AJS")
            && v.is_finite()
        {
            self.ajs = v;
        }
        if let Some(&v) = params.get("PC")
            && v.is_finite()
            && v > 0.0
        {
            self.vjc = v;
            self.vjc_nominal = v;
        }
        if let Some(&v) = params.get("VJC")
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
        if let Some(&v) = params.get("ITF")
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
        if let Some(&v) = params.get("IKF") {
            self.ikf_nominal = v.max(0.0);
        }
        if let Some(&v) = params.get("IKR") {
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
            && let Some(&v) = params.get("ISE")
        {
            self.iben_nominal = v.max(0.0);
        }
        if let Some(&v) = params.get("IBEN") {
            self.iben_nominal = v.max(0.0);
        }
        if let Some(&v) = params.get("IBCI") {
            self.ibci_nominal = v.max(0.0);
            has_ibci = true;
        }
        if self.charge_model == BjtChargeModel::LegacyGummelPoon
            && let Some(&v) = params.get("ISC")
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
            && let Some(&v) = params.get("NE")
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
        if !has_ibci && self.charge_model == BjtChargeModel::LegacyGummelPoon {
            self.ibci_nominal = self.is_nominal / self.br.max(1e-18);
        }
        if self.charge_model == BjtChargeModel::Vbic && has_rth {
            // ngspice VBIC setup semantics:
            // - If RTH is provided, clamp CTH to at least 1e-12.
            if self.cth_nominal < 1e-12 {
                self.cth_nominal = 1e-12;
            }
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
        self.rc = 0.0;
    }

    /// Clear the emitter series resistance after the builder externalizes
    /// it onto a real circuit resistor.
    pub fn clear_emitter_series_resistance(&mut self) {
        self.re = 0.0;
        self.re_nominal = 0.0;
    }

    /// Clear the constant part of the base resistance after the builder
    /// externalizes it onto a real circuit resistor. The bias-dependent
    /// part (`rbi`, nonzero only when the card gives `RBM < RB`) stays on
    /// the device; `rb` tracks the remaining internal total so downstream
    /// reporting stays consistent. The nominal is cleared too so a later
    /// temperature refresh cannot resurrect the internal copy.
    pub fn clear_base_constant_resistance(&mut self) {
        self.rbx = 0.0;
        self.rbx_nominal = 0.0;
        self.rb = self.rbi.max(0.0);
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

    /// Apply instance-level BJT scaling and thermal overrides.
    ///
    /// Supported keys:
    /// - `AREA`: area multiplier (default 1)
    /// - `M` / `MULT`: multiplicity (default 1)
    /// - `TEMP`: absolute device temperature in Celsius
    /// - `DTEMP`: temperature delta in Celsius
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

            if name.eq_ignore_ascii_case("TEMP") {
                self.instance_temp = Some(*value + 273.15);
                continue;
            }

            if name.eq_ignore_ascii_case("DTEMP") {
                self.instance_dtemp = *value;
            }
        }

        self.refresh_operating_scaling();
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    fn model_with(params: &[(&str, Value)]) -> Bjt {
        let params = params
            .iter()
            .map(|(key, value)| ((*key).to_string(), *value))
            .collect::<HashMap<_, _>>();
        Bjt::new_npn("q1".to_string(), 1, 2, 3).with_params(&params)
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
    fn legacy_rbm_partitions_base_resistance() {
        let bjt = model_with(&[("RB", 50.0), ("RBM", 10.0)]);

        assert_eq!(bjt.rb, 50.0);
        assert_eq!(bjt.rbx, 10.0);
        assert_eq!(bjt.rbi, 40.0);
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
}
