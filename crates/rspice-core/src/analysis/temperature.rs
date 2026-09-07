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
//! Semiconductor temperature equations are owned by each device model.
//! This module supplies the shared temperature context and passive scaling.

use crate::Value;

//=============================================================================
// Constants
//=============================================================================

pub use crate::constants::{TEMP_REFERENCE as T_NOMINAL, thermal_voltage};

//=============================================================================
// Temperature Utilities
//=============================================================================

// `celsius_to_kelvin` and `kelvin_to_celsius` live in `crate::constants`.
// They sit on the parse/evaluate boundary and are called from the parser,
// the builder, the device models and the expression VM — layers that would
// otherwise have to reach up into an analysis module for arithmetic on a
// physical constant.
use crate::constants::{celsius_to_kelvin, kelvin_to_celsius};

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
