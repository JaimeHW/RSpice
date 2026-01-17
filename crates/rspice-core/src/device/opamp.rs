//! OpAmp Subcircuit Library
//!
//! Provides behavioral macromodels for common operational amplifiers.
//! These models capture the key characteristics without full transistor-level detail:
//! - Finite gain and bandwidth (GBW product)
//! - Input/output impedance
//! - Slew rate limiting
//! - Input bias current
//! - Common-mode rejection

use crate::{circuit::NodeId, Value};

//=============================================================================
// OpAmp Model Parameters
//=============================================================================

/// Standard OpAmp macromodel parameters
#[derive(Debug, Clone)]
pub struct OpAmpParams {
    /// Open-loop DC gain (A_OL) in V/V
    pub aol: Value,
    /// Gain-bandwidth product (GBW) in Hz
    pub gbw: Value,
    /// Slew rate in V/μs
    pub slew_rate: Value,
    /// Input resistance in Ohms
    pub rin: Value,
    /// Output resistance in Ohms
    pub rout: Value,
    /// Input bias current in A
    pub ibias: Value,
    /// Input offset voltage in V
    pub vos: Value,
    /// Common-mode rejection ratio in dB
    pub cmrr_db: Value,
    /// Power supply rejection ratio in dB
    pub psrr_db: Value,
    /// Positive output swing voltage
    pub vout_max: Value,
    /// Negative output swing voltage
    pub vout_min: Value,
}

impl Default for OpAmpParams {
    fn default() -> Self {
        // Generic OpAmp defaults (similar to TL071)
        Self {
            aol: 200_000.0,      // 106 dB
            gbw: 3e6,            // 3 MHz
            slew_rate: 13.0,     // 13 V/μs
            rin: 1e12,           // 1 TΩ (JFET input)
            rout: 75.0,          // 75Ω
            ibias: 30e-12,       // 30 pA
            vos: 3e-3,           // 3 mV typical
            cmrr_db: 100.0,      // 100 dB
            psrr_db: 100.0,      // 100 dB
            vout_max: 13.5,      // +13.5V with ±15V supplies
            vout_min: -13.5,     // -13.5V
        }
    }
}

impl OpAmpParams {
    /// Create LM741 parameters (classic BJT opamp)
    pub fn lm741() -> Self {
        Self {
            aol: 200_000.0,      // 106 dB
            gbw: 1e6,            // 1 MHz
            slew_rate: 0.5,      // 0.5 V/μs
            rin: 2e6,            // 2 MΩ (BJT input)
            rout: 75.0,          // 75Ω
            ibias: 80e-9,        // 80 nA
            vos: 2e-3,           // 2 mV typical
            cmrr_db: 90.0,       // 90 dB
            psrr_db: 90.0,       // 90 dB
            vout_max: 13.0,
            vout_min: -13.0,
        }
    }

    /// Create TL072 parameters (JFET input, dual opamp)
    pub fn tl072() -> Self {
        Self {
            aol: 200_000.0,      // 106 dB
            gbw: 3e6,            // 3 MHz
            slew_rate: 13.0,     // 13 V/μs
            rin: 1e12,           // 1 TΩ (JFET input)
            rout: 100.0,         // 100Ω
            ibias: 30e-12,       // 30 pA
            vos: 3e-3,           // 3 mV typical
            cmrr_db: 100.0,      // 100 dB
            psrr_db: 100.0,      // 100 dB
            vout_max: 13.5,
            vout_min: -13.5,
        }
    }

    /// Create LM324 parameters (single supply, quad opamp)
    pub fn lm324() -> Self {
        Self {
            aol: 100_000.0,      // 100 dB
            gbw: 1e6,            // 1 MHz
            slew_rate: 0.5,      // 0.5 V/μs
            rin: 2e6,            // 2 MΩ
            rout: 100.0,         // 100Ω
            ibias: 45e-9,        // 45 nA
            vos: 2e-3,           // 2 mV
            cmrr_db: 85.0,       // 85 dB
            psrr_db: 100.0,      // 100 dB
            vout_max: 3.5,       // Within 1.5V of Vcc
            vout_min: 0.005,     // Down to 5mV above Vee
        }
    }

