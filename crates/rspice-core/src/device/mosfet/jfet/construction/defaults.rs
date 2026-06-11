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
            last_raw_vgs_prev: Value::NAN,
            last_raw_vgd_prev: Value::NAN,
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
            eval_gmg: 0.0,
            eval_gmd: 0.0,
            eval_ggd: 0.0,
            eval_vds_linear: 0.0,
            lin_vgs: 0.0,
            lin_vgd: 0.0,
            lin_cg: 0.0,
            lin_cd: 0.0,
            model_order: usize::MAX,
            hfet_legacy_inverse_mode: false,
            hfet_legacy_inverse_active: false,
            junction_gmin: 1.0e-12,
            gate_generation_scale: 1.0,
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
            last_raw_vgs_prev: Value::NAN,
            last_raw_vgd_prev: Value::NAN,
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
            eval_gmg: 0.0,
            eval_gmd: 0.0,
            eval_ggd: 0.0,
            eval_vds_linear: 0.0,
            lin_vgs: 0.0,
            lin_vgd: 0.0,
            lin_cg: 0.0,
            lin_cd: 0.0,
            model_order: usize::MAX,
            hfet_legacy_inverse_mode: false,
            hfet_legacy_inverse_active: false,
            junction_gmin: 1.0e-12,
            gate_generation_scale: 1.0,
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
        self.params.hfet_gatemod = false;
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
        self.params.hfet_gatemod = false;
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
}

