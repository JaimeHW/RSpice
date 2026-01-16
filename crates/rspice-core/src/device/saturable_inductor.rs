//! Saturable Inductor Model
//!
//! Implements a nonlinear inductor with magnetic saturation behavior.
//! Essential for modeling:
//! - Flyback transformer cores
//! - Boost/buck inductor cores
//! - Filter inductors with DC bias
//! - Power transformer saturation
//!
//! # Model
//! The saturation is modeled using a smooth hyperbolic tangent curve:
//! ```text
//! L(Φ) = L0 / (1 + (Φ/Φsat)^n)
//! ```
//! where:
//! - L0 is the unsaturated inductance
//! - Φ is the magnetic flux (integral of voltage)
//! - Φsat is the saturation flux level
//! - n controls the sharpness of the saturation knee
//!
//! # Parameters
//! | Parameter | Description | Default |
//! |-----------|-------------|---------|
//! | L0 | Unsaturated inductance | Required |
//! | LMIN | Minimum inductance (saturation) | L0/100 |
//! | ISAT | Saturation current | 1A |
//! | NSAT | Saturation exponent (knee sharpness) | 2.0 |
//! | IC | Initial current | 0.0 |

use crate::{circuit::NodeId, Value};
use super::traits::{DynamicDevice, MatrixStamper, NonlinearDevice};

//=============================================================================
// Saturable Inductor
//=============================================================================

/// Saturable inductor with magnetic core saturation
#[derive(Debug, Clone)]
pub struct SaturableInductor {
    /// Device instance name
    pub name: String,
    /// Positive terminal node
    pub node_pos: NodeId,
    /// Negative terminal node
    pub node_neg: NodeId,
    
    // Current branch variable index in MNA
    pub branch_index: Option<NodeId>,
    
    //=========================================================================
    // Model Parameters
    //=========================================================================
    
    /// Unsaturated (initial) inductance in Henries
    pub l0: Value,
    /// Minimum inductance when fully saturated (H)
    pub l_min: Value,
    /// Saturation current (A) - current at which L = L0/2
    pub i_sat: Value,
    /// Saturation exponent - controls knee sharpness (typically 1-10)
    pub n_sat: Value,
    
    //=========================================================================
    // State Variables  
    //=========================================================================
    
    /// Current magnetic flux linkage (Weber-turns = V·s)
    flux: Value,
    /// Current through inductor
    current: Value,
    /// Previous current (for trapezoidal integration)
    current_prev: Value,
    /// Previous flux (for integration)
    flux_prev: Value,
    /// Previous voltage
    voltage_prev: Value,
    /// Current effective inductance
    l_eff: Value,
}

impl SaturableInductor {
    /// Create a new saturable inductor with given parameters
    pub fn new(name: String, node_pos: NodeId, node_neg: NodeId, l0: Value) -> Self {
        Self {
            name,
            node_pos,
            node_neg,
            branch_index: None,
            
            l0,
            l_min: l0 / 100.0,  // Default: saturated L is 1% of unsaturated
            i_sat: 1.0,         // Default: 1A saturation current
            n_sat: 2.0,         // Default: quadratic saturation curve
            
            flux: 0.0,
            current: 0.0,
            current_prev: 0.0,
            flux_prev: 0.0,
            voltage_prev: 0.0,
            l_eff: l0,
        }
    }

    /// Set model parameters from a parameter map
    pub fn with_params(mut self, params: &std::collections::HashMap<String, Value>) -> Self {
        if let Some(&v) = params.get("L0") { self.l0 = v; self.l_eff = v; }
        if let Some(&v) = params.get("L") { self.l0 = v; self.l_eff = v; }  // Alias
        if let Some(&v) = params.get("LMIN") { self.l_min = v; }
        if let Some(&v) = params.get("ISAT") { self.i_sat = v; }
        if let Some(&v) = params.get("NSAT") { self.n_sat = v; }
        if let Some(&v) = params.get("N") { self.n_sat = v; }  // Alias
        if let Some(&v) = params.get("IC") { self.current_prev = v; self.current = v; }
        self
    }

    /// Set saturation parameters
    pub fn with_saturation(mut self, i_sat: Value, l_min: Value, n_sat: Value) -> Self {
        self.i_sat = i_sat;
        self.l_min = l_min;
        self.n_sat = n_sat;
        self
    }

    /// Set initial current
    pub fn set_initial_current(&mut self, current: Value) {
        self.current_prev = current;
        self.current = current;
        // Update flux to match initial current
        self.flux = self.l0 * current;
        self.flux_prev = self.flux;
    }

    /// Set branch index for MNA
    pub fn set_branch_index(&mut self, index: NodeId) {
        self.branch_index = Some(index);
    }

    /// Calculate effective inductance L(I) based on current
    /// 
    /// Uses a smooth saturation curve:
    /// L(I) = L_min + (L0 - L_min) / (1 + |I/I_sat|^n)
    pub fn inductance_at_current(&self, current: Value) -> Value {
        let i_normalized = (current / self.i_sat).abs();
        let saturation_factor = 1.0 + i_normalized.powf(self.n_sat);
        self.l_min + (self.l0 - self.l_min) / saturation_factor
    }

