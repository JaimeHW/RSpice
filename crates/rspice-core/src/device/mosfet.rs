//! MOSFET (Metal-Oxide-Semiconductor Field-Effect Transistor) device model
//!
//! Implements a Level 1 SPICE MOSFET model (Shichman-Hodges).
//! Supports NMOS and PMOS devices in cutoff, linear, and saturation regions.

use crate::{circuit::NodeId, Value};
use crate::solver::{StaticMatrix, CscIndex};
use super::traits::{NonlinearDevice, MatrixStamper};

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
    pub fn new_nmos(name: String, drain: NodeId, gate: NodeId, source: NodeId, bulk: NodeId) -> Self {
        Self::new(name, MosType::Nmos, drain, gate, source, bulk)
    }

    /// Create a new PMOS with default parameters
    pub fn new_pmos(name: String, drain: NodeId, gate: NodeId, source: NodeId, bulk: NodeId) -> Self {
        Self::new(name, MosType::Pmos, drain, gate, source, bulk)
    }

    fn new(name: String, mos_type: MosType, drain: NodeId, gate: NodeId, source: NodeId, bulk: NodeId) -> Self {
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
            vto: 0.7,           // Threshold voltage
            kp: 110e-6,         // Transconductance (NMOS typical)
            gamma: 0.4,         // Body effect
            phi: 0.65,          // Surface potential
            lambda: 0.01,       // Channel-length modulation
            is_bulk: 1e-14,     // Bulk diode saturation current
            cox: 7e-4,          // Oxide capacitance
            
            // BSIM3-like parameters
            level: 1,           // Default to Level 1
            u0: 400.0,          // Low-field mobility (cm^2/V*s)
            ua: 2.25e-9,        // Mobility degradation coefficient
            ub: 5.87e-19,       // Second-order mobility coefficient
            vsat: 1.5e5,        // Saturation velocity (m/s)
            eta0: 0.08,         // DIBL coefficient
            etab: -0.07,        // DIBL body-bias coefficient
            nfactor: 1.0,       // Subthreshold swing
            pclm: 1.3,          // Channel length modulation
            rdsw: 200.0,        // S/D resistance (ohm*um)
            
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
        if let Some(&v) = params.get("VTO") { self.vto = v; }
        if let Some(&v) = params.get("KP") { self.kp = v; }
        if let Some(&v) = params.get("GAMMA") { self.gamma = v; }
        if let Some(&v) = params.get("PHI") { self.phi = v; }
        if let Some(&v) = params.get("LAMBDA") { self.lambda = v; }
        if let Some(&v) = params.get("L") { self.l = v; }
        if let Some(&v) = params.get("W") { self.w = v; }
        // BSIM3 parameters
        if let Some(&v) = params.get("U0") { self.u0 = v; }
        if let Some(&v) = params.get("UA") { self.ua = v; }
        if let Some(&v) = params.get("UB") { self.ub = v; }
        if let Some(&v) = params.get("VSAT") { self.vsat = v; }
        if let Some(&v) = params.get("ETA0") { self.eta0 = v; }
        if let Some(&v) = params.get("ETAB") { self.etab = v; }
        if let Some(&v) = params.get("NFACTOR") { self.nfactor = v; }
        if let Some(&v) = params.get("PCLM") { self.pclm = v; }
        if let Some(&v) = params.get("RDSW") { self.rdsw = v; }
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
        if d > 0 && d > 0 { self.indices.dd = matrix.get_index(d - 1, d - 1); }
        if d > 0 && g > 0 { self.indices.dg = matrix.get_index(d - 1, g - 1); }
        if d > 0 && s > 0 { self.indices.ds = matrix.get_index(d - 1, s - 1); }
        if d > 0 && b > 0 { self.indices.db = matrix.get_index(d - 1, b - 1); }
        // Source row (4 columns)
        if s > 0 && d > 0 { self.indices.sd = matrix.get_index(s - 1, d - 1); }
        if s > 0 && g > 0 { self.indices.sg = matrix.get_index(s - 1, g - 1); }
        if s > 0 && s > 0 { self.indices.ss = matrix.get_index(s - 1, s - 1); }
        if s > 0 && b > 0 { self.indices.sb = matrix.get_index(s - 1, b - 1); }
    }

    /// Stamp using O(1) direct indexing (call after link)
    pub fn stamp_direct(&self, matrix: &mut StaticMatrix, rhs: &mut [Value], voltages: &[Value]) {
        let vd = if self.node_drain == 0 { 0.0 } else { voltages[self.node_drain - 1] };
        let vg = if self.node_gate == 0 { 0.0 } else { voltages[self.node_gate - 1] };
        let vs = if self.node_source == 0 { 0.0 } else { voltages[self.node_source - 1] };
        let vb = if self.node_bulk == 0 { 0.0 } else { voltages[self.node_bulk - 1] };
        
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
        if let Some(idx) = self.indices.dd { matrix.stamp_direct(idx, gds); }
        if let Some(idx) = self.indices.dg { matrix.stamp_direct(idx, gm); }
        if let Some(idx) = self.indices.ds { matrix.stamp_direct(idx, -gm - gds - gmb); }
        if let Some(idx) = self.indices.db { matrix.stamp_direct(idx, gmb); }
        // Source row
        if let Some(idx) = self.indices.sd { matrix.stamp_direct(idx, -gds); }
        if let Some(idx) = self.indices.sg { matrix.stamp_direct(idx, -gm); }
        if let Some(idx) = self.indices.ss { matrix.stamp_direct(idx, gm + gds + gmb); }
        if let Some(idx) = self.indices.sb { matrix.stamp_direct(idx, -gmb); }
        
        // Stamp RHS
        if self.node_drain > 0 { rhs[self.node_drain - 1] -= id_eq; }
        if self.node_source > 0 { rhs[self.node_source - 1] += id_eq; }
    }

    /// Get polarity multiplier (+1 for NMOS, -1 for PMOS)
    fn polarity(&self) -> Value {
        match self.mos_type {
            MosType::Nmos => 1.0,
            MosType::Pmos => -1.0,
        }
    }

    /// Calculate effective threshold voltage with body effect
    fn vth(&self, vbs: Value) -> Value {
        let p = self.polarity();
        let vbs_eff = p * vbs;
        
        // Body effect: Vth = Vto + gamma * (sqrt(phi - Vbs) - sqrt(phi))
        let phi_vbs = (self.phi - vbs_eff).max(0.0);
        self.vto + self.gamma * (phi_vbs.sqrt() - self.phi.sqrt())
    }

    /// Calculate W/L ratio
    fn wl_ratio(&self) -> Value {
        self.w / self.l
    }

    /// Beta = KP * W/L
    fn beta(&self) -> Value {
        self.kp * self.wl_ratio()
    }

    /// Determine operating region and calculate drain current
    fn calculate_id(&self, vgs: Value, vds: Value, vbs: Value) -> (Value, MosRegion) {
        if self.level >= 3 {
            self.calculate_id_bsim3(vgs, vds, vbs)
        } else {
            self.calculate_id_level1(vgs, vds, vbs)
        }
    }

    /// Level 1 (Shichman-Hodges) drain current calculation
    fn calculate_id_level1(&self, vgs: Value, vds: Value, vbs: Value) -> (Value, MosRegion) {
        let p = self.polarity();
        let vgs_eff = p * vgs;
        let vds_eff = p * vds;
        let vth = self.vth(vbs);
        
        let vgt = vgs_eff - vth; // Gate overdrive
        
        if vgt <= 0.0 {
            // Cutoff region
            (0.0, MosRegion::Cutoff)
        } else if vds_eff < vgt {
            // Linear (triode) region
            // Id = beta * ((Vgs - Vth) * Vds - Vds^2/2) * (1 + lambda * Vds)
            let id = p * self.beta() * (vgt * vds_eff - 0.5 * vds_eff * vds_eff) 
                   * (1.0 + self.lambda * vds_eff);
            (id, MosRegion::Linear)
        } else {
            // Saturation region
            // Id = beta/2 * (Vgs - Vth)^2 * (1 + lambda * Vds)
            let id = p * 0.5 * self.beta() * vgt * vgt 
                   * (1.0 + self.lambda * vds_eff);
            (id, MosRegion::Saturation)
        }
    }

    /// BSIM3-like drain current with short-channel effects
    /// Includes:
    /// - Mobility degradation due to vertical electric field
    /// - Velocity saturation
    /// - Drain-Induced Barrier Lowering (DIBL)
    /// - Channel length modulation
    fn calculate_id_bsim3(&self, vgs: Value, vds: Value, vbs: Value) -> (Value, MosRegion) {
        let p = self.polarity();
        let vgs_eff = p * vgs;
        let vds_eff = (p * vds).max(1e-12);  // Avoid division by zero
        let vbs_eff = p * vbs;
        
        // DIBL: threshold voltage reduction with drain bias
        let vth_dibl = self.vth(vbs) - self.eta0 * vds_eff - self.etab * vbs_eff * vds_eff;
        let vth = vth_dibl.max(0.1);  // Minimum threshold
        
        let vgt = vgs_eff - vth;  // Gate overdrive
        
        if vgt <= 0.0 {
            // Subthreshold region (simplified exponential)
            // Subthreshold current for proper convergence
            let vt = 0.0259;  // Thermal voltage at 300K
            let n = self.nfactor;
            let i_sub = 1e-12 * (vgt / (n * vt)).exp().min(1e6);
            return (p * i_sub, MosRegion::Cutoff);
        }
        
        // Mobility degradation (vertical field effect)
        // µeff = µ0 / (1 + Ua*(Vgs-Vth)/tox + Ub*((Vgs-Vth)/tox)^2)
        let eeff = vgt / (6e-9);  // Assume tox = 6nm
        let mobility = self.u0 / (1.0 + self.ua * eeff + self.ub * eeff * eeff);
        
        // Effective beta with mobility degradation
        let beta_eff = mobility * 1e-4 * self.cox * self.wl_ratio();  // Convert cm^2 to m^2
        
        // Saturation voltage with velocity saturation
        // Vdsat = (Vgs - Vth) / (1 + (Vgs - Vth) / (L * Vsat / µ))
        let vsat_over_l = self.vsat / self.l;
        let mu_m2 = mobility * 1e-4;  // Convert to m^2/V*s
        let vdsat = vgt / (1.0 + vgt / (self.l * vsat_over_l / mu_m2).max(1e-6));
        
        if vds_eff < vdsat {
            // Linear region
            let id = p * beta_eff * (vgt * vds_eff - 0.5 * vds_eff * vds_eff);
            (id, MosRegion::Linear)
        } else {
            // Saturation region with CLM
            let vds_over_vdsat = vds_eff / vdsat.max(1e-6);
            let clm = 1.0 + self.pclm * (vds_over_vdsat - 1.0).max(0.0).ln_1p();
            let id = p * 0.5 * beta_eff * vgt * vgt * clm;
            (id, MosRegion::Saturation)
        }
    }

    /// Calculate transconductance gm = dId/dVgs
    fn gm(&self, vgs: Value, vds: Value, vbs: Value) -> Value {
        let p = self.polarity();
        let vgs_eff = p * vgs;
        let vds_eff = p * vds;
        let vth = self.vth(vbs);
        let vgt = vgs_eff - vth;
        
        if vgt <= 0.0 {
            1e-12 // Minimum conductance in cutoff
        } else if vds_eff < vgt {
            // Linear region: gm = beta * Vds
            self.beta() * vds_eff * (1.0 + self.lambda * vds_eff)
        } else {
            // Saturation region: gm = beta * (Vgs - Vth)
            self.beta() * vgt * (1.0 + self.lambda * vds_eff)
        }
    }

    /// Calculate output conductance gds = dId/dVds
    fn gds(&self, vgs: Value, vds: Value, vbs: Value) -> Value {
        let p = self.polarity();
        let vgs_eff = p * vgs;
        let vds_eff = p * vds;
        let vth = self.vth(vbs);
        let vgt = vgs_eff - vth;
        
        if vgt <= 0.0 {
            1e-12 // Minimum conductance in cutoff
        } else if vds_eff < vgt {
            // Linear region: gds = beta * (Vgt - Vds)
            self.beta() * (vgt - vds_eff) * (1.0 + self.lambda * vds_eff)
                + self.beta() * (vgt * vds_eff - 0.5 * vds_eff * vds_eff) * self.lambda
        } else {
            // Saturation region: gds = lambda * Id
            0.5 * self.beta() * vgt * vgt * self.lambda
        }
    }

    /// Calculate body transconductance gmb = dId/dVbs
    fn gmb(&self, vgs: Value, vds: Value, vbs: Value) -> Value {
        let p = self.polarity();
        let vbs_eff = p * vbs;
        
        // gmb = -gm * (gamma / (2 * sqrt(phi - Vbs)))
        let gm = self.gm(vgs, vds, vbs);
        let phi_vbs = (self.phi - vbs_eff).max(1e-6);
        gm * self.gamma / (2.0 * phi_vbs.sqrt())
    }
}

