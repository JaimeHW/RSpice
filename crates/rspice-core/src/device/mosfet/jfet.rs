//! JFET (Junction Field-Effect Transistor) Device Model
//!
//! Implements the Shichman-Hodges model for N-channel and P-channel JFETs.
//!
//! # Model Equations
//!
//! For N-JFET (P-JFET uses opposite polarities):
//!
//! **Cutoff** (Vgs - Vto â‰¤ 0):
//! ```text
//! Ids = 0
//! ```
//!
//! **Linear** (Vds < Vgs - Vto):
//! ```text
//! Ids = Beta * (2*(Vgs-Vto)*Vds - VdsÂ²) * (1 + Lambda*Vds)
//! ```
//!
//! **Saturation** (Vds â‰¥ Vgs - Vto):
//! ```text
//! Ids = Beta * (Vgs - Vto)Â² * (1 + Lambda*Vds)
//! ```
//!
//! where Beta is typically derived from IDSS: `Beta = IDSS / VtoÂ²`
//!
//! # Example
//!
//! ```ignore
//! J1 drain gate source JMOD
//! .MODEL JMOD NJF(VTO=-2 BETA=1E-3 LAMBDA=0.01)
//! ```

#![allow(clippy::too_many_arguments)]
use crate::Value;
use crate::circuit::NodeId;
use crate::device::traits::{MatrixStamper, NonlinearConvergenceCriteria, NonlinearDevice};
use crate::solver::{CscIndex, StaticMatrix};

//=============================================================================
// JFET Type
//=============================================================================

/// JFET channel type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JfetType {
    /// N-channel JFET (current flows drain to source)
    NJF,
    /// P-channel JFET (current flows source to drain)
    PJF,
}

impl JfetType {
    /// Get polarity multiplier (+1 for NJF, -1 for PJF)
    pub fn polarity(&self) -> Value {
        match self {
            JfetType::NJF => 1.0,
            JfetType::PJF => -1.0,
        }
    }
}

/// Channel current model selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum JfetChannelModel {
    /// Classic Shichman-Hodges JFET equations.
    ShichmanHodges,
    /// HFET1-compatible MESFET channel equations (ngspice-derived).
    Hfet1,
}

//=============================================================================
// JFET Parameters
//=============================================================================

/// JFET model parameters (Shichman-Hodges level 1)
#[derive(Debug, Clone)]
pub struct JfetParams {
    /// Threshold voltage (V) - negative for N-JFET depletion mode
    pub vto: Value,
    /// Transconductance coefficient (A/VÂ²)
    pub beta: Value,
    /// Channel-length modulation (1/V)
    pub lambda: Value,
    /// Gate junction saturation current (A)
    pub is: Value,
    /// Gate-source zero-bias capacitance (F)
    pub cgs: Value,
    /// Gate-drain zero-bias capacitance (F)
    pub cgd: Value,
    /// Gate junction potential (V)
    pub pb: Value,
    /// Capacitance grading coefficient
    pub m: Value,
    /// Drain ohmic resistance (Î©)
    pub rd: Value,
    /// Source ohmic resistance (Î©)
    pub rs: Value,
    /// Forward bias junction coefficient
    pub fc: Value,
    /// Gate junction emission coefficient
    pub n: Value,
    /// HFET-style DIBL/overdrive modulation coefficient
    pub eta: Value,
    /// HFET/MESFET low-field channel conductivity term
    pub sigma0: Value,
    /// Flicker noise coefficient (KF)
    pub kf: Value,
    /// Flicker noise current exponent (AF)
    pub af: Value,
    /// Flicker noise frequency exponent (EF)
    pub ef: Value,
    /// Nominal temperature (K)
    pub tnom: Value,
    /// Channel model to evaluate.
    pub channel_model: JfetChannelModel,
    /// MESFET/HFET level selector from model card.
    /// - `<=1`: HFET1-style Schottky leak model
    /// - `2..=4`: MESA-style gate branches
    /// - `>=5`: HFET2-style gate branches
    pub hfet_level: i32,
    /// HFET/MESFET knee-shape parameter `M`.
    pub hfet_m: Value,
    /// HFET/MESFET capacitance knee parameter `MC`.
    pub hfet_mc: Value,
    /// HFET/MESFET knee-shape exponent `GAMMA`.
    pub hfet_gamma: Value,
    /// HFET/MESFET sigma transition voltage `VSIGMAT`.
    pub hfet_vsigmat: Value,
    /// HFET/MESFET sigma slope voltage `VSIGMA`.
    pub hfet_vsigma: Value,
    /// HFET/MESFET channel mobility `MU`.
    pub hfet_mu: Value,
    /// HFET/MESFET channel depth `DI`.
    pub hfet_di: Value,
    /// HFET/MESFET smoothing parameter `DELTA`.
    pub hfet_delta: Value,
    /// HFET/MESFET saturation velocity `VS`.
    pub hfet_vs: Value,
    /// HFET/MESFET maximum carrier density `NMAX`.
    pub hfet_nmax: Value,
    /// HFET/MESFET depth correction `DELTAD`.
    pub hfet_deltad: Value,
    /// HFET/MESFET internal drain resistance `RDI`.
    pub hfet_rdi: Value,
    /// HFET/MESFET internal source resistance `RSI`.
    pub hfet_rsi: Value,
    /// HFET/MESFET dielectric constant `EPSI`.
    pub hfet_epsi: Value,
    /// HFET/MESFET Schottky source leakage coefficient `JS1S`.
    pub hfet_js1s: Value,
    /// HFET/MESFET Schottky source leakage coefficient `JS2S`.
    pub hfet_js2s: Value,
    /// HFET/MESFET Schottky drain leakage coefficient `JS1D`.
    pub hfet_js1d: Value,
    /// HFET/MESFET Schottky drain leakage coefficient `JS2D`.
    pub hfet_js2d: Value,
    /// HFET/MESFET leakage ideality coefficient `M1S`.
    pub hfet_m1s: Value,
    /// HFET/MESFET leakage ideality coefficient `M2S`.
    pub hfet_m2s: Value,
    /// HFET/MESFET leakage ideality coefficient `M1D`.
    pub hfet_m1d: Value,
    /// HFET/MESFET leakage ideality coefficient `M2D`.
    pub hfet_m2d: Value,
    /// HFET/MESFET source gate resistance `RGS`.
    pub hfet_rgs: Value,
    /// HFET/MESFET drain gate resistance `RGD`.
    pub hfet_rgd: Value,
    /// HFET/MESFET gate generation-recombination parameter `GGR`.
    pub hfet_ggr: Value,
    /// HFET/MESFET gate leakage exponential coefficient `DEL`.
    pub hfet_del: Value,
    /// HFET1 capacitance branch ideality parameter `ETA1`.
    pub hfet_eta1: Value,
    /// HFET1 capacitance depth parameter `D1`.
    pub hfet_d1: Value,
    /// HFET1 capacitance threshold-shift parameter `VT1`.
    pub hfet_vt1: Value,
    /// HFET1 capacitance partition parameter `P`.
    pub hfet_p: Value,
    /// MESA/HFET emission constant `ASTAR`.
    pub mesa_astar: Value,
    /// MESA/HFET barrier potential `PHIB` (J).
    pub mesa_phib: Value,
    /// MESA/HFET G-R temperature coefficient `XCHI`.
    pub mesa_xchi: Value,
    /// MESA thickness parameter `DU`.
    pub mesa_du: Value,
    /// MESA channel doping `ND`.
    pub mesa_nd: Value,
    /// MESA upper-layer doping `NDU`.
    pub mesa_ndu: Value,
    /// MESA upper-layer thickness `TH`.
    pub mesa_th: Value,
    /// MESA transition doping `NDELTA`.
    pub mesa_ndelta: Value,
    /// MESA mobility-bias coefficient `THETA`.
    pub mesa_theta: Value,
    /// MESA velocity-modulation coefficient `ALPHA`.
    pub mesa_alpha: Value,
    /// MESA current softening coefficient `TC`.
    pub mesa_tc: Value,
    /// MESA transport factor `ZETA`.
    pub mesa_zeta: Value,
    /// MESA level-4 accumulation capacitance scale `CAS`.
    pub mesa_cas: Value,
    /// MESA level-4 depletion capacitance scale `CBS`.
    pub mesa_cbs: Value,
}

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
            mesa_tc: 0.0,
            mesa_zeta: 1.0,
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

//=============================================================================
// JFET Device
//=============================================================================

/// Pre-computed stamp indices for O(1) matrix access.
#[derive(Debug, Clone, Default)]
pub struct JfetIndices {
    // Drain row
    pub dd: Option<CscIndex>,
    pub dg: Option<CscIndex>,
    pub ds: Option<CscIndex>,
    // Gate row
    pub gd: Option<CscIndex>,
    pub gg: Option<CscIndex>,
    pub gs: Option<CscIndex>,
    // Source row
    pub sd: Option<CscIndex>,
    pub sg: Option<CscIndex>,
    pub ss: Option<CscIndex>,
}

