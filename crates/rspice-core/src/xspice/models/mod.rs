//! Built-in XSPICE Code Models
//!
//! Provides a comprehensive library of analog and digital code models.
//!
//! # Analog Models
//!
//! - **gain** - Voltage gain block
//! - **summer** - Analog summer
//! - **mult** - Analog multiplier  
//! - **divider** - Analog divider
//! - **limit** - Limiter with clipping
//! - **integrator** - Continuous-time integrator
//! - **differentiator** - Continuous-time differentiator
//! - **aswitch** - Analog switch
//! - **s_xfer** - S-domain transfer function (future)
//!
//! # A/D and D/A Bridges
//!
//! - **adc_bridge** - Analog to digital converter
//! - **dac_bridge** - Digital to analog converter
//!
//! # Digital Models
//!
//! - **d_source** - Digital stimulus from file
//! - **d_inverter** - Inverter gate
//! - **d_buffer** - Buffer gate
//! - **d_and** / **d_nand** - AND/NAND gates
//! - **d_or** / **d_nor** - OR/NOR gates
//! - **d_xor** / **d_xnor** - XOR/XNOR gates
//! - **d_tristate** - Tri-state buffer
//! - **d_pullup** / **d_pulldown** - Pull resistors
//! - **d_dff** - D flip-flop
//! - **d_jkff** - JK flip-flop
//! - **d_tff** - T flip-flop
//! - **d_srff** - SR flip-flop
//! - **d_dlatch** - D latch
//! - **d_srlatch** - SR latch
//! - **d_state** - State machine
//! - **d_ram** - Random access memory
//! - **d_rom** - Read-only memory

// Analog behavioral blocks
mod analog;

// A/D and D/A bridges
mod bridges;

// Digital primitives
mod digital;

// Re-export all models
pub use analog::{
    AnalogSwitch, Differentiator, Divider, Gain, Integrator, Limiter, Multiplier, SampleHold,
    Summer,
};

pub use bridges::{AdcBridge, DacBridge};

pub use digital::{
    DFlipFlop, DLatch, DigitalAnd, DigitalBuffer, DigitalInverter, DigitalNand, DigitalNor,
    DigitalOr, DigitalPulldown, DigitalPullup, DigitalRam, DigitalRom, DigitalSource,
    DigitalStateMachine, DigitalTristate, DigitalXnor, DigitalXor, JkFlipFlop, SrFlipFlop, SrLatch,
    TFlipFlop,
};
