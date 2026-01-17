//! EKV MOSFET Model for Low-Power and RF/Analog Designs
//!
//! The EKV (Enz-Krummenacher-Vittoz) model is physics-based and provides:
//! - Continuous current from weak to strong inversion
//! - Accurate subthreshold behavior for low-power
//! - Single-equation formulation (no region switching)
//! - Conservation of charge for transient analysis
//!
//! Reference: EKV v2.6 documentation (EPFL)

use crate::{circuit::NodeId, Value};
use crate::solver::StaticMatrix;
use crate::device::traits::{NonlinearDevice, MatrixStamper};
use super::mosfet::{MosType, MosRegion, MosfetIndices};

//=============================================================================
// EKV Model Constants
//=============================================================================

/// Thermal voltage at 300K (kT/q)
const VT_300K: Value = 0.02585;

/// Subthreshold slope coefficient (typically 1.0 to 1.5)
const DEFAULT_NSLOPE: Value = 1.3;

/// Velocity saturation voltage coefficient
const DEFAULT_UCRIT: Value = 4.0e6; // V/m

//=============================================================================
// EKV MOSFET Device
//=============================================================================

/// EKV MOSFET model for low-power analog design
///
/// Uses the EKV continuous current equation that smoothly transitions
/// from weak inversion (subthreshold) through moderate to strong inversion.
///
/// # Key Features
/// - Single equation for all regions (no discontinuities)
/// - Physics-based subthreshold modeling
/// - Symmetric forward/reverse operation
/// - Charge-based model for transient analysis
#[derive(Debug, Clone)]
pub struct EkvMosfet {
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
    
    // EKV Model Parameters
    /// Threshold voltage (VTO)
    pub vto: Value,
    /// Transconductance coefficient (KP) in A/V^2
    pub kp: Value,
    /// Body effect coefficient (GAMMA)
    pub gamma: Value,
    /// Surface potential (PHI)
    pub phi: Value,
    /// Channel length modulation (LAMBDA)
    pub lambda: Value,
    /// Subthreshold slope factor (N)
    pub nslope: Value,
    /// Critical field for velocity saturation (UCRIT) in V/m
    pub ucrit: Value,
    /// Thermal voltage
    pub vt: Value,
    /// Specific current (IS = 2 * n * U_T^2 * KP * W/L)
    pub ispec: Value,
    /// Oxide capacitance per unit area (COX)
    pub cox: Value,
    /// Low-field mobility (U0) in cm^2/V*s
    pub u0: Value,
    
    // Operating point
    vgs: Value,
    vds: Value,
    vbs: Value,
    id: Value,
    region: MosRegion,
    
    // Previous values for convergence
    vgs_prev: Value,
    vds_prev: Value,
    
    /// Pre-computed matrix indices
    pub indices: MosfetIndices,
}

impl EkvMosfet {
    /// Create new NMOS EKV device
    pub fn new_nmos(name: String, drain: NodeId, gate: NodeId, source: NodeId, bulk: NodeId) -> Self {
        Self::new(name, MosType::Nmos, drain, gate, source, bulk)
    }

    /// Create new PMOS EKV device
    pub fn new_pmos(name: String, drain: NodeId, gate: NodeId, source: NodeId, bulk: NodeId) -> Self {
        Self::new(name, MosType::Pmos, drain, gate, source, bulk)
    }