/// JFET device instance
#[derive(Debug, Clone)]
pub struct Jfet {
    /// Instance name
    pub name: String,
    /// JFET type (NJF or PJF)
    pub jfet_type: JfetType,
    /// Drain node index
    pub drain: NodeId,
    /// Gate node index
    pub gate: NodeId,
    /// Source node index
    pub source: NodeId,
    /// Model parameters
    pub params: JfetParams,
    /// Device multiplier
    pub m: Value,
    /// Area factor
    pub area: Value,
    /// Effective channel width (m) for HFET-compatible equations.
    pub width: Value,
    /// Effective channel length (m) for HFET-compatible equations.
    pub length: Value,
    /// Optional instance TEMP override.
    instance_temp: Option<Value>,
    /// Optional instance DTEMP offset added when TEMP is not given.
    instance_dtemp: Value,
    /// Optional instance source terminal temperature override.
    instance_ts: Option<Value>,
    /// Optional instance drain terminal temperature override.
    instance_td: Option<Value>,
    /// Previous/current iteration state for convergence checks
    vgs: Value,
    vds: Value,
    vgs_prev: Value,
    vds_prev: Value,
    eval_valid: bool,
    limiter_applied: bool,
    eval_ids: Value,
    eval_gm: Value,
    eval_gds: Value,
    eval_igs: Value,
    eval_igd: Value,
    eval_ggs: Value,
    eval_ggd: Value,
    eval_vds_linear: Value,
    lin_vgs: Value,
    lin_vgd: Value,
    lin_cg: Value,
    lin_cd: Value,
    model_order: usize,
    hfet_legacy_inverse_mode: bool,
    hfet_legacy_inverse_active: bool,
    /// Pre-computed matrix indices for O(1) direct stamping
    pub indices: JfetIndices,
}

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
            .or_else(|| params.get("B"))
            .copied()
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

    /// Thermal voltage at given temperature
    fn thermal_voltage(&self, temp: Value) -> Value {
        const K_BOLTZMANN: Value = 1.380649e-23;
        const Q_ELECTRON: Value = 1.602176634e-19;
        K_BOLTZMANN * temp / Q_ELECTRON
    }

    #[inline]
    fn junction_scale(&self) -> Value {
        self.area * self.m
    }

    #[inline]
    fn resolved_temperatures(&self, ambient: Value) -> (Value, Value, Value) {
        let mut base = if ambient.is_finite() && ambient > 0.0 {
            ambient
        } else {
            self.params.tnom.max(1.0)
        };

        if let Some(temp) = self.instance_temp.filter(|v| v.is_finite() && *v > 0.0) {
            base = temp;
        } else {
            base += self.instance_dtemp;
        }
        if !base.is_finite() || base <= 0.0 {
            base = self.params.tnom.max(1.0);
        }

        let ts = self
            .instance_ts
            .filter(|v| v.is_finite() && *v > 0.0)
            .unwrap_or(base);
        let td = self
            .instance_td
            .filter(|v| v.is_finite() && *v > 0.0)
            .unwrap_or(base);

        (base.max(1.0), ts.max(1.0), td.max(1.0))
    }

    /// ngspice-compatible gate-junction branch evaluation for Level-1 JFETs.
    ///
    /// Mirrors `jfetload.c`:
    /// - reverse branch asymptote for `v < -3*n*Vt`
    /// - explicit `gmin * v` current contribution
    /// - explicit `+ gmin` small-signal conductance floor
    #[inline]
    fn junction_diode_terms(&self, v_ak: Value, temp: Value) -> (Value, Value) {
        const JFET_GMIN: Value = 1e-12;
        let temp_k = if temp.is_finite() && temp > 0.0 {
            temp
        } else {
            self.params.tnom.max(1.0)
        };
        let nvt = (self.params.n.max(1e-12) * self.thermal_voltage(temp_k)).max(1e-12);
        let isat = (self.params.is * self.junction_scale()).max(0.0);

        if v_ak < -3.0 * nvt {
            // ngspice `jfetload.c` reverse-bias asymptote:
            // arg = (3*vt/(v*e))^3
            let mut arg = 3.0 * nvt / (v_ak * std::f64::consts::E);
            arg = arg * arg * arg;
            let i = -isat * (1.0 + arg) + JFET_GMIN * v_ak;
            let g = isat * 3.0 * arg / v_ak + JFET_GMIN;
            if i.is_finite() && g.is_finite() {
                (i, g.max(JFET_GMIN))
            } else {
                (JFET_GMIN * v_ak, JFET_GMIN)
            }
        } else {
            // Clamp exponent for robustness outside pnjlim/fetlim regimes.
            let exp_term = (v_ak / nvt).clamp(-80.0, 80.0).exp();
            let i = isat * (exp_term - 1.0) + JFET_GMIN * v_ak;
            let g = isat * exp_term / nvt + JFET_GMIN;
            if i.is_finite() && g.is_finite() {
                (i, g.max(JFET_GMIN))
            } else {
                (JFET_GMIN * v_ak, JFET_GMIN)
            }
        }
    }

    /// Gate junction diode current for internal anode-cathode voltage.
    fn junction_diode_current(&self, v_ak: Value, temp: Value) -> Value {
        self.junction_diode_terms(v_ak, temp).0
    }

    /// Gate junction diode small-signal conductance for internal anode-cathode voltage.
    fn junction_diode_conductance(&self, v_ak: Value, temp: Value) -> Value {
        self.junction_diode_terms(v_ak, temp).1
    }

    #[inline]
    fn hfet_gate_geometry_scale(&self) -> Value {
        let w = self.width.max(1e-12);
        let l = self.length.max(1e-12);
        (0.5 * w * l * self.area.max(0.0) * self.m.max(0.0)).max(0.0)
    }

    /// ngspice HFET helper (`hfetload.c:diode`) used by `leak()`.
    fn hfet_diode_aux(u: Value) -> Value {
        const U0: Value = -2.303;
        const A: Value = 2.221;
        const B: Value = 6.804;
        const C: Value = 1.685;

        let expu = u.exp();
        let it = if u <= U0 {
            expu * (1.0 - expu)
        } else {
            let b = 0.5 * (u - U0);
            u + A * ((U0 - u) / B).exp() - (b + (b * b + 0.25 * C * C).sqrt()).ln()
        };
        let it = if it.is_finite() && it > 1e-30 {
            it
        } else {
            1e-30
        };
        let ut = it + it.ln();
        let b = u - ut;
        let c = 1.0 + it;
        it * (1.0 + b / c + 0.5 * b * b / (c * c * c))
    }

    /// ngspice HFET Schottky branch model (`hfetload.c:leak`).
    fn hfet_leak(
        gmin: Value,
        vt: Value,
        v: Value,
        rs: Value,
        is1: Value,
        is2: Value,
        m1: Value,
        m2: Value,
    ) -> (Value, Value) {
        let vt1 = (vt * m1).max(1e-18);
        let vt2 = (vt * m2).max(1e-18);
        let rs = rs.max(0.0);
        let is1 = is1.max(0.0);
        let is2 = is2.max(0.0);
        let gmin = gmin.max(1e-30);

        if v > -10.0 * vt1 {
            let vteff = (vt1 + vt2).max(1e-18);
            let msum = (m1 + m2).max(1e-18);
            let ratio = if is2 > 0.0 { is1 / is2 } else { 0.0 };
            let iseff = if is1 > 0.0 && is2 > 0.0 && ratio.is_finite() && ratio > 0.0 {
                is2 * ratio.powf(m1 / msum)
            } else {
                0.0
            };

            let (iaprox1, iaprox2) = if rs > 0.0 {
                let rsis1 = (rs * is1).max(1e-30);
                let rsiseff = (rs * iseff).max(1e-30);
                let u1 = (v + rs * is1) / vt1 + (rsis1 / vt1).ln();
                let u2 = (v + rs * iseff) / vteff + (rsiseff / vteff).ln();
                let i1 = vt1 * Self::hfet_diode_aux(u1) / rs - is1;
                let i2 = vteff * Self::hfet_diode_aux(u2) / rs - iseff;
                (i1, i2)
            } else {
                (
                    is1 * ((v / vt1).exp() - 1.0),
                    iseff * ((v / vteff).exp() - 1.0),
                )
            };

            let iaprox = if (iaprox1 * iaprox2) != 0.0 {
                1.0 / (1.0 / iaprox1 + 1.0 / iaprox2)
            } else {
                0.5 * (iaprox1 + iaprox2)
            };

            let dvdi0 = rs + vt1 / (iaprox + is1).max(1e-30) + vt2 / (iaprox + is2).max(1e-30);
            let v0 =
                rs * iaprox + vt1 * (iaprox / is1 + 1.0).ln() + vt2 * (iaprox / is2 + 1.0).ln();
            let il = (iaprox + (v - v0) / dvdi0).max(-is1) * 0.99999;
            let gl = 1.0 / (rs + vt1 / (il + is1).max(1e-30) + vt2 / (il + is2).max(1e-30));
            let il = if il.is_finite() { il } else { -is1 };
            let gl = if gl.is_finite() { gl.max(0.0) } else { gmin };
            (il, gl)
        } else {
            let gl = gmin;
            let il = gl * v - is1;
            (il, gl)
        }
    }

    /// HFET1 gate branch current + conductance for internal gate-source/drain voltage.
    fn hfet_gate_branch(
        &self,
        v_int: Value,
        temp: Value,
        js1: Value,
        js2: Value,
        m1: Value,
        m2: Value,
        rg: Value,
    ) -> (Value, Value) {
        const HFET_GMIN: Value = 1e-12;
        let temp_k = if temp.is_finite() && temp > 0.0 {
            temp
        } else {
            self.params.tnom.max(1.0)
        };
        let vt = self.thermal_voltage(temp_k).max(1e-12);
        let scale = self.hfet_gate_geometry_scale();
        let is1 = js1.max(0.0) * scale;
        let is2 = js2.max(0.0) * scale;

        let (mut il, mut gl) = if is1 > 0.0 && is2 > 0.0 {
            Self::hfet_leak(
                HFET_GMIN,
                vt,
                v_int,
                rg.max(0.0),
                is1,
                is2,
                m1.max(1e-12),
                m2.max(1e-12),
            )
        } else {
            (0.0, 0.0)
        };

        // ngspice HFET generation-recombination branch: GGRWL * v * exp(-v*DEL/vt)
        let ggrwl = self.params.hfet_ggr.max(0.0) * scale;
        if ggrwl > 0.0 {
            let arg = -v_int * self.params.hfet_del / vt;
            let arg_eff = arg.clamp(-80.0, 80.0);
            let earg = arg_eff.exp();
            il += ggrwl * v_int * earg;
            gl += ggrwl * earg * (1.0 - arg_eff);
        }

        if !il.is_finite() {
            il = 0.0;
        }
        if !gl.is_finite() {
            gl = 0.0;
        }
        (il, gl)
    }

    /// MESA gate branch approximation (`mesaload.c`): ASTAR Schottky + GGR + GMIN.
    fn mesa_gate_branch(&self, v_int: Value, temp: Value) -> (Value, Value) {
        const K_BOLTZMANN: Value = 1.380649e-23;
        const MESA_GMIN: Value = 1e-12;

        let temp_k = if temp.is_finite() && temp > 0.0 {
            temp
        } else {
            self.params.tnom.max(1.0)
        };
        let vt = self.thermal_voltage(temp_k).max(1e-12);
        let nvt = (self.params.n.max(1e-12) * vt).max(1e-12);
        let scale = self.hfet_gate_geometry_scale();
        let astar = self.params.mesa_astar.max(0.0);
        let phib = self.params.mesa_phib.max(0.0);
        let texp = (-phib / (K_BOLTZMANN * temp_k)).clamp(-80.0, 80.0).exp();
        let csat = 0.5 * astar * temp_k * temp_k * texp * 2.0 * scale;
        let ggrwl = self.params.hfet_ggr.max(0.0)
            * 2.0
            * scale
            * (self.params.mesa_xchi * (temp_k - self.params.tnom)).exp();

        let expe = (v_int / nvt).clamp(-80.0, 80.0).exp();
        let arg = -v_int * self.params.hfet_del / vt;
        let arg_eff = arg.clamp(-80.0, 80.0);
        let earg = arg_eff.exp();

        let mut g = csat * expe / nvt + ggrwl * earg * (1.0 - arg_eff) + MESA_GMIN;
        let mut i = csat * (expe - 1.0) + ggrwl * v_int * earg + MESA_GMIN * v_int;
        if !i.is_finite() {
            i = 0.0;
        }
        if !g.is_finite() {
            g = MESA_GMIN;
        }
        (i, g)
    }

    #[inline]
    fn node_voltage(voltages: &[Value], node: NodeId) -> Value {
        if node == 0 {
            0.0
        } else {
            voltages.get(node - 1).copied().unwrap_or(0.0)
        }
    }

    pub(crate) fn uses_hfet_legacy_inverse_mode(&self) -> bool {
        self.hfet_legacy_inverse_mode
            && matches!(self.params.channel_model, JfetChannelModel::Hfet1)
            && self.params.hfet_level >= 5
    }

    #[inline]
    pub(crate) fn internal_vds_limited_state(&self) -> Value {
        self.jfet_type.polarity() * self.vds
    }

    #[inline]
    pub(crate) fn internal_branch_state_voltages(&self) -> Option<(Value, Value, Value)> {
        if self.vgs.is_finite() && self.vds.is_finite() {
            let vgs = self.vgs;
            let vds = self.vds;
            let vgd = vgs - vds;
            Some((vgs, vgd, vds))
        } else {
            None
        }
    }

    #[inline]
    pub(crate) fn set_hfet_legacy_inverse_active(&mut self, active: bool) {
        self.hfet_legacy_inverse_active = self.uses_hfet_legacy_inverse_mode() && active;
    }

    #[inline]
    pub(crate) fn set_model_order(&mut self, order: usize) {
        self.model_order = order;
    }

    #[inline]
    pub(crate) fn model_order(&self) -> usize {
        self.model_order
    }

    /// SPICE DEVfetlim voltage limiting helper.
    ///
    /// This bounds per-iteration FET gate-voltage excursions and improves
    /// convergence robustness for stiff nonlinear bias points.
    #[inline]
    fn fetlim(vnew: Value, vold: Value, vto: Value) -> Value {
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

    /// SPICE DEVpnjlim helper for Schottky/PN gate-junction limiting.
    ///
    /// This limits overly aggressive forward-bias updates before `fetlim`
    /// in MESA level-2..4 paths to match ngspice `mesaload.c` behavior.
    #[inline]
    fn pnjlim(vnew: Value, vold: Value, vt: Value, vcrit: Value) -> Value {
        if !vnew.is_finite()
            || !vold.is_finite()
            || !vt.is_finite()
            || !vcrit.is_finite()
            || vt <= 0.0
        {
            return vnew;
        }

        // ngspice DEVpnjlim (devsup.c): forward limiting branch.
        if (vnew > vcrit) && ((vnew - vold).abs() > (vt + vt)) {
            if vold > 0.0 {
                let arg = (vnew - vold) / vt;
                if arg > 0.0 {
                    return vold + vt * (2.0 + (arg - 2.0).ln());
                }
                return vold - vt * (2.0 + (2.0 - arg).ln());
            }
            return vt * (vnew / vt).ln();
        }

        // ngspice DEVpnjlim negative-voltage clamp branch.
        if vnew < 0.0 {
            let arg = if vold > 0.0 {
                -vold - 1.0
            } else {
                2.0 * vold - 1.0
            };
            if vnew < arg {
                return arg;
            }
        }
        vnew
    }

    #[inline]
    fn mesa_gate_csat(&self, temp_k: Value) -> Value {
        const K_BOLTZMANN: Value = 1.380649e-23;

        let scale = self.hfet_gate_geometry_scale();
        let astar = self.params.mesa_astar.max(0.0);
        let phib = self.params.mesa_phib.max(0.0);
        let texp = (-phib / (K_BOLTZMANN * temp_k)).clamp(-80.0, 80.0).exp();
        0.5 * astar * temp_k * temp_k * texp * 2.0 * scale
    }

    #[inline]
    fn mesa_gate_vcrit(&self, temp_k: Value, nvt: Value) -> Value {
        let csat = self.mesa_gate_csat(temp_k);
        if csat > 0.0 && nvt > 0.0 {
            let arg = (nvt / (core::f64::consts::SQRT_2 * csat)).max(1.0);
            nvt * arg.ln()
        } else {
            1.0
        }
    }

    /// Resolve branch voltages used for nonlinear stamping.
    ///
    /// Prefer the device state updated in `update()` (which may include
    /// HFET-specific limiting), and fall back to raw terminal differences if
    /// no state is available yet.
    #[inline]
    fn state_or_raw_branch_voltages(&self, voltages: &[Value]) -> (Value, Value, Value) {
        if self.vgs.is_finite() && self.vds.is_finite() {
            let vgs = self.vgs;
            let vds = self.vds;
            let vgd = vgs - vds;
            return (vgs, vds, vgd);
        }

        let vd = Self::node_voltage(voltages, self.drain);
        let vg = Self::node_voltage(voltages, self.gate);
        let vs = Self::node_voltage(voltages, self.source);

        let vgs = vg - vs;
        let vgd = vg - vd;
        let vds = vgs - vgd;
        (vgs, vds, vgd)
    }

    /// HFET branch-voltage limiting and startup seed.
    ///
    /// Returns `(vgs, vgd)` in external terminal orientation.
    #[inline]
    fn hfet_limited_branch_voltages(&self, vgs_new: Value, vgd_new: Value) -> (Value, Value) {
        let pol = self.jfet_type.polarity();
        if !pol.is_finite() || pol.abs() < 0.5 {
            return (vgs_new, vgd_new);
        }

        if !self.vgs.is_finite() || !self.vds.is_finite() {
            // Match ngspice MODEINITJCT startup for active HFET devices:
            // seed internal vgs/vgd to -1V so Newton lands on the intended
            // low-current branch before regular limiting takes over.
            let seed = -1.0 / pol;
            return (seed, seed);
        }

        let vto_int = pol * self.params.vto;
        let vgs_old_int = pol * self.vgs;
        let vgd_old_int = pol * (self.vgs - self.vds);
        let vgs_new_int = pol * vgs_new;
        let vgd_new_int = pol * vgd_new;
        let vgs_limited_int = Self::fetlim(vgs_new_int, vgs_old_int, vto_int);
        let vgd_limited_int = Self::fetlim(vgd_new_int, vgd_old_int, vto_int);
        (vgs_limited_int / pol, vgd_limited_int / pol)
    }

    #[inline]
    fn compute_operating_terms(
        &self,
        vgs: Value,
        vds: Value,
        vgd: Value,
    ) -> (Value, Value, Value, Value, Value, Value, Value, Value) {
        let mut vds_linear = vds;
        let (mut ids, mut gm, mut gds_raw) = self.calculate(vgs, vds, self.params.tnom);
        if self.hfet_legacy_inverse_active
            && matches!(self.params.channel_model, JfetChannelModel::Hfet1)
            && vds >= 0.0
        {
            match self.params.hfet_level {
                2..=4 => {
                    let (_, temp_source, _) = self.resolved_temperatures(self.params.tnom);
                    let (ids_legacy, gm_legacy, gds_legacy) = self.calculate_mesa_level(
                        vgs,
                        vds,
                        temp_source,
                        self.params.hfet_level,
                        true,
                    );
                    ids = ids_legacy;
                    gm = gm_legacy;
                    gds_raw = gds_legacy;
                }
                5.. => {
                    let (ids_forward, gm_forward, gds_forward) =
                        self.calculate(vgs, vds.abs(), self.params.tnom);
                    ids = -ids_forward;
                    gm = gm_forward;
                    gds_raw = gds_forward;
                    vds_linear = -vds.abs();
                }
                _ => {}
            }
        }
        let (igs, igd, ggs, ggd) = self.gate_junctions(vgs, vgd, self.params.tnom);
        let gds = if gds_raw.is_finite() { gds_raw } else { 0.0 };
        (ids, gm, gds, igs, igd, ggs, ggd, vds_linear)
    }

    /// Calculate classic Shichman-Hodges drain current and conductances.
    fn calculate_shichman_hodges(&self, vgs: Value, vds: Value) -> (Value, Value, Value) {
        let pol = self.jfet_type.polarity();

        // Apply polarity for P-JFET
        let vgs_int = pol * vgs;
        let vds_int = pol * vds;

        let vto = self.params.vto;
        let beta = self.params.beta * self.area * self.m;
        let lambda = self.params.lambda;

        // Effective Vgs (gate-source overdrive)
        let vgst = vgs_int - vto;

        let (ids, gm, gds) = if vgst <= 0.0 {
            // Cutoff region
            (0.0, 0.0, 0.0)
        } else if vds_int < 0.0 {
            // Reverse operation - swap drain and source
            // This handles the symmetric JFET behavior
            let vds_rev = -vds_int;
            let vgs_rev = vgs_int - vds_int;
            let vgst_rev = vgs_rev - vto;

            if vgst_rev <= 0.0 {
                (0.0, 0.0, 0.0)
            } else if vds_rev <= vgst_rev {
                // Linear (reversed)
                // Evaluate forward current from swapped terminals, then map back
                // to the original drain-source orientation.
                let ids_fwd = beta
                    * (2.0 * vgst_rev * vds_rev - vds_rev * vds_rev)
                    * (1.0 + lambda * vds_rev);
                let gm_fwd = 2.0 * beta * vds_rev * (1.0 + lambda * vds_rev);
                let gds_fwd = beta * 2.0 * (vgst_rev - vds_rev) * (1.0 + lambda * vds_rev)
                    + beta * (2.0 * vgst_rev * vds_rev - vds_rev * vds_rev) * lambda;
                (-ids_fwd, -gm_fwd, gm_fwd + gds_fwd)
            } else {
                // Saturation (reversed)
                let ids_fwd = beta * vgst_rev * vgst_rev * (1.0 + lambda * vds_rev);
                let gm_fwd = 2.0 * beta * vgst_rev * (1.0 + lambda * vds_rev);
                let gds_fwd = beta * vgst_rev * vgst_rev * lambda;
                (-ids_fwd, -gm_fwd, gm_fwd + gds_fwd)
            }
        } else if vds_int <= vgst {
            // Linear (triode) region: Vds < Vgs - Vto
            let ids = beta * (2.0 * vgst * vds_int - vds_int * vds_int) * (1.0 + lambda * vds_int);

            // gm = dIds/dVgs = 2 * beta * Vds * (1 + lambda * Vds)
            let gm = 2.0 * beta * vds_int * (1.0 + lambda * vds_int);

            // gds = dIds/dVds
            let gds = beta * 2.0 * (vgst - vds_int) * (1.0 + lambda * vds_int)
                + beta * (2.0 * vgst * vds_int - vds_int * vds_int) * lambda;

            (ids, gm, gds)
        } else {
            // Saturation region: Vds >= Vgs - Vto
            let ids = beta * vgst * vgst * (1.0 + lambda * vds_int);
            let gm = 2.0 * beta * vgst * (1.0 + lambda * vds_int);
            let gds = beta * vgst * vgst * lambda;
            (ids, gm, gds)
        };

        // Apply polarity for output current
        (pol * ids, gm, gds)
    }

    #[inline]
    fn exp_limited(x: Value) -> Value {
        x.clamp(-80.0, 80.0).exp()
    }

    fn mesa_level2_ids(&self, vgs: Value, vds: Value, temp_k: Value, vto: Value) -> Value {
        const Q_ELECTRON: Value = 1.602176634e-19;
        const EPSILONGAAS: Value = 12.244 * 8.85418e-12;

        let p = &self.params;
        let w = self.width.max(1e-12);
        let l = self.length.max(1e-12);
        let vt = self.thermal_voltage(temp_k).max(1e-12);
        let eta = p.eta.abs().max(1e-12);
        let etavth = (eta * vt).max(1e-12);
        let d = p.hfet_di.max(1e-12);
        let nd = p.mesa_nd.max(1e-30);
        let vpo = Q_ELECTRON * nd * d * d / (2.0 * EPSILONGAAS);
        let sigma = p.sigma0.max(0.0)
            / (1.0 + Self::exp_limited((vgs - vto - p.hfet_vsigmat) / p.hfet_vsigma.max(1e-12)));
        let vgt = (vgs - vto) + sigma * vds;

        let mu = (p.hfet_mu + p.mesa_theta * vgt).max(1e-12);
        let vl = (p.hfet_vs.max(1e-12) / mu * l).max(1e-30);
        let beta_inst = 2.0 * EPSILONGAAS * p.hfet_vs.max(1e-12) * p.mesa_zeta.max(1e-12) * w / d;
        let beta = beta_inst / (vpo + 3.0 * vl).max(1e-30);

        let u = vgt / vt - 1.0;
        let t = (p.hfet_delta.max(1e-9).powi(2) + u * u).sqrt();
        let vgte = 0.5 * vt * (2.0 + u + t);
        let b = Self::exp_limited(-vgt / etavth);
        let n0 = (EPSILONGAAS * eta * vt / (Q_ELECTRON * d)).max(1e-30);
        let sqrt1 = if vgte >= vpo {
            0.0
        } else {
            (1.0 - vgte / vpo.max(1e-30)).max(0.0).sqrt()
        };
        let q = (1.0 - sqrt1).max(1e-30);
        let denom = 1.0 / (nd * d * q).max(1e-30) + b / n0;
        let ns = 1.0 / denom.max(1e-30);
        if !ns.is_finite() || ns < 1e-38 {
            return 0.0;
        }

        let rt = p.hfet_rsi.max(0.0) + p.hfet_rdi.max(0.0);
        let gchi0 = Q_ELECTRON * w / l;
        let gchi = gchi0 * mu * ns;
        let gch = gchi / (1.0 + gchi * rt);
        if !gch.is_finite() || gch <= 0.0 {
            return 0.0;
        }

        let a = 2.0 * beta * vgte;
        let f = (1.0 + 2.0 * a * p.hfet_rsi.max(0.0)).sqrt();
        let d_term = 1.0 + a * p.hfet_rsi.max(0.0) + f;
        let e_term = 1.0 + p.mesa_tc * vgte;
        let isata = a * vgte / (d_term * e_term).max(1e-30);
        let isatb0 = Q_ELECTRON * n0 * vt * w / l;
        let isatb = isatb0 * mu * Self::exp_limited(vgt / etavth);
        let isat = if (isata + isatb).abs() > 1e-30 {
            isata * isatb / (isata + isatb)
        } else {
            0.0
        };
        let vsate = (isat / gch).abs().max(1e-30);
        let m = (p.hfet_m + p.mesa_alpha * vgte).max(1e-9);
        let g = (vds / vsate).max(0.0).powf(m);
        let h = (1.0 + g).powf(1.0 / m);
        let ids = gch * vds * (1.0 + p.lambda * vds) / h;
        if ids.is_finite() { ids } else { 0.0 }
    }

    fn mesa_level3_ids(&self, vgs: Value, vds: Value, temp_k: Value, vto: Value) -> Value {
        const Q_ELECTRON: Value = 1.602176634e-19;
        const EPSILONGAAS: Value = 12.244 * 8.85418e-12;

        let p = &self.params;
        let w = self.width.max(1e-12);
        let l = self.length.max(1e-12);
        let vt = self.thermal_voltage(temp_k).max(1e-12);
        let eta = p.eta.abs().max(1e-12);
        let etavth = (eta * vt).max(1e-12);
        let du = p.mesa_du.max(1e-12);
        let th = p.mesa_th.max(1e-12);
        let ndelta = p.mesa_ndelta.max(1e-30);
        let ndu = p.mesa_ndu.max(1e-30);
        let vpou = Q_ELECTRON * ndu * du * du / (2.0 * EPSILONGAAS);
        let vpod = Q_ELECTRON * ndelta * th * (2.0 * du + th) / (2.0 * EPSILONGAAS);
        let vpo = vpou + vpod;

        let sigma = p.sigma0.max(0.0)
            / (1.0 + Self::exp_limited((vgs - vto - p.hfet_vsigmat) / p.hfet_vsigma.max(1e-12)));
        let vgt = (vgs - vto) + sigma * vds;
        let t = vgt / vt - 1.0;
        let q = (p.hfet_delta.max(1e-9).powi(2) + t * t).sqrt();
        let vgte = 0.5 * vt * (2.0 + t + q);
        let a = 2.0 * p.beta.max(1e-30) * vgte;

        let nsa = if vgt > vpod {
            if vgte > vpo {
                ndelta * th + ndu * du
            } else {
                let r = ((vpo - vgte) / vpou.max(1e-30)).max(0.0).sqrt();
                ndelta * th + ndu * du * (1.0 - r)
            }
        } else if vpod - vgte < 0.0 {
            ndelta * th * (1.0 - du / th)
        } else {
            let r = (1.0 + ndu / ndelta * (vpod - vgte) / vpou.max(1e-30))
                .max(0.0)
                .sqrt();
            ndelta * th * (1.0 - du / th * (r - 1.0))
        };

        let b = Self::exp_limited(vgt / etavth);
        let nsb0 = (EPSILONGAAS * eta * vt / (Q_ELECTRON * (du + th))).max(1e-30);
        let nsb = nsb0 * b;
        let ns = if (nsa + nsb).abs() > 1e-30 {
            nsa * nsb / (nsa + nsb)
        } else {
            0.0
        };
        if !ns.is_finite() || ns < 1e-38 {
            return 0.0;
        }

        let rt = p.hfet_rsi.max(0.0) + p.hfet_rdi.max(0.0);
        let gchi0 = Q_ELECTRON * w / l * p.hfet_mu.max(1e-12);
        let gchi = gchi0 * ns;
        let gch = gchi / (1.0 + gchi * rt);
        if !gch.is_finite() || gch <= 0.0 {
            return 0.0;
        }

        let f = (1.0 + 2.0 * a * p.hfet_rsi.max(0.0)).sqrt();
        let d_term = 1.0 + a * p.hfet_rsi.max(0.0) + f;
        let e_term = 1.0 + p.mesa_tc * vgte;
        let isata = a * vgte / (d_term * e_term).max(1e-30);
        let n0 = (EPSILONGAAS * eta * vt / (Q_ELECTRON * du)).max(1e-30);
        let isatb0 = Q_ELECTRON * n0 * vt * w / l;
        let isatb = isatb0 * b;
        let isat = if (isata + isatb).abs() > 1e-30 {
            isata * isatb / (isata + isatb)
        } else {
            0.0
        };
        let vsate = (isat / gch).abs().max(1e-30);
        let m = p.hfet_m.max(1e-9);
        let g = (vds / vsate).max(0.0).powf(m);
        let h = (1.0 + g).powf(1.0 / m);
        let ids = gch * vds * (1.0 + p.lambda * vds) / h;
        if ids.is_finite() { ids } else { 0.0 }
    }

    fn mesa_level4_ids(&self, vgs: Value, vds: Value, temp_k: Value, vto: Value) -> Value {
        const Q_ELECTRON: Value = 1.602176634e-19;

        let p = &self.params;
        let w = self.width.max(1e-12);
        let l = self.length.max(1e-12);
        let vt = self.thermal_voltage(temp_k).max(1e-12);
        let eta = p.eta.abs().max(1e-12);
        let etavth = (eta * vt).max(1e-12);
        let d0 = p.hfet_di.max(1e-12);
        let sigma = p.sigma0.max(0.0)
            / (1.0 + Self::exp_limited((vgs - vto - p.hfet_vsigmat) / p.hfet_vsigma.max(1e-12)));
        let vgt = (vgs - vto) + sigma * vds;
        let u = 0.5 * vgt / vt - 1.0;
        let t = (p.hfet_delta.max(1e-9).powi(2) + u * u).sqrt();
        let vgte = vt * (2.0 + u + t);
        let b = Self::exp_limited(vgt / etavth);
        let n0 = (p.hfet_epsi.max(1e-30) * eta * vt / (2.0 * Q_ELECTRON * d0)).max(1e-30);
        let nsm = 2.0 * n0 * (1.0 + 0.5 * b).ln();
        if !nsm.is_finite() || nsm < 1e-38 {
            return 0.0;
        }

        let c = (nsm / p.hfet_nmax.max(1e-30))
            .max(0.0)
            .powf(p.hfet_gamma.max(1e-9));
        let q = (1.0 + c).powf(1.0 / p.hfet_gamma.max(1e-9));
        let ns = nsm / q;
        let gchi0 = Q_ELECTRON * w * p.hfet_mu.max(1e-12) / l;
        let gchi = gchi0 * ns;
        let rt = p.hfet_rsi.max(0.0) + p.hfet_rdi.max(0.0);
        let gch = gchi / (1.0 + gchi * rt);
        if !gch.is_finite() || gch <= 0.0 {
            return 0.0;
        }

        let gchim = gchi0 * nsm;
        let vl = (p.hfet_vs.max(1e-12) / p.hfet_mu.max(1e-12) * l).max(1e-30);
        let h = (1.0 + 2.0 * gchim * p.hfet_rsi.max(0.0) + vgte * vgte / (vl * vl)).sqrt();
        let p_denom = 1.0 + gchim * p.hfet_rsi.max(0.0) + h;
        let isatm = gchim * vgte / p_denom.max(1e-30);
        let imax = (Q_ELECTRON * p.hfet_nmax.max(1e-12) * p.hfet_vs.max(1e-12) * w).max(1e-30);
        let g = (isatm / imax).max(0.0).powf(p.hfet_gamma.max(1e-9));
        let isat = isatm / (1.0 + g).powf(1.0 / p.hfet_gamma.max(1e-9));
        let vsate = (isat / gch).abs().max(1e-30);
        let d = (vds / vsate).max(0.0).powf(p.hfet_m.max(1e-9));
        let e = (1.0 + d).powf(1.0 / p.hfet_m.max(1e-9));
        let ids = gch * vds * (1.0 + p.lambda * vds) / e;
        if ids.is_finite() { ids } else { 0.0 }
    }

    fn mesa_ids_external(
        &self,
        vgs: Value,
        vds: Value,
        temp: Value,
        level: i32,
        force_inverse: bool,
    ) -> Value {
        let pol = self.jfet_type.polarity();
        let temp_k = if temp.is_finite() && temp > 0.0 {
            temp
        } else {
            self.params.tnom.max(1.0)
        };

        let mut vgs_int = pol * vgs;
        let mut vds_int = pol * vds;
        let mut inverse = false;
        if vds_int < 0.0 {
            vgs_int -= vds_int;
            vds_int = -vds_int;
            inverse = true;
        }
        if force_inverse && !inverse {
            // ngspice MESA legacy inverse latch forces subsequent devices down
            // the inverse-control path (use Vgd as channel control) while
            // keeping |Vds| for the core model equation.
            vgs_int -= vds_int;
            inverse = true;
        }

        let vto = pol * self.params.vto;
        let mut ids_int = match level {
            2 => self.mesa_level2_ids(vgs_int, vds_int, temp_k, vto),
            3 => self.mesa_level3_ids(vgs_int, vds_int, temp_k, vto),
            _ => self.mesa_level4_ids(vgs_int, vds_int, temp_k, vto),
        };
        if inverse {
            ids_int = -ids_int;
        }
        pol * ids_int
    }

    fn calculate_mesa_level(
        &self,
        vgs: Value,
        vds: Value,
        temp: Value,
        level: i32,
        force_inverse: bool,
    ) -> (Value, Value, Value) {
        let ids = self.mesa_ids_external(vgs, vds, temp, level, force_inverse);

        let dvgs = (1e-8_f64).max(1e-6 * (1.0 + vgs.abs()));
        let dvds = (1e-8_f64).max(1e-6 * (1.0 + vds.abs()));
        let gm = (self.mesa_ids_external(vgs + dvgs, vds, temp, level, force_inverse)
            - self.mesa_ids_external(vgs - dvgs, vds, temp, level, force_inverse))
            / (2.0 * dvgs);
        let gds = (self.mesa_ids_external(vgs, vds + dvds, temp, level, force_inverse)
            - self.mesa_ids_external(vgs, vds - dvds, temp, level, force_inverse))
            / (2.0 * dvds);

        (
            if ids.is_finite() { ids } else { 0.0 },
            if gm.is_finite() { gm } else { 0.0 },
            if gds.is_finite() { gds } else { 0.0 },
        )
    }

    fn mesa_level2_capacitances_mode(
        &self,
        vgs: Value,
        vgd: Value,
        temp: Value,
        force_inverse: bool,
    ) -> (Value, Value) {
        const Q_ELECTRON: Value = 1.602176634e-19;
        const EPSILONGAAS: Value = 12.244 * 8.85418e-12;

        let pol = self.jfet_type.polarity();
        let p = &self.params;
        let w = self.width.max(1e-12);
        let l = self.length.max(1e-12);
        let temp_k = if temp.is_finite() && temp > 0.0 {
            temp
        } else {
            p.tnom.max(1.0)
        };
        let vt = self.thermal_voltage(temp_k).max(1e-12);
        let eta = p.eta.abs().max(1e-12);
        let etavth = (eta * vt).max(1e-12);

        let vgs_int = pol * vgs;
        let vgd_int = pol * vgd;
        let mut vds_int = vgs_int - vgd_int;
        let mut vgsch = vgs_int;
        let mut inverse = false;
        if vds_int < 0.0 {
            inverse = true;
            vds_int = -vds_int;
            vgsch = vgd_int;
        }
        if force_inverse && !inverse {
            inverse = true;
            vgsch = vgd_int;
        }

        let vto = pol * p.vto;
        let vgt0 = vgsch - vto;
        let sigma = p.sigma0.max(0.0)
            / (1.0 + Self::exp_limited((vgt0 - p.hfet_vsigmat) / p.hfet_vsigma.max(1e-12)));
        let vgt = vgt0 + sigma * vds_int;

        let mu = (p.hfet_mu + p.mesa_theta * vgt).max(1e-12);
        let d = p.hfet_di.max(1e-12);
        let nd = p.mesa_nd.max(1e-30);
        let vpo = Q_ELECTRON * nd * d * d / (2.0 * EPSILONGAAS);
        let vl = (p.hfet_vs.max(1e-12) / mu * l).max(1e-30);
        let beta_inst = 2.0 * EPSILONGAAS * p.hfet_vs.max(1e-12) * p.mesa_zeta.max(1e-12) * w / d;
        let beta = beta_inst / (vpo + 3.0 * vl).max(1e-30);

        let u = vgt / vt - 1.0;
        let t = (p.hfet_delta.max(1e-9).powi(2) + u * u).sqrt();
        let vgte = 0.5 * vt * (2.0 + u + t);
        let b = Self::exp_limited(-vgt / etavth);
        let n0 = (EPSILONGAAS * eta * vt / (Q_ELECTRON * d)).max(1e-30);
        let sqrt1 = if vgte >= vpo {
            0.0
        } else {
            (1.0 - vgte / vpo.max(1e-30)).max(0.0).sqrt()
        };
        let q = (1.0 - sqrt1).max(1e-30);
        let denom = 1.0 / (nd * d * q).max(1e-30) + b / n0;
        let ns = 1.0 / denom.max(1e-30);
        if !ns.is_finite() || ns < 1e-38 {
            let cf = 0.5 * EPSILONGAAS * w;
            return (cf, cf);
        }

        let rt = p.hfet_rsi.max(0.0) + p.hfet_rdi.max(0.0);
        let gchi0 = Q_ELECTRON * w / l;
        let gchi = gchi0 * mu * ns;
        let gch = gchi / (1.0 + gchi * rt);
        if !gch.is_finite() || gch <= 0.0 {
            let cf = 0.5 * EPSILONGAAS * w;
            return (cf, cf);
        }

        let a = 2.0 * beta * vgte;
        let f = (1.0 + 2.0 * a * p.hfet_rsi.max(0.0)).sqrt();
        let d_term = 1.0 + a * p.hfet_rsi.max(0.0) + f;
        let e_term = 1.0 + p.mesa_tc * vgte;
        let isata = a * vgte / (d_term * e_term).max(1e-30);
        let isatb0 = Q_ELECTRON * n0 * vt * w / l;
        let isatb = isatb0 * mu * Self::exp_limited(vgt / etavth);
        let isat = if (isata + isatb).abs() > 1e-30 {
            isata * isatb / (isata + isatb)
        } else {
            0.0
        };
        let vsate = (isat / gch).abs().max(1e-30);
        let vdse = vds_int
            * (1.0 + (vds_int / vsate).max(0.0).powf(p.hfet_mc.max(1e-9)))
                .powf(-1.0 / p.hfet_mc.max(1e-9));

        let cf = 0.5 * EPSILONGAAS * w;
        let temp_sqrt = if vgt > vpo {
            0.0
        } else {
            (1.0 - vgt / vpo.max(1e-30)).max(0.0).sqrt()
        };
        let cgc = w * l * EPSILONGAAS / ((temp_sqrt + b).max(1e-30)) / d;
        let c1_denom = (2.0 * vsate - vdse).max(1e-30);
        let c1 = ((vsate - vdse) / c1_denom).powi(2);
        let mut capgs = cf + (2.0 / 3.0) * cgc * (1.0 - c1);
        let c2 = (vsate / c1_denom).powi(2);
        let mut capgd = cf + (2.0 / 3.0) * cgc * (1.0 - c2);
        if inverse {
            std::mem::swap(&mut capgs, &mut capgd);
        }
        (
            if capgs.is_finite() {
                capgs.max(1e-18)
            } else {
                cf.max(1e-18)
            },
            if capgd.is_finite() {
                capgd.max(1e-18)
            } else {
                cf.max(1e-18)
            },
        )
    }

    /// Calculate HFET1-compatible channel current and Jacobian.
    ///
    /// Equations are ported from ngspice HFET1 (`hfetload.c` / `hfettemp.c`)
    /// for DC small-signal terms (`Ids`, `gm`, `gds`).
    fn calculate_hfet1(&self, vgs: Value, vds: Value, temp: Value) -> (Value, Value, Value) {
        const Q_ELECTRON: Value = 1.602176634e-19;
        let pol = self.jfet_type.polarity();
        let p = &self.params;

        let vgs_int = pol * vgs;
        let mut vds_int = pol * vds;
        let mut inverse = false;
        if vds_int < 0.0 {
            // ngspice HFET level-5 evaluates reverse Vds by flipping channel
            // current sign while keeping the controlling gate branch on Vgs.
            vds_int = -vds_int;
            inverse = true;
        }

        let temp_k = if temp.is_finite() && temp > 0.0 {
            temp
        } else {
            p.tnom.max(1.0)
        };
        let vt = self.thermal_voltage(temp_k).max(1e-6);
        let eta = p.eta.abs().max(1e-9);
        let etavth = (eta * vt).max(1e-12);

        let mu = p.hfet_mu.max(1e-12);
        let vs = p.hfet_vs.max(1e-12);
        let l = self.length.max(1e-12);
        let w = self.width.max(1e-12);
        let di = p.hfet_di.max(1e-12);
        let deltad = p.hfet_deltad.max(0.0);
        let nmax = p.hfet_nmax.max(1e-12);
        let gamma = p.hfet_gamma.max(1e-9);
        let m = p.hfet_m.max(1e-9);
        let sigma0 = p.sigma0.max(0.0);
        let vsigma = p.hfet_vsigma.max(1e-12);
        let vsigmat = p.hfet_vsigmat;
        let rsi = p.hfet_rsi.max(0.0);
        let rdi = p.hfet_rdi.max(0.0);
        let rt = rsi + rdi;

        let vto = pol * p.vto;

        let n0 = p.hfet_epsi.max(1e-30) * eta * vt / (2.0 * Q_ELECTRON * (di + deltad).max(1e-30));
        let gchi0 = Q_ELECTRON * w * mu / l;
        let imax = (Q_ELECTRON * nmax * vs * w).max(1e-30);
        let vl = (vs / mu * l).max(1e-30);

        let vgt0 = vgs_int - vto;
        let s = ((vgt0 - vsigmat) / vsigma).clamp(-80.0, 80.0).exp();
        let sigma = sigma0 / (1.0 + s);
        let vgt = vgt0 + sigma * vds_int;
        let u = 0.5 * vgt / vt - 1.0;
        let delta = p.hfet_delta.max(1e-9);
        let t = (delta * delta + u * u).sqrt();
        let vgte = vt * (2.0 + u + t);
        let b = (vgt / etavth).clamp(-80.0, 80.0).exp();
        let nsm = 2.0 * n0 * (1.0 + 0.5 * b).ln();
        if !nsm.is_finite() || nsm < 1e-38 {
            return (0.0, 0.0, 0.0);
        }

        let c = (nsm / nmax).max(0.0).powf(gamma);
        let q = (1.0 + c).powf(1.0 / gamma);
        let ns = nsm / q;
        let gchi = gchi0 * ns;
        let gch = gchi / (1.0 + gchi * rt);
        if !gch.is_finite() || gch <= 0.0 {
            return (0.0, 0.0, 0.0);
        }

        let gchim = gchi0 * nsm;
        let h = (1.0 + 2.0 * gchim * rsi + vgte * vgte / (vl * vl)).sqrt();
        let p_denom = 1.0 + gchim * rsi + h;
        if !p_denom.is_finite() || p_denom <= 0.0 {
            return (0.0, 0.0, 0.0);
        }

        let isatm = gchim * vgte / p_denom;
        let g = (isatm / imax).max(0.0).powf(gamma);
        let isat = isatm / (1.0 + g).powf(1.0 / gamma);
        if !isat.is_finite() {
            return (0.0, 0.0, 0.0);
        }

        let vsate = (isat / gch).abs().max(1e-30);
        let d = (vds_int / vsate).max(0.0).powf(m);
        let e = (1.0 + d).powf(1.0 / m);
        let delidgch = vds_int * (1.0 + p.lambda * vds_int) / e;
        let ids_fwd = gch * delidgch;

        let delidvsate = ids_fwd * d / (vsate * (1.0 + d));
        let dmd = if vds_int <= 0.0 {
            0.0
        } else {
            (vds_int / vsate).powf(m - 1.0)
        };
        let delidvds =
            gch * (1.0 + 2.0 * p.lambda * vds_int) / e - ids_fwd * dmd / (vsate * (1.0 + d));

        let a = 1.0 + gchi * rt;
        let delgchgchi = 1.0 / (a * a);
        let delgchins = gchi0;
        let delnsnsm = ns / nsm * (1.0 - c / (1.0 + c));
        let delvgtevgt = 0.5 * (1.0 + u / t.max(1e-30));
        let delnsmvgt = n0 / etavth / (1.0 / b + 0.5);
        let delvsateisat = 1.0 / gch;
        let delisatisatm = isat / isatm.max(1e-30) * (1.0 - g / (1.0 + g));
        let delisatmvgte =
            gchim * (p_denom - vgte * vgte / (vl * vl * h.max(1e-30))) / (p_denom * p_denom);
        let delvsategch = -vsate / gch;
        let delisatmgchim =
            vgte * (p_denom - gchim * rsi * (1.0 + 1.0 / h.max(1e-30))) / (p_denom * p_denom);
        let delvgtvgs = 1.0 - vds_int * sigma0 / vsigma * s / ((1.0 + s) * (1.0 + s));
        let p_chain = delgchgchi * delgchins * delnsnsm * delnsmvgt;
        let delvsatevgt = delvsateisat
            * delisatisatm
            * (delisatmvgte * delvgtevgt + delisatmgchim * gchi0 * delnsmvgt)
            + delvsategch * p_chain;
        let g_total = delidgch * p_chain + delidvsate * delvsatevgt;
        let gm_fwd = g_total * delvgtvgs;
        let gds_fwd = delidvds + g_total * sigma;
        let (mut ids, mut gm, mut gds) = if inverse {
            (-ids_fwd, gm_fwd, gds_fwd)
        } else {
            (ids_fwd, gm_fwd, gds_fwd)
        };

        if !ids.is_finite() {
            ids = 0.0;
        }
        if !gm.is_finite() {
            gm = 0.0;
        }
        if !gds.is_finite() {
            gds = 0.0;
        }

        (pol * ids, gm, gds)
    }

    /// Calculate drain current and conductances
    ///
    /// Returns (Ids, gm, gds) where:
    /// - Ids: drain-source current
    /// - gm: transconductance dIds/dVgs
    /// - gds: output conductance dIds/dVds
    pub fn calculate(&self, vgs: Value, vds: Value, temp: Value) -> (Value, Value, Value) {
        let (temp_common, temp_source, _) = self.resolved_temperatures(temp);
        match self.params.channel_model {
            JfetChannelModel::ShichmanHodges => self.calculate_shichman_hodges(vgs, vds),
            JfetChannelModel::Hfet1 => match self.params.hfet_level {
                2 => self.calculate_mesa_level(vgs, vds, temp_source, 2, false),
                3 => self.calculate_mesa_level(vgs, vds, temp_source, 3, false),
                4 => self.calculate_mesa_level(vgs, vds, temp_source, 4, false),
                _ => self.calculate_hfet1(vgs, vds, temp_common),
            },
        }
    }

    fn mesa_level3_capacitances_mode(
        &self,
        vgs: Value,
        vgd: Value,
        temp: Value,
        force_inverse: bool,
    ) -> (Value, Value) {
        const Q_ELECTRON: Value = 1.602176634e-19;
        const EPSILONGAAS: Value = 12.244 * 8.85418e-12;

        let pol = self.jfet_type.polarity();
        let p = &self.params;
        let w = self.width.max(1e-12);
        let l = self.length.max(1e-12);
        let temp_k = if temp.is_finite() && temp > 0.0 {
            temp
        } else {
            p.tnom.max(1.0)
        };
        let vt = self.thermal_voltage(temp_k).max(1e-12);
        let eta = p.eta.abs().max(1e-12);
        let etavth = (eta * vt).max(1e-12);

        let vgs_int = pol * vgs;
        let vgd_int = pol * vgd;
        let mut vds_int = vgs_int - vgd_int;
        let mut vgsch = vgs_int;
        let mut inverse = false;
        if vds_int < 0.0 {
            inverse = true;
            vds_int = -vds_int;
            vgsch = vgd_int;
        }
        if force_inverse && !inverse {
            inverse = true;
            vgsch = vgd_int;
        }

        let vto = pol * p.vto;
        let vgt0 = vgsch - vto;
        let sigma = p.sigma0.max(0.0)
            / (1.0 + Self::exp_limited((vgt0 - p.hfet_vsigmat) / p.hfet_vsigma.max(1e-12)));
        let vgt = vgt0 + sigma * vds_int;
        let t = vgt / vt - 1.0;
        let q_term = (p.hfet_delta.max(1e-9).powi(2) + t * t).sqrt();
        let vgte = 0.5 * vt * (2.0 + t + q_term);
        let a = 2.0 * p.beta.max(1e-30) * vgte;

        let du = p.mesa_du.max(1e-12);
        let th = p.mesa_th.max(1e-12);
        let ndelta = p.mesa_ndelta.max(1e-30);
        let ndu = p.mesa_ndu.max(1e-30);
        let vpou = Q_ELECTRON * ndu * du * du / (2.0 * EPSILONGAAS);
        let vpod = Q_ELECTRON * ndelta * th * (2.0 * du + th) / (2.0 * EPSILONGAAS);
        let vpo = vpou + vpod;

        let (nsa, ca) = if vgt > vpod {
            if vgte > vpo {
                (ndelta * th + ndu * du, EPSILONGAAS / du)
            } else {
                let r = ((vpo - vgte) / vpou.max(1e-30)).max(0.0).sqrt().max(1e-30);
                (ndelta * th + ndu * du * (1.0 - r), EPSILONGAAS / (du * r))
            }
        } else if vpod - vgte < 0.0 {
            (ndelta * th * (1.0 - du / th), EPSILONGAAS / du)
        } else {
            let r = (1.0 + ndu / ndelta * (vpod - vgte) / vpou.max(1e-30))
                .max(0.0)
                .sqrt()
                .max(1e-30);
            (
                ndelta * th * (1.0 - du / th * (r - 1.0)),
                EPSILONGAAS / (du * r),
            )
        };

        let b = Self::exp_limited(vgt / etavth);
        let cb = EPSILONGAAS / (du + th).max(1e-30) * b;
        let nsb0 = (EPSILONGAAS * eta * vt / (Q_ELECTRON * (du + th).max(1e-30))).max(1e-30);
        let nsb = nsb0 * b;
        let ns = if (nsa + nsb).abs() > 1e-30 {
            nsa * nsb / (nsa + nsb)
        } else {
            0.0
        };

        let cf = 0.5 * EPSILONGAAS * w;
        if !ns.is_finite()
            || ns < 1e-38
            || !ca.is_finite()
            || !cb.is_finite()
            || ca <= 0.0
            || cb <= 0.0
        {
            return (cf.max(1e-18), cf.max(1e-18));
        }

        let rt = p.hfet_rsi.max(0.0) + p.hfet_rdi.max(0.0);
        let gchi0 = Q_ELECTRON * w * p.hfet_mu.max(1e-12) / l;
        let gchi = gchi0 * ns;
        let gch = gchi / (1.0 + gchi * rt);
        if !gch.is_finite() || gch <= 0.0 {
            return (cf.max(1e-18), cf.max(1e-18));
        }

        let f = (1.0 + 2.0 * a * p.hfet_rsi.max(0.0)).sqrt();
        let d_term = 1.0 + a * p.hfet_rsi.max(0.0) + f;
        let e_term = 1.0 + p.mesa_tc * vgte;
        let isata = a * vgte / (d_term * e_term).max(1e-30);
        let n0 = (EPSILONGAAS * eta * vt / (Q_ELECTRON * du)).max(1e-30);
        let isatb0 = Q_ELECTRON * n0 * vt * w / l;
        let isatb = isatb0 * b;
        let isat = if (isata + isatb).abs() > 1e-30 {
            isata * isatb / (isata + isatb)
        } else {
            0.0
        };
        let vsate = (isat / gch).abs().max(1e-30);
        let vdse = vds_int
            * (1.0 + (vds_int / vsate).max(0.0).powf(p.hfet_mc.max(1e-9)))
                .powf(-1.0 / p.hfet_mc.max(1e-9));
        let cgc = w * l * ca * cb / (ca + cb).max(1e-30);

        let c1_denom = (2.0 * vsate - vdse).max(1e-30);
        let c1 = ((vsate - vdse) / c1_denom).powi(2);
        let mut capgs = cf + (2.0 / 3.0) * cgc * (1.0 - c1);
        let c2 = (vsate / c1_denom).powi(2);
        let mut capgd = cf + (2.0 / 3.0) * cgc * (1.0 - c2);
        if inverse {
            std::mem::swap(&mut capgs, &mut capgd);
        }
        (
            if capgs.is_finite() {
                capgs.max(1e-18)
            } else {
                cf.max(1e-18)
            },
            if capgd.is_finite() {
                capgd.max(1e-18)
            } else {
                cf.max(1e-18)
            },
        )
    }

    fn mesa_level4_capacitances_mode(
        &self,
        vgs: Value,
        vgd: Value,
        temp: Value,
        force_inverse: bool,
    ) -> (Value, Value) {
        const Q_ELECTRON: Value = 1.602176634e-19;

        let pol = self.jfet_type.polarity();
        let p = &self.params;
        let w = self.width.max(1e-12);
        let l = self.length.max(1e-12);
        let temp_k = if temp.is_finite() && temp > 0.0 {
            temp
        } else {
            p.tnom.max(1.0)
        };
        let vt = self.thermal_voltage(temp_k).max(1e-12);
        let eta = p.eta.abs().max(1e-12);
        let etavth = (eta * vt).max(1e-12);

        let vgs_int = pol * vgs;
        let vgd_int = pol * vgd;
        let mut vds_int = vgs_int - vgd_int;
        let mut vgsch = vgs_int;
        let mut inverse = false;
        if vds_int < 0.0 {
            inverse = true;
            vds_int = -vds_int;
            vgsch = vgd_int;
        }
        if force_inverse && !inverse {
            inverse = true;
            vgsch = vgd_int;
        }

        let vto = pol * p.vto;
        let vgt0 = vgsch - vto;
        let sigma = p.sigma0.max(0.0)
            / (1.0 + Self::exp_limited((vgt0 - p.hfet_vsigmat) / p.hfet_vsigma.max(1e-12)));
        let vgt = vgt0 + sigma * vds_int;
        let u = 0.5 * vgt / vt - 1.0;
        let t = (p.hfet_delta.max(1e-9).powi(2) + u * u).sqrt();
        let vgte = vt * (2.0 + u + t);
        let b = Self::exp_limited(vgt / etavth);

        let epsi = p.hfet_epsi.max(1e-30);
        let d0 = p.hfet_di.max(1e-12);
        let n0 = (epsi * eta * vt / (2.0 * Q_ELECTRON * d0)).max(1e-30);
        let nsm = 2.0 * n0 * (1.0 + 0.5 * b).ln();
        let cf = 0.5 * epsi * w;
        if !nsm.is_finite() || nsm < 1e-38 {
            return (cf.max(1e-18), cf.max(1e-18));
        }

        let gamma = p.hfet_gamma.max(1e-9);
        let comp = (nsm / p.hfet_nmax.max(1e-30)).max(0.0).powf(gamma);
        let ns = nsm / (1.0 + comp).powf(1.0 / gamma);
        let mu = p.hfet_mu.max(1e-12);
        let gchi0 = Q_ELECTRON * w * mu / l;
        let gchi = gchi0 * ns;
        let rt = p.hfet_rsi.max(0.0) + p.hfet_rdi.max(0.0);
        let gch = gchi / (1.0 + gchi * rt);
        if !gch.is_finite() || gch <= 0.0 {
            return (cf.max(1e-18), cf.max(1e-18));
        }

        let gchim = gchi0 * nsm;
        let vl = (p.hfet_vs.max(1e-12) / mu * l).max(1e-30);
        let h = (1.0 + 2.0 * gchim * p.hfet_rsi.max(0.0) + vgte * vgte / (vl * vl)).sqrt();
        let p_denom = 1.0 + gchim * p.hfet_rsi.max(0.0) + h;
        let isatm = gchim * vgte / p_denom.max(1e-30);
        let imax = (Q_ELECTRON * p.hfet_nmax.max(1e-12) * p.hfet_vs.max(1e-12) * w).max(1e-30);
        let g = (isatm / imax).max(0.0).powf(gamma);
        let isat = isatm / (1.0 + g).powf(1.0 / gamma);
        let vsate = (isat / gch).abs().max(1e-30);
        let vdse = vds_int
            * (1.0 + (vds_int / vsate).max(0.0).powf(p.hfet_mc.max(1e-9)))
                .powf(-1.0 / p.hfet_mc.max(1e-9));

        let cas = p.mesa_cas.max(1e-12);
        let cbs = p.mesa_cbs.max(1e-12);
        let cgcm_denom = d0 / (cas * epsi).max(1e-30)
            + etavth / (cbs * Q_ELECTRON * n0).max(1e-30) * Self::exp_limited(-vgt / etavth);
        let cgcm = 1.0 / cgcm_denom.max(1e-30);
        let cgc = w * l * cgcm / (1.0 + comp).powf(1.0 + 1.0 / gamma);

        let c1_denom = (2.0 * vsate - vdse).max(1e-30);
        let c1 = ((vsate - vdse) / c1_denom).powi(2);
        let mut capgs = cf + (2.0 / 3.0) * cgc * (1.0 - c1);
        let c2 = (vsate / c1_denom).powi(2);
        let mut capgd = cf + (2.0 / 3.0) * cgc * (1.0 - c2);
        if inverse {
            std::mem::swap(&mut capgs, &mut capgd);
        }
        (
            if capgs.is_finite() {
                capgs.max(1e-18)
            } else {
                cf.max(1e-18)
            },
            if capgd.is_finite() {
                capgd.max(1e-18)
            } else {
                cf.max(1e-18)
            },
        )
    }

    /// Calculate gate junction current (reverse-biased diodes)
    ///
    /// Returns (Igs, Igd) - gate-source and gate-drain junction currents
    pub fn gate_current(&self, vgs: Value, vgd: Value, temp: Value) -> (Value, Value) {
        let (igs, igd, _, _) = self.gate_junctions(vgs, vgd, temp);
        (igs, igd)
    }

    /// Return flicker-noise coefficients normalized by active area.
    pub fn flicker_noise_coefficients(&self) -> Option<(Value, Value, Value)> {
        if self.params.kf <= 0.0 || !self.params.kf.is_finite() {
            return None;
        }

        let area = (self.width.max(1e-18) * self.length.max(1e-18)).max(1e-30);
        Some((
            self.params.kf / area,
            self.params.af.max(1e-12),
            self.params.ef.max(1e-12),
        ))
    }

    /// Calculate gate junction currents and conductances.
    ///
    /// Returned currents are defined in external terminal orientation:
    /// - `igs`: current from gate to source
    /// - `igd`: current from gate to drain
    fn gate_junctions(&self, vgs: Value, vgd: Value, temp: Value) -> (Value, Value, Value, Value) {
        let (temp_common, temp_source, temp_drain) = self.resolved_temperatures(temp);
        let pol = self.jfet_type.polarity();
        let vgs_int = pol * vgs;
        let vgd_int = pol * vgd;

        if matches!(self.params.channel_model, JfetChannelModel::Hfet1) {
            let (igs_int, ggs, igd_int, ggd) =
                if self.params.hfet_level >= 2 && self.params.hfet_level < 5 {
                    let (igs_int, ggs) = self.mesa_gate_branch(vgs_int, temp_source);
                    let (igd_int, ggd) = self.mesa_gate_branch(vgd_int, temp_drain);
                    (igs_int, ggs, igd_int, ggd)
                } else {
                    let (igs_int, ggs) = self.hfet_gate_branch(
                        vgs_int,
                        temp_source,
                        self.params.hfet_js1s,
                        self.params.hfet_js2s,
                        self.params.hfet_m1s,
                        self.params.hfet_m2s,
                        self.params.hfet_rgs,
                    );
                    let (igd_int, ggd) = self.hfet_gate_branch(
                        vgd_int,
                        temp_drain,
                        self.params.hfet_js1d,
                        self.params.hfet_js2d,
                        self.params.hfet_m1d,
                        self.params.hfet_m2d,
                        self.params.hfet_rgd,
                    );
                    (igs_int, ggs, igd_int, ggd)
                };
            return (pol * igs_int, pol * igd_int, ggs, ggd);
        }

        let igs = pol * self.junction_diode_current(vgs_int, temp_common);
        let igd = pol * self.junction_diode_current(vgd_int, temp_common);
        let ggs = self.junction_diode_conductance(vgs_int, temp_common);
        let ggd = self.junction_diode_conductance(vgd_int, temp_common);
        (igs, igd, ggs, ggd)
    }

    fn hfet1_capacitances(&self, vgs: Value, vgd: Value, temp: Value) -> (Value, Value) {
        const Q_ELECTRON: Value = 1.602176634e-19;

        let pol = self.jfet_type.polarity();
        let p = &self.params;
        let w = self.width.max(1e-12);
        let l = self.length.max(1e-12);
        let epsi = p.hfet_epsi.max(1e-30);
        let cf = 0.5 * epsi * w;

        let vgs_int = pol * vgs;
        let vgd_int = pol * vgd;
        let vgs_eff = vgs_int;
        let mut vds_int = vgs_int - vgd_int;
        let mut inverse = false;
        if vds_int < 0.0 {
            // Match ngspice HFET1 load path: evaluate with |Vds| while keeping
            // channel control on Vgs, then swap terminal caps in inverse mode.
            vds_int = -vds_int;
            inverse = true;
        }

        let temp_k = if temp.is_finite() && temp > 0.0 {
            temp
        } else {
            p.tnom.max(1.0)
        };
        let vt = self.thermal_voltage(temp_k).max(1e-12);
        let eta = p.eta.abs().max(1e-9);
        let etavth = (eta * vt).max(1e-12);

        let mu = p.hfet_mu.max(1e-12);
        let vs = p.hfet_vs.max(1e-12);
        let di = p.hfet_di.max(1e-12);
        let deltad = p.hfet_deltad.max(0.0);
        let nmax = p.hfet_nmax.max(1e-12);
        let gamma = p.hfet_gamma.max(1e-9);
        let sigma0 = p.sigma0.max(0.0);
        let vsigma = p.hfet_vsigma.max(1e-12);
        let vsigmat = p.hfet_vsigmat;
        let rsi = p.hfet_rsi.max(0.0);
        let rdi = p.hfet_rdi.max(0.0);
        let rt = rsi + rdi;

        let vto = pol * p.vto;
        let n0 = epsi * eta * vt / (2.0 * Q_ELECTRON * (di + deltad).max(1e-30));
        let gchi0 = Q_ELECTRON * w * mu / l;
        let imax = (Q_ELECTRON * nmax * vs * w).max(1e-30);
        let vl = (vs / mu * l).max(1e-30);

        let vgt0 = vgs_eff - vto;
        let s = ((vgt0 - vsigmat) / vsigma).clamp(-80.0, 80.0).exp();
        let sigma = sigma0 / (1.0 + s);
        let vgt = vgt0 + sigma * vds_int;
        let u = 0.5 * vgt / vt - 1.0;
        let t = (p.hfet_delta.max(1e-9).powi(2) + u * u).sqrt();
        let vgte = vt * (2.0 + u + t);
        let b = (vgt / etavth).clamp(-80.0, 80.0).exp();
        let nsm = 2.0 * n0 * (1.0 + 0.5 * b).ln();
        if !nsm.is_finite() || nsm < 1.0e-38 {
            return (cf.max(1e-18), cf.max(1e-18));
        }

        let c = (nsm / nmax).max(0.0).powf(gamma);
        let q = (1.0 + c).powf(1.0 / gamma);
        let ns = nsm / q;
        let gchi = gchi0 * ns;
        let gch = gchi / (1.0 + gchi * rt);
        if !gch.is_finite() || gch <= 0.0 {
            return (cf.max(1e-18), cf.max(1e-18));
        }

        let gchim = gchi0 * nsm;
        let h = (1.0 + 2.0 * gchim * rsi + vgte * vgte / (vl * vl)).sqrt();
        let p_denom = 1.0 + gchim * rsi + h;
        if !p_denom.is_finite() || p_denom <= 0.0 {
            return (cf.max(1e-18), cf.max(1e-18));
        }

        let isatm = gchim * vgte / p_denom;
        let g = (isatm / imax).max(0.0).powf(gamma);
        let isat = isatm / (1.0 + g).powf(1.0 / gamma);
        if !isat.is_finite() {
            return (cf.max(1e-18), cf.max(1e-18));
        }
        let vsate = (isat / gch).abs().max(1e-30);

        let delnsnsm = ns / nsm * (1.0 - c / (1.0 + c));
        let delnsmvgt = n0 / etavth / (1.0 / b + 0.5);
        let delvgtvgs = 1.0 - vds_int * sigma0 / vsigma * s / ((1.0 + s) * (1.0 + s));

        let eta1 = p.hfet_eta1.max(1e-9);
        let d1 = p.hfet_d1.max(1e-12);
        let temp_eta1 = (eta1 * vt).max(1e-18);
        let vt1 = if p.hfet_vt1.is_finite() {
            p.hfet_vt1
        } else {
            vto + Q_ELECTRON * nmax * di / epsi
        };
        let cg1 = 1.0
            / (d1 / epsi + temp_eta1 * Self::exp_limited(-(vgs_eff - vt1) / temp_eta1)).max(1e-30);
        let mut cgc = w * l * (Q_ELECTRON * delnsnsm * delnsmvgt * delvgtvgs + cg1);
        if !cgc.is_finite() || cgc < 0.0 {
            cgc = 0.0;
        }

        let mc = p.hfet_mc.max(1e-9);
        let vdse = vds_int * (1.0 + (vds_int / vsate).max(0.0).powf(mc)).powf(-1.0 / mc);
        let c1_denom = (2.0 * vsate - vdse).max(1e-30);
        let a_gs = ((vsate - vdse) / c1_denom).powi(2);
        let pcap = p.hfet_p + (1.0 - p.hfet_p) * Self::exp_limited(-vds_int / vsate);
        let mut capgs = cf + (4.0 / 3.0) * cgc * (1.0 - a_gs) / (1.0 + pcap);

        let a_gd = (vsate / c1_denom).powi(2);
        let mut capgd = cf + (4.0 / 3.0) * pcap * cgc * (1.0 - a_gd) / (1.0 + pcap);

        if inverse {
            std::mem::swap(&mut capgs, &mut capgd);
        }

        (
            if capgs.is_finite() {
                capgs.max(1e-18)
            } else {
                cf.max(1e-18)
            },
            if capgd.is_finite() {
                capgd.max(1e-18)
            } else {
                cf.max(1e-18)
            },
        )
    }

    /// Transient capacitances for the active JFET/MESFET model.
    ///
    /// Inputs are external branch voltages (`vgs = Vg - Vs`, `vgd = Vg - Vd`).
    pub fn transient_capacitances(&self, vgs: Value, vgd: Value, temp: Value) -> (Value, Value) {
        let (temp_common, temp_source, _) = self.resolved_temperatures(temp);
        let (mut cgs, mut cgd) = match self.params.channel_model {
            JfetChannelModel::ShichmanHodges => {
                let pol = self.jfet_type.polarity();
                self.capacitances(pol * vgs, pol * vgd)
            }
            JfetChannelModel::Hfet1 => match self.params.hfet_level {
                2..=4 => {
                    let pol = self.jfet_type.polarity();
                    let vds_int = pol * (vgs - vgd);
                    let local_inverse = vds_int < 0.0;
                    let force_inverse = self.hfet_legacy_inverse_active && !local_inverse;
                    match self.params.hfet_level {
                        2 => {
                            self.mesa_level2_capacitances_mode(vgs, vgd, temp_source, force_inverse)
                        }
                        3 => {
                            self.mesa_level3_capacitances_mode(vgs, vgd, temp_source, force_inverse)
                        }
                        _ => {
                            self.mesa_level4_capacitances_mode(vgs, vgd, temp_source, force_inverse)
                        }
                    }
                }
                _ => self.hfet1_capacitances(vgs, vgd, temp_common),
            },
        };
        if self.hfet_legacy_inverse_active
            && matches!(self.params.channel_model, JfetChannelModel::Hfet1)
            && self.params.hfet_level >= 5
        {
            // ngspice HFET keeps a legacy inverse latch across instances and applies
            // one effective cap swap when that latch is active. Local inverse mode
            // (vds_int < 0) is already handled inside hfet*_capacitances(), so only
            // swap here for forward-oriented instances to avoid a double swap.
            let pol = self.jfet_type.polarity();
            let vds_int = pol * (vgs - vgd);
            let local_inverse = vds_int < 0.0;
            if !local_inverse {
                std::mem::swap(&mut cgs, &mut cgd);
            }
        }
        (cgs, cgd)
    }

    /// Calculate junction capacitances
    ///
    /// Returns (Cgs, Cgd) - gate-source and gate-drain capacitances
    pub fn capacitances(&self, vgs: Value, vgd: Value) -> (Value, Value) {
        let scale = self.junction_scale();
        let cgs0 = self.params.cgs * scale;
        let cgd0 = self.params.cgd * scale;
        let pb = self.params.pb;
        let m = self.params.m;
        let fc = self.params.fc;

        // Depletion capacitance model
        let cgs = if vgs <= fc * pb {
            cgs0 / (1.0 - vgs / pb).powf(m)
        } else {
            // Forward bias region - use linear extrapolation
            let f1 = (1.0 - fc).powf(1.0 + m);
            let f2 = 1.0 + m * fc;
            cgs0 / f1 * (f2 + m * vgs / pb)
        };

        let cgd = if vgd <= fc * pb {
            cgd0 / (1.0 - vgd / pb).powf(m)
        } else {
            let f1 = (1.0 - fc).powf(1.0 + m);
            let f2 = 1.0 + m * fc;
            cgd0 / f1 * (f2 + m * vgd / pb)
        };

        (cgs.max(cgs0 * 0.01), cgd.max(cgd0 * 0.01))
    }

    /// Get IDSS (drain current at Vgs=0 in saturation)
    pub fn idss(&self) -> Value {
        self.params.beta * self.params.vto * self.params.vto * self.area * self.m
    }

    /// Link this device to a StaticMatrix for O(1) direct stamping.
    pub fn link(&mut self, matrix: &StaticMatrix) {
        let d = self.drain;
        let g = self.gate;
        let s = self.source;

        if d > 0 {
            self.indices.dd = matrix.get_index(d - 1, d - 1);
        }
        if d > 0 && g > 0 {
            self.indices.dg = matrix.get_index(d - 1, g - 1);
        }
        if d > 0 && s > 0 {
            self.indices.ds = matrix.get_index(d - 1, s - 1);
        }

        if g > 0 && d > 0 {
            self.indices.gd = matrix.get_index(g - 1, d - 1);
        }
        if g > 0 {
            self.indices.gg = matrix.get_index(g - 1, g - 1);
        }
        if g > 0 && s > 0 {
            self.indices.gs = matrix.get_index(g - 1, s - 1);
        }

        if s > 0 && d > 0 {
            self.indices.sd = matrix.get_index(s - 1, d - 1);
        }
        if s > 0 && g > 0 {
            self.indices.sg = matrix.get_index(s - 1, g - 1);
        }
        if s > 0 {
            self.indices.ss = matrix.get_index(s - 1, s - 1);
        }
    }

    /// Stamp using O(1) direct indexing (call after `link`).
    pub fn stamp_direct(&self, matrix: &mut StaticMatrix, rhs: &mut [Value], voltages: &[Value]) {
        let (vgs, vds, vgd) = self.state_or_raw_branch_voltages(voltages);

        let (ids, gm, gds, igs, igd, ggs, ggd, vds_linear) = if self.eval_valid {
            (
                self.eval_ids,
                self.eval_gm,
                self.eval_gds,
                self.eval_igs,
                self.eval_igd,
                self.eval_ggs,
                self.eval_ggd,
                self.eval_vds_linear,
            )
        } else {
            self.compute_operating_terms(vgs, vds, vgd)
        };
        let ids_eq = ids - gm * vgs - gds * vds_linear;
        let igs_eq = igs - ggs * vgs;
        let igd_eq = igd - ggd * vgd;

        // Drain row
        if let Some(idx) = self.indices.dd {
            matrix.stamp_direct(idx, gds + ggd);
        }
        if let Some(idx) = self.indices.dg {
            matrix.stamp_direct(idx, gm - ggd);
        }
        if let Some(idx) = self.indices.ds {
            matrix.stamp_direct(idx, -gm - gds);
        }

        // Gate row
        if let Some(idx) = self.indices.gd {
            matrix.stamp_direct(idx, -ggd);
        }
        if let Some(idx) = self.indices.gg {
            matrix.stamp_direct(idx, ggs + ggd);
        }
        if let Some(idx) = self.indices.gs {
            matrix.stamp_direct(idx, -ggs);
        }

        // Source row
        if let Some(idx) = self.indices.sd {
            matrix.stamp_direct(idx, -gds);
        }
        if let Some(idx) = self.indices.sg {
            matrix.stamp_direct(idx, -gm - ggs);
        }
        if let Some(idx) = self.indices.ss {
            matrix.stamp_direct(idx, gm + gds + ggs);
        }

        if self.drain > 0 {
            rhs[self.drain - 1] -= ids_eq - igd_eq;
        }
        if self.gate > 0 {
            rhs[self.gate - 1] -= igs_eq + igd_eq;
        }
        if self.source > 0 {
            rhs[self.source - 1] += ids_eq + igs_eq;
        }
    }
}

