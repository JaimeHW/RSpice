//! MOSFET (Metal-Oxide-Semiconductor Field-Effect Transistor) device model
//!
//! Implements a Level 1 SPICE MOSFET model (Shichman-Hodges).
//! Supports NMOS and PMOS devices in cutoff, linear, and saturation regions.

use super::smooth::{SMOOTH_VOLTAGE, smooth_max, smooth_min, smooth_positive, smooth_step};
use crate::constants::VT_REFERENCE;
use crate::device::traits::{MatrixStamper, NonlinearConvergenceCriteria, NonlinearDevice};
use crate::solver::{CscIndex, StaticMatrix};
use crate::{Value, circuit::NodeId};

/// Separate smoothing width for Vds-dependent region transitions.
///
/// Keep this much smaller than threshold smoothing to avoid artificial channel
/// current at Vdsâ‰ˆ0 while retaining C1 continuity for Newton.
const VDS_SMOOTHING: Value = SMOOTH_VOLTAGE * 1e-1;

/// MOSFET type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MosType {
    Nmos,
    Pmos,
}

/// MOSFET operating region
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MosRegion {
    Cutoff,
    Linear,
    Saturation,
}

/// Pre-computed stamp indices for O(1) matrix access (4-terminal device, but only D and S rows)
/// Note: Gate draws no DC current so has no G row stamps
#[derive(Debug, Clone, Default)]
pub struct MosfetIndices {
    // Drain row (4 columns)
    pub dd: Option<CscIndex>,
    pub dg: Option<CscIndex>,
    pub ds: Option<CscIndex>,
    pub db: Option<CscIndex>,
    // Source row (4 columns)
    pub sd: Option<CscIndex>,
    pub sg: Option<CscIndex>,
    pub ss: Option<CscIndex>,
    pub sb: Option<CscIndex>,
}

/// MOSFET device supporting Level 1 (Shichman-Hodges) and Level 3 (BSIM3-like) models
///
/// Terminal connections:
/// - Drain (D)
/// - Gate (G)
/// - Source (S)
/// - Bulk/Body (B)
#[derive(Debug, Clone)]
pub struct Mosfet {
    pub name: String,
    pub mos_type: MosType,

    // Node connections
    pub node_drain: NodeId,
    pub node_gate: NodeId,
    pub node_source: NodeId,
    pub node_bulk: NodeId,

    // Geometry
    /// Channel length (L) in meters
    pub l: Value,
    /// Channel width (W) in meters
    pub w: Value,
    /// Lateral diffusion/channel shortening per side (LD) in meters
    pub ld: Value,

    // Model parameters (Level 1)
    /// Threshold voltage (VTO)
    pub vto: Value,
    /// Transconductance parameter (KP) in A/V^2
    pub kp: Value,
    /// Body effect coefficient (GAMMA)
    pub gamma: Value,
    /// Surface potential (PHI)
    pub phi: Value,
    /// Channel-length modulation (LAMBDA)
    pub lambda: Value,
    /// Bulk junction saturation current (IS)
    pub is_bulk: Value,
    /// Bulk junction saturation current density (JS)
    pub js_bulk: Value,
    /// Oxide capacitance per unit area (COX)
    pub cox: Value,
    /// Zero-bias bulk junction bottom capacitance density (CJ)
    pub cj: Value,
    /// Zero-bias bulk junction sidewall capacitance density (CJSW)
    pub cjsw: Value,
    /// Bulk junction built-in potential (PB)
    pub pb: Value,
    /// Bottom-junction grading coefficient (MJ)
    pub mj: Value,
    /// Sidewall grading coefficient (MJSW)
    pub mjsw: Value,
    /// Forward-bias depletion-capacitance coefficient (FC)
    pub fc: Value,
    /// Explicit zero-bias bulk-drain capacitance override (CBD/CAPBD)
    pub drain_bulk_cap_zero_bias: Option<Value>,
    /// Explicit zero-bias bulk-source capacitance override (CBS/CAPBS)
    pub source_bulk_cap_zero_bias: Option<Value>,
    /// Drain diffusion area (AD)
    pub drain_area: Value,
    /// Source diffusion area (AS)
    pub source_area: Value,
    /// Drain perimeter (PD)
    pub drain_perimeter: Value,
    /// Source perimeter (PS)
    pub source_perimeter: Value,
    /// Device-local junction GMIN used by ngspice-style bulk diode loading.
    pub junction_gmin: Value,

    // BSIM3-like parameters for short-channel effects
    /// Model level (1 = Level 1, 3 = BSIM3-like)
    pub level: i32,
    /// Mobility at low field (U0) in cm^2/V*s
    pub u0: Value,
    /// First-order mobility degradation (UA)
    pub ua: Value,
    /// Second-order mobility degradation (UB)
    pub ub: Value,
    /// Velocity saturation (VSAT) in m/s
    pub vsat: Value,
    /// DIBL coefficient 1 (ETA0)
    pub eta0: Value,
    /// DIBL coefficient 2 (ETAB)
    pub etab: Value,
    /// Subthreshold swing coefficient (NFACTOR)
    pub nfactor: Value,
    /// Drain saturation voltage coefficient (PCLM)
    pub pclm: Value,
    /// Source/drain resistance (RDSW) in ohm*um
    pub rdsw: Value,

    // BSIM4-specific parameters for enhanced short-channel modeling
    /// Short-channel Vth roll-off coefficient 0 (DVT0)
    pub dvt0: Value,
    /// Short-channel Vth roll-off coefficient 1 (DVT1)
    pub dvt1: Value,
    /// Short-channel Vth roll-off body-bias coefficient (DVT2)
    pub dvt2: Value,
    /// First body effect coefficient (K1)
    pub k1: Value,
    /// Second body effect coefficient (K2)
    pub k2: Value,
    /// Gate-source overlap capacitance per width (CGSO) in F/m
    pub cgso: Value,
    /// Gate-drain overlap capacitance per width (CGDO) in F/m
    pub cgdo: Value,
    /// Gate-bulk overlap capacitance per length (CGBO) in F/m
    pub cgbo: Value,
    /// Source/drain sheet resistance (RSH) in ohm/square
    pub rsh: Value,
    /// Flicker noise coefficient (KF)
    pub kf: Value,
    /// Flicker noise current exponent (AF)
    pub af: Value,
    /// Flicker noise frequency exponent (EF)
    pub ef: Value,
    /// Channel thermal-noise coefficient (gamma)
    pub thermal_noise_gamma: Value,

    // Level 6 (double-exponent/simplified) MOSFET parameters
    /// Current gain coefficient (KC) - drain current multiplier
    pub kc: Value,
    /// Current gain exponent (NC) - affects Vgs dependence
    pub nc: Value,
    /// Voltage clipping coefficient (KV) - saturation factor
    pub kv: Value,
    /// Voltage clipping exponent (NV) - affects Vds dependence  
    pub nv: Value,
    /// Secondary back-gate effect coefficient (GAMMA1)
    pub gamma1: Value,
    /// Drain-induced threshold modulation coefficient (SIGMA)
    pub sigma: Value,
    /// First-order channel length modulation (LAMBDA0)
    pub lambda0: Value,
    /// Second-order channel length modulation (LAMBDA1)
    pub lambda1: Value,

    // Operating point values
    vgs: Value,
    vds: Value,
    vbs: Value,
    eval_vgs: Value,
    eval_vds: Value,
    eval_vbs: Value,
    id: Value,
    gm: Value,
    gds: Value,
    gmb: Value,
    id_eq: Value,
    region: MosRegion,

    // Previous iteration values
    vgs_prev: Value,
    vds_prev: Value,
    vbs_prev: Value,
    eval_vgs_prev: Value,
    eval_vds_prev: Value,
    eval_vbs_prev: Value,
    id_prev: Value,
    gm_prev: Value,
    gds_prev: Value,
    gmb_prev: Value,
    ibs_prev: Value,
    gbs_prev: Value,
    ibd_prev: Value,
    gbd_prev: Value,
    has_branch_history: bool,