    /// Create OP27 parameters (precision, low noise)
    pub fn op27() -> Self {
        Self {
            aol: 1_500_000.0,    // 124 dB
            gbw: 8e6,            // 8 MHz
            slew_rate: 2.8,      // 2.8 V/μs
            rin: 4e6,            // 4 MΩ
            rout: 70.0,          // 70Ω
            ibias: 15e-9,        // 15 nA
            vos: 30e-6,          // 30 μV (precision)
            cmrr_db: 126.0,      // 126 dB
            psrr_db: 120.0,      // 120 dB
            vout_max: 13.0,
            vout_min: -13.0,
        }
    }

    /// Create LM358 parameters (dual, single or split supply)
    pub fn lm358() -> Self {
        Self {
            aol: 100_000.0,      // 100 dB
            gbw: 1e6,            // 1 MHz
            slew_rate: 0.6,      // 0.6 V/μs
            rin: 2e6,            // 2 MΩ
            rout: 100.0,         // 100Ω
            ibias: 45e-9,        // 45 nA
            vos: 3e-3,           // 3 mV
            cmrr_db: 85.0,       // 85 dB
            psrr_db: 100.0,      // 100 dB
            vout_max: 13.5,
            vout_min: 0.0,       // Output goes to ground
        }
    }

    /// Create OPA2134 parameters (high-performance audio)
    pub fn opa2134() -> Self {
        Self {
            aol: 1_000_000.0,    // 120 dB
            gbw: 8e6,            // 8 MHz
            slew_rate: 20.0,     // 20 V/μs
            rin: 1e13,           // 10 TΩ (FET input)
            rout: 50.0,          // 50Ω
            ibias: 5e-12,        // 5 pA
            vos: 500e-6,         // 0.5 mV
            cmrr_db: 110.0,      // 110 dB
            psrr_db: 110.0,      // 110 dB
            vout_max: 13.5,
            vout_min: -13.5,
        }
    }

    /// Create NE5532 parameters (low-noise audio)
    pub fn ne5532() -> Self {
        Self {
            aol: 100_000.0,      // 100 dB
            gbw: 10e6,           // 10 MHz
            slew_rate: 9.0,      // 9 V/μs
            rin: 300e3,          // 300 kΩ (BJT input)
            rout: 30.0,          // 30Ω
            ibias: 200e-9,       // 200 nA
            vos: 500e-6,         // 0.5 mV
            cmrr_db: 100.0,      // 100 dB
            psrr_db: 100.0,      // 100 dB
            vout_max: 13.0,
            vout_min: -13.0,
        }
    }
}

//=============================================================================
// OpAmp Macromodel (behavioral)
//=============================================================================

/// Behavioral OpAmp macromodel for simulation
///
/// Uses a simplified model with:
/// - Single-pole frequency response (GBW product)
/// - Output limiting
/// - Input resistance
/// 
/// The transfer function is: Vout = A * (V+ - V-) / (1 + s/ω_p)
/// where A is DC gain and ω_p = GBW/A is the dominant pole.
#[derive(Debug, Clone)]
pub struct OpAmpMacromodel {
    pub name: String,
    pub params: OpAmpParams,
    
    // Node connections
    pub node_out: NodeId,
    pub node_inp: NodeId,   // non-inverting (+)
    pub node_inm: NodeId,   // inverting (-)
    pub node_vcc: Option<NodeId>,  // positive supply
    pub node_vee: Option<NodeId>,  // negative supply
    
    // State for slew rate limiting
    vout_prev: Value,
    time_prev: Value,
    
    // Calculated values
    dominant_pole: Value,  // ω_p in rad/s
}