impl NonlinearDevice for Jfet {
    fn update(&mut self, voltages: &[Value]) {
        let vd = Self::node_voltage(voltages, self.drain);
        let vg = Self::node_voltage(voltages, self.gate);
        let vs = Self::node_voltage(voltages, self.source);
        let vgs_raw = vg - vs;
        let vgd_raw = vg - vd;

        let vgs_prev = self.vgs;
        let vds_prev = self.vds;
        let vgd_prev = if vgs_prev.is_finite() && vds_prev.is_finite() {
            vgs_prev - vds_prev
        } else {
            Value::NAN
        };

        self.vgs_prev = vgs_prev;
        self.vds_prev = vds_prev;

        let mut vgs = vgs_raw;
        let mut vgd = vgd_raw;
        let mut limiter_applied = false;

        if matches!(self.params.channel_model, JfetChannelModel::Hfet1) {
            if matches!(self.params.hfet_level, 2..=4)
                && vgs_prev.is_finite()
                && vgd_prev.is_finite()
            {
                let (_, temp_source, temp_drain) = self.resolved_temperatures(self.params.tnom);
                let n = self.params.n.max(1e-12);
                let vtes = (n * self.thermal_voltage(temp_source)).max(1e-12);
                let vted = (n * self.thermal_voltage(temp_drain)).max(1e-12);
                let vcrits = self.mesa_gate_vcrit(temp_source, vtes);
                let vcritd = self.mesa_gate_vcrit(temp_drain, vted);
                let vgs_limited = Self::pnjlim(vgs, vgs_prev, vtes, vcrits);
                let vgd_limited = Self::pnjlim(vgd, vgd_prev, vted, vcritd);
                limiter_applied |= (vgs_limited - vgs).abs() > 0.0;
                limiter_applied |= (vgd_limited - vgd).abs() > 0.0;
                vgs = vgs_limited;
                vgd = vgd_limited;
            }
            let vgs_before_fetlim = vgs;
            let vgd_before_fetlim = vgd;
            let (vgs_limited, vgd_limited) = self.hfet_limited_branch_voltages(vgs, vgd);
            limiter_applied |= (vgs_limited - vgs_before_fetlim).abs() > 0.0;
            limiter_applied |= (vgd_limited - vgd_before_fetlim).abs() > 0.0;
            vgs = vgs_limited;
            vgd = vgd_limited;
        }

        let mut bypassed = false;
        if self.eval_valid && vgs_prev.is_finite() && vgd_prev.is_finite() {
            const RELTOL: Value = 1e-3;
            const VOLT_TOL: Value = 1e-6;
            const ABSTOL: Value = 1e-12;

            let delvgs = vgs - vgs_prev;
            let delvgd = vgd - vgd_prev;
            let delvds = delvgs - delvgd;

            let cghat = self.lin_cg + self.eval_ggs * delvgs + self.eval_ggd * delvgd;
            let cdhat = self.lin_cd + self.eval_gm * delvgs + self.eval_gds * delvds
                - self.eval_ggd * delvgd;

            let vgs_ok = delvgs.abs() <= RELTOL * vgs.abs().max(vgs_prev.abs()) + VOLT_TOL;
            let vgd_ok = delvgd.abs() <= RELTOL * vgd.abs().max(vgd_prev.abs()) + VOLT_TOL;
            let cg_ok =
                (cghat - self.lin_cg).abs() <= RELTOL * cghat.abs().max(self.lin_cg.abs()) + ABSTOL;
            let cd_ok =
                (cdhat - self.lin_cd).abs() <= RELTOL * cdhat.abs().max(self.lin_cd.abs()) + ABSTOL;

            if vgs_ok && vgd_ok && cg_ok && cd_ok {
                vgs = vgs_prev;
                vgd = vgd_prev;
                bypassed = true;
            }
        }

        let vds = vgs - vgd;
        self.vgs = vgs;
        self.vds = vds;
        self.limiter_applied = limiter_applied;

        if !bypassed {
            let (ids, gm, gds, igs, igd, ggs, ggd, vds_linear) =
                self.compute_operating_terms(vgs, vds, vgd);
            self.eval_ids = ids;
            self.eval_gm = gm;
            self.eval_gds = gds;
            self.eval_igs = igs;
            self.eval_igd = igd;
            self.eval_ggs = ggs;
            self.eval_ggd = ggd;
            self.eval_vds_linear = vds_linear;
            self.lin_vgs = vgs;
            self.lin_vgd = vgd;
            self.lin_cg = igs + igd;
            self.lin_cd = ids - igd;
            self.eval_valid = true;
        }
    }

