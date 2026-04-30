//! JFET model parameter defaults and builders.

use super::*;

impl Default for JfetParams {
    fn default() -> Self {
        Self {
            vto: -2.0,   // Threshold voltage (depletion mode)
            beta: 1e-4,  // Transconductance coefficient
            lambda: 0.0, // Channel-length modulation
            is: 1e-14,   // Gate saturation current
            cgs: 0.0,    // Gate-source capacitance
            cgd: 0.0,    // Gate-drain capacitance
            pb: 1.0,     // Junction potential
            m: 0.5,      // Grading coefficient
            rd: 0.0,     // Drain resistance
            rs: 0.0,     // Source resistance
            fc: 0.5,     // Forward bias coefficient
            n: 1.0,      // Emission coefficient
            eta: 0.0,    // DIBL disabled by default
            sigma0: 0.0, // No extra linear channel conductivity by default
            kf: 0.0,
            af: 1.0,
            ef: 1.0,
            tnom: 300.15, // 27C nominal
            channel_model: JfetChannelModel::ShichmanHodges,
            hfet_level: 2,
            hfet_gatemod: false,
            hfet_m: 3.0,
            hfet_mc: 3.0,
            hfet_gamma: 3.0,
            hfet_vsigmat: 0.3,
            hfet_vsigma: 0.1,
            hfet_mu: 0.4,
            hfet_di: 0.04e-6,
            hfet_delta: 3.0,
            hfet_vs: 1.5e5,
            hfet_nmax: 2.0e16,
            hfet_deltad: 4.5e-9,
            hfet_rdi: 0.0,
            hfet_rsi: 0.0,
            hfet_epsi: 12.244 * 8.85418e-12,
            hfet_js1s: 1.0,
            hfet_js2s: 1.15e6,
            hfet_js1d: 1.0,
            hfet_js2d: 1.15e6,
            hfet_m1s: 1.32,
            hfet_m2s: 6.9,
            hfet_m1d: 1.32,
            hfet_m2d: 6.9,
            hfet_rgs: 90.0,
            hfet_rgd: 90.0,
            hfet_ggr: 40.0,
            hfet_del: 0.04,
            hfet_eta1: 2.0,
            hfet_d1: 0.03e-6,
            hfet_vt1: Value::NAN,
            hfet_p: 1.0,
            hfet_kappa: 0.0,
            hfet_delf_freq: 0.0,
            hfet_fgds: 0.0,
            hfet_tf: 300.15,
            hfet_cds: 0.0,
            mesa_astar: 4.0e4,
            mesa_phib: 0.5 * 1.602176634e-19,
            mesa_xchi: 0.033,
            mesa_du: 0.035e-6,
            mesa_nd: 2.0e23,
            mesa_ndu: 1.0e22,
            mesa_th: 0.01e-6,
            mesa_ndelta: 6.0e24,
            mesa_theta: 0.0,
            mesa_alpha: 0.0,
            mes_b: 0.3,
            mesa_tc: 0.0,
            mesa_zeta: 1.0,
            mesa_lambdahf: Value::NAN,
            mesa_tf: 300.15,
            mesa_flo: 0.0,
            mesa_delfo: 0.0,
            mesa_cas: 1.0,
            mesa_cbs: 1.0,
        }
    }
}

impl JfetParams {
    /// Create parameters from IDSS and VTO
    ///
    /// IDSS is the drain current at Vgs=0, Vds >> Vgs-Vto (saturation)
    /// Beta = IDSS / VtoÂ²
    pub fn from_idss(idss: Value, vto: Value) -> Self {
        let beta = idss / (vto * vto);
        Self {
            vto,
            beta,
            ..Default::default()
        }
    }

    /// Create with specified parameters
    pub fn new() -> Self {
        Self::default()
    }

    /// Set VTO
    pub fn with_vto(mut self, vto: Value) -> Self {
        self.vto = vto;
        self
    }

    /// Set BETA
    pub fn with_beta(mut self, beta: Value) -> Self {
        self.beta = beta;
        self
    }

    /// Set LAMBDA
    pub fn with_lambda(mut self, lambda: Value) -> Self {
        self.lambda = lambda;
        self
    }

    /// Set capacitances
    pub fn with_capacitances(mut self, cgs: Value, cgd: Value) -> Self {
        self.cgs = cgs;
        self.cgd = cgd;
        self
    }

    /// Set junction parameters
    pub fn with_junction(mut self, is: Value, pb: Value) -> Self {
        self.is = is;
        self.pb = pb;
        self
    }
}
