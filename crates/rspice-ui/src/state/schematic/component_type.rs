//! Component Types
//!
//! Enumeration of all supported schematic component types with their
//! SPICE prefixes, display names, terminal counts, and terminal positions.

mod categories;
mod geometry;
mod metadata;

use serde::{Deserialize, Serialize};

/// Component types available in the schematic.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ComponentType {
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
    /// Diode (SPICE prefix: D)
    Diode,
    /// NPN Bipolar Junction Transistor (SPICE prefix: Q)
    NpnBjt,
    /// PNP Bipolar Junction Transistor (SPICE prefix: Q)
    PnpBjt,
    /// N-channel MOSFET (SPICE prefix: M)
    Nmos,
    /// P-channel MOSFET (SPICE prefix: M)
    Pmos,
    /// N-channel JFET (SPICE prefix: J)
    Njfet,
    /// P-channel JFET (SPICE prefix: J)
    Pjfet,
    /// N-channel VDMOS Power MOSFET (SPICE prefix: M)
    NVdmos,
    /// P-channel VDMOS Power MOSFET (SPICE prefix: M)
    PVdmos,
    /// Saturable Core Inductor (SPICE prefix: L)
    SaturableInductor,
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
    /// Voltage-Controlled Voltage Source (SPICE prefix: E)
    Vcvs,
    /// Voltage-Controlled Current Source (SPICE prefix: G)
    Vccs,
    /// Current-Controlled Voltage Source (SPICE prefix: H)
    Ccvs,
    /// Current-Controlled Current Source (SPICE prefix: F)
    Cccs,
    /// Ideal operational amplifier — emitted as a ground-referenced VCVS
    /// (SPICE prefix: E)
    OpAmp,
    /// Ground node (no SPICE prefix - implicit node 0)
    Ground,
    /// Generic hierarchical/library cell instance (SPICE prefix: X)
    CellInstance,
    /// Gain block
    XspiceGain,
    /// Summing amplifier
    XspiceSummer,
    /// Analog multiplier
    XspiceMultiplier,
    /// Analog divider
    XspiceDivider,
    /// Hard limiter
    XspiceLimiter,
    /// Integrator
    XspiceIntegrator,
    /// Differentiator
    XspiceDifferentiator,
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
    /// D Flip-Flop
    XspiceDFlipFlop,
    /// JK Flip-Flop
    XspiceJkFlipFlop,
    /// SR Latch
    XspiceSrLatch,
    /// Analog-to-Digital converter bridge
    XspiceAdcBridge,
    /// Digital-to-Analog converter bridge
    XspiceDacBridge,
}