    fn stamp_nonlinear(
        &self,
        voltages: &[Value],
        matrix: &mut impl MatrixStamper,
        _rhs: &mut [Value],
    ) {
        let (vgs, vds, vgd) = self.state_or_raw_branch_voltages(voltages);

        let (ids, gm, gds, igs, igd, ggs, ggd, vds_linear) = if self.eval_valid {
            (
                self.eval_ids,
                self.eval_gm,
                self.eval_gds,
                self.eval_igs,
                self.eval_igd,
                self.eval_ggs,
                self.eval_ggd,
                self.eval_vds_linear,
            )
        } else {
            self.compute_operating_terms(vgs, vds, vgd)
        };
        let ids_eq = ids - gm * vgs - gds * vds_linear;
        let igs_eq = igs - ggs * vgs;
        let igd_eq = igd - ggd * vgd;

        matrix.stamp(self.drain, self.drain, gds + ggd);
        matrix.stamp(self.drain, self.gate, gm - ggd);
        matrix.stamp(self.drain, self.source, -gm - gds);

        matrix.stamp(self.gate, self.drain, -ggd);
        matrix.stamp(self.gate, self.gate, ggs + ggd);
        matrix.stamp(self.gate, self.source, -ggs);

        matrix.stamp(self.source, self.drain, -gds);
        matrix.stamp(self.source, self.gate, -gm - ggs);
        matrix.stamp(self.source, self.source, gm + gds + ggs);

        matrix.stamp_rhs(self.drain, -ids_eq + igd_eq);
        matrix.stamp_rhs(self.gate, -igs_eq - igd_eq);
        matrix.stamp_rhs(self.source, ids_eq + igs_eq);
    }

