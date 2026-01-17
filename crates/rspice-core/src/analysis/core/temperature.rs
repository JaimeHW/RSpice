//! Temperature Simulation
//!
//! Implements temperature-dependent simulation parameters and device scaling.
//!
//! # SPICE Syntax
//! ```text
//! .TEMP <temperature>        ; Set simulation temperature
//! .TEMP 27 50 100            ; Multi-temperature run  
//! .OPTIONS TNOM=27           ; Nominal reference temperature
//! ```
//!
//! # Temperature Dependence
//! Many device parameters are temperature-dependent:
//!
//! ## Resistors
//! R(T) = R(Tnom) * (1 + TC1*(T-Tnom) + TC2*(T-Tnom)^2)
//!
//! ## Diodes/BJTs  
//! - Saturation current: Is(T) = Is(Tnom) * (T/Tnom)^(XTI/N) * exp(-Eg/((N*k*T) - Eg/(N*k*Tnom)))
//! - Thermal voltage: Vt = k*T/q
//!
//! ## MOSFETs
//! - Threshold voltage shift
//! - Mobility degradation

use crate::Value;

//=============================================================================
// Constants
//=============================================================================

/// Boltzmann constant (J/K)
pub const K_BOLTZMANN: Value = 1.380649e-23;
/// Electron charge (C)
pub const Q_ELECTRON: Value = 1.602176634e-19;
/// Default nominal temperature (K) = 27°C
pub const T_NOMINAL: Value = 300.15;
/// Absolute zero offset (°C to K)
pub const KELVIN_OFFSET: Value = 273.15;
/// Silicon bandgap at 300K (eV)
pub const EG_SILICON: Value = 1.12;
/// Silicon bandgap temperature coefficient
pub const EG_ALPHA: Value = 7.02e-4;
/// Silicon bandgap temperature reference
pub const EG_BETA: Value = 1108.0;

//=============================================================================
// Temperature Utilities
//=============================================================================

/// Convert Celsius to Kelvin
#[inline]
pub fn celsius_to_kelvin(celsius: Value) -> Value {
    celsius + KELVIN_OFFSET
}

/// Convert Kelvin to Celsius
#[inline]
pub fn kelvin_to_celsius(kelvin: Value) -> Value {
    kelvin - KELVIN_OFFSET
}

/// Calculate thermal voltage Vt = kT/q
#[inline]
pub fn thermal_voltage(temperature_k: Value) -> Value {
    K_BOLTZMANN * temperature_k / Q_ELECTRON
}

/// Calculate silicon bandgap as function of temperature (eV)
/// Eg(T) = Eg(0) - α*T²/(T+β)
pub fn bandgap_silicon(temperature_k: Value) -> Value {
    // Eg(0) ≈ 1.17 eV for silicon
    let eg0 = 1.17;
    eg0 - EG_ALPHA * temperature_k * temperature_k / (temperature_k + EG_BETA)
}

//=============================================================================
// Temperature Context
//=============================================================================

/// Temperature simulation context
#[derive(Debug, Clone)]
pub struct TemperatureContext {
    /// Current simulation temperature (K)
    pub temperature: Value,
    /// Nominal/reference temperature (K)
    pub tnom: Value,
    /// Temperature ratio (T/Tnom)
    pub ratio: Value,
    /// Temperature difference (T - Tnom)
    pub delta_t: Value,
    /// Thermal voltage at current temperature
    pub vt: Value,
    /// Thermal voltage at nominal temperature
    pub vt_nom: Value,
}

impl TemperatureContext {
    /// Create a new temperature context
    pub fn new(temperature_k: Value) -> Self {
        let tnom = T_NOMINAL;
        Self::with_tnom(temperature_k, tnom)
    }

    /// Create with custom nominal temperature
    pub fn with_tnom(temperature_k: Value, tnom_k: Value) -> Self {
        Self {
            temperature: temperature_k,
            tnom: tnom_k,
            ratio: temperature_k / tnom_k,
            delta_t: temperature_k - tnom_k,
            vt: thermal_voltage(temperature_k),
            vt_nom: thermal_voltage(tnom_k),
        }
    }

    /// Create from Celsius temperatures
    pub fn from_celsius(temperature_c: Value, tnom_c: Value) -> Self {
        Self::with_tnom(celsius_to_kelvin(temperature_c), celsius_to_kelvin(tnom_c))
    }

