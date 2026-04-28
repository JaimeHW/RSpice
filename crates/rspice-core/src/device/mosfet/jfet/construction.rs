//! JFET constructors, model-card mapping, and instance parameter helpers.

use super::*;

impl Jfet {
    /// Create a new N-JFET
    pub fn njf(name: &str, drain: NodeId, gate: NodeId, source: NodeId) -> Self {
        Self {
            name: name.to_string(),
            jfet_type: JfetType::NJF,
            drain,
            gate,
            source,
            params: JfetParams::default(),
            m: 1.0,
            area: 1.0,
            width: 1e-6,
            length: 1e-6,
            instance_temp: None,
            instance_dtemp: 0.0,
            instance_ts: None,
            instance_td: None,
            // Leave branch-state uninitialized until the first Newton update so
            // HFET MODEINITJCT startup seeding can run.
            vgs: Value::NAN,
            vds: Value::NAN,
            vgs_prev: Value::NAN,
            vds_prev: Value::NAN,
            last_raw_vgs: Value::NAN,
            last_raw_vgd: Value::NAN,
            eval_valid: false,
            limiter_applied: false,
            eval_ids: 0.0,
            eval_gm: 0.0,
            eval_gds: 0.0,
            eval_igs: 0.0,
            eval_igd: 0.0,
            eval_ggs: 0.0,
            eval_ggd: 0.0,
            eval_vds_linear: 0.0,
            lin_vgs: 0.0,
            lin_vgd: 0.0,
            lin_cg: 0.0,
            lin_cd: 0.0,
            model_order: usize::MAX,
            hfet_legacy_inverse_mode: false,
            hfet_legacy_inverse_active: false,
            indices: JfetIndices::default(),
        }
    }

    /// Create a new P-JFET
    pub fn pjf(name: &str, drain: NodeId, gate: NodeId, source: NodeId) -> Self {
        Self {
            name: name.to_string(),
            jfet_type: JfetType::PJF,
            drain,
            gate,
            source,
            params: JfetParams::default(),
            m: 1.0,
            area: 1.0,
            width: 1e-6,
            length: 1e-6,
            instance_temp: None,
            instance_dtemp: 0.0,
            instance_ts: None,
            instance_td: None,
            // Leave branch-state uninitialized until the first Newton update so
            // HFET MODEINITJCT startup seeding can run.
            vgs: Value::NAN,
            vds: Value::NAN,
            vgs_prev: Value::NAN,
            vds_prev: Value::NAN,
            last_raw_vgs: Value::NAN,
            last_raw_vgd: Value::NAN,
            eval_valid: false,
            limiter_applied: false,
            eval_ids: 0.0,
            eval_gm: 0.0,
            eval_gds: 0.0,
            eval_igs: 0.0,
            eval_igd: 0.0,
            eval_ggs: 0.0,
            eval_ggd: 0.0,
            eval_vds_linear: 0.0,
            lin_vgs: 0.0,
            lin_vgd: 0.0,
            lin_cg: 0.0,
            lin_cd: 0.0,
            model_order: usize::MAX,
            hfet_legacy_inverse_mode: false,
            hfet_legacy_inverse_active: false,
            indices: JfetIndices::default(),
        }
    }

    /// Set model parameters
    pub fn with_params(mut self, params: JfetParams) -> Self {
        self.params = params;
        self
    }

    /// Set device multiplier
    pub fn with_multiplier(mut self, m: Value) -> Self {
        self.m = m;
        self
    }

    /// Set area factor
    pub fn with_area(mut self, area: Value) -> Self {
        self.area = area;
        self
    }

