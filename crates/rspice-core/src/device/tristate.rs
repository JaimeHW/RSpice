//! Tristate Bus and Signal Strength Models
//!
//! Implements advanced tristate logic with signal strength resolution
//! for mixed-signal simulation. Essential for modeling:
//! - Tristate buffers and drivers
//! - I2C, SPI, and other open-drain buses
//! - Memory buses with multiple drivers
//! - Pull-up/pull-down networks
//!
//! # Signal Strength Model
//!
//! Based on IEEE 1164 (std_logic) signal strength conventions:
//! - Strong: Active driver output (e.g., CMOS push-pull)
//! - Pull: Resistive connection (e.g., open-drain with pull-up)
//! - Weak: Very weak resistive connection
//! - High-Z: High impedance (no drive)
//!
//! # Bus Resolution
//!
//! When multiple drivers connect to a bus, the strongest signal wins:
//! ```text
//! Strong_1 + Strong_0 = X (conflict)
//! Strong_1 + Pull_0   = 1 (strong wins)
//! Pull_1   + Pull_0   = X (conflict)
//! Strong_1 + High_Z   = 1 (any drive beats Z)
//! Pull_1   + High_Z   = 1 (pull wins)
//! High_Z   + High_Z   = Z (no drive)
//! ```
//!
//! # Analog Equivalent
//!
//! For SPICE-level simulation, strengths map to equivalent resistances:
//! - Strong: 1Ω - 100Ω (very low impedance)
//! - Pull: 1kΩ - 100kΩ (typical pull-up/down)
//! - Weak: 100kΩ - 10MΩ (weak keeper)
//! - High-Z: >1GΩ (essentially open)

use crate::Value;
use std::fmt;

//=============================================================================
// Signal Strength
//=============================================================================

/// Drive strength levels (IEEE 1164 inspired)
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub enum DriveStrength {
    /// High impedance - no drive
    HighZ = 0,
    /// Weak drive (e.g., very weak keeper)
    Weak = 1,
    /// Pull strength (e.g., resistive pull-up/down)
    Pull = 2,
    /// Strong drive (e.g., active CMOS output)
    #[default]
    Strong = 3,
    /// Supply level (direct VDD/VSS connection)
    Supply = 4,
}

impl DriveStrength {
    /// Convert strength to equivalent resistance (Ω)
    pub fn to_resistance(&self) -> Value {
        match self {
            DriveStrength::HighZ => 1e12,   // 1TΩ (essentially open)
            DriveStrength::Weak => 1e7,     // 10MΩ
            DriveStrength::Pull => 1e4,     // 10kΩ (typical pull resistor)
            DriveStrength::Strong => 100.0, // 100Ω (active driver)
            DriveStrength::Supply => 1.0,   // 1Ω (direct supply)
        }
    }

    /// Create from resistance value
    pub fn from_resistance(r: Value) -> Self {
        if r > 1e9 {
            DriveStrength::HighZ
        } else if r > 1e5 {
            DriveStrength::Weak
        } else if r > 1e3 {
            DriveStrength::Pull
        } else if r > 10.0 {
            DriveStrength::Strong
        } else {
            DriveStrength::Supply
        }
    }

    /// Get conductance (1/R)
    pub fn to_conductance(&self) -> Value {
        1.0 / self.to_resistance()
    }
}

impl fmt::Display for DriveStrength {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            DriveStrength::HighZ => write!(f, "Z"),
            DriveStrength::Weak => write!(f, "W"),
            DriveStrength::Pull => write!(f, "P"),
            DriveStrength::Strong => write!(f, "S"),
            DriveStrength::Supply => write!(f, "U"),
        }
    }
}

//=============================================================================
// Logic Value with Strength
//=============================================================================

/// Logic value (high/low/unknown/high-z)
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum LogicValue {
    /// Logic low (0)
    Low,
    /// Logic high (1)
    High,
    /// Unknown/conflict (X)
    #[default]
    Unknown,
    /// High impedance (Z)
    HighZ,
}

impl LogicValue {
    /// Convert to analog voltage
    pub fn to_voltage(&self, vdd: Value, vss: Value) -> Option<Value> {
        match self {
            LogicValue::Low => Some(vss),
            LogicValue::High => Some(vdd),
            LogicValue::Unknown => Some((vdd + vss) / 2.0), // Mid-rail for X
            LogicValue::HighZ => None,                      // No voltage defined
        }
    }

