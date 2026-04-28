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
    /// Berkeley SPICE level-1 MESFET equations (`mes` device).
    LegacyMesfet,
    /// HFET1-compatible MESFET channel equations (ngspice-derived).
    Hfet1,
}

#[derive(Debug, Clone, Copy)]
struct MesaLevel2Linearization {
    ids: Value,
    gm: Value,
    gds: Value,
    vds: Value,
    delidgch0: Value,
    delidvds0: Value,
    delidvds1: Value,
    gm0: Value,
    gm1: Value,
    gm2: Value,
    gds0: Value,
}

impl MesaLevel2Linearization {
    fn zero() -> Self {
        Self {
            ids: 0.0,
            gm: 0.0,
            gds: 0.0,
            vds: 0.0,
            delidgch0: 0.0,
            delidvds0: 0.0,
            delidvds1: 0.0,
            gm0: 0.0,
            gm1: 0.0,
            gm2: 0.0,
            gds0: 0.0,
        }
    }

    fn dc_terms(self) -> (Value, Value, Value) {
        (self.ids, self.gm, self.gds)
    }

    fn ac_conductances(self, lambda: Value) -> (Value, Value) {
        let delidgch = self.delidgch0 * (1.0 + lambda * self.vds);
        let delidvds = self.delidvds0 * (1.0 + 2.0 * lambda * self.vds) - self.delidvds1;
        let gm = (delidgch * self.gm0 + self.gm1) * self.gm2;
        let gds = delidvds + self.gds0;
        (
            if gm.is_finite() { gm } else { 0.0 },
            if gds.is_finite() { gds } else { 0.0 },
        )
    }
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
    /// HFET AC output-conductance shaping coefficient `KAPPA`.
    pub hfet_kappa: Value,
    /// HFET AC output-conductance transition width `DELF`.
    pub hfet_delf_freq: Value,
    /// HFET AC output-conductance corner frequency `FGDS`.
    pub hfet_fgds: Value,
    /// HFET AC temperature-shaping denominator `TF` (K).
    pub hfet_tf: Value,
    /// HFET drain-source capacitance `CDS`.
    pub hfet_cds: Value,
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
    /// Berkeley MESFET denominator coefficient `B`.
    pub mes_b: Value,
    /// MESA current softening coefficient `TC`.
    pub mesa_tc: Value,
    /// MESA transport factor `ZETA`.
    pub mesa_zeta: Value,
    /// MESA high-frequency channel-length modulation `LAMBDAHF`.
    pub mesa_lambdahf: Value,
    /// MESA AC temperature-shaping denominator `TF` (K).
    pub mesa_tf: Value,
    /// MESA AC channel-length corner frequency `FLO`.
    pub mesa_flo: Value,
    /// MESA AC channel-length transition width `DELFO`.
    pub mesa_delfo: Value,
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
    last_raw_vgs: Value,
    last_raw_vgd: Value,
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