impl OpAmpMacromodel {
    /// Create new opamp with given parameters
    pub fn new(name: String, params: OpAmpParams, out: NodeId, inp: NodeId, inm: NodeId) -> Self {
        let dominant_pole = 2.0 * std::f64::consts::PI * params.gbw / params.aol;
        Self {
            name,
            params,
            node_out: out,
            node_inp: inp,
            node_inm: inm,
            node_vcc: None,
            node_vee: None,
            vout_prev: 0.0,
            time_prev: 0.0,
            dominant_pole,
        }
    }

    /// Create LM741 opamp
    pub fn lm741(name: String, out: NodeId, inp: NodeId, inm: NodeId) -> Self {
        Self::new(name, OpAmpParams::lm741(), out, inp, inm)
    }

    /// Create TL072 opamp
    pub fn tl072(name: String, out: NodeId, inp: NodeId, inm: NodeId) -> Self {
        Self::new(name, OpAmpParams::tl072(), out, inp, inm)
    }

    /// Create LM324 opamp
    pub fn lm324(name: String, out: NodeId, inp: NodeId, inm: NodeId) -> Self {
        Self::new(name, OpAmpParams::lm324(), out, inp, inm)
    }

    /// Create OP27 precision opamp
    pub fn op27(name: String, out: NodeId, inp: NodeId, inm: NodeId) -> Self {
        Self::new(name, OpAmpParams::op27(), out, inp, inm)
    }

    /// Add power supply nodes
    pub fn with_supplies(mut self, vcc: NodeId, vee: NodeId) -> Self {
        self.node_vcc = Some(vcc);
        self.node_vee = Some(vee);
        self
    }

    /// Calculate ideal output voltage (before limiting)
    pub fn calculate_vout(&self, vp: Value, vm: Value) -> Value {
        let vdiff = vp - vm + self.params.vos;
        let vout_ideal = self.params.aol * vdiff;
        
        // Output limiting
        vout_ideal.clamp(self.params.vout_min, self.params.vout_max)
    }

    /// Calculate output with slew rate limiting
    pub fn calculate_vout_slewed(&mut self, vp: Value, vm: Value, dt: Value) -> Value {
        let vout_ideal = self.calculate_vout(vp, vm);
        
        // Slew rate limiting
        if dt > 0.0 {
            let slew_rate_vs = self.params.slew_rate * 1e6; // Convert to V/s
            let max_change = slew_rate_vs * dt;
            let delta = vout_ideal - self.vout_prev;
            
            let vout_slewed = if delta.abs() > max_change {
                self.vout_prev + max_change * delta.signum()
            } else {
                vout_ideal
            };
            
            self.vout_prev = vout_slewed;
            vout_slewed
        } else {
            vout_ideal
        }
    }

    /// Get equivalent input conductance
    pub fn input_conductance(&self) -> Value {
        1.0 / self.params.rin
    }

    /// Get open-loop output conductance
    pub fn output_conductance(&self) -> Value {
        1.0 / self.params.rout
    }

    /// Get dominant pole frequency in Hz
    pub fn pole_frequency(&self) -> Value {
        self.dominant_pole / (2.0 * std::f64::consts::PI)
    }

    /// Get frequency response gain at given frequency
    pub fn gain_at_frequency(&self, freq_hz: Value) -> Value {
        let omega = 2.0 * std::f64::consts::PI * freq_hz;
        let aol = self.params.aol;
        aol / (1.0 + (omega / self.dominant_pole).powi(2)).sqrt()
    }

    /// Get -3dB bandwidth
    pub fn bandwidth_3db(&self) -> Value {
        self.pole_frequency()
    }
}

//=============================================================================
// SPICE Subcircuit Generation
//=============================================================================

