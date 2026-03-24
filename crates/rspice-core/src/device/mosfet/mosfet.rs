//! MOSFET (Metal-Oxide-Semiconductor Field-Effect Transistor) device model
//!
//! Implements a Level 1 SPICE MOSFET model (Shichman-Hodges).
//! Supports NMOS and PMOS devices in cutoff, linear, and saturation regions.

use super::smooth::{SMOOTH_VOLTAGE, smooth_max, smooth_min, smooth_positive, smooth_step};
use crate::device::traits::{MatrixStamper, NonlinearDevice};
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
    /// Oxide capacitance per unit area (COX)
    pub cox: Value,

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

    // Level 6 (double-exponent/simplified) MOSFET parameters
    /// Current gain coefficient (KC) - drain current multiplier
    pub kc: Value,
    /// Current gain exponent (NC) - affects Vgs dependence
    pub nc: Value,
    /// Voltage clipping coefficient (KV) - saturation factor
    pub kv: Value,
    /// Voltage clipping exponent (NV) - affects Vds dependence  
    pub nv: Value,
    /// First-order channel length modulation (LAMBDA0)
    pub lambda0: Value,
    /// Second-order channel length modulation (LAMBDA1)
    pub lambda1: Value,

    // Operating point values
    vgs: Value,
    vds: Value,
    vbs: Value,
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

            // Level 1 parameters
            vto: 0.7,       // Threshold voltage
            kp: 110e-6,     // Transconductance (NMOS typical)
            gamma: 0.4,     // Body effect
            phi: 0.65,      // Surface potential
            lambda: 0.01,   // Channel-length modulation
            is_bulk: 1e-14, // Bulk diode saturation current
            cox: 7e-4,      // Oxide capacitance

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

            // Level 6 parameters (double-exponent model)
            kc: 110e-6,    // Current gain (similar to KP)
            nc: 1.0,       // Current exponent
            kv: 0.9,       // Voltage clipping coefficient
            nv: 0.9,       // Voltage exponent
            lambda0: 0.01, // First-order CLM
            lambda1: 0.0,  // Second-order CLM

            vgs: 0.0,
            vds: 0.0,
            vbs: 0.0,
            id: 0.0,
            gm: 0.0,
            gds: 0.0,
            gmb: 0.0,
            id_eq: 0.0,
            region: MosRegion::Cutoff,

            vgs_prev: 0.0,
            vds_prev: 0.0,
            vbs_prev: 0.0,
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
    fn linearized_operating_point(
        &self,
        vgs: Value,
        vds: Value,
        vbs: Value,
    ) -> (Value, MosRegion, Value, Value, Value, Value) {
        let (id, region) = self.calculate_id(vgs, vds, vbs);
        let (gm, gds, gmb) = self.small_signal(vgs, vds, vbs);
        let id_eq = id - gm * vgs - gds * vds - gmb * vbs;
        (id, region, gm, gds, gmb, id_eq)
    }

    #[inline]
    fn cached_linearization_matches(&self, vgs: Value, vds: Value, vbs: Value) -> bool {
        self.vgs.is_finite()
            && self.vds.is_finite()
            && self.vbs.is_finite()
            && self.vgs == vgs
            && self.vds == vds
            && self.vbs == vbs
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
        if let Some(&v) = params.get("L") {
            self.l = v;
        }
        if let Some(&v) = params.get("W") {
            self.w = v;
        }
        if let Some(tox) = tox {
            self.cox = EPS_OX_REL * EPS0 / tox;
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
        if gamma_explicit.is_none() || phi_explicit.is_none() {
            if let Some(nsub_cm3) = nsub {
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
        if let Some(&v) = params.get("LAMBDA0") {
            self.lambda0 = v;
        } else if let Some(&v) = params.get("LAMBDA") {
            // Some model cards only provide legacy LAMBDA even for higher levels.
            self.lambda0 = v;
        }
        if let Some(&v) = params.get("LAMBDA1") {
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

            if name.eq_ignore_ascii_case("NF")
                && *value > 0.0 {
                    nf = *value;
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
        let (gm, gds, gmb, id_eq) = if self.cached_linearization_matches(vgs, vds, vbs) {
            (self.gm, self.gds, self.gmb, self.id_eq)
        } else {
            let (_, _, gm, gds, gmb, id_eq) = self.linearized_operating_point(vgs, vds, vbs);
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
            // Most model levels use polarity-folded equations and therefore a
            // positive PMOS threshold magnitude. Legacy Level-6 cards are
            // evaluated in an unfurled signed-voltage space and expect the
            // original signed VTO from the model card.
            MosType::Pmos => {
                if self.level == 6 {
                    self.vto
                } else {
                    self.vto.abs()
                }
            }
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

    /// Calculate overlap capacitances for AC analysis
    /// Returns (Cgs_overlap, Cgd_overlap, Cgb_overlap)
    pub fn overlap_capacitances(&self) -> (Value, Value, Value) {
        // Cgs_overlap = CGSO * W
        let cgs = self.cgso * self.w;
        // Cgd_overlap = CGDO * W
        let cgd = self.cgdo * self.w;
        // Cgb_overlap = CGBO * L
        let cgb = self.cgbo * self.l;

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
        let cox_wl = self.cox * self.w * self.l;

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
        let cox_wl = self.cox * self.w * self.l;

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
        // Legacy SPICE Level-6 equations are historically asymmetric and tuned
        // around the declared terminal ordering. Preserve that behavior to
        // match ngspice regression vectors.
        if self.level == 6 {
            return self.calculate_id_forward(vgs, vds, vbs);
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

    /// Level 6 (Double-Exponent Simplified) drain current calculation
    ///
    /// Level 6 MOSFET model uses empirical fits with double-exponent equations:
    /// - KC/NC control the current gain and gate voltage dependence
    /// - KV/NV control the saturation characteristics  
    /// - LAMBDA0/LAMBDA1 control channel length modulation
    ///
    /// Id = KC * W/L * (Vgs - Vth)^NC * (1 - exp(-Vds * KV))^NV * (1 + LAMBDA * Vds)
    fn calculate_id_level6(&self, vgs: Value, vds: Value, vbs: Value) -> (Value, MosRegion) {
        // Keep Level-6 in a legacy signed-voltage space (no PMOS polarity fold).
        let p = 1.0;
        let vgs_eff = p * vgs;
        let vds_eff = smooth_positive(p * vds, VDS_SMOOTHING);
        let vth = self.vth(vbs);

        // Gate overdrive with smooth cutoff transition
        let vgt_raw = vgs_eff - vth;
        let vgt = smooth_positive(vgt_raw, SMOOTH_VOLTAGE);

        // Determine effective region for reporting
        let region = if vgt_raw <= -SMOOTH_VOLTAGE {
            MosRegion::Cutoff
        } else if vds_eff < vgt - SMOOTH_VOLTAGE {
            MosRegion::Linear
        } else {
            MosRegion::Saturation
        };

        // Level 6 double-exponent model equations
        // Current gain term: KC * W/L * Vgt^NC
        let wl = self.wl_ratio();
        let vgt_pow = vgt.max(0.0);
        let current_term = if vgt_pow > 0.0 {
            self.kc * wl * vgt_pow.powf(self.nc)
        } else {
            0.0
        };

        // Saturation term: (1 - exp(-KV * Vds))^NV
        // This smoothly transitions from linear to saturation
        let exp_term = (-self.kv * vds_eff).exp();
        let sat_factor = (1.0 - exp_term).max(0.0).powf(self.nv);

        // Channel length modulation: 1 + LAMBDA0 * Vds + LAMBDA1 * Vds^2
        let clm = 1.0 + self.lambda0 * vds_eff + self.lambda1 * vds_eff * vds_eff;

        // Above-threshold branch
        let id_above = current_term * sat_factor * clm;

        // Weak-inversion tail (legacy decks rely on finite subthreshold leakage
        // for startup biasing and small-signal internal-node excursions).
        const THERMAL_VOLTAGE_300K: Value = 0.02585;
        let slope_n = self.nc.max(1.0);
        let exp_arg = (vgt_raw / (slope_n * THERMAL_VOLTAGE_300K)).clamp(-120.0, 60.0);
        let i_sub0 = 1e-12 * wl.max(1e-3);
        let id_sub = i_sub0 * exp_arg.exp() * sat_factor * clm;

        // Smoothly blend between weak and strong inversion around Vgt=0.
        let above_blend = smooth_step(vgt_raw, SMOOTH_VOLTAGE);
        let id = p * (above_blend * id_above + (1.0 - above_blend) * id_sub);

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
            return (
                self.gm_forward(vgs, vds, vbs),
                self.gds_forward(vgs, vds, vbs),
                self.gmb_forward(vgs, vds, vbs),
            );
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
        let p = if self.level == 6 {
            1.0
        } else {
            self.polarity()
        };
        let vgs_eff = p * vgs;
        let _vds_eff = smooth_positive(p * vds, VDS_SMOOTHING);
        let vth = self.vth(vbs);
        let vgt_raw = vgs_eff - vth;
        let vgt = smooth_positive(vgt_raw, SMOOTH_VOLTAGE);
        let dvgt_dvgs = smooth_step(vgt_raw, SMOOTH_VOLTAGE);

        // Level 6: gm = dId/dVgs = Id / Vgt * NC * dVgt/dVgs (when Vgt > 0)
        if self.level == 6 {
            let (id, _) = self.calculate_id_forward(vgs, vds, vbs);
            const THERMAL_VOLTAGE_300K: Value = 0.02585;
            let slope_n = self.nc.max(1.0);
            let above_blend = smooth_step(vgt_raw, SMOOTH_VOLTAGE);
            let gm_above = if vgt > 1e-12 {
                id.abs() / vgt * self.nc * dvgt_dvgs
            } else {
                0.0
            };
            let gm_sub = id.abs() / (slope_n * THERMAL_VOLTAGE_300K);
            let gm = above_blend * gm_above + (1.0 - above_blend) * gm_sub;
            return gm.abs().max(1e-12);
        }

        // Analytical formula for Level 1/3 (optimized path)
        let p = self.polarity();
        let vgs_eff = p * vgs;
        let vds_eff = smooth_positive(p * vds, VDS_SMOOTHING);
        let vth = self.vth(vbs);
        let vgt_raw = vgs_eff - vth;
        let vgt = smooth_positive(vgt_raw, SMOOTH_VOLTAGE);
        let vdsat = smooth_min(vgt, vds_eff, SMOOTH_VOLTAGE);
        let dvgt_dvgs = smooth_step(vgt_raw, SMOOTH_VOLTAGE);
        let sat_blend = smooth_step(vgt - vds_eff, SMOOTH_VOLTAGE);
        let dvdsat_dvgs = sat_blend * dvgt_dvgs;
        let gm_core = self.beta() * (vdsat * dvgt_dvgs + (vgt - vdsat) * dvdsat_dvgs);
        (gm_core * (1.0 + self.lambda * vds_eff)).max(1e-12)
    }

    fn gds_forward(&self, vgs: Value, vds: Value, vbs: Value) -> Value {
        let p = if self.level == 6 {
            1.0
        } else {
            self.polarity()
        };
        let vgs_eff = p * vgs;
        let vds_eff = smooth_positive(p * vds, VDS_SMOOTHING);
        let vth = self.vth(vbs);
        let vgt_raw = vgs_eff - vth;
        let vgt = smooth_positive(vgt_raw, SMOOTH_VOLTAGE);

        // Level 6: analytical gds from saturation factor and CLM derivatives
        if self.level == 6 {
            let (id, _) = self.calculate_id_forward(vgs, vds, vbs);

            // gds has two components:
            // 1. From saturation factor: d/dVds[(1-exp(-KV*Vds))^NV]
            // 2. From CLM: d/dVds[1 + LAMBDA0*Vds + LAMBDA1*Vds^2]

            // Saturation factor contribution
            let exp_term = (-self.kv * vds_eff).exp();
            let sat_factor = (1.0 - exp_term).max(1e-12);
            let dsat_dvds = self.nv * sat_factor.powf(self.nv - 1.0) * self.kv * exp_term;

            // CLM contribution
            let clm = 1.0 + self.lambda0 * vds_eff + self.lambda1 * vds_eff * vds_eff;
            let dclm_dvds = self.lambda0 + 2.0 * self.lambda1 * vds_eff;

            // Using product rule: gds = Id * (dsat/sat + dclm/clm)
            let gds = id.abs()
                * (dsat_dvds / sat_factor.powf(self.nv).max(1e-12) + dclm_dvds / clm.max(1e-12));
            return gds.abs().max(1e-12);
        }

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
        let p = if self.level == 6 {
            1.0
        } else {
            self.polarity()
        };
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

        let (vgs, vds, vbs) = self.branch_voltages(voltages);
        self.vgs = vgs;
        self.vds = vds;
        self.vbs = vbs;

        let (id, region, gm, gds, gmb, id_eq) =
            self.linearized_operating_point(self.vgs, self.vds, self.vbs);
        self.id = id;
        self.region = region;
        self.gm = gm;
        self.gds = gds;
        self.gmb = gmb;
        self.id_eq = id_eq;
    }

    fn stamp_nonlinear(
        &self,
        voltages: &[Value],
        matrix: &mut impl MatrixStamper,
        _rhs: &mut [Value],
    ) {
        let (vgs, vds, vbs) = self.branch_voltages(voltages);
        let (gm, gds, gmb, id_eq) = if self.cached_linearization_matches(vgs, vds, vbs) {
            (self.gm, self.gds, self.gmb, self.id_eq)
        } else {
            let (_, _, gm, gds, gmb, id_eq) = self.linearized_operating_point(vgs, vds, vbs);
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
    }

    fn is_converged(&self, tolerance: Value) -> bool {
        const RELTOL: Value = 1e-3;

        if !self.vgs.is_finite()
            || !self.vgs_prev.is_finite()
            || !self.vds.is_finite()
            || !self.vds_prev.is_finite()
            || !self.vbs.is_finite()
            || !self.vbs_prev.is_finite()
        {
            return false;
        }

        let vgs_diff = (self.vgs - self.vgs_prev).abs();
        let vds_diff = (self.vds - self.vds_prev).abs();
        let vbs_diff = (self.vbs - self.vbs_prev).abs();

        let vgs_tol = RELTOL * self.vgs.abs().max(self.vgs_prev.abs()) + tolerance;
        let vds_tol = RELTOL * self.vds.abs().max(self.vds_prev.abs()) + tolerance;
        let vbs_tol = RELTOL * self.vbs.abs().max(self.vbs_prev.abs()) + tolerance;

        vgs_diff < vgs_tol && vds_diff < vds_tol && vbs_diff < vbs_tol
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
    fn test_is_converged_accepts_small_relative_branch_deltas() {
        let mut m = Mosfet::new_nmos("M1".to_string(), 3, 2, 1, 0);
        m.vgs_prev = 1.0;
        m.vgs = 1.0008;
        m.vds_prev = 2.0;
        m.vds = 2.0008;
        m.vbs_prev = -0.4;
        m.vbs = -0.3996;

        assert!(
            m.is_converged(1e-6),
            "relative tolerance should allow sub-millivolt branch deltas around operating bias"
        );
    }

    #[test]
    fn test_is_converged_rejects_large_branch_delta() {
        let mut m = Mosfet::new_nmos("M1".to_string(), 3, 2, 1, 0);
        m.vgs_prev = 1.0;
        m.vgs = 1.01;
        m.vds_prev = 2.0;
        m.vds = 2.0;
        m.vbs_prev = 0.0;
        m.vbs = 0.0;

        assert!(
            !m.is_converged(1e-6),
            "large branch-voltage jump must fail convergence"
        );
    }

    #[test]
    fn test_is_converged_rejects_body_voltage_jump() {
        let mut m = Mosfet::new_nmos("M1".to_string(), 3, 2, 1, 0);
        m.vgs_prev = 1.2;
        m.vgs = 1.2001;
        m.vds_prev = 1.8;
        m.vds = 1.8001;
        m.vbs_prev = -0.2;
        m.vbs = -0.23;

        assert!(
            !m.is_converged(1e-6),
            "body-bias discontinuities must participate in convergence checks"
        );
    }

    #[test]
    fn test_is_converged_rejects_non_finite_history() {
        let mut m = Mosfet::new_nmos("M1".to_string(), 3, 2, 1, 0);
        m.vgs_prev = f64::NAN;
        m.vgs = 1.0;
        m.vds_prev = 2.0;
        m.vds = 2.0;
        m.vbs_prev = 0.0;
        m.vbs = 0.0;

        assert!(
            !m.is_converged(1e-6),
            "non-finite branch history must force another Newton update"
        );
    }

    #[test]
    fn test_update_caches_linearized_operating_point() {
        let mut m = Mosfet::new_nmos("M1".to_string(), 3, 2, 1, 0);
        let voltages = vec![0.15, 1.35, 0.95];

        m.update(&voltages);

        let (id, region, gm, gds, gmb, id_eq) =
            m.linearized_operating_point(m.vgs, m.vds, m.vbs);
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

        let mut matrix = CaptureMatrix::default();
        m.stamp_nonlinear(&live_voltages, &mut matrix, &mut []);

        assert!((matrix.g.get(&(3, 3)).copied().unwrap_or(0.0) - gds).abs() < 1e-18);
        assert!((matrix.g.get(&(3, 2)).copied().unwrap_or(0.0) - gm).abs() < 1e-18);
        assert!(
            (matrix.g.get(&(3, 1)).copied().unwrap_or(0.0) - (-gm - gds - gmb)).abs() < 1e-18
        );
        assert!((matrix.g.get(&(1, 3)).copied().unwrap_or(0.0) - (-gds)).abs() < 1e-18);
        assert!((matrix.rhs.get(&3).copied().unwrap_or(0.0) - (-id_eq)).abs() < 1e-18);
        assert!((matrix.rhs.get(&1).copied().unwrap_or(0.0) - id_eq).abs() < 1e-18);
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
    fn test_level6_pmos_legacy_signed_mode_gate_low_is_off() {
        // Legacy ngspice Level-6 PMOS convention in these decks:
        // Vg=0V, Vd=0V, Vs=5V, Vb=5V -> vgs=-5V keeps PMOS off.
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

        let (id, region) = m.calculate_id(-5.0, -5.0, 0.0);
        assert!(
            id.abs() < 1e-10,
            "Level-6 PMOS should be near cutoff for negative Vgs, got Id={id}"
        );
        assert_eq!(region, MosRegion::Cutoff);
    }

    #[test]
    fn test_level6_pmos_legacy_signed_mode_vgs_zero_is_finite() {
        // Legacy ngspice Level-6 PMOS with declared source at high rail:
        // Vg=Vs=5V, Vd=0V, Vb=5V -> vgs=0V, vds=-5V.
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

        let (id, _) = m.calculate_id(0.0, -5.0, 0.0);
        assert!(
            id.is_finite(),
            "Level-6 PMOS should remain finite at Vgs=0, got Id={id}"
        );
    }
}
