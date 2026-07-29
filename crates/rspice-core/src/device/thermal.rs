//! Self-Heating Thermal Network Model
//!
//! Implements thermal self-heating effects for power devices like:
//! - Power MOSFETs (VDMOS, LDMOS)
//! - Power BJTs and IGBTs
//! - High-power diodes
//!
//! # Theory
//!
//! Self-heating couples electrical and thermal domains:
//!
//! ```text
//!   P = V · I                    (power dissipation)
//!   dTj/dt = (P - (Tj-Ta)/Rth) / Cth   (thermal dynamics)
//!   
//! where:
//!   - Tj = junction temperature
//!   - Ta = ambient temperature
//!   - Rth = thermal resistance (junction to ambient)
//!   - Cth = thermal capacitance
//! ```
//!
//! # RC Thermal Network (Foster/Cauer)
//!
//! Multi-pole thermal models use a network of RC elements:
//!
//! ```text
//!   Foster:  P --> (R1||C1) --> (R2||C2) --> ... --> Ta
//!   Cauer:   P --> R1 --> C1 --> R2 --> C2 --> ... --> Ta
//! ```
//!
//! Foster models are behaviorally fitted, while Cauer models
//! represent physical layers (die, solder, package, heatsink).
//!
//! # Usage in Simulation
//!
//! 1. Calculate power dissipation from device currents/voltages
//! 2. Solve thermal network for junction temperature
//! 3. Update device parameters based on temperature
//! 4. Iterate until self-consistent solution

use crate::Value;

//=============================================================================
// Constants
//=============================================================================

/// Reference temperature: 27°C = 300.15K (SPICE convention, ngspice REFTEMP)
pub const TREF: Value = 300.15;
pub use crate::constants::KELVIN_OFFSET;
/// Default thermal resistance (°C/W) for discrete device
pub const RTH_DEFAULT: Value = 100.0;
/// Default thermal capacitance (J/°C)
pub const CTH_DEFAULT: Value = 1e-3;
/// Default ambient temperature (°C)
pub const TAMB_DEFAULT: Value = 27.0;
/// Maximum temperature iterations for self-consistency
pub const THERMAL_MAX_ITERS: usize = 20;
/// Temperature convergence tolerance (°C)
pub const THERMAL_TOLERANCE: Value = 0.1;

//=============================================================================
// Thermal Network Types
//=============================================================================

/// Type of thermal network model
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ThermalNetworkType {
    /// Single RC pole (simple lumped model)
    #[default]
    SinglePole,
    /// Foster network (parallel RC sections in series)
    Foster,
    /// Cauer network (series R with grounded C, physically meaningful)
    Cauer,
    /// No thermal modeling (isothermal)
    Isothermal,
}

//=============================================================================
// Thermal RC Element
//=============================================================================

/// Single thermal RC element in the network
#[derive(Debug, Clone)]
pub struct ThermalRcElement {
    /// Thermal resistance (°C/W or K/W)
    pub rth: Value,
    /// Thermal capacitance (J/°C or J/K)
    pub cth: Value,
    /// Current temperature at this node (°C or K)
    pub temperature: Value,
    /// Previous temperature (for transient)
    pub temperature_prev: Value,
}

impl ThermalRcElement {
    /// Create a new thermal RC element
    pub fn new(rth: Value, cth: Value) -> Self {
        Self {
            rth: rth.max(1e-6), // Prevent division by zero
            cth: cth.max(1e-12),
            temperature: TAMB_DEFAULT,
            temperature_prev: TAMB_DEFAULT,
        }
    }

    /// Single-pole DC response: ΔT = P · Rth
    #[inline]
    pub fn dc_temperature_rise(&self, power: Value) -> Value {
        power * self.rth
    }

    /// Thermal time constant τ = Rth × Cth
    #[inline]
    pub fn time_constant(&self) -> Value {
        self.rth * self.cth
    }

    /// Update temperature for transient analysis using backward Euler
    ///
    /// dT/dt = (P - (T - T_in)) / (Rth × Cth)
    ///
    /// Returns new temperature after timestep dt
    pub fn transient_update(&mut self, power: Value, t_in: Value, dt: Value) -> Value {
        let tau = self.time_constant();

        if tau < 1e-15 {
            // Very fast thermal, treat as instantaneous
            self.temperature = t_in + power * self.rth;
            return self.temperature;
        }

        // Backward Euler: T_n+1 = T_n + dt * dT/dt_n+1
        // (T_n+1 - T_n) / dt = (P * Rth - (T_n+1 - T_in)) / tau
        // T_n+1 * (1 + dt/tau) = T_n + dt * P * Rth / tau + dt * T_in / tau
        // T_n+1 = (T_n + dt * (P * Rth + T_in) / tau) / (1 + dt/tau)

        let alpha = dt / tau;
        self.temperature =
            (self.temperature_prev + alpha * (power * self.rth + t_in)) / (1.0 + alpha);

        self.temperature
    }

    /// Accept current temperature as previous for next step
    pub fn accept_step(&mut self) {
        self.temperature_prev = self.temperature;
    }

    /// Reset to ambient temperature
    pub fn reset(&mut self, t_amb: Value) {
        self.temperature = t_amb;
        self.temperature_prev = t_amb;
    }
}

impl Default for ThermalRcElement {
    fn default() -> Self {
        Self::new(RTH_DEFAULT, CTH_DEFAULT)
    }
}

//=============================================================================
// Thermal Network
//=============================================================================

