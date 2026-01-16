//! JFET (Junction Field-Effect Transistor) Device Model
//!
//! Implements the Shichman-Hodges model for N-channel and P-channel JFETs.
//!
//! # Model Equations
//!
//! For N-JFET (P-JFET uses opposite polarities):
//!
//! **Cutoff** (Vgs - Vto ≤ 0):
//! ```text
//! Ids = 0
//! ```
//!
//! **Linear** (Vds < Vgs - Vto):
//! ```text
//! Ids = Beta * (2*(Vgs-Vto)*Vds - Vds²) * (1 + Lambda*Vds)
//! ```
//!
//! **Saturation** (Vds ≥ Vgs - Vto):
//! ```text
//! Ids = Beta * (Vgs - Vto)² * (1 + Lambda*Vds)
//! ```
//!
//! where Beta is typically derived from IDSS: `Beta = IDSS / Vto²`
//!
//! # Example
//!
//! ```ignore
//! J1 drain gate source JMOD
//! .MODEL JMOD NJF(VTO=-2 BETA=1E-3 LAMBDA=0.01)
//! ```

use crate::Value;
use std::f64::consts::E;

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

//=============================================================================
// JFET Parameters
//=============================================================================

/// JFET model parameters (Shichman-Hodges level 1)
#[derive(Debug, Clone)]
pub struct JfetParams {
    /// Threshold voltage (V) - negative for N-JFET depletion mode
    pub vto: Value,
    /// Transconductance coefficient (A/V²)
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
    /// Drain ohmic resistance (Ω)
    pub rd: Value,
    /// Source ohmic resistance (Ω)
    pub rs: Value,
    /// Forward bias junction coefficient
    pub fc: Value,
    /// Gate junction emission coefficient
    pub n: Value,
    /// Nominal temperature (K)
    pub tnom: Value,
}

impl Default for JfetParams {
    fn default() -> Self {
        Self {
            vto: -2.0,        // Threshold voltage (depletion mode)
            beta: 1e-4,       // Transconductance coefficient
            lambda: 0.0,      // Channel-length modulation
            is: 1e-14,        // Gate saturation current
            cgs: 0.0,         // Gate-source capacitance
            cgd: 0.0,         // Gate-drain capacitance
            pb: 1.0,          // Junction potential
            m: 0.5,           // Grading coefficient
            rd: 0.0,          // Drain resistance
            rs: 0.0,          // Source resistance
            fc: 0.5,          // Forward bias coefficient
            n: 1.0,           // Emission coefficient
            tnom: 300.15,     // 27°C nominal
        }
    }
}

impl JfetParams {
    /// Create parameters from IDSS and VTO
    ///
    /// IDSS is the drain current at Vgs=0, Vds >> Vgs-Vto (saturation)
    /// Beta = IDSS / Vto²
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

/// JFET device instance
#[derive(Debug, Clone)]
pub struct Jfet {
    /// Instance name
    pub name: String,
    /// JFET type (NJF or PJF)
    pub jfet_type: JfetType,
    /// Drain node index
    pub drain: usize,
    /// Gate node index
    pub gate: usize,
    /// Source node index
    pub source: usize,
    /// Model parameters
    pub params: JfetParams,
    /// Device multiplier
    pub m: Value,
    /// Area factor
    pub area: Value,
}

impl Jfet {
    /// Create a new N-JFET
    pub fn njf(name: &str, drain: usize, gate: usize, source: usize) -> Self {
        Self {
            name: name.to_string(),
            jfet_type: JfetType::NJF,
            drain,
            gate,
            source,
            params: JfetParams::default(),
            m: 1.0,
            area: 1.0,
        }
    }