    /// Enable HFET1-compatible channel equations and defaults.
    ///
    /// Used for MESFET/HFET model families (`NMF/PMF/NHFET/PHFET`).
    pub fn enable_hfet_model(mut self) -> Self {
        let is_n = matches!(self.jfet_type, JfetType::NJF);
        self.params.channel_model = JfetChannelModel::Hfet1;
        self.hfet_legacy_inverse_mode = true;
        self.params.hfet_level = 5;
        self.params.vto = if is_n { 0.15 } else { -0.15 };
        self.params.lambda = 0.15;
        self.params.eta = if is_n { 1.28 } else { 1.4 };
        self.params.hfet_m = 3.0;
        self.params.hfet_mc = 3.0;
        self.params.hfet_gamma = 3.0;
        self.params.sigma0 = 0.057;
        self.params.hfet_vsigmat = 0.3;
        self.params.hfet_vsigma = 0.1;
        self.params.hfet_mu = if is_n { 0.4 } else { 0.03 };
        self.params.hfet_di = 0.04e-6;
        self.params.hfet_delta = 3.0;
        self.params.hfet_vs = if is_n { 1.5e5 } else { 0.8e5 };
        self.params.hfet_nmax = 2.0e16;
        self.params.hfet_deltad = 4.5e-9;
        self.params.hfet_rdi = 0.0;
        self.params.hfet_rsi = 0.0;
        self.params.hfet_epsi = 12.244 * 8.85418e-12;
        self.params.hfet_js1s = 1.0;
        self.params.hfet_js2s = 1.15e6;
        self.params.hfet_js1d = 1.0;
        self.params.hfet_js2d = 1.15e6;
        self.params.hfet_m1s = 1.32;
        self.params.hfet_m2s = 6.9;
        self.params.hfet_m1d = 1.32;
        self.params.hfet_m2d = 6.9;
        self.params.hfet_rgs = 90.0;
        self.params.hfet_rgd = 90.0;
        self.params.hfet_ggr = 40.0;
        self.params.hfet_del = 0.04;
        self.params.hfet_eta1 = 2.0;
        self.params.hfet_d1 = 0.03e-6;
        self.params.hfet_vt1 = Value::NAN;
        self.params.hfet_p = 1.0;
        self.params.hfet_kappa = 0.0;
        self.params.hfet_delf_freq = 0.0;
        self.params.hfet_fgds = 0.0;
        self.params.hfet_tf = 300.15;
        self.params.hfet_cds = 0.0;
        self.params.mesa_astar = 4.0e4;
        self.params.mesa_phib = 0.5 * 1.602176634e-19;
        self.params.mesa_xchi = 0.033;
        self.params.mesa_du = 0.035e-6;
        self.params.mesa_nd = 2.0e23;
        self.params.mesa_ndu = 1.0e22;
        self.params.mesa_th = 0.01e-6;
        self.params.mesa_ndelta = 6.0e24;
        self.params.mesa_theta = 0.0;
        self.params.mesa_alpha = 0.0;
        self.params.mesa_tc = 0.0;
        self.params.mesa_zeta = 1.0;
        self.params.mesa_lambdahf = Value::NAN;
        self.params.mesa_tf = 300.15;
        self.params.mesa_flo = 0.0;
        self.params.mesa_delfo = 0.0;
        self.params.mesa_cas = 1.0;
        self.params.mesa_cbs = 1.0;
        self.width = 20e-6;
        self.length = 1e-6;
        self.area = 1.0;
        self.vgs = Value::NAN;
        self.vds = Value::NAN;
        self.vgs_prev = Value::NAN;
        self.vds_prev = Value::NAN;
        self.eval_valid = false;
        self.limiter_applied = false;
        self
    }

