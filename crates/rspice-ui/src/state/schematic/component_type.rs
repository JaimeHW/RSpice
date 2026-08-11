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
    /// N-channel MESFET / HFET (SPICE prefix: Z)
    Nmesfet,
    /// P-channel MESFET / HFET (SPICE prefix: Z)
    Pmesfet,
    /// N-channel VDMOS Power MOSFET (SPICE prefix: M)
    NVdmos,
    /// P-channel VDMOS Power MOSFET (SPICE prefix: M)
    PVdmos,
    /// N-channel SOI MOSFET — D G S E (back gate) P (body contact)
    /// (SPICE prefix: M)
    NmosSoi,
    /// P-channel SOI MOSFET (SPICE prefix: M)
    PmosSoi,
    /// NPN BJT with substrate terminal — C B E S (SPICE prefix: Q)
    NpnBjt4,
    /// PNP BJT with substrate terminal (SPICE prefix: Q)
    PnpBjt4,
    /// NPN BJT with substrate and thermal terminals — C B E S dT,
    /// for electrothermal VBIC-class models (SPICE prefix: Q)
    NpnBjt5,
    /// PNP BJT with substrate and thermal terminals (SPICE prefix: Q)
    PnpBjt5,
    /// Saturable Core Inductor (SPICE prefix: L)
    SaturableInductor,
    /// Memristor — emitted as a Xyce YMEMRISTOR element (designator
    /// prefix: MR)
    Memristor,
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
    /// File-Backed Piecewise Linear Voltage Source (SPICE prefix: V)
    VoltageSourcePwlFile,
    /// Exponential Voltage Source (SPICE prefix: V)
    VoltageSourceExp,
    /// Single-Frequency FM Voltage Source (SPICE prefix: V)
    VoltageSourceSffm,
    /// Amplitude-Modulated Voltage Source (SPICE prefix: V)
    VoltageSourceAm,
    /// Digital Pattern Voltage Source (SPICE prefix: V)
    VoltageSourcePat,
    /// Noise Voltage Source (SPICE prefix: V)
    VoltageSourceNoise,
    /// AC Current Source (SPICE prefix: I)
    CurrentSourceAc,
    /// Pulse Current Source (SPICE prefix: I)
    CurrentSourcePulse,
    /// Sinusoidal Current Source (SPICE prefix: I)
    CurrentSourceSin,
    /// Piecewise Linear Current Source (SPICE prefix: I)
    CurrentSourcePwl,
    /// File-Backed Piecewise Linear Current Source (SPICE prefix: I)
    CurrentSourcePwlFile,
    /// Exponential Current Source (SPICE prefix: I)
    CurrentSourceExp,
    /// Single-Frequency FM Current Source (SPICE prefix: I)
    CurrentSourceSffm,
    /// Amplitude-Modulated Current Source (SPICE prefix: I)
    CurrentSourceAm,
    /// Digital Pattern Current Source (SPICE prefix: I)
    CurrentSourcePat,
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
    /// Behavioral source — arbitrary V=/I= expression (SPICE prefix: B)
    BehavioralSource,
    /// Voltage-controlled switch (SPICE prefix: S)
    VSwitch,
    /// Current-controlled switch — senses the current through a named
    /// V source (SPICE prefix: W)
    ISwitch,
    /// Expression-controlled two-terminal switch (SPICE prefix: S)
    GenericSwitch,
    /// Lossless transmission line (SPICE prefix: T)
    TransmissionLine,
    /// Lossy transmission line bound to an LTRA or TXL model
    /// (SPICE prefix: O)
    LossyTransmissionLine,
    /// Two-conductor coupled transmission line bound to a CPL model
    /// (SPICE prefix: P)
    CoupledTransmissionLine,
    /// RF port — Z0-terminated Thevenin port for S-parameter and RF work
    /// (SPICE prefix: P)
    RfPort,
    /// Ground node (no SPICE prefix - implicit node 0)
    Ground,
    /// Interface port — names its net and declares it as a pin of the
    /// containing cell (no SPICE prefix; ports shape the .SUBCKT header)
    Port,
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

impl ComponentType {
    /// Exhaustive component-family inventory used by schema, editor, and
    /// netlist contract tests. Adding a new enum variant must update this
    /// list, making missing commercial editor coverage visible immediately.
    pub const ALL: [Self; 82] = [
        Self::Resistor,
        Self::Capacitor,
        Self::Inductor,
        Self::Transformer,
        Self::CoupledInductor,
        Self::Diode,
        Self::NpnBjt,
        Self::PnpBjt,
        Self::Nmos,
        Self::Pmos,
        Self::Njfet,
        Self::Pjfet,
        Self::Nmesfet,
        Self::Pmesfet,
        Self::NVdmos,
        Self::PVdmos,
        Self::NmosSoi,
        Self::PmosSoi,
        Self::NpnBjt4,
        Self::PnpBjt4,
        Self::NpnBjt5,
        Self::PnpBjt5,
        Self::SaturableInductor,
        Self::Memristor,
        Self::VoltageSource,
        Self::CurrentSource,
        Self::VoltageSourceAc,
        Self::VoltageSourcePulse,
        Self::VoltageSourceSin,
        Self::VoltageSourcePwl,
        Self::VoltageSourcePwlFile,
        Self::VoltageSourceExp,
        Self::VoltageSourceSffm,
        Self::VoltageSourceAm,
        Self::VoltageSourcePat,
        Self::VoltageSourceNoise,
        Self::CurrentSourceAc,
        Self::CurrentSourcePulse,
        Self::CurrentSourceSin,
        Self::CurrentSourcePwl,
        Self::CurrentSourcePwlFile,
        Self::CurrentSourceExp,
        Self::CurrentSourceSffm,
        Self::CurrentSourceAm,
        Self::CurrentSourcePat,
        Self::CurrentSourceNoise,
        Self::Vcvs,
        Self::Vccs,
        Self::Ccvs,
        Self::Cccs,
        Self::OpAmp,
        Self::BehavioralSource,
        Self::VSwitch,
        Self::ISwitch,
        Self::GenericSwitch,
        Self::TransmissionLine,
        Self::LossyTransmissionLine,
        Self::CoupledTransmissionLine,
        Self::RfPort,
        Self::Ground,
        Self::Port,
        Self::CellInstance,
        Self::XspiceGain,
        Self::XspiceSummer,
        Self::XspiceMultiplier,
        Self::XspiceDivider,
        Self::XspiceLimiter,
        Self::XspiceIntegrator,
        Self::XspiceDifferentiator,
        Self::XspiceInverter,
        Self::XspiceBuffer,
        Self::XspiceAndGate,
        Self::XspiceOrGate,
        Self::XspiceNandGate,
        Self::XspiceNorGate,
        Self::XspiceXorGate,
        Self::XspiceTristate,
        Self::XspiceDFlipFlop,
        Self::XspiceJkFlipFlop,
        Self::XspiceSrLatch,
        Self::XspiceAdcBridge,
        Self::XspiceDacBridge,
    ];
}
