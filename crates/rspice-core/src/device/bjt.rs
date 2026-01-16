//! BJT (Bipolar Junction Transistor) device model
//!
//! Implements the Ebers-Moll model for NPN and PNP transistors.
//! Supports both large-signal DC and small-signal AC analysis.

use crate::{circuit::NodeId, Value};
use crate::solver::{StaticMatrix, CscIndex};
use super::traits::{NonlinearDevice, MatrixStamper};

/// BJT transistor type
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BjtType {
    Npn,
    Pnp,
}

/// Pre-computed stamp indices for O(1) matrix access (3-terminal device)
/// Layout: [row][col] where row/col are C, B, E
#[derive(Debug, Clone, Default)]
pub struct BjtIndices {
    // Collector row
    pub cc: Option<CscIndex>,
    pub cb: Option<CscIndex>,
    pub ce: Option<CscIndex>,
    // Base row
    pub bc: Option<CscIndex>,
    pub bb: Option<CscIndex>,
    pub be: Option<CscIndex>,
    // Emitter row
    pub ec: Option<CscIndex>,
    pub eb: Option<CscIndex>,
    pub ee: Option<CscIndex>,
}

/// BJT device using the Ebers-Moll model
/// 
/// Terminal connections:
/// - Collector (C)
/// - Base (B)
/// - Emitter (E)
#[derive(Debug, Clone)]
pub struct Bjt {
    pub name: String,
    pub bjt_type: BjtType,
    
    // Node connections
    pub node_collector: NodeId,
    pub node_base: NodeId,
    pub node_emitter: NodeId,
    
    // Model parameters (Ebers-Moll)
    /// Saturation current (IS)
    pub is: Value,
    /// Forward current gain (BF)
    pub bf: Value,
    /// Reverse current gain (BR)
    pub br: Value,
    /// Forward emission coefficient (NF)
    pub nf: Value,
    /// Reverse emission coefficient (NR)
    pub nr: Value,
    /// Thermal voltage (VT = kT/q, ~26mV at 300K)
    pub vt: Value,
    /// Base-emitter built-in potential
    pub vje: Value,
    /// Base-collector built-in potential
    pub vjc: Value,
    /// Forward Early voltage (VAF)
    pub vaf: Value,
    /// Reverse Early voltage (VAR)
    pub var: Value,
    /// Base resistance
    pub rb: Value,
    /// Collector resistance
    pub rc: Value,
    /// Emitter resistance
    pub re: Value,
    
    // Gummel-Poon charge model parameters
    /// Zero-bias B-E junction capacitance (CJE)
    pub cje: Value,
    /// B-E built-in potential (VJE)
    pub mje: Value,
    /// Zero-bias B-C junction capacitance (CJC)
    pub cjc: Value,
    /// B-C grading coefficient (MJC)
    pub mjc: Value,
    /// Forward transit time (TF)
    pub tf: Value,
    /// Reverse transit time (TR)
    pub tr: Value,
    /// Knee current for high-level injection (IKF)
    pub ikf: Value,
    /// Reverse knee current (IKR)
    pub ikr: Value,
    
    // Operating point values (for linearization)
    vbe: Value,
    vbc: Value,
    ic: Value,
    ib: Value,
    ie: Value,
    
    // Previous iteration values (for convergence)
    vbe_prev: Value,
    vbc_prev: Value,
    
    /// Pre-computed matrix indices for O(1) stamping
    pub indices: BjtIndices,
}

impl Bjt {
    /// Create a new NPN BJT with default 2N2222 parameters
    pub fn new_npn(name: String, collector: NodeId, base: NodeId, emitter: NodeId) -> Self {
        Self::new(name, BjtType::Npn, collector, base, emitter)
    }

    /// Create a new PNP BJT with default 2N2907 parameters
    pub fn new_pnp(name: String, collector: NodeId, base: NodeId, emitter: NodeId) -> Self {
        Self::new(name, BjtType::Pnp, collector, base, emitter)
    }