    /// Enable MESA-compatible defaults (NMF/PMF level=2..4 family).
    pub fn enable_mesa_model(mut self) -> Self {
        let is_n = matches!(self.jfet_type, JfetType::NJF);
        self.params.channel_model = JfetChannelModel::Hfet1;
        // ngspice MESA/HFET2 level-2..4 handles inverse mode per instance;
        // it does not use the HFET1 global inverse latch quirk.
        self.hfet_legacy_inverse_mode = false;
        self.params.hfet_level = 2;
        self.params.vto = if is_n { -1.26 } else { 1.26 };
        self.params.beta = 0.0085;
        self.params.lambda = 0.045;
        self.params.eta = 1.73;
        self.params.hfet_m = 2.5;
        self.params.hfet_mc = 3.0;
        self.params.sigma0 = 0.081;
        self.params.hfet_vsigmat = 1.01;
        self.params.hfet_vsigma = 0.1;
        self.params.hfet_mu = 0.23;
        self.params.hfet_di = 0.12e-6;
        self.params.hfet_delta = 5.0;
        self.params.hfet_vs = 1.5e5;
        self.params.hfet_nmax = 2.0e16;
        self.params.hfet_deltad = 0.0;
        self.params.hfet_epsi = 12.244 * 8.85418e-12;
        self.params.hfet_rdi = 0.0;
        self.params.hfet_rsi = 0.0;
        self.params.is = 0.0;
        self.params.n = 1.0;
        self.params.hfet_ggr = 40.0;
        self.params.hfet_del = 0.04;
        self.params.hfet_eta1 = 2.0;
        self.params.hfet_d1 = 0.03e-6;
        self.params.hfet_vt1 = Value::NAN;
        self.params.hfet_p = 1.0;
        self.params.hfet_kappa = 0.0;
        self.params.hfet_delf_freq = 0.0;
        self.params.hfet_fgds = 0.0;
        self.params.hfet_tf = 300.15;
        self.params.hfet_cds = 0.0;
        self.params.mesa_astar = 4.0e4;
        self.params.mesa_phib = 0.5 * 1.602176634e-19;
        self.params.mesa_xchi = 0.033;
        self.params.mesa_du = 0.035e-6;
        self.params.mesa_nd = 2.0e23;
        self.params.mesa_ndu = 1.0e22;
        self.params.mesa_th = 0.01e-6;
        self.params.mesa_ndelta = 6.0e24;
        self.params.mesa_theta = 0.0;
        self.params.mesa_alpha = 0.0;
        self.params.mes_b = 0.3;
        self.params.mesa_tc = 0.0;
        self.params.mesa_zeta = 1.0;
        self.params.mesa_lambdahf = Value::NAN;
        self.params.mesa_tf = 300.15;
        self.params.mesa_flo = 0.0;
        self.params.mesa_delfo = 0.0;
        self.params.mesa_cas = 1.0;
        self.params.mesa_cbs = 1.0;
        self.width = 20e-6;
        self.length = 1e-6;
        self.area = 1.0;
        self.vgs = Value::NAN;
        self.vds = Value::NAN;
        self.vgs_prev = Value::NAN;
        self.vds_prev = Value::NAN;
        self.eval_valid = false;
        self.limiter_applied = false;
        self
    }

    /// Enable Berkeley SPICE level-1 MESFET defaults (`mes` device).
    pub fn enable_legacy_mesfet_model(mut self) -> Self {
        let is_n = matches!(self.jfet_type, JfetType::NJF);
        self.params.channel_model = JfetChannelModel::LegacyMesfet;
        self.hfet_legacy_inverse_mode = false;
        self.params.hfet_level = 1;
        self.params.vto = if is_n { -2.0 } else { 2.0 };
        self.params.beta = 2.5e-3;
        self.params.lambda = 0.0;
        self.params.mes_b = 0.3;
        self.params.mesa_alpha = 2.0;
        self.params.rd = 0.0;
        self.params.rs = 0.0;
        self.params.is = 1.0e-14;
        self.params.cgs = 0.0;
        self.params.cgd = 0.0;
        self.params.pb = 1.0;
        self.params.fc = 0.5;
        self.width = 20e-6;
        self.length = 1e-6;
        self.area = 1.0;
        self.vgs = Value::NAN;
        self.vds = Value::NAN;
        self.vgs_prev = Value::NAN;
        self.vds_prev = Value::NAN;
        self.eval_valid = false;
        self.limiter_applied = false;
        self
    }