    /// Create from analog voltage with thresholds
    pub fn from_voltage(v: Value, vth_low: Value, vth_high: Value) -> Self {
        if v < vth_low {
            LogicValue::Low
        } else if v > vth_high {
            LogicValue::High
        } else {
            LogicValue::Unknown
        }
    }

    /// Invert the logic value
    pub fn invert(&self) -> Self {
        match self {
            LogicValue::Low => LogicValue::High,
            LogicValue::High => LogicValue::Low,
            LogicValue::Unknown => LogicValue::Unknown,
            LogicValue::HighZ => LogicValue::HighZ,
        }
    }
}

impl fmt::Display for LogicValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            LogicValue::Low => write!(f, "0"),
            LogicValue::High => write!(f, "1"),
            LogicValue::Unknown => write!(f, "X"),
            LogicValue::HighZ => write!(f, "Z"),
        }
    }
}

//=============================================================================
// Driven Signal (Value + Strength)
//=============================================================================

/// A signal with both logic value and drive strength
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DrivenSignal {
    /// Logic value
    pub value: LogicValue,
    /// Drive strength
    pub strength: DriveStrength,
}

impl DrivenSignal {
    /// Create a new driven signal
    pub fn new(value: LogicValue, strength: DriveStrength) -> Self {
        Self { value, strength }
    }

    /// Create high-impedance (Z) signal
    pub fn high_z() -> Self {
        Self {
            value: LogicValue::HighZ,
            strength: DriveStrength::HighZ,
        }
    }

    /// Create strong high
    pub fn strong_high() -> Self {
        Self {
            value: LogicValue::High,
            strength: DriveStrength::Strong,
        }
    }

    /// Create strong low
    pub fn strong_low() -> Self {
        Self {
            value: LogicValue::Low,
            strength: DriveStrength::Strong,
        }
    }

    /// Create pull-up (weak high)
    pub fn pull_up() -> Self {
        Self {
            value: LogicValue::High,
            strength: DriveStrength::Pull,
        }
    }

    /// Create pull-down (weak low)
    pub fn pull_down() -> Self {
        Self {
            value: LogicValue::Low,
            strength: DriveStrength::Pull,
        }
    }

    /// Create weak keeper high
    pub fn weak_high() -> Self {
        Self {
            value: LogicValue::High,
            strength: DriveStrength::Weak,
        }
    }

    /// Create weak keeper low
    pub fn weak_low() -> Self {
        Self {
            value: LogicValue::Low,
            strength: DriveStrength::Weak,
        }
    }

    /// Check if this is high-impedance
    pub fn is_high_z(&self) -> bool {
        self.strength == DriveStrength::HighZ || self.value == LogicValue::HighZ
    }

    /// Convert to analog voltage with equivalent resistance
    /// Returns (voltage, resistance) for SPICE stamping
    pub fn to_analog(&self, vdd: Value, vss: Value) -> (Option<Value>, Value) {
        let r = self.strength.to_resistance();
        let v = match self.value {
            LogicValue::High => Some(vdd),
            LogicValue::Low => Some(vss),
            LogicValue::Unknown => Some((vdd + vss) / 2.0),
            LogicValue::HighZ => None,
        };
        (v, r)
    }
}

impl Default for DrivenSignal {
    fn default() -> Self {
        Self::high_z()
    }
}

impl fmt::Display for DrivenSignal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}{}", self.strength, self.value)
    }
}

//=============================================================================
// Bus Resolution
//=============================================================================

/// Resolves multiple drivers on a bus to a single signal
///
/// This implements the standard resolution function where stronger
/// signals override weaker ones, and equal-strength conflicts produce X.
#[derive(Debug, Clone)]
pub struct BusResolver {
    /// Supply voltage (VDD)
    vdd: Value,
    /// Ground voltage (VSS)
    vss: Value,
}

impl BusResolver {
    /// Create a new bus resolver with given supply rails
    pub fn new(vdd: Value, vss: Value) -> Self {
        Self { vdd, vss }
    }

    /// Create with standard 3.3V logic levels
    pub fn logic_3v3() -> Self {
        Self::new(3.3, 0.0)
    }

    /// Create with standard 5V logic levels
    pub fn logic_5v() -> Self {
        Self::new(5.0, 0.0)
    }