    /// Create a new P-JFET
    pub fn pjf(name: &str, drain: usize, gate: usize, source: usize) -> Self {
        Self {
            name: name.to_string(),
            jfet_type: JfetType::PJF,
            drain,
            gate,
            source,
            params: JfetParams::default(),
            m: 1.0,
            area: 1.0,
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

    /// Thermal voltage at given temperature
    fn thermal_voltage(&self, temp: Value) -> Value {
        const K_BOLTZMANN: Value = 1.380649e-23;
        const Q_ELECTRON: Value = 1.602176634e-19;
        K_BOLTZMANN * temp / Q_ELECTRON
    }

    /// Calculate drain current and conductances
    ///
    /// Returns (Ids, gm, gds) where:
    /// - Ids: drain-source current
    /// - gm: transconductance ∂Ids/∂Vgs
    /// - gds: output conductance ∂Ids/∂Vds
    pub fn calculate(&self, vgs: Value, vds: Value, temp: Value) -> (Value, Value, Value) {
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
                let ids = -beta * (2.0 * vgst_rev * vds_rev - vds_rev * vds_rev) 
                    * (1.0 + lambda * vds_rev);
                let gm = 2.0 * beta * vds_rev * (1.0 + lambda * vds_rev);
                let gds = beta * 2.0 * (vgst_rev - vds_rev) * (1.0 + lambda * vds_rev)
                    + beta * (2.0 * vgst_rev * vds_rev - vds_rev * vds_rev) * lambda;
                (-ids, gm, gds)
            } else {
                // Saturation (reversed)
                let ids = -beta * vgst_rev * vgst_rev * (1.0 + lambda * vds_rev);
                let gm = 2.0 * beta * vgst_rev * (1.0 + lambda * vds_rev);
                let gds = beta * vgst_rev * vgst_rev * lambda;
                (-ids, gm, gds)
            }
        } else if vds_int <= vgst {
            // Linear (triode) region: Vds < Vgs - Vto
            let ids = beta * (2.0 * vgst * vds_int - vds_int * vds_int) 
                * (1.0 + lambda * vds_int);
            
            // gm = ∂Ids/∂Vgs = 2 * beta * Vds * (1 + lambda * Vds)
            let gm = 2.0 * beta * vds_int * (1.0 + lambda * vds_int);
            
            // gds = ∂Ids/∂Vds = beta * 2 * (Vgst - Vds) * (1 + lambda*Vds) 
            //                   + beta * (2*Vgst*Vds - Vds²) * lambda
            let gds = beta * 2.0 * (vgst - vds_int) * (1.0 + lambda * vds_int)
                + beta * (2.0 * vgst * vds_int - vds_int * vds_int) * lambda;
            
            (ids, gm, gds)
        } else {
            // Saturation region: Vds >= Vgs - Vto
            let ids = beta * vgst * vgst * (1.0 + lambda * vds_int);
            
            // gm = ∂Ids/∂Vgs = 2 * beta * Vgst * (1 + lambda * Vds)
            let gm = 2.0 * beta * vgst * (1.0 + lambda * vds_int);
            
            // gds = ∂Ids/∂Vds = beta * Vgst² * lambda
            let gds = beta * vgst * vgst * lambda;
            
            (ids, gm, gds)
        };
        
        // Apply polarity for output current
        (pol * ids, gm, gds)
    }

    /// Calculate gate junction current (reverse-biased diodes)
    ///
    /// Returns (Igs, Igd) - gate-source and gate-drain junction currents
    pub fn gate_current(&self, vgs: Value, vgd: Value, temp: Value) -> (Value, Value) {
        let vt = self.thermal_voltage(temp);
        let is = self.params.is * self.area;
        let n = self.params.n;
        
        // Gate-source junction (reverse biased for normal operation)
        let igs = is * (E.powf(vgs / (n * vt)) - 1.0);
        
        // Gate-drain junction
        let igd = is * (E.powf(vgd / (n * vt)) - 1.0);
        
        (igs, igd)
    }

