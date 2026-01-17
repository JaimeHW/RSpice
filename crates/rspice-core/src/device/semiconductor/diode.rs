//! Diode device model

use crate::{circuit::NodeId, Value};
use crate::solver::{StaticMatrix, CscIndex};
use crate::device::traits::{NonlinearDevice, MatrixStamper};

/// Pre-computed stamp indices for O(1) matrix access (2-terminal device)
#[derive(Debug, Clone, Default)]
pub struct DiodeIndices {
    /// Matrix stamps: (anode,anode), (anode,cathode), (cathode,anode), (cathode,cathode)
    pub aa: Option<CscIndex>,
    pub ac: Option<CscIndex>,
    pub ca: Option<CscIndex>,
    pub cc: Option<CscIndex>,
}

/// Semiconductor diode with Shockley equation model
#[derive(Debug, Clone)]
pub struct Diode {
    pub name: String,
    pub node_anode: NodeId,
    pub node_cathode: NodeId,
    /// Saturation current (Is)
    pub is: Value,
    /// Emission coefficient (N)
    pub n: Value,
    /// Thermal voltage (Vt = kT/q, ~26mV at room temp)
    pub vt: Value,
    /// Series resistance
    pub rs: Value,
    
    // Junction capacitance parameters
    /// Zero-bias junction capacitance (CJ0)
    pub cj0: Value,
    /// Built-in potential (VJ)
    pub vj: Value,
    /// Grading coefficient (M)
    pub m: Value,
    /// Transit time for diffusion capacitance (TT)
    pub tt: Value,
    
    /// Previous iteration voltage (for convergence check)
    prev_vd: Value,
    /// Previous voltage for convergence
    prev_vd_old: Value,
    /// Previous iteration current
    prev_id: Value,
    /// Pre-computed matrix indices for O(1) stamping
    pub indices: DiodeIndices,
}

impl Diode {
    /// Create a new diode with default 1N4148 parameters
    pub fn new(name: String, node_anode: NodeId, node_cathode: NodeId) -> Self {
        Self {
            name,
            node_anode,
            node_cathode,
            is: 2.52e-9,    // Saturation current
            n: 1.752,       // Emission coefficient
            vt: 0.02585,    // Thermal voltage at 300K
            rs: 0.568,      // Series resistance
            
            // Junction capacitance (1N4148-like)
            cj0: 4e-12,     // Zero-bias junction capacitance (4pF)
            vj: 0.7,        // Built-in potential
            m: 0.5,         // Grading coefficient
            tt: 8e-9,       // Transit time (8ns)
            
            prev_vd: 0.0,
            prev_vd_old: 0.0,
            prev_id: 0.0,
            indices: DiodeIndices::default(),
        }
    }

    /// Create diode with custom DC model parameters
    pub fn with_params(mut self, is: Value, n: Value, rs: Value) -> Self {
        self.is = is;
        self.n = n;
        self.rs = rs;
        self
    }

    /// Set junction capacitance parameters
    pub fn with_capacitance(mut self, cj0: Value, vj: Value, m: Value, tt: Value) -> Self {
        self.cj0 = cj0;
        self.vj = vj;
        self.m = m;
        self.tt = tt;
        self
    }

    /// Calculate junction capacitance: Cj = CJ0 / (1 - Vd/VJ)^M
    /// Includes depletion (junction) and diffusion capacitance
    pub fn junction_capacitance(&self, vd: Value) -> Value {
        // Clamp voltage to avoid singularity at forward bias > VJ
        let v_clamped = vd.min(0.9 * self.vj);
        
        // Junction (depletion) capacitance
        let cj = if v_clamped < 0.0 {
            // Reverse bias: standard formula
            self.cj0 / (1.0 - v_clamped / self.vj).powf(self.m)
        } else {
            // Forward bias: linear approximation above VJ/2
            let fc = 0.5;  // Where to switch to linear
            if v_clamped < fc * self.vj {
                self.cj0 / (1.0 - v_clamped / self.vj).powf(self.m)
            } else {
                // Linear extrapolation
                let cj_fc = self.cj0 / (1.0 - fc).powf(self.m);
                let dcj = cj_fc * self.m / (self.vj * (1.0 - fc));
                cj_fc + dcj * (v_clamped - fc * self.vj)
            }
        };
        
        // Diffusion capacitance: Cd = TT * gd (where gd is conductance)
        let gd = self.diode_conductance(vd);
        let cd = self.tt * gd;
        
        cj + cd
    }

    /// Link this device to a StaticMatrix for O(1) stamping
    pub fn link(&mut self, matrix: &StaticMatrix) {
        let a = self.node_anode;
        let c = self.node_cathode;
        
        if a > 0 && a > 0 { self.indices.aa = matrix.get_index(a - 1, a - 1); }
        if a > 0 && c > 0 { self.indices.ac = matrix.get_index(a - 1, c - 1); }
        if c > 0 && a > 0 { self.indices.ca = matrix.get_index(c - 1, a - 1); }
        if c > 0 && c > 0 { self.indices.cc = matrix.get_index(c - 1, c - 1); }
    }