    fn is_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        const RELTOL: Value = 1e-3;
        let tolerance = criteria.voltage_tolerance();

        if self.limiter_applied {
            return false;
        }

        if !self.vgs.is_finite()
            || !self.vgs_prev.is_finite()
            || !self.vds.is_finite()
            || !self.vds_prev.is_finite()
        {
            return false;
        }

        let vgs_diff = (self.vgs - self.vgs_prev).abs();
        let vds_diff = (self.vds - self.vds_prev).abs();
        let vgs_tol = RELTOL * self.vgs.abs().max(self.vgs_prev.abs()) + tolerance;
        let vds_tol = RELTOL * self.vds.abs().max(self.vds_prev.abs()) + tolerance;

        vgs_diff < vgs_tol && vds_diff < vds_tol
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jfet_creation() {
        let jfet = Jfet::njf("J1", 1, 2, 3);
        assert_eq!(jfet.name, "J1");
        assert_eq!(jfet.jfet_type, JfetType::NJF);
        assert_eq!(jfet.drain, 1);
        assert_eq!(jfet.gate, 2);
        assert_eq!(jfet.source, 3);
    }

    #[test]
    fn test_pjf_creation() {
        let jfet = Jfet::pjf("J2", 1, 2, 3);
        assert_eq!(jfet.jfet_type, JfetType::PJF);
        assert_eq!(jfet.jfet_type.polarity(), -1.0);
    }