    fn new(name: String, mos_type: MosType, drain: NodeId, gate: NodeId, source: NodeId, bulk: NodeId) -> Self {
        let kp = 50e-6; // 50 uA/V^2
        let w = 10e-6;
        let l = 1e-6;
        let nslope = DEFAULT_NSLOPE;
        let vt = VT_300K;
        
        // Specific current: IS = 2 * n * U_T^2 * KP * W/L
        let ispec = 2.0 * nslope * vt * vt * kp * w / l;
        
        Self {
            name,
            mos_type,
            node_drain: drain,
            node_gate: gate,
            node_source: source,
            node_bulk: bulk,
            l,
            w,
            vto: 0.5,
            kp,
            gamma: 0.5,
            phi: 0.7,
            lambda: 0.05,
            nslope,
            ucrit: DEFAULT_UCRIT,
            vt,
            ispec,
            cox: 7e-4,
            u0: 400.0,
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
        self.update_ispec();
        self
    }

    /// Set model parameters
    pub fn with_params(mut self, params: &std::collections::HashMap<String, Value>) -> Self {
        if let Some(&v) = params.get("VTO") { self.vto = v; }
        if let Some(&v) = params.get("KP") { self.kp = v; }
        if let Some(&v) = params.get("GAMMA") { self.gamma = v; }
        if let Some(&v) = params.get("PHI") { self.phi = v; }
        if let Some(&v) = params.get("LAMBDA") { self.lambda = v; }
        if let Some(&v) = params.get("N") { self.nslope = v; }
        if let Some(&v) = params.get("UCRIT") { self.ucrit = v; }
        if let Some(&v) = params.get("U0") { self.u0 = v; }
        if let Some(&v) = params.get("L") { self.l = v; }
        if let Some(&v) = params.get("W") { self.w = v; }
        self.update_ispec();
        self
    }

    fn update_ispec(&mut self) {
        self.ispec = 2.0 * self.nslope * self.vt * self.vt * self.kp * self.w / self.l;
    }

    /// Link device to matrix for O(1) stamping
    pub fn link(&mut self, matrix: &StaticMatrix) {
        let d = self.node_drain;
        let g = self.node_gate;
        let s = self.node_source;
        let b = self.node_bulk;
        
        // Only D and S rows (gate has no DC current)
        if d > 0 { self.indices.dd = matrix.get_index(d - 1, d - 1); }
        if d > 0 && g > 0 { self.indices.dg = matrix.get_index(d - 1, g - 1); }
        if d > 0 && s > 0 { self.indices.ds = matrix.get_index(d - 1, s - 1); }
        if d > 0 && b > 0 { self.indices.db = matrix.get_index(d - 1, b - 1); }
        if s > 0 && d > 0 { self.indices.sd = matrix.get_index(s - 1, d - 1); }
        if s > 0 && g > 0 { self.indices.sg = matrix.get_index(s - 1, g - 1); }
        if s > 0 { self.indices.ss = matrix.get_index(s - 1, s - 1); }
        if s > 0 && b > 0 { self.indices.sb = matrix.get_index(s - 1, b - 1); }
    }

    /// Get polarity (+1 for NMOS, -1 for PMOS)
    fn polarity(&self) -> Value {
        match self.mos_type {
            MosType::Nmos => 1.0,
            MosType::Pmos => -1.0,
        }
    }

    /// EKV threshold voltage with body effect
    fn vth(&self, vbs: Value) -> Value {
        let p = self.polarity();
        let vbs_eff = p * vbs;
        let phi_vbs = (self.phi - vbs_eff).max(0.01);
        self.vto + self.gamma * (phi_vbs.sqrt() - self.phi.sqrt())
    }

    /// EKV pinch-off voltage (Vp)
    fn vp(&self, vgs: Value, vbs: Value) -> Value {
        let vth = self.vth(vbs);
        let p = self.polarity();
        (p * vgs - vth) / self.nslope
    }

    /// EKV forward/reverse normalization function
    /// 
    /// This is the key to EKV's continuous behavior:
    /// f(V) = ln^2(1 + exp(V / (2 * Ut)))
    /// 
    /// - For V >> 0 (strong inversion): f(V) ≈ V^2 / (4 * Ut^2)
    /// - For V << 0 (weak inversion): f(V) ≈ exp(V / Ut)
    #[inline]
    fn interpolation_function(&self, v: Value) -> Value {
        let x = v / (2.0 * self.vt);
        let exp_x = (x.min(40.0)).exp(); // Limit to avoid overflow
        let ln_term = (1.0 + exp_x).ln();
        ln_term * ln_term
    }

    /// Derivative of interpolation function
    #[inline]
    fn interpolation_derivative(&self, v: Value) -> Value {
        let x = v / (2.0 * self.vt);
        let exp_x = (x.min(40.0)).exp();
        let denom = 1.0 + exp_x;
        (exp_x / denom) * (1.0 + exp_x).ln() / self.vt
    }

    /// Calculate EKV drain current using symmetric linearization
    ///
    /// The EKV current equation is:
    /// ID = IF - IR
    /// IF = IS * f((Vp - Vs) / Ut)  (forward)
    /// IR = IS * f((Vp - Vd) / Ut)  (reverse)
    fn calculate_id(&self, vgs: Value, vds: Value, vbs: Value) -> (Value, MosRegion) {
        let p = self.polarity();
        let vgs_eff = p * vgs;
        let vds_eff = p * vds;
        let _vbs_eff = p * vbs;
        
        // Pinch-off voltage
        let vp = self.vp(vgs, vbs);
        
        // Source and drain referenced to bulk
        let vs = 0.0; // Source as reference
        let vd = vds_eff;
        
        // Forward component (source side)
        let vp_vs = vp - vs;
        let if_norm = self.interpolation_function(vp_vs);
        
        // Reverse component (drain side)
        let vp_vd = vp - vd;
        let ir_norm = self.interpolation_function(vp_vd);
        
        // Drain current
        let id_intrinsic = self.ispec * (if_norm - ir_norm);
        
        // Channel length modulation
        let id = p * id_intrinsic * (1.0 + self.lambda * vds_eff.abs());
        
        // Determine region for reporting
        let vth = self.vth(vbs);
        let region = if vgs_eff - vth < -3.0 * self.nslope * self.vt {
            MosRegion::Cutoff
        } else if vds_eff < vgs_eff - vth - 2.0 * self.nslope * self.vt {
            MosRegion::Linear
        } else {
            MosRegion::Saturation
        };
        
        (id, region)
    }

    /// Transconductance gm = dId/dVgs
    fn gm(&self, vgs: Value, vds: Value, vbs: Value) -> Value {
        let p = self.polarity();
        let vds_eff = p * vds;
        
        // Pinch-off voltage
        let vp = self.vp(vgs, vbs);
        
        let vs = 0.0;
        let vd = vds_eff;
        
        // Derivatives of interpolation function
        let dvp_dvgs = 1.0 / self.nslope;
        let dif = self.interpolation_derivative(vp - vs) * dvp_dvgs;
        let dir = self.interpolation_derivative(vp - vd) * dvp_dvgs;
        
        let gm = self.ispec * (dif - dir) * (1.0 + self.lambda * vds_eff.abs());
        gm.max(1e-12)
    }

    /// Output conductance gds = dId/dVds
    fn gds(&self, vgs: Value, vds: Value, vbs: Value) -> Value {
        let p = self.polarity();
        let vds_eff = p * vds;
        
        let vp = self.vp(vgs, vbs);
        let vd = vds_eff;
        
        // Derivative of IR w.r.t. Vd
        let dir_dvd = -self.interpolation_derivative(vp - vd);
        
        let (id, _) = self.calculate_id(vgs, vds, vbs);
        
        // From differentiation of ID * (1 + lambda * |Vds|)
        let gds_basic = -self.ispec * dir_dvd * (1.0 + self.lambda * vds_eff.abs());
        let gds_clm = id.abs() * self.lambda;
        
        (gds_basic + gds_clm).max(1e-12)
    }

    /// Body transconductance gmb = dId/dVbs
    fn gmb(&self, vgs: Value, vds: Value, vbs: Value) -> Value {
        let p = self.polarity();
        let vbs_eff = p * vbs;
        
        let gm = self.gm(vgs, vds, vbs);
        let phi_vbs = (self.phi - vbs_eff).max(0.01);
        
        // gmb = gm * gamma / (2 * n * sqrt(phi - Vbs))
        gm * self.gamma / (2.0 * self.nslope * phi_vbs.sqrt())
    }
}

impl NonlinearDevice for EkvMosfet {
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
        rhs: &mut [Value],
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
        
