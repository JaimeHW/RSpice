//! MOSFET (Metal-Oxide-Semiconductor Field-Effect Transistor) device model
//!
//! Implements a Level 1 SPICE MOSFET model (Shichman-Hodges).
//! Supports NMOS and PMOS devices in cutoff, linear, and saturation regions.

use super::smooth::{SMOOTH_VOLTAGE, smooth_max, smooth_min, smooth_positive, smooth_step};
use crate::device::traits::{MatrixStamper, NonlinearDevice};
use crate::solver::{CscIndex, StaticMatrix};
use crate::{Value, circuit::NodeId};

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
    region: MosRegion,

    // Previous iteration values
    vgs_prev: Value,
    vds_prev: Value,

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
            region: MosRegion::Cutoff,

            vgs_prev: 0.0,
            vds_prev: 0.0,
            indices: MosfetIndices::default(),
        }
    }

    /// Set device geometry
    pub fn with_geometry(mut self, w: Value, l: Value) -> Self {
        self.w = w;
        self.l = l;
        self
    }

    /// Set model parameters from a DeviceModel
    pub fn with_params(mut self, params: &std::collections::HashMap<String, Value>) -> Self {
        // Level 1 parameters
        if let Some(&v) = params.get("VTO") {
            self.vto = v;
        }
        if let Some(&v) = params.get("KP") {
            self.kp = v;
        }
        if let Some(&v) = params.get("GAMMA") {
            self.gamma = v;
        }
        if let Some(&v) = params.get("PHI") {
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
        // BSIM3 parameters
        if let Some(&v) = params.get("U0") {
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

    /// Link this device to a StaticMatrix for O(1) stamping
    pub fn link(&mut self, matrix: &StaticMatrix) {
        let d = self.node_drain;
        let g = self.node_gate;
        let s = self.node_source;
        let b = self.node_bulk;

        // Drain row (4 columns)
        if d > 0 && d > 0 {
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
        if s > 0 && s > 0 {
            self.indices.ss = matrix.get_index(s - 1, s - 1);
        }
        if s > 0 && b > 0 {
            self.indices.sb = matrix.get_index(s - 1, b - 1);
        }
    }

    /// Stamp using O(1) direct indexing (call after link)
    pub fn stamp_direct(&self, matrix: &mut StaticMatrix, rhs: &mut [Value], voltages: &[Value]) {
        let vd = if self.node_drain == 0 {
            0.0
        } else {
            voltages[self.node_drain - 1]
        };
        let vg = if self.node_gate == 0 {
            0.0
        } else {
            voltages[self.node_gate - 1]
        };
        let vs = if self.node_source == 0 {
            0.0
        } else {
            voltages[self.node_source - 1]
        };
        let vb = if self.node_bulk == 0 {
            0.0
        } else {
            voltages[self.node_bulk - 1]
        };

        let vgs = vg - vs;
        let vds = vd - vs;
        let vbs = vb - vs;

        // Conductances
        let gm = self.gm(vgs, vds, vbs);
        let gds = self.gds(vgs, vds, vbs);
        let gmb = self.gmb(vgs, vds, vbs);

        // Drain current and equivalent current source
        let (id, _) = self.calculate_id(vgs, vds, vbs);
        let id_eq = id - gm * vgs - gds * vds - gmb * vbs;

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

        // Base body effect: Vth = Vto + gamma * (sqrt(phi - Vbs) - sqrt(phi))
        let phi_vbs = (self.phi - vbs_eff).max(0.0);
        let vth_base = self.vto + self.gamma * (phi_vbs.sqrt() - self.phi.sqrt());

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
        let vth_k1k2 = self.vto + self.k1 * phi_vbs.sqrt() + self.k2 * (self.phi - vbs_eff);

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

    /// Determine operating region and calculate drain current
    fn calculate_id(&self, vgs: Value, vds: Value, vbs: Value) -> (Value, MosRegion) {
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
        let vds_eff = smooth_positive(p * vds, SMOOTH_VOLTAGE * 0.1); // Ensure positive Vds
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
    /// BSIM3-like drain current with C1 continuous transitions
    fn calculate_id_bsim3(&self, vgs: Value, vds: Value, vbs: Value) -> (Value, MosRegion) {
        let p = self.polarity();
        let vgs_eff = p * vgs;
        let vds_eff = smooth_positive(p * vds, SMOOTH_VOLTAGE * 0.1);
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
        let p = self.polarity();
        let vgs_eff = p * vgs;
        let vds_eff = smooth_positive(p * vds, SMOOTH_VOLTAGE * 0.1);
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
        let current_term = self.kc * wl * vgt.powf(self.nc);

        // Saturation term: (1 - exp(-KV * Vds))^NV
        // This smoothly transitions from linear to saturation
        let exp_term = (-self.kv * vds_eff).exp();
        let sat_factor = (1.0 - exp_term).max(0.0).powf(self.nv);

        // Channel length modulation: 1 + LAMBDA0 * Vds + LAMBDA1 * Vds^2
        let clm = 1.0 + self.lambda0 * vds_eff + self.lambda1 * vds_eff * vds_eff;

        // Total drain current
        let id = p * current_term * sat_factor * clm;

        (id, region)
    }

    /// Calculate transconductance gm = dId/dVgs
    ///
    /// Uses analytical formulas for all model levels for performance
    fn gm(&self, vgs: Value, vds: Value, vbs: Value) -> Value {
        let p = self.polarity();
        let vgs_eff = p * vgs;
        let _vds_eff = smooth_positive(p * vds, SMOOTH_VOLTAGE * 0.1);
        let vth = self.vth(vbs);
        let vgt_raw = vgs_eff - vth;
        let vgt = smooth_positive(vgt_raw, SMOOTH_VOLTAGE);
        let dvgt_dvgs = smooth_step(vgt_raw, SMOOTH_VOLTAGE);

        // Level 6: gm = dId/dVgs = Id / Vgt * NC * dVgt/dVgs (when Vgt > 0)
        if self.level == 6 {
            let (id, _) = self.calculate_id(vgs, vds, vbs);
            let gm = if vgt > 1e-12 {
                id.abs() / vgt * self.nc * dvgt_dvgs
            } else {
                1e-12
            };
            return gm.abs().max(1e-12);
        }

        // Analytical formula for Level 1/3 (optimized path)
        let p = self.polarity();
        let vgs_eff = p * vgs;
        let vds_eff = smooth_positive(p * vds, SMOOTH_VOLTAGE * 0.1);
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

    /// Calculate output conductance gds = dId/dVds
    ///
    /// Uses analytical formulas for all model levels for performance
    fn gds(&self, vgs: Value, vds: Value, vbs: Value) -> Value {
        let p = self.polarity();
        let vgs_eff = p * vgs;
        let vds_eff = smooth_positive(p * vds, SMOOTH_VOLTAGE * 0.1);
        let vth = self.vth(vbs);
        let vgt_raw = vgs_eff - vth;
        let vgt = smooth_positive(vgt_raw, SMOOTH_VOLTAGE);

        // Level 6: analytical gds from saturation factor and CLM derivatives
        if self.level == 6 {
            let (id, _) = self.calculate_id(vgs, vds, vbs);

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

    /// Calculate body transconductance gmb = dId/dVbs with C1 continuity
    fn gmb(&self, vgs: Value, vds: Value, vbs: Value) -> Value {
        let p = self.polarity();
        let vbs_eff = p * vbs;

        // gmb = -gm * (gamma / (2 * sqrt(phi - Vbs)))
        // The gm function is already smooth, so gmb inherits smoothness
        let gm = self.gm(vgs, vds, vbs);

        // Smooth the phi - Vbs term to avoid singularity
        let phi_vbs = smooth_max(self.phi - vbs_eff, SMOOTH_VOLTAGE, SMOOTH_VOLTAGE);

        gm * self.gamma / (2.0 * phi_vbs.sqrt())
    }
}

impl NonlinearDevice for Mosfet {
    fn update(&mut self, voltages: &[Value]) {
        let vd = if self.node_drain == 0 {
            0.0
        } else {
            voltages[self.node_drain - 1]
        };
        let vg = if self.node_gate == 0 {
            0.0
        } else {
            voltages[self.node_gate - 1]
        };
        let vs = if self.node_source == 0 {
            0.0
        } else {
            voltages[self.node_source - 1]
        };
        let vb = if self.node_bulk == 0 {
            0.0
        } else {
            voltages[self.node_bulk - 1]
        };

        self.vgs_prev = self.vgs;
        self.vds_prev = self.vds;

        self.vgs = vg - vs;
        self.vds = vd - vs;
        self.vbs = vb - vs;

        let (id, region) = self.calculate_id(self.vgs, self.vds, self.vbs);
        self.id = id;
        self.region = region;
    }

    fn stamp_nonlinear(
        &self,
        voltages: &[Value],
        matrix: &mut impl MatrixStamper,
        _rhs: &mut [Value],
    ) {
        let vd = if self.node_drain == 0 {
            0.0
        } else {
            voltages[self.node_drain - 1]
        };
        let vg = if self.node_gate == 0 {
            0.0
        } else {
            voltages[self.node_gate - 1]
        };
        let vs = if self.node_source == 0 {
            0.0
        } else {
            voltages[self.node_source - 1]
        };
        let vb = if self.node_bulk == 0 {
            0.0
        } else {
            voltages[self.node_bulk - 1]
        };

        let vgs = vg - vs;
        let vds = vd - vs;
        let vbs = vb - vs;

        // Conductances
        let gm = self.gm(vgs, vds, vbs);
        let gds = self.gds(vgs, vds, vbs);
        let gmb = self.gmb(vgs, vds, vbs);

        // Drain current and equivalent current source
        let (id, _) = self.calculate_id(vgs, vds, vbs);
        let id_eq = id - gm * vgs - gds * vds - gmb * vbs;

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
        let vgs_diff = (self.vgs - self.vgs_prev).abs();
        let vds_diff = (self.vds - self.vds_prev).abs();
        vgs_diff < tolerance && vds_diff < tolerance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

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
            let gm = m.gm(vgs, vds, vbs);

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

            let gds = m.gds(vgs, vds, vbs);

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
}