impl NonlinearDevice for Mosfet {
    fn update(&mut self, voltages: &[Value]) {
        let vd = if self.node_drain == 0 { 0.0 } else { voltages[self.node_drain - 1] };
        let vg = if self.node_gate == 0 { 0.0 } else { voltages[self.node_gate - 1] };
        let vs = if self.node_source == 0 { 0.0 } else { voltages[self.node_source - 1] };
        let vb = if self.node_bulk == 0 { 0.0 } else { voltages[self.node_bulk - 1] };
        
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
        let vd = if self.node_drain == 0 { 0.0 } else { voltages[self.node_drain - 1] };
        let vg = if self.node_gate == 0 { 0.0 } else { voltages[self.node_gate - 1] };
        let vs = if self.node_source == 0 { 0.0 } else { voltages[self.node_source - 1] };
        let vb = if self.node_bulk == 0 { 0.0 } else { voltages[self.node_bulk - 1] };
        
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
        
        // Vgs < Vth -> cutoff
        let (id, region) = m.calculate_id(0.3, 5.0, 0.0);
        assert_eq!(region, MosRegion::Cutoff);
        assert_eq!(id, 0.0);
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
        let m_bsim3 = Mosfet::new_nmos("M2".to_string(), 3, 2, 1, 0)
            .with_level(3);
        
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
            .with_geometry(10e-6, 0.1e-6);  // Short channel
        
        // High drive should show velocity saturation effects
        let (id, region) = m.calculate_id(3.0, 3.0, 0.0);
        assert_eq!(region, MosRegion::Saturation);
        assert!(id > 0.0);
    }
}