    /// Create with standard 1.8V logic levels
    pub fn logic_1v8() -> Self {
        Self::new(1.8, 0.0)
    }

    /// Resolve two signals according to strength rules
    pub fn resolve_pair(&self, a: DrivenSignal, b: DrivenSignal) -> DrivenSignal {
        // If either is pure high-Z, return the other
        if a.is_high_z() {
            return b;
        }
        if b.is_high_z() {
            return a;
        }

        // Compare strengths
        match a.strength.cmp(&b.strength) {
            std::cmp::Ordering::Greater => a,
            std::cmp::Ordering::Less => b,
            std::cmp::Ordering::Equal => {
                // Same strength - check for conflict
                if a.value == b.value {
                    a // Same value, no conflict
                } else {
                    // Conflict - result is unknown
                    DrivenSignal::new(LogicValue::Unknown, a.strength)
                }
            }
        }
    }

    /// Resolve multiple drivers on a bus
    pub fn resolve(&self, drivers: &[DrivenSignal]) -> DrivenSignal {
        if drivers.is_empty() {
            return DrivenSignal::high_z();
        }

        let mut result = DrivenSignal::high_z();
        for &driver in drivers {
            result = self.resolve_pair(result, driver);
        }
        result
    }

    /// Resolve to analog voltage and equivalent resistance
    ///
    /// For SPICE integration, calculates the Thevenin equivalent of
    /// all drivers on the bus.
    pub fn resolve_analog(&self, drivers: &[DrivenSignal]) -> (Value, Value) {
        if drivers.is_empty() {
            // No drivers - return high-Z
            return ((self.vdd + self.vss) / 2.0, 1e12);
        }

        // Use conductance-weighted average for analog resolution
        let mut total_conductance = 0.0;
        let mut weighted_voltage = 0.0;

        for driver in drivers {
            if driver.is_high_z() {
                continue;
            }

            let g = driver.strength.to_conductance();
            let v = match driver.value {
                LogicValue::High => self.vdd,
                LogicValue::Low => self.vss,
                LogicValue::Unknown => (self.vdd + self.vss) / 2.0,
                LogicValue::HighZ => continue,
            };

            total_conductance += g;
            weighted_voltage += g * v;
        }

        if total_conductance < 1e-15 {
            // All high-Z
            return ((self.vdd + self.vss) / 2.0, 1e12);
        }

        let veq = weighted_voltage / total_conductance;
        let req = 1.0 / total_conductance;

        (veq, req)
    }

    /// Get VDD level
    pub fn vdd(&self) -> Value {
        self.vdd
    }

    /// Get VSS level
    pub fn vss(&self) -> Value {
        self.vss
    }
}

impl Default for BusResolver {
    fn default() -> Self {
        Self::logic_3v3()
    }
}

//=============================================================================
// Tristate Buffer
//=============================================================================

/// Tristate buffer with configurable drive strength
#[derive(Debug, Clone)]
pub struct TristateBuffer {
    /// Device name
    pub name: String,
    /// Input node
    pub node_in: usize,
    /// Output node  
    pub node_out: usize,
    /// Enable node (active high)
    pub node_enable: usize,
    /// Output drive strength when enabled
    pub drive_strength: DriveStrength,
    /// Output resistance when enabled
    pub r_on: Value,
    /// Output resistance when disabled (high-Z)
    pub r_off: Value,
    /// Current enable state
    enabled: bool,
    /// Current output value
    output: LogicValue,
}

impl TristateBuffer {
    /// Create a new tristate buffer
    pub fn new(name: String, node_in: usize, node_out: usize, node_enable: usize) -> Self {
        Self {
            name,
            node_in,
            node_out,
            node_enable,
            drive_strength: DriveStrength::Strong,
            r_on: 100.0, // 100Ω when enabled
            r_off: 1e12, // 1TΩ when disabled
            enabled: false,
            output: LogicValue::HighZ,
        }
    }

    /// Set drive strength
    pub fn with_strength(mut self, strength: DriveStrength) -> Self {
        self.drive_strength = strength;
        self.r_on = strength.to_resistance();
        self
    }

    /// Update buffer state from analog voltages
    pub fn update(&mut self, v_enable: Value, v_in: Value, vth: Value) {
        // Check enable
        self.enabled = v_enable > vth;

        // Determine output
        if self.enabled {
            self.output = if v_in > vth {
                LogicValue::High
            } else {
                LogicValue::Low
            };
        } else {
            self.output = LogicValue::HighZ;
        }
    }