    /// Get temperature in Celsius
    pub fn celsius(&self) -> Value {
        kelvin_to_celsius(self.temperature)
    }

    /// Get nominal temperature in Celsius
    pub fn tnom_celsius(&self) -> Value {
        kelvin_to_celsius(self.tnom)
    }
}

impl Default for TemperatureContext {
    fn default() -> Self {
        Self::new(T_NOMINAL)
    }
}

//=============================================================================
// Temperature Coefficients
//=============================================================================

/// Temperature coefficients for resistors
#[derive(Debug, Clone, Copy, Default)]
pub struct ResistorTempCoeffs {
    /// First-order coefficient (1/°C)
    pub tc1: Value,
    /// Second-order coefficient (1/°C²)
    pub tc2: Value,
}

impl ResistorTempCoeffs {
    /// Create new coefficients
    pub fn new(tc1: Value, tc2: Value) -> Self {
        Self { tc1, tc2 }
    }

    /// Calculate resistance at temperature
    /// R(T) = R(Tnom) * (1 + TC1*(T-Tnom) + TC2*(T-Tnom)^2)
    pub fn scale_resistance(&self, r_nom: Value, temp: &TemperatureContext) -> Value {
        let dt = temp.delta_t;
        r_nom * (1.0 + self.tc1 * dt + self.tc2 * dt * dt)
    }
}

/// Temperature coefficients for capacitors
#[derive(Debug, Clone, Copy, Default)]
pub struct CapacitorTempCoeffs {
    /// First-order voltage coefficient
    pub vc1: Value,
    /// Second-order voltage coefficient
    pub vc2: Value,
    /// First-order temperature coefficient
    pub tc1: Value,
    /// Second-order temperature coefficient
    pub tc2: Value,
}

impl CapacitorTempCoeffs {
    /// Calculate capacitance at temperature
    pub fn scale_capacitance(&self, c_nom: Value, temp: &TemperatureContext) -> Value {
        let dt = temp.delta_t;
        c_nom * (1.0 + self.tc1 * dt + self.tc2 * dt * dt)
    }
}

//=============================================================================
// Semiconductor Temperature Scaling
//=============================================================================

/// Temperature scaling for PN junctions (diodes, BJT junctions)
#[derive(Debug, Clone, Copy)]
pub struct JunctionTempScaling {
    /// Emission coefficient (N)
    pub n: Value,
    /// Temperature exponent for Is (XTI, typically 3 for diodes)
    pub xti: Value,
    /// Bandgap energy at nominal temperature (eV)
    pub eg: Value,
}

impl Default for JunctionTempScaling {
    fn default() -> Self {
        Self {
            n: 1.0,
            xti: 3.0,
            eg: EG_SILICON,
        }
    }
}

impl JunctionTempScaling {
    /// Scale saturation current with temperature
    /// Is(T) = Is(Tnom) * (T/Tnom)^(XTI/N) * exp(Eg/(N*Vt_nom) - Eg/(N*Vt))
    pub fn scale_is(&self, is_nom: Value, temp: &TemperatureContext) -> Value {
        let vt_ratio = temp.vt_nom / temp.vt;
        let t_factor = temp.ratio.powf(self.xti / self.n);
        let eg_factor = (self.eg / (self.n * temp.vt_nom) * (1.0 - vt_ratio)).exp();
        
        is_nom * t_factor * eg_factor
    }

    /// Scale junction capacitance with temperature (via built-in potential)
    /// VJ(T) = VJ(Tnom) * T/Tnom - 3*Vt*ln(T/Tnom) - Eg(Tnom)*(T/Tnom - 1)
    pub fn scale_vj(&self, vj_nom: Value, temp: &TemperatureContext) -> Value {
        let eg_tnom = self.eg;
        let term1 = vj_nom * temp.ratio;
        let term2 = 3.0 * temp.vt * temp.ratio.ln();
        let term3 = eg_tnom * (temp.ratio - 1.0);
        
        term1 - term2 - term3
    }
}

/// Temperature scaling for MOSFETs
#[derive(Debug, Clone, Copy)]
pub struct MosfetTempScaling {
    /// Threshold voltage temperature coefficient (V/°C, typically -2mV/°C)
    pub kt1: Value,
    /// Threshold voltage temperature coefficient 2
    pub kt1l: Value,
    /// Mobility temperature exponent (typically 1.5-2.0)
    pub ute: Value,
    /// Saturation velocity temperature coefficient
    pub at: Value,
}

