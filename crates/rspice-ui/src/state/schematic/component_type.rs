//! Component Types
//!
//! Enumeration of all supported schematic component types with their
//! SPICE prefixes, display names, terminal counts, and terminal positions.

use super::point::Point;
use serde::{Deserialize, Serialize};

// =============================================================================
// ComponentType Enum
// =============================================================================

/// Component types available in the schematic
///
/// This enumeration covers all supported circuit elements:
/// - Passive components (R, L, C)
/// - Semiconductors (diodes, transistors)
/// - Sources (voltage, current, controlled)
/// - XSPICE behavioral models
/// - XSPICE digital gates
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ComponentType {
    // =========================================================================
    // Passive Components
    // =========================================================================
    /// Resistor (SPICE prefix: R)
    Resistor,
    /// Capacitor (SPICE prefix: C)
    Capacitor,
    /// Inductor (SPICE prefix: L)
    Inductor,
    /// Coupled inductor for transformers (SPICE prefix: K)
    CoupledInductor,

    // =========================================================================
    // Semiconductors - Diodes
    // =========================================================================
    /// Diode (SPICE prefix: D)
    Diode,

    // =========================================================================
    // Semiconductors - Bipolar Transistors
    // =========================================================================
    /// NPN Bipolar Junction Transistor (SPICE prefix: Q)
    NpnBjt,
    /// PNP Bipolar Junction Transistor (SPICE prefix: Q)
    PnpBjt,

    // =========================================================================
    // Semiconductors - MOSFETs
    // =========================================================================
    /// N-channel MOSFET (SPICE prefix: M)
    Nmos,
    /// P-channel MOSFET (SPICE prefix: M)
    Pmos,

    // =========================================================================
    // Semiconductors - JFETs
    // =========================================================================
    /// N-channel JFET (SPICE prefix: J)
    Njfet,
    /// P-channel JFET (SPICE prefix: J)
    Pjfet,

    // =========================================================================
    // Power Electronics Devices
    // =========================================================================
    /// N-channel VDMOS Power MOSFET (SPICE prefix: M)
    NVdmos,
    /// P-channel VDMOS Power MOSFET (SPICE prefix: M)
    PVdmos,
    /// Saturable Core Inductor (SPICE prefix: L)
    SaturableInductor,
    // =========================================================================
    // Independent Sources
    // =========================================================================
    /// DC Voltage Source (SPICE prefix: V)
    VoltageSource,
    /// DC Current Source (SPICE prefix: I)
    CurrentSource,
    /// AC Voltage Source (SPICE prefix: V)
    VoltageSourceAc,
    /// Pulse Voltage Source (SPICE prefix: V)
    VoltageSourcePulse,
    /// Sinusoidal Voltage Source (SPICE prefix: V)
    VoltageSourceSin,
    /// Piecewise Linear Voltage Source (SPICE prefix: V)
    VoltageSourcePwl,
    /// Exponential Voltage Source (SPICE prefix: V)
    VoltageSourceExp,
    /// Single-Frequency FM Voltage Source (SPICE prefix: V)
    VoltageSourceSffm,

    // =========================================================================
    // Advanced Current Sources
    // =========================================================================
    /// AC Current Source (SPICE prefix: I)
    CurrentSourceAc,
    /// Pulse Current Source (SPICE prefix: I)
    CurrentSourcePulse,
    /// Sinusoidal Current Source (SPICE prefix: I)
    CurrentSourceSin,
    /// Piecewise Linear Current Source (SPICE prefix: I)
    CurrentSourcePwl,
    /// Exponential Current Source (SPICE prefix: I)
    CurrentSourceExp,
    /// Noise Current Source (SPICE prefix: I)
    CurrentSourceNoise,

    // =========================================================================
    // Controlled (Dependent) Sources
    // =========================================================================
    /// Voltage-Controlled Voltage Source (SPICE prefix: E)
    Vcvs,
    /// Voltage-Controlled Current Source (SPICE prefix: G)
    Vccs,
    /// Current-Controlled Voltage Source (SPICE prefix: H)
    Ccvs,
    /// Current-Controlled Current Source (SPICE prefix: F)
    Cccs,

    // =========================================================================
    // Special
    // =========================================================================
    /// Ground node (no SPICE prefix - implicit node 0)
    Ground,

    // =========================================================================
    // XSPICE Analog Behavioral Models
    // =========================================================================
    /// Gain block (×k)
    XspiceGain,
    /// Summing amplifier (Σ)
    XspiceSummer,
    /// Analog multiplier (×)
    XspiceMultiplier,
    /// Analog divider (÷)
    XspiceDivider,
    /// Hard limiter
    XspiceLimiter,
    /// Integrator (∫)
    XspiceIntegrator,
    /// Differentiator (d/dt)
    XspiceDifferentiator,

    // =========================================================================
    // XSPICE Digital Gates
    // =========================================================================
    /// NOT gate (inverter)
    XspiceInverter,
    /// Digital buffer
    XspiceBuffer,
    /// AND gate
    XspiceAndGate,
    /// OR gate
    XspiceOrGate,
    /// NAND gate
    XspiceNandGate,
    /// NOR gate
    XspiceNorGate,
    /// XOR gate
    XspiceXorGate,
    /// Tri-state buffer
    XspiceTristate,

    // =========================================================================
    // XSPICE Digital Sequential
    // =========================================================================
    /// D Flip-Flop
    XspiceDFlipFlop,
    /// JK Flip-Flop
    XspiceJkFlipFlop,
    /// SR Latch
    XspiceSrLatch,

    // =========================================================================
    // XSPICE Analog/Digital Bridges
    // =========================================================================
    /// Analog-to-Digital converter bridge
    XspiceAdcBridge,
    /// Digital-to-Analog converter bridge
    XspiceDacBridge,
}