    fn new(name: String, bjt_type: BjtType, collector: NodeId, base: NodeId, emitter: NodeId) -> Self {
        Self {
            name,
            bjt_type,
            node_collector: collector,
            node_base: base,
            node_emitter: emitter,
            
            // Default parameters (2N2222-like for NPN)
            is: 1e-14,      // Saturation current
            bf: 200.0,      // Forward current gain
            br: 1.0,        // Reverse current gain
            nf: 1.0,        // Forward emission coefficient
            nr: 1.0,        // Reverse emission coefficient
            vt: 0.02585,    // Thermal voltage at 300K
            vje: 0.75,      // B-E built-in potential
            vjc: 0.75,      // B-C built-in potential
            vaf: 100.0,     // Forward Early voltage
            var: f64::INFINITY, // Reverse Early voltage
            rb: 10.0,       // Base resistance
            rc: 1.0,        // Collector resistance
            re: 0.1,        // Emitter resistance
            
            // Gummel-Poon parameters
            cje: 1e-12,     // B-E junction capacitance
            mje: 0.33,      // B-E grading coefficient
            cjc: 0.5e-12,   // B-C junction capacitance
            mjc: 0.33,      // B-C grading coefficient
            tf: 4e-10,      // Forward transit time (400ps)
            tr: 5e-9,       // Reverse transit time (5ns)
            ikf: 0.1,       // Knee current (100mA)
            ikr: 0.01,      // Reverse knee
            
            vbe: 0.0,
            vbc: 0.0,
            ic: 0.0,
            ib: 0.0,
            ie: 0.0,
            vbe_prev: 0.0,
            vbc_prev: 0.0,
            indices: BjtIndices::default(),
        }
    }

    /// Set model parameters from a DeviceModel
    pub fn with_params(mut self, params: &std::collections::HashMap<String, Value>) -> Self {
        // DC parameters
        if let Some(&v) = params.get("IS") { self.is = v; }
        if let Some(&v) = params.get("BF") { self.bf = v; }
        if let Some(&v) = params.get("BR") { self.br = v; }
        if let Some(&v) = params.get("NF") { self.nf = v; }
        if let Some(&v) = params.get("NR") { self.nr = v; }
        if let Some(&v) = params.get("VAF") { self.vaf = v; }
        if let Some(&v) = params.get("VAR") { self.var = v; }
        if let Some(&v) = params.get("RB") { self.rb = v; }
        if let Some(&v) = params.get("RC") { self.rc = v; }
        if let Some(&v) = params.get("RE") { self.re = v; }
        // Gummel-Poon charge parameters
        if let Some(&v) = params.get("CJE") { self.cje = v; }
        if let Some(&v) = params.get("MJE") { self.mje = v; }
        if let Some(&v) = params.get("CJC") { self.cjc = v; }
        if let Some(&v) = params.get("MJC") { self.mjc = v; }
        if let Some(&v) = params.get("TF") { self.tf = v; }
        if let Some(&v) = params.get("TR") { self.tr = v; }
        if let Some(&v) = params.get("IKF") { self.ikf = v; }
        if let Some(&v) = params.get("IKR") { self.ikr = v; }
        self
    }

    /// Calculate base-emitter junction capacitance
    /// Cbe = CJE / (1 - Vbe/VJE)^MJE + gm * TF
    pub fn cbe(&self, vbe: Value, gm: Value) -> Value {
        let p = self.polarity();
        let v = (p * vbe).min(0.9 * self.vje);  // Clamp to avoid singularity
        let cj = self.cje / (1.0 - v / self.vje).powf(self.mje);
        let cd = gm * self.tf;  // Diffusion capacitance
        cj + cd
    }

    /// Calculate base-collector junction capacitance
    /// Cbc = CJC / (1 - Vbc/VJC)^MJC
    pub fn cbc(&self, vbc: Value) -> Value {
        let p = self.polarity();
        let v = (p * vbc).min(0.9 * self.vjc);  // Clamp to avoid singularity
        self.cjc / (1.0 - v / self.vjc).powf(self.mjc)
    }

    /// Calculate total capacitances for transient analysis
    /// Returns (Cbe, Cbc)
    pub fn junction_capacitances(&self, vbe: Value, vbc: Value) -> (Value, Value) {
        let gm = self.gm(vbe);
        (self.cbe(vbe, gm), self.cbc(vbc))
    }