    #[test]
    fn test_cutoff_region() {
        let params = JfetParams::default(); // VTO = -2V
        let jfet = Jfet::njf("J1", 1, 2, 0).with_params(params);

        // Vgs = -3V < Vto = -2V â†’ cutoff
        let (ids, gm, gds) = jfet.calculate(-3.0, 5.0, 300.0);

        assert!(ids.abs() < 1e-15, "Ids should be ~0 in cutoff, got {}", ids);
        assert!(gm.abs() < 1e-15, "gm should be 0 in cutoff");
        assert!(gds.abs() < 1e-15, "gds should be 0 in cutoff");
    }

    #[test]
    fn test_saturation_region() {
        let params = JfetParams::new()
            .with_vto(-2.0)
            .with_beta(1e-3)
            .with_lambda(0.0);
        let jfet = Jfet::njf("J1", 1, 2, 0).with_params(params);

        // Vgs = 0V, Vds = 5V â†’ saturation (Vds > Vgs - Vto = 0 - (-2) = 2)
        let (ids, gm, gds) = jfet.calculate(0.0, 5.0, 300.0);

        // Ids = beta * (Vgs - Vto)Â² = 1e-3 * (0 - (-2))Â² = 1e-3 * 4 = 4mA
        assert!((ids - 4e-3).abs() < 1e-6, "Expected Ids=4mA, got {}", ids);

        // gm = 2 * beta * (Vgs - Vto) = 2 * 1e-3 * 2 = 4mS
        assert!((gm - 4e-3).abs() < 1e-6, "Expected gm=4mS, got {}", gm);

        // gds = 0 when lambda = 0
        assert!(gds.abs() < 1e-10, "gds should be ~0 with lambda=0");
    }

    #[test]
    fn test_linear_region() {
        let params = JfetParams::new()
            .with_vto(-2.0)
            .with_beta(1e-3)
            .with_lambda(0.0);
        let jfet = Jfet::njf("J1", 1, 2, 0).with_params(params);

        // Vgs = 0V, Vds = 1V â†’ linear (Vds < Vgs - Vto = 2)
        let (ids, gm, gds) = jfet.calculate(0.0, 1.0, 300.0);

        // Ids = beta * (2*(Vgs-Vto)*Vds - VdsÂ²) = 1e-3 * (2*2*1 - 1) = 1e-3 * 3 = 3mA
        assert!((ids - 3e-3).abs() < 1e-6, "Expected Ids=3mA, got {}", ids);

        // gm = 2 * beta * Vds = 2 * 1e-3 * 1 = 2mS
        assert!((gm - 2e-3).abs() < 1e-6, "Expected gm=2mS, got {}", gm);

        // gds = beta * 2 * (Vgst - Vds) = 1e-3 * 2 * (2 - 1) = 2mS
        assert!((gds - 2e-3).abs() < 1e-6, "Expected gds=2mS, got {}", gds);
    }

    #[test]
    fn test_reverse_vds_changes_current_and_gm_sign_for_njf() {
        let params = JfetParams::new()
            .with_vto(-2.0)
            .with_beta(1e-3)
            .with_lambda(0.0);
        let jfet = Jfet::njf("J1", 1, 2, 0).with_params(params);

        let (ids_fwd, gm_fwd, gds_fwd) = jfet.calculate(0.0, 1.0, 300.0);
        let (ids_rev, gm_rev, gds_rev) = jfet.calculate(0.0, -1.0, 300.0);

        assert!(
            ids_fwd > 0.0,
            "forward Ids should be positive, got {}",
            ids_fwd
        );
        assert!(
            gm_fwd > 0.0,
            "forward gm should be positive, got {}",
            gm_fwd
        );
        assert!(
            gds_fwd > 0.0,
            "forward gds should be positive, got {}",
            gds_fwd
        );

        assert!(
            ids_rev < 0.0,
            "reverse Vds should invert drain current direction, got {}",
            ids_rev
        );
        assert!(
            gm_rev < 0.0,
            "reverse Vds should invert gm sign in original terminal orientation, got {}",
            gm_rev
        );
        assert!(gds_rev > 0.0, "gds should remain positive, got {}", gds_rev);
    }

    #[test]
    fn test_channel_length_modulation() {
        let params = JfetParams::new()
            .with_vto(-2.0)
            .with_beta(1e-3)
            .with_lambda(0.01);
        let jfet = Jfet::njf("J1", 1, 2, 0).with_params(params);

        // Saturation with lambda > 0
        let (ids1, _, gds1) = jfet.calculate(0.0, 5.0, 300.0);
        let (ids2, _, _) = jfet.calculate(0.0, 10.0, 300.0);

        // Ids should increase with Vds due to lambda
        assert!(ids2 > ids1, "Ids should increase with Vds when lambda > 0");

        // gds = beta * VgstÂ² * lambda = 1e-3 * 4 * 0.01 = 40ÂµS
        assert!(
            (gds1 - 40e-6).abs() < 1e-9,
            "Expected gds=40ÂµS, got {}",
            gds1
        );
    }

    #[test]
    fn test_pjf_polarity() {
        // P-JFET uses same VTO sign convention as N-JFET in the model
        // The polarity multiplier handles the sign transformation
        let params = JfetParams::new()
            .with_vto(-2.0) // VTO=-2 (same as N-JFET)
            .with_beta(1e-3)
            .with_lambda(0.0);
        let jfet = Jfet::pjf("J1", 1, 2, 0).with_params(params);

        // P-JFET: Vgs = 0, Vds = -5V
        // Internal: vgs_int = -1*0 = 0, vds_int = -1*(-5) = 5
        // vgst = 0 - (-2) = 2 > 0, so device is ON
        // Saturation: vds_int=5 > vgst=2
        let (ids, _gm, _) = jfet.calculate(0.0, -5.0, 300.0);

        // Current flows opposite direction (negative Ids for P-JFET)
        // Internal ids = beta * vgst^2 = 1e-3 * 4 = 4mA
        // Output: pol * ids = -1 * 4mA = -4mA
        assert!(ids < 0.0, "P-JFET Ids should be negative, got {}", ids);
        assert!(
            (ids.abs() - 4e-3).abs() < 1e-6,
            "Expected |Ids|=4mA, got {}",
            ids.abs()
        );
    }

    #[test]
    fn test_idss_calculation() {
        let params = JfetParams::from_idss(10e-3, -2.0); // 10mA IDSS
        let jfet = Jfet::njf("J1", 1, 2, 0).with_params(params);

        // IDSS = Ids at Vgs=0, saturation
        let (ids, _, _) = jfet.calculate(0.0, 10.0, 300.0);

        // Should be close to 10mA (exactly 10mA with lambda=0)
        assert!(
            (ids - 10e-3).abs() < 1e-6,
            "Expected Idsâ‰ˆIDSS=10mA, got {}",
            ids
        );
    }

    #[test]
    fn test_gate_current() {
        let params = JfetParams::new().with_junction(1e-14, 0.8);
        let jfet = Jfet::njf("J1", 1, 2, 0).with_params(params);

        // Reverse biased gate (normal operation)
        let (igs, igd) = jfet.gate_current(-1.0, -6.0, 300.0);

        // ngspice-compatible reverse branch includes explicit gmin*v term,
        // so reverse current is pA-level, not pure saturation current.
        assert!(
            igs < -5e-13,
            "Gate-source reverse current should include gmin branch, got {}",
            igs
        );
        assert!(
            igd < -5e-12,
            "Gate-drain reverse current should include gmin branch, got {}",
            igd
        );
    }

    #[test]
    fn test_gate_current_reverse_branch_matches_ngspice_gmin_asymptote() {
        let params = JfetParams::new().with_junction(1e-14, 1.0);
        let jfet = Jfet::njf("J1", 1, 2, 0).with_params(params);

        let (igs, ggs) = jfet.junction_diode_terms(-3.0, 300.15);
        assert!(
            (igs - (-2.78e-12)).abs() < 4.0e-13,
            "expected ngspice-like reverse current near -2.8pA at -3V, got {}",
            igs
        );
        assert!(
            (ggs - 1e-12).abs() < 2e-15,
            "expected gmin-dominated reverse conductance near 1e-12, got {}",
            ggs
        );
    }

    #[test]
    fn test_gate_current_pjf_forward_bias_has_negative_gate_current() {
        let params = JfetParams::new().with_junction(1e-12, 0.8);
        let jfet = Jfet::pjf("J1", 1, 2, 0).with_params(params);

        // For P-JFET, source-to-gate forward bias means gate current is negative
        // when defined as current flowing from gate to source/drain.
        let (igs, igd) = jfet.gate_current(-0.6, -0.6, 300.0);
        assert!(
            igs < 0.0,
            "P-JFET gate-source current should be negative in forward bias, got {}",
            igs
        );
        assert!(
            igd < 0.0,
            "P-JFET gate-drain current should be negative in forward bias, got {}",
            igd
        );
    }

    #[test]
    fn test_gate_current_large_forward_is_finite() {
        let params = JfetParams::new().with_junction(1e-12, 0.8);
        let jfet = Jfet::njf("J1", 1, 2, 0).with_params(params);
        let (igs, igd) = jfet.gate_current(100.0, 80.0, 300.0);

        assert!(
            igs.is_finite(),
            "Igs must remain finite under large forward bias"
        );
        assert!(
            igd.is_finite(),
            "Igd must remain finite under large forward bias"
        );
        assert!(
            igs > 0.0,
            "Igs should be positive for strongly forward-biased NJF"
        );
        assert!(
            igd > 0.0,
            "Igd should be positive for strongly forward-biased NJF"
        );
    }

    #[test]
    fn test_capacitances() {
        let params = JfetParams::new().with_capacitances(5e-12, 2e-12); // 5pF CGS, 2pF CGD
        let jfet = Jfet::njf("J1", 1, 2, 0).with_params(params);

        // Zero bias capacitances
        let (cgs, cgd) = jfet.capacitances(0.0, 0.0);

        assert!(
            (cgs - 5e-12).abs() < 1e-15,
            "CGS at zero bias should be 5pF"
        );
        assert!(
            (cgd - 2e-12).abs() < 1e-15,
            "CGD at zero bias should be 2pF"
        );

        // Reverse bias increases depletion width, decreases capacitance
        let (cgs_rev, _) = jfet.capacitances(-2.0, -5.0);
        assert!(cgs_rev < cgs, "CGS should decrease with reverse bias");
    }

    #[test]
    fn test_capacitances_scale_with_area_and_multiplier() {
        let params = JfetParams::new().with_capacitances(1e-12, 0.5e-12);
        let jfet = Jfet::njf("J1", 1, 2, 0)
            .with_params(params)
            .with_area(2.0)
            .with_multiplier(3.0);

        let (cgs, cgd) = jfet.capacitances(0.0, 0.0);
        assert!(
            (cgs - 6e-12).abs() < 1e-18,
            "expected CGS to scale with area*m, got {}",
            cgs
        );
        assert!(
            (cgd - 3e-12).abs() < 1e-18,
            "expected CGD to scale with area*m, got {}",
            cgd
        );
    }

    #[test]
    fn test_params_builder() {
        let params = JfetParams::new()
            .with_vto(-3.0)
            .with_beta(2e-3)
            .with_lambda(0.02)
            .with_capacitances(10e-12, 5e-12)
            .with_junction(1e-15, 0.9);

        assert_eq!(params.vto, -3.0);
        assert_eq!(params.beta, 2e-3);
        assert_eq!(params.lambda, 0.02);
        assert_eq!(params.cgs, 10e-12);
        assert_eq!(params.cgd, 5e-12);
        assert_eq!(params.is, 1e-15);
        assert_eq!(params.pb, 0.9);
    }