    /// Calculate junction capacitances
    ///
    /// Returns (Cgs, Cgd) - gate-source and gate-drain capacitances
    pub fn capacitances(&self, vgs: Value, vgd: Value) -> (Value, Value) {
        let cgs0 = self.params.cgs * self.area;
        let cgd0 = self.params.cgd * self.area;
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
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_jfet_creation() {
        let jfet = Jfet::njf("J1", 1, 2, 3);
        assert_eq!(jfet.name, "J1");
        assert_eq!(jfet.jfet_type, JfetType::NJF);
        assert_eq!(jfet.drain, 1);
        assert_eq!(jfet.gate, 2);
        assert_eq!(jfet.source, 3);
    }

    #[test]
    fn test_pjf_creation() {
        let jfet = Jfet::pjf("J2", 1, 2, 3);
        assert_eq!(jfet.jfet_type, JfetType::PJF);
        assert_eq!(jfet.jfet_type.polarity(), -1.0);
    }

    #[test]
    fn test_cutoff_region() {
        let params = JfetParams::default(); // VTO = -2V
        let jfet = Jfet::njf("J1", 1, 2, 0).with_params(params);
        
        // Vgs = -3V < Vto = -2V → cutoff
        let (ids, gm, gds) = jfet.calculate(-3.0, 5.0, 300.0);
        
        assert!(ids.abs() < 1e-15, "Ids should be ~0 in cutoff, got {}", ids);
        assert!(gm.abs() < 1e-15, "gm should be 0 in cutoff");
        assert!(gds.abs() < 1e-15, "gds should be 0 in cutoff");
    }

    #[test]
    fn test_saturation_region() {
        let params = JfetParams::new()
            .with_vto(-2.0)
            .with_beta(1e-3)
            .with_lambda(0.0);
        let jfet = Jfet::njf("J1", 1, 2, 0).with_params(params);
        
        // Vgs = 0V, Vds = 5V → saturation (Vds > Vgs - Vto = 0 - (-2) = 2)
        let (ids, gm, gds) = jfet.calculate(0.0, 5.0, 300.0);
        
        // Ids = beta * (Vgs - Vto)² = 1e-3 * (0 - (-2))² = 1e-3 * 4 = 4mA
        assert!((ids - 4e-3).abs() < 1e-6, "Expected Ids=4mA, got {}", ids);
        
        // gm = 2 * beta * (Vgs - Vto) = 2 * 1e-3 * 2 = 4mS
        assert!((gm - 4e-3).abs() < 1e-6, "Expected gm=4mS, got {}", gm);
        
        // gds = 0 when lambda = 0
        assert!(gds.abs() < 1e-10, "gds should be ~0 with lambda=0");
    }

    #[test]
    fn test_linear_region() {
        let params = JfetParams::new()
            .with_vto(-2.0)
            .with_beta(1e-3)
            .with_lambda(0.0);
        let jfet = Jfet::njf("J1", 1, 2, 0).with_params(params);
        
        // Vgs = 0V, Vds = 1V → linear (Vds < Vgs - Vto = 2)
        let (ids, gm, gds) = jfet.calculate(0.0, 1.0, 300.0);
        
        // Ids = beta * (2*(Vgs-Vto)*Vds - Vds²) = 1e-3 * (2*2*1 - 1) = 1e-3 * 3 = 3mA
        assert!((ids - 3e-3).abs() < 1e-6, "Expected Ids=3mA, got {}", ids);
        
        // gm = 2 * beta * Vds = 2 * 1e-3 * 1 = 2mS
        assert!((gm - 2e-3).abs() < 1e-6, "Expected gm=2mS, got {}", gm);
        
        // gds = beta * 2 * (Vgst - Vds) = 1e-3 * 2 * (2 - 1) = 2mS
        assert!((gds - 2e-3).abs() < 1e-6, "Expected gds=2mS, got {}", gds);
    }

    #[test]
    fn test_channel_length_modulation() {
        let params = JfetParams::new()
            .with_vto(-2.0)
            .with_beta(1e-3)
            .with_lambda(0.01);
        let jfet = Jfet::njf("J1", 1, 2, 0).with_params(params);
        
        // Saturation with lambda > 0
        let (ids1, _, gds1) = jfet.calculate(0.0, 5.0, 300.0);
        let (ids2, _, _) = jfet.calculate(0.0, 10.0, 300.0);
        
        // Ids should increase with Vds due to lambda
        assert!(ids2 > ids1, "Ids should increase with Vds when lambda > 0");
        
        // gds = beta * Vgst² * lambda = 1e-3 * 4 * 0.01 = 40µS
        assert!((gds1 - 40e-6).abs() < 1e-9, "Expected gds=40µS, got {}", gds1);
    }

    #[test]
    fn test_pjf_polarity() {
        // P-JFET uses same VTO sign convention as N-JFET in the model
        // The polarity multiplier handles the sign transformation
        let params = JfetParams::new()
            .with_vto(-2.0)  // VTO=-2 (same as N-JFET)
            .with_beta(1e-3)
            .with_lambda(0.0);
        let jfet = Jfet::pjf("J1", 1, 2, 0).with_params(params);
        
        // P-JFET: Vgs = 0, Vds = -5V
        // Internal: vgs_int = -1*0 = 0, vds_int = -1*(-5) = 5
        // vgst = 0 - (-2) = 2 > 0, so device is ON
        // Saturation: vds_int=5 > vgst=2
        let (ids, _gm, _) = jfet.calculate(0.0, -5.0, 300.0);
        
        // Current flows opposite direction (negative Ids for P-JFET)
        // Internal ids = beta * vgst^2 = 1e-3 * 4 = 4mA
        // Output: pol * ids = -1 * 4mA = -4mA
        assert!(ids < 0.0, "P-JFET Ids should be negative, got {}", ids);
        assert!((ids.abs() - 4e-3).abs() < 1e-6, "Expected |Ids|=4mA, got {}", ids.abs());
    }

    #[test]
    fn test_idss_calculation() {
        let params = JfetParams::from_idss(10e-3, -2.0);  // 10mA IDSS
        let jfet = Jfet::njf("J1", 1, 2, 0).with_params(params);
        
        // IDSS = Ids at Vgs=0, saturation
        let (ids, _, _) = jfet.calculate(0.0, 10.0, 300.0);
        
        // Should be close to 10mA (exactly 10mA with lambda=0)
        assert!((ids - 10e-3).abs() < 1e-6, "Expected Ids≈IDSS=10mA, got {}", ids);
    }

    #[test]
    fn test_gate_current() {
        let params = JfetParams::new().with_junction(1e-14, 0.8);
        let jfet = Jfet::njf("J1", 1, 2, 0).with_params(params);
        
        // Reverse biased gate (normal operation)
        let (igs, igd) = jfet.gate_current(-1.0, -6.0, 300.0);
        
        // Should be very small (reverse saturation)
        assert!(igs.abs() < 1e-12, "Gate current should be tiny reverse biased");
        assert!(igd.abs() < 1e-12, "Gate-drain current should be tiny");
    }

    #[test]
    fn test_capacitances() {
        let params = JfetParams::new()
            .with_capacitances(5e-12, 2e-12);  // 5pF CGS, 2pF CGD
        let jfet = Jfet::njf("J1", 1, 2, 0).with_params(params);
        
        // Zero bias capacitances
        let (cgs, cgd) = jfet.capacitances(0.0, 0.0);
        
        assert!((cgs - 5e-12).abs() < 1e-15, "CGS at zero bias should be 5pF");
        assert!((cgd - 2e-12).abs() < 1e-15, "CGD at zero bias should be 2pF");
        
        // Reverse bias increases depletion width, decreases capacitance
        let (cgs_rev, _) = jfet.capacitances(-2.0, -5.0);
        assert!(cgs_rev < cgs, "CGS should decrease with reverse bias");
    }

    #[test]
    fn test_params_builder() {
        let params = JfetParams::new()
            .with_vto(-3.0)
            .with_beta(2e-3)
            .with_lambda(0.02)
            .with_capacitances(10e-12, 5e-12)
            .with_junction(1e-15, 0.9);
        
        assert_eq!(params.vto, -3.0);
        assert_eq!(params.beta, 2e-3);
        assert_eq!(params.lambda, 0.02);
        assert_eq!(params.cgs, 10e-12);
        assert_eq!(params.cgd, 5e-12);
        assert_eq!(params.is, 1e-15);
        assert_eq!(params.pb, 0.9);
    }
}