impl Default for MosfetTempScaling {
    fn default() -> Self {
        Self {
            kt1: -0.002,  // -2mV/°C typical for silicon
            kt1l: 0.0,
            ute: -1.5,    // Mobility ~ T^-1.5
            at: 3.3e4,
        }
    }
}

impl MosfetTempScaling {
    /// Scale threshold voltage with temperature
    pub fn scale_vth(&self, vth_nom: Value, temp: &TemperatureContext) -> Value {
        vth_nom + self.kt1 * temp.delta_t
    }

    /// Scale mobility with temperature
    /// μ(T) = μ(Tnom) * (T/Tnom)^UTE
    pub fn scale_mobility(&self, mu_nom: Value, temp: &TemperatureContext) -> Value {
        mu_nom * temp.ratio.powf(self.ute)
    }
}

//=============================================================================
// Tests
//=============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_celsius_kelvin_conversion() {
        let c = 27.0;
        let k = celsius_to_kelvin(c);
        assert!((k - 300.15).abs() < 0.01);
        
        let c2 = kelvin_to_celsius(k);
        assert!((c2 - c).abs() < 0.01);
    }

    #[test]
    fn test_thermal_voltage() {
        // At 300K, Vt ≈ 25.9mV
        let vt = thermal_voltage(300.0);
        assert!((vt - 0.0259).abs() < 0.001);
    }

    #[test]
    fn test_temperature_context() {
        let ctx = TemperatureContext::from_celsius(27.0, 27.0);
        
        // At Tnom, ratio should be 1
        assert!((ctx.ratio - 1.0).abs() < 0.01);
        assert!((ctx.delta_t).abs() < 1.0);
    }

    #[test]
    fn test_temperature_elevated() {
        let ctx = TemperatureContext::from_celsius(100.0, 27.0);
        
        // At 100°C, ratio > 1
        assert!(ctx.ratio > 1.0);
        assert!(ctx.delta_t > 0.0);
        
        // Thermal voltage should be higher
        assert!(ctx.vt > ctx.vt_nom);
    }

    #[test]
    fn test_resistor_temp_coeffs() {
        let coeffs = ResistorTempCoeffs::new(0.001, 0.0);  // 1000 ppm/°C
        let ctx = TemperatureContext::from_celsius(127.0, 27.0);  // 100°C rise
        
        let r_scaled = coeffs.scale_resistance(1000.0, &ctx);
        
        // R should increase by ~10% (100°C * 0.001/°C)
        assert!((r_scaled - 1100.0).abs() < 10.0);
    }

    #[test]
    fn test_junction_is_scaling() {
        let scaling = JunctionTempScaling::default();
        let ctx = TemperatureContext::from_celsius(85.0, 27.0);
        
        let is_scaled = scaling.scale_is(1e-14, &ctx);
        
        // Is should increase significantly with temperature
        assert!(is_scaled > 1e-14);
        assert!(is_scaled > 1e-12);  // Usually >100x increase
    }

    #[test]
    fn test_mosfet_vth_scaling() {
        let scaling = MosfetTempScaling::default();
        let ctx = TemperatureContext::from_celsius(85.0, 27.0);
        
        let vth_scaled = scaling.scale_vth(0.7, &ctx);
        
        // Vth should decrease with temperature (negative coefficient)
        assert!(vth_scaled < 0.7);
        // About -2mV * 58°C ≈ -116mV
        assert!((vth_scaled - (0.7 - 0.116)).abs() < 0.02);
    }

    #[test]
    fn test_mosfet_mobility_scaling() {
        let scaling = MosfetTempScaling::default();
        let ctx = TemperatureContext::from_celsius(85.0, 27.0);
        
        let mu_scaled = scaling.scale_mobility(500.0, &ctx);
        
        // Mobility should decrease with temperature
        assert!(mu_scaled < 500.0);
    }

    #[test]
    fn test_bandgap_silicon() {
        let eg_300 = bandgap_silicon(300.0);
        let eg_400 = bandgap_silicon(400.0);
        
        // Bandgap should be ~1.12 eV at 300K
        assert!((eg_300 - 1.12).abs() < 0.05);
        
        // Bandgap decreases with temperature
        assert!(eg_400 < eg_300);
    }
}