/// Generate SPICE subcircuit definition for an opamp
/// 
/// This creates a behavioral macromodel that can be used in netlists.
/// The subcircuit uses a VCVS (E source) for gain and RC for frequency response.
pub fn generate_spice_subcircuit(name: &str, params: &OpAmpParams) -> String {
    let tau = params.aol / (2.0 * std::f64::consts::PI * params.gbw);
    
    format!(
        r#"* {} OpAmp Macromodel
* A_OL = {:.0} V/V ({:.1} dB)
* GBW = {:.2e} Hz
* Slew Rate = {:.1} V/us
.SUBCKT {} INP INM OUT VCC VEE
* Input stage
RIN INP INM {:.2e}
* Input offset
VOS INP 1 {:.2e}
* Gain stage (single pole)
E1 2 0 1 INM {:.0}
R1 2 3 1k
C1 3 0 {:.6e}
* Output buffer
EOUT 4 0 3 0 1
ROUT 4 OUT {:.1}
* Output limiting diodes
DPOS OUT VCC DCLAMP
DNEG VEE OUT DCLAMP
.MODEL DCLAMP D(IS=1e-15 BV=0.7)
.ENDS {}
"#,
        name,
        params.aol,
        20.0 * params.aol.log10(),
        params.gbw,
        params.slew_rate,
        name,
        params.rin,
        params.vos,
        params.aol,
        tau / 1e3, // C = tau / R, with R = 1k
        params.rout,
        name
    )
}

//=============================================================================
// Common OpAmp Library
//=============================================================================

/// Get a named opamp from the library
pub fn get_opamp_params(name: &str) -> Option<OpAmpParams> {
    match name.to_uppercase().as_str() {
        "LM741" | "UA741" | "741" => Some(OpAmpParams::lm741()),
        "TL072" | "TL071" | "TL074" => Some(OpAmpParams::tl072()),
        "LM324" => Some(OpAmpParams::lm324()),
        "LM358" => Some(OpAmpParams::lm358()),
        "OP27" | "OP37" => Some(OpAmpParams::op27()),
        "OPA2134" | "OPA134" => Some(OpAmpParams::opa2134()),
        "NE5532" | "SA5532" => Some(OpAmpParams::ne5532()),
        _ => None,
    }
}

/// List available opamp models
pub fn list_available_opamps() -> Vec<&'static str> {
    vec![
        "LM741",
        "TL072",
        "TL074",
        "LM324",
        "LM358",
        "OP27",
        "OPA2134",
        "NE5532",
    ]
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lm741_params() {
        let params = OpAmpParams::lm741();
        assert_eq!(params.gbw, 1e6);
        assert!(params.slew_rate < 1.0); // 0.5 V/us
    }

    #[test]
    fn test_tl072_params() {
        let params = OpAmpParams::tl072();
        assert_eq!(params.gbw, 3e6);
        assert!(params.rin > 1e11); // High-Z JFET input
    }

    #[test]
    fn test_opamp_macromodel() {
        let opamp = OpAmpMacromodel::lm741("U1".to_string(), 3, 1, 2);
        
        // Test gain
        let vout = opamp.calculate_vout(0.0001, 0.0); // 100uV differential
        assert!(vout > 0.0);
        
        // Test limiting
        let vout_big = opamp.calculate_vout(1.0, 0.0);
        assert!(vout_big <= opamp.params.vout_max);
    }

    #[test]
    fn test_frequency_response() {
        let opamp = OpAmpMacromodel::tl072("U1".to_string(), 3, 1, 2);
        
        // At DC
        let gain_dc = opamp.gain_at_frequency(0.0);
        assert!((gain_dc - opamp.params.aol).abs() / opamp.params.aol < 0.01);
        
        // At GBW, gain should be close to 1
        let gain_gbw = opamp.gain_at_frequency(opamp.params.gbw);
        assert!(gain_gbw < 10.0); // Should be approximately 1
    }

    #[test]
    fn test_get_opamp_params() {
        assert!(get_opamp_params("LM741").is_some());
        assert!(get_opamp_params("TL072").is_some());
        assert!(get_opamp_params("OP27").is_some());
        assert!(get_opamp_params("UNKNOWN").is_none());
    }

    #[test]
    fn test_spice_subcircuit() {
        let params = OpAmpParams::lm741();
        let subckt = generate_spice_subcircuit("LM741", &params);
        
        assert!(subckt.contains(".SUBCKT LM741"));
        assert!(subckt.contains(".ENDS LM741"));
        assert!(subckt.contains("RIN"));
    }
}
