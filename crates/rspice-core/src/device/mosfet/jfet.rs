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

mod bias;
mod capacitance;
mod construction;
mod mesa;
mod params;
mod stamping;

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
    /// HFET1 `GATEMOD` selector for the optional gate-current equations.
    pub hfet_gatemod: bool,
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
    /// HFET1 GATEMOD=1 knee-voltage slope `CK1`.
    pub hfet_ck1: Value,
    /// HFET1 GATEMOD=1 knee-voltage offset `CK2` (V).
    pub hfet_ck2: Value,
    /// HFET1 GATEMOD=1 critical-voltage slope `CM1`.
    pub hfet_cm1: Value,
    /// HFET1 GATEMOD=1 critical-voltage offset `CM2` (V).
    pub hfet_cm2: Value,
    /// HFET1 GATEMOD=1 correction critical-voltage slope `CM3`.
    pub hfet_cm3: Value,
    /// HFET1 GATEMOD=1 knee shape exponent `MT1`.
    pub hfet_mt1: Value,
    /// HFET1 GATEMOD=1 drain-voltage fold exponent `MT2`.
    pub hfet_mt2: Value,
    /// HFET1 GATEMOD=1 correction fold exponent `MV1`.
    pub hfet_mv1: Value,
    /// HFET1 GATEMOD=1 channel-heating coefficient `TALPHA` (K/V^2).
    pub hfet_talpha: Value,
    /// HFET1 GATEMOD=1 Schottky barrier height `PHIB` (J).
    pub hfet_phib: Value,
    /// HFET1 GATEMOD=1 drain-current correction gain `A1`.
    pub hfet_a1: Value,
    /// HFET1 GATEMOD=1 drain-current correction coefficient `A2` (1/V^2).
    pub hfet_a2: Value,
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
    /// Builder-resolved thermal-noise temperature offset in kelvin
    /// (jfetnoi.c `dtemp` semantics).
    pub noise_dtemp: Value,
    /// Optional instance source terminal temperature override.
    instance_ts: Option<Value>,
    /// Optional instance drain terminal temperature override.
    instance_td: Option<Value>,
    /// Previous/current iteration state for convergence checks
    vgs: Value,
    vds: Value,
    vgs_prev: Value,
    vds_prev: Value,
    last_raw_vgs_prev: Value,
    last_raw_vgd_prev: Value,
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
    /// GATEMOD=1 gate-drain current sensitivity to vgs (zero otherwise).
    eval_gmg: Value,
    /// GATEMOD=1 gate-drain current sensitivity to vds (zero otherwise).
    eval_gmd: Value,
    eval_vds_linear: Value,
    lin_vgs: Value,
    lin_vgd: Value,
    lin_cg: Value,
    lin_cd: Value,
    model_order: usize,
    hfet_legacy_inverse_mode: bool,
    hfet_legacy_inverse_active: bool,
    /// Device-local junction GMIN used by ngspice-style gate diode loading.
    junction_gmin: Value,
    /// Continuation scale for stiff gate generation-recombination branches.
    gate_generation_scale: Value,
    /// Pre-computed matrix indices for O(1) direct stamping
    pub indices: JfetIndices,
}