    /// Link this device to a StaticMatrix for O(1) stamping
    pub fn link(&mut self, matrix: &StaticMatrix) {
        let c = self.node_collector;
        let b = self.node_base;
        let e = self.node_emitter;
        
        // Collector row
        if c > 0 && c > 0 { self.indices.cc = matrix.get_index(c - 1, c - 1); }
        if c > 0 && b > 0 { self.indices.cb = matrix.get_index(c - 1, b - 1); }
        if c > 0 && e > 0 { self.indices.ce = matrix.get_index(c - 1, e - 1); }
        // Base row
        if b > 0 && c > 0 { self.indices.bc = matrix.get_index(b - 1, c - 1); }
        if b > 0 && b > 0 { self.indices.bb = matrix.get_index(b - 1, b - 1); }
        if b > 0 && e > 0 { self.indices.be = matrix.get_index(b - 1, e - 1); }
        // Emitter row
        if e > 0 && c > 0 { self.indices.ec = matrix.get_index(e - 1, c - 1); }
        if e > 0 && b > 0 { self.indices.eb = matrix.get_index(e - 1, b - 1); }
        if e > 0 && e > 0 { self.indices.ee = matrix.get_index(e - 1, e - 1); }
    }

    /// Stamp using O(1) direct indexing (call after link)
    pub fn stamp_direct(&self, matrix: &mut StaticMatrix, rhs: &mut [Value], voltages: &[Value]) {
        let vc = if self.node_collector == 0 { 0.0 } else { voltages[self.node_collector - 1] };
        let vb = if self.node_base == 0 { 0.0 } else { voltages[self.node_base - 1] };
        let ve = if self.node_emitter == 0 { 0.0 } else { voltages[self.node_emitter - 1] };
        
        let vbe = vb - ve;
        let vbc = vb - vc;
        
        // Linearized conductances
        let gm = self.gm(vbe);
        let go = self.go(self.ic);
        let gbe = self.gbe(vbe);
        let gbc = self.gbc(vbc);
        
        // Equivalent currents
        let (ic, ib, _ie) = self.calculate_currents(vbe, vbc);
        let ic_eq = ic - gm * vbe - go * (vc - ve);
        let ib_eq = ib - gbe * vbe - gbc * vbc;
        
        // Stamp matrix using direct indexing
        // Collector row
        if let Some(idx) = self.indices.cc { matrix.stamp_direct(idx, go + gbc); }
        if let Some(idx) = self.indices.cb { matrix.stamp_direct(idx, gm - gbc); }
        if let Some(idx) = self.indices.ce { matrix.stamp_direct(idx, -gm - go); }
        // Base row
        if let Some(idx) = self.indices.bc { matrix.stamp_direct(idx, -gbc); }
        if let Some(idx) = self.indices.bb { matrix.stamp_direct(idx, gbe + gbc); }
        if let Some(idx) = self.indices.be { matrix.stamp_direct(idx, -gbe); }
        // Emitter row
        if let Some(idx) = self.indices.ec { matrix.stamp_direct(idx, -go); }
        if let Some(idx) = self.indices.eb { matrix.stamp_direct(idx, -gm); }
        if let Some(idx) = self.indices.ee { matrix.stamp_direct(idx, gm + go); }
        
        // Stamp RHS
        if self.node_collector > 0 { rhs[self.node_collector - 1] -= ic_eq; }
        if self.node_base > 0 { rhs[self.node_base - 1] -= ib_eq; }
        if self.node_emitter > 0 { rhs[self.node_emitter - 1] += ic_eq + ib_eq; }
    }

    /// Get polarity multiplier (+1 for NPN, -1 for PNP)
    fn polarity(&self) -> Value {
        match self.bjt_type {
            BjtType::Npn => 1.0,
            BjtType::Pnp => -1.0,
        }
    }

    /// Diode current: I = Is * (exp(V / (n * Vt)) - 1)
    fn diode_current(&self, v: Value, n: Value) -> Value {
        let v_limited = v.min(80.0 * n * self.vt);
        self.is * ((v_limited / (n * self.vt)).exp() - 1.0)
    }

    /// Diode conductance: g = Is / (n * Vt) * exp(V / (n * Vt))
    fn diode_conductance(&self, v: Value, n: Value) -> Value {
        let v_limited = v.min(80.0 * n * self.vt);
        (self.is / (n * self.vt)) * (v_limited / (n * self.vt)).exp()
    }