/// Multi-pole thermal network for self-heating
#[derive(Debug, Clone)]
pub struct ThermalNetwork {
    /// Network type (Foster or Cauer)
    pub network_type: ThermalNetworkType,

    /// Thermal RC elements (from junction to ambient)
    pub elements: Vec<ThermalRcElement>,

    /// Ambient (case/heatsink) temperature (°C)
    pub t_ambient: Value,

    /// Current junction temperature (°C)
    pub t_junction: Value,

    /// Total thermal resistance (sum of all elements)
    pub rth_total: Value,

    /// Current power dissipation (W)
    pub power: Value,

    /// Whether self-heating is enabled
    pub enabled: bool,
}

impl ThermalNetwork {
    /// Create a single-pole thermal network
    pub fn single_pole(rth: Value, cth: Value) -> Self {
        let element = ThermalRcElement::new(rth, cth);
        Self {
            network_type: ThermalNetworkType::SinglePole,
            elements: vec![element],
            t_ambient: TAMB_DEFAULT,
            t_junction: TAMB_DEFAULT,
            rth_total: rth,
            power: 0.0,
            enabled: true,
        }
    }



    /// Create isothermal (no self-heating) model
    pub fn isothermal(temperature: Value) -> Self {
        Self {
            network_type: ThermalNetworkType::Isothermal,
            elements: vec![],
            t_ambient: temperature,
            t_junction: temperature,
            rth_total: 0.0,
            power: 0.0,
            enabled: false,
        }
    }




    /// Get junction temperature in Kelvin
    #[inline]
    pub fn junction_temperature_kelvin(&self) -> Value {
        self.t_junction + KELVIN_OFFSET
    }



    /// Accept current state for next transient step
    pub fn accept_step(&mut self) {
        for elem in &mut self.elements {
            elem.accept_step();
        }
    }

    /// Reset thermal network to ambient
    pub fn reset(&mut self) {
        self.t_junction = self.t_ambient;
        self.power = 0.0;
        for elem in &mut self.elements {
            elem.reset(self.t_ambient);
        }
    }

    /// Calculate temperature-dependent mobility factor
    ///
    /// μ(T) = μ(T_ref) × (T/T_ref)^(-BEX)
    /// where BEX ≈ 1.5-2.5 for silicon
    #[inline]
    pub fn mobility_factor(&self, bex: Value) -> Value {
        let t_k = self.junction_temperature_kelvin();
        (t_k / TREF).powf(-bex)
    }

    /// Calculate temperature-dependent threshold voltage shift
    ///
    /// ΔVth = KT1 × (T - T_ref) + KT2 × (T - T_ref)²
    #[inline]
    pub fn vth_shift(&self, kt1: Value, kt2: Value) -> Value {
        let dt = self.t_junction - (TREF - KELVIN_OFFSET);
        kt1 * dt + kt2 * dt * dt
    }

    /// Calculate temperature-dependent saturation current scaling
    ///
    /// Is(T) = Is(T_ref) × (T/T_ref)^XTI × exp(Eg/k × (1/T_ref - 1/T))
    #[inline]
    pub fn is_factor(&self, xti: Value, eg: Value) -> Value {
        let t_k = self.junction_temperature_kelvin();
        let k_b = 8.617e-5; // Boltzmann constant in eV/K

        let temp_ratio = (t_k / TREF).powf(xti);
        let exp_factor = (eg / k_b * (1.0 / TREF - 1.0 / t_k)).exp();

        temp_ratio * exp_factor
    }
}

impl Default for ThermalNetwork {
    fn default() -> Self {
        Self::single_pole(RTH_DEFAULT, CTH_DEFAULT)
    }
}

//=============================================================================
// Temperature Coefficients
//=============================================================================

/// Standard temperature coefficients for device parameters
#[derive(Debug, Clone)]
pub struct TemperatureCoefficients {
    /// Mobility temperature exponent (typically 1.5-2.5)
    pub bex: Value,
    /// Threshold voltage linear TC (V/°C)
    pub kt1: Value,
    /// Threshold voltage quadratic TC (V/°C²)
    pub kt2: Value,
    /// Saturation current temperature exponent
    pub xti: Value,
    /// Energy gap (eV)
    pub eg: Value,
    /// Series resistance TC (1/°C)
    pub trs: Value,
}

impl Default for TemperatureCoefficients {
    fn default() -> Self {
        Self {
            bex: 1.5,   // Silicon mobility exponent
            kt1: -2e-3, // Typical MOSFET Vth TC
            kt2: 0.0,
            xti: 3.0, // Diode/BJT Is exponent
            eg: 1.12, // Silicon bandgap
            trs: 0.0, // No resistance TC by default
        }
    }
}

impl TemperatureCoefficients {
    /// Create for MOSFET
    pub fn mosfet() -> Self {
        Self {
            bex: 2.0,
            kt1: -2.5e-3,
            kt2: 0.0,
            xti: 0.0,
            eg: 1.12,
            trs: 4e-3, // Positive TC for on-resistance
        }
    }

    /// Create for BJT
    pub fn bjt() -> Self {
        Self {
            bex: 1.5,
            kt1: 0.0,
            kt2: 0.0,
            xti: 3.0,
            eg: 1.12,
            trs: 0.0,
        }
    }

    /// Create for diode
    pub fn diode() -> Self {
        Self {
            bex: 0.0,
            kt1: 0.0,
            kt2: 0.0,
            xti: 3.0,
            eg: 1.12,
            trs: 0.0,
        }
    }
}

//=============================================================================
// Tests
//=============================================================================
