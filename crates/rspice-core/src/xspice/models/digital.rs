//! Digital Code Models - Gates, Flip-Flops, Memory
//!
//! Provides digital logic primitives for mixed-signal simulation.

use crate::xspice::{
    CmContext, CmResult, CodeModel, DigitalState, DigitalStrength, DigitalValue, ParamSpec,
    PortSpec, PortType,
};

mod cosim;
mod gates;
mod lookup;
mod memory;
mod oscillators;
mod process;
mod sequential;
mod sources;

pub use cosim::DigitalCosim;
pub use gates::{
    DigitalAnd, DigitalBuffer, DigitalInverter, DigitalNand, DigitalNor, DigitalOpenCollector,
    DigitalOpenEmitter, DigitalOr, DigitalPulldown, DigitalPullup, DigitalTristate, DigitalXnor,
    DigitalXor,
};
pub use lookup::{DigitalGenericLookupTable, DigitalLookupTable};
pub use memory::DigitalRam;
pub use oscillators::{DigitalOscillator, DigitalPwmOscillator};
pub use process::DigitalProcess;
pub use sequential::{
    DFlipFlop, DLatch, DigitalFrequencyDivider, JkFlipFlop, SrFlipFlop, SrLatch, TFlipFlop,
};
pub use sources::{DigitalSource, DigitalStateMachine};