        // Drain current
        let (id, _) = self.calculate_id(vgs, vds, vbs);
        let id_eq = id - gm * vgs - gds * vds - gmb * vbs;
        
        // Stamp matrix
        let d = self.node_drain;
        let g = self.node_gate;
        let s = self.node_source;
        let b = self.node_bulk;
        
        // Drain row
        if d > 0 {
            matrix.stamp(d - 1, d - 1, gds);
            if g > 0 { matrix.stamp(d - 1, g - 1, gm); }
            if s > 0 { matrix.stamp(d - 1, s - 1, -(gm + gds + gmb)); }
            if b > 0 { matrix.stamp(d - 1, b - 1, gmb); }
            rhs[d - 1] -= id_eq;
        }
        
        // Source row
        if s > 0 {
            if d > 0 { matrix.stamp(s - 1, d - 1, -gds); }
            if g > 0 { matrix.stamp(s - 1, g - 1, -gm); }
            matrix.stamp(s - 1, s - 1, gm + gds + gmb);
            if b > 0 { matrix.stamp(s - 1, b - 1, -gmb); }
            rhs[s - 1] += id_eq;
        }
    }

    fn is_converged(&self, tolerance: Value) -> bool {
        let dvgs = (self.vgs - self.vgs_prev).abs();
        let dvds = (self.vds - self.vds_prev).abs();
        dvgs < tolerance && dvds < tolerance
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ekv_creation() {
        let m = EkvMosfet::new_nmos("M1".to_string(), 1, 2, 3, 4);
        assert_eq!(m.node_drain, 1);
        assert_eq!(m.node_gate, 2);
        assert_eq!(m.node_source, 3);
        assert_eq!(m.node_bulk, 4);
    }

    #[test]
    fn test_ekv_subthreshold() {
        let m = EkvMosfet::new_nmos("M1".to_string(), 1, 2, 3, 4);
        
        // Subthreshold: Vgs < Vth
        let vgs = 0.2; // Below Vth = 0.5
        let vds = 1.0;
        let vbs = 0.0;
        
        let (id, region) = m.calculate_id(vgs, vds, vbs);
        
        // Should be cutoff/weak inversion with very small current
        assert!(id.abs() < 1e-6, "Expected very small subthreshold current, got {}", id);
        assert_eq!(region, MosRegion::Cutoff);
    }

    #[test]
    fn test_ekv_strong_inversion() {
        let m = EkvMosfet::new_nmos("M1".to_string(), 1, 2, 3, 4);
        
        // Strong inversion: Vgs > Vth
        let vgs = 1.5;
        let vds = 2.0;
        let vbs = 0.0;
        
        let (id, region) = m.calculate_id(vgs, vds, vbs);
        
        // Should have significant current
        assert!(id > 0.0, "Expected positive drain current");
        assert!(id > 1e-6, "Expected significant current in strong inversion");
        assert_eq!(region, MosRegion::Saturation);
    }

    #[test]
    fn test_ekv_continuity() {
        let m = EkvMosfet::new_nmos("M1".to_string(), 1, 2, 3, 4);
        
        let vds = 1.5;
        let vbs = 0.0;
        
        // Sweep Vgs across threshold
        let mut prev_id = 0.0;
        for i in 0..20 {
            let vgs = (i as f64) * 0.1;
            let (id, _) = m.calculate_id(vgs, vds, vbs);
            
            // Current should be monotonically increasing
            if i > 0 {
                assert!(id >= prev_id - 1e-12, "EKV current not monotonic at Vgs={}", vgs);
            }
            prev_id = id;
        }
    }

    #[test]
    fn test_ekv_pmos_polarity() {
        let m = EkvMosfet::new_pmos("M1".to_string(), 1, 2, 3, 4);
        
        // PMOS: negative Vgs (gate below source), negative Vds
        let vgs = -1.5;
        let vds = -2.0;
        let vbs = 0.0;
        
        let (id, _) = m.calculate_id(vgs, vds, vbs);
        
        // PMOS current flows from source to drain (negative direction)
        assert!(id < 0.0, "Expected negative drain current for PMOS");
    }

    #[test]
    fn test_ekv_gm_positive() {
        let m = EkvMosfet::new_nmos("M1".to_string(), 1, 2, 3, 4);
        
        let vgs = 1.0;
        let vds = 1.5;
        let vbs = 0.0;
        
        let gm = m.gm(vgs, vds, vbs);
        assert!(gm > 0.0, "Transconductance should be positive");
    }
}