    /// Calculate BJT currents using Ebers-Moll with Gummel-Poon enhancements
    /// 
    /// Base model is Ebers-Moll for stability. Early voltage and high-injection
    /// effects are applied via go() output conductance and base charge modulation.
    fn calculate_currents(&self, vbe: Value, vbc: Value) -> (Value, Value, Value) {
        let p = self.polarity();
        let vbe_eff = p * vbe;
        let vbc_eff = p * vbc;
        
        // Forward and reverse diode currents
        let if_diode = self.diode_current(vbe_eff, self.nf);
        let ir_diode = self.diode_current(vbc_eff, self.nr);
        
        // High-injection correction (Gummel-Poon)
        // At high currents (If >> IKF), effective beta is reduced
        let ikf_ratio = if_diode / self.ikf.max(1e-6);
        let ikr_ratio = ir_diode / self.ikr.max(1e-6);
        
        // Smooth high-injection factor: approaches 1/sqrt(I/IK) at high currents
        let hf_factor = 1.0 / (1.0 + ikf_ratio.sqrt()).max(0.1);
        let hr_factor = 1.0 / (1.0 + ikr_ratio.sqrt()).max(0.1);
        
        // Ebers-Moll with high-injection modification
        let ic = p * (if_diode * hf_factor - ir_diode * hr_factor - ir_diode / self.br);
        let ib = p * (if_diode / self.bf + ir_diode / self.br);
        let ie = -(ic + ib); // KCL: Ic + Ib + Ie = 0
        
        (ic, ib, ie)
    }

    /// Get transconductance gm = dIc/dVbe with Gummel-Poon high-injection
    /// 
    /// Includes the reduction in gm at high currents due to high-injection.
    fn gm(&self, vbe: Value) -> Value {
        let p = self.polarity();
        let vbe_eff = p * vbe;
        
        // Base diode conductance
        let g_diode = self.diode_conductance(vbe_eff, self.nf);
        let if_diode = self.diode_current(vbe_eff, self.nf);
        
        // High-injection correction factor and its derivative
        let ikf_ratio = if_diode / self.ikf.max(1e-6);
        let hf = 1.0 / (1.0 + ikf_ratio.sqrt()).max(0.1);
        
        // d(hf)/dVbe approx for smooth behavior (simplified)
        // At low currents: gm ≈ g_diode
        // At high currents: gm ≈ g_diode * hf (reduced)
        g_diode * hf
    }

    /// Get output conductance go = dIc/dVce (Early effect)
    fn go(&self, ic: Value) -> Value {
        if self.vaf.is_finite() {
            ic.abs() / self.vaf
        } else {
            1e-12 // Minimum conductance
        }
    }

    /// Get base-emitter junction conductance
    fn gbe(&self, vbe: Value) -> Value {
        self.diode_conductance(self.polarity() * vbe, self.nf) / self.bf
    }

    /// Get base-collector junction conductance
    fn gbc(&self, vbc: Value) -> Value {
        self.diode_conductance(self.polarity() * vbc, self.nr) / self.br
    }
}

impl NonlinearDevice for Bjt {
    fn update(&mut self, voltages: &[Value]) {
        let vc = if self.node_collector == 0 { 0.0 } else { voltages[self.node_collector - 1] };
        let vb = if self.node_base == 0 { 0.0 } else { voltages[self.node_base - 1] };
        let ve = if self.node_emitter == 0 { 0.0 } else { voltages[self.node_emitter - 1] };
        
        self.vbe_prev = self.vbe;
        self.vbc_prev = self.vbc;
        
        self.vbe = vb - ve;
        self.vbc = vb - vc;
        
        let (ic, ib, ie) = self.calculate_currents(self.vbe, self.vbc);
        self.ic = ic;
        self.ib = ib;
        self.ie = ie;
    }