    /// Get output as driven signal
    pub fn output_signal(&self) -> DrivenSignal {
        if self.enabled {
            DrivenSignal::new(self.output, self.drive_strength)
        } else {
            DrivenSignal::high_z()
        }
    }

    /// Get equivalent output resistance
    pub fn output_resistance(&self) -> Value {
        if self.enabled { self.r_on } else { self.r_off }
    }

    /// Check if buffer is enabled
    pub fn is_enabled(&self) -> bool {
        self.enabled
    }
}

//=============================================================================
// Pull Resistor
//=============================================================================

/// Pull-up or pull-down resistor with strength
#[derive(Debug, Clone)]
pub struct PullResistor {
    /// Device name
    pub name: String,
    /// Node to pull
    pub node: usize,
    /// Pull direction (high or low)
    pub direction: LogicValue,
    /// Resistance value
    pub resistance: Value,
    /// Drive strength
    pub strength: DriveStrength,
}

impl PullResistor {
    /// Create a pull-up resistor
    pub fn pull_up(name: String, node: usize, resistance: Value) -> Self {
        Self {
            name,
            node,
            direction: LogicValue::High,
            resistance,
            strength: DriveStrength::from_resistance(resistance),
        }
    }

    /// Create a pull-down resistor
    pub fn pull_down(name: String, node: usize, resistance: Value) -> Self {
        Self {
            name,
            node,
            direction: LogicValue::Low,
            resistance,
            strength: DriveStrength::from_resistance(resistance),
        }
    }

    /// Get as driven signal
    pub fn as_signal(&self) -> DrivenSignal {
        DrivenSignal::new(self.direction, self.strength)
    }
}

//=============================================================================
// Bus
//=============================================================================

/// A bus with multiple potential drivers
#[derive(Debug, Clone)]
pub struct Bus {
    /// Bus name
    pub name: String,
    /// Main node for the bus
    pub node: usize,
    /// Connected drivers
    drivers: Vec<DrivenSignal>,
    /// Pull resistors
    pulls: Vec<DrivenSignal>,
    /// Current resolved value
    resolved: DrivenSignal,
    /// Bus resolver
    resolver: BusResolver,
}

impl Bus {
    /// Create a new bus
    pub fn new(name: String, node: usize, vdd: Value, vss: Value) -> Self {
        Self {
            name,
            node,
            drivers: Vec::new(),
            pulls: Vec::new(),
            resolved: DrivenSignal::high_z(),
            resolver: BusResolver::new(vdd, vss),
        }
    }

    /// Add a driver to the bus
    pub fn add_driver(&mut self, signal: DrivenSignal) {
        self.drivers.push(signal);
    }

    /// Add a pull resistor
    pub fn add_pull(&mut self, signal: DrivenSignal) {
        self.pulls.push(signal);
    }

    /// Clear all drivers (called each simulation step)
    pub fn clear_drivers(&mut self) {
        self.drivers.clear();
    }

    /// Resolve the bus value from all drivers
    pub fn resolve(&mut self) -> DrivenSignal {
        // Combine drivers and pulls
        let mut all_signals: Vec<DrivenSignal> = self.drivers.clone();
        all_signals.extend(self.pulls.iter().cloned());

        self.resolved = self.resolver.resolve(&all_signals);
        self.resolved
    }

    /// Get Thevenin equivalent for SPICE stamping
    /// Returns (voltage, resistance)
    pub fn thevenin_equivalent(&self) -> (Value, Value) {
        let mut all_signals: Vec<DrivenSignal> = self.drivers.clone();
        all_signals.extend(self.pulls.iter().cloned());
        self.resolver.resolve_analog(&all_signals)
    }

    /// Get current resolved value
    pub fn value(&self) -> DrivenSignal {
        self.resolved
    }

    /// Check if bus has a conflict
    pub fn has_conflict(&self) -> bool {
        self.resolved.value == LogicValue::Unknown
            && self.resolved.strength >= DriveStrength::Strong
    }

    /// Get number of active drivers
    pub fn active_driver_count(&self) -> usize {
        self.drivers.iter().filter(|d| !d.is_high_z()).count()
    }
}

//=============================================================================
// Tests
//=============================================================================
