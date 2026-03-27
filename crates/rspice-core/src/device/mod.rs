//! Device models module
//!
//! Provides behavioral models for circuit components, organized into:
//! - `passive/` - Resistors, capacitors, inductors
//! - `semiconductor/` - Diodes, BJTs
//! - `mosfet/` - MOSFETs, VDMOSFETs, JFETs, BSIM4, EKV
//! - `veriloga/` - Verilog-A compiled models (optional feature)
#![allow(clippy::large_enum_variant)]

// Organized device subdirectories
pub mod mosfet;
pub mod passive;
pub mod semiconductor;

// Other devices (kept at root level)
pub mod behavioral;
pub mod controlled;
mod coupled_transmission_line;
pub mod ffi;
pub mod gan_hemt;
pub mod model_bypass;
pub mod opamp;
pub mod pwl_file;
mod sources;
mod switch;
pub mod thermal;
mod traits;
mod transmission_line;
pub mod tristate;

// Verilog-A support (feature-gated)
#[cfg(feature = "veriloga")]
pub mod veriloga;

// Batch device storage for SIMD acceleration (feature-gated)
#[cfg(feature = "simd")]
pub mod batch;

// Re-export from subdirectories for backwards compatibility
pub use mosfet::{
    Bsim4, Bsim4Params, Bsim4Type, EkvMosfet, Jfet, JfetChannelModel, JfetParams, JfetType,
    MosParams, MosRegion, MosType, Mosfet, MosfetIndices, Vdmos, VdmosRegion, VdmosType,
};
pub use passive::{
    Capacitor, CoupledInductorPair, Inductor, InductorCoupling, MultiWindingTransformer, Resistor,
    SaturableInductor,
};
pub use semiconductor::{Bjt, BjtType, Diode};

// Re-export from root-level modules
pub use behavioral::{BehavioralCurrentSource, BehavioralSources, BehavioralVoltageSource};
pub use controlled::{Cccs, Ccvs, Vccs, Vcvs};
pub use coupled_transmission_line::CoupledTransmissionLine;
pub use sources::{CurrentSource, VoltageSource};
pub use switch::{CurrentSwitch, SwitchState, VoltageSwitch};
pub use traits::*;
pub use transmission_line::{LossyTransmissionLine, TransmissionLine};
pub(crate) use transmission_line::TlineTransientResponse;

use crate::Value;

/// A device instance in the circuit
#[derive(Debug, Clone)]
pub enum Device {
    Resistor(Resistor),
    Capacitor(Capacitor),
    Inductor(Inductor),
    SaturableInductor(SaturableInductor),
    VoltageSource(VoltageSource),
    CurrentSource(CurrentSource),
    Diode(Diode),
    Bjt(Bjt),
    Mosfet(Mosfet),
    Vdmos(Vdmos),
    Jfet(Jfet),
}

impl Device {
    /// Get the device name
    pub fn name(&self) -> &str {
        match self {
            Device::Resistor(d) => &d.name,
            Device::Capacitor(d) => &d.name,
            Device::Inductor(d) => &d.name,
            Device::SaturableInductor(d) => &d.name,
            Device::VoltageSource(d) => &d.name,
            Device::CurrentSource(d) => &d.name,
            Device::Diode(d) => &d.name,
            Device::Bjt(d) => &d.name,
            Device::Mosfet(d) => &d.name,
            Device::Vdmos(d) => &d.name,
            Device::Jfet(d) => &d.name,
        }
    }
}

/// Device model parameters
#[derive(Debug, Clone)]
pub struct DeviceModel {
    pub name: String,
    pub model_type: ModelType,
    pub params: std::collections::HashMap<String, Value>,
}

/// Types of device models
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModelType {
    Resistor,
    Capacitor,
    Inductor,
    SaturableInductor,
    Diode,
    NpnBjt,
    PnpBjt,
    Nmos,
    Pmos,
    NVdmos,
    PVdmos,
    Njf,
    Pjf,
}

impl DeviceModel {
    pub fn new(name: String, model_type: ModelType) -> Self {
        Self {
            name,
            model_type,
            params: std::collections::HashMap::new(),
        }
    }

    pub fn with_param(mut self, key: &str, value: Value) -> Self {
        self.params.insert(key.to_string(), value);
        self
    }

    pub fn get_param(&self, key: &str, default: Value) -> Value {
        self.params.get(key).copied().unwrap_or(default)
    }
}
