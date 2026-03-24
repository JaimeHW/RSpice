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

/// Reference temperature (300K = 27°C)
pub const TREF: Value = 300.0;
/// Absolute zero offset (Kelvin to Celsius)
pub const KELVIN_OFFSET: Value = 273.15;
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
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[derive(Default)]
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

    /// Create a multi-pole Foster network
    ///
    /// Foster network: Each (R, C) pair is a parallel RC connected in series
    /// Input pairs are (Rth, Cth) tuples from junction toward ambient
    pub fn foster(rc_pairs: &[(Value, Value)]) -> Self {
        let elements: Vec<ThermalRcElement> = rc_pairs
            .iter()
            .map(|&(r, c)| ThermalRcElement::new(r, c))
            .collect();

        let rth_total: Value = elements.iter().map(|e| e.rth).sum();

        Self {
            network_type: ThermalNetworkType::Foster,
            elements,
            t_ambient: TAMB_DEFAULT,
            t_junction: TAMB_DEFAULT,
            rth_total,
            power: 0.0,
            enabled: true,
        }
    }

    /// Create a multi-pole Cauer (ladder) network
    ///
    /// Cauer network: Series R followed by grounded C (physically meaningful)
    pub fn cauer(rc_pairs: &[(Value, Value)]) -> Self {
        let elements: Vec<ThermalRcElement> = rc_pairs
            .iter()
            .map(|&(r, c)| ThermalRcElement::new(r, c))
            .collect();

        let rth_total: Value = elements.iter().map(|e| e.rth).sum();

        Self {
            network_type: ThermalNetworkType::Cauer,
            elements,
            t_ambient: TAMB_DEFAULT,
            t_junction: TAMB_DEFAULT,
            rth_total,
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

    /// Set ambient temperature
    pub fn with_ambient(mut self, t_amb: Value) -> Self {
        self.t_ambient = t_amb;
        self.t_junction = t_amb;
        for elem in &mut self.elements {
            elem.reset(t_amb);
        }
        self
    }

    /// Enable or disable self-heating
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }

    /// Get current junction temperature
    #[inline]
    pub fn junction_temperature(&self) -> Value {
        self.t_junction
    }

    /// Get junction temperature in Kelvin
    #[inline]
    pub fn junction_temperature_kelvin(&self) -> Value {
        self.t_junction + KELVIN_OFFSET
    }

    /// Update junction temperature for DC analysis
    ///
    /// Uses steady-state thermal model: Tj = Ta + P × Rth_total
    pub fn update_dc(&mut self, power: Value) {
        self.power = power;

        if !self.enabled || self.network_type == ThermalNetworkType::Isothermal {
            return;
        }

        self.t_junction = self.t_ambient + power * self.rth_total;

        // Update internal nodes (for monitoring)
        match self.network_type {
            ThermalNetworkType::Foster => {
                // Foster: Parallel RC sections, each sees full power
                for elem in &mut self.elements {
                    elem.temperature = self.t_ambient + elem.dc_temperature_rise(power);
                }
            }
            ThermalNetworkType::Cauer => {
                // Cauer: Series ladder, temperature drops across each R
                let mut t_in = self.t_junction;
                for elem in self.elements.iter_mut().rev() {
                    let dt = elem.dc_temperature_rise(power);
                    elem.temperature = t_in;
                    t_in -= dt;
                }
            }
            _ => {}
        }
    }

    /// Update junction temperature for transient analysis
    ///
    /// Solves thermal network dynamics with given timestep
    pub fn update_transient(&mut self, power: Value, dt: Value) {
        self.power = power;

        if !self.enabled || self.network_type == ThermalNetworkType::Isothermal {
            return;
        }

        match self.network_type {
            ThermalNetworkType::SinglePole => {
                if let Some(elem) = self.elements.first_mut() {
                    self.t_junction = elem.transient_update(power, self.t_ambient, dt);
                }
            }
            ThermalNetworkType::Foster => {
                // Foster: Independent parallel RC sections
                self.t_junction = self.t_ambient;
                for elem in &mut self.elements {
                    let delta_t = elem.transient_update(power, 0.0, dt);
                    self.t_junction += delta_t;
                }
            }
            ThermalNetworkType::Cauer => {
                // Cauer: Ladder from junction to ambient
                // Solve from ambient toward junction
                let mut t_prev = self.t_ambient;
                for elem in self.elements.iter_mut().rev() {
                    t_prev = elem.transient_update(power, t_prev, dt);
                }
                self.t_junction = self
                    .elements
                    .first()
                    .map(|e| e.temperature)
                    .unwrap_or(self.t_ambient);
            }
            ThermalNetworkType::Isothermal => {}
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_thermal_rc_element_creation() {
        let elem = ThermalRcElement::new(50.0, 1e-3);
        assert!((elem.rth - 50.0).abs() < 1e-10);
        assert!((elem.cth - 1e-3).abs() < 1e-10);
        assert!((elem.temperature - TAMB_DEFAULT).abs() < 1e-10);
    }

    #[test]
    fn test_thermal_rc_dc_response() {
        let elem = ThermalRcElement::new(100.0, 1e-3);
        let dt = elem.dc_temperature_rise(1.0); // 1W dissipation
        assert!((dt - 100.0).abs() < 1e-10); // ΔT = 100°C
    }

    #[test]
    fn test_thermal_rc_time_constant() {
        let elem = ThermalRcElement::new(100.0, 0.01);
        let tau = elem.time_constant();
        assert!((tau - 1.0).abs() < 1e-10); // τ = 1 second
    }

    #[test]
    fn test_thermal_network_single_pole() {
        let mut net = ThermalNetwork::single_pole(50.0, 1e-3);
        net.update_dc(2.0); // 2W

        // Tj = Ta + P × Rth = 27 + 2 × 50 = 127°C
        assert!((net.junction_temperature() - 127.0).abs() < 0.1);
    }

    #[test]
    fn test_thermal_network_foster() {
        // 3-pole Foster network (typical IC package)
        let net = ThermalNetwork::foster(&[
            (10.0, 1e-4), // Die
            (30.0, 1e-3), // Package
            (60.0, 1e-2), // Heatsink
        ]);

        assert!((net.rth_total - 100.0).abs() < 1e-10);
        assert_eq!(net.elements.len(), 3);
    }

    #[test]
    fn test_thermal_network_dc_update() {
        let mut net = ThermalNetwork::foster(&[(25.0, 1e-3), (75.0, 1e-2)]).with_ambient(25.0);

        net.update_dc(1.0); // 1W

        // Tj = 25 + 1 × 100 = 125°C
        assert!((net.junction_temperature() - 125.0).abs() < 0.1);
    }

    #[test]
    fn test_thermal_network_transient() {
        let mut net = ThermalNetwork::single_pole(100.0, 0.1).with_ambient(25.0);

        // Apply 1W for 100ms (τ = 10s, so barely warms up)
        net.update_transient(1.0, 0.1);

        let t1 = net.junction_temperature();
        assert!(t1 > 25.0 && t1 < 125.0, "Should partially warm up");

        // Apply for much longer (10 × τ = 100s)
        for _ in 0..100 {
            net.update_transient(1.0, 1.0);
            net.accept_step();
        }

        // Should be near steady state
        assert!((net.junction_temperature() - 125.0).abs() < 5.0);
    }

    #[test]
    fn test_thermal_network_isothermal() {
        let mut net = ThermalNetwork::isothermal(85.0);

        net.update_dc(10.0); // Should have no effect
        assert!((net.junction_temperature() - 85.0).abs() < 1e-10);
    }

    #[test]
    fn test_thermal_network_reset() {
        let mut net = ThermalNetwork::single_pole(100.0, 1e-3).with_ambient(25.0);

        net.update_dc(2.0);
        assert!(net.junction_temperature() > 100.0);

        net.reset();
        assert!((net.junction_temperature() - 25.0).abs() < 1e-10);
    }

    #[test]
    fn test_mobility_factor() {
        let net = ThermalNetwork::isothermal(27.0); // Room temp
        let factor = net.mobility_factor(2.0);
        assert!((factor - 1.0).abs() < 0.01); // Near unity at T_ref

        // At higher temp, mobility should decrease
        let hot_net = ThermalNetwork::isothermal(127.0);
        let hot_factor = hot_net.mobility_factor(2.0);
        assert!(
            hot_factor < 1.0,
            "Mobility should decrease with temperature"
        );
    }

    #[test]
    fn test_vth_shift() {
        // Use exact reference temperature: TREF - KELVIN_OFFSET = 300 - 273.15 = 26.85°C
        let t_ref_celsius = TREF - KELVIN_OFFSET;
        let net = ThermalNetwork::isothermal(t_ref_celsius);
        let shift = net.vth_shift(-2.5e-3, 0.0);
        assert!(shift.abs() < 1e-10, "No shift at T_ref: got {}", shift);

        // At 100°C higher
        let hot_net = ThermalNetwork::isothermal(t_ref_celsius + 100.0);
        let hot_shift = hot_net.vth_shift(-2.5e-3, 0.0);
        // Should be about -250mV for 100°C rise
        assert!(
            (hot_shift - (-0.25)).abs() < 0.01,
            "Expected -0.25V, got {}",
            hot_shift
        );
    }

    #[test]
    fn test_is_factor() {
        // Use exact reference temperature: TREF - KELVIN_OFFSET = 300 - 273.15 = 26.85°C
        let t_ref_celsius = TREF - KELVIN_OFFSET;
        let net = ThermalNetwork::isothermal(t_ref_celsius);
        let factor = net.is_factor(3.0, 1.12);
        assert!(
            (factor - 1.0).abs() < 0.01,
            "Expected 1.0 at T_ref, got {}",
            factor
        );

        // At higher temp, Is should increase dramatically
        let hot_net = ThermalNetwork::isothermal(t_ref_celsius + 100.0);
        let hot_factor = hot_net.is_factor(3.0, 1.12);
        assert!(
            hot_factor > 100.0,
            "Is should increase ~2x per 10°C, got {}",
            hot_factor
        );
    }

    #[test]
    fn test_temperature_coefficients_defaults() {
        let tc = TemperatureCoefficients::default();
        assert!((tc.bex - 1.5).abs() < 1e-10);
        assert!((tc.eg - 1.12).abs() < 1e-10);
    }

    #[test]
    fn test_temperature_coefficients_mosfet() {
        let tc = TemperatureCoefficients::mosfet();
        assert!(tc.trs > 0.0); // Positive TC for Rdson
    }

    #[test]
    fn test_junction_temperature_kelvin() {
        let net = ThermalNetwork::isothermal(27.0);
        let t_k = net.junction_temperature_kelvin();
        assert!((t_k - 300.15).abs() < 0.1);
    }

    #[test]
    fn test_thermal_network_type_default() {
        assert_eq!(
            ThermalNetworkType::default(),
            ThermalNetworkType::SinglePole
        );
    }

    #[test]
    fn test_cauer_network() {
        let net = ThermalNetwork::cauer(&[(20.0, 1e-3), (80.0, 1e-2)]).with_ambient(25.0);

        assert_eq!(net.network_type, ThermalNetworkType::Cauer);
        assert!((net.rth_total - 100.0).abs() < 1e-10);
    }

    #[test]
    fn test_thermal_enabled_disabled() {
        let mut net = ThermalNetwork::single_pole(100.0, 1e-3)
            .with_ambient(25.0)
            .with_enabled(false);

        net.update_dc(2.0);
        // Should not heat up when disabled
        assert!((net.junction_temperature() - 25.0).abs() < 1e-10);
    }
}