    #[test]
    fn test_with_model_params_derives_beta_from_idss_when_beta_absent() {
        use std::collections::HashMap;

        let mut model = HashMap::new();
        model.insert("VTO".to_string(), -3.0);
        model.insert("IDSS".to_string(), 12e-3);

        let jfet = Jfet::njf("J1", 1, 2, 0).with_model_params(&model);
        let expected = 12e-3 / 9.0;
        assert!(
            (jfet.params.beta - expected).abs() < 1e-15,
            "expected beta={} from IDSS/VTO^2, got {}",
            expected,
            jfet.params.beta
        );
    }

    #[test]
    fn test_with_model_params_explicit_beta_overrides_idss() {
        use std::collections::HashMap;

        let mut model = HashMap::new();
        model.insert("VTO".to_string(), -3.0);
        model.insert("IDSS".to_string(), 12e-3);
        model.insert("BETA".to_string(), 2.5e-3);

        let jfet = Jfet::njf("J1", 1, 2, 0).with_model_params(&model);
        assert!(
            (jfet.params.beta - 2.5e-3).abs() < 1e-18,
            "explicit BETA should override IDSS-derived beta, got {}",
            jfet.params.beta
        );
    }

    #[test]
    fn test_with_model_params_applies_tnom_and_grading_coefficient() {
        use std::collections::HashMap;

        let mut model = HashMap::new();
        model.insert("TNOM".to_string(), 325.0);
        model.insert("M".to_string(), 0.35);
        model.insert("FC".to_string(), 0.4);

        let jfet = Jfet::njf("J1", 1, 2, 0).with_model_params(&model);
        assert!((jfet.params.tnom - 325.0).abs() < 1e-12);
        assert!((jfet.params.m - 0.35).abs() < 1e-12);
        assert!((jfet.params.fc - 0.4).abs() < 1e-12);
    }

    #[test]
    fn test_with_model_params_applies_noise_coefficients() {
        use std::collections::HashMap;

        let mut model = HashMap::new();
        model.insert("KF".to_string(), 5e-18);
        model.insert("AF".to_string(), 1.1);
        model.insert("EF".to_string(), 1.6);

        let jfet = Jfet::njf("J1", 1, 2, 0)
            .with_model_params(&model)
            .with_instance_params(&[("W".to_string(), 20e-6), ("L".to_string(), 0.8e-6)]);

        let (kf, af, ef) = jfet
            .flicker_noise_coefficients()
            .expect("KF should enable flicker noise");
        let expected_kf = 5e-18 / (jfet.width * jfet.length);
        assert!((kf - expected_kf).abs() / expected_kf < 1e-12);
        assert!((af - 1.1).abs() < 1e-12);
        assert!((ef - 1.6).abs() < 1e-12);
    }

    #[test]
    fn test_with_model_params_rejects_nonphysical_values() {
        use std::collections::HashMap;

        let baseline = Jfet::njf("J1", 1, 2, 0);
        let mut model = HashMap::new();
        model.insert("BETA".to_string(), -1e-3);
        model.insert("LAMBDA".to_string(), -0.1);
        model.insert("IS".to_string(), -1e-9);
        model.insert("PB".to_string(), 0.0);
        model.insert("FC".to_string(), 1.2);
        model.insert("N".to_string(), 0.0);
        model.insert("TNOM".to_string(), -10.0);
        model.insert("RD".to_string(), -10.0);
        model.insert("RS".to_string(), -12.0);

        let jfet = baseline.clone().with_model_params(&model);
        assert!(
            (jfet.params.beta - baseline.params.beta).abs() < 1e-30,
            "invalid BETA should be ignored"
        );
        assert!(
            (jfet.params.lambda - baseline.params.lambda).abs() < 1e-30,
            "invalid LAMBDA should be ignored"
        );
        assert!(
            (jfet.params.is - baseline.params.is).abs() < 1e-30,
            "invalid IS should be ignored"
        );
        assert!(
            (jfet.params.pb - baseline.params.pb).abs() < 1e-30,
            "invalid PB should be ignored"
        );
        assert!(
            (jfet.params.fc - baseline.params.fc).abs() < 1e-30,
            "invalid FC should be ignored"
        );
        assert!(
            (jfet.params.n - baseline.params.n).abs() < 1e-30,
            "invalid N should be ignored"
        );
        assert!(
            (jfet.params.tnom - baseline.params.tnom).abs() < 1e-30,
            "invalid TNOM should be ignored"
        );
        assert!(
            (jfet.params.rd - baseline.params.rd).abs() < 1e-30,
            "invalid RD should be ignored"
        );
        assert!(
            (jfet.params.rs - baseline.params.rs).abs() < 1e-30,
            "invalid RS should be ignored"
        );

        let (cgs, cgd) = jfet.capacitances(0.0, 0.0);
        assert!(cgs.is_finite() && cgd.is_finite());
    }

    #[test]
    fn test_with_model_params_accepts_hfet_aliases() {
        use std::collections::HashMap;

        let mut model = HashMap::new();
        model.insert("VT0".to_string(), 0.3);
        model.insert("RDI".to_string(), 12.0);
        model.insert("RSI".to_string(), 8.0);

        let jfet = Jfet::njf("J1", 1, 2, 0).with_model_params(&model);
        assert!((jfet.params.vto - 0.3).abs() < 1e-15);
        assert!((jfet.params.rd - 12.0).abs() < 1e-15);
        assert!((jfet.params.rs - 8.0).abs() < 1e-15);
    }

    #[test]
    fn test_with_model_params_preserves_m_as_grading_parameter() {
        use std::collections::HashMap;

        let mut model = HashMap::new();
        model.insert("M".to_string(), 2.5);

        let jfet = Jfet::njf("J1", 1, 2, 0).with_model_params(&model);
        assert!((jfet.m - 1.0).abs() < 1e-15);
        assert!(
            (jfet.params.m - 2.5).abs() < 1e-15,
            "model M should map to grading coefficient"
        );
    }

    #[test]
    fn test_with_instance_params_prefers_area_and_applies_multiplier() {
        let params = vec![
            ("AREA".to_string(), 2.0),
            ("M".to_string(), 3.0),
            ("W".to_string(), 10e-6),
            ("L".to_string(), 1e-6),
        ];
        let jfet = Jfet::njf("J1", 1, 2, 0).with_instance_params(&params);
        assert!((jfet.area - 2.0).abs() < 1e-15);
        assert!((jfet.m - 3.0).abs() < 1e-15);
    }

    #[test]
    fn test_with_instance_params_uses_w_over_l_scaling_when_area_absent() {
        let params = vec![
            ("W".to_string(), 20e-6),
            ("L".to_string(), 0.5e-6),
            ("NF".to_string(), 2.0),
        ];
        let jfet = Jfet::njf("J1", 1, 2, 0).with_instance_params(&params);
        let expected = (20e-6 / 0.5e-6) * 2.0;
        assert!((jfet.area - expected).abs() < 1e-12);
        assert!((jfet.m - 1.0).abs() < 1e-15);
    }

    #[test]
    fn test_with_instance_params_tracks_temperature_overrides() {
        let params = vec![
            ("TEMP".to_string(), 340.0),
            ("DTEMP".to_string(), 25.0),
            ("TS".to_string(), 400.0),
            ("TD".to_string(), 450.0),
        ];
        let jfet = Jfet::njf("Z1", 1, 2, 0).with_instance_params(&params);
        let (t, ts, td) = jfet.resolved_temperatures(300.15);
        assert!((t - (340.0 + 273.15)).abs() < 1e-12);
        assert!((ts - (400.0 + 273.15)).abs() < 1e-12);
        assert!((td - (450.0 + 273.15)).abs() < 1e-12);
    }

    #[test]
    fn test_with_instance_params_applies_dtemp_when_temp_absent() {
        let params = vec![("DTEMP".to_string(), 25.0)];
        let jfet = Jfet::njf("Z1", 1, 2, 0).with_instance_params(&params);
        let (t, ts, td) = jfet.resolved_temperatures(300.15);
        assert!((t - 325.15).abs() < 1e-12);
        assert!((ts - 325.15).abs() < 1e-12);
        assert!((td - 325.15).abs() < 1e-12);
    }

    #[test]
    fn test_mesa_channel_current_uses_instance_source_temperature() {
        let cold = Jfet::njf("Z1", 1, 2, 0).enable_mesa_model();
        let hot = Jfet::njf("Z1", 1, 2, 0)
            .enable_mesa_model()
            .with_instance_params(&[("TS".to_string(), 400.0), ("TD".to_string(), 400.0)]);

        let (ids_cold, _, _) = cold.calculate(-3.0, 0.1, 300.15);
        let (ids_hot, _, _) = hot.calculate(-3.0, 0.1, 300.15);
        assert!(
            ids_hot.abs() > ids_cold.abs() * 100.0,
            "expected TS/TD override to strongly increase subthreshold current: cold={ids_cold}, hot={ids_hot}"
        );
    }

    #[test]
    fn test_mesa_level2_single_point_matches_ngspice_reference() {
        // Reference generated from ngspice 45.2:
        // vgs=-3V, vgd=-3.1V, LEVEL=2, default model, TS=TD=400:
        // igd ~= -4.6945e-3 A (dominant branch current at this operating point).
        let jfet = Jfet::njf("Z1", 1, 2, 0)
            .enable_mesa_model()
            .with_instance_params(&[("TS".to_string(), 400.0), ("TD".to_string(), 400.0)]);
        let (_igs, igd) = jfet.gate_current(-3.0, -3.1, 300.15);
        let expected = 4.694_517e-3;
        let rel = (igd.abs() - expected).abs() / expected.abs();
        assert!(
            rel < 0.05,
            "mesa LEVEL=2 gate-drain current mismatch: igd={igd} expected={expected} rel={rel}"
        );
    }

    #[test]
    fn test_mesa_model_params_load_cas_cbs_terms() {
        use std::collections::HashMap;

        let mut model = HashMap::new();
        model.insert("LEVEL".to_string(), 4.0);
        model.insert("CAS".to_string(), 2.5);
        model.insert("CBS".to_string(), 0.75);

        let jfet = Jfet::njf("Z1", 1, 2, 0)
            .enable_mesa_model()
            .with_model_params(&model);
        assert!((jfet.params.mesa_cas - 2.5).abs() < 1e-15);
        assert!((jfet.params.mesa_cbs - 0.75).abs() < 1e-15);
    }

    #[test]
    fn test_mesa_transient_capacitances_route_by_level() {
        let mut l2 = Jfet::njf("Z2", 1, 2, 0).enable_mesa_model();
        l2.params.hfet_level = 2;

        let mut l3 = l2.clone();
        l3.params.hfet_level = 3;

        let mut l4 = l2.clone();
        l4.params.hfet_level = 4;

        let bias_points = [
            (-2.9, -3.0),
            (-1.2, -1.0),
            (-0.3, -0.6),
            (0.2, 0.0),
            (0.4, 0.1),
        ];

        let mut saw_level23_delta = false;
        let mut saw_level34_delta = false;
        for (vgs, vgd) in bias_points {
            let (cgs2, cgd2) = l2.transient_capacitances(vgs, vgd, 300.15);
            let (cgs3, cgd3) = l3.transient_capacitances(vgs, vgd, 300.15);
            let (cgs4, cgd4) = l4.transient_capacitances(vgs, vgd, 300.15);

            assert!(
                cgs2.is_finite() && cgs2 > 0.0 && cgd2.is_finite() && cgd2 > 0.0,
                "LEVEL=2 caps should remain finite/positive at vgs={vgs}, vgd={vgd}"
            );
            assert!(
                cgs3.is_finite() && cgs3 > 0.0 && cgd3.is_finite() && cgd3 > 0.0,
                "LEVEL=3 caps should remain finite/positive at vgs={vgs}, vgd={vgd}"
            );
            assert!(
                cgs4.is_finite() && cgs4 > 0.0 && cgd4.is_finite() && cgd4 > 0.0,
                "LEVEL=4 caps should remain finite/positive at vgs={vgs}, vgd={vgd}"
            );

            let abs_delta = |a: f64, b: f64| (a - b).abs();
            if abs_delta(cgs2, cgs3) > 1e-24 || abs_delta(cgd2, cgd3) > 1e-24 {
                saw_level23_delta = true;
            }
            if abs_delta(cgs3, cgs4) > 1e-24 || abs_delta(cgd3, cgd4) > 1e-24 {
                saw_level34_delta = true;
            }
        }

        assert!(
            saw_level23_delta,
            "LEVEL=2 and LEVEL=3 capacitance paths should diverge at least one bias point"
        );
        assert!(
            saw_level34_delta,
            "LEVEL=3 and LEVEL=4 capacitance paths should diverge at least one bias point"
        );
    }

    #[test]
    fn test_mesa_level4_capacitances_respond_to_cas_cbs() {
        use std::collections::HashMap;

        let mut base_model = HashMap::new();
        base_model.insert("LEVEL".to_string(), 4.0);

        let mut scaled_model = base_model.clone();
        scaled_model.insert("CAS".to_string(), 2.0);
        scaled_model.insert("CBS".to_string(), 0.5);

        let base = Jfet::njf("Z1", 1, 2, 0)
            .enable_mesa_model()
            .with_model_params(&base_model);
        let scaled = Jfet::njf("Z2", 1, 2, 0)
            .enable_mesa_model()
            .with_model_params(&scaled_model);

        let bias_points = [
            (-2.9, -3.0),
            (-1.2, -1.0),
            (-0.3, -0.6),
            (0.2, 0.0),
            (0.4, 0.1),
        ];
        let mut saw_delta = false;
        for (vgs, vgd) in bias_points {
            let (cgs_base, cgd_base) = base.transient_capacitances(vgs, vgd, 300.15);
            let (cgs_scaled, cgd_scaled) = scaled.transient_capacitances(vgs, vgd, 300.15);
            if (cgs_base - cgs_scaled).abs() > 1e-24 || (cgd_base - cgd_scaled).abs() > 1e-24 {
                saw_delta = true;
            }
        }

        assert!(
            saw_delta,
            "LEVEL=4 CAS/CBS must influence transient capacitances"
        );
    }

    #[test]
    fn test_eta_and_sigma0_are_loaded_from_model_card() {
        use std::collections::HashMap;

        let mut model = HashMap::new();
        model.insert("VT0".to_string(), 0.3);
        model.insert("BETA".to_string(), 1e-3);
        model.insert("ETA".to_string(), 1.2);
        model.insert("SIGMA0".to_string(), 0.04);

        let jfet = Jfet::njf("Z1", 1, 2, 0).with_model_params(&model);
        assert!((jfet.params.eta - 1.2).abs() < 1e-15);
        assert!((jfet.params.sigma0 - 0.04).abs() < 1e-15);
    }

    #[test]
    fn test_enable_hfet_model_applies_hfet_defaults() {
        let jfet = Jfet::njf("Z1", 1, 2, 0).enable_hfet_model();
        assert_eq!(jfet.params.channel_model, JfetChannelModel::Hfet1);
        assert!((jfet.params.vto - 0.15).abs() < 1e-15);
        assert!((jfet.params.hfet_mu - 0.4).abs() < 1e-15);
        assert!((jfet.width - 20e-6).abs() < 1e-18);
        assert!((jfet.length - 1e-6).abs() < 1e-18);
        assert!(jfet.uses_hfet_legacy_inverse_mode());
    }

    #[test]
    fn test_enable_mesa_model_disables_hfet1_legacy_inverse_mode() {
        let jfet = Jfet::njf("Z1", 1, 2, 0).enable_mesa_model();
        assert_eq!(jfet.params.channel_model, JfetChannelModel::Hfet1);
        assert_eq!(jfet.params.hfet_level, 2);
        assert!(!jfet.uses_hfet_legacy_inverse_mode());
    }

    #[test]
    fn test_hfet_model_params_separate_rdi_rsi_from_rd_rs() {
        use std::collections::HashMap;

        let mut model = HashMap::new();
        model.insert("RD".to_string(), 60.0);
        model.insert("RS".to_string(), 40.0);
        model.insert("RDI".to_string(), 7.0);
        model.insert("RSI".to_string(), 9.0);

        let jfet = Jfet::njf("Z1", 1, 2, 0)
            .enable_hfet_model()
            .with_model_params(&model);
        assert!((jfet.params.rd - 60.0).abs() < 1e-15);
        assert!((jfet.params.rs - 40.0).abs() < 1e-15);
        assert!((jfet.params.hfet_rdi - 7.0).abs() < 1e-15);
        assert!((jfet.params.hfet_rsi - 9.0).abs() < 1e-15);
    }

    #[test]
    fn test_hfet_instance_params_set_geometry_without_w_over_l_area_scaling() {
        let params = vec![
            ("W".to_string(), 10e-6),
            ("L".to_string(), 2e-6),
            ("NF".to_string(), 3.0),
            ("M".to_string(), 2.0),
        ];
        let jfet = Jfet::njf("Z1", 1, 2, 0)
            .enable_hfet_model()
            .with_instance_params(&params);
        assert!((jfet.width - 30e-6).abs() < 1e-18);
        assert!((jfet.length - 2e-6).abs() < 1e-18);
        assert!((jfet.area - 1.0).abs() < 1e-18);
        assert!((jfet.m - 2.0).abs() < 1e-18);
    }

    #[test]
    fn test_hfet_leak_large_reverse_bias_matches_gmin_branch() {
        let (il, gl) = Jfet::hfet_leak(1e-12, 0.02585, -1.0, 90.0, 5e-12, 5e-6, 1.32, 6.9);
        let expected_il = -1e-12 - 5e-12;
        assert!((gl - 1e-12).abs() < 1e-24, "reverse branch should use gmin");
        assert!(
            (il - expected_il).abs() < 1e-18,
            "reverse branch current should follow gmin*v-is1"
        );
    }