impl ComponentType {
    /// Get the SPICE prefix for this component type
    ///
    /// This is the single-character prefix used in SPICE netlists
    /// (e.g., "R" for resistor, "C" for capacitor).
    pub fn spice_prefix(&self) -> &'static str {
        match self {
            ComponentType::Resistor => "R",
            ComponentType::Capacitor => "C",
            ComponentType::Inductor => "L",
            ComponentType::CoupledInductor => "K",
            ComponentType::Diode => "D",
            ComponentType::NpnBjt | ComponentType::PnpBjt => "Q",
            ComponentType::Nmos
            | ComponentType::Pmos
            | ComponentType::NVdmos
            | ComponentType::PVdmos => "M",
            ComponentType::Njfet | ComponentType::Pjfet => "J",
            ComponentType::SaturableInductor => "L",
            ComponentType::VoltageSource
            | ComponentType::VoltageSourceAc
            | ComponentType::VoltageSourcePulse
            | ComponentType::VoltageSourceSin
            | ComponentType::VoltageSourcePwl
            | ComponentType::VoltageSourceExp
            | ComponentType::VoltageSourceSffm => "V",
            ComponentType::CurrentSource
            | ComponentType::CurrentSourceAc
            | ComponentType::CurrentSourcePulse
            | ComponentType::CurrentSourceSin
            | ComponentType::CurrentSourcePwl
            | ComponentType::CurrentSourceExp
            | ComponentType::CurrentSourceNoise => "I",
            ComponentType::Vcvs => "E",
            ComponentType::Vccs => "G",
            ComponentType::Ccvs => "H",
            ComponentType::Cccs => "F",
            ComponentType::Ground => "",
            // All XSPICE components use "A" prefix
            ComponentType::XspiceGain
            | ComponentType::XspiceSummer
            | ComponentType::XspiceMultiplier
            | ComponentType::XspiceDivider
            | ComponentType::XspiceLimiter
            | ComponentType::XspiceIntegrator
            | ComponentType::XspiceDifferentiator
            | ComponentType::XspiceInverter
            | ComponentType::XspiceBuffer
            | ComponentType::XspiceAndGate
            | ComponentType::XspiceOrGate
            | ComponentType::XspiceNandGate
            | ComponentType::XspiceNorGate
            | ComponentType::XspiceXorGate
            | ComponentType::XspiceTristate
            | ComponentType::XspiceDFlipFlop
            | ComponentType::XspiceJkFlipFlop
            | ComponentType::XspiceSrLatch
            | ComponentType::XspiceAdcBridge
            | ComponentType::XspiceDacBridge => "A",
        }
    }

    /// Get the display name for this component type
    ///
    /// This is the human-readable name shown in the UI.
    pub fn display_name(&self) -> &'static str {
        match self {
            ComponentType::Resistor => "Resistor",
            ComponentType::Capacitor => "Capacitor",
            ComponentType::Inductor => "Inductor",
            ComponentType::CoupledInductor => "Coupled Inductor",
            ComponentType::Diode => "Diode",
            ComponentType::NpnBjt => "NPN BJT",
            ComponentType::PnpBjt => "PNP BJT",
            ComponentType::Nmos => "NMOS",
            ComponentType::Pmos => "PMOS",
            ComponentType::Njfet => "N-JFET",
            ComponentType::Pjfet => "P-JFET",
            ComponentType::NVdmos => "N-VDMOS",
            ComponentType::PVdmos => "P-VDMOS",
            ComponentType::SaturableInductor => "Saturable L",
            ComponentType::VoltageSource => "V DC",
            ComponentType::CurrentSource => "I DC",
            ComponentType::VoltageSourceAc => "V AC",
            ComponentType::VoltageSourcePulse => "V Pulse",
            ComponentType::VoltageSourceSin => "V Sin",
            ComponentType::VoltageSourcePwl => "V PWL",
            ComponentType::VoltageSourceExp => "V Exp",
            ComponentType::VoltageSourceSffm => "V SFFM",
            ComponentType::CurrentSourceAc => "I AC",
            ComponentType::CurrentSourcePulse => "I Pulse",
            ComponentType::CurrentSourceSin => "I Sin",
            ComponentType::CurrentSourcePwl => "I PWL",
            ComponentType::CurrentSourceExp => "I Exp",
            ComponentType::CurrentSourceNoise => "I Noise",
            ComponentType::Vcvs => "VCVS (E)",
            ComponentType::Vccs => "VCCS (G)",
            ComponentType::Ccvs => "CCVS (H)",
            ComponentType::Cccs => "CCCS (F)",
            ComponentType::Ground => "Ground",
            // XSPICE Analog Behavioral
            ComponentType::XspiceGain => "Gain",
            ComponentType::XspiceSummer => "Summer",
            ComponentType::XspiceMultiplier => "Multiplier",
            ComponentType::XspiceDivider => "Divider",
            ComponentType::XspiceLimiter => "Limiter",
            ComponentType::XspiceIntegrator => "Integrator",
            ComponentType::XspiceDifferentiator => "Differentiator",
            // XSPICE Digital Gates
            ComponentType::XspiceInverter => "Inverter",
            ComponentType::XspiceBuffer => "Buffer",
            ComponentType::XspiceAndGate => "AND Gate",
            ComponentType::XspiceOrGate => "OR Gate",
            ComponentType::XspiceNandGate => "NAND Gate",
            ComponentType::XspiceNorGate => "NOR Gate",
            ComponentType::XspiceXorGate => "XOR Gate",
            ComponentType::XspiceTristate => "Tri-State",
            // XSPICE Sequential
            ComponentType::XspiceDFlipFlop => "D Flip-Flop",
            ComponentType::XspiceJkFlipFlop => "JK Flip-Flop",
            ComponentType::XspiceSrLatch => "SR Latch",
            // XSPICE Bridges
            ComponentType::XspiceAdcBridge => "ADC Bridge",
            ComponentType::XspiceDacBridge => "DAC Bridge",
        }
    }

    /// Get the number of terminals for this component type
    pub fn terminal_count(&self) -> usize {
        match self {
            ComponentType::Ground => 1,
            ComponentType::NpnBjt | ComponentType::PnpBjt => 3,
            ComponentType::Njfet | ComponentType::Pjfet => 3,
            ComponentType::Nmos
            | ComponentType::Pmos
            | ComponentType::NVdmos
            | ComponentType::PVdmos => 4,
            // Controlled sources have 4 terminals: output+, output-, control+, control-
            ComponentType::Vcvs
            | ComponentType::Vccs
            | ComponentType::Ccvs
            | ComponentType::Cccs => 4,
            // Coupled inductor is special - refers to two inductors
            ComponentType::CoupledInductor => 0,
            // XSPICE components with 3+ terminals
            ComponentType::XspiceSummer
            | ComponentType::XspiceMultiplier
            | ComponentType::XspiceDivider => 3,
            ComponentType::XspiceAndGate
            | ComponentType::XspiceOrGate
            | ComponentType::XspiceNandGate
            | ComponentType::XspiceNorGate
            | ComponentType::XspiceXorGate
            | ComponentType::XspiceTristate => 3,
            ComponentType::XspiceDFlipFlop | ComponentType::XspiceSrLatch => 4,
            ComponentType::XspiceJkFlipFlop => 5,
            _ => 2, // Most components have 2 terminals
        }
    }

    /// Get terminal offsets relative to component position
    ///
    /// Returns (name, offset) pairs for each terminal.
    /// Offsets are in grid units from the component center.
    pub fn terminal_offsets(&self) -> Vec<(&'static str, Point)> {
        match self {
            ComponentType::Resistor | ComponentType::Capacitor | ComponentType::Inductor => {
                vec![("+", Point::new(-2, 0)), ("-", Point::new(2, 0))]
            }
            ComponentType::Diode => vec![
                ("A", Point::new(-2, 0)), // Anode
                ("K", Point::new(2, 0)),  // Cathode
            ],
            ComponentType::VoltageSource
            | ComponentType::VoltageSourceAc
            | ComponentType::VoltageSourcePulse
            | ComponentType::VoltageSourceSin
            | ComponentType::VoltageSourcePwl
            | ComponentType::VoltageSourceExp
            | ComponentType::VoltageSourceSffm => {
                vec![("+", Point::new(0, -2)), ("-", Point::new(0, 2))]
            }
            ComponentType::CurrentSource
            | ComponentType::CurrentSourceAc
            | ComponentType::CurrentSourcePulse
            | ComponentType::CurrentSourceSin
            | ComponentType::CurrentSourcePwl
            | ComponentType::CurrentSourceExp
            | ComponentType::CurrentSourceNoise => {
                vec![("+", Point::new(0, -2)), ("-", Point::new(0, 2))]
            }
            ComponentType::NpnBjt => vec![
                ("B", Point::new(-2, 0)), // Base
                ("C", Point::new(1, -2)), // Collector
                ("E", Point::new(1, 2)),  // Emitter
            ],
            ComponentType::PnpBjt => vec![
                ("B", Point::new(-2, 0)),
                ("C", Point::new(1, 2)),
                ("E", Point::new(1, -2)),
            ],
            ComponentType::Nmos | ComponentType::Pmos => vec![
                ("G", Point::new(-2, 0)), // Gate
                ("D", Point::new(2, -1)), // Drain
                ("S", Point::new(2, 1)),  // Source
                ("B", Point::new(2, 0)),  // Bulk (usually tied to source)
            ],
            ComponentType::Njfet | ComponentType::Pjfet => vec![
                ("G", Point::new(-2, 0)), // Gate
                ("D", Point::new(2, -1)), // Drain
                ("S", Point::new(2, 1)),  // Source
            ],
            // Power MOSFETs (VDMOS) - 4 terminals like MOSFET
            ComponentType::NVdmos | ComponentType::PVdmos => vec![
                ("G", Point::new(-2, 0)), // Gate
                ("D", Point::new(2, -1)), // Drain
                ("S", Point::new(2, 1)),  // Source
                ("B", Point::new(2, 0)),  // Bulk
            ],
            // Saturable inductor - 2 terminals like regular inductor
            ComponentType::SaturableInductor => {
                vec![("+", Point::new(-2, 0)), ("-", Point::new(2, 0))]
            }
            // Controlled sources: output on left, control on right
            ComponentType::Vcvs
            | ComponentType::Vccs
            | ComponentType::Ccvs
            | ComponentType::Cccs => vec![
                ("O+", Point::new(-2, -1)), // Output +
                ("O-", Point::new(-2, 1)),  // Output -
                ("C+", Point::new(2, -1)),  // Control +
                ("C-", Point::new(2, 1)),   // Control -
            ],
            // Coupled inductor doesn't have terminals (it's a coupling statement)
            ComponentType::CoupledInductor => vec![],
            ComponentType::Ground => vec![("GND", Point::new(0, -2))],

            // XSPICE 2-terminal analog blocks: input left, output right
            ComponentType::XspiceGain
            | ComponentType::XspiceLimiter
            | ComponentType::XspiceIntegrator
            | ComponentType::XspiceDifferentiator => {
                vec![("in", Point::new(-2, 0)), ("out", Point::new(2, 0))]
            }
            // Summer: multiple inputs (top/bottom left), one output right
            ComponentType::XspiceSummer => vec![
                ("in1", Point::new(-2, -1)),
                ("in2", Point::new(-2, 1)),
                ("out", Point::new(2, 0)),
            ],
            // Multiplier/Divider: two inputs, one output
            ComponentType::XspiceMultiplier | ComponentType::XspiceDivider => vec![
                ("in1", Point::new(-2, -1)),
                ("in2", Point::new(-2, 1)),
                ("out", Point::new(2, 0)),
            ],
            // Digital gates: inputs left, output right
            ComponentType::XspiceInverter | ComponentType::XspiceBuffer => {
                vec![("in", Point::new(-2, 0)), ("out", Point::new(2, 0))]
            }
            ComponentType::XspiceAndGate
            | ComponentType::XspiceOrGate
            | ComponentType::XspiceNandGate
            | ComponentType::XspiceNorGate
            | ComponentType::XspiceXorGate => vec![
                ("a", Point::new(-2, -1)),
                ("b", Point::new(-2, 1)),
                ("out", Point::new(2, 0)),
            ],
            // Tri-state: input, enable, output
            ComponentType::XspiceTristate => vec![
                ("in", Point::new(-2, 0)),
                ("en", Point::new(0, -2)),
                ("out", Point::new(2, 0)),
            ],
            // D Flip-Flop: D, CLK, Q, Qbar
            ComponentType::XspiceDFlipFlop => vec![
                ("d", Point::new(-2, -1)),
                ("clk", Point::new(-2, 1)),
                ("q", Point::new(2, -1)),
                ("qbar", Point::new(2, 1)),
            ],
            // JK Flip-Flop: J, K, CLK, Q, Qbar
            ComponentType::XspiceJkFlipFlop => vec![
                ("j", Point::new(-2, -1)),
                ("k", Point::new(-2, 1)),
                ("clk", Point::new(-2, 0)),
                ("q", Point::new(2, -1)),
                ("qbar", Point::new(2, 1)),
            ],
            // SR Latch: S, R, Q, Qbar
            ComponentType::XspiceSrLatch => vec![
                ("s", Point::new(-2, -1)),
                ("r", Point::new(-2, 1)),
                ("q", Point::new(2, -1)),
                ("qbar", Point::new(2, 1)),
            ],
            // ADC Bridge: analog input, digital output
            ComponentType::XspiceAdcBridge => {
                vec![("in", Point::new(-2, 0)), ("out", Point::new(2, 0))]
            }
            // DAC Bridge: digital input, analog output
            ComponentType::XspiceDacBridge => {
                vec![("in", Point::new(-2, 0)), ("out", Point::new(2, 0))]
            }
        }
    }

    /// Check if this is a passive component (R, L, C)
    pub fn is_passive(&self) -> bool {
        matches!(
            self,
            ComponentType::Resistor
                | ComponentType::Capacitor
                | ComponentType::Inductor
                | ComponentType::CoupledInductor
        )
    }

    /// Check if this is a semiconductor device
    pub fn is_semiconductor(&self) -> bool {
        matches!(
            self,
            ComponentType::Diode
                | ComponentType::NpnBjt
                | ComponentType::PnpBjt
                | ComponentType::Nmos
                | ComponentType::Pmos
                | ComponentType::Njfet
                | ComponentType::Pjfet
                | ComponentType::NVdmos
                | ComponentType::PVdmos
        )
    }

    /// Check if this is a source (voltage or current)
    pub fn is_source(&self) -> bool {
        matches!(
            self,
            ComponentType::VoltageSource
                | ComponentType::CurrentSource
                | ComponentType::VoltageSourceAc
                | ComponentType::VoltageSourcePulse
                | ComponentType::VoltageSourceSin
                | ComponentType::VoltageSourcePwl
                | ComponentType::VoltageSourceExp
                | ComponentType::VoltageSourceSffm
                | ComponentType::CurrentSourceAc
                | ComponentType::CurrentSourcePulse
                | ComponentType::CurrentSourceSin
                | ComponentType::CurrentSourcePwl
                | ComponentType::CurrentSourceExp
                | ComponentType::CurrentSourceNoise
                | ComponentType::Vcvs
                | ComponentType::Vccs
                | ComponentType::Ccvs
                | ComponentType::Cccs
        )
    }

    /// Check if this is an XSPICE component
    pub fn is_xspice(&self) -> bool {
        self.spice_prefix() == "A"
    }

    /// Check if this is a digital component
    pub fn is_digital(&self) -> bool {
        matches!(
            self,
            ComponentType::XspiceInverter
                | ComponentType::XspiceBuffer
                | ComponentType::XspiceAndGate
                | ComponentType::XspiceOrGate
                | ComponentType::XspiceNandGate
                | ComponentType::XspiceNorGate
                | ComponentType::XspiceXorGate
                | ComponentType::XspiceTristate
                | ComponentType::XspiceDFlipFlop
                | ComponentType::XspiceJkFlipFlop
                | ComponentType::XspiceSrLatch
                | ComponentType::XspiceAdcBridge
                | ComponentType::XspiceDacBridge
        )
    }

    /// Get default value for this component type
    pub fn default_value(&self) -> &'static str {
        match self {
            ComponentType::Resistor => "1k",
            ComponentType::Capacitor => "1u",
            ComponentType::Inductor => "1m",
            ComponentType::VoltageSource
            | ComponentType::VoltageSourceAc
            | ComponentType::VoltageSourcePulse
            | ComponentType::VoltageSourceSin => "5",
            ComponentType::CurrentSource => "1m",
            _ => "",
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spice_prefix_passives() {
        assert_eq!(ComponentType::Resistor.spice_prefix(), "R");
        assert_eq!(ComponentType::Capacitor.spice_prefix(), "C");
        assert_eq!(ComponentType::Inductor.spice_prefix(), "L");
    }

    #[test]
    fn test_spice_prefix_semiconductors() {
        assert_eq!(ComponentType::Diode.spice_prefix(), "D");
        assert_eq!(ComponentType::NpnBjt.spice_prefix(), "Q");
        assert_eq!(ComponentType::PnpBjt.spice_prefix(), "Q");
        assert_eq!(ComponentType::Nmos.spice_prefix(), "M");
        assert_eq!(ComponentType::Njfet.spice_prefix(), "J");
    }

    #[test]
    fn test_spice_prefix_sources() {
        assert_eq!(ComponentType::VoltageSource.spice_prefix(), "V");
        assert_eq!(ComponentType::CurrentSource.spice_prefix(), "I");
        assert_eq!(ComponentType::Vcvs.spice_prefix(), "E");
        assert_eq!(ComponentType::Vccs.spice_prefix(), "G");
    }

    #[test]
    fn test_spice_prefix_xspice() {
        assert_eq!(ComponentType::XspiceGain.spice_prefix(), "A");
        assert_eq!(ComponentType::XspiceAndGate.spice_prefix(), "A");
        assert_eq!(ComponentType::XspiceDFlipFlop.spice_prefix(), "A");
    }

    #[test]
    fn test_terminal_count() {
        assert_eq!(ComponentType::Resistor.terminal_count(), 2);
        assert_eq!(ComponentType::NpnBjt.terminal_count(), 3);
        assert_eq!(ComponentType::Nmos.terminal_count(), 4);
        assert_eq!(ComponentType::Ground.terminal_count(), 1);
        assert_eq!(ComponentType::Vcvs.terminal_count(), 4);
    }

    #[test]
    fn test_terminal_offsets_resistor() {
        let offsets = ComponentType::Resistor.terminal_offsets();
        assert_eq!(offsets.len(), 2);
        assert_eq!(offsets[0].0, "+");
        assert_eq!(offsets[1].0, "-");
    }

    #[test]
    fn test_terminal_offsets_bjt() {
        let offsets = ComponentType::NpnBjt.terminal_offsets();
        assert_eq!(offsets.len(), 3);
        let names: Vec<_> = offsets.iter().map(|(n, _)| *n).collect();
        assert!(names.contains(&"B")); // Base
        assert!(names.contains(&"C")); // Collector
        assert!(names.contains(&"E")); // Emitter
    }

    #[test]
    fn test_is_passive() {
        assert!(ComponentType::Resistor.is_passive());
        assert!(ComponentType::Capacitor.is_passive());
        assert!(!ComponentType::Diode.is_passive());
        assert!(!ComponentType::VoltageSource.is_passive());
    }

    #[test]
    fn test_is_semiconductor() {
        assert!(ComponentType::Diode.is_semiconductor());
        assert!(ComponentType::NpnBjt.is_semiconductor());
        assert!(ComponentType::Nmos.is_semiconductor());
        assert!(!ComponentType::Resistor.is_semiconductor());
    }

    #[test]
    fn test_is_source() {
        assert!(ComponentType::VoltageSource.is_source());
        assert!(ComponentType::CurrentSource.is_source());
        assert!(ComponentType::Vcvs.is_source());
        assert!(!ComponentType::Resistor.is_source());
    }

    #[test]
    fn test_is_xspice() {
        assert!(ComponentType::XspiceGain.is_xspice());
        assert!(ComponentType::XspiceAndGate.is_xspice());
        assert!(!ComponentType::Resistor.is_xspice());
    }

    #[test]
    fn test_is_digital() {
        assert!(ComponentType::XspiceAndGate.is_digital());
        assert!(ComponentType::XspiceDFlipFlop.is_digital());
        assert!(!ComponentType::XspiceGain.is_digital());
        assert!(!ComponentType::Resistor.is_digital());
    }

    #[test]
    fn test_default_value() {
        assert_eq!(ComponentType::Resistor.default_value(), "1k");
        assert_eq!(ComponentType::Capacitor.default_value(), "1u");
        assert_eq!(ComponentType::Inductor.default_value(), "1m");
        assert_eq!(ComponentType::VoltageSource.default_value(), "5");
        assert_eq!(ComponentType::Diode.default_value(), "");
    }

    #[test]
    fn test_display_name() {
        assert_eq!(ComponentType::Resistor.display_name(), "Resistor");
        assert_eq!(ComponentType::NpnBjt.display_name(), "NPN BJT");
        assert_eq!(ComponentType::XspiceAndGate.display_name(), "AND Gate");
    }

    // =========================================================================
    // Advanced Source Types - Comprehensive Tests
    // =========================================================================

    #[test]
    fn test_advanced_voltage_sources_spice_prefix() {
        // All voltage sources use "V" prefix
        assert_eq!(ComponentType::VoltageSourcePwl.spice_prefix(), "V");
        assert_eq!(ComponentType::VoltageSourceExp.spice_prefix(), "V");
        assert_eq!(ComponentType::VoltageSourceSffm.spice_prefix(), "V");
    }

    #[test]
    fn test_advanced_current_sources_spice_prefix() {
        // All current sources use "I" prefix
        assert_eq!(ComponentType::CurrentSourceAc.spice_prefix(), "I");
        assert_eq!(ComponentType::CurrentSourcePulse.spice_prefix(), "I");
        assert_eq!(ComponentType::CurrentSourceSin.spice_prefix(), "I");
        assert_eq!(ComponentType::CurrentSourcePwl.spice_prefix(), "I");
        assert_eq!(ComponentType::CurrentSourceExp.spice_prefix(), "I");
        assert_eq!(ComponentType::CurrentSourceNoise.spice_prefix(), "I");
    }

    #[test]
    fn test_advanced_voltage_sources_display_names() {
        assert_eq!(ComponentType::VoltageSourcePwl.display_name(), "V PWL");
        assert_eq!(ComponentType::VoltageSourceExp.display_name(), "V Exp");
        assert_eq!(ComponentType::VoltageSourceSffm.display_name(), "V SFFM");
    }

    #[test]
    fn test_advanced_current_sources_display_names() {
        assert_eq!(ComponentType::CurrentSourceAc.display_name(), "I AC");
        assert_eq!(ComponentType::CurrentSourcePulse.display_name(), "I Pulse");
        assert_eq!(ComponentType::CurrentSourceSin.display_name(), "I Sin");
        assert_eq!(ComponentType::CurrentSourcePwl.display_name(), "I PWL");
        assert_eq!(ComponentType::CurrentSourceExp.display_name(), "I Exp");
        assert_eq!(ComponentType::CurrentSourceNoise.display_name(), "I Noise");
    }

    #[test]
    fn test_advanced_sources_terminal_count() {
        // All independent sources have 2 terminals
        assert_eq!(ComponentType::VoltageSourcePwl.terminal_count(), 2);
        assert_eq!(ComponentType::VoltageSourceExp.terminal_count(), 2);
        assert_eq!(ComponentType::VoltageSourceSffm.terminal_count(), 2);
        assert_eq!(ComponentType::CurrentSourceAc.terminal_count(), 2);
        assert_eq!(ComponentType::CurrentSourcePulse.terminal_count(), 2);
        assert_eq!(ComponentType::CurrentSourceSin.terminal_count(), 2);
        assert_eq!(ComponentType::CurrentSourcePwl.terminal_count(), 2);
        assert_eq!(ComponentType::CurrentSourceExp.terminal_count(), 2);
        assert_eq!(ComponentType::CurrentSourceNoise.terminal_count(), 2);
    }

    #[test]
    fn test_advanced_sources_terminal_offsets() {
        // All sources have +/- terminals with vertical layout
        for source_type in [
            ComponentType::VoltageSourcePwl,
            ComponentType::VoltageSourceExp,
            ComponentType::VoltageSourceSffm,
            ComponentType::CurrentSourceAc,
            ComponentType::CurrentSourcePulse,
            ComponentType::CurrentSourceSin,
            ComponentType::CurrentSourcePwl,
            ComponentType::CurrentSourceExp,
            ComponentType::CurrentSourceNoise,
        ] {
            let offsets = source_type.terminal_offsets();
            assert_eq!(
                offsets.len(),
                2,
                "Source {:?} should have 2 terminals",
                source_type
            );
            assert_eq!(
                offsets[0].0, "+",
                "First terminal should be + for {:?}",
                source_type
            );
            assert_eq!(
                offsets[1].0, "-",
                "Second terminal should be - for {:?}",
                source_type
            );
            // Verify vertical layout (y differs, x is same)
            assert_eq!(
                offsets[0].1.x, offsets[1].1.x,
                "Terminals should be vertically aligned for {:?}",
                source_type
            );
            assert!(
                offsets[0].1.y != offsets[1].1.y,
                "Terminals should be at different y for {:?}",
                source_type
            );
        }
    }

    #[test]
    fn test_advanced_sources_is_source() {
        // All advanced sources should return true for is_source()
        assert!(ComponentType::VoltageSourcePwl.is_source());
        assert!(ComponentType::VoltageSourceExp.is_source());
        assert!(ComponentType::VoltageSourceSffm.is_source());
        assert!(ComponentType::CurrentSourceAc.is_source());
        assert!(ComponentType::CurrentSourcePulse.is_source());
        assert!(ComponentType::CurrentSourceSin.is_source());
        assert!(ComponentType::CurrentSourcePwl.is_source());
        assert!(ComponentType::CurrentSourceExp.is_source());
        assert!(ComponentType::CurrentSourceNoise.is_source());
    }

    #[test]
    fn test_advanced_sources_not_passive() {
        // All sources are not passive
        for source_type in [
            ComponentType::VoltageSourcePwl,
            ComponentType::VoltageSourceExp,
            ComponentType::VoltageSourceSffm,
            ComponentType::CurrentSourceAc,
            ComponentType::CurrentSourcePulse,
            ComponentType::CurrentSourceSin,
            ComponentType::CurrentSourcePwl,
            ComponentType::CurrentSourceExp,
            ComponentType::CurrentSourceNoise,
        ] {
            assert!(
                !source_type.is_passive(),
                "{:?} should not be passive",
                source_type
            );
        }
    }

    #[test]
    fn test_advanced_sources_not_semiconductor() {
        // All sources are not semiconductors
        for source_type in [
            ComponentType::VoltageSourcePwl,
            ComponentType::VoltageSourceExp,
            ComponentType::VoltageSourceSffm,
            ComponentType::CurrentSourceAc,
            ComponentType::CurrentSourcePulse,
            ComponentType::CurrentSourceSin,
            ComponentType::CurrentSourcePwl,
            ComponentType::CurrentSourceExp,
            ComponentType::CurrentSourceNoise,
        ] {
            assert!(
                !source_type.is_semiconductor(),
                "{:?} should not be semiconductor",
                source_type
            );
        }
    }

    #[test]
    fn test_advanced_sources_not_xspice() {
        // All independent sources are not XSPICE
        for source_type in [
            ComponentType::VoltageSourcePwl,
            ComponentType::VoltageSourceExp,
            ComponentType::VoltageSourceSffm,
            ComponentType::CurrentSourceAc,
            ComponentType::CurrentSourcePulse,
            ComponentType::CurrentSourceSin,
            ComponentType::CurrentSourcePwl,
            ComponentType::CurrentSourceExp,
            ComponentType::CurrentSourceNoise,
        ] {
            assert!(
                !source_type.is_xspice(),
                "{:?} should not be XSPICE",
                source_type
            );
        }
    }

    #[test]
    fn test_power_electronics_spice_prefix() {
        // Power electronics MOSFETs use "M" prefix
        assert_eq!(ComponentType::NVdmos.spice_prefix(), "M");
        assert_eq!(ComponentType::PVdmos.spice_prefix(), "M");
        // Saturable inductor uses "L" prefix
        assert_eq!(ComponentType::SaturableInductor.spice_prefix(), "L");
    }

    #[test]
    fn test_power_electronics_display_names() {
        assert_eq!(ComponentType::NVdmos.display_name(), "N-VDMOS");
        assert_eq!(ComponentType::PVdmos.display_name(), "P-VDMOS");
        assert_eq!(
            ComponentType::SaturableInductor.display_name(),
            "Saturable L"
        );
    }

    #[test]
    fn test_power_electronics_terminal_count() {
        // VDMOS has 4 terminals like regular MOSFET
        assert_eq!(ComponentType::NVdmos.terminal_count(), 4);
        assert_eq!(ComponentType::PVdmos.terminal_count(), 4);
        // Saturable inductor has 2 terminals
        assert_eq!(ComponentType::SaturableInductor.terminal_count(), 2);
    }

    #[test]
    fn test_power_electronics_is_semiconductor() {
        // VDMOS devices are semiconductors
        assert!(ComponentType::NVdmos.is_semiconductor());
        assert!(ComponentType::PVdmos.is_semiconductor());
        // Saturable inductor is not a semiconductor
        assert!(!ComponentType::SaturableInductor.is_semiconductor());
    }
}