    /// Pre-computed matrix indices for O(1) stamping
    pub indices: MosfetIndices,
}

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

    fn new(
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
            level: 1,     // Default to Level 1
            u0: 400.0,    // Low-field mobility (cm^2/V*s)
            ua: 2.25e-9,  // Mobility degradation coefficient
            ub: 5.87e-19, // Second-order mobility coefficient
            vsat: 1.5e5,  // Saturation velocity (m/s)
            eta0: 0.08,   // DIBL coefficient
            etab: -0.07,  // DIBL body-bias coefficient
            nfactor: 1.0, // Subthreshold swing
            pclm: 1.3,    // Channel length modulation
            rdsw: 200.0,  // S/D resistance (ohm*um)

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
        self
    }

    #[inline]
    pub fn set_junction_gmin(&mut self, gmin: Value) {
        self.junction_gmin = gmin.max(0.0);
    }

    #[inline]
    fn terminal_voltage(voltages: &[Value], node: NodeId) -> Value {
        if node == 0 { 0.0 } else { voltages[node - 1] }
    }

    #[inline]
    fn branch_voltages(&self, voltages: &[Value]) -> (Value, Value, Value) {
        let vd = Self::terminal_voltage(voltages, self.node_drain);
        let vg = Self::terminal_voltage(voltages, self.node_gate);
        let vs = Self::terminal_voltage(voltages, self.node_source);
        let vb = Self::terminal_voltage(voltages, self.node_bulk);
        (vg - vs, vd - vs, vb - vs)
    }

    #[inline]
    fn model_space_onset_voltage(&self, vgs: Value, vds: Value, vbs: Value) -> Value {
        if self.level == 6 {
            self.level6_meyer_state(vgs, vds, vbs).1
        } else {
            self.polarity() * self.vth(vbs)
        }
    }

    #[inline]
    fn body_junction_vcrit(isat: Value) -> Value {
        let vt = Self::body_junction_thermal_voltage();
        if !isat.is_finite() || isat <= 0.0 {
            return vt * 40.0;
        }

        let arg = (vt / ((2.0_f64).sqrt() * isat)).max(1.0);
        vt * arg.ln()
    }

    #[inline]
    fn source_body_vcrit(&self) -> Value {
        let isat = self.effective_body_junction_saturation_current(self.source_area);
        Self::body_junction_vcrit(isat)
    }

    #[inline]
    fn drain_body_vcrit(&self) -> Value {
        let isat = self.effective_body_junction_saturation_current(self.drain_area);
        Self::body_junction_vcrit(isat)
    }

    #[inline]
    fn dev_limvds(vnew: Value, vold: Value) -> Value {
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
    fn dev_pnjlim(vnew: Value, vold: Value, vt: Value, vcrit: Value) -> Value {
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
    fn dev_fetlim(vnew: Value, vold: Value, vto: Value) -> Value {
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
    fn limited_branch_voltages_for_eval(
        &self,
        vgs: Value,
        vds: Value,
        vbs: Value,
    ) -> (Value, Value, Value) {
        if !self.has_branch_history
            || !self.vgs_prev.is_finite()
            || !self.vds_prev.is_finite()
            || !self.vbs_prev.is_finite()
        {
            return (vgs, vds, vbs);
        }

        let p = self.polarity();
        let mut vgs_m = p * vgs;
        let mut vds_m = p * vds;
        let mut vbs_m = p * vbs;
        let vold_vgs = p * self.vgs_prev;
        let vold_vds = p * self.vds_prev;
        let vold_vbs = p * self.vbs_prev;
        let mut vbd_m = vbs_m - vds_m;
        let vgd_initial_m = vgs_m - vds_m;
        let vgdo = vold_vgs - vold_vds;
        let von = self.model_space_onset_voltage(self.vgs_prev, self.vds_prev, self.vbs_prev);

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
    fn linearized_operating_point(
        &self,
        vgs: Value,
        vds: Value,
        vbs: Value,
    ) -> (Value, MosRegion, Value, Value, Value, Value) {
        if self.level == 6 {
            let (id, region, gm, gds, gmb) = self.level6_operating_point(vgs, vds, vbs);
            let id_eq = id - gm * vgs - gds * vds - gmb * vbs;
            return (id, region, gm, gds, gmb, id_eq);
        }

        let (id, region) = self.calculate_id(vgs, vds, vbs);
        let (gm, gds, gmb) = self.small_signal(vgs, vds, vbs);
        let id_eq = id - gm * vgs - gds * vds - gmb * vbs;
        (id, region, gm, gds, gmb, id_eq)
    }

    #[inline]
    fn cached_linearization_matches(&self, vgs: Value, vds: Value, vbs: Value) -> bool {
        let (eval_vgs, eval_vds, eval_vbs) = self.limited_branch_voltages_for_eval(vgs, vds, vbs);
        self.eval_vgs.is_finite()
            && self.eval_vds.is_finite()
            && self.eval_vbs.is_finite()
            && self.eval_vgs == eval_vgs
            && self.eval_vds == eval_vds
            && self.eval_vbs == eval_vbs
    }

    /// Set model parameters from a DeviceModel
    pub fn with_params(mut self, params: &std::collections::HashMap<String, Value>) -> Self {
        const EPS0: Value = 8.854_187_817e-12;
        const EPS_SI_REL: Value = 11.7;
        const EPS_OX_REL: Value = 3.9;
        const Q_E: Value = 1.602_176_634e-19;
        const V_T_REF: Value = 0.025_85;
        const N_I_CM3: Value = 1.45e10;

        let kp_explicit = params
            .get("KP")
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

        let tox = params
            .get("TOX")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0);
        let nsub = params
            .get("NSUB")
            .copied()
            .filter(|v| v.is_finite() && *v > 0.0);

        // Level 1 parameters
        if let Some(&v) = params
            .get("VTO")
            .or_else(|| params.get("VT0"))
            .or_else(|| params.get("VTH0"))
        {
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
        // BSIM3 parameters
        if let Some(&v) = params.get("U0").or_else(|| params.get("UO")) {
            self.u0 = v;
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
        if let Some(&v) = params.get("KC") {
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
        self
    }

    /// Set model level (1 = Level 1, 3 = BSIM3-like)
    pub fn with_level(mut self, level: i32) -> Self {
        self.level = level;
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

        self
    }

    /// Link this device to a StaticMatrix for O(1) stamping
    pub fn link(&mut self, matrix: &StaticMatrix) {
        let d = self.node_drain;
        let g = self.node_gate;
        let s = self.node_source;
        let b = self.node_bulk;

        // Drain row (4 columns)
        if d > 0 {
            self.indices.dd = matrix.get_index(d - 1, d - 1);
        }
        if d > 0 && g > 0 {
            self.indices.dg = matrix.get_index(d - 1, g - 1);
        }
        if d > 0 && s > 0 {
            self.indices.ds = matrix.get_index(d - 1, s - 1);
        }
        if d > 0 && b > 0 {
            self.indices.db = matrix.get_index(d - 1, b - 1);
        }
        // Source row (4 columns)
        if s > 0 && d > 0 {
            self.indices.sd = matrix.get_index(s - 1, d - 1);
        }
        if s > 0 && g > 0 {
            self.indices.sg = matrix.get_index(s - 1, g - 1);
        }
        if s > 0 {
            self.indices.ss = matrix.get_index(s - 1, s - 1);
        }
        if s > 0 && b > 0 {
            self.indices.sb = matrix.get_index(s - 1, b - 1);
        }
    }

    /// Stamp using O(1) direct indexing (call after link)
    pub fn stamp_direct(&self, matrix: &mut StaticMatrix, rhs: &mut [Value], voltages: &[Value]) {
        let (vgs, vds, vbs) = self.branch_voltages(voltages);
        let (eval_vgs, eval_vds, eval_vbs) = self.limited_branch_voltages_for_eval(vgs, vds, vbs);
        let (gm, gds, gmb, id_eq) = if self.cached_linearization_matches(vgs, vds, vbs) {
            (self.gm, self.gds, self.gmb, self.id_eq)
        } else {
            let (_, _, gm, gds, gmb, id_eq) =
                self.linearized_operating_point(eval_vgs, eval_vds, eval_vbs);
            (gm, gds, gmb, id_eq)
        };

        // Stamp matrix using direct indexing
        // Drain row
        if let Some(idx) = self.indices.dd {
            matrix.stamp_direct(idx, gds);
        }
        if let Some(idx) = self.indices.dg {
            matrix.stamp_direct(idx, gm);
        }
        if let Some(idx) = self.indices.ds {
            matrix.stamp_direct(idx, -gm - gds - gmb);
        }
        if let Some(idx) = self.indices.db {
            matrix.stamp_direct(idx, gmb);
        }
        // Source row
        if let Some(idx) = self.indices.sd {
            matrix.stamp_direct(idx, -gds);
        }
        if let Some(idx) = self.indices.sg {
            matrix.stamp_direct(idx, -gm);
        }
        if let Some(idx) = self.indices.ss {
            matrix.stamp_direct(idx, gm + gds + gmb);
        }
        if let Some(idx) = self.indices.sb {
            matrix.stamp_direct(idx, -gmb);
        }

        // Stamp RHS
        if self.node_drain > 0 {
            rhs[self.node_drain - 1] -= id_eq;
        }
        if self.node_source > 0 {
            rhs[self.node_source - 1] += id_eq;
        }

        let (bs_anode, bs_cathode, gbs, ieq_bs) = self.body_source_junction_linearization(eval_vbs);
        Self::stamp_diode_linearization_direct(matrix, rhs, bs_anode, bs_cathode, gbs, ieq_bs);

        let (bd_anode, bd_cathode, gbd, ieq_bd) =
            self.body_drain_junction_linearization(eval_vds, eval_vbs);
        Self::stamp_diode_linearization_direct(matrix, rhs, bd_anode, bd_cathode, gbd, ieq_bd);
    }

    /// Get polarity multiplier (+1 for NMOS, -1 for PMOS)
    pub fn polarity(&self) -> Value {
        match self.mos_type {
            MosType::Nmos => 1.0,
            MosType::Pmos => -1.0,
        }
    }

    /// Calculate effective threshold voltage with body effect and short-channel effects
    ///
    /// For Level 1: Standard body effect formula
    /// For Level 3+: Includes BSIM4 short-channel Vth roll-off
    fn vth(&self, vbs: Value) -> Value {
        let p = self.polarity();
        let vbs_eff = p * vbs;
        let vto_eff = match self.mos_type {
            MosType::Nmos => self.vto,
            MosType::Pmos => self.vto.abs(),
        };

        // Base body effect: Vth = Vto + gamma * (sqrt(phi - Vbs) - sqrt(phi))
        let phi_vbs = (self.phi - vbs_eff).max(0.0);
        let vth_base = vto_eff + self.gamma * (phi_vbs.sqrt() - self.phi.sqrt());

        if self.level < 3 || self.level == 6 {
            // Level 1 and Level 6 use simple body effect
            return vth_base;
        }

        // BSIM4 short-channel Vth roll-off
        // Delta_Vth = -DVT0 * L_eff / Ldrawn * (1 + DVT2 * Vbs)
        // where L_eff adjustment factor uses DVT1
        let l_ratio = 1e-6 / self.l.max(1e-9); // Normalize to 1um
        let dvth_sce = -self.dvt0 * l_ratio * (1.0 + self.dvt1 * l_ratio);

        // Body-bias modulation of SCE
        let dvth_bias = self.dvt2 * vbs_eff * l_ratio;

        // Enhanced body effect using K1/K2 (BSIM4 style)
        // Vth = Vto + K1 * sqrt(phi - Vbs) + K2 * (phi - Vbs)
        let vth_k1k2 = vto_eff + self.k1 * phi_vbs.sqrt() + self.k2 * (self.phi - vbs_eff);

        // Blend between GAMMA-based and K1/K2-based body effect based on model level
        // Use K1/K2 formulation for short channels (level 3+)
        vth_k1k2 + dvth_sce + dvth_bias
    }

    #[inline]
    fn level6_effective_length(&self) -> Value {
        let leff = self.l - 2.0 * self.ld;
        leff.max(1e-12)
    }

    #[inline]
    fn oxide_capacitance_total(&self) -> Value {
        let channel_length = if self.level == 6 {
            self.level6_effective_length()
        } else {
            self.l
        };
        self.cox * self.w * channel_length
    }

    #[inline]
    fn meyer_intrinsic_capacitances(
        vgs: Value,
        vgd: Value,
        vgb: Value,
        von: Value,
        vdsat: Value,
        phi: Value,
        oxide_cap: Value,
    ) -> (Value, Value, Value) {
        let _ = vgb;
        const MAGIC_VDS: Value = 0.025;

        let vgst = vgs - von;
        let vds = vgs - vgd;
        let vdsat = vdsat.max(MAGIC_VDS);

        if vgst <= -phi {
            (0.0, 0.0, oxide_cap / 2.0)
        } else if vgst <= -phi / 2.0 {
            (0.0, 0.0, -vgst * oxide_cap / (2.0 * phi))
        } else if vgst <= 0.0 {
            let mut capgs = vgst * oxide_cap / (1.5 * phi) + oxide_cap / 3.0;
            let capgd = if vds >= vdsat {
                0.0
            } else {
                let vddif = 2.0 * vdsat - vds;
                let vddif1 = vdsat - vds;
                let vddif2 = vddif * vddif;
                let capgd = capgs * (1.0 - vdsat * vdsat / vddif2);
                capgs *= 1.0 - vddif1 * vddif1 / vddif2;
                capgd
            };
            let capgb = -vgst * oxide_cap / (2.0 * phi);
            (capgs.max(0.0), capgd.max(0.0), capgb.max(0.0))
        } else if vdsat <= vds {
            (oxide_cap / 3.0, 0.0, 0.0)
        } else {
            let vddif = 2.0 * vdsat - vds;
            let vddif1 = vdsat - vds;
            let vddif2 = vddif * vddif;
            let capgd = oxide_cap * (1.0 - vdsat * vdsat / vddif2) / 3.0;
            let capgs = oxide_cap * (1.0 - vddif1 * vddif1 / vddif2) / 3.0;
            (capgs.max(0.0), capgd.max(0.0), 0.0)
        }
    }

    fn level6_meyer_state(&self, vgs: Value, vds: Value, vbs: Value) -> (Value, Value, Value) {
        let p = self.polarity();
        let vgs_m = p * vgs;
        let vds_m = p * vds;
        let vbs_m = p * vbs;
        let vbd_m = vbs_m - vds_m;
        let vgd_m = vgs_m - vds_m;

        let mode = if vds_m >= 0.0 { 1.0 } else { -1.0 };
        let vdshere = vds_m * mode;
        let vbsvbd = if mode > 0.0 { vbs_m } else { vbd_m };
        let vg_active = if mode > 0.0 { vgs_m } else { vgd_m };

        let phi = self.phi.max(1e-12);
        let sqrt_phi = phi.sqrt();
        let sarg1 = if vbsvbd <= 0.0 {
            (phi - vbsvbd).max(0.0).sqrt()
        } else {
            (sqrt_phi - vbsvbd / (2.0 * sqrt_phi.max(1e-12))).max(0.0)
        };
        let von = p * self.vto + self.gamma * (sarg1 - sqrt_phi)
            - self.gamma1 * vbsvbd
            - self.sigma * vdshere;
        let vgon = (vg_active - von).max(0.0);
        let vdsat = if vgon > 0.0 {
            self.kv * vgon.powf(self.nv)
        } else {
            0.0
        };

        (mode, von, vdsat)
    }

    pub(crate) fn transient_capacitance_halves_at(
        &self,
        vgs: Value,
        vds: Value,
        vbs: Value,
    ) -> (Value, Value, Value) {
        let oxide_cap = self.oxide_capacitance_total();
        let phi = self.phi.max(1e-12);
        let p = self.polarity();
        let vgs_m = p * vgs;
        let vds_m = p * vds;
        let vbs_m = p * vbs;
        let vgd_m = vgs_m - vds_m;
        let vgb_m = vgs_m - vbs_m;

        let (mode, von, vdsat) = if self.level == 6 {
            self.level6_meyer_state(vgs, vds, vbs)
        } else {
            let mode = if vds_m >= 0.0 { 1.0 } else { -1.0 };
            let vg_active = if mode > 0.0 { vgs_m } else { vgd_m };
            let von = self.vth(vbs);
            let vdsat = (vg_active - von).max(0.0);
            (mode, von, vdsat)
        };

        let (cgs_int, cgd_int, cgb_int) = if mode > 0.0 {
            Self::meyer_intrinsic_capacitances(vgs_m, vgd_m, vgb_m, von, vdsat, phi, oxide_cap)
        } else {
            let (capgd_int, capgs_int, capgb_int) =
                Self::meyer_intrinsic_capacitances(vgd_m, vgs_m, vgb_m, von, vdsat, phi, oxide_cap);
            (capgs_int, capgd_int, capgb_int)
        };

        (cgs_int, cgd_int, cgb_int)
    }

    #[cfg(test)]
    pub(crate) fn transient_capacitances_at(
        &self,
        vgs: Value,
        vds: Value,
        vbs: Value,
    ) -> (Value, Value, Value) {
        let (cgs_half, cgd_half, cgb_half) = self.transient_capacitance_halves_at(vgs, vds, vbs);
        let (cgs_ov, cgd_ov, cgb_ov) = self.overlap_capacitances();
        (
            2.0 * cgs_half + cgs_ov,
            2.0 * cgd_half + cgd_ov,
            2.0 * cgb_half + cgb_ov,
        )
    }

    /// Calculate overlap capacitances for AC analysis
    /// Returns (Cgs_overlap, Cgd_overlap, Cgb_overlap)
    pub fn overlap_capacitances(&self) -> (Value, Value, Value) {
        // Cgs_overlap = CGSO * W
        let cgs = self.cgso * self.w;
        // Cgd_overlap = CGDO * W
        let cgd = self.cgdo * self.w;
        // MOS6 uses effective channel length for gate-bulk overlap.
        let cgb_length = if self.level == 6 {
            self.level6_effective_length()
        } else {
            self.l
        };
        let cgb = self.cgbo * cgb_length;

        (cgs, cgd, cgb)
    }

    /// Calculate total AC small-signal capacitances using Meyer model
    ///
    /// Returns (Cgs, Cgd, Cgb) including both intrinsic channel capacitances
    /// and overlap capacitances. Values depend on operating region.
    ///
    /// # Meyer Capacitance Model
    /// - Cutoff: Cgb dominates, Cgs = Cgd = overlap only
    /// - Linear: Cgs = Cgd = Cox*W*L/2 + overlap
    /// - Saturation: Cgs = 2/3*Cox*W*L + overlap, Cgd = overlap only
    pub fn ac_capacitances(&self) -> (Value, Value, Value) {
        let (cgs_ov, cgd_ov, cgb_ov) = self.overlap_capacitances();

        // Intrinsic gate oxide capacitance
        let channel_length = if self.level == 6 {
            self.level6_effective_length()
        } else {
            self.l
        };
        let cox_wl = self.cox * self.w * channel_length;

        // Determine operating region from stored values
        let vgs_eff = self.polarity() * self.vgs;
        let vds_eff = self.polarity() * self.vds;
        let vth = self.vth(self.vbs);
        let vgt = vgs_eff - self.polarity() * vth;

        if vgt <= 0.0 {
            // Cutoff region: only overlap capacitances, Cgb = Cox*W*L
            (cgs_ov, cgd_ov, cox_wl + cgb_ov)
        } else if vds_eff < vgt {
            // Linear region: symmetric distribution
            let cgs_int = 0.5 * cox_wl;
            let cgd_int = 0.5 * cox_wl;
            (cgs_int + cgs_ov, cgd_int + cgd_ov, cgb_ov)
        } else {
            // Saturation region: 2/3 to source, nearly zero to drain
            let cgs_int = (2.0 / 3.0) * cox_wl;
            let cgd_int = 0.0; // Small in saturation
            (cgs_int + cgs_ov, cgd_int + cgd_ov, cgb_ov)
        }
    }

    /// Return the cached drain current at the converged operating point.
    pub fn drain_current(&self) -> Value {
        self.id
    }

    pub(crate) fn branch_voltages_at(&self, voltages: &[Value]) -> (Value, Value, Value) {
        self.branch_voltages(voltages)
    }

    #[inline]
    fn cached_eval_branch_voltages(&self) -> Option<(Value, Value, Value)> {
        if !self.has_branch_history
            || !self.vgs.is_finite()
            || !self.vds.is_finite()
            || !self.vbs.is_finite()
            || !self.eval_vgs.is_finite()
            || !self.eval_vds.is_finite()
            || !self.eval_vbs.is_finite()
        {
            return None;
        }

        Some((self.eval_vgs, self.eval_vds, self.eval_vbs))
    }

    #[inline]
    pub(crate) fn eval_branch_voltages_at(&self, voltages: &[Value]) -> (Value, Value, Value) {
        let (vgs, vds, vbs) = self.branch_voltages(voltages);
        if self.has_branch_history && self.vgs == vgs && self.vds == vds && self.vbs == vbs {
            if let Some(eval) = self.cached_eval_branch_voltages() {
                return eval;
            }
        }

        self.limited_branch_voltages_for_eval(vgs, vds, vbs)
    }

    #[inline]
    fn body_junction_thermal_voltage() -> Value {
        VT_REFERENCE.max(1e-12)
    }

    #[inline]
    fn junction_diode_current(isat: Value, v: Value, gmin: Value) -> Value {
        let isat = if isat.is_finite() && isat > 0.0 {
            isat
        } else {
            0.0
        };
        let gmin = gmin.max(0.0);
        let nvt = Self::body_junction_thermal_voltage();
        if v <= -3.0 * nvt {
            gmin * v - isat
        } else {
            let expv = (v / nvt).clamp(-80.0, 80.0).exp();
            isat * (expv - 1.0) + gmin * v
        }
    }

    #[inline]
    fn junction_diode_conductance(isat: Value, v: Value, gmin: Value) -> Value {
        let isat = if isat.is_finite() && isat > 0.0 {
            isat
        } else {
            0.0
        };
        let gmin = gmin.max(0.0);
        let nvt = Self::body_junction_thermal_voltage();
        if v <= -3.0 * nvt {
            gmin
        } else {
            let expv = (v / nvt).clamp(-80.0, 80.0).exp();
            (isat / nvt) * expv + gmin
        }
    }

    #[inline]
    fn effective_body_junction_saturation_current(&self, area: Value) -> Value {
        let area_scaled = if self.js_bulk > 0.0 && area > 0.0 {
            self.js_bulk * area
        } else {
            self.is_bulk
        };
        if area_scaled.is_finite() && area_scaled > 0.0 {
            area_scaled
        } else {
            0.0
        }
    }

    #[inline]
    fn source_zero_bias_bottom_junction_capacitance(&self) -> Value {
        self.source_bulk_cap_zero_bias
            .unwrap_or(self.cj * self.source_area)
            .max(0.0)
    }

    #[inline]
    fn drain_zero_bias_bottom_junction_capacitance(&self) -> Value {
        self.drain_bulk_cap_zero_bias
            .unwrap_or(self.cj * self.drain_area)
            .max(0.0)
    }

    #[inline]
    fn source_zero_bias_sidewall_junction_capacitance(&self) -> Value {
        (self.cjsw * self.source_perimeter).max(0.0)
    }

    #[inline]
    fn drain_zero_bias_sidewall_junction_capacitance(&self) -> Value {
        (self.cjsw * self.drain_perimeter).max(0.0)
    }

    #[inline]
    fn junction_depletion_scaling(arg: Value, grading: Value) -> Value {
        if !arg.is_finite() || arg <= 0.0 {
            return 0.0;
        }
        if (grading - 0.5).abs() < 1e-15 {
            1.0 / arg.sqrt()
        } else {
            (-grading * arg.ln()).exp()
        }
    }

    #[inline]
    fn junction_depletion_charge_term(
        c0: Value,
        bulk_potential: Value,
        grading: Value,
        arg: Value,
        scaling: Value,
    ) -> Value {
        if c0 <= 0.0 {
            return 0.0;
        }
        if (1.0 - grading).abs() < 1e-12 {
            -c0 * bulk_potential * arg.ln()
        } else {
            c0 * bulk_potential * (1.0 - arg * scaling) / (1.0 - grading)
        }
    }

    #[inline]
    fn junction_depletion_charge_and_capacitance(
        junction_voltage: Value,
        bottom_zero_bias_cap: Value,
        sidewall_zero_bias_cap: Value,
        bulk_potential: Value,
        bottom_grading: Value,
        sidewall_grading: Value,
        forward_cap_coeff: Value,
    ) -> (Value, Value) {
        let bottom_zero_bias_cap = bottom_zero_bias_cap.max(0.0);
        let sidewall_zero_bias_cap = sidewall_zero_bias_cap.max(0.0);
        if bottom_zero_bias_cap == 0.0 && sidewall_zero_bias_cap == 0.0 {
            return (0.0, 0.0);
        }

        let bulk_potential = bulk_potential.max(1e-12);
        let forward_cap_coeff = forward_cap_coeff.clamp(0.0, 0.999_999_999_999);
        let depletion_corner = forward_cap_coeff * bulk_potential;

        if junction_voltage < depletion_corner {
            let arg = (1.0 - junction_voltage / bulk_potential).max(1e-18);
            let bottom_scale = Self::junction_depletion_scaling(arg, bottom_grading);
            let sidewall_scale = Self::junction_depletion_scaling(arg, sidewall_grading);
            let charge = Self::junction_depletion_charge_term(
                bottom_zero_bias_cap,
                bulk_potential,
                bottom_grading,
                arg,
                bottom_scale,
            ) + Self::junction_depletion_charge_term(
                sidewall_zero_bias_cap,
                bulk_potential,
                sidewall_grading,
                arg,
                sidewall_scale,
            );
            let capacitance = (bottom_zero_bias_cap * bottom_scale
                + sidewall_zero_bias_cap * sidewall_scale)
                .max(0.0);
            return (charge, capacitance);
        }

        let arg = (1.0 - forward_cap_coeff).max(1e-18);
        let bottom_scale = Self::junction_depletion_scaling(arg, bottom_grading);
        let sidewall_scale = Self::junction_depletion_scaling(arg, sidewall_grading);
        let f2 = bottom_zero_bias_cap
            * (1.0 - forward_cap_coeff * (1.0 + bottom_grading))
            * bottom_scale
            / arg
            + sidewall_zero_bias_cap
                * (1.0 - forward_cap_coeff * (1.0 + sidewall_grading))
                * sidewall_scale
                / arg;
        let f3 = bottom_zero_bias_cap * bottom_grading * bottom_scale / arg / bulk_potential
            + sidewall_zero_bias_cap * sidewall_grading * sidewall_scale / arg / bulk_potential;
        let edge_charge = Self::junction_depletion_charge_term(
            bottom_zero_bias_cap,
            bulk_potential,
            bottom_grading,
            arg,
            bottom_scale,
        ) + Self::junction_depletion_charge_term(
            sidewall_zero_bias_cap,
            bulk_potential,
            sidewall_grading,
            arg,
            sidewall_scale,
        );
        let f4 =
            edge_charge - 0.5 * f3 * depletion_corner * depletion_corner - depletion_corner * f2;
        let charge = f4 + junction_voltage * (f2 + 0.5 * junction_voltage * f3);
        let capacitance = (f2 + junction_voltage * f3).max(0.0);
        (charge, capacitance)
    }

    #[inline]
    fn body_source_diode_nodes(&self) -> (NodeId, NodeId) {
        match self.mos_type {
            MosType::Nmos => (self.node_bulk, self.node_source),
            MosType::Pmos => (self.node_source, self.node_bulk),
        }
    }

    #[inline]
    fn body_drain_diode_nodes(&self) -> (NodeId, NodeId) {
        match self.mos_type {
            MosType::Nmos => (self.node_bulk, self.node_drain),
            MosType::Pmos => (self.node_drain, self.node_bulk),
        }
    }

    #[inline]
    fn body_source_diode_voltage(&self, vbs: Value) -> Value {
        match self.mos_type {
            MosType::Nmos => vbs,
            MosType::Pmos => -vbs,
        }
    }

    #[inline]
    fn body_drain_diode_voltage(&self, vds: Value, vbs: Value) -> Value {
        let vbd = vbs - vds;
        match self.mos_type {
            MosType::Nmos => vbd,
            MosType::Pmos => -vbd,
        }
    }

    #[inline]
    pub(crate) fn body_source_charge_nodes(&self) -> (NodeId, NodeId) {
        self.body_source_diode_nodes()
    }

    #[inline]
    pub(crate) fn body_drain_charge_nodes(&self) -> (NodeId, NodeId) {
        self.body_drain_diode_nodes()
    }

    #[inline]
    pub(crate) fn body_source_charge_branch_voltage(&self, vbs: Value) -> Value {
        self.body_source_diode_voltage(vbs)
    }

    #[inline]
    pub(crate) fn body_drain_charge_branch_voltage(&self, vds: Value, vbs: Value) -> Value {
        self.body_drain_diode_voltage(vds, vbs)
    }

    #[inline]
    fn body_source_junction_current_and_conductance(&self, vbs: Value) -> (Value, Value) {
        let vd = self.body_source_diode_voltage(vbs);
        let isat = self.effective_body_junction_saturation_current(self.source_area);
        (
            Self::junction_diode_current(isat, vd, self.junction_gmin),
            Self::junction_diode_conductance(isat, vd, self.junction_gmin),
        )
    }

    #[inline]
    fn body_drain_junction_current_and_conductance(
        &self,
        vds: Value,
        vbs: Value,
    ) -> (Value, Value) {
        let vd = self.body_drain_diode_voltage(vds, vbs);
        let isat = self.effective_body_junction_saturation_current(self.drain_area);
        (
            Self::junction_diode_current(isat, vd, self.junction_gmin),
            Self::junction_diode_conductance(isat, vd, self.junction_gmin),
        )
    }

    #[inline]
    pub(crate) fn body_source_junction_charge_and_capacitance_at(
        &self,
        vbs: Value,
    ) -> (Value, Value) {
        Self::junction_depletion_charge_and_capacitance(
            self.body_source_diode_voltage(vbs),
            self.source_zero_bias_bottom_junction_capacitance(),
            self.source_zero_bias_sidewall_junction_capacitance(),
            self.pb,
            self.mj,
            self.mjsw,
            self.fc,
        )
    }

    #[inline]
    pub(crate) fn body_drain_junction_charge_and_capacitance_at(
        &self,
        vds: Value,
        vbs: Value,
    ) -> (Value, Value) {
        Self::junction_depletion_charge_and_capacitance(
            self.body_drain_diode_voltage(vds, vbs),
            self.drain_zero_bias_bottom_junction_capacitance(),
            self.drain_zero_bias_sidewall_junction_capacitance(),
            self.pb,
            self.mj,
            self.mjsw,
            self.fc,
        )
    }

    #[inline]
    fn body_source_junction_linearization(&self, vbs: Value) -> (NodeId, NodeId, Value, Value) {
        let (anode, cathode) = self.body_source_diode_nodes();
        let vd = self.body_source_diode_voltage(vbs);
        let (id, gd) = self.body_source_junction_current_and_conductance(vbs);
        let ieq = id - gd * vd;
        (anode, cathode, gd, ieq)
    }

    #[inline]
    fn body_drain_junction_linearization(
        &self,
        vds: Value,
        vbs: Value,
    ) -> (NodeId, NodeId, Value, Value) {
        let (anode, cathode) = self.body_drain_diode_nodes();
        let vd = self.body_drain_diode_voltage(vds, vbs);
        let (id, gd) = self.body_drain_junction_current_and_conductance(vds, vbs);
        let ieq = id - gd * vd;
        (anode, cathode, gd, ieq)
    }

    #[inline]
    fn stamp_diode_linearization(
        matrix: &mut impl MatrixStamper,
        anode: NodeId,
        cathode: NodeId,
        gd: Value,
        ieq: Value,
    ) {
        if gd == 0.0 && ieq == 0.0 {
            return;
        }

        matrix.stamp(anode, anode, gd);
        matrix.stamp(anode, cathode, -gd);
        matrix.stamp(cathode, anode, -gd);
        matrix.stamp(cathode, cathode, gd);
        matrix.stamp_rhs(anode, -ieq);
        matrix.stamp_rhs(cathode, ieq);
    }

    #[inline]
    fn stamp_diode_linearization_direct(
        matrix: &mut StaticMatrix,
        rhs: &mut [Value],
        anode: NodeId,
        cathode: NodeId,
        gd: Value,
        ieq: Value,
    ) {
        if gd == 0.0 && ieq == 0.0 {
            return;
        }

        if anode > 0 {
            matrix.add(anode - 1, anode - 1, gd);
            if cathode > 0 {
                matrix.add(anode - 1, cathode - 1, -gd);
            }
            rhs[anode - 1] -= ieq;
        }
        if cathode > 0 {
            if anode > 0 {
                matrix.add(cathode - 1, anode - 1, -gd);
            }
            matrix.add(cathode - 1, cathode - 1, gd);
            rhs[cathode - 1] += ieq;
        }
    }

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

    /// Voltages with intrinsic source/drain swapped.
    ///
    /// (Vgs', Vds', Vbs') correspond to using original drain as intrinsic source:
    /// - Vgs' = Vgs - Vds = Vg - Vd
    /// - Vds' = -Vds = Vs - Vd
    /// - Vbs' = Vbs - Vds = Vb - Vd
    fn reverse_voltages(vgs: Value, vds: Value, vbs: Value) -> (Value, Value, Value) {
        (vgs - vds, -vds, vbs - vds)
    }

    /// Determine operating region and calculate drain current
    fn calculate_id(&self, vgs: Value, vds: Value, vbs: Value) -> (Value, MosRegion) {
        if self.level == 6 {
            return self.calculate_id_level6(vgs, vds, vbs);
        }

        // Superimpose forward and reverse-oriented channel currents to preserve
        // source/drain symmetry while maintaining smooth behavior around Vds = 0.
        let (id_forward, region_forward) = self.calculate_id_forward(vgs, vds, vbs);

        let (vgs_rev, vds_rev, vbs_rev) = Self::reverse_voltages(vgs, vds, vbs);
        let (id_reverse_fwd, region_reverse) = self.calculate_id_forward(vgs_rev, vds_rev, vbs_rev);
        let id = id_forward - id_reverse_fwd;

        // Region is used for reporting only; choose the dominant orientation.
        let region = if id_forward.abs() >= id_reverse_fwd.abs() {
            region_forward
        } else {
            region_reverse
        };

        (id, region)
    }

    fn calculate_id_forward(&self, vgs: Value, vds: Value, vbs: Value) -> (Value, MosRegion) {
        if self.level == 6 {
            self.calculate_id_level6(vgs, vds, vbs)
        } else if self.level >= 3 {
            self.calculate_id_bsim3(vgs, vds, vbs)
        } else {
            self.calculate_id_level1(vgs, vds, vbs)
        }
    }

    /// Level 1 (Shichman-Hodges) drain current calculation with C1 continuous transitions
    ///
    /// Uses smooth blending between regions to ensure continuous first derivatives,
    /// which is critical for Newton-Raphson convergence.
    fn calculate_id_level1(&self, vgs: Value, vds: Value, vbs: Value) -> (Value, MosRegion) {
        let p = self.polarity();
        let vgs_eff = p * vgs;
        let vds_eff = smooth_positive(p * vds, VDS_SMOOTHING); // Ensure positive Vds
        let vth = self.vth(vbs);

        // Gate overdrive with smooth cutoff transition
        // vgt_smooth ≈ 0 when vgs < vth, ≈ (vgs - vth) when vgs > vth
        let vgt_raw = vgs_eff - vth;
        let vgt = smooth_positive(vgt_raw, SMOOTH_VOLTAGE);

        // Determine effective region for reporting (but calculations are smooth)
        let region = if vgt_raw <= -SMOOTH_VOLTAGE {
            MosRegion::Cutoff
        } else if vds_eff < vgt - SMOOTH_VOLTAGE {
            MosRegion::Linear
        } else {
            MosRegion::Saturation
        };

        // Smooth saturation voltage: Vdsat = min(Vgt, Vds) but smooth
        // This naturally blends linear and saturation regions
        let vdsat = smooth_min(vgt, vds_eff, SMOOTH_VOLTAGE);

        // Unified current equation that smoothly transitions between regions:
        // In linear: Id = beta * (Vgt * Vds - Vds²/2)
        // In saturation: Id = beta/2 * Vgt² (when Vds = Vgt)
        //
        // Using Vdsat as the effective drain voltage gives us both:
        // Id = beta * (Vgt * Vdsat - Vdsat²/2) * (1 + lambda * Vds)
        let id_core = self.beta() * (vgt * vdsat - 0.5 * vdsat * vdsat);
        let id = p * id_core * (1.0 + self.lambda * vds_eff);

        (id, region)
    }

    /// BSIM3-like drain current with short-channel effects
    /// Includes:
    /// - Mobility degradation due to vertical electric field
    /// - Velocity saturation
    /// - Drain-Induced Barrier Lowering (DIBL)
    /// - Channel length modulation
    ///
    /// BSIM3-like drain current with C1 continuous transitions
    fn calculate_id_bsim3(&self, vgs: Value, vds: Value, vbs: Value) -> (Value, MosRegion) {
        let p = self.polarity();
        let vgs_eff = p * vgs;
        let vds_eff = smooth_positive(p * vds, VDS_SMOOTHING);
        let vbs_eff = p * vbs;

        // DIBL: threshold voltage reduction with drain bias (smooth minimum for Vth)
        let vth_dibl = self.vth(vbs) - self.eta0 * vds_eff - self.etab * vbs_eff * vds_eff;
        let vth = smooth_max(vth_dibl, 0.1, SMOOTH_VOLTAGE);

        // Gate overdrive with smooth transition
        let vgt_raw = vgs_eff - vth;
        let vgt = smooth_positive(vgt_raw, SMOOTH_VOLTAGE);

        // Subthreshold current blended smoothly with above-threshold current
        let vt = 0.0259; // Thermal voltage at 300K
        let n = self.nfactor;
        // Smooth blend factor: 0 when well above threshold, 1 when below
        let subthreshold_blend = 1.0 - smooth_step(vgt_raw, SMOOTH_VOLTAGE);
        let i_sub = 1e-12 * (vgt_raw.min(100.0 * vt) / (n * vt)).exp().min(1e6);

        // Mobility degradation (vertical field effect)
        let eeff = vgt / 6e-9; // Assume tox = 6nm
        let mobility = self.u0 / (1.0 + self.ua * eeff + self.ub * eeff * eeff);

        // Effective beta with mobility degradation
        let beta_eff = mobility * 1e-4 * self.cox * self.wl_ratio();

        // Saturation voltage with velocity saturation (smooth formulation)
        let vsat_over_l = self.vsat / self.l;
        let mu_m2 = mobility * 1e-4;
        let vdsat_vel = vgt / (1.0 + vgt / (self.l * vsat_over_l / mu_m2).max(1e-6));

        // Smooth min between Vds and Vdsat for unified linear/saturation
        let vdsat = smooth_min(vdsat_vel, vds_eff, SMOOTH_VOLTAGE);

        // Channel length modulation (smooth)
        let vds_over_vdsat = vds_eff / vdsat_vel.max(1e-6);
        let clm_arg = smooth_positive(vds_over_vdsat - 1.0, 0.01);
        let clm = 1.0 + self.pclm * clm_arg.ln_1p();

        // Unified current equation
        let id_above = beta_eff * (vgt * vdsat - 0.5 * vdsat * vdsat) * clm;

        // Blend subthreshold and above-threshold currents
        let id = p * (subthreshold_blend * i_sub + (1.0 - subthreshold_blend) * id_above);

        // Region determination (for reporting only)
        let region = if vgt_raw <= -SMOOTH_VOLTAGE {
            MosRegion::Cutoff
        } else if vds_eff < vdsat_vel - SMOOTH_VOLTAGE {
            MosRegion::Linear
        } else {
            MosRegion::Saturation
        };

        (id, region)
    }

    /// Source-matched ngspice MOS6 operating point.
    ///
    /// The Level-6 implementation follows the ngspice `mos6load.c` equations:
    /// - polarity-folded model-space voltages
    /// - explicit normal/inverse mode selection from `Vds`
    /// - `vgon`, `vdsat`, `idsat`, and linear-region backoff
    /// - direct inverse-mode Jacobian transform back into original variables
    fn level6_operating_point(
        &self,
        vgs: Value,
        vds: Value,
        vbs: Value,
    ) -> (Value, MosRegion, Value, Value, Value) {
        let p = self.polarity();
        let vgs_m = p * vgs;
        let vds_m = p * vds;
        let vbs_m = p * vbs;
        let vbd_m = vbs_m - vds_m;
        let vgd_m = vgs_m - vds_m;

        let mode = if vds_m >= 0.0 { 1.0 } else { -1.0 };
        let vdshere = vds_m * mode;
        let vbsvbd = if mode > 0.0 { vbs_m } else { vbd_m };
        let vg_active = if mode > 0.0 { vgs_m } else { vgd_m };

        let phi = self.phi.max(1e-12);
        let sqrt_phi = phi.sqrt();
        let sarg1 = if vbsvbd <= 0.0 {
            (phi - vbsvbd).max(0.0).sqrt()
        } else {
            (sqrt_phi - vbsvbd / (2.0 * sqrt_phi.max(1e-12))).max(0.0)
        };

        let von = p * self.vto + self.gamma * (sarg1 - sqrt_phi)
            - self.gamma1 * vbsvbd
            - self.sigma * vdshere;
        let vgon = vg_active - von;
        if !vgon.is_finite() || vgon <= 0.0 {
            return (0.0, MosRegion::Cutoff, 0.0, 0.0, 0.0);
        }

        let vonbm = if sarg1 <= 0.0 {
            0.0
        } else if vbsvbd <= 0.0 {
            self.gamma1 + self.gamma / (2.0 * sarg1)
        } else {
            self.gamma1 + self.gamma / (2.0 * sqrt_phi.max(1e-12))
        };

        let betac = self.kc * self.w / self.level6_effective_length();
        let vdsat = self.kv * vgon.powf(self.nv);
        let idsat = betac * vgon.powf(self.nc);
        let lambda = self.lambda0 - self.lambda1 * vbsvbd;

        let mut region = MosRegion::Saturation;
        let mut cdrain = idsat * (1.0 + lambda * vdshere);
        let mut gm_model = cdrain * self.nc / vgon;
        let mut gds_model = gm_model * self.sigma + idsat * lambda;
        let mut gmb_model = gm_model * vonbm - idsat * self.lambda1 * vdshere;

        if vdsat > vdshere {
            region = MosRegion::Linear;
            let vdst = vdshere / vdsat;
            let vdst2 = (2.0 - vdst) * vdst;
            let vdstg = -vdst * self.nv / vgon;
            let ivdst1 = cdrain * (2.0 - 2.0 * vdst);
            cdrain *= vdst2;
            gm_model = gm_model * vdst2 + ivdst1 * vdstg;
            gds_model = gds_model * vdst2 + ivdst1 * (1.0 / vdsat + vdstg * self.sigma);
            gmb_model = gmb_model * vdst2 + ivdst1 * vdstg * vonbm;
        }

        let id = p * mode * cdrain;
        let (gm, gds, gmb) = if mode > 0.0 {
            (gm_model, gds_model, gmb_model)
        } else {
            (-gm_model, gm_model + gds_model + gmb_model, -gmb_model)
        };

        let sanitize = |value: Value| if value.is_finite() { value } else { 0.0 };
        (
            sanitize(id),
            region,
            sanitize(gm),
            sanitize(gds),
            sanitize(gmb),
        )
    }

    fn calculate_id_level6(&self, vgs: Value, vds: Value, vbs: Value) -> (Value, MosRegion) {
        let (id, region, _, _, _) = self.level6_operating_point(vgs, vds, vbs);
        (id, region)
    }

    /// Calculate (gm, gds, gmb) including both forward and reverse-oriented
    /// channel contributions for source/drain symmetry.
    ///
    /// If `f` is the forward-oriented current and:
    /// `Id = f(Vgs, Vds, Vbs) - f(Vgs - Vds, -Vds, Vbs - Vds)`,
    /// then for the reverse term chain rule yields:
    /// - gm_rev  = -gm_fwd_rev
    /// - gds_rev = gm_fwd_rev + gds_fwd_rev + gmb_fwd_rev
    /// - gmb_rev = -gmb_fwd_rev
    fn small_signal(&self, vgs: Value, vds: Value, vbs: Value) -> (Value, Value, Value) {
        if self.level == 6 {
            let (_, _, gm, gds, gmb) = self.level6_operating_point(vgs, vds, vbs);
            return (gm, gds, gmb);
        }

        let gm_forward = self.gm_forward(vgs, vds, vbs);
        let gds_forward = self.gds_forward(vgs, vds, vbs);
        let gmb_forward = self.gmb_forward(vgs, vds, vbs);

        let (vgs_rev, vds_rev, vbs_rev) = Self::reverse_voltages(vgs, vds, vbs);
        let gm_fwd_rev = self.gm_forward(vgs_rev, vds_rev, vbs_rev);
        let gds_fwd_rev = self.gds_forward(vgs_rev, vds_rev, vbs_rev);
        let gmb_fwd_rev = self.gmb_forward(vgs_rev, vds_rev, vbs_rev);

        let gm = gm_forward - gm_fwd_rev;
        let gds = gds_forward + gm_fwd_rev + gds_fwd_rev + gmb_fwd_rev;
        let gmb = gmb_forward - gmb_fwd_rev;
        (gm, gds, gmb)
    }

    fn gm_forward(&self, vgs: Value, vds: Value, vbs: Value) -> Value {
        let p = self.polarity();
        let vgs_eff = p * vgs;
        let _vds_eff = smooth_positive(p * vds, VDS_SMOOTHING);
        let vth = self.vth(vbs);
        let vgt_raw = vgs_eff - vth;
        let vgt = smooth_positive(vgt_raw, SMOOTH_VOLTAGE);
        let dvgt_dvgs = smooth_step(vgt_raw, SMOOTH_VOLTAGE);

        // Analytical formula for Level 1/3 (optimized path)
        let vds_eff = smooth_positive(p * vds, VDS_SMOOTHING);
        let vdsat = smooth_min(vgt, vds_eff, SMOOTH_VOLTAGE);
        let sat_blend = smooth_step(vgt - vds_eff, SMOOTH_VOLTAGE);
        let dvdsat_dvgs = sat_blend * dvgt_dvgs;
        let gm_core = self.beta() * (vdsat * dvgt_dvgs + (vgt - vdsat) * dvdsat_dvgs);
        (gm_core * (1.0 + self.lambda * vds_eff)).max(1e-12)
    }

    fn gds_forward(&self, vgs: Value, vds: Value, vbs: Value) -> Value {
        let p = self.polarity();
        let vgs_eff = p * vgs;
        let vds_eff = smooth_positive(p * vds, VDS_SMOOTHING);
        let vth = self.vth(vbs);
        let vgt_raw = vgs_eff - vth;
        let vgt = smooth_positive(vgt_raw, SMOOTH_VOLTAGE);

        // Analytical formula for Level 1/3 (optimized path)
        let vdsat = smooth_min(vgt, vds_eff, SMOOTH_VOLTAGE);
        let lin_blend = smooth_step(vgt - vds_eff, SMOOTH_VOLTAGE);
        let dvdsat_dvds = 1.0 - lin_blend;
        let gds_core = self.beta() * (vgt - vdsat) * dvdsat_dvds;
        let id_core = self.beta() * (vgt * vdsat - 0.5 * vdsat * vdsat);
        let gds_clm = id_core * self.lambda;
        (gds_core * (1.0 + self.lambda * vds_eff) + gds_clm).max(1e-12)
    }

    fn gmb_forward(&self, vgs: Value, vds: Value, vbs: Value) -> Value {
        let p = self.polarity();
        let vbs_eff = p * vbs;

        // gmb = -gm * (gamma / (2 * sqrt(phi - Vbs)))
        // The gm function is already smooth, so gmb inherits smoothness
        let gm = self.gm_forward(vgs, vds, vbs);

        // Smooth the phi - Vbs term to avoid singularity
        let phi_vbs = smooth_max(self.phi - vbs_eff, SMOOTH_VOLTAGE, SMOOTH_VOLTAGE);

        gm * self.gamma / (2.0 * phi_vbs.sqrt())
    }
}

impl NonlinearDevice for Mosfet {
    fn update(&mut self, voltages: &[Value]) {
        self.vgs_prev = self.vgs;
        self.vds_prev = self.vds;
        self.vbs_prev = self.vbs;
        self.eval_vgs_prev = self.eval_vgs;
        self.eval_vds_prev = self.eval_vds;
        self.eval_vbs_prev = self.eval_vbs;
        self.id_prev = self.id;
        self.gm_prev = self.gm;
        self.gds_prev = self.gds;
        self.gmb_prev = self.gmb;
        (self.ibs_prev, self.gbs_prev) =
            self.body_source_junction_current_and_conductance(self.eval_vbs_prev);
        (self.ibd_prev, self.gbd_prev) = self
            .body_drain_junction_current_and_conductance(self.eval_vds_prev, self.eval_vbs_prev);

        let (vgs, vds, vbs) = self.branch_voltages(voltages);
        let (eval_vgs, eval_vds, eval_vbs) = self.limited_branch_voltages_for_eval(vgs, vds, vbs);
        self.vgs = vgs;
        self.vds = vds;
        self.vbs = vbs;
        self.eval_vgs = eval_vgs;
        self.eval_vds = eval_vds;
        self.eval_vbs = eval_vbs;

        let (id, region, gm, gds, gmb, id_eq) =
            self.linearized_operating_point(self.eval_vgs, self.eval_vds, self.eval_vbs);
        self.id = id;
        self.region = region;
        self.gm = gm;
        self.gds = gds;
        self.gmb = gmb;
        self.id_eq = id_eq;
        self.has_branch_history = true;
    }

    fn stamp_nonlinear(
        &self,
        voltages: &[Value],
        matrix: &mut impl MatrixStamper,
        _rhs: &mut [Value],
    ) {
        let (vgs, vds, vbs) = self.branch_voltages(voltages);
        let (eval_vgs, eval_vds, eval_vbs) = self.limited_branch_voltages_for_eval(vgs, vds, vbs);
        let (gm, gds, gmb, id_eq) = if self.cached_linearization_matches(vgs, vds, vbs) {
            (self.gm, self.gds, self.gmb, self.id_eq)
        } else {
            let (_, _, gm, gds, gmb, id_eq) =
                self.linearized_operating_point(eval_vgs, eval_vds, eval_vbs);
            (gm, gds, gmb, id_eq)
        };

        // Stamp the linearized model (Gate draws no DC current)
        // Drain node equation
        matrix.stamp(self.node_drain, self.node_drain, gds);
        matrix.stamp(self.node_drain, self.node_gate, gm);
        matrix.stamp(self.node_drain, self.node_source, -gm - gds - gmb);
        matrix.stamp(self.node_drain, self.node_bulk, gmb);

        // Source node equation (current exits source)
        matrix.stamp(self.node_source, self.node_drain, -gds);
        matrix.stamp(self.node_source, self.node_gate, -gm);
        matrix.stamp(self.node_source, self.node_source, gm + gds + gmb);
        matrix.stamp(self.node_source, self.node_bulk, -gmb);

        // Stamp equivalent current source
        matrix.stamp_rhs(self.node_drain, -id_eq);
        matrix.stamp_rhs(self.node_source, id_eq);

        let (bs_anode, bs_cathode, gbs, ieq_bs) = self.body_source_junction_linearization(eval_vbs);
        Self::stamp_diode_linearization(matrix, bs_anode, bs_cathode, gbs, ieq_bs);

        let (bd_anode, bd_cathode, gbd, ieq_bd) =
            self.body_drain_junction_linearization(eval_vds, eval_vbs);
        Self::stamp_diode_linearization(matrix, bd_anode, bd_cathode, gbd, ieq_bd);
    }

    fn is_converged(&self, criteria: NonlinearConvergenceCriteria) -> bool {
        if !self.vgs.is_finite()
            || !self.vgs_prev.is_finite()
            || !self.vds.is_finite()
            || !self.vds_prev.is_finite()
            || !self.vbs.is_finite()
            || !self.vbs_prev.is_finite()
            || !self.eval_vgs.is_finite()
            || !self.eval_vgs_prev.is_finite()
            || !self.eval_vds.is_finite()
            || !self.eval_vds_prev.is_finite()
            || !self.eval_vbs.is_finite()
            || !self.eval_vbs_prev.is_finite()
            || !self.id.is_finite()
            || !self.id_prev.is_finite()
            || !self.gm_prev.is_finite()
            || !self.gds_prev.is_finite()
            || !self.gmb_prev.is_finite()
            || !self.ibs_prev.is_finite()
            || !self.gbs_prev.is_finite()
            || !self.ibd_prev.is_finite()
            || !self.gbd_prev.is_finite()
        {
            return false;
        }

        let reltol = criteria.relative_tolerance();
        let voltage_tol = criteria.voltage_tolerance();
        let current_tol = criteria.current_tolerance();

        let vgs_diff = (self.vgs - self.vgs_prev).abs();
        let vds_diff = (self.vds - self.vds_prev).abs();
        let vbs_diff = (self.vbs - self.vbs_prev).abs();

        let vgs_tol = reltol * self.vgs.abs().max(self.vgs_prev.abs()) + voltage_tol;
        let vds_tol = reltol * self.vds.abs().max(self.vds_prev.abs()) + voltage_tol;
        let vbs_tol = reltol * self.vbs.abs().max(self.vbs_prev.abs()) + voltage_tol;

        if !(vgs_diff < vgs_tol && vds_diff < vds_tol && vbs_diff < vbs_tol) {
            return false;
        }

        let drain_current_hat = self.id_prev
            + self.gm_prev * (self.eval_vgs - self.eval_vgs_prev)
            + self.gds_prev * (self.eval_vds - self.eval_vds_prev)
            + self.gmb_prev * (self.eval_vbs - self.eval_vbs_prev);
        let drain_current_tol = reltol * self.id.abs().max(drain_current_hat.abs()) + current_tol;
        if (drain_current_hat - self.id).abs() >= drain_current_tol {
            return false;
        }

        let (ibs, _) = self.body_source_junction_current_and_conductance(self.eval_vbs);
        let (ibd, _) =
            self.body_drain_junction_current_and_conductance(self.eval_vds, self.eval_vbs);
        let body_source_delta = self.body_source_diode_voltage(self.eval_vbs)
            - self.body_source_diode_voltage(self.eval_vbs_prev);
        let body_drain_delta = self.body_drain_diode_voltage(self.eval_vds, self.eval_vbs)
            - self.body_drain_diode_voltage(self.eval_vds_prev, self.eval_vbs_prev);
        let bulk_current = ibs + ibd;
        let bulk_current_hat = self.ibs_prev
            + self.ibd_prev
            + self.gbs_prev * body_source_delta
            + self.gbd_prev * body_drain_delta;
        let bulk_current_tol =
            reltol * bulk_current.abs().max(bulk_current_hat.abs()) + current_tol;

        (bulk_current_hat - bulk_current).abs() < bulk_current_tol
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::device::traits::MatrixStamper;
    use std::collections::HashMap;

    #[derive(Default)]
    struct CaptureMatrix {
        g: HashMap<(NodeId, NodeId), Value>,
        rhs: HashMap<NodeId, Value>,
    }

    fn configured_level6_transient_mos(mos_type: MosType) -> Mosfet {
        let mut params = HashMap::new();
        match mos_type {
            MosType::Nmos => {
                params.insert("VTO".to_string(), 0.69486);
                params.insert("GAMMA".to_string(), 0.60309);
                params.insert("PHI".to_string(), 1.0);
                params.insert("KC".to_string(), 3.8921e-05);
                params.insert("NC".to_string(), 1.1739);
                params.insert("KV".to_string(), 0.91602);
                params.insert("NV".to_string(), 0.87225);
                params.insert("LAMBDA0".to_string(), 0.013333);
                params.insert("LAMBDA1".to_string(), 0.0046901);
                params.insert("TOX".to_string(), 1.98e-8);
                params.insert("CGSO".to_string(), 3.93e-10);
                params.insert("CGDO".to_string(), 3.93e-10);
                params.insert("CGBO".to_string(), 0.0);
                params.insert("LD".to_string(), 0.1e-6);
                Mosfet::new_nmos("M1".to_string(), 3, 2, 1, 0)
                    .with_level(6)
                    .with_geometry(5.0e-6, 1.0e-6)
                    .with_params(&params)
            }
            MosType::Pmos => {
                params.insert("VTO".to_string(), -0.60865);
                params.insert("GAMMA".to_string(), 0.89213);
                params.insert("PHI".to_string(), 1.0);
                params.insert("KC".to_string(), 6.42696e-06);
                params.insert("NC".to_string(), 1.6536);
                params.insert("KV".to_string(), 0.92145);
                params.insert("NV".to_string(), 0.88345);
                params.insert("LAMBDA0".to_string(), 0.018966);
                params.insert("LAMBDA1".to_string(), 0.0084012);
                params.insert("TOX".to_string(), 1.98e-8);
                params.insert("CGSO".to_string(), 7.29e-10);
                params.insert("CGDO".to_string(), 7.29e-10);
                params.insert("CGBO".to_string(), 0.0);
                params.insert("LD".to_string(), 0.28e-6);
                Mosfet::new_pmos("M1".to_string(), 3, 2, 1, 0)
                    .with_level(6)
                    .with_geometry(5.0e-6, 1.2e-6)
                    .with_params(&params)
            }
        }
    }

    fn literal_qmeyer_half_caps(
        vgs: Value,
        vgd: Value,
        vgb: Value,
        von: Value,
        vdsat: Value,
        phi: Value,
        cox: Value,
    ) -> (Value, Value, Value) {
        const MAGIC_VDS: Value = 0.025;

        let _ = vgb;
        let vgst = vgs - von;
        let vds = vgs - vgd;
        let vdsat = vdsat.max(MAGIC_VDS);

        if vgst <= -phi {
            (0.0, 0.0, cox / 2.0)
        } else if vgst <= -phi / 2.0 {
            (0.0, 0.0, -vgst * cox / (2.0 * phi))
        } else if vgst <= 0.0 {
            let mut capgs = vgst * cox / (1.5 * phi) + cox / 3.0;
            let capgd = if vds >= vdsat {
                0.0
            } else {
                let vddif = 2.0 * vdsat - vds;
                let vddif1 = vdsat - vds;
                let vddif2 = vddif * vddif;
                let capgd = capgs * (1.0 - vdsat * vdsat / vddif2);
                capgs *= 1.0 - vddif1 * vddif1 / vddif2;
                capgd
            };
            let capgb = -vgst * cox / (2.0 * phi);
            (capgs, capgd, capgb)
        } else if vdsat <= vds {
            (cox / 3.0, 0.0, 0.0)
        } else {
            let vddif = 2.0 * vdsat - vds;
            let vddif1 = vdsat - vds;
            let vddif2 = vddif * vddif;
            let capgd = cox * (1.0 - vdsat * vdsat / vddif2) / 3.0;
            let capgs = cox * (1.0 - vddif1 * vddif1 / vddif2) / 3.0;
            (capgs, capgd, 0.0)
        }
    }

    fn literal_level6_transient_capacitances(
        m: &Mosfet,
        vgs: Value,
        vds: Value,
        vbs: Value,
    ) -> (Value, Value, Value) {
        let p = m.polarity();
        let vgs_m = p * vgs;
        let vds_m = p * vds;
        let vbs_m = p * vbs;
        let vbd_m = vbs_m - vds_m;
        let vgd_m = vgs_m - vds_m;
        let vgb_m = vgs_m - vbs_m;

        let mode = if vds_m >= 0.0 { 1.0 } else { -1.0 };
        let vdshere = vds_m * mode;
        let vbsvbd = if mode > 0.0 { vbs_m } else { vbd_m };
        let vg_active = if mode > 0.0 { vgs_m } else { vgd_m };

        let phi = m.phi.max(1e-12);
        let sqrt_phi = phi.sqrt();
        let sarg1 = if vbsvbd <= 0.0 {
            (phi - vbsvbd).max(0.0).sqrt()
        } else {
            (sqrt_phi - vbsvbd / (2.0 * sqrt_phi.max(1e-12))).max(0.0)
        };
        let von = p * m.vto + m.gamma * (sarg1 - sqrt_phi) - m.gamma1 * vbsvbd - m.sigma * vdshere;
        let vgon = (vg_active - von).max(0.0);
        let vdsat = if vgon > 0.0 {
            m.kv * vgon.powf(m.nv)
        } else {
            0.0
        };

        let oxide_cap = m.oxide_capacitance_total();
        let (cgs_half, cgd_half, cgb_half) = if mode > 0.0 {
            literal_qmeyer_half_caps(vgs_m, vgd_m, vgb_m, von, vdsat, phi, oxide_cap)
        } else {
            let (capgd_half, capgs_half, capgb_half) =
                literal_qmeyer_half_caps(vgd_m, vgs_m, vgb_m, von, vdsat, phi, oxide_cap);
            (capgs_half, capgd_half, capgb_half)
        };
        let (cgs_ov, cgd_ov, cgb_ov) = m.overlap_capacitances();

        (
            2.0 * cgs_half + cgs_ov,
            2.0 * cgd_half + cgd_ov,
            2.0 * cgb_half + cgb_ov,
        )
    }

    impl MatrixStamper for CaptureMatrix {
        fn stamp(&mut self, row: NodeId, col: NodeId, value: Value) {
            *self.g.entry((row, col)).or_insert(0.0) += value;
        }

        fn stamp_rhs(&mut self, index: NodeId, value: Value) {
            *self.rhs.entry(index).or_insert(0.0) += value;
        }
    }

    #[test]
    fn test_mosfet_creation() {
        let m = Mosfet::new_nmos("M1".to_string(), 3, 2, 1, 0);
        assert_eq!(m.mos_type, MosType::Nmos);
        assert_eq!(m.node_drain, 3);
        assert_eq!(m.node_gate, 2);
        assert_eq!(m.node_source, 1);
        assert_eq!(m.node_bulk, 0);
    }

    #[test]
    fn test_with_params_accepts_vth0_alias_for_vto() {
        let mut params = std::collections::HashMap::new();
        params.insert("VTH0".to_string(), 0.52);

        let m = Mosfet::new_nmos("M1".to_string(), 3, 2, 1, 0).with_params(&params);
        assert!((m.vto - 0.52).abs() < 1e-12);
    }

    #[test]
    fn test_with_params_accepts_vt0_alias_for_vto() {
        let mut params = std::collections::HashMap::new();
        params.insert("VT0".to_string(), -0.61);

        let m = Mosfet::new_pmos("M1".to_string(), 3, 2, 1, 0).with_params(&params);
        assert!((m.vto + 0.61).abs() < 1e-12);
    }

    #[test]
    fn test_with_params_accepts_uo_alias_and_derives_kp_from_tox() {
        let mut params = std::collections::HashMap::new();
        params.insert("UO".to_string(), 575.0);
        params.insert("TOX".to_string(), 0.11e-6);

        let m = Mosfet::new_nmos("M1".to_string(), 3, 2, 1, 0).with_params(&params);
        assert!((m.u0 - 575.0).abs() < 1e-12);
        // Expected order from U0*COX with TOX=0.11um is around 1.8e-5 A/V^2.
        assert!(
            m.kp > 1.5e-5 && m.kp < 2.2e-5,
            "unexpected derived KP={}",
            m.kp
        );
    }

    #[test]
    fn test_with_params_derives_gamma_and_phi_from_nsub_and_tox() {
        let mut params = std::collections::HashMap::new();
        params.insert("NSUB".to_string(), 2.2e15);
        params.insert("TOX".to_string(), 0.11e-6);

        let m = Mosfet::new_nmos("M1".to_string(), 3, 2, 1, 0).with_params(&params);
        assert!(
            m.phi > 0.5 && m.phi < 0.8,
            "unexpected derived PHI={}",
            m.phi
        );
        assert!(
            m.gamma > 0.6 && m.gamma < 1.2,
            "unexpected derived GAMMA={}",
            m.gamma
        );
    }

    #[test]
    fn test_with_params_applies_noise_coefficients() {
        let mut params = std::collections::HashMap::new();
        params.insert("KF".to_string(), 4e-24);
        params.insert("AF".to_string(), 1.3);
        params.insert("EF".to_string(), 1.1);
        params.insert("TNOIA".to_string(), 1.8);

        let m = Mosfet::new_nmos("M1".to_string(), 3, 2, 1, 0)
            .with_geometry(20e-6, 0.5e-6)
            .with_params(&params);

        let (kf, af, ef) = m
            .flicker_noise_coefficients()
            .expect("KF should enable flicker noise");
        let expected_kf = 4e-24 / (m.cox * m.w * m.l);
        assert!((kf - expected_kf).abs() / expected_kf < 1e-12);
        assert!((af - 1.3).abs() < 1e-12);
        assert!((ef - 1.1).abs() < 1e-12);
        assert!((m.channel_thermal_noise_gamma() - 1.2).abs() < 1e-12);
    }

    #[test]
    fn test_with_params_preserves_explicit_kp_gamma_phi_over_derivations() {
        let mut params = std::collections::HashMap::new();
        params.insert("U0".to_string(), 700.0);
        params.insert("TOX".to_string(), 0.09e-6);
        params.insert("NSUB".to_string(), 5.0e15);
        params.insert("KP".to_string(), 3.21e-5);
        params.insert("GAMMA".to_string(), 0.55);
        params.insert("PHI".to_string(), 0.71);

        let m = Mosfet::new_nmos("M1".to_string(), 3, 2, 1, 0).with_params(&params);
        assert!((m.kp - 3.21e-5).abs() < 1e-15);
        assert!((m.gamma - 0.55).abs() < 1e-15);
        assert!((m.phi - 0.71).abs() < 1e-15);
    }

    #[test]
    fn test_with_params_parses_bulk_junction_model_parameters() {
        let params = HashMap::from([
            ("IS".to_string(), 2.5e-13),
            ("JS".to_string(), 1.2e-8),
            ("CJ".to_string(), 4.1e-4),
            ("CJSW".to_string(), 3.7e-10),
            ("PB".to_string(), 0.92),
            ("MJ".to_string(), 0.41),
            ("MJSW".to_string(), 0.19),
            ("FC".to_string(), 0.42),
        ]);
        let m = Mosfet::new_nmos("M1".to_string(), 3, 2, 1, 0).with_params(&params);

        assert!((m.is_bulk - 2.5e-13).abs() < 1e-24);
        assert!((m.js_bulk - 1.2e-8).abs() < 1e-24);
        assert!((m.cj - 4.1e-4).abs() < 1e-18);
        assert!((m.cjsw - 3.7e-10).abs() < 1e-24);
        assert!((m.pb - 0.92).abs() < 1e-15);
        assert!((m.mj - 0.41).abs() < 1e-15);
        assert!((m.mjsw - 0.19).abs() < 1e-15);
        assert!((m.fc - 0.42).abs() < 1e-15);
    }

    #[test]
    fn test_with_params_parses_explicit_bulk_terminal_caps() {
        let params = HashMap::from([("CBD".to_string(), 4.5e-15), ("CBS".to_string(), 5.5e-15)]);
        let m = Mosfet::new_nmos("M1".to_string(), 3, 2, 1, 0).with_params(&params);

        assert_eq!(m.drain_bulk_cap_zero_bias, Some(4.5e-15));
        assert_eq!(m.source_bulk_cap_zero_bias, Some(5.5e-15));
    }

    #[test]
    fn test_with_instance_params_overrides_geometry() {
        let params = vec![("W".to_string(), 20e-6), ("L".to_string(), 0.25e-6)];
        let m = Mosfet::new_nmos("M1".to_string(), 3, 2, 1, 0).with_instance_params(&params);
        assert!((m.w - 20e-6).abs() < 1e-18);
        assert!((m.l - 0.25e-6).abs() < 1e-18);
    }

    #[test]
    fn test_with_instance_params_applies_multiplicity() {
        let params = vec![
            ("W".to_string(), 10e-6),
            ("M".to_string(), 3.0),
            ("NF".to_string(), 2.0),
        ];
        let m = Mosfet::new_nmos("M1".to_string(), 3, 2, 1, 0).with_instance_params(&params);
        assert!((m.w - 60e-6).abs() < 1e-18);
    }

    #[test]
    fn test_with_instance_params_parses_junction_geometry() {
        let params = vec![
            ("AD".to_string(), 1.5e-12),
            ("AS".to_string(), 1.7e-12),
            ("PD".to_string(), 6.2e-6),
            ("PS".to_string(), 6.8e-6),
        ];
        let m = Mosfet::new_nmos("M1".to_string(), 3, 2, 1, 0).with_instance_params(&params);

        assert!((m.drain_area - 1.5e-12).abs() < 1e-24);
        assert!((m.source_area - 1.7e-12).abs() < 1e-24);
        assert!((m.drain_perimeter - 6.2e-6).abs() < 1e-18);
        assert!((m.source_perimeter - 6.8e-6).abs() < 1e-18);
    }

    #[test]
    fn test_body_source_junction_capacitance_is_continuous_across_forward_corner() {
        let params = HashMap::from([
            ("PB".to_string(), 1.0),
            ("MJ".to_string(), 0.5),
            ("MJSW".to_string(), 0.33),
            ("FC".to_string(), 0.5),
            ("CBS".to_string(), 2.0e-12),
            ("CJSW".to_string(), 4.0e-10),
        ]);
        let m = Mosfet::new_nmos("M1".to_string(), 3, 2, 1, 0)
            .with_params(&params)
            .with_instance_params(&[("PS".to_string(), 8.0e-6)]);
        let corner = m.fc * m.pb;
        let (q_minus, c_minus) = m.body_source_junction_charge_and_capacitance_at(corner - 1e-9);
        let (q_plus, c_plus) = m.body_source_junction_charge_and_capacitance_at(corner + 1e-9);

        assert!(
            (q_plus - q_minus).abs() < 1e-18,
            "junction charge should stay continuous across the forward-cap corner: below={q_minus:.6e}, above={q_plus:.6e}"
        );
        assert!(
            (c_plus - c_minus).abs() < 1e-12,
            "junction capacitance should stay continuous across the forward-cap corner: below={c_minus:.6e}, above={c_plus:.6e}"
        );
    }

    #[test]
    fn test_is_converged_accepts_small_relative_branch_deltas() {
        let criteria = NonlinearConvergenceCriteria::voltage_only(1e-6);
        let mut m = Mosfet::new_nmos("M1".to_string(), 3, 2, 1, 0);
        m.vgs_prev = 1.0;
        m.vgs = 1.0008;
        m.vds_prev = 2.0;
        m.vds = 2.0008;
        m.vbs_prev = -0.4;
        m.vbs = -0.3996;

        assert!(
            m.is_converged(criteria),
            "relative tolerance should allow sub-millivolt branch deltas around operating bias"
        );
    }

    #[test]
    fn test_is_converged_rejects_large_branch_delta() {
        let criteria = NonlinearConvergenceCriteria::voltage_only(1e-6);
        let mut m = Mosfet::new_nmos("M1".to_string(), 3, 2, 1, 0);
        m.vgs_prev = 1.0;
        m.vgs = 1.01;
        m.vds_prev = 2.0;
        m.vds = 2.0;
        m.vbs_prev = 0.0;
        m.vbs = 0.0;

        assert!(
            !m.is_converged(criteria),
            "large branch-voltage jump must fail convergence"
        );
    }

    #[test]
    fn test_is_converged_rejects_body_voltage_jump() {
        let criteria = NonlinearConvergenceCriteria::voltage_only(1e-6);
        let mut m = Mosfet::new_nmos("M1".to_string(), 3, 2, 1, 0);
        m.vgs_prev = 1.2;
        m.vgs = 1.2001;
        m.vds_prev = 1.8;
        m.vds = 1.8001;
        m.vbs_prev = -0.2;
        m.vbs = -0.23;

        assert!(
            !m.is_converged(criteria),
            "body-bias discontinuities must participate in convergence checks"
        );
    }

    #[test]
    fn test_is_converged_rejects_non_finite_history() {
        let criteria = NonlinearConvergenceCriteria::voltage_only(1e-6);
        let mut m = Mosfet::new_nmos("M1".to_string(), 3, 2, 1, 0);
        m.vgs_prev = f64::NAN;
        m.vgs = 1.0;
        m.vds_prev = 2.0;
        m.vds = 2.0;
        m.vbs_prev = 0.0;
        m.vbs = 0.0;

        assert!(
            !m.is_converged(criteria),
            "non-finite branch history must force another Newton update"
        );
    }

    #[test]
    fn test_update_caches_linearized_operating_point() {
        let mut m = Mosfet::new_nmos("M1".to_string(), 3, 2, 1, 0);
        let voltages = vec![0.15, 1.35, 0.95];

        m.update(&voltages);

        let (id, region, gm, gds, gmb, id_eq) = m.linearized_operating_point(m.vgs, m.vds, m.vbs);
        assert!((m.id - id).abs() < 1e-18);
        assert_eq!(m.region, region);
        assert!((m.gm - gm).abs() < 1e-18);
        assert!((m.gds - gds).abs() < 1e-18);
        assert!((m.gmb - gmb).abs() < 1e-18);
        assert!((m.id_eq - id_eq).abs() < 1e-18);
    }

    #[test]
    fn test_stamp_nonlinear_recomputes_for_changed_voltage_context() {
        let mut m = Mosfet::new_nmos("M1".to_string(), 3, 2, 1, 0);
        let cached_voltages = vec![0.1, 1.2, 0.85];
        let live_voltages = vec![0.2, 1.35, 0.95];

        m.update(&cached_voltages);
        // Corrupt the cached linearization to prove stamping recomputes when
        // called with a different Newton state.
        m.gm = 123.0;
        m.gds = 456.0;
        m.gmb = 789.0;
        m.id_eq = 321.0;

        let (vgs, vds, vbs) = m.branch_voltages(&live_voltages);
        let (_, _, gm, gds, gmb, id_eq) = m.linearized_operating_point(vgs, vds, vbs);
        let (_bs_anode, bs_cathode, gbs, ieq_bs) = m.body_source_junction_linearization(vbs);
        let (_bd_anode, bd_cathode, gbd, ieq_bd) = m.body_drain_junction_linearization(vds, vbs);

        let mut matrix = CaptureMatrix::default();
        m.stamp_nonlinear(&live_voltages, &mut matrix, &mut []);

        let drain_diag = gds + if bd_cathode == 3 { gbd } else { 0.0 };
        let source_diag = gm + gds + gmb + if bs_cathode == 1 { gbs } else { 0.0 };
        let drain_rhs = -id_eq + if bd_cathode == 3 { ieq_bd } else { -ieq_bd };
        let source_rhs = id_eq + if bs_cathode == 1 { ieq_bs } else { -ieq_bs };

        assert!((matrix.g.get(&(3, 3)).copied().unwrap_or(0.0) - drain_diag).abs() < 1e-18);
        assert!((matrix.g.get(&(3, 2)).copied().unwrap_or(0.0) - gm).abs() < 1e-18);
        assert!((matrix.g.get(&(3, 1)).copied().unwrap_or(0.0) - (-gm - gds - gmb)).abs() < 1e-18);
        assert!((matrix.g.get(&(1, 3)).copied().unwrap_or(0.0) - (-gds)).abs() < 1e-18);
        assert!((matrix.g.get(&(1, 1)).copied().unwrap_or(0.0) - source_diag).abs() < 1e-18);
        assert!((matrix.rhs.get(&3).copied().unwrap_or(0.0) - drain_rhs).abs() < 1e-18);
        assert!((matrix.rhs.get(&1).copied().unwrap_or(0.0) - source_rhs).abs() < 1e-18);
    }

    #[test]
    fn test_stamp_nonlinear_includes_forward_nmos_bulk_source_diode() {
        let mut m = Mosfet::new_nmos("M1".to_string(), 3, 2, 1, 4);
        m.is_bulk = 1e-14;
        let voltages = vec![0.0, 0.0, 1.5, 0.8];
        let close = |actual: Value, expected: Value| {
            (actual - expected).abs() <= 1e-12 * expected.abs().max(1.0)
        };

        let (vgs, vds, vbs) = m.branch_voltages(&voltages);
        let (_, _, gm, gds, gmb, id_eq) = m.linearized_operating_point(vgs, vds, vbs);
        let (bs_anode, bs_cathode, gbs, ieq_bs) = m.body_source_junction_linearization(vbs);
        let (bd_anode, bd_cathode, gbd, ieq_bd) = m.body_drain_junction_linearization(vds, vbs);

        let mut matrix = CaptureMatrix::default();
        m.stamp_nonlinear(&voltages, &mut matrix, &mut []);

        assert_eq!((bs_anode, bs_cathode), (4, 1));
        assert!(
            gbs > 0.0,
            "forward-biased body/source diode must stamp conductance"
        );
        assert_eq!((bd_anode, bd_cathode), (4, 3));
        assert!(close(
            matrix.g.get(&(4, 4)).copied().unwrap_or(0.0),
            gbs + gbd
        ));
        assert!(close(matrix.g.get(&(4, 1)).copied().unwrap_or(0.0), -gbs));
        assert!(close(matrix.g.get(&(4, 3)).copied().unwrap_or(0.0), -gbd));
        assert!(close(
            matrix.g.get(&(1, 4)).copied().unwrap_or(0.0),
            -gmb - gbs
        ));
        assert!(close(
            matrix.g.get(&(1, 1)).copied().unwrap_or(0.0),
            gm + gds + gmb + gbs
        ));
        let bulk_rhs = matrix.rhs.get(&4).copied().unwrap_or(0.0);
        assert!(
            close(bulk_rhs, -ieq_bs - ieq_bd),
            "bulk rhs mismatch: actual={bulk_rhs:.16e} expected={:.16e}",
            -ieq_bs - ieq_bd
        );
        assert!(close(
            matrix.rhs.get(&1).copied().unwrap_or(0.0),
            id_eq + ieq_bs
        ));
        assert!(close(
            matrix.rhs.get(&3).copied().unwrap_or(0.0),
            -id_eq + ieq_bd
        ));
    }

    #[test]
    fn test_stamp_nonlinear_includes_forward_pmos_bulk_source_diode() {
        let mut m = Mosfet::new_pmos("M1".to_string(), 3, 2, 1, 4);
        m.is_bulk = 1e-14;
        let voltages = vec![0.8, 0.8, -0.5, 0.0];
        let close = |actual: Value, expected: Value| {
            (actual - expected).abs() <= 1e-12 * expected.abs().max(1.0)
        };

        let (vgs, vds, vbs) = m.branch_voltages(&voltages);
        let (_, _, gm, gds, gmb, id_eq) = m.linearized_operating_point(vgs, vds, vbs);
        let (bs_anode, bs_cathode, gbs, ieq_bs) = m.body_source_junction_linearization(vbs);
        let (bd_anode, bd_cathode, gbd, ieq_bd) = m.body_drain_junction_linearization(vds, vbs);

        let mut matrix = CaptureMatrix::default();
        m.stamp_nonlinear(&voltages, &mut matrix, &mut []);

        assert_eq!((bs_anode, bs_cathode), (1, 4));
        assert_eq!((bd_anode, bd_cathode), (3, 4));
        assert!(
            gbs > 0.0,
            "forward-biased PMOS source/body diode must stamp conductance"
        );
        assert!(close(
            matrix.g.get(&(1, 1)).copied().unwrap_or(0.0),
            gm + gds + gmb + gbs
        ));
        assert!(close(
            matrix.g.get(&(1, 4)).copied().unwrap_or(0.0),
            -gmb - gbs
        ));
        assert!(close(matrix.g.get(&(4, 1)).copied().unwrap_or(0.0), -gbs));
        assert!(close(
            matrix.g.get(&(4, 4)).copied().unwrap_or(0.0),
            gbs + gbd
        ));
        assert!(close(matrix.g.get(&(4, 3)).copied().unwrap_or(0.0), -gbd));
        assert!(close(
            matrix.rhs.get(&1).copied().unwrap_or(0.0),
            id_eq - ieq_bs
        ));
        assert!(close(
            matrix.rhs.get(&3).copied().unwrap_or(0.0),
            -id_eq - ieq_bd
        ));
        assert!(close(
            matrix.rhs.get(&4).copied().unwrap_or(0.0),
            ieq_bs + ieq_bd
        ));
    }

    #[test]
    fn test_mosfet_cutoff() {
        let m = Mosfet::new_nmos("M1".to_string(), 3, 2, 1, 0);

        // Vgs < Vth -> cutoff (with smooth transitions, current is very small but not exactly zero)
        let (id, region) = m.calculate_id(0.3, 5.0, 0.0);
        assert_eq!(region, MosRegion::Cutoff);
        assert!(
            id.abs() < 1e-6,
            "Cutoff current should be negligible, got {}",
            id
        );
    }

    #[test]
    fn test_mosfet_saturation() {
        let m = Mosfet::new_nmos("M1".to_string(), 3, 2, 1, 0);

        // Vgs > Vth, Vds > Vgs - Vth -> saturation
        let (id, region) = m.calculate_id(2.0, 5.0, 0.0);
        assert_eq!(region, MosRegion::Saturation);
        assert!(id > 0.0);
    }

    #[test]
    fn test_mosfet_linear() {
        let m = Mosfet::new_nmos("M1".to_string(), 3, 2, 1, 0);

        // Vgs > Vth, Vds < Vgs - Vth -> linear
        let (id, region) = m.calculate_id(3.0, 0.5, 0.0);
        assert_eq!(region, MosRegion::Linear);
        assert!(id > 0.0);
    }

    #[test]
    fn test_pmos_polarity() {
        let nmos = Mosfet::new_nmos("M1".to_string(), 3, 2, 1, 0);
        let pmos = Mosfet::new_pmos("M2".to_string(), 3, 2, 1, 0);

        assert_eq!(nmos.polarity(), 1.0);
        assert_eq!(pmos.polarity(), -1.0);
    }

    #[test]
    fn test_bsim3_short_channel() {
        let m_level1 = Mosfet::new_nmos("M1".to_string(), 3, 2, 1, 0);
        let m_bsim3 = Mosfet::new_nmos("M2".to_string(), 3, 2, 1, 0).with_level(3);

        // Both should work in saturation
        let (id1, region1) = m_level1.calculate_id(2.0, 5.0, 0.0);
        let (id3, region3) = m_bsim3.calculate_id(2.0, 5.0, 0.0);

        assert_eq!(region1, MosRegion::Saturation);
        assert_eq!(region3, MosRegion::Saturation);
        assert!(id1 > 0.0);
        assert!(id3 > 0.0);

        // BSIM3 should have lower current due to mobility degradation
        // (for typical parameters at high Vgs)
    }

    #[test]
    fn test_bsim3_velocity_saturation() {
        let m = Mosfet::new_nmos("M1".to_string(), 3, 2, 1, 0)
            .with_level(3)
            .with_geometry(10e-6, 0.1e-6); // Short channel

        // High drive should show velocity saturation effects
        let (id, region) = m.calculate_id(3.0, 3.0, 0.0);
        assert_eq!(region, MosRegion::Saturation);
        assert!(id > 0.0);
    }

    //=========================================================================
    // C1 Continuity Tests - Verify smooth transitions across region boundaries
    //=========================================================================

    #[test]
    fn test_id_continuity_at_threshold() {
        // Test that drain current is continuous as Vgs crosses threshold
        let m = Mosfet::new_nmos("M1".to_string(), 3, 2, 1, 0);
        let vth = m.vto; // ~0.7V
        let vds = 2.0;
        let vbs = 0.0;

        // Sample around the threshold with fine resolution
        let delta = 0.001; // 1mV steps
        let mut prev_id: Option<Value> = None;

        for i in -50..=50 {
            let vgs = vth + (i as f64) * delta;
            let (id, _) = m.calculate_id(vgs, vds, vbs);

            if let Some(prev) = prev_id {
                let change = (id - prev).abs();
                // Current should change smoothly, not jump
                assert!(
                    change < 1e-4,
                    "Discontinuity at Vgs={:.4}: Id jumped by {:.2e}",
                    vgs,
                    change
                );
            }
            prev_id = Some(id);
        }
    }

    #[test]
    fn test_id_continuity_at_saturation_boundary() {
        // Test that drain current is continuous as Vds crosses Vdsat (linear <-> saturation)
        let m = Mosfet::new_nmos("M1".to_string(), 3, 2, 1, 0);
        let vgs = 2.0;
        let vbs = 0.0;
        let vdsat = vgs - m.vto; // ~1.3V

        let delta = 0.001;
        let mut prev_id: Option<Value> = None;

        for i in -50..=50 {
            let vds = vdsat + (i as f64) * delta;
            if vds <= 0.0 {
                continue;
            }

            let (id, _) = m.calculate_id(vgs, vds, vbs);

            if let Some(prev) = prev_id {
                let change = (id - prev).abs();
                assert!(
                    change < 1e-4,
                    "Discontinuity at Vds={:.4}: Id jumped by {:.2e}",
                    vds,
                    change
                );
            }
            prev_id = Some(id);
        }
    }

    #[test]
    fn test_gm_continuity() {
        // Test that transconductance gm is continuous (C1 of Id)
        let m = Mosfet::new_nmos("M1".to_string(), 3, 2, 1, 0);
        let vth = m.vto;
        let vds = 2.0;
        let vbs = 0.0;

        let delta = 0.001;
        let mut prev_gm: Option<Value> = None;

        for i in -50..=50 {
            let vgs = vth + (i as f64) * delta;
            let gm = m.small_signal(vgs, vds, vbs).0;

            if let Some(prev) = prev_gm {
                let change = (gm - prev).abs();
                // gm should change smoothly
                assert!(
                    change < 1e-3,
                    "gm discontinuity at Vgs={:.4}: jumped by {:.2e}",
                    vgs,
                    change
                );
            }
            prev_gm = Some(gm);
        }
    }

    #[test]
    fn test_gds_continuity() {
        // Test that output conductance gds is continuous
        let m = Mosfet::new_nmos("M1".to_string(), 3, 2, 1, 0);
        let vgs = 2.0;
        let vbs = 0.0;
        let vdsat = vgs - m.vto;

        let delta = 0.001;
        let mut prev_gds: Option<Value> = None;

        for i in -50..=50 {
            let vds = vdsat + (i as f64) * delta;
            if vds <= 0.0 {
                continue;
            }

            let gds = m.small_signal(vgs, vds, vbs).1;

            if let Some(prev) = prev_gds {
                let change = (gds - prev).abs();
                assert!(
                    change < 1e-3,
                    "gds discontinuity at Vds={:.4}: jumped by {:.2e}",
                    vds,
                    change
                );
            }
            prev_gds = Some(gds);
        }
    }

    #[test]
    fn test_pmos_with_swapped_terminals_conducts() {
        // Regression for decks that instantiate PMOS with D/S reversed.
        // Example operating point: Vd=5V, Vs=0V, Vg=0V, Vb=5V.
        let mut m = Mosfet::new_pmos("M1".to_string(), 3, 2, 1, 0);
        m.vto = -0.8;
        m.kp = 21e-6;
        m.gamma = 0.45;
        m.phi = 0.61;
        m.lambda = 0.0;

        // In original variables: vgs = Vg-Vs = 0, vds = Vd-Vs = 5, vbs = Vb-Vs = 5.
        let (id, region) = m.calculate_id(0.0, 5.0, 5.0);
        assert!(
            id.abs() > 1e-4,
            "PMOS should conduct strongly with swapped D/S, got Id={id}"
        );
        assert_ne!(region, MosRegion::Cutoff);
    }

    #[test]
    fn test_pmos_with_swapped_terminals_turns_off_when_gate_high() {
        // With D/S reversed and gate tied high (to the intrinsic source), PMOS
        // should be off: Vsg ~= 0.
        let mut m = Mosfet::new_pmos("M1".to_string(), 3, 2, 1, 0);
        m.vto = -0.8;
        m.kp = 21e-6;
        m.gamma = 0.45;
        m.phi = 0.61;
        m.lambda = 0.0;

        // Original-variable bias for Vd=5V, Vs=0V, Vg=5V, Vb=5V.
        let (id, _) = m.calculate_id(5.0, 5.0, 5.0);
        assert!(
            id.abs() < 1e-6,
            "PMOS should be off for Vsg~0 in swapped orientation, got Id={id}"
        );
    }

    #[test]
    fn test_source_drain_permutation_invariance_level1() {
        let mut m = Mosfet::new_nmos("M1".to_string(), 3, 2, 1, 0);
        m.lambda = 0.0;

        // Symmetry identity:
        // Id(Vgs, Vds, Vbs) = -Id(Vgs - Vds, -Vds, Vbs - Vds)
        let test_points = [
            (2.0, 1.0, 0.0),
            (1.8, -0.8, -0.2),
            (0.6, 0.2, 0.0),
            (2.5, -1.7, -0.4),
        ];

        for (vgs, vds, vbs) in test_points {
            let (id_a, _) = m.calculate_id(vgs, vds, vbs);
            let (id_b, _) = m.calculate_id(vgs - vds, -vds, vbs - vds);
            let err = (id_a + id_b).abs();
            let scale = id_a.abs().max(id_b.abs());
            let tol = 1e-12 + 1e-3 * scale;
            assert!(
                err <= tol,
                "Permutation invariance failed at (vgs={vgs}, vds={vds}, vbs={vbs}): \
                 Id={id_a}, swapped={id_b}, err={err}, tol={tol}"
            );
        }
    }

    #[test]
    fn test_small_signal_matches_numeric_derivatives_in_swapped_mode() {
        let mut m = Mosfet::new_pmos("M1".to_string(), 3, 2, 1, 0);
        m.vto = -0.8;
        m.kp = 21e-6;
        m.gamma = 0.45;
        m.phi = 0.61;
        m.lambda = 0.0;

        let vgs = 0.0;
        let vds = 5.0;
        let vbs = 5.0;
        let (gm, gds, gmb) = m.small_signal(vgs, vds, vbs);

        let h = 1e-6;
        let id = |vgs: Value, vds: Value, vbs: Value| -> Value { m.calculate_id(vgs, vds, vbs).0 };
        let gm_num = (id(vgs + h, vds, vbs) - id(vgs - h, vds, vbs)) / (2.0 * h);
        let gds_num = (id(vgs, vds + h, vbs) - id(vgs, vds - h, vbs)) / (2.0 * h);
        let gmb_num = (id(vgs, vds, vbs + h) - id(vgs, vds, vbs - h)) / (2.0 * h);

        let rel = |a: Value, b: Value| -> Value { (a - b).abs() / (b.abs().max(1e-12)) };
        assert!(
            rel(gm, gm_num) < 2e-1,
            "gm mismatch in swapped mode: analytical={gm}, numeric={gm_num}"
        );
        assert!(
            rel(gds, gds_num) < 2e-1,
            "gds mismatch in swapped mode: analytical={gds}, numeric={gds_num}"
        );
        assert!(
            rel(gmb, gmb_num) < 2e-1,
            "gmb mismatch in swapped mode: analytical={gmb}, numeric={gmb_num}"
        );
    }

    #[test]
    fn test_level6_pmos_gate_low_turns_on_with_folded_model_space() {
        let mut m = Mosfet::new_pmos("M1".to_string(), 3, 2, 1, 0).with_level(6);
        m.vto = -0.60865;
        m.kc = 6.42696e-06;
        m.nc = 1.6536;
        m.kv = 0.92145;
        m.nv = 0.88345;
        m.lambda0 = 0.018966;
        m.lambda1 = 0.0084012;
        m.gamma = 0.89213;
        m.phi = 1.0;
        m.ld = 0.28e-6;

        let (id, region) = m.calculate_id(-5.0, -5.0, 0.0);
        assert!(
            id < -1e-6,
            "Level-6 PMOS should source current with gate low and source high, got Id={id}"
        );
        assert_eq!(region, MosRegion::Saturation);
    }

    #[test]
    fn test_level6_pmos_gate_tied_to_source_turns_off() {
        let mut m = Mosfet::new_pmos("M1".to_string(), 3, 2, 1, 0).with_level(6);
        m.vto = -0.60865;
        m.kc = 6.42696e-06;
        m.nc = 1.6536;
        m.kv = 0.92145;
        m.nv = 0.88345;
        m.lambda0 = 0.018966;
        m.lambda1 = 0.0084012;
        m.gamma = 0.89213;
        m.phi = 1.0;
        m.ld = 0.28e-6;

        let (id, region) = m.calculate_id(0.0, -5.0, 0.0);
        assert!(
            id.abs() < 1e-18,
            "Level-6 PMOS should be in cutoff when gate is tied to source, got Id={id}"
        );
        assert_eq!(region, MosRegion::Cutoff);
    }

    #[test]
    fn test_level6_pmos_inverse_mode_conducts_with_drain_above_source() {
        let mut m = Mosfet::new_pmos("M1".to_string(), 3, 2, 1, 0).with_level(6);
        m.vto = -0.60865;
        m.kc = 6.42696e-06;
        m.nc = 1.6536;
        m.kv = 0.92145;
        m.nv = 0.88345;
        m.lambda0 = 0.018966;
        m.lambda1 = 0.0084012;
        m.gamma = 0.89213;
        m.phi = 1.0;
        m.ld = 0.28e-6;

        let (id, region) = m.calculate_id(-4.0, 1.0, 0.0);
        assert!(
            id > 1e-8,
            "Level-6 PMOS should conduct in inverse mode when the drain rises above the declared source, got Id={id}"
        );
        assert!(matches!(region, MosRegion::Linear | MosRegion::Saturation));
    }

    #[test]
    fn test_level6_small_signal_matches_numeric_derivatives_in_inverse_mode() {
        let mut m = Mosfet::new_pmos("M1".to_string(), 3, 2, 1, 0).with_level(6);
        m.vto = -0.60865;
        m.kc = 6.42696e-06;
        m.nc = 1.6536;
        m.kv = 0.92145;
        m.nv = 0.88345;
        m.lambda0 = 0.018966;
        m.lambda1 = 0.0084012;
        m.gamma = 0.89213;
        m.phi = 1.0;
        m.ld = 0.28e-6;

        let vgs = -4.0;
        let vds = 1.0;
        let vbs = 0.0;
        let (gm, gds, gmb) = m.small_signal(vgs, vds, vbs);

        let h = 1e-5;
        let id = |vgs: Value, vds: Value, vbs: Value| -> Value { m.calculate_id(vgs, vds, vbs).0 };
        let gm_num = (id(vgs + h, vds, vbs) - id(vgs - h, vds, vbs)) / (2.0 * h);
        let gds_num = (id(vgs, vds + h, vbs) - id(vgs, vds - h, vbs)) / (2.0 * h);
        let gmb_num = (id(vgs, vds, vbs + h) - id(vgs, vds, vbs - h)) / (2.0 * h);

        let rel = |a: Value, b: Value| -> Value { (a - b).abs() / b.abs().max(1e-12) };
        assert!(
            gm < 0.0,
            "expected inverse-mode PMOS gm to be negative, got {gm}"
        );
        assert!(
            gds > 0.0,
            "expected inverse-mode PMOS gds to be positive, got {gds}"
        );
        assert!(
            rel(gm, gm_num) < 1e-2,
            "gm mismatch: analytical={gm}, numeric={gm_num}"
        );
        assert!(
            rel(gds, gds_num) < 1e-2,
            "gds mismatch: analytical={gds}, numeric={gds_num}"
        );
        assert!(
            rel(gmb, gmb_num) < 1e-2,
            "gmb mismatch: analytical={gmb}, numeric={gmb_num}"
        );
    }

    #[test]
    fn test_level6_ld_shortens_effective_channel_and_increases_drive_current() {
        let mut base = Mosfet::new_nmos("M1".to_string(), 3, 2, 1, 0).with_level(6);
        base.vto = 0.69486;
        base.kc = 3.8921e-05;
        base.nc = 1.1739;
        base.kv = 0.91602;
        base.nv = 0.87225;
        base.lambda0 = 0.013333;
        base.lambda1 = 0.0046901;
        base.gamma = 0.60309;
        base.phi = 1.0;
        base.l = 1.0e-6;
        base.w = 5.0e-6;

        let mut with_ld = base.clone();
        with_ld.ld = 0.1e-6;

        let id_nominal = base.calculate_id(5.0, 5.0, 0.0).0;
        let id_with_ld = with_ld.calculate_id(5.0, 5.0, 0.0).0;

        assert!(
            id_with_ld > id_nominal * 1.2,
            "expected positive LD to shorten Leff and increase Level-6 drive current, got nominal={id_nominal}, with_ld={id_with_ld}"
        );
    }

    #[test]
    fn test_level6_transient_capacitances_match_qmeyer_cutoff_limit() {
        let mut m = Mosfet::new_nmos("M1".to_string(), 3, 2, 1, 0).with_level(6);
        m.vto = 0.69486;
        m.gamma = 0.60309;
        m.phi = 1.0;
        m.l = 1.0e-6;
        m.w = 5.0e-6;
        m.ld = 0.1e-6;
        m.cgso = 2.0e-10;
        m.cgdo = 3.0e-10;
        m.cgbo = 4.0e-10;
        m.cox = 7.0e-3;

        let (cgs, cgd, cgb) = m.transient_capacitances_at(-0.5, 1.0, 0.0);
        let oxide_cap = m.oxide_capacitance_total();
        let (cgs_ov, cgd_ov, cgb_ov) = m.overlap_capacitances();

        assert!(
            (cgs - cgs_ov).abs() < 1e-24,
            "expected cutoff Cgs to be overlap only"
        );
        assert!(
            (cgd - cgd_ov).abs() < 1e-24,
            "expected cutoff Cgd to be overlap only"
        );
        assert!(
            (cgb - (oxide_cap + cgb_ov)).abs() < 1e-24,
            "expected cutoff Cgb to match qmeyer TRANOP total oxide limit, got {cgb}"
        );
    }

    #[test]
    fn test_level6_transient_capacitances_swap_source_and_drain_in_inverse_mode() {
        let mut m = Mosfet::new_nmos("M1".to_string(), 3, 2, 1, 0).with_level(6);
        m.vto = 0.69486;
        m.kc = 3.8921e-05;
        m.nc = 1.1739;
        m.kv = 0.91602;
        m.nv = 0.87225;
        m.lambda0 = 0.013333;
        m.lambda1 = 0.0046901;
        m.gamma = 0.60309;
        m.phi = 1.0;
        m.l = 1.0e-6;
        m.w = 5.0e-6;
        m.ld = 0.1e-6;

        let forward = m.transient_capacitances_at(2.0, 0.2, 0.0);
        let inverse = m.transient_capacitances_at(2.0, -0.2, 0.0);

        assert!(
            forward.0 > forward.1,
            "expected forward-mode Meyer charge to couple more strongly to source than drain, got {:?}",
            forward
        );
        assert!(
            inverse.1 > inverse.0,
            "expected inverse-mode Meyer charge to swap source/drain weighting, got {:?}",
            inverse
        );
    }

    #[test]
    fn test_level6_transient_capacitances_match_literal_ngspice_qmeyer_grid() {
        for mos_type in [MosType::Nmos, MosType::Pmos] {
            let m = configured_level6_transient_mos(mos_type);
            for &vgs in &[-5.0, -2.0, -0.5, 0.0, 0.5, 1.0, 2.5, 5.0] {
                for &vds in &[-5.0, -2.0, -0.5, -0.05, 0.0, 0.05, 0.5, 1.0, 2.5, 5.0] {
                    for &vbs in &[-2.0, -1.0, -0.2, 0.0, 0.2, 1.0, 2.0] {
                        let actual = m.transient_capacitances_at(vgs, vds, vbs);
                        let expected = literal_level6_transient_capacitances(&m, vgs, vds, vbs);
                        let actual_vals = [actual.0, actual.1, actual.2];
                        let expected_vals = [expected.0, expected.1, expected.2];
                        let labels = ["cgs", "cgd", "cgb"];
                        for idx in 0..3 {
                            let abs_err = (actual_vals[idx] - expected_vals[idx]).abs();
                            let scale = expected_vals[idx].abs().max(1e-18);
                            assert!(
                                abs_err / scale < 1e-12,
                                "{mos_type:?} {} mismatch at vgs={vgs}, vds={vds}, vbs={vbs}: expected={:?}, actual={:?}",
                                labels[idx],
                                expected,
                                actual
                            );
                        }
                    }
                }
            }
        }
    }

    #[test]
    fn test_update_preserves_previous_limited_linearization_for_convergence() {
        let mut m = configured_level6_transient_mos(MosType::Nmos);

        let first = vec![0.1, 1.8, 1.0];
        m.update(&first);
        let saved_eval_vgs = m.eval_vgs;
        let saved_eval_vds = m.eval_vds;
        let saved_eval_vbs = m.eval_vbs;
        let saved_id = m.id;
        let saved_gm = m.gm;
        let saved_gds = m.gds;
        let saved_gmb = m.gmb;

        let second = vec![0.2, 1.9, 1.1];
        m.update(&second);

        assert!((m.eval_vgs_prev - saved_eval_vgs).abs() < 1e-18);
        assert!((m.eval_vds_prev - saved_eval_vds).abs() < 1e-18);
        assert!((m.eval_vbs_prev - saved_eval_vbs).abs() < 1e-18);
        assert!((m.id_prev - saved_id).abs() < 1e-18);
        assert!((m.gm_prev - saved_gm).abs() < 1e-18);
        assert!((m.gds_prev - saved_gds).abs() < 1e-18);
        assert!((m.gmb_prev - saved_gmb).abs() < 1e-18);
    }

    #[test]
    fn test_eval_branch_voltages_at_uses_cached_limited_state_for_matching_solution() {
        use crate::device::NonlinearDevice;

        let mut m = configured_level6_transient_mos(MosType::Nmos);
        m.update(&[0.0, 0.0, 0.0]);

        let solution = [0.0, 5.0, 5.0];
        m.update(&solution);

        let raw = m.branch_voltages_at(&solution);
        let eval = m.eval_branch_voltages_at(&solution);
        let charge = m.gate_charge_branch_voltages_at(&solution);

        assert!((eval.0 - m.eval_vgs).abs() < 1e-18);
        assert!((eval.1 - m.eval_vds).abs() < 1e-18);
        assert!((eval.2 - m.eval_vbs).abs() < 1e-18);
        assert!(
            (eval.0 - raw.0).abs() > 1e-6
                || (eval.1 - raw.1).abs() > 1e-6
                || (eval.2 - raw.2).abs() > 1e-6,
            "expected limiter-clipped eval voltages to differ from raw branch voltages"
        );
        assert!((charge.0 - eval.0).abs() < 1e-18);
        assert!((charge.1 - (eval.0 - eval.1)).abs() < 1e-18);
        assert!((charge.2 - (eval.0 - eval.2)).abs() < 1e-18);
    }

    #[test]
    fn test_eval_branch_voltages_at_recomputes_when_cached_state_is_stale() {
        use crate::device::NonlinearDevice;

        let mut m = configured_level6_transient_mos(MosType::Nmos);
        let stale_solution = [0.0, 5.0, 5.0];
        m.update(&stale_solution);

        let query_solution = [0.2, 1.4, 1.1];
        let raw = m.branch_voltages_at(&query_solution);
        let expected = m.limited_branch_voltages_for_eval(raw.0, raw.1, raw.2);
        let actual = m.eval_branch_voltages_at(&query_solution);
        let charge = m.gate_charge_branch_voltages_at(&query_solution);

        assert!((actual.0 - expected.0).abs() < 1e-18);
        assert!((actual.1 - expected.1).abs() < 1e-18);
        assert!((actual.2 - expected.2).abs() < 1e-18);
        assert!(
            (actual.0 - m.eval_vgs).abs() > 1e-6
                || (actual.1 - m.eval_vds).abs() > 1e-6
                || (actual.2 - m.eval_vbs).abs() > 1e-6,
            "expected helper to recompute limited eval state instead of reusing stale cache"
        );
        assert!((charge.0 - actual.0).abs() < 1e-18);
        assert!((charge.1 - (actual.0 - actual.1)).abs() < 1e-18);
        assert!((charge.2 - (actual.0 - actual.2)).abs() < 1e-18);
    }

    #[test]
    fn test_mos_convergence_rejects_drain_current_prediction_mismatch() {
        let mut m = Mosfet::new_nmos("M1".to_string(), 3, 2, 1, 0);
        let criteria = NonlinearConvergenceCriteria::new(1e-6, 1e-12, 1e-3);

        m.vgs_prev = 1.0;
        m.vds_prev = 1.0;
        m.vbs_prev = 0.0;
        m.vgs = 1.0 + 1e-7;
        m.vds = 1.0;
        m.vbs = 0.0;
        m.eval_vgs_prev = 1.0;
        m.eval_vds_prev = 1.0;
        m.eval_vbs_prev = 0.0;
        m.eval_vgs = 1.0 + 1e-7;
        m.eval_vds = 1.0;
        m.eval_vbs = 0.0;
        m.id_prev = 1e-3;
        m.gm_prev = 5e-4;
        m.gds_prev = 1e-5;
        m.gmb_prev = 0.0;
        m.id = 2e-3;
        m.ibs_prev = 0.0;
        m.gbs_prev = 0.0;
        m.ibd_prev = 0.0;
        m.gbd_prev = 0.0;

        assert!(!m.is_converged(criteria));
    }

    #[test]
    fn test_mos_convergence_accepts_consistent_predicted_currents() {
        let mut m = Mosfet::new_nmos("M1".to_string(), 3, 2, 1, 0);
        let criteria = NonlinearConvergenceCriteria::new(1e-6, 1e-12, 1e-3);
        let delta_vgs = 1e-7;
        let delta_vds = -2e-7;
        let delta_vbs = 5e-8;

        m.vgs_prev = 1.0;
        m.vds_prev = 1.0;
        m.vbs_prev = -0.2;
        m.vgs = m.vgs_prev + delta_vgs;
        m.vds = m.vds_prev + delta_vds;
        m.vbs = m.vbs_prev + delta_vbs;
        m.eval_vgs_prev = m.vgs_prev;
        m.eval_vds_prev = m.vds_prev;
        m.eval_vbs_prev = m.vbs_prev;
        m.eval_vgs = m.vgs;
        m.eval_vds = m.vds;
        m.eval_vbs = m.vbs;
        m.id_prev = 1e-3;
        m.gm_prev = 5e-4;
        m.gds_prev = 2e-4;
        m.gmb_prev = 1e-4;
        m.id = m.id_prev + m.gm_prev * delta_vgs + m.gds_prev * delta_vds + m.gmb_prev * delta_vbs;

        let (ibs_prev, gbs_prev) = m.body_source_junction_current_and_conductance(m.eval_vbs_prev);
        let (ibd_prev, gbd_prev) =
            m.body_drain_junction_current_and_conductance(m.eval_vds_prev, m.eval_vbs_prev);
        m.ibs_prev = ibs_prev;
        m.gbs_prev = gbs_prev;
        m.ibd_prev = ibd_prev;
        m.gbd_prev = gbd_prev;

        assert!(m.is_converged(criteria));
    }
}