    #[test]
    fn test_hfet_gate_branch_generation_recombination_active_with_zero_js() {
        use std::collections::HashMap;

        let mut model = HashMap::new();
        model.insert("JS1S".to_string(), 0.0);
        model.insert("JS2S".to_string(), 0.0);
        model.insert("JS1D".to_string(), 0.0);
        model.insert("JS2D".to_string(), 0.0);
        model.insert("LEVEL".to_string(), 1.0);
        model.insert("GGR".to_string(), 40.0);
        model.insert("DEL".to_string(), 0.04);

        let z = Jfet::njf("Z1", 1, 2, 0)
            .enable_hfet_model()
            .with_model_params(&model)
            .with_instance_params(&[("W".to_string(), 10e-6), ("L".to_string(), 1e-6)]);

        let (igs, igd, ggs, ggd) = z.gate_junctions(0.3, 0.2, 300.15);
        assert!(igs.is_finite() && igd.is_finite() && ggs.is_finite() && ggd.is_finite());
        assert!(
            igs.abs() > 0.0 || igd.abs() > 0.0,
            "GGR branch should produce non-zero gate current when JS terms are disabled"
        );
        assert!(
            ggs > 0.0 && ggd > 0.0,
            "GGR branch should produce positive gate conductances"
        );
    }

    #[test]
    fn test_hfet_gate_junctions_respond_to_second_schottky_branch() {
        use std::collections::HashMap;

        let mut with_js2 = HashMap::new();
        with_js2.insert("JS1S".to_string(), 1e-12);
        with_js2.insert("JS2S".to_string(), 1.15e6);
        with_js2.insert("JS1D".to_string(), 1e-12);
        with_js2.insert("JS2D".to_string(), 1.15e6);
        with_js2.insert("LEVEL".to_string(), 1.0);
        with_js2.insert("GGR".to_string(), 0.0);

        let mut without_js2 = with_js2.clone();
        without_js2.insert("JS2S".to_string(), 0.0);
        without_js2.insert("JS2D".to_string(), 0.0);

        let z_with = Jfet::njf("Z1", 1, 2, 0)
            .enable_hfet_model()
            .with_model_params(&with_js2)
            .with_instance_params(&[("W".to_string(), 10e-6), ("L".to_string(), 1e-6)]);
        let z_without = Jfet::njf("Z2", 1, 2, 0)
            .enable_hfet_model()
            .with_model_params(&without_js2)
            .with_instance_params(&[("W".to_string(), 10e-6), ("L".to_string(), 1e-6)]);

        let vgs = 0.35;
        let vgd = 0.10;
        let (igs_with, igd_with, _, _) = z_with.gate_junctions(vgs, vgd, 300.15);
        let (igs_without, igd_without, _, _) = z_without.gate_junctions(vgs, vgd, 300.15);

        assert!(
            igs_with.abs() > igs_without.abs(),
            "JS2 source branch should increase forward gate current magnitude"
        );
        assert!(
            igd_with.abs() > igd_without.abs(),
            "JS2 drain branch should increase forward gate current magnitude"
        );
    }

    #[test]
    fn test_hfet_calculate_reverse_vds_flips_current_sign() {
        use std::collections::HashMap;

        let mut model = HashMap::new();
        model.insert("VT0".to_string(), 0.3);
        model.insert("ETA".to_string(), 1.32);
        model.insert("SIGMA0".to_string(), 0.04);
        model.insert("VSIGMA".to_string(), 0.1);
        model.insert("VSIGMAT".to_string(), 0.3);
        model.insert("MU".to_string(), 0.385);
        model.insert("VS".to_string(), 1.5e5);
        model.insert("NMAX".to_string(), 6e15);
        model.insert("M".to_string(), 2.57);
        model.insert("LAMBDA".to_string(), 0.17);

        let z = Jfet::njf("Z1", 1, 2, 0)
            .enable_hfet_model()
            .with_model_params(&model)
            .with_instance_params(&[("W".to_string(), 10e-6), ("L".to_string(), 1e-6)]);

        let (ids_fwd, gm_fwd, gds_fwd) = z.calculate(0.0, 0.3, 300.0);
        let (ids_rev, gm_rev, gds_rev) = z.calculate(0.0, -0.3, 300.0);
        assert!(ids_fwd.is_finite() && gm_fwd.is_finite() && gds_fwd.is_finite());
        assert!(ids_rev.is_finite() && gm_rev.is_finite() && gds_rev.is_finite());
        assert!(ids_fwd > 0.0, "forward HFET current should be positive");
        assert!(ids_rev < 0.0, "reverse HFET current should flip sign");
        assert!(
            gm_rev > 0.0,
            "reverse HFET gm should keep forward sign, got {}",
            gm_rev
        );

        let tol = 1e-9;
        assert!(
            (ids_rev + ids_fwd).abs() <= tol * ids_fwd.abs().max(1.0),
            "reverse Ids should be forward Ids with sign flip"
        );
        assert!(
            (gm_rev - gm_fwd).abs() <= tol * gm_fwd.abs().max(1.0),
            "reverse gm should match forward gm"
        );
        assert!(
            (gds_rev - gds_fwd).abs() <= tol * gds_fwd.abs().max(1.0),
            "reverse gds should match forward gds"
        );
    }

    #[test]
    fn test_hfet_calculate_reverse_vds_flips_ids_for_pjf_and_keeps_derivatives() {
        use std::collections::HashMap;

        let mut model = HashMap::new();
        model.insert("VT0".to_string(), 0.3);
        model.insert("ETA".to_string(), 1.32);
        model.insert("SIGMA0".to_string(), 0.04);
        model.insert("VSIGMA".to_string(), 0.1);
        model.insert("VSIGMAT".to_string(), 0.3);
        model.insert("MU".to_string(), 0.385);
        model.insert("VS".to_string(), 1.5e5);
        model.insert("NMAX".to_string(), 6e15);
        model.insert("M".to_string(), 2.57);
        model.insert("LAMBDA".to_string(), 0.17);

        let z = Jfet::pjf("Z1", 1, 2, 0)
            .enable_hfet_model()
            .with_model_params(&model)
            .with_instance_params(&[("W".to_string(), 10e-6), ("L".to_string(), 1e-6)]);

        // For PJF, positive external Vds maps to negative internal Vds.
        // Compare against the same internal-control forward branch by flipping
        // external Vds sign while keeping Vgs fixed.
        let (ids_rev, gm_rev, gds_rev) = z.calculate(0.0, 0.3, 300.0);
        let (ids_fwd, gm_fwd, gds_fwd) = z.calculate(0.0, -0.3, 300.0);
        assert!(ids_rev.is_finite() && gm_rev.is_finite() && gds_rev.is_finite());
        assert!(ids_fwd.is_finite() && gm_fwd.is_finite() && gds_fwd.is_finite());

        let tol = 1e-9;
        assert!(
            (ids_rev + ids_fwd).abs() <= tol * ids_fwd.abs().max(1.0),
            "reverse Ids should be forward Ids with sign flip for PJF"
        );
        assert!(
            (gm_rev - gm_fwd).abs() <= tol * gm_fwd.abs().max(1.0),
            "reverse gm should match forward gm for PJF"
        );
        assert!(
            (gds_rev - gds_fwd).abs() <= tol * gds_fwd.abs().max(1.0),
            "reverse gds should match forward gds for PJF"
        );
    }

    #[test]
    fn test_local_inverse_caps_symmetry() {
        use std::collections::HashMap;

        let mut model = HashMap::new();
        model.insert("LEVEL".to_string(), 5.0);
        model.insert("VT0".to_string(), 0.3);
        model.insert("ETA".to_string(), 1.32);
        model.insert("SIGMA0".to_string(), 0.04);
        model.insert("VSIGMA".to_string(), 0.1);
        model.insert("VSIGMAT".to_string(), 0.3);
        model.insert("MU".to_string(), 0.385);
        model.insert("VS".to_string(), 1.5e5);
        model.insert("NMAX".to_string(), 6e15);
        model.insert("M".to_string(), 2.57);
        model.insert("MC".to_string(), 2.57);

        let z = Jfet::njf("Z1", 1, 2, 0)
            .enable_hfet_model()
            .with_model_params(&model)
            .with_instance_params(&[("W".to_string(), 10e-6), ("L".to_string(), 1e-6)]);

        // Local inverse orientation: vds_int = vgs - vgd < 0.
        let vgs = 0.05;
        let vgd = 0.35;
        let (cgs_inv, cgd_inv) = z.transient_capacitances(vgs, vgd, 300.15);

        // ngspice HFET1 semantics for local inverse:
        // evaluate caps with |Vds| while keeping Vgs control, then swap outputs.
        let vgd_forward = vgs - (vgs - vgd).abs();
        let (cgs_fwd, cgd_fwd) = z.transient_capacitances(vgs, vgd_forward, 300.15);
        let rel = |a: f64, b: f64| (a - b).abs() / a.abs().max(b.abs()).max(1e-30);

        assert!(
            rel(cgs_inv, cgd_fwd) < 1e-12,
            "inverse cgs should match forward-eval cgd"
        );
        assert!(
            rel(cgd_inv, cgs_fwd) < 1e-12,
            "inverse cgd should match forward-eval cgs"
        );
    }

    #[test]
    fn test_hfet_legacy_inverse_cap_swap_matches_ngspice_single_swap_semantics() {
        use std::collections::HashMap;

        let mut model = HashMap::new();
        model.insert("VT0".to_string(), 0.3);
        model.insert("ETA".to_string(), 1.32);
        model.insert("SIGMA0".to_string(), 0.04);
        model.insert("VSIGMA".to_string(), 0.1);
        model.insert("VSIGMAT".to_string(), 0.3);
        model.insert("MU".to_string(), 0.385);
        model.insert("VS".to_string(), 1.5e5);
        model.insert("NMAX".to_string(), 6e15);
        model.insert("M".to_string(), 2.57);
        model.insert("MC".to_string(), 2.57);
        model.insert("LAMBDA".to_string(), 0.17);

        let mut z = Jfet::njf("Z1", 1, 2, 0)
            .enable_hfet_model()
            .with_model_params(&model)
            .with_instance_params(&[("W".to_string(), 10e-6), ("L".to_string(), 1e-6)]);

        // Forward-oriented local branch (vds_int > 0).
        let vgs_fwd = 0.0;
        let vgd_fwd = -0.25;
        let (cgs_fwd, cgd_fwd) = z.transient_capacitances(vgs_fwd, vgd_fwd, 300.15);
        assert!(cgs_fwd.is_finite() && cgd_fwd.is_finite());

        // Locally inverse branch (vds_int < 0): local model path already swaps.
        let vgs_inv = 0.0;
        let vgd_inv = 0.25;
        let (cgs_inv, cgd_inv) = z.transient_capacitances(vgs_inv, vgd_inv, 300.15);
        assert!(cgs_inv.is_finite() && cgd_inv.is_finite());

        z.set_hfet_legacy_inverse_active(true);
        let (cgs_fwd_legacy, cgd_fwd_legacy) = z.transient_capacitances(vgs_fwd, vgd_fwd, 300.15);
        let (cgs_inv_legacy, cgd_inv_legacy) = z.transient_capacitances(vgs_inv, vgd_inv, 300.15);

        let rel = |a: f64, b: f64| (a - b).abs() / a.abs().max(b.abs()).max(1.0);

        // Legacy latch should swap forward-oriented caps once.
        assert!(
            rel(cgs_fwd_legacy, cgd_fwd) < 1e-12 && rel(cgd_fwd_legacy, cgs_fwd) < 1e-12,
            "legacy inverse should swap forward caps exactly once"
        );

        // If local inverse already swapped, legacy latch must NOT swap again.
        assert!(
            rel(cgs_inv_legacy, cgs_inv) < 1e-12 && rel(cgd_inv_legacy, cgd_inv) < 1e-12,
            "legacy inverse must not double-swap local-inverse caps"
        );
    }

    #[test]
    fn test_mesa_model_ignores_hfet1_legacy_inverse_latch() {
        use std::collections::HashMap;

        let mut model = HashMap::new();
        model.insert("LEVEL".to_string(), 2.0);
        model.insert("VT0".to_string(), 0.15);
        model.insert("ETA".to_string(), 1.44);
        model.insert("SIGMA0".to_string(), 0.02);
        model.insert("VSIGMAT".to_string(), 0.5);
        model.insert("VSIGMA".to_string(), 0.1);
        model.insert("VS".to_string(), 1.9e5);
        model.insert("MU".to_string(), 0.25);
        model.insert("D".to_string(), 1e-7);
        model.insert("M".to_string(), 2.0);
        model.insert("LAMBDA".to_string(), 0.15);

        let mut z = Jfet::njf("Z1", 1, 2, 0)
            .enable_mesa_model()
            .with_model_params(&model)
            .with_instance_params(&[("W".to_string(), 20e-6), ("L".to_string(), 0.7e-6)]);

        // Forward-oriented local branch (vds_int > 0).
        let vgs_fwd = 0.0;
        let vgd_fwd = -0.25;
        let (cgs_fwd, cgd_fwd) = z.transient_capacitances(vgs_fwd, vgd_fwd, 300.15);
        assert!(cgs_fwd.is_finite() && cgd_fwd.is_finite());

        // Locally inverse branch (vds_int < 0): local model path already swaps.
        let vgs_inv = 0.0;
        let vgd_inv = 0.25;
        let (cgs_inv, cgd_inv) = z.transient_capacitances(vgs_inv, vgd_inv, 300.15);
        assert!(cgs_inv.is_finite() && cgd_inv.is_finite());

        z.set_hfet_legacy_inverse_active(true);
        let (cgs_fwd_legacy, cgd_fwd_legacy) = z.transient_capacitances(vgs_fwd, vgd_fwd, 300.15);
        let (cgs_inv_legacy, cgd_inv_legacy) = z.transient_capacitances(vgs_inv, vgd_inv, 300.15);

        let rel = |a: f64, b: f64| (a - b).abs() / a.abs().max(b.abs()).max(1.0);

        // MESA mode should ignore HFET1 legacy latch forcing.
        assert!(
            rel(cgs_fwd_legacy, cgs_fwd) < 1e-12 && rel(cgd_fwd_legacy, cgd_fwd) < 1e-12,
            "MESA forward caps should be unchanged by legacy latch state"
        );

        // Locally inverse path should also be unaffected.
        assert!(
            rel(cgs_inv_legacy, cgs_inv) < 1e-12 && rel(cgd_inv_legacy, cgd_inv) < 1e-12,
            "MESA inverse caps should be unchanged by legacy latch state"
        );
    }

    #[test]
    fn test_is_converged_uses_relative_voltage_criterion() {
        let criteria = NonlinearConvergenceCriteria::voltage_only(1e-6);
        let mut jfet = Jfet::njf("J1", 1, 2, 0);
        jfet.vgs_prev = 1.0;
        jfet.vgs = 1.0005;
        jfet.vds_prev = 2.0;
        jfet.vds = 2.0008;

        assert!(
            jfet.is_converged(criteria),
            "relative tolerance should allow sub-millivolt deltas around 1-2V biases"
        );
    }

    #[test]
    fn test_is_converged_rejects_large_branch_delta() {
        let criteria = NonlinearConvergenceCriteria::voltage_only(1e-6);
        let mut jfet = Jfet::njf("J1", 1, 2, 0);
        jfet.vgs_prev = 1.0;
        jfet.vgs = 1.01;
        jfet.vds_prev = 2.0;
        jfet.vds = 2.0;

        assert!(
            !jfet.is_converged(criteria),
            "large branch-voltage jump must fail convergence"
        );
    }

    #[test]
    fn test_is_converged_rejects_non_finite_history() {
        let criteria = NonlinearConvergenceCriteria::voltage_only(1e-6);
        let mut jfet = Jfet::njf("J1", 1, 2, 0);
        jfet.vgs_prev = f64::NAN;
        jfet.vgs = 0.0;
        jfet.vds_prev = 0.0;
        jfet.vds = 0.0;

        assert!(
            !jfet.is_converged(criteria),
            "non-finite branch history must force another Newton update"
        );
    }

    #[test]
    fn test_is_converged_rejects_when_limiter_was_applied() {
        let criteria = NonlinearConvergenceCriteria::voltage_only(1e-6);
        let mut jfet = Jfet::njf("J1", 1, 2, 0);
        jfet.vgs_prev = 1.0;
        jfet.vgs = 1.0001;
        jfet.vds_prev = 2.0;
        jfet.vds = 2.0001;
        jfet.limiter_applied = true;

        assert!(
            !jfet.is_converged(criteria),
            "limiter-applied flag must force another Newton iteration"
        );
    }

    #[test]
    fn test_mesa_update_sets_limiter_applied_on_large_forward_step() {
        let mut jfet = Jfet::njf("Z1", 1, 2, 3).enable_mesa_model();
        // Seed finite previous state so pnjlim/fetlim operate in iterative mode.
        jfet.vgs = 0.0;
        jfet.vds = 0.0;

        // Node order is [drain, gate, source] for node ids 1..3.
        let voltages = [0.0, 3.0, 0.0];
        jfet.update(&voltages);

        assert!(
            jfet.limiter_applied,
            "large forward branch jump should trigger pnjlim/fetlim limiting"
        );
    }
}