    #[inline]
    fn matches_last_raw_branch_input(&self, vgs_raw: Value, vgd_raw: Value) -> bool {
        self.eval_valid
            && self.last_raw_vgs.is_finite()
            && self.last_raw_vgd.is_finite()
            && vgs_raw == self.last_raw_vgs
            && vgd_raw == self.last_raw_vgd
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

    #[inline]
    fn temperature_shape_scale(temp_k: Value, tf_k: Value) -> Value {
        if !temp_k.is_finite() || temp_k <= 0.0 || !tf_k.is_finite() || tf_k.abs() < 1e-18 {
            return 1.0;
        }
        (temp_k / tf_k.abs()).clamp(-80.0, 80.0).exp()
    }

    #[inline]
    fn mesa_ac_lambda(&self, temp_k: Value, frequency_hz: Option<Value>) -> Value {
        let lambda_lo = self.params.lambda;
        let lambda_hi = if self.params.mesa_lambdahf.is_finite() {
            self.params.mesa_lambdahf
        } else {
            lambda_lo
        };
        let Some(frequency_hz) = frequency_hz.filter(|f| f.is_finite() && *f >= 0.0) else {
            return lambda_lo;
        };

        let transition = self.params.mesa_delfo.abs();
        if transition <= 0.0 {
            return lambda_lo;
        }

        let scale = Self::temperature_shape_scale(temp_k, self.params.mesa_tf);
        let flo = self.params.mesa_flo.max(0.0) * scale;
        let delf = transition * scale;
        lambda_lo + 0.5 * (lambda_hi - lambda_lo) * (1.0 + ((frequency_hz - flo) / delf).tanh())
    }

    #[inline]
    fn hfet_ac_gds_scale(&self, temp_k: Value, frequency_hz: Value) -> Value {
        if !frequency_hz.is_finite() || frequency_hz < 0.0 {
            return 1.0;
        }
        if self.params.hfet_kappa == 0.0 {
            return 1.0;
        }
        let transition = self.params.hfet_delf_freq.abs();
        if transition <= 0.0 {
            return 1.0;
        }

        let scale = Self::temperature_shape_scale(temp_k, self.params.hfet_tf);
        let fgds = self.params.hfet_fgds.max(0.0) * scale;
        let delf = transition * scale;
        1.0 + 0.5 * self.params.hfet_kappa * (1.0 + ((frequency_hz - fgds) / delf).tanh())
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

    /// Calculate Berkeley SPICE level-1 MESFET drain current and conductances.
    fn calculate_legacy_mesfet(&self, vgs: Value, vds: Value) -> (Value, Value, Value) {
        let pol = self.jfet_type.polarity();
        let vgs_int = pol * vgs;
        let vds_int = pol * vds;
        let vgd_int = vgs_int - vds_int;
        let beta = (self.params.beta * self.area * self.m).max(0.0);
        let lambda = self.params.lambda;
        let alpha = if self.params.mesa_alpha.is_finite() && self.params.mesa_alpha.abs() > 1.0e-30
        {
            self.params.mesa_alpha
        } else {
            2.0
        };
        let b = self.params.mes_b.max(0.0);
        let vto = self.params.vto;

        let (ids, gm, gds) = if vds_int >= 0.0 {
            let vgst = vgs_int - vto;
            if vgst <= 0.0 {
                (0.0, 0.0, 0.0)
            } else {
                let prod = 1.0 + lambda * vds_int;
                let betap = beta * prod;
                let denom = (1.0 + b * vgst).max(1.0e-30);
                let inv_denom = 1.0 / denom;
                let vgst2_over_denom = vgst * vgst * inv_denom;
                let gm_sat = betap * vgst * (1.0 + denom) * inv_denom * inv_denom;
                if vds_int >= 3.0 / alpha {
                    (
                        betap * vgst2_over_denom,
                        gm_sat,
                        lambda * beta * vgst2_over_denom,
                    )
                } else {
                    let afact = 1.0 - alpha * vds_int / 3.0;
                    let lfact = 1.0 - afact * afact * afact;
                    (
                        betap * vgst2_over_denom * lfact,
                        gm_sat * lfact,
                        beta * vgst2_over_denom * (alpha * afact * afact * prod + lfact * lambda),
                    )
                }
            }
        } else {
            let vgdt = vgd_int - vto;
            if vgdt <= 0.0 {
                (0.0, 0.0, 0.0)
            } else {
                let prod = 1.0 - lambda * vds_int;
                let betap = beta * prod;
                let denom = (1.0 + b * vgdt).max(1.0e-30);
                let inv_denom = 1.0 / denom;
                let vgdt2_over_denom = vgdt * vgdt * inv_denom;
                let gm_sat = -betap * vgdt * (1.0 + denom) * inv_denom * inv_denom;
                if -vds_int >= 3.0 / alpha {
                    (
                        -betap * vgdt2_over_denom,
                        gm_sat,
                        lambda * beta * vgdt2_over_denom - gm_sat,
                    )
                } else {
                    let afact = 1.0 + alpha * vds_int / 3.0;
                    let lfact = 1.0 - afact * afact * afact;
                    (
                        -betap * vgdt2_over_denom * lfact,
                        gm_sat * lfact,
                        beta * vgdt2_over_denom * (alpha * afact * afact * prod + lfact * lambda)
                            - gm_sat * lfact,
                    )
                }
            }
        };

        (
            if ids.is_finite() { pol * ids } else { 0.0 },
            if gm.is_finite() { gm } else { 0.0 },
            if gds.is_finite() { gds } else { 0.0 },
        )
    }

    #[inline]
    fn exp_limited(x: Value) -> Value {
        x.clamp(-80.0, 80.0).exp()
    }

    fn mesa_level2_ids(
        &self,
        vgs: Value,
        vds: Value,
        temp_k: Value,
        vto: Value,
        lambda: Value,
    ) -> Value {
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
        let ids = gch * vds * (1.0 + lambda * vds) / h;
        if ids.is_finite() { ids } else { 0.0 }
    }

    fn mesa_level2_linearization_forward(
        &self,
        vgs: Value,
        vds: Value,
        temp_k: Value,
        vto: Value,
        lambda: Value,
    ) -> MesaLevel2Linearization {
        const Q_ELECTRON: Value = 1.602176634e-19;
        const EPSILONGAAS: Value = 12.244 * 8.85418e-12;

        let p = &self.params;
        let w = self.width.max(1e-12);
        let l = self.length.max(1e-12);
        let d = p.hfet_di.max(1e-12);
        let nd = p.mesa_nd.max(1e-30);
        let vpo = Q_ELECTRON * nd * d * d / (2.0 * EPSILONGAAS);
        let vt = self.thermal_voltage(temp_k).max(1e-12);
        let eta = p.eta.abs().max(1e-12);
        let etavth = (eta * vt).max(1e-12);
        let rsi = p.hfet_rsi.max(0.0);
        let rt = rsi + p.hfet_rdi.max(0.0);
        let vsigma = p.hfet_vsigma.max(1e-12);

        let vgt0 = vgs - vto;
        let s = Self::exp_limited((vgt0 - p.hfet_vsigmat) / vsigma);
        let sigma = p.sigma0.max(0.0) / (1.0 + s);
        let vgt = vgt0 + sigma * vds;
        let mu = (p.hfet_mu + p.mesa_theta * vgt).max(1e-12);
        let vl = (p.hfet_vs.max(1e-12) / mu * l).max(1e-30);
        let beta_num = 2.0 * EPSILONGAAS * p.hfet_vs.max(1e-12) * p.mesa_zeta.max(1e-12) * w / d;
        let beta = beta_num / (vpo + 3.0 * vl).max(1e-30);

        let u = vgt / vt - 1.0;
        let t = (p.hfet_delta.max(1e-9).powi(2) + u * u).sqrt();
        let vgte = 0.5 * vt * (2.0 + u + t);
        let b = Self::exp_limited(-vgt / etavth);

        let sqrt1 = if vgte > vpo {
            0.0
        } else {
            (1.0 - vgte / vpo.max(1e-30)).max(0.0).sqrt()
        };
        let q = (1.0 - sqrt1).max(1e-30);
        let n0 = (EPSILONGAAS * eta * vt / (Q_ELECTRON * d)).max(1e-30);
        let ns = 1.0 / (1.0 / (nd * d * q).max(1e-30) + b / n0);
        if !ns.is_finite() || ns < 1e-38 {
            return MesaLevel2Linearization::zero();
        }

        let gchi0 = Q_ELECTRON * w / l;
        let gchi = gchi0 * mu * ns;
        let gch = gchi / (1.0 + gchi * rt);
        if !gch.is_finite() || gch <= 0.0 {
            return MesaLevel2Linearization::zero();
        }

        let a = 2.0 * beta * vgte;
        let f = (1.0 + 2.0 * a * rsi).sqrt();
        let d_term = 1.0 + a * rsi + f;
        let e_term = 1.0 + p.mesa_tc * vgte;
        let isata = a * vgte / (d_term * e_term).max(1e-30);
        let isatb0 = Q_ELECTRON * n0 * vt * w / l;
        let isatb = isatb0 * mu * Self::exp_limited(vgt / etavth);
        let isat_sum = (isata + isatb).max(1e-30);
        let isat = isata * isatb / isat_sum;
        let vsate = (isat / gch).abs().max(1e-30);
        let m = (p.hfet_m + p.mesa_alpha * vgte).max(1e-9);
        let g = (vds / vsate).max(0.0).powf(m);
        let h = (1.0 + g).powf(1.0 / m);

        let delidgch0 = vds / h;
        let delidgch = delidgch0 * (1.0 + lambda * vds);
        let ids = gch * delidgch;

        let delgchgchi = 1.0 / (1.0 + gchi * rt).powi(2);
        let delgchins = gchi0 * mu;
        let delnsvgt = ns * ns * (1.0 / n0) / etavth * b;
        let delnsvgte = if sqrt1 == 0.0 {
            0.0
        } else {
            0.5 * ns * ns / (vpo.max(1e-30) * nd * d * sqrt1 * q * q)
        };
        let delvgtevgt = 0.5 * (1.0 + u / t.max(1e-30));
        let delidvds0 = gch / h;
        let delidvds1 = if vds != 0.0 {
            ids * (vds / vsate).powf(m - 1.0) / (vsate * (1.0 + g))
        } else {
            0.0
        };
        let delidvds = delidvds0 * (1.0 + 2.0 * lambda * vds) - delidvds1;
        let delidvsate = ids * g / (vsate * (1.0 + g));
        let delvsateisat = 1.0 / gch;
        let isat_sq = isat_sum * isat_sum;
        let delisatisata = isatb * isatb / isat_sq;
        let v_term = 1.0 + 1.0 / f;
        let ddevgte = 2.0 * beta * rsi * v_term * e_term + d_term * p.mesa_tc;
        let denom = (d_term * d_term * e_term * e_term).max(1e-30);
        let delisatavgte = (2.0 * a * d_term * e_term - a * vgte * ddevgte) / denom;
        let delisatabeta =
            2.0 * vgte * vgte * (d_term * e_term - a * e_term * rsi * v_term) / denom;
        let delisatisatb = isata * isata / isat_sq;
        let delvsategch = -vsate / gch;
        let dvgtvgs = 1.0 - p.sigma0.max(0.0) * vds * s / (vsigma * (1.0 + s).powi(2));
        let theta_term = gchi0 * ns * p.mesa_theta;
        let dgchivgt = delgchins * (delnsvgte * delvgtevgt + delnsvgt) + theta_term;
        let dvgtevds = delvgtevgt * sigma;
        let dgchivds = delgchins * (delnsvgte * dvgtevds + delnsvgt * sigma) + theta_term * sigma;
        let beta_theta_term =
            delisatabeta * 3.0 * beta * vl * p.mesa_theta / (mu * (vpo + 3.0 * vl).max(1e-30));
        let disatavgt = delisatavgte * delvgtevgt + beta_theta_term;
        let disatavds = delisatavgte * dvgtevds + beta_theta_term * sigma;
        let disatbvgt = isatb / etavth + isatb / mu * p.mesa_theta;
        let p_term = delgchgchi * dgchivgt;
        let w_term = delgchgchi * dgchivds;
        let dvsatevgt = delvsateisat * (delisatisata * disatavgt + delisatisatb * disatbvgt)
            + delvsategch * p_term;
        let dvsatevds = delvsateisat
            * (delisatisata * disatavds + delisatisatb * disatbvgt * sigma)
            + delvsategch * w_term;

        let (gmmadd, gdsmadd) = if p.mesa_alpha != 0.0 && vds != 0.0 {
            let gmmadd = ids
                * ((1.0 + g).ln() / (m * m) - g * (vds / vsate).ln() / (m * (1.0 + g)))
                * p.mesa_alpha
                * delvgtevgt;
            (gmmadd, gmmadd * sigma)
        } else {
            (0.0, 0.0)
        };

        let gm1 = delidvsate * dvsatevgt;
        let gm = (delidgch * p_term + gm1 + gmmadd) * dvgtvgs;
        let gds0 = delidvsate * dvsatevds + delidgch * w_term + gdsmadd;
        let gds = delidvds + gds0;

        MesaLevel2Linearization {
            ids: if ids.is_finite() { ids } else { 0.0 },
            gm: if gm.is_finite() { gm } else { 0.0 },
            gds: if gds.is_finite() { gds } else { 0.0 },
            vds,
            delidgch0,
            delidvds0,
            delidvds1,
            gm0: p_term,
            gm1,
            gm2: dvgtvgs,
            gds0,
        }
    }

    fn mesa_level2_small_signal_forward(
        &self,
        vgs: Value,
        vds: Value,
        temp_k: Value,
        vto: Value,
        lambda: Value,
    ) -> (Value, Value, Value) {
        self.mesa_level2_linearization_forward(vgs, vds, temp_k, vto, lambda)
            .dc_terms()
    }

    fn mesa_level2_ac_conductances_forward(
        &self,
        vgs: Value,
        vds: Value,
        temp_k: Value,
        vto: Value,
        ac_lambda: Value,
    ) -> (Value, Value) {
        let dc_lambda = self.mesa_ac_lambda(temp_k, None);
        self.mesa_level2_linearization_forward(vgs, vds, temp_k, vto, dc_lambda)
            .ac_conductances(ac_lambda)
    }

    fn mesa_level3_ids(
        &self,
        vgs: Value,
        vds: Value,
        temp_k: Value,
        vto: Value,
        lambda: Value,
    ) -> Value {
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
        let ids = gch * vds * (1.0 + lambda * vds) / h;
        if ids.is_finite() { ids } else { 0.0 }
    }

    fn mesa_level4_ids(
        &self,
        vgs: Value,
        vds: Value,
        temp_k: Value,
        vto: Value,
        lambda: Value,
    ) -> Value {
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
        let ids = gch * vds * (1.0 + lambda * vds) / e;
        if ids.is_finite() { ids } else { 0.0 }
    }

    fn mesa_ids_external(
        &self,
        vgs: Value,
        vds: Value,
        temp: Value,
        level: i32,
        force_inverse: bool,
        frequency_hz: Option<Value>,
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
        let lambda = self.mesa_ac_lambda(temp_k, frequency_hz);
        let mut ids_int = match level {
            2 => self.mesa_level2_ids(vgs_int, vds_int, temp_k, vto, lambda),
            3 => self.mesa_level3_ids(vgs_int, vds_int, temp_k, vto, lambda),
            _ => self.mesa_level4_ids(vgs_int, vds_int, temp_k, vto, lambda),
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
        if level == 2 && !force_inverse {
            let pol = self.jfet_type.polarity();
            let vgs_int = pol * vgs;
            let vds_int = pol * vds;
            if vds_int >= 0.0 {
                let temp_k = if temp.is_finite() && temp > 0.0 {
                    temp
                } else {
                    self.params.tnom.max(1.0)
                };
                let vto = pol * self.params.vto;
                let lambda = self.mesa_ac_lambda(temp_k, None);
                let (ids_int, gm, gds) =
                    self.mesa_level2_small_signal_forward(vgs_int, vds_int, temp_k, vto, lambda);
                return (pol * ids_int, gm, gds);
            }
        }

        let ids = self.mesa_ids_external(vgs, vds, temp, level, force_inverse, None);

        let dvgs = (1e-8_f64).max(1e-6 * (1.0 + vgs.abs()));
        let dvds = (1e-8_f64).max(1e-6 * (1.0 + vds.abs()));
        let gm = (self.mesa_ids_external(vgs + dvgs, vds, temp, level, force_inverse, None)
            - self.mesa_ids_external(vgs - dvgs, vds, temp, level, force_inverse, None))
            / (2.0 * dvgs);
        let gds = (self.mesa_ids_external(vgs, vds + dvds, temp, level, force_inverse, None)
            - self.mesa_ids_external(vgs, vds - dvds, temp, level, force_inverse, None))
            / (2.0 * dvds);

        (
            if ids.is_finite() { ids } else { 0.0 },
            if gm.is_finite() { gm } else { 0.0 },
            if gds.is_finite() { gds } else { 0.0 },
        )
    }

    fn calculate_mesa_level_ac(
        &self,
        vgs: Value,
        vds: Value,
        temp: Value,
        level: i32,
        force_inverse: bool,
        frequency_hz: Value,
    ) -> (Value, Value, Value) {
        let ids = self.mesa_ids_external(vgs, vds, temp, level, force_inverse, Some(frequency_hz));

        let dvgs = (1e-8_f64).max(1e-6 * (1.0 + vgs.abs()));
        let dvds = (1e-8_f64).max(1e-6 * (1.0 + vds.abs()));
        let gm = (self.mesa_ids_external(
            vgs + dvgs,
            vds,
            temp,
            level,
            force_inverse,
            Some(frequency_hz),
        ) - self.mesa_ids_external(
            vgs - dvgs,
            vds,
            temp,
            level,
            force_inverse,
            Some(frequency_hz),
        )) / (2.0 * dvgs);
        let gds = (self.mesa_ids_external(
            vgs,
            vds + dvds,
            temp,
            level,
            force_inverse,
            Some(frequency_hz),
        ) - self.mesa_ids_external(
            vgs,
            vds - dvds,
            temp,
            level,
            force_inverse,
            Some(frequency_hz),
        )) / (2.0 * dvds);

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
            JfetChannelModel::LegacyMesfet => self.calculate_legacy_mesfet(vgs, vds),
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
            JfetChannelModel::ShichmanHodges | JfetChannelModel::LegacyMesfet => {
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

    pub(crate) fn ac_capacitances(
        &self,
        vgs: Value,
        vgd: Value,
        temp: Value,
    ) -> (Value, Value, Value) {
        let cds = if matches!(self.params.channel_model, JfetChannelModel::Hfet1)
            && self.params.hfet_level >= 5
        {
            self.params.hfet_cds.max(0.0)
        } else {
            0.0
        };

        match self.params.channel_model {
            JfetChannelModel::ShichmanHodges | JfetChannelModel::LegacyMesfet => {
                let pol = self.jfet_type.polarity();
                let (cgs, cgd) = self.capacitances(pol * vgs, pol * vgd);
                (cgs, cgd, cds)
            }
            JfetChannelModel::Hfet1 => {
                let (cgs, cgd) = self.transient_capacitances(vgs, vgd, temp);
                (cgs, cgd, cds)
            }
        }
    }

    fn ac_real_terms_at_frequency(
        &self,
        vgs: Value,
        vds: Value,
        vgd: Value,
        frequency_hz: Value,
    ) -> (Value, Value, Value, Value) {
        let (temp_common, temp_source, _) = self.resolved_temperatures(self.params.tnom);
        let (_, gm_base, gds_base, _, _, ggs, ggd, _) = self.compute_operating_terms(vgs, vds, vgd);

        let (gm, gds) = match self.params.channel_model {
            JfetChannelModel::ShichmanHodges | JfetChannelModel::LegacyMesfet => {
                (gm_base, gds_base)
            }
            JfetChannelModel::Hfet1 => match self.params.hfet_level {
                2..=4 => {
                    let force_inverse = self.hfet_legacy_inverse_active && vds >= 0.0;
                    if self.params.hfet_level == 2 && !force_inverse {
                        let pol = self.jfet_type.polarity();
                        let vto = pol * self.params.vto;
                        let vgs_int = pol * vgs;
                        let vds_int = pol * vds;
                        if vds_int >= 0.0 {
                            let lambda = self.mesa_ac_lambda(temp_source, Some(frequency_hz));
                            let (gm, gds) = self.mesa_level2_ac_conductances_forward(
                                vgs_int,
                                vds_int,
                                temp_source,
                                vto,
                                lambda,
                            );
                            (gm, gds)
                        } else {
                            let (_, gm, gds) = self.calculate_mesa_level_ac(
                                vgs,
                                vds,
                                temp_source,
                                self.params.hfet_level,
                                force_inverse,
                                frequency_hz,
                            );
                            (gm, gds)
                        }
                    } else {
                        let (_, gm, gds) = self.calculate_mesa_level_ac(
                            vgs,
                            vds,
                            temp_source,
                            self.params.hfet_level,
                            force_inverse,
                            frequency_hz,
                        );
                        (gm, gds)
                    }
                }
                5.. => (
                    gm_base,
                    gds_base * self.hfet_ac_gds_scale(temp_common, frequency_hz),
                ),
                _ => (gm_base, gds_base),
            },
        };

        (gm, gds, ggs, ggd)
    }

    pub(crate) fn stamp_small_signal_ac(
        &self,
        voltages: &[Value],
        frequency_hz: Value,
        matrix: &mut impl MatrixStamper,
    ) {
        let (vgs, vds, vgd) = self.state_or_raw_branch_voltages(voltages);
        let (gm, gds, ggs, ggd) = self.ac_real_terms_at_frequency(vgs, vds, vgd, frequency_hz);

        matrix.stamp(self.drain, self.drain, gds + ggd);
        matrix.stamp(self.drain, self.gate, gm - ggd);
        matrix.stamp(self.drain, self.source, -gm - gds);

        matrix.stamp(self.gate, self.drain, -ggd);
        matrix.stamp(self.gate, self.gate, ggs + ggd);
        matrix.stamp(self.gate, self.source, -ggs);

        matrix.stamp(self.source, self.drain, -gds);
        matrix.stamp(self.source, self.gate, -gm - ggs);
        matrix.stamp(self.source, self.source, gm + gds + ggs);
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
        if self.matches_last_raw_branch_input(vgs_raw, vgd_raw) {
            if self.vgs.is_finite() && self.vds.is_finite() {
                self.vgs_prev = self.vgs;
                self.vds_prev = self.vds;
            }
            return;
        }

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
            let (vgs_limited, vgd_limited) = self.hfet_limited_branch_voltages(vgs, vgd);
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
        self.last_raw_vgs = vgs_raw;
        self.last_raw_vgd = vgd_raw;

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