    /// Calculate dL/dI (derivative of inductance with respect to current)
    /// Needed for accurate Newton-Raphson convergence
    pub fn dl_di(&self, current: Value) -> Value {
        let i_abs = current.abs();
        if i_abs < 1e-12 {
            return 0.0;  // At zero current, derivative is zero
        }
        
        let i_norm = i_abs / self.i_sat;
        let i_norm_n = i_norm.powf(self.n_sat);
        let denom = 1.0 + i_norm_n;
        
        // d/dI [ 1/(1 + |I/Isat|^n) ] = -n * |I/Isat|^(n-1) * sign(I) / (Isat * (1 + |I/Isat|^n)^2)
        let dl_di = -(self.l0 - self.l_min) * self.n_sat * i_norm.powf(self.n_sat - 1.0) 
                    * current.signum() / (self.i_sat * denom * denom);
        
        dl_di
    }

    /// Calculate equivalent resistance for trapezoidal integration
    /// Using current effective inductance
    pub fn req(&self, dt: Value) -> Value {
        2.0 * self.l_eff / dt
    }

    /// Calculate voltage across inductor: v = L*di/dt + i*dL/dt
    /// For saturable inductor: v = d(L*i)/dt = L*di/dt + i*dL/di*di/dt = (L + i*dL/di)*di/dt
    pub fn incremental_inductance(&self, current: Value) -> Value {
        let l = self.inductance_at_current(current);
        let dl_di = self.dl_di(current);
        // Incremental inductance: L_inc = L + I * dL/dI
        l + current * dl_di
    }

    /// Get current flux
    pub fn flux(&self) -> Value {
        self.flux
    }

    /// Get current through inductor
    pub fn current(&self) -> Value {
        self.current
    }

    /// Get effective inductance
    pub fn effective_inductance(&self) -> Value {
        self.l_eff
    }

    /// Get saturation ratio (0 = unsaturated, 1 = fully saturated)  
    pub fn saturation_ratio(&self) -> Value {
        let l_range = self.l0 - self.l_min;
        if l_range < 1e-15 {
            return 0.0;
        }
        1.0 - (self.l_eff - self.l_min) / l_range
    }
}

impl DynamicDevice for SaturableInductor {
    fn stamp_transient(
        &self,
        _voltages: &[Value],
        dt: Value,
        matrix: &mut impl MatrixStamper,
        _rhs: &mut [Value],
    ) {
        let branch = self.branch_index.expect("Branch index must be set for inductor");
        
        // Use incremental inductance for better convergence
        let l_inc = self.incremental_inductance(self.current);
        let req = 2.0 * l_inc.max(self.l_min) / dt;
        
        // MNA stamp for inductor (treated as voltage source with series resistance)
        // Row for branch current equation: v+ - v- - req*i = veq
        matrix.stamp(branch, self.node_pos, 1.0);
        matrix.stamp(branch, self.node_neg, -1.0);
        matrix.stamp(branch, branch, -req);
        
        // Node equations: current contribution
        matrix.stamp(self.node_pos, branch, 1.0);
        matrix.stamp(self.node_neg, branch, -1.0);
        
        // Equivalent voltage source for trapezoidal rule
        let veq = req * self.current_prev + self.voltage_prev;
        matrix.stamp_rhs(branch, veq);
    }

    fn step(&mut self, voltages: &[Value], dt: Value) {
        let v_pos = if self.node_pos == 0 { 0.0 } else { voltages[self.node_pos - 1] };
        let v_neg = if self.node_neg == 0 { 0.0 } else { voltages[self.node_neg - 1] };
        let v = v_pos - v_neg;
        
        // Update flux: Φ = ∫v dt (using trapezoidal rule)
        self.flux = self.flux_prev + (dt / 2.0) * (v + self.voltage_prev);
        
        // Update effective inductance based on current
        self.l_eff = self.inductance_at_current(self.current);
        
        // Update current for next step using trapezoidal rule
        // Using current effective inductance
        let l_inc = self.incremental_inductance(self.current).max(self.l_min);
        self.current = self.current_prev + (dt / (2.0 * l_inc)) * (v + self.voltage_prev);
        
        // Store for next iteration
        self.current_prev = self.current;
        self.flux_prev = self.flux;
        self.voltage_prev = v;
    }
}

impl NonlinearDevice for SaturableInductor {
    fn update(&mut self, voltages: &[Value]) {
        let branch = self.branch_index.unwrap_or(0);
        if branch > 0 && branch <= voltages.len() {
            self.current = voltages[branch - 1];
        }
        
        // Update effective inductance
        self.l_eff = self.inductance_at_current(self.current);
    }

    fn stamp_nonlinear(
        &self,
        voltages: &[Value],
        matrix: &mut impl MatrixStamper,
        _rhs: &mut [Value],
    ) {
        // The nonlinear stamp handles the current-dependent inductance
        // This is used during Newton-Raphson iteration within a timestep
        
        // Note: Most of the stamping is done in stamp_transient
        // This method can be used for additional nonlinear corrections if needed
    }

