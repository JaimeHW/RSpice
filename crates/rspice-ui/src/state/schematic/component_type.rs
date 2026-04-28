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
    /// Two-winding schematic transformer synthesized into coupled inductors
    Transformer,
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
    /// Generic hierarchical/library cell instance (SPICE prefix: X)
    CellInstance,

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
            ComponentType::Transformer => "T",
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
            ComponentType::CellInstance => "X",
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
            ComponentType::Transformer => "Transformer",
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
            ComponentType::CellInstance => "Cell Instance",
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
            ComponentType::CellInstance => 2,
            ComponentType::Transformer => 4,
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
    ///
    /// **IMPORTANT**: Terminal positions are derived from `symbol_dimensions()`
    /// using half-width (hw) and half-height (hh). Since dimensions are always
    /// even integers, terminals always land on grid intersections.
    pub fn terminal_offsets(&self) -> Vec<(&'static str, Point)> {
        let (w, h) = self.symbol_dimensions();
        let hw = w / 2; // Half-width - always integer since w is even
        let hh = h / 2; // Half-height - always integer since h is even

        match self {
            // Two-terminal horizontal passives: terminals at left/right edges
            ComponentType::Resistor | ComponentType::Capacitor | ComponentType::Inductor => {
                vec![("+", Point::new(-hw, 0)), ("-", Point::new(hw, 0))]
            }
            ComponentType::Transformer => {
                let pin_inset = 10;
                vec![
                    ("P1", Point::new(-hw + pin_inset, -hh)),
                    ("P2", Point::new(-hw + pin_inset, hh)),
                    ("S1", Point::new(hw - pin_inset, -hh)),
                    ("S2", Point::new(hw - pin_inset, hh)),
                ]
            }
            ComponentType::Diode => vec![
                ("A", Point::new(-hw, 0)), // Anode
                ("K", Point::new(hw, 0)),  // Cathode
            ],
            // Vertical sources: terminals at top/bottom
            ComponentType::VoltageSource
            | ComponentType::VoltageSourceAc
            | ComponentType::VoltageSourcePulse
            | ComponentType::VoltageSourceSin
            | ComponentType::VoltageSourcePwl
            | ComponentType::VoltageSourceExp
            | ComponentType::VoltageSourceSffm => {
                vec![("+", Point::new(0, -hh)), ("-", Point::new(0, hh))]
            }
            ComponentType::CurrentSource
            | ComponentType::CurrentSourceAc
            | ComponentType::CurrentSourcePulse
            | ComponentType::CurrentSourceSin
            | ComponentType::CurrentSourcePwl
            | ComponentType::CurrentSourceExp
            | ComponentType::CurrentSourceNoise => {
                vec![("+", Point::new(0, -hh)), ("-", Point::new(0, hh))]
            }
            // BJTs: SPICE format is Q name C B E model, so order must be C, B, E
            // With 40x80 dimensions (matching SVG aspect ratio), C/E at ±40 (on major grid)
            ComponentType::NpnBjt => vec![
                ("C", Point::new(hw, -hh)), // Collector (top-right) - SPICE order: 1st
                ("B", Point::new(-hw, 0)),  // Base (center-left) - SPICE order: 2nd
                ("E", Point::new(hw, hh)),  // Emitter (bottom-right) - SPICE order: 3rd
            ],
            ComponentType::PnpBjt => vec![
                ("C", Point::new(hw, hh)),  // Collector (bottom for PNP) - SPICE order: 1st
                ("B", Point::new(-hw, 0)),  // Base - SPICE order: 2nd
                ("E", Point::new(hw, -hh)), // Emitter (top for PNP) - SPICE order: 3rd
            ],
            // MOSFETs: SPICE format is M name D G S B model, so order must be D, G, S, B
            // With 40x80 dimensions, D/S at ±40 (on major grid)
            ComponentType::Nmos | ComponentType::Pmos => vec![
                ("D", Point::new(hw, -hh)), // Drain (top-right) - SPICE order: 1st
                ("G", Point::new(-hw, 0)),  // Gate (center-left) - SPICE order: 2nd
                ("S", Point::new(hw, hh)),  // Source (bottom-right) - SPICE order: 3rd
                ("B", Point::new(hw, 0)),   // Bulk (center-right) - SPICE order: 4th
            ],
            // JFETs: SPICE format is J name D G S model, so order must be D, G, S
            ComponentType::Njfet | ComponentType::Pjfet => vec![
                ("D", Point::new(hw, -hh)), // Drain (top-right) - SPICE order: 1st
                ("G", Point::new(-hw, 0)),  // Gate (center-left) - SPICE order: 2nd
                ("S", Point::new(hw, hh)),  // Source (bottom-right) - SPICE order: 3rd
            ],
            // Power MOSFETs (VDMOS) - same terminal layout as MOSFET (D, G, S, B)
            ComponentType::NVdmos | ComponentType::PVdmos => vec![
                ("D", Point::new(hw, -hh)), // Drain (top-right) - SPICE order: 1st
                ("G", Point::new(-hw, 0)),  // Gate (center-left) - SPICE order: 2nd
                ("S", Point::new(hw, hh)),  // Source (bottom-right) - SPICE order: 3rd
                ("B", Point::new(hw, 0)),   // Bulk (center-right) - SPICE order: 4th
            ],
            // Saturable inductor - same as regular inductor
            ComponentType::SaturableInductor => {
                vec![("+", Point::new(-hw, 0)), ("-", Point::new(hw, 0))]
            }
            // Controlled sources: output on left, control on right
            ComponentType::Vcvs
            | ComponentType::Vccs
            | ComponentType::Ccvs
            | ComponentType::Cccs => vec![
                ("O+", Point::new(-hw, -hh / 2)), // Output +
                ("O-", Point::new(-hw, hh / 2)),  // Output -
                ("C+", Point::new(hw, -hh / 2)),  // Control +
                ("C-", Point::new(hw, hh / 2)),   // Control -
            ],
            // Coupled inductor doesn't have terminals (it's a coupling statement)
            ComponentType::CoupledInductor => vec![],
            // Generic instance: default 2-pin symbol, dynamic pin layouts are handled in Component
            ComponentType::CellInstance => {
                vec![("1", Point::new(-hw, 0)), ("2", Point::new(hw, 0))]
            }
            // Ground: single terminal at top
            ComponentType::Ground => vec![("GND", Point::new(0, -hh))],

            // XSPICE 2-terminal analog blocks: input left, output right
            ComponentType::XspiceGain
            | ComponentType::XspiceLimiter
            | ComponentType::XspiceIntegrator
            | ComponentType::XspiceDifferentiator => {
                vec![("in", Point::new(-hw, 0)), ("out", Point::new(hw, 0))]
            }
            // Summer: multiple inputs (top/bottom left), one output right
            ComponentType::XspiceSummer => vec![
                ("in1", Point::new(-hw, -hh / 2)),
                ("in2", Point::new(-hw, hh / 2)),
                ("out", Point::new(hw, 0)),
            ],
            // Multiplier/Divider: two inputs, one output
            ComponentType::XspiceMultiplier | ComponentType::XspiceDivider => vec![
                ("in1", Point::new(-hw, -hh / 2)),
                ("in2", Point::new(-hw, hh / 2)),
                ("out", Point::new(hw, 0)),
            ],
            // Digital gates: inputs left, output right
            ComponentType::XspiceInverter | ComponentType::XspiceBuffer => {
                vec![("in", Point::new(-hw, 0)), ("out", Point::new(hw, 0))]
            }
            ComponentType::XspiceAndGate
            | ComponentType::XspiceOrGate
            | ComponentType::XspiceNandGate
            | ComponentType::XspiceNorGate
            | ComponentType::XspiceXorGate => vec![
                ("a", Point::new(-hw, -hh / 2)),
                ("b", Point::new(-hw, hh / 2)),
                ("out", Point::new(hw, 0)),
            ],
            // Tri-state: input, enable, output
            ComponentType::XspiceTristate => vec![
                ("in", Point::new(-hw, 0)),
                ("en", Point::new(0, -hh)),
                ("out", Point::new(hw, 0)),
            ],
            // D Flip-Flop: D, CLK, Q, Qbar
            ComponentType::XspiceDFlipFlop => vec![
                ("d", Point::new(-hw, -hh / 2)),
                ("clk", Point::new(-hw, hh / 2)),
                ("q", Point::new(hw, -hh / 2)),
                ("qbar", Point::new(hw, hh / 2)),
            ],
            // JK Flip-Flop: J, K, CLK, Q, Qbar
            ComponentType::XspiceJkFlipFlop => vec![
                ("j", Point::new(-hw, -hh / 2)),
                ("k", Point::new(-hw, hh / 2)),
                ("clk", Point::new(-hw, 0)),
                ("q", Point::new(hw, -hh / 2)),
                ("qbar", Point::new(hw, hh / 2)),
            ],
            // SR Latch: S, R, Q, Qbar
            ComponentType::XspiceSrLatch => vec![
                ("s", Point::new(-hw, -hh / 2)),
                ("r", Point::new(-hw, hh / 2)),
                ("q", Point::new(hw, -hh / 2)),
                ("qbar", Point::new(hw, hh / 2)),
            ],
            // ADC Bridge: analog input, digital output
            ComponentType::XspiceAdcBridge => {
                vec![("in", Point::new(-hw, 0)), ("out", Point::new(hw, 0))]
            }
            // DAC Bridge: digital input, analog output
            ComponentType::XspiceDacBridge => {
                vec![("in", Point::new(-hw, 0)), ("out", Point::new(hw, 0))]
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
                | ComponentType::Transformer
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

    /// Check if this is a PWL (Piecewise Linear) source
    pub fn is_pwl_source(&self) -> bool {
        matches!(
            self,
            ComponentType::VoltageSourcePwl | ComponentType::CurrentSourcePwl
        )
    }

    /// Get symbol dimensions in grid units (width, height)
    ///
    /// These dimensions define the rendered size of the component symbol.
    /// **CRITICAL**: All dimensions MUST be multiples of 20 so that
    /// half-width and half-height (used for terminal positions) are
    /// multiples of 10, ensuring terminals land on major grid lines.
    ///
    /// Grid spacing is 10 units. This is the commercial EDA standard.
    pub fn symbol_dimensions(&self) -> (i32, i32) {
        match self {
            // Passive components: horizontal orientation
            // 40 wide x 20 tall → terminals at ±20 grid units (on major grid)
            ComponentType::Resistor | ComponentType::Inductor => (40, 20),
            // Transformer: taller two-coil body with four winding terminals
            ComponentType::Transformer => (60, 80),
            // Capacitor: SVG is 31x31 (square aspect ratio)
            // Using 40x40 for uniform scaling - terminals at ±20
            ComponentType::Capacitor => (40, 40),

            // Sources: vertical orientation with leads
            // SVG v_src_dc.svg is 35x51, so target dimensions preserve that aspect ratio.
            // 28x40 gives aspect ratio 0.7 ≈ 35/51 = 0.686, ensuring leads reach terminals.
            ComponentType::VoltageSource
            | ComponentType::VoltageSourceAc
            | ComponentType::VoltageSourcePulse
            | ComponentType::VoltageSourceSin
            | ComponentType::VoltageSourcePwl
            | ComponentType::VoltageSourceExp
            | ComponentType::VoltageSourceSffm
            | ComponentType::CurrentSource
            | ComponentType::CurrentSourceAc
            | ComponentType::CurrentSourcePulse
            | ComponentType::CurrentSourceSin
            | ComponentType::CurrentSourcePwl
            | ComponentType::CurrentSourceExp
            | ComponentType::CurrentSourceNoise => (28, 40),

            // Ground: compact but on grid
            ComponentType::Ground => (20, 20),
            // Generic instance: wider rectangular body for block-style symbols
            ComponentType::CellInstance => (60, 40),

            // Diode: horizontal
            ComponentType::Diode => (40, 20),

            // BJTs: 40x80 to match SVG aspect ratio (40.5x81mm) - uniform scaling
            // This preserves the arrow shape without distortion
            ComponentType::NpnBjt | ComponentType::PnpBjt => (40, 80),

            // MOSFETs/JFETs: 40x80 for 1:2 aspect ratio - uniform scaling with grid alignment
            // D/S terminals at ±40 (on major grid)
            ComponentType::Nmos
            | ComponentType::Pmos
            | ComponentType::Njfet
            | ComponentType::Pjfet
            | ComponentType::NVdmos
            | ComponentType::PVdmos => (40, 80),

            // Saturable inductor: same as regular inductor
            ComponentType::SaturableInductor => (40, 20),

            // Controlled sources: 4-terminal box
            ComponentType::Vcvs
            | ComponentType::Vccs
            | ComponentType::Ccvs
            | ComponentType::Cccs => (40, 40),

            // Coupled inductor: no visual (coupling statement)
            ComponentType::CoupledInductor => (0, 0),

            // XSPICE blocks: horizontal box
            ComponentType::XspiceGain
            | ComponentType::XspiceSummer
            | ComponentType::XspiceMultiplier
            | ComponentType::XspiceDivider
            | ComponentType::XspiceLimiter
            | ComponentType::XspiceIntegrator
            | ComponentType::XspiceDifferentiator => (40, 20),

            // XSPICE digital gates: horizontal
            ComponentType::XspiceInverter
            | ComponentType::XspiceBuffer
            | ComponentType::XspiceAndGate
            | ComponentType::XspiceOrGate
            | ComponentType::XspiceNandGate
            | ComponentType::XspiceNorGate
            | ComponentType::XspiceXorGate
            | ComponentType::XspiceTristate => (40, 20),

            // XSPICE sequential: taller for more pins
            ComponentType::XspiceDFlipFlop
            | ComponentType::XspiceJkFlipFlop
            | ComponentType::XspiceSrLatch => (40, 40),

            // XSPICE bridges
            ComponentType::XspiceAdcBridge | ComponentType::XspiceDacBridge => (40, 20),
        }
    }

    /// Get default value for this component type
    pub fn default_value(&self) -> &'static str {
        match self {
            ComponentType::Resistor => "1k",
            ComponentType::Capacitor => "1u",
            ComponentType::Inductor => "1m",
            ComponentType::Transformer => "1m",
            ComponentType::VoltageSource
            | ComponentType::VoltageSourceAc
            | ComponentType::VoltageSourcePulse
            | ComponentType::VoltageSourceSin => "5",
            ComponentType::CurrentSource => "1m",
            ComponentType::CellInstance => "",
            _ => "",
        }
    }
}

// =============================================================================
// Tests
// =============================================================================