    /// Stamp using O(1) direct indexing (call after link)
    pub fn stamp_direct(&self, matrix: &mut StaticMatrix, rhs: &mut [Value], voltages: &[Value]) {
        let va = if self.node_anode == 0 { 0.0 } else { voltages[self.node_anode - 1] };
        let vc = if self.node_cathode == 0 { 0.0 } else { voltages[self.node_cathode - 1] };
        let vd = va - vc;
        
        // Linearize around current operating point
        let id = self.current(vd);
        let gd = self.diode_conductance(vd);
        
        // Equivalent current source: ieq = id - gd * vd
        let ieq = id - gd * vd;
        
        // Stamp conductance using direct indexing
        if let Some(idx) = self.indices.aa { matrix.stamp_direct(idx, gd); }
        if let Some(idx) = self.indices.ac { matrix.stamp_direct(idx, -gd); }
        if let Some(idx) = self.indices.ca { matrix.stamp_direct(idx, -gd); }
        if let Some(idx) = self.indices.cc { matrix.stamp_direct(idx, gd); }
        
        // Stamp RHS (still needs bounds check for ground nodes)
        if self.node_anode > 0 { rhs[self.node_anode - 1] -= ieq; }
        if self.node_cathode > 0 { rhs[self.node_cathode - 1] += ieq; }
    }

    /// Shockley diode equation: I = Is * (exp(Vd / (N * Vt)) - 1)
    /// Public for noise analysis (shot noise = 2qI)
    pub fn current(&self, vd: Value) -> Value {
        // Limit voltage to prevent overflow
        let vd_limited = vd.min(80.0 * self.n * self.vt);
        self.is * ((vd_limited / (self.n * self.vt)).exp() - 1.0)
    }

    /// Diode conductance (derivative of current): gd = Is / (N * Vt) * exp(Vd / (N * Vt))
    fn diode_conductance(&self, vd: Value) -> Value {
        let vd_limited = vd.min(80.0 * self.n * self.vt);
        (self.is / (self.n * self.vt)) * (vd_limited / (self.n * self.vt)).exp()
    }
}

impl NonlinearDevice for Diode {
    fn update(&mut self, voltages: &[Value]) {
        let va = if self.node_anode == 0 { 0.0 } else { voltages[self.node_anode - 1] };
        let vc = if self.node_cathode == 0 { 0.0 } else { voltages[self.node_cathode - 1] };
        self.prev_vd_old = self.prev_vd;
        self.prev_vd = va - vc;
        self.prev_id = self.current(self.prev_vd);
    }

    fn stamp_nonlinear(
        &self,
        voltages: &[Value],
        matrix: &mut impl MatrixStamper,
        _rhs: &mut [Value],
    ) {
        let va = if self.node_anode == 0 { 0.0 } else { voltages[self.node_anode - 1] };
        let vc = if self.node_cathode == 0 { 0.0 } else { voltages[self.node_cathode - 1] };
        let vd = va - vc;
        
        // Linearize around current operating point
        let id = self.current(vd);
        let gd = self.diode_conductance(vd);
        
        // Equivalent current source: ieq = id - gd * vd
        let ieq = id - gd * vd;
        
        // Stamp conductance
        matrix.stamp(self.node_anode, self.node_anode, gd);
        matrix.stamp(self.node_anode, self.node_cathode, -gd);
        matrix.stamp(self.node_cathode, self.node_anode, -gd);
        matrix.stamp(self.node_cathode, self.node_cathode, gd);
        
        // Stamp equivalent current source
        matrix.stamp_rhs(self.node_anode, -ieq);
        matrix.stamp_rhs(self.node_cathode, ieq);
    }

    fn is_converged(&self, tolerance: Value) -> bool {
        (self.prev_vd - self.prev_vd_old).abs() < tolerance
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diode_forward() {
        let d = Diode::new("D1".to_string(), 1, 0);
        // Forward bias at ~0.7V should give significant current
        let id = d.current(0.7);
        assert!(id > 0.001); // Should be in mA range
    }

    #[test]
    fn test_diode_reverse() {
        let d = Diode::new("D1".to_string(), 1, 0);
        // Reverse bias should give ~Is
        let id = d.current(-1.0);
        assert!(id.abs() < 1e-8);
    }

    #[test]
    fn test_junction_capacitance() {
        let d = Diode::new("D1".to_string(), 1, 0);
        
        // Zero bias: should be close to CJ0
        let c0 = d.junction_capacitance(0.0);
        assert!(c0 > 3e-12 && c0 < 10e-12, "Expected ~CJ0 at 0V, got {:.2e}", c0);
        
        // Reverse bias: capacitance decreases
        let c_rev = d.junction_capacitance(-5.0);
        assert!(c_rev < c0, "Expected lower cap at reverse bias");
        
        // Forward bias: capacitance increases (junction) + diffusion
        let c_fwd = d.junction_capacitance(0.5);
        assert!(c_fwd > c0, "Expected higher cap at forward bias");
    }

    #[test]
    fn test_diode_with_capacitance() {
        let d = Diode::new("D1".to_string(), 1, 0)
            .with_capacitance(10e-12, 0.7, 0.5, 5e-9);
        
        assert_eq!(d.cj0, 10e-12);
        assert_eq!(d.vj, 0.7);
        assert_eq!(d.m, 0.5);
        assert_eq!(d.tt, 5e-9);
    }
}