    /// Set model parameters from a HashMap (for .MODEL statement parsing)
    pub fn with_model_params(mut self, params: &std::collections::HashMap<String, Value>) -> Self {
        let mut p = self.params.clone();

        if let Some(level) = params.get("LEVEL").copied().filter(|v| v.is_finite()) {
            p.hfet_level = level.round() as i32;
        }

        if let Some(v) = params
            .get("VTO")
            .or_else(|| params.get("VT0"))
            .copied()
            .filter(|v| v.is_finite())
        {
            p.vto = v;
        }

        let beta_from_card = params
            .get("BETA")
            .copied()
            .or_else(|| {
                (!matches!(p.channel_model, JfetChannelModel::LegacyMesfet))
                    .then(|| params.get("B").copied())
                    .flatten()
            })
            .filter(|v| v.is_finite() && *v >= 0.0);
        let idss_from_card = params
            .get("IDSS")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0);
        if let Some(beta) = beta_from_card {
            p.beta = beta;
        } else if let Some(idss) = idss_from_card {
            let vto2 = p.vto * p.vto;
            if vto2 > 1e-30 {
                p.beta = idss / vto2;
            }
        }

        if let Some(v) = params
            .get("LAMBDA")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.lambda = v;
        }
        if let Some(v) = params
            .get("IS")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.is = v;
        }
        if let Some(v) = params
            .get("JS")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.is = v;
        }
        if let Some(v) = params
            .get("CGS")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.cgs = v;
        }
        if let Some(v) = params
            .get("CGD")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.cgd = v;
        }
        if let Some(v) = params
            .get("PB")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.pb = v;
        }
        if let Some(v) = params
            .get("M")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            if matches!(p.channel_model, JfetChannelModel::Hfet1) {
                p.hfet_m = v;
            } else {
                p.m = v;
            }
        }

        if matches!(p.channel_model, JfetChannelModel::Hfet1) {
            if let Some(v) = params
                .get("RD")
                .copied()
                .filter(|v| v.is_finite() && *v >= 0.0)
            {
                p.rd = v;
            }
            if let Some(v) = params
                .get("RS")
                .copied()
                .filter(|v| v.is_finite() && *v >= 0.0)
            {
                p.rs = v;
            }
            if let Some(v) = params
                .get("RDI")
                .copied()
                .filter(|v| v.is_finite() && *v >= 0.0)
            {
                p.hfet_rdi = v;
            }
            if let Some(v) = params
                .get("RSI")
                .copied()
                .filter(|v| v.is_finite() && *v >= 0.0)
            {
                p.hfet_rsi = v;
            }
        } else {
            if let Some(v) = params
                .get("RD")
                .or_else(|| params.get("RDI"))
                .copied()
                .filter(|v| v.is_finite() && *v >= 0.0)
            {
                p.rd = v;
            }
            if let Some(v) = params
                .get("RS")
                .or_else(|| params.get("RSI"))
                .copied()
                .filter(|v| v.is_finite() && *v >= 0.0)
            {
                p.rs = v;
            }
        }

        if let Some(v) = params
            .get("FC")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0 && *v < 1.0)
        {
            p.fc = v;
        }
        if let Some(v) = params
            .get("N")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.n = v;
        }
        if let Some(v) = params.get("ETA").copied().filter(|v| v.is_finite()) {
            p.eta = v;
        }
        if let Some(v) = params.get("THETA").copied().filter(|v| v.is_finite()) {
            p.mesa_theta = v;
        }
        if let Some(v) = params.get("ALPHA").copied().filter(|v| v.is_finite()) {
            p.mesa_alpha = v;
        }
        if let Some(v) = params
            .get("B")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.mes_b = v;
        }
        if let Some(v) = params.get("TC").copied().filter(|v| v.is_finite()) {
            p.mesa_tc = v;
        }
        if let Some(v) = params
            .get("ZETA")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.mesa_zeta = v;
        }
        if let Some(v) = params
            .get("LAMBDAHF")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.mesa_lambdahf = v;
        }
        if let Some(v) = params
            .get("SIGMA0")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.sigma0 = v;
        }
        if let Some(v) = params
            .get("KF")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.kf = v;
        }
        if let Some(v) = params
            .get("AF")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.af = v;
        }
        if let Some(v) = params
            .get("EF")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.ef = v;
        }

        if let Some(v) = params
            .get("MC")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.hfet_mc = v;
        }
        if let Some(v) = params
            .get("GAMMA")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.hfet_gamma = v;
        }
        if let Some(v) = params.get("VSIGMAT").copied().filter(|v| v.is_finite()) {
            p.hfet_vsigmat = v;
        }
        if let Some(v) = params
            .get("VSIGMA")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.hfet_vsigma = v;
        }
        if let Some(v) = params
            .get("MU")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.hfet_mu = v;
        }
        if let Some(v) = params
            .get("DI")
            .or_else(|| params.get("D"))
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.hfet_di = v;
        }
        if let Some(v) = params
            .get("DU")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.mesa_du = v;
        }
        if let Some(v) = params
            .get("ND")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.mesa_nd = v;
        }
        if let Some(v) = params
            .get("NDU")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.mesa_ndu = v;
        }
        if let Some(v) = params
            .get("TH")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.mesa_th = v;
        }
        if let Some(v) = params
            .get("NDELTA")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.mesa_ndelta = v;
        }
        if let Some(v) = params
            .get("DELTA")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.hfet_delta = v;
        }
        if let Some(v) = params
            .get("VS")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.hfet_vs = v;
        }
        if let Some(v) = params
            .get("NMAX")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.hfet_nmax = v;
        }
        if let Some(v) = params
            .get("DELTAD")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.hfet_deltad = v;
        }
        if let Some(v) = params
            .get("EPSI")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.hfet_epsi = v;
        }
        if let Some(v) = params
            .get("CAS")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.mesa_cas = v;
        }
        if let Some(v) = params
            .get("CBS")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.mesa_cbs = v;
        }
        if let Some(v) = params
            .get("JS1S")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.hfet_js1s = v;
        }
        if let Some(v) = params
            .get("JS2S")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.hfet_js2s = v;
        }
        if let Some(v) = params
            .get("JS1D")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.hfet_js1d = v;
        }
        if let Some(v) = params
            .get("JS2D")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.hfet_js2d = v;
        }
        if let Some(v) = params
            .get("M1S")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.hfet_m1s = v;
        }
        if let Some(v) = params
            .get("M2S")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.hfet_m2s = v;
        }
        if let Some(v) = params
            .get("M1D")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.hfet_m1d = v;
        }
        if let Some(v) = params
            .get("M2D")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.hfet_m2d = v;
        }
        if let Some(v) = params
            .get("RGS")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.hfet_rgs = v;
        }
        if let Some(v) = params
            .get("RGD")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.hfet_rgd = v;
        }
        if let Some(v) = params
            .get("GGR")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.hfet_ggr = v;
        }
        if let Some(v) = params.get("DEL").copied().filter(|v| v.is_finite()) {
            p.hfet_del = v;
        }
        if let Some(v) = params
            .get("ETA1")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.hfet_eta1 = v;
        }
        if let Some(v) = params
            .get("D1")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.hfet_d1 = v;
        }
        if let Some(v) = params.get("VT1").copied().filter(|v| v.is_finite()) {
            p.hfet_vt1 = v;
        }
        if let Some(v) = params
            .get("P")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.hfet_p = v;
        }
        if let Some(v) = params.get("KAPPA").copied().filter(|v| v.is_finite()) {
            p.hfet_kappa = v;
        }
        if let Some(v) = params
            .get("DELF")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.hfet_delf_freq = v;
        }
        if let Some(v) = params
            .get("FGDS")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.hfet_fgds = v;
        }
        if let Some(v) = params
            .get("CDS")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.hfet_cds = v;
        }
        if let Some(v) = params
            .get("ASTAR")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.mesa_astar = v;
        }
        if let Some(v) = params
            .get("PHIB")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.mesa_phib = v;
        }
        if let Some(v) = params.get("XCHI").copied().filter(|v| v.is_finite()) {
            p.mesa_xchi = v;
        }
        if let Some(v) = params.get("TF").copied().filter(|v| v.is_finite()) {
            let tf_k = v + 273.15;
            if matches!(p.hfet_level, 2..=4) {
                p.mesa_tf = tf_k;
            } else {
                p.hfet_tf = tf_k;
            }
        }
        if let Some(v) = params
            .get("FLO")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.mesa_flo = v;
        }
        if let Some(v) = params
            .get("DELFO")
            .copied()
            .filter(|v| v.is_finite() && *v >= 0.0)
        {
            p.mesa_delfo = v;
        }
        if let Some(v) = params
            .get("TNOM")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0)
        {
            p.tnom = v;
        }
        self.params = p;
        self
    }

    /// Apply instance-level JFET/MESFET geometry and multiplicity parameters.
    ///
    /// Supported keys:
    /// - `AREA`: direct area scaling
    /// - `M` / `MULT`: multiplicity
    /// - `W`, `L`, optional `NF`: width/length scaling fallback (`W/L * NF`)
    pub fn with_instance_params(mut self, params: &[(String, Value)]) -> Self {
        let mut area_override: Option<Value> = None;
        let mut width: Option<Value> = None;
        let mut length: Option<Value> = None;
        let mut nf = 1.0;
        let mut mult = 1.0;
        let mut temp_override: Option<Value> = None;
        let mut dtemp = 0.0;
        let mut ts_override: Option<Value> = None;
        let mut td_override: Option<Value> = None;

        for (name, value) in params {
            if !value.is_finite() {
                continue;
            }

            if name.eq_ignore_ascii_case("AREA") {
                if *value > 0.0 {
                    area_override = Some(*value);
                }
                continue;
            }

            if name.eq_ignore_ascii_case("W") {
                if *value > 0.0 {
                    width = Some(*value);
                }
                continue;
            }

            if name.eq_ignore_ascii_case("L") {
                if *value > 0.0 {
                    length = Some(*value);
                }
                continue;
            }

            if name.eq_ignore_ascii_case("NF") {
                if *value > 0.0 {
                    nf = *value;
                }
                continue;
            }

            if name.eq_ignore_ascii_case("M") || name.eq_ignore_ascii_case("MULT") {
                if *value > 0.0 {
                    mult = *value;
                }
                continue;
            }

            if name.eq_ignore_ascii_case("TEMP") {
                if *value > 0.0 {
                    temp_override = Some(*value + 273.15);
                }
                continue;
            }

            if name.eq_ignore_ascii_case("DTEMP") {
                dtemp = *value;
                continue;
            }

            if name.eq_ignore_ascii_case("TS") {
                if *value > 0.0 {
                    ts_override = Some(*value + 273.15);
                }
                continue;
            }

            if name.eq_ignore_ascii_case("TD") && *value > 0.0 {
                td_override = Some(*value + 273.15);
            }
        }

        if let Some(w) = width {
            self.width = w;
        }
        if let Some(l) = length {
            self.length = l;
        }

        if let Some(area) = area_override {
            self.area *= area;
        } else if !matches!(self.params.channel_model, JfetChannelModel::Hfet1)
            && let (Some(w), Some(l)) = (width, length)
        {
            let wl_scale = w / l;
            if wl_scale.is_finite() && wl_scale > 0.0 {
                self.area *= wl_scale * nf;
            }
        }

        if matches!(self.params.channel_model, JfetChannelModel::Hfet1)
            && nf.is_finite()
            && nf > 0.0
        {
            self.width *= nf;
        }

        if mult.is_finite() && mult > 0.0 {
            self.m *= mult;
        }

        self.instance_temp = temp_override;
        self.instance_dtemp = dtemp;
        self.instance_ts = ts_override;
        self.instance_td = td_override;

        self
    }
}
