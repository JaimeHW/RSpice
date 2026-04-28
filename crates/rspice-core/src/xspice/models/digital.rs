//! Digital Code Models - Gates, Flip-Flops, Memory
//!
//! Provides digital logic primitives for mixed-signal simulation.

use crate::xspice::{
    CmContext, CmResult, CodeModel, DigitalState, DigitalStrength, DigitalValue, ParamSpec,
    PortSpec, PortType,
};

mod gates;
mod memory;
mod sequential;
mod sources;

pub use gates::{
    DigitalAnd, DigitalBuffer, DigitalInverter, DigitalNand, DigitalNor, DigitalOr,
    DigitalPulldown, DigitalPullup, DigitalTristate, DigitalXnor, DigitalXor,
};
pub use memory::{DigitalRam, DigitalRom};
pub use sequential::{DFlipFlop, DLatch, JkFlipFlop, SrFlipFlop, SrLatch, TFlipFlop};
pub use sources::{DigitalSource, DigitalStateMachine};