    fn is_converged(&self, tolerance: Value) -> bool {
        // Convergence is typically checked globally
        // But we can add inductor-specific checks if needed
        true
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_saturable_inductor_creation() {
        let ind = SaturableInductor::new("L1".to_string(), 1, 2, 100e-6);
        assert_eq!(ind.name, "L1");
        assert_eq!(ind.l0, 100e-6);
        assert_eq!(ind.l_eff, 100e-6);
    }

    #[test]
    fn test_inductance_unsaturated() {
        let ind = SaturableInductor::new("L1".to_string(), 1, 2, 100e-6)
            .with_saturation(10.0, 10e-6, 2.0);  // 10A saturation current
        
        // At zero current, inductance should be L0
        let l = ind.inductance_at_current(0.0);
        assert!((l - 100e-6).abs() < 1e-12);
    }

    #[test]
    fn test_inductance_at_isat() {
        let ind = SaturableInductor::new("L1".to_string(), 1, 2, 100e-6)
            .with_saturation(1.0, 0.0, 1.0);  // Linear saturation for easy math
        
        // At I = I_sat with n=1, L = L0/2
        let l = ind.inductance_at_current(1.0);
        assert!((l - 50e-6).abs() < 1e-9);
    }

    #[test]
    fn test_inductance_fully_saturated() {
        let ind = SaturableInductor::new("L1".to_string(), 1, 2, 100e-6)
            .with_saturation(1.0, 10e-6, 2.0);
        
        // At very high current, inductance should approach L_min
        let l = ind.inductance_at_current(100.0);  // 100x saturation current
        assert!(l < 15e-6);  // Should be close to L_min = 10e-6
        assert!(l >= 10e-6);  // But not below L_min
    }

    #[test]
    fn test_saturation_symmetric() {
        let ind = SaturableInductor::new("L1".to_string(), 1, 2, 100e-6)
            .with_saturation(1.0, 10e-6, 2.0);
        
        let l_pos = ind.inductance_at_current(2.0);
        let l_neg = ind.inductance_at_current(-2.0);
        
        // Saturation should be symmetric
        assert!((l_pos - l_neg).abs() < 1e-15);
    }

    #[test]
    fn test_incremental_inductance() {
        let ind = SaturableInductor::new("L1".to_string(), 1, 2, 100e-6)
            .with_saturation(1.0, 10e-6, 2.0);
        
        // At I=0, incremental inductance = L0 (no dL/dI contribution)
        let l_inc_0 = ind.incremental_inductance(0.0);
        assert!((l_inc_0 - 100e-6).abs() < 1e-12);
        
        // At higher current, incremental inductance is less than chord inductance
        // due to the negative slope
        let l_chord = ind.inductance_at_current(2.0);
        let l_inc = ind.incremental_inductance(2.0);
        assert!(l_inc < l_chord);  // Incremental < chord in saturation region
    }

    #[test]
    fn test_dl_di() {
        let ind = SaturableInductor::new("L1".to_string(), 1, 2, 100e-6)
            .with_saturation(1.0, 10e-6, 2.0);
        
        // At zero current, dL/dI should be zero (at the peak of L(I))
        let dl_di_0 = ind.dl_di(0.0);
        assert_eq!(dl_di_0, 0.0);
        
        // At positive current, dL/dI should be negative (L decreases as |I| increases)
        let dl_di_pos = ind.dl_di(1.0);
        assert!(dl_di_pos < 0.0);
        
        // dL/dI should be antisymmetric: dL/dI(-I) = -dL/dI(I)
        let dl_di_neg = ind.dl_di(-1.0);
        assert!((dl_di_pos + dl_di_neg).abs() < 1e-15);
    }

    #[test]
    fn test_saturation_ratio() {
        let ind = SaturableInductor::new("L1".to_string(), 1, 2, 100e-6)
            .with_saturation(1.0, 10e-6, 2.0);
        
        // At zero current (unsaturated), ratio should be 0
        let ratio_0 = ind.saturation_ratio();
        assert!((ratio_0 - 0.0).abs() < 1e-10);
    }

    #[test]
    fn test_with_params() {
        use std::collections::HashMap;
        let mut params = HashMap::new();
        params.insert("L0".to_string(), 200e-6);
        params.insert("ISAT".to_string(), 5.0);
        params.insert("NSAT".to_string(), 3.0);
        
        let ind = SaturableInductor::new("L1".to_string(), 1, 2, 100e-6)
            .with_params(&params);
        
        assert_eq!(ind.l0, 200e-6);
        assert_eq!(ind.i_sat, 5.0);
        assert_eq!(ind.n_sat, 3.0);
    }

    #[test]
    fn test_initial_current() {
        let mut ind = SaturableInductor::new("L1".to_string(), 1, 2, 100e-6);
        ind.set_initial_current(2.0);
        
        assert_eq!(ind.current, 2.0);
        assert_eq!(ind.current_prev, 2.0);
        // Flux should be set to match initial current
        assert!((ind.flux - 200e-6).abs() < 1e-12);  // Φ = L0 * I = 100e-6 * 2 = 200e-6 Wb
    }
}