    fn stamp_nonlinear(
        &self,
        voltages: &[Value],
        matrix: &mut impl MatrixStamper,
        _rhs: &mut [Value],
    ) {
        let vc = if self.node_collector == 0 { 0.0 } else { voltages[self.node_collector - 1] };
        let vb = if self.node_base == 0 { 0.0 } else { voltages[self.node_base - 1] };
        let ve = if self.node_emitter == 0 { 0.0 } else { voltages[self.node_emitter - 1] };
        
        let vbe = vb - ve;
        let vbc = vb - vc;
        
        // Linearized conductances
        let gm = self.gm(vbe);
        let go = self.go(self.ic);
        let gbe = self.gbe(vbe);
        let gbc = self.gbc(vbc);
        
        // Equivalent currents for linearization
        let (ic, ib, _ie) = self.calculate_currents(vbe, vbc);
        let ic_eq = ic - gm * vbe - go * (vc - ve);
        let ib_eq = ib - gbe * vbe - gbc * vbc;
        
        // Stamp the linearized model
        // Collector node equation
        matrix.stamp(self.node_collector, self.node_collector, go + gbc);
        matrix.stamp(self.node_collector, self.node_base, gm - gbc);
        matrix.stamp(self.node_collector, self.node_emitter, -gm - go);
        
        // Base node equation  
        matrix.stamp(self.node_base, self.node_collector, -gbc);
        matrix.stamp(self.node_base, self.node_base, gbe + gbc);
        matrix.stamp(self.node_base, self.node_emitter, -gbe);
        
        // Emitter node equation
        matrix.stamp(self.node_emitter, self.node_collector, -go);
        matrix.stamp(self.node_emitter, self.node_base, -gm);
        matrix.stamp(self.node_emitter, self.node_emitter, gm + go);
        
        // Stamp equivalent current sources
        matrix.stamp_rhs(self.node_collector, -ic_eq);
        matrix.stamp_rhs(self.node_base, -ib_eq);
        matrix.stamp_rhs(self.node_emitter, ic_eq + ib_eq);
    }

    fn is_converged(&self, tolerance: Value) -> bool {
        let vbe_diff = (self.vbe - self.vbe_prev).abs();
        let vbc_diff = (self.vbc - self.vbc_prev).abs();
        vbe_diff < tolerance && vbc_diff < tolerance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bjt_creation() {
        let q = Bjt::new_npn("Q1".to_string(), 2, 1, 0);
        assert_eq!(q.bjt_type, BjtType::Npn);
        assert_eq!(q.node_collector, 2);
        assert_eq!(q.node_base, 1);
        assert_eq!(q.node_emitter, 0);
    }

    #[test]
    fn test_bjt_forward_active() {
        let q = Bjt::new_npn("Q1".to_string(), 2, 1, 0);
        
        // Typical forward active: Vbe ~ 0.7V, Vbc < 0
        let (ic, ib, ie) = q.calculate_currents(0.7, -5.0);
        
        // Ic should be positive and >> Ib
        assert!(ic > 0.0);
        assert!(ib > 0.0);
        assert!(ic > ib * 10.0); // Beta > 10
        
        // KCL check
        assert!((ic + ib + ie).abs() < 1e-12);
    }

    #[test]
    fn test_pnp_polarity() {
        let npn = Bjt::new_npn("Q1".to_string(), 2, 1, 0);
        let pnp = Bjt::new_pnp("Q2".to_string(), 2, 1, 0);
        
        assert_eq!(npn.polarity(), 1.0);
        assert_eq!(pnp.polarity(), -1.0);
    }

    #[test]
    fn test_bjt_junction_capacitances() {
        let q = Bjt::new_npn("Q1".to_string(), 2, 1, 0);
        
        // Forward active: Vbe=0.7V, Vbc=-5V
        let (cbe, cbc) = q.junction_capacitances(0.7, -5.0);
        
        // Cbe should be larger (forward biased + diffusion cap from TF*gm)
        assert!(cbe > 1e-12, "Expected Cbe > 1pF, got {:.2e}", cbe);
        
        // Cbc should be small (reverse biased)
        assert!(cbc > 0.1e-12 && cbc < 5e-12, "Expected Cbc ~0.5pF, got {:.2e}", cbc);
        
        // Cbe should be larger than Cbc in forward active
        assert!(cbe > cbc, "Expected Cbe > Cbc in forward active");
    }

    #[test]
    fn test_bjt_gummel_poon_params() {
        use std::collections::HashMap;
        
        let mut params = HashMap::new();
        params.insert("CJE".to_string(), 2e-12);
        params.insert("CJC".to_string(), 1e-12);
        params.insert("TF".to_string(), 1e-9);
        params.insert("TR".to_string(), 10e-9);
        params.insert("IKF".to_string(), 0.05);
        
        let q = Bjt::new_npn("Q1".to_string(), 2, 1, 0)
            .with_params(&params);
        
        assert_eq!(q.cje, 2e-12);
        assert_eq!(q.cjc, 1e-12);
        assert_eq!(q.tf, 1e-9);
        assert_eq!(q.tr, 10e-9);
        assert_eq!(q.ikf, 0.05);
    }
}
